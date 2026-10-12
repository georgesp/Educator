//! [`SpeechSynthesizer`] running the Piper command-line engine as a long-lived child process
//! (`--json-input`): the voice model is loaded once, then each request is one JSON line on stdin;
//! Piper writes a WAV file in a private folder and prints its path on stdout. Requests are
//! serialized (one at a time). The process is restarted after a crash, a timeout or a change of
//! speed (`--length_scale` is a process-wide option).

use super::PiperInstallation;
use super::PiperPlatform;
use educator_core::clock::{uuid_v7, Clock, SystemClock};
use educator_core::speech::{speed, SpeechEngineStatus, SpeechSynthesizer};
use educator_core::{Error, Result};
use std::fmt;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub const MAX_TEXT_LENGTH: usize = 1000;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(20);

struct Running {
    child: Child,
    stdin: ChildStdin,
    lines: Receiver<std::io::Result<String>>,
    length_scale: f64,
}

pub struct PiperSpeechSynthesizer {
    installation: PiperInstallation,
    output_directory: PathBuf,
    process: Mutex<Option<Running>>,
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
        let output_directory = std::env::temp_dir()
            .join("educator-piper")
            .join(uuid_v7(SystemClock.now_ms()).simple().to_string());
        Self {
            installation,
            output_directory,
            process: Mutex::new(None),
        }
    }

    fn synthesize_locked(
        &self,
        slot: &mut Option<Running>,
        text: &str,
        length_scale: f64,
    ) -> Result<Vec<u8>> {
        if slot.as_mut().is_none_or(|r| {
            r.length_scale != length_scale || !matches!(r.child.try_wait(), Ok(None))
        }) {
            stop(slot.take());
            *slot = Some(self.start(length_scale)?);
        }

        let running = slot.as_mut().expect("started above");
        let outcome = (|| {
            clear_directory(&self.output_directory);
            writeln!(running.stdin, "{}", build_input_line(text))
                .and_then(|()| running.stdin.flush())
                .map_err(|e| unavailable(format!("Piper failed: {e}")))?;

            // The printed path is only a signal: the file is looked up in our private folder, so that
            // a path with non-ASCII characters printed in the console encoding cannot break us.
            match running.lines.recv_timeout(REQUEST_TIMEOUT) {
                Ok(Ok(line)) => log::debug!("Piper wrote {line}."),
                Ok(Err(e)) => return Err(unavailable(format!("Piper failed: {e}"))),
                Err(RecvTimeoutError::Timeout) => {
                    return Err(unavailable(format!(
                        "Piper did not answer within {} s.",
                        REQUEST_TIMEOUT.as_secs()
                    )))
                }
                Err(RecvTimeoutError::Disconnected) => {
                    let code = running
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

            let wav = std::fs::read_dir(&self.output_directory)
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
            stop(slot.take());
        }

        outcome
    }

    fn start(&self, length_scale: f64) -> Result<Running> {
        std::fs::create_dir_all(&self.output_directory)
            .map_err(|e| unavailable(format!("Piper output folder: {e}")))?;
        let installation = &self.installation;
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
            .arg(&self.output_directory)
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
            length_scale,
        })
    }
}

impl SpeechSynthesizer for PiperSpeechSynthesizer {
    fn status(&self) -> SpeechEngineStatus {
        SpeechEngineStatus::available(self.installation.voice_name.clone())
    }

    fn synthesize(&self, text: &str, speed: f64) -> Result<Vec<u8>> {
        if text.trim().is_empty() || text.chars().count() > MAX_TEXT_LENGTH {
            return Err(unavailable(format!(
                "Text of 1 to {MAX_TEXT_LENGTH} characters expected."
            )));
        }

        let length_scale = to_length_scale(speed);
        let mut slot = self.process.lock().unwrap_or_else(|e| e.into_inner());
        self.synthesize_locked(&mut slot, text, length_scale)
    }

    fn warm_up(&self, speed: f64) {
        let started = Instant::now();
        match self.synthesize("Bonjour.", speed) {
            Ok(_) => log::info!(
                "Piper ready with voice {} in {} ms.",
                self.installation.voice_name,
                started.elapsed().as_millis()
            ),
            Err(e) => log::warn!("Piper warm-up failed: {e}"),
        }
    }
}

impl Drop for PiperSpeechSynthesizer {
    fn drop(&mut self) {
        stop(
            self.process
                .get_mut()
                .unwrap_or_else(|e| e.into_inner())
                .take(),
        );
        let _ = std::fs::remove_dir_all(&self.output_directory);
    }
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
