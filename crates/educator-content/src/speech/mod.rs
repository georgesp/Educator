//! Piper voice on the desktop (SUB000111, ADR 0002, ADR 0004): locating the engine and the voice,
//! the long-lived `piper --json-input` processes (one per speed), and the disk cache of the
//! sentences already read.

mod cache;
mod locator;
mod piper;

pub use cache::{SpeechCache, MAX_BYTES};
pub use locator::{
    locate, PiperInstallation, PiperLocatorOptions, PiperPlatform, DEFAULT_VOICE, ROOT_VARIABLE,
    VOICE_VARIABLE,
};
pub use piper::{
    build_input_line, to_length_scale, PiperSpeechSynthesizer, IDLE_TIMEOUT, MAX_TEXT_LENGTH,
};

use educator_core::speech::{SpeechSynthesizer, UnavailableSpeechSynthesizer};
use std::sync::Arc;

/// Piper when it is found, otherwise a synthesizer reporting why the voice is unavailable.
pub fn create_synthesizer(options: &PiperLocatorOptions) -> Arc<dyn SpeechSynthesizer> {
    match locate(options) {
        Ok(installation) => {
            log::info!(
                "Piper found: {}, voice {}.",
                installation.executable_path.display(),
                installation.voice_name
            );
            let piper = Arc::new(PiperSpeechSynthesizer::new(installation));
            piper.watch_idle_processes();
            piper
        }
        Err(problem) => {
            log::warn!("Speech synthesis unavailable: {problem}");
            Arc::new(UnavailableSpeechSynthesizer::new(problem))
        }
    }
}
