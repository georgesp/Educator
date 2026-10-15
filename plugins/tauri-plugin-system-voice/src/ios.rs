//! Rust side of the Swift plugin (`ios/Sources`): registration and the two calls used by the
//! application. Calls are blocking: make them from a worker thread, never from the main thread.

use crate::{Error, Voices};
use serde::{Deserialize, Serialize};
use std::fmt;
use tauri::plugin::{Builder, PluginHandle, TauriPlugin};
use tauri::{Manager, Runtime};

tauri::ios_plugin_binding!(init_plugin_system_voice);

/// Registers the plugin; its handle is then in the state of the application ([`SystemVoice`]).
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("system-voice")
        .setup(|app, api| {
            let handle = api.register_ios_plugin(init_plugin_system_voice)?;
            app.manage(SystemVoice(handle));
            Ok(())
        })
        .build()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SynthesizeArgs<'a> {
    /// Plain text, read when the voice does not take SSML.
    text: &'a str,
    /// The same text in SSML, with a clear pause after each colon.
    ssml: &'a str,
    speed: f64,
    voice: Option<&'a str>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Synthesized {
    samples: String,
    sample_rate: u32,
    voice: String,
    milliseconds: u64,
}

/// Access to the system voice.
pub struct SystemVoice<R: Runtime>(PluginHandle<R>);

// Written by hand: a derived `Clone` would ask `R: Clone`, which `Wry` is not.
impl<R: Runtime> Clone for SystemVoice<R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<R: Runtime> fmt::Debug for SystemVoice<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SystemVoice")
    }
}

impl<R: Runtime> SystemVoice<R> {
    /// French voices of the device.
    pub fn voices(&self) -> Result<Voices, Error> {
        self.0
            .run_mobile_plugin("voices", ())
            .map_err(|e| Error(e.to_string()))
    }

    /// `text` read by the best French voice (or `voice`), at `speed` (1 = normal), as a WAV file.
    /// Colons get a clear pause ([`crate::COLON_PAUSE_MS`]) so that the core can cut the carrier
    /// « Le mot : … » before the word (ADR 0004).
    pub fn synthesize(
        &self,
        text: &str,
        speed: f64,
        voice: Option<&str>,
    ) -> Result<Vec<u8>, Error> {
        let ssml = crate::ssml_with_colon_pauses(text, crate::COLON_PAUSE_MS);
        let args = SynthesizeArgs {
            text,
            ssml: &ssml,
            speed,
            voice,
        };
        let synthesized: Synthesized = self
            .0
            .run_mobile_plugin("synthesize", args)
            .map_err(|e| Error(e.to_string()))?;
        log::debug!(
            "System voice {}: {} ms at {} Hz",
            synthesized.voice,
            synthesized.milliseconds,
            synthesized.sample_rate
        );
        crate::wav_from_base64_samples(&synthesized.samples, synthesized.sample_rate)
    }
}
