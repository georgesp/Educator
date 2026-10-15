//! The system voice of the iPad (`AVSpeechSynthesizer`, plugin `tauri-plugin-system-voice`) behind
//! the `SpeechSynthesizer` trait of the core (SUB023805, ADR 0005). A thin adapter, no business rule:
//! texts, speeds and the cutting of the words are decided by the core, as with Piper.
//! The plugin calls are blocking: they are made from worker threads, never from the main thread.
//!
//! Compiled on iOS and in the tests: the plugin is reached through [`VoiceBridge`], faked in the
//! tests so that the adapter is checked on the desktop.

use educator_core::speech::{SpeechEngineStatus, SpeechSynthesizer};
use std::fmt;
use std::sync::Once;
use tauri_plugin_system_voice::{Error as VoiceError, Voices};

/// Short text read at start-up so that the voice is loaded before the child's first request.
const WARM_UP_TEXT: &str = "Bonjour.";

/// What the adapter needs from the system voice: the plugin on the iPad, a fake in the tests.
pub trait VoiceBridge: Send + Sync {
    /// French voices of the device and the one used.
    fn voices(&self) -> Result<Voices, VoiceError>;

    /// `text` read at `speed` (1 = normal), as a 16-bit mono WAV.
    fn synthesize(&self, text: &str, speed: f64) -> Result<Vec<u8>, VoiceError>;
}

#[cfg(target_os = "ios")]
impl VoiceBridge for tauri_plugin_system_voice::SystemVoice<tauri::Wry> {
    fn voices(&self) -> Result<Voices, VoiceError> {
        tauri_plugin_system_voice::SystemVoice::voices(self)
    }

    fn synthesize(&self, text: &str, speed: f64) -> Result<Vec<u8>, VoiceError> {
        tauri_plugin_system_voice::SystemVoice::synthesize(self, text, speed, None)
    }
}

pub struct SystemVoiceSynthesizer<B> {
    bridge: B,
    voices_logged: Once,
}

impl<B> fmt::Debug for SystemVoiceSynthesizer<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SystemVoiceSynthesizer")
    }
}

impl<B: VoiceBridge> SystemVoiceSynthesizer<B> {
    pub fn new(bridge: B) -> Self {
        Self {
            bridge,
            voices_logged: Once::new(),
        }
    }
}

impl<B: VoiceBridge> SpeechSynthesizer for SystemVoiceSynthesizer<B> {
    fn status(&self) -> SpeechEngineStatus {
        match self.bridge.voices() {
            Ok(voices) => match voices.chosen {
                // The name only: the quality (« default »…) is English, the status bar shows French.
                Some(voice) => SpeechEngineStatus::available(voice.name),
                None => SpeechEngineStatus::voice_missing(
                    "No French (France) voice is installed on the iPad.",
                ),
            },
            Err(e) => SpeechEngineStatus::unavailable(format!("System voice unavailable: {e}")),
        }
    }

    fn synthesize(&self, text: &str, speed: f64) -> educator_core::Result<Vec<u8>> {
        self.bridge
            .synthesize(text, speed)
            .map_err(|e| educator_core::Error::SpeechUnavailable(e.to_string()))
    }

    fn warm_up(&self, speed: f64) {
        self.voices_logged.call_once(|| match self.bridge.voices() {
            Ok(voices) => {
                for voice in &voices.voices {
                    log::info!(
                        "French voice of the iPad: {} ({}, {}, {})",
                        voice.name,
                        voice.language,
                        voice.quality,
                        voice.id
                    );
                }
                log::info!(
                    "System voice used: {}",
                    voices.chosen.map_or_else(|| "none".to_owned(), |v| v.id)
                );
            }
            Err(e) => log::warn!("French voices of the iPad not listed: {e}"),
        });
        if let Err(e) = self.bridge.synthesize(WARM_UP_TEXT, speed) {
            log::warn!("System voice warm-up: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use tauri_plugin_system_voice::Voice;

    /// A system voice without a tablet: given voices, a given answer, and the requests it got.
    struct FakeBridge {
        voices: Result<Voices, VoiceError>,
        answer: Result<Vec<u8>, VoiceError>,
        requests: Mutex<Vec<(String, f64)>>,
    }

    fn thomas() -> Voice {
        Voice {
            id: "com.apple.voice.compact.fr-FR.Thomas".to_owned(),
            name: "Thomas".to_owned(),
            language: "fr-FR".to_owned(),
            quality: "default".to_owned(),
        }
    }

    fn bridge(chosen: Option<Voice>, answer: Result<Vec<u8>, VoiceError>) -> FakeBridge {
        FakeBridge {
            voices: Ok(Voices {
                voices: chosen.iter().cloned().collect(),
                chosen,
            }),
            answer,
            requests: Mutex::new(Vec::new()),
        }
    }

    impl VoiceBridge for FakeBridge {
        fn voices(&self) -> Result<Voices, VoiceError> {
            self.voices.clone()
        }

        fn synthesize(&self, text: &str, speed: f64) -> Result<Vec<u8>, VoiceError> {
            self.requests.lock().unwrap().push((text.to_owned(), speed));
            self.answer.clone()
        }
    }

    #[test]
    fn the_status_names_the_voice_chosen_by_the_ipad() {
        let voice = SystemVoiceSynthesizer::new(bridge(Some(thomas()), Ok(vec![1])));

        let status = voice.status();

        assert!(status.is_available);
        assert_eq!(Some("Thomas".to_owned()), status.voice);
        assert!(!status.voice_to_install);
    }

    #[test]
    fn without_a_french_voice_the_parent_is_asked_to_install_one() {
        let voice = SystemVoiceSynthesizer::new(bridge(None, Ok(vec![1])));

        let status = voice.status();

        assert!(!status.is_available);
        assert!(status.voice_to_install);
        assert!(status.problem.unwrap().contains("No French"));
    }

    #[test]
    fn a_plugin_that_fails_makes_the_voice_unavailable_without_asking_to_install() {
        let mut failing = bridge(Some(thomas()), Ok(vec![1]));
        failing.voices = Err(VoiceError("plugin not registered".to_owned()));
        let voice = SystemVoiceSynthesizer::new(failing);

        let status = voice.status();

        assert!(!status.is_available);
        assert!(!status.voice_to_install);
    }

    #[test]
    fn the_text_and_the_speed_of_the_core_are_passed_unchanged() {
        let voice = SystemVoiceSynthesizer::new(bridge(Some(thomas()), Ok(vec![1, 2, 3])));

        let wav = voice.synthesize("Le mot : jaune.", 0.85).unwrap();

        assert_eq!(vec![1, 2, 3], wav);
        assert_eq!(
            vec![("Le mot : jaune.".to_owned(), 0.85)],
            *voice.bridge.requests.lock().unwrap()
        );
    }

    #[test]
    fn a_failed_synthesis_is_a_missing_voice_for_the_core() {
        let voice = SystemVoiceSynthesizer::new(bridge(
            Some(thomas()),
            Err(VoiceError("timeout".to_owned())),
        ));

        let error = voice.synthesize("un stylo.", 1.0).unwrap_err();

        assert!(matches!(error, educator_core::Error::SpeechUnavailable(ref m) if m == "timeout"));
    }

    #[test]
    fn the_warm_up_reads_a_short_text_and_never_fails() {
        let voice = SystemVoiceSynthesizer::new(bridge(
            Some(thomas()),
            Err(VoiceError("timeout".to_owned())),
        ));

        voice.warm_up(0.9);

        assert_eq!(
            vec![(WARM_UP_TEXT.to_owned(), 0.9)],
            *voice.bridge.requests.lock().unwrap()
        );
    }
}
