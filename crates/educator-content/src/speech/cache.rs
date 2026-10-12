//! Disk cache of the synthesized sentences (SUB000111): a sentence already read once is served from
//! a WAV file instead of running Piper again. Key: SHA-256 of voice + speed + text. The cache is
//! disposable: any read or write problem falls back to the engine.

use educator_core::clock::{uuid_v7, Clock, SystemClock};
use educator_core::speech::{SpeechEngineStatus, SpeechSynthesizer};
use educator_core::Result;
use sha2::{Digest, Sha256};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Files kept at most (about 50 KB each); beyond, the least recently written are removed.
pub const MAX_FILES: usize = 2000;

/// The size is checked every this many writes.
const PRUNE_EVERY: usize = 50;

pub struct SpeechCache {
    engine: Arc<dyn SpeechSynthesizer>,
    directory: PathBuf,
    writes: AtomicUsize,
}

impl fmt::Debug for SpeechCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpeechCache")
            .field("directory", &self.directory)
            .finish_non_exhaustive()
    }
}

impl SpeechCache {
    pub fn new(engine: Arc<dyn SpeechSynthesizer>, directory: PathBuf) -> Self {
        Self {
            engine,
            directory,
            writes: AtomicUsize::new(0),
        }
    }

    pub fn directory(&self) -> &Path {
        &self.directory
    }

    /// Removes the oldest files down to 90 % of [`MAX_FILES`]. Only touches this cache's .wav files.
    pub fn prune(&self) {
        let Ok(entries) = std::fs::read_dir(&self.directory) else {
            return;
        };

        let mut files: Vec<(std::time::SystemTime, PathBuf)> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|e| e == "wav"))
            .map(|p| {
                (
                    std::fs::metadata(&p)
                        .and_then(|m| m.modified())
                        .unwrap_or(std::time::UNIX_EPOCH),
                    p,
                )
            })
            .collect();
        if files.len() <= MAX_FILES {
            return;
        }

        files.sort();
        let excess = files.len() - MAX_FILES * 9 / 10;
        for (_, path) in files.into_iter().take(excess) {
            if let Err(e) = std::fs::remove_file(&path) {
                log::warn!("Speech cache: cannot remove {}: {e}", path.display());
            }
        }
    }

    pub fn key(voice: &str, speed: f64, text: &str) -> String {
        let material = format!("{voice}\n{}\n{text}", format_speed(speed));
        Sha256::digest(material.as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }
}

/// `0.###`: at most three decimals, without trailing zeros (0.9 → "0.9", 1 → "1").
fn format_speed(speed: f64) -> String {
    let text = format!("{speed:.3}");
    text.trim_end_matches('0').trim_end_matches('.').to_owned()
}

impl SpeechSynthesizer for SpeechCache {
    fn status(&self) -> SpeechEngineStatus {
        self.engine.status()
    }

    fn synthesize(&self, text: &str, speed: f64) -> Result<Vec<u8>> {
        let voice = self
            .engine
            .status()
            .voice
            .unwrap_or_else(|| "none".to_owned());
        let path = self
            .directory
            .join(format!("{}.wav", Self::key(&voice, speed, text)));
        if path.is_file() {
            match std::fs::read(&path) {
                Ok(bytes) => return Ok(bytes),
                Err(e) => log::warn!(
                    "Speech cache unavailable for {}; using the engine: {e}",
                    path.display()
                ),
            }
        }

        let wav = self.engine.synthesize(text, speed)?;
        let write = || -> std::io::Result<()> {
            std::fs::create_dir_all(&self.directory)?;
            let temporary =
                path.with_extension(format!("{}.tmp", uuid_v7(SystemClock.now_ms()).simple()));
            std::fs::write(&temporary, &wav)?;
            std::fs::rename(&temporary, &path)
        };
        match write() {
            Ok(()) => {
                if (self.writes.fetch_add(1, Ordering::SeqCst) + 1) % PRUNE_EVERY == 0 {
                    self.prune();
                }
            }
            Err(e) => log::warn!(
                "Speech cache unavailable for {}; using the engine: {e}",
                path.display()
            ),
        }

        Ok(wav)
    }

    fn warm_up(&self, speed: f64) {
        self.engine.warm_up(speed);
    }
}
