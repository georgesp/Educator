//! Lifecycle of the .NET backend (`educator-host`).
//!
//! Protocol (see src/Educator.Host/Program.cs): the host is started with
//! `--port 0 --parent-pid <shell pid>` and the environment variable `EDUCATOR_UI_TOKEN`
//! (random, generated here). It picks a free port and prints one JSON object per line on stdout:
//! `{"event":"ready","baseUrl":"http://127.0.0.1:54321"}` or `{"event":"error","message":"…"}`.
//! The host also stops by itself when the shell process disappears (crash of the shell).
//!
//! Dev shortcut: when `EDUCATOR_BACKEND_URL` and `EDUCATOR_UI_TOKEN` are both set, the shell
//! uses that already-running host (e.g. `dotnet watch`) instead of starting one.

use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager, State};
use uuid::Uuid;

/// Maximum time for the host to report `ready`. Dev builds run `dotnet run`, which compiles first.
#[cfg(not(debug_assertions))]
const STARTUP_TIMEOUT: Duration = Duration::from_secs(10);
#[cfg(debug_assertions)]
const STARTUP_TIMEOUT: Duration = Duration::from_secs(180);

/// Backend status, as seen by the front (`get_backend` command, `backend-status` event).
#[derive(Clone, Serialize)]
#[serde(tag = "state", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum BackendStatus {
    Starting,
    Ready { base_url: String, token: String },
    Failed { message: String },
}

pub struct BackendState {
    status: Mutex<BackendStatus>,
    child: Mutex<Option<Child>>,
}

impl Default for BackendState {
    fn default() -> Self {
        Self {
            status: Mutex::new(BackendStatus::Starting),
            child: Mutex::new(None),
        }
    }
}

/// One line written by the host on stdout.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HostMessage {
    event: String,
    base_url: Option<String>,
    message: Option<String>,
}

/// Current backend status. The front polls it until the backend is ready or has failed.
#[tauri::command]
pub fn get_backend(state: State<'_, BackendState>) -> BackendStatus {
    state.status.lock().unwrap().clone()
}

/// Starts the backend (or uses the external one), without blocking the UI.
pub fn start(app: &AppHandle) {
    if let (Ok(base_url), Ok(token)) = (
        std::env::var("EDUCATOR_BACKEND_URL"),
        std::env::var("EDUCATOR_UI_TOKEN"),
    ) {
        set_status(app, BackendStatus::Ready { base_url, token });
        return;
    }

    let mut command = match host_command(app) {
        Ok(command) => command,
        Err(message) => {
            set_status(app, BackendStatus::Failed { message });
            return;
        }
    };

    // 256 random bits, never logged: only the host (environment) and the front (command) know it.
    let token = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
    command
        .args(["--port", "0", "--parent-pid", &std::process::id().to_string()])
        .env("EDUCATOR_UI_TOKEN", &token)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            set_status(
                app,
                BackendStatus::Failed {
                    message: format!("Could not start the backend: {error}"),
                },
            );
            return;
        }
    };

    let stdout = child.stdout.take().expect("stdout is piped");
    *app.state::<BackendState>().child.lock().unwrap() = Some(child);

    watch_startup_timeout(app);

    let app = app.clone();
    std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            match serde_json::from_str::<HostMessage>(&line) {
                Ok(message) if message.event == "ready" => {
                    if let Some(base_url) = message.base_url {
                        set_status(
                            &app,
                            BackendStatus::Ready {
                                base_url,
                                token: token.clone(),
                            },
                        );
                    }
                }
                Ok(message) if message.event == "error" => set_status(
                    &app,
                    BackendStatus::Failed {
                        message: message
                            .message
                            .unwrap_or_else(|| "The backend failed to start.".into()),
                    },
                ),
                // Build output of `dotnet run`, or anything else: shown in the terminal.
                _ => println!("[educator-host] {line}"),
            }
        }

        // stdout closed: the host has exited.
        let failed_already = matches!(
            *app.state::<BackendState>().status.lock().unwrap(),
            BackendStatus::Failed { .. }
        );
        if !failed_already {
            set_status(
                &app,
                BackendStatus::Failed {
                    message: "The backend stopped. See the terminal and the log files.".into(),
                },
            );
        }
    });
}

/// Fails (and stops the host) when it has not reported `ready` within `STARTUP_TIMEOUT`.
fn watch_startup_timeout(app: &AppHandle) {
    let app = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(STARTUP_TIMEOUT);
        let still_starting = matches!(
            *app.state::<BackendState>().status.lock().unwrap(),
            BackendStatus::Starting
        );
        if still_starting {
            stop(&app);
            set_status(
                &app,
                BackendStatus::Failed {
                    message: format!(
                        "The backend did not start within {} s. See the log files.",
                        STARTUP_TIMEOUT.as_secs()
                    ),
                },
            );
        }
    });
}

/// Stops the backend started by `start` (the host would also stop by itself, see `--parent-pid`).
pub fn stop(app: &AppHandle) {
    if let Some(mut child) = app.state::<BackendState>().child.lock().unwrap().take() {
        let _ = child.kill();
        let _ = child.wait();
    }
}

fn set_status(app: &AppHandle, status: BackendStatus) {
    *app.state::<BackendState>().status.lock().unwrap() = status.clone();
    let _ = app.emit("backend-status", status);
}

/// Dev: `dotnet run` on the host project of this repository (built on the fly).
#[cfg(debug_assertions)]
fn host_command(_app: &AppHandle) -> Result<Command, String> {
    let project = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("src")
        .join("Educator.Host");
    let mut command = Command::new("dotnet");
    command
        .arg("run")
        .arg("--project")
        .arg(project)
        .arg("--");
    Ok(command)
}

/// Release: the self-contained host published next to the application (lot packaging, US0069).
#[cfg(not(debug_assertions))]
fn host_command(app: &AppHandle) -> Result<Command, String> {
    let name = if cfg!(windows) { "educator-host.exe" } else { "educator-host" };
    let path = app
        .path()
        .resource_dir()
        .map_err(|error| format!("Resource folder not found: {error}"))?
        .join("host")
        .join(name);
    if path.exists() {
        Ok(Command::new(path))
    } else {
        Err(format!(
            "Backend not found ({}). Packaged builds arrive with the packaging lot: use `npm run tauri dev`.",
            path.display()
        ))
    }
}
