//! System voice of the iPad for Educator (SUB023805, ADR 0005): `AVSpeechSynthesizer` called from
//! Rust through a local Tauri plugin (Swift side in `ios/`). The voice returns 16-bit PCM samples,
//! wrapped here in a WAV file, the format the core already gets from Piper on the desktop.
//!
//! The plugin only exists on iOS. On other targets the crate holds the models and helpers only, so
//! that the workspace builds and tests everywhere.

mod models;
mod ssml;
mod wav;

pub use models::{wav_from_base64_samples, Error, Voice, Voices};
pub use ssml::{ssml_with_colon_pauses, COLON_PAUSE_MS};
pub use wav::wav_from_pcm16;

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "ios")]
pub use ios::{init, SystemVoice};
