//! Per-user locations of the application's data files:
//! - macOS: `~/Library/Application Support/Educator/`
//! - iPad (iOS): `<app container>/Library/Application Support/Educator/`, the only writable places of an
//!   iOS app being `Documents`, `Library` and `tmp` (ADR 0006)
//! - Windows: `%APPDATA%\Educator\`
//! - Linux: `$XDG_CONFIG_HOME/Educator/` (or `~/.config`), CI only
//!
//! The environment variable `EDUCATOR_DATA_DIR` overrides the folder (tests, debugging).
//!
//! The resolution itself is a pure function of the platform and of the environment variables
//! ([`data_directory_for`]), so that every platform is tested on any computer.

use std::ffi::OsString;
use std::path::PathBuf;

pub const APP_FOLDER_NAME: &str = "Educator";
pub const DATABASE_FILE_NAME: &str = "educator.db";
pub const LOGS_FOLDER_NAME: &str = "logs";
pub const DATA_DIR_VARIABLE: &str = "EDUCATOR_DATA_DIR";

/// Platform deciding where the data files go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Platform {
    MacOs,
    Ios,
    Windows,
    /// Linux and anything else (CI only).
    Other,
}

impl Platform {
    /// Platform the application is compiled for.
    pub fn current() -> Self {
        if cfg!(target_os = "macos") {
            Self::MacOs
        } else if cfg!(target_os = "ios") {
            Self::Ios
        } else if cfg!(windows) {
            Self::Windows
        } else {
            Self::Other
        }
    }
}

/// Folder holding every data file of the application (database, logs).
pub fn app_data_directory() -> PathBuf {
    data_directory_for(Platform::current(), |name| std::env::var_os(name))
}

/// Folder holding the data files on `platform`, `variable` giving the environment variables
/// (`HOME`, `APPDATA`, `XDG_CONFIG_HOME`, `EDUCATOR_DATA_DIR`…). An empty variable counts as missing.
pub fn data_directory_for(
    platform: Platform,
    variable: impl Fn(&str) -> Option<OsString>,
) -> PathBuf {
    let variable = |name: &str| variable(name).filter(|v| !v.is_empty()).map(PathBuf::from);
    if let Some(overridden) = variable(DATA_DIR_VARIABLE) {
        return std::path::absolute(&overridden).unwrap_or(overridden);
    }

    let home = variable("HOME")
        .or_else(|| variable("USERPROFILE"))
        .unwrap_or_else(|| PathBuf::from("."));
    let base = match platform {
        Platform::MacOs | Platform::Ios => home.join("Library").join("Application Support"),
        Platform::Windows => {
            variable("APPDATA").unwrap_or_else(|| home.join("AppData").join("Roaming"))
        }
        Platform::Other => variable("XDG_CONFIG_HOME").unwrap_or_else(|| home.join(".config")),
    };

    base.join(APP_FOLDER_NAME)
}
