//! Per-user locations of the application's data files:
//! - macOS: `~/Library/Application Support/Educator/`
//! - Windows: `%APPDATA%\Educator\`
//! - Linux: `$XDG_CONFIG_HOME/Educator/` (or `~/.config`), CI only
//!
//! The environment variable `EDUCATOR_DATA_DIR` overrides the folder (tests, debugging).

use std::path::PathBuf;

pub const APP_FOLDER_NAME: &str = "Educator";
pub const DATABASE_FILE_NAME: &str = "educator.db";
pub const LOGS_FOLDER_NAME: &str = "logs";
pub const DATA_DIR_VARIABLE: &str = "EDUCATOR_DATA_DIR";

/// Folder holding every data file of the application (database, logs).
pub fn app_data_directory() -> PathBuf {
    let variable = |name: &str| {
        std::env::var_os(name)
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
    };
    if let Some(overridden) = variable(DATA_DIR_VARIABLE) {
        return std::path::absolute(&overridden).unwrap_or(overridden);
    }

    let home = variable("HOME")
        .or_else(|| variable("USERPROFILE"))
        .unwrap_or_else(|| PathBuf::from("."));
    let base = if cfg!(target_os = "macos") {
        home.join("Library").join("Application Support")
    } else if cfg!(windows) {
        variable("APPDATA").unwrap_or_else(|| home.join("AppData").join("Roaming"))
    } else {
        variable("XDG_CONFIG_HOME").unwrap_or_else(|| home.join(".config"))
    };

    base.join(APP_FOLDER_NAME)
}
