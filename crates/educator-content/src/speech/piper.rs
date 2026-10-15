//! [`SpeechSynthesizer`] running the Piper command-line engine as long-lived child processes
//! (`--json-input`): the voice model is loaded once per process, then each request is one JSON line
//! on stdin; Piper writes a WAV file in a private folder and prints its path on stdout.
//!
//! `--length_scale` is a process-wide option (a speed field in the JSON request is ignored), so one
//! process is kept **per speed** (SUB026708): going from an instruction to the dictated word, then
//! to the slow listening, never restarts Piper. Requests at one speed are served one at a time;
//! different speeds run side by side. The speeds given to [`SpeechSynthesizer::warm_up`] stay ready
//! for good; any other process stops after [`IDLE_TIMEOUT`] without a request (≈ 110 MB each). A
//! process that crashed or did not answer is restarted at the next request.

use super::PiperInstallation;
use super::PiperPlatform;
use educator_core::clock::{uuid_v7, Clock, SystemClock};
use educator_core::speech::{speed, SpeechEngineStatus, SpeechSynthesizer};
use educator_core::{Error, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::sync::{Arc, Mutex, Weak};
use std::time::{Duration, Instant};

pub const MAX_TEXT_LENGTH: usize = 1000;

/// A process not kept warm stops after this long without a request.
pub const IDLE_TIMEOUT: Duration = Duration::from_secs(120);

/// How often the idle processes are looked for ([`PiperSpeechSynthesizer::watch_idle_processes`]).
const IDLE_CHECK_EVERY: Duration = Duration::from_secs(15);

const REQUEST_TIMEOUT: Duration = Duration::from_secs(20);

/// A speed as Piper sees it: the length scale in hundredths (speed 0.85 → length 1.18 → 118).
type ScaleKey = u32;

struct Running {
    child: Child,
    stdin: ChildStdin,
    lines: Receiver<std::io::Result<String>>,
}

/// The process of one speed and its private output folder.
struct Slot {
    length_scale: f64,
    output_directory: PathBuf,
    running: Mutex<Option<Running>>,
    /// Last request (start or end), Unix milliseconds of the injected clock.
    last_used_ms: AtomicI64,
}

pub struct PiperSpeechSynthesizer {
    installation: PiperInstallation,
    output_root: PathBuf,
    clock: Arc<dyn Clock>,
    slots: Mutex<BTreeMap<ScaleKey, Arc<Slot>>>,
    /// Speeds warmed up: never stopped for idleness.
    kept_warm: Mutex<BTreeSet<ScaleKey>>,
    starts: AtomicUsize,
    /// Why the last warm-up failed (Piper cannot start: Rosetta missing, broken files…), until a
    /// synthesis succeeds: the voice is then reported unavailable (SUB026704).
    problem: Mutex<Option<String>>,
}

impl fmt::Debug for PiperSpeechSynthesizer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PiperSpeechSynthesizer")
            .field("installation", &self.installation)
            .finish_non_exhaustive()
    }
}

impl PiperSpeechSynthesizer {
    pub fn new(installation: PiperInstallation) -> Self {
        Self::with_clock(installation, Arc::new(SystemClock))
    }

    /// With an injected clock (tests of the idle processes).
    pub fn with_clock(installation: PiperInstallation, clock: Arc<dyn Clock>) -> Self {
        let output_root = std::env::temp_dir()
            .join("educator-piper")
            .join(uuid_v7(SystemClock.now_ms()).simple().to_string());
        Self {
            installation,
            output_root,
            clock,
            slots: Mutex::new(BTreeMap::new()),
            kept_warm: Mutex::new(BTreeSet::new()),
            starts: AtomicUsize::new(0),
            problem: Mutex::new(None),
        }
    }

    /// Stops, in the background, the processes idle for [`IDLE_TIMEOUT`]. The watch ends with the
    /// synthesizer.
    pub fn watch_idle_processes(self: &Arc<Self>) {
        let synthesizer: Weak<Self> = Arc::downgrade(self);
        let spawned = std::thread::Builder::new()
            .name("piper-idle".to_owned())
            .spawn(move || loop {
                std::thread::sleep(IDLE_CHECK_EVERY);
                match synthesizer.upgrade() {
                    Some(synthesizer) => synthesizer.stop_idle_processes(),
                    None => return,
                }
            });
        if let Err(e) = spawned {
            log::warn!("Piper idle watch not started: {e}");
        }
    }

    /// Stops the processes not kept warm and idle for [`IDLE_TIMEOUT`] (a busy one is left alone).
    pub fn stop_idle_processes(&self) {
        let now = self.clock.now_ms();
        let idle_ms = i64::try_from(IDLE_TIMEOUT.as_millis()).unwrap_or(i64::MAX);
        let kept_warm = self
            .kept_warm
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone();
        let slots: Vec<(ScaleKey, Arc<Slot>)> = self
            .slots
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .map(|(key, slot)| (*key, Arc::clone(slot)))
            .collect();
        for (key, slot) in slots {
            if kept_warm.contains(&key)
                || now.saturating_sub(slot.last_used_ms.load(Ordering::SeqCst)) < idle_ms
            {
                continue;
            }

            let Ok(mut running) = slot.running.try_lock() else {
                continue;
            };
            if let Some(process) = running.take() {
                log::info!(
                    "Piper stopped after {} s without a request (length scale {}).",
                    IDLE_TIMEOUT.as_secs(),
                    slot.length_scale
                );
                stop(Some(process));
            }
        }
    }

    /// The speeds whose process is running now (length scales), slowest last.
    pub fn running_length_scales(&self) -> Vec<f64> {
        self.slots
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .values()
            .filter(|slot| {
                slot.running
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .is_some()
            })
            .map(|slot| slot.length_scale)
            .collect()
    }

    /// How many Piper processes were started since the beginning (diagnostics, tests).
    pub fn process_starts(&self) -> usize {
        self.starts.load(Ordering::SeqCst)
    }

    fn slot(&self, length_scale: f64) -> Arc<Slot> {
        let key = scale_key(length_scale);
        let mut slots = self.slots.lock().unwrap_or_else(|e| e.into_inner());
        Arc::clone(slots.entry(key).or_insert_with(|| {
            Arc::new(Slot {
                length_scale,
                output_directory: self.output_root.join(key.to_string()),
                running: Mutex::new(None),
                last_used_ms: AtomicI64::new(self.clock.now_ms()),
            })
        }))
    }

    fn synthesize_in(&self, slot: &Slot, text: &str) -> Result<Vec<u8>> {
        let mut running = slot.running.lock().unwrap_or_else(|e| e.into_inner());
        slot.last_used_ms
            .store(self.clock.now_ms(), Ordering::SeqCst);
        let outcome = self.synthesize_locked(slot, &mut running, text);
        slot.last_used_ms
            .store(self.clock.now_ms(), Ordering::SeqCst);
        outcome
    }

    fn synthesize_locked(
        &self,
        slot: &Slot,
        running: &mut Option<Running>,
        text: &str,
    ) -> Result<Vec<u8>> {
        if running
            .as_mut()
            .is_none_or(|r| !matches!(r.child.try_wait(), Ok(None)))
        {
            stop(running.take());
            *running = Some(self.start(slot)?);
        }

        let process = running.as_mut().expect("started above");
        let outcome = (|| {
            clear_directory(&slot.output_directory);
            writeln!(process.stdin, "{}", build_input_line(text))
                .and_then(|()| process.stdin.flush())
                .map_err(|e| unavailable(format!("Piper failed: {e}")))?;

            // The printed path is only a signal: the file is looked up in our private folder, so that
            // a path with non-ASCII characters printed in the console encoding cannot break us.
            match process.lines.recv_timeout(REQUEST_TIMEOUT) {
                Ok(Ok(line)) => log::debug!("Piper wrote {line}."),
                Ok(Err(e)) => return Err(unavailable(format!("Piper failed: {e}"))),
                Err(RecvTimeoutError::Timeout) => {
                    return Err(unavailable(format!(
                        "Piper did not answer within {} s.",
                        REQUEST_TIMEOUT.as_secs()
                    )))
                }
                Err(RecvTimeoutError::Disconnected) => {
                    let code = process
                        .child
                        .try_wait()
                        .ok()
                        .flatten()
                        .map_or("running".to_owned(), |s| s.to_string());
                    return Err(unavailable(format!(
                        "Piper exited unexpectedly (exit code {code})."
                    )));
                }
            }

            let wav = std::fs::read_dir(&slot.output_directory)
                .ok()
                .and_then(|entries| {
                    entries
                        .flatten()
                        .map(|e| e.path())
                        .find(|p| p.extension().is_some_and(|e| e == "wav"))
                })
                .ok_or_else(|| unavailable("Piper did not write any audio file."))?;
            let bytes =
                std::fs::read(&wav).map_err(|e| unavailable(format!("Piper failed: {e}")))?;
            let _ = std::fs::remove_file(&wav);
            Ok(bytes)
        })();

        if outcome.is_err() {
            // Piper may still be working on that sentence and print its line later: restart it,
            // otherwise the next request would read that stale answer.
            stop(running.take());
        }

        outcome
    }

    fn start(&self, slot: &Slot) -> Result<Running> {
        std::fs::create_dir_all(&slot.output_directory)
            .map_err(|e| unavailable(format!("Piper output folder: {e}")))?;
        let installation = &self.installation;
        let length_scale = slot.length_scale;
        let mut command = Command::new(&installation.executable_path);
        command
            .current_dir(installation.engine_directory())
            .arg("--model")
            .arg(&installation.model_path)
            .arg("--config")
            .arg(&installation.model_config_path)
            .arg("--espeak_data")
            .arg(&installation.espeak_data_directory)
            .arg("--output_dir")
            .arg(&slot.output_directory)
            .arg("--length_scale")
            .arg(length_scale.to_string())
            .arg("--json-input")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if let Some(variable) = PiperPlatform::current().library_path_variable {
            command.env(variable, installation.engine_directory());
        }

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            command.creation_flags(CREATE_NO_WINDOW);
        }

        let mut child = command.spawn().map_err(|e| {
            unavailable(format!(
                "Could not start Piper ({}): {e}",
                installation.executable_path.display()
            ))
        })?;
        self.starts.fetch_add(1, Ordering::SeqCst);
        let stdin = child.stdin.take().expect("piped");
        let stdout = child.stdout.take().expect("piped");
        let stderr = child.stderr.take().expect("piped");

        let (sender, lines) = mpsc::channel();
        std::thread::Builder::new()
            .name("piper-stdout".to_owned())
            .spawn(move || {
                for line in BufReader::new(stdout).lines() {
                    if sender.send(line).is_err() {
                        break;
                    }
                }
            })
            .map_err(|e| unavailable(format!("Piper reader: {e}")))?;
        std::thread::Builder::new()
            .name("piper-stderr".to_owned())
            .spawn(move || {
                for line in BufReader::new(stderr)
                    .lines()
                    .map_while(std::io::Result::ok)
                {
                    if !line.trim().is_empty() {
                        log::debug!("[piper] {line}");
                    }
                }
            })
            .map_err(|e| unavailable(format!("Piper reader: {e}")))?;

        log::info!(
            "Piper started (pid {}, voice {}, length scale {length_scale}).",
            child.id(),
            installation.voice_name
        );
        Ok(Running {
            child,
            stdin,
            lines,
        })
    }
}

impl SpeechSynthesizer for PiperSpeechSynthesizer {
    fn status(&self) -> SpeechEngineStatus {
        match self
            .problem
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
        {
            None => SpeechEngineStatus::available(self.installation.voice_name.clone()),
            // The voice is known (keys of the cache) but does not work.
            Some(problem) => SpeechEngineStatus {
                is_available: false,
                voice: Some(self.installation.voice_name.clone()),
                problem: Some(problem),
                voice_to_install: false,
            },
        }
    }

    fn synthesize(&self, text: &str, speed: f64) -> Result<Vec<u8>> {
        if text.trim().is_empty() || text.chars().count() > MAX_TEXT_LENGTH {
            return Err(unavailable(format!(
                "Text of 1 to {MAX_TEXT_LENGTH} characters expected."
            )));
        }

        let slot = self.slot(to_length_scale(speed));
        let outcome = self.synthesize_in(&slot, text);
        if outcome.is_ok() {
            *self.problem.lock().unwrap_or_else(|e| e.into_inner()) = None;
        }
        outcome
    }

    /// Starts the process of this speed and keeps it ready for good.
    fn warm_up(&self, speed: f64) {
        let length_scale = to_length_scale(speed);
        self.kept_warm
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(scale_key(length_scale));
        let started = Instant::now();
        match self.synthesize("Bonjour.", speed) {
            Ok(_) => log::info!(
                "Piper ready with voice {} at length scale {length_scale} in {} ms.",
                self.installation.voice_name,
                started.elapsed().as_millis()
            ),
            Err(e) => {
                log::warn!("Piper warm-up failed: {e}");
                *self.problem.lock().unwrap_or_else(|e| e.into_inner()) = Some(e.to_string());
            }
        }
    }
}

impl Drop for PiperSpeechSynthesizer {
    fn drop(&mut self) {
        let slots = std::mem::take(self.slots.get_mut().unwrap_or_else(|e| e.into_inner()));
        for slot in slots.into_values() {
            stop(
                slot.running
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .take(),
            );
        }
        let _ = std::fs::remove_dir_all(&self.output_root);
    }
}

fn scale_key(length_scale: f64) -> ScaleKey {
    // Length scales are rounded to hundredths (to_length_scale), from 0.67 to 2.
    (length_scale * 100.0).round() as ScaleKey
}

fn stop(running: Option<Running>) {
    let Some(Running {
        mut child, stdin, ..
    }) = running
    else {
        return;
    };

    // Closing stdin asks Piper to exit; it is killed if it does not within 2 s.
    drop(stdin);
    let deadline = Instant::now() + Duration::from_secs(2);
    while Instant::now() < deadline {
        match child.try_wait() {
            Ok(Some(_)) | Err(_) => return,
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
        }
    }

    let _ = child.kill();
    let _ = child.wait();
}

fn clear_directory(directory: &std::path::Path) {
    if let Ok(entries) = std::fs::read_dir(directory) {
        for entry in entries.flatten() {
            let _ = std::fs::remove_file(entry.path());
        }
    }
}

fn unavailable(message: impl Into<String>) -> Error {
    Error::SpeechUnavailable(message.into())
}

/// One input line of `piper --json-input`. Non-ASCII characters are escaped (`é`), so the
/// line is pure ASCII whatever the console encoding of the OS; line breaks become spaces.
pub fn build_input_line(text: &str) -> String {
    let single_line = text.replace("\r\n", " ").replace(
        ['\r', '\n', '\u{0085}', '\u{2028}', '\u{2029}', '\u{000C}'],
        " ",
    );
    let mut line = String::from("{\"text\":\"");
    for c in single_line.chars() {
        match c {
            '"' => line.push_str("\\\""),
            '\\' => line.push_str("\\\\"),
            c if c.is_ascii() && !c.is_ascii_control() => line.push(c),
            c => {
                let mut units = [0u16; 2];
                for unit in c.encode_utf16(&mut units) {
                    line.push_str(&format!("\\u{unit:04x}"));
                }
            }
        }
    }

    line.push_str("\"}");
    line
}

/// Piper's `--length_scale` is a duration factor: speed 2 = length 0.5.
pub fn to_length_scale(speed: f64) -> f64 {
    let clamped = if speed.is_finite() {
        speed.clamp(speed::MIN, speed::MAX)
    } else {
        speed::NORMAL
    };
    (100.0 / clamped).round() / 100.0
}
