//! What the Swift side answers, and its decoding: shared by every target so that it is tested on the
//! desktop (the calls themselves exist on iOS only).

use base64::Engine;
use serde::Deserialize;
use std::fmt;

/// Failure of the system voice (no French voice, synthesis refused, timeout), in English.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(pub String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

/// A voice of the device.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Voice {
    pub id: String,
    pub name: String,
    pub language: String,
    /// "default", "enhanced" or "premium".
    pub quality: String,
}

/// French voices of the device and the one used by default (the best of France), if any.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Voices {
    pub voices: Vec<Voice>,
    pub chosen: Option<Voice>,
}

/// WAV file made of the samples sent by the Swift side (16-bit little-endian mono, in base64).
pub fn wav_from_base64_samples(samples: &str, sample_rate: u32) -> Result<Vec<u8>, Error> {
    if sample_rate == 0 {
        return Err(Error("The system voice gave no sample rate.".to_owned()));
    }
    let samples = base64::engine::general_purpose::STANDARD
        .decode(samples)
        .map_err(|e| Error(format!("Samples of the system voice unreadable: {e}")))?;
    if samples.is_empty() {
        return Err(Error("The system voice produced no sound.".to_owned()));
    }
    Ok(crate::wav_from_pcm16(&samples, sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples_in_base64_become_a_wav() {
        // Two samples: 1 and -1.
        let wav = wav_from_base64_samples("AQD//w==", 22_050).unwrap();

        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[44..], &[0x01, 0x00, 0xff, 0xff]);
    }

    #[test]
    fn unreadable_or_empty_samples_are_errors() {
        assert!(wav_from_base64_samples("not base64!", 22_050).is_err());
        assert!(wav_from_base64_samples("", 22_050).is_err());
        assert!(wav_from_base64_samples("AQD//w==", 0).is_err());
    }

    #[test]
    fn the_answer_of_voices_is_read() {
        let voices: Voices = serde_json::from_str(
            r#"{"voices":[{"id":"com.apple.voice.compact.fr-FR.Thomas","name":"Thomas","language":"fr-FR","quality":"default"}],
               "chosen":{"id":"com.apple.voice.compact.fr-FR.Thomas","name":"Thomas","language":"fr-FR","quality":"default"}}"#,
        )
        .unwrap();

        assert_eq!(1, voices.voices.len());
        assert_eq!(
            Some("Thomas"),
            voices.chosen.as_ref().map(|v| v.name.as_str())
        );
    }

    #[test]
    fn no_voice_chosen_reads_as_none() {
        let voices: Voices = serde_json::from_str(r#"{"voices":[],"chosen":null}"#).unwrap();

        assert!(voices.chosen.is_none());
    }
}
