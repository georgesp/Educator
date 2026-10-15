//! Disk cache of the synthesized sentences (SUB000111, SUB026701): a sentence already read once is
//! served from a WAV file instead of running Piper again. Key: SHA-256 of voice + speed + text, so
//! that another voice or another pronunciation never serves an old file. The cache is capped in
//! size ([`MAX_BYTES`] by default); beyond, the files least recently **read** go first. It is
//! disposable: any read or write problem falls back to the engine.

use educator_core::clock::{uuid_v7, Clock, SystemClock};
use educator_core::speech::{SpeechEngineStatus, SpeechSynthesizer};
use educator_core::Result;
use sha2::{Digest, Sha256};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Default size cap: 30 MB (about 600 words and sentences at the speed of the dictation).
pub const MAX_BYTES: u64 = 30 * 1024 * 1024;

/// The size of the folder is counted again every this many writes.
const RECOUNT_EVERY: usize = 100;

pub struct SpeechCache {
    engine: Arc<dyn SpeechSynthesizer>,
    directory: PathBuf,
    max_bytes: u64,
    clock: Arc<dyn Clock>,
    /// Size of the cached files, counted at the first write (`None` before).
    size: Mutex<Option<u64>>,
    writes: AtomicUsize,
}

impl fmt::Debug for SpeechCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpeechCache")
            .field("directory", &self.directory)
            .field("max_bytes", &self.max_bytes)
            .finish_non_exhaustive()
    }
}

impl SpeechCache {
    pub fn new(engine: Arc<dyn SpeechSynthesizer>, directory: PathBuf) -> Self {
        Self::with_limit(engine, directory, MAX_BYTES, Arc::new(SystemClock))
    }

    /// With another size cap and an injected clock (tests).
    pub fn with_limit(
        engine: Arc<dyn SpeechSynthesizer>,
        directory: PathBuf,
        max_bytes: u64,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            engine,
            directory,
            max_bytes,
            clock,
            size: Mutex::new(None),
            writes: AtomicUsize::new(0),
        }
    }

    pub fn directory(&self) -> &Path {
        &self.directory
    }

    /// Total size of the cached files, in bytes.
    pub fn size(&self) -> u64 {
        self.files().iter().map(|f| f.bytes).sum()
    }

    /// Removes the least recently read files until the cache is under 90 % of its cap (when it is
    /// above the cap). Only touches this cache's .wav files.
    pub fn prune(&self) {
        let mut size = self.size.lock().unwrap_or_else(|e| e.into_inner());
        *size = Some(self.prune_files());
    }

    fn prune_files(&self) -> u64 {
        let mut files = self.files();
        let mut total: u64 = files.iter().map(|f| f.bytes).sum();
        if total <= self.max_bytes {
            return total;
        }

        // Oldest use first; same instant: by name, for a stable order.
        files.sort_by(|a, b| a.used.cmp(&b.used).then_with(|| a.path.cmp(&b.path)));
        let target = self.max_bytes / 10 * 9;
        for file in files {
            if total <= target {
                break;
            }
            match std::fs::remove_file(&file.path) {
                Ok(()) => total = total.saturating_sub(file.bytes),
                Err(e) => log::warn!("Speech cache: cannot remove {}: {e}", file.path.display()),
            }
        }

        total
    }

    fn files(&self) -> Vec<CachedFile> {
        let Ok(entries) = std::fs::read_dir(&self.directory) else {
            return Vec::new();
        };

        entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|e| e == "wav"))
            .filter_map(|path| {
                let metadata = std::fs::metadata(&path).ok()?;
                Some(CachedFile {
                    used: metadata.modified().unwrap_or(UNIX_EPOCH),
                    bytes: metadata.len(),
                    path,
                })
            })
            .collect()
    }

    pub fn key(voice: &str, speed: f64, text: &str) -> String {
        let material = format!("{voice}\n{}\n{text}", format_speed(speed));
        Sha256::digest(material.as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }

    /// Path of the cached reading of a text, for the current voice.
    fn path_of(&self, text: &str, speed: f64) -> PathBuf {
        let voice = self
            .engine
            .status()
            .voice
            .unwrap_or_else(|| "none".to_owned());
        self.directory
            .join(format!("{}.wav", Self::key(&voice, speed, text)))
    }

    /// Whether a text is already in the cache (prepared words, SUB026705).
    pub fn contains(&self, text: &str, speed: f64) -> bool {
        self.path_of(text, speed).is_file()
    }

    /// Removes the cached reading of a text, so that the next one is synthesized again (Piper
    /// draws at random: a failed reading can be done again, SUB026706).
    pub fn forget(&self, text: &str, speed: f64) {
        let path = self.path_of(text, speed);
        if let Ok(metadata) = std::fs::metadata(&path) {
            if std::fs::remove_file(&path).is_ok() {
                let mut size = self.size.lock().unwrap_or_else(|e| e.into_inner());
                if let Some(total) = size.as_mut() {
                    *total = total.saturating_sub(metadata.len());
                }
            }
        }
    }

    /// The instant of the injected clock as a file time.
    fn now(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_millis(u64::try_from(self.clock.now_ms()).unwrap_or(0))
    }

    /// Marks a file as just used: it stays longer in the cache.
    fn touch(&self, path: &Path) {
        let touched = std::fs::File::options()
            .write(true)
            .open(path)
            .and_then(|file| file.set_modified(self.now()));
        if let Err(e) = touched {
            log::debug!("Speech cache: cannot mark {} as read: {e}", path.display());
        }
    }

    fn store(&self, path: &Path, wav: &[u8]) {
        // A file written again (two requests of the same text at once) is not counted twice.
        let replaced = std::fs::metadata(path).map_or(0, |m| m.len());
        let write = || -> std::io::Result<()> {
            std::fs::create_dir_all(&self.directory)?;
            let temporary =
                path.with_extension(format!("{}.tmp", uuid_v7(SystemClock.now_ms()).simple()));
            std::fs::write(&temporary, wav)?;
            std::fs::rename(&temporary, path)
        };
        if let Err(e) = write() {
            log::warn!(
                "Speech cache unavailable for {}; using the engine: {e}",
                path.display()
            );
            return;
        }

        self.touch(path);
        let mut size = self.size.lock().unwrap_or_else(|e| e.into_inner());
        let writes = self.writes.fetch_add(1, Ordering::SeqCst) + 1;
        let total = match *size {
            // The first write counts what earlier runs left; every RECOUNT_EVERY writes, what
            // others wrote in the folder (scripts/listen-words) is counted again.
            Some(total) if writes % RECOUNT_EVERY != 0 => {
                (total + wav.len() as u64).saturating_sub(replaced)
            }
            _ => self.files().iter().map(|f| f.bytes).sum(),
        };
        *size = Some(if total > self.max_bytes {
            self.prune_files()
        } else {
            total
        });
    }
}

struct CachedFile {
    path: PathBuf,
    used: SystemTime,
    bytes: u64,
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
        let path = self.path_of(text, speed);
        if path.is_file() {
            match std::fs::read(&path) {
                Ok(bytes) => {
                    self.touch(&path);
                    return Ok(bytes);
                }
                Err(e) => log::warn!(
                    "Speech cache unavailable for {}; using the engine: {e}",
                    path.display()
                ),
            }
        }

        let wav = self.engine.synthesize(text, speed)?;
        self.store(&path, &wav);
        Ok(wav)
    }

    fn warm_up(&self, speed: f64) {
        self.engine.warm_up(speed);
    }
}
