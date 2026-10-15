//! Start-up of the application core, in the background so that the window shows at once:
//! the front polls `get_startup` until it is ready or has failed.

use crate::contracts::{CommandError, StartupStatus};
use crate::services::{Locations, Services};
use educator_core::clock::SystemClock;
use educator_core::random::ThreadRandom;
use educator_core::skills::Date;
use educator_core::speech::SpeechSynthesizer;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

enum Startup {
    Starting,
    Ready(Arc<Services>),
    Failed(String),
}

pub struct AppState(RwLock<Startup>);

impl Default for AppState {
    fn default() -> Self {
        Self(RwLock::new(Startup::Starting))
    }
}

impl AppState {
    pub fn status(&self) -> StartupStatus {
        match &*self.0.read().unwrap_or_else(|e| e.into_inner()) {
            Startup::Starting => StartupStatus::Starting,
            Startup::Ready(_) => StartupStatus::Ready,
            Startup::Failed(message) => StartupStatus::Failed {
                message: message.clone(),
            },
        }
    }

    pub fn services(&self) -> Result<Arc<Services>, CommandError> {
        match &*self.0.read().unwrap_or_else(|e| e.into_inner()) {
            Startup::Ready(services) => Ok(Arc::clone(services)),
            Startup::Starting => Err(CommandError::new(
                503,
                "Starting",
                "The application is starting.",
            )),
            Startup::Failed(message) => {
                Err(CommandError::new(503, "Start-up failed", message.clone()))
            }
        }
    }

    fn set(&self, startup: Startup) {
        *self.0.write().unwrap_or_else(|e| e.into_inner()) = startup;
    }
}

/// Folders given by Tauri: the resources of the packaged application and the folder of the executable.
pub struct Folders {
    pub resources: Option<PathBuf>,
    pub executable: PathBuf,
    pub data: PathBuf,
}

/// Loads the content, opens the database and finds the voice, then warms the voice up.
pub fn start(state: Arc<AppState>, folders: Folders) {
    let spawned = std::thread::Builder::new()
        .name("educator-startup".to_owned())
        .spawn(move || {
            let candidates: Vec<PathBuf> = folders
                .resources
                .iter()
                .map(|r| r.join(educator_content::FOLDER_NAME))
                .chain(std::iter::once(
                    folders.executable.join(educator_content::FOLDER_NAME),
                ))
                .collect();
            let Some(content_directory) =
                educator_content::find_content_directory(&candidates, &folders.executable)
            else {
                state.set(Startup::Failed(format!(
                    "Pedagogical content not found (set {} or run from the repository).",
                    educator_content::CONTENT_DIR_VARIABLE
                )));
                return;
            };

            let speech = speech_synthesizer(&folders);
            let locations = Locations {
                content_directory,
                database_path: folders.data.join(educator_data::DATABASE_FILE_NAME),
            };

            match Services::start(
                &locations,
                speech.clone(),
                Arc::new(ThreadRandom),
                Arc::new(SystemClock),
                Box::new(today),
            ) {
                Ok(services) => {
                    let services = Arc::new(services.with_local_date(Box::new(local_date)));
                    state.set(Startup::Ready(Arc::clone(&services)));
                    // The instructions and the dictated words are then read without delay: one
                    // Piper process each, kept ready (SUB026708).
                    speech.warm_up(educator_core::speech::speed::INSTRUCTIONS);
                    speech.warm_up(educator_core::speech::speed::WORDS);
                    // A voice that could not start counts as missing (SUB026704).
                    services.refresh_listening();
                }
                Err(message) => {
                    log::error!("Start-up failed: {message}");
                    state.set(Startup::Failed(message));
                }
            }
        });

    if let Err(e) = spawned {
        log::error!("Start-up thread: {e}");
    }
}

/// Piper with its disk cache on the desktop (ADR 0002, ADR 0004).
#[cfg(desktop)]
fn speech_synthesizer(folders: &Folders) -> Arc<dyn SpeechSynthesizer> {
    use educator_content::speech::{create_synthesizer, PiperLocatorOptions, SpeechCache};
    let piper = PiperLocatorOptions::from_environment(
        folders.resources.iter().cloned().collect(),
        folders.executable.clone(),
    );
    Arc::new(SpeechCache::new(
        create_synthesizer(&piper),
        folders.data.join("speech-cache"),
    ))
}

/// No voice on tablets until the tablet lot.
#[cfg(mobile)]
fn speech_synthesizer(_folders: &Folders) -> Arc<dyn SpeechSynthesizer> {
    Arc::new(educator_core::speech::UnavailableSpeechSynthesizer::new(
        "No voice on tablets yet.",
    ))
}

/// Local date of the computer.
fn today() -> Date {
    use chrono::Datelike;
    let now = chrono::Local::now().date_naive();
    Date::new(now.year(), now.month(), now.day())
}

/// Local date of an instant (Unix milliseconds), in the time zone of the computer.
fn local_date(instant_ms: i64) -> Date {
    use chrono::{Datelike, TimeZone};
    match chrono::Local.timestamp_millis_opt(instant_ms).earliest() {
        Some(local) => Date::new(local.year(), local.month(), local.day()),
        None => educator_core::statistics::utc_date_of(instant_ms),
    }
}
