//! Finds the Piper engine and the voice. Expected layout (produced by `scripts/fetch-piper.sh` /
//! `scripts/fetch-piper.ps1`, see `docs/adr/0002-voix-piper.md`):
//!
//! ```text
//! piper/                       ← root
//!   macos/ | windows/ | linux/ ← engine of each platform: piper[.exe], native libraries, espeak-ng-data/
//!   voices/                    ← <voice>.onnx and <voice>.onnx.json
//! ```
//!
//! Root search order: [`PiperLocatorOptions::explicit_root`] (environment variable
//! `EDUCATOR_PIPER_DIR`), then the `piper` folder of each packaged location (Tauri resources, next to
//! the executable), then the first `vendor/piper` found above the executable (development).

use std::path::{Path, PathBuf};

pub const ROOT_VARIABLE: &str = "EDUCATOR_PIPER_DIR";
pub const VOICE_VARIABLE: &str = "EDUCATOR_PIPER_VOICE";
pub const DEFAULT_VOICE: &str = "fr_FR-siwis-medium";

/// Files of a usable Piper installation (engine + one voice).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiperInstallation {
    pub executable_path: PathBuf,
    pub espeak_data_directory: PathBuf,
    pub model_path: PathBuf,
    pub model_config_path: PathBuf,
    pub voice_name: String,
}

impl PiperInstallation {
    /// Folder of the executable; also holds its native libraries.
    pub fn engine_directory(&self) -> &Path {
        self.executable_path.parent().unwrap_or(Path::new("."))
    }
}

/// Engine folder and executable name of one operating system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PiperPlatform {
    pub folder_name: &'static str,
    pub executable_name: &'static str,
    pub library_path_variable: Option<&'static str>,
}

impl PiperPlatform {
    pub const MACOS: PiperPlatform = PiperPlatform {
        folder_name: "macos",
        executable_name: "piper",
        library_path_variable: Some("DYLD_LIBRARY_PATH"),
    };
    pub const WINDOWS: PiperPlatform = PiperPlatform {
        folder_name: "windows",
        executable_name: "piper.exe",
        library_path_variable: None,
    };
    pub const LINUX: PiperPlatform = PiperPlatform {
        folder_name: "linux",
        executable_name: "piper",
        library_path_variable: Some("LD_LIBRARY_PATH"),
    };

    pub fn current() -> Self {
        if cfg!(windows) {
            Self::WINDOWS
        } else if cfg!(target_os = "macos") {
            Self::MACOS
        } else {
            Self::LINUX
        }
    }
}

/// Where to look for Piper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiperLocatorOptions {
    pub explicit_root: Option<PathBuf>,
    pub voice_name: String,
    /// Folders of the packaged application (Tauri resources, folder of the executable): their
    /// `piper` subfolder is used when present.
    pub packaged_directories: Vec<PathBuf>,
    /// Start of the upward search for `vendor/piper` (folder of the executable).
    pub base_directory: PathBuf,
    pub platform: PiperPlatform,
}

impl PiperLocatorOptions {
    /// `EDUCATOR_PIPER_DIR` and `EDUCATOR_PIPER_VOICE`, with the given folders.
    pub fn from_environment(packaged_directories: Vec<PathBuf>, base_directory: PathBuf) -> Self {
        let variable = |name: &str| {
            std::env::var(name)
                .ok()
                .map(|v| v.trim().to_owned())
                .filter(|v| !v.is_empty())
        };
        Self {
            explicit_root: variable(ROOT_VARIABLE).map(PathBuf::from),
            voice_name: variable(VOICE_VARIABLE).unwrap_or_else(|| DEFAULT_VOICE.to_owned()),
            packaged_directories,
            base_directory,
            platform: PiperPlatform::current(),
        }
    }
}

/// The installation, or why none was found (English, shown in the status bar tooltip).
pub fn locate(options: &PiperLocatorOptions) -> Result<PiperInstallation, String> {
    let root = find_root(options).ok_or_else(|| {
        format!(
            "Piper not found: set {ROOT_VARIABLE} or run scripts/fetch-piper to fill vendor/piper."
        )
    })?;

    let engine_directory = root.join(options.platform.folder_name);
    let executable = engine_directory.join(options.platform.executable_name);
    if !executable.is_file() {
        return Err(format!(
            "Piper executable missing: {}.",
            executable.display()
        ));
    }

    let espeak_data = engine_directory.join("espeak-ng-data");
    if !espeak_data.is_dir() {
        return Err(format!(
            "espeak-ng data missing: {}.",
            espeak_data.display()
        ));
    }

    let model = root
        .join("voices")
        .join(format!("{}.onnx", options.voice_name));
    let config = PathBuf::from(format!("{}.json", model.display()));
    if !model.is_file() || !config.is_file() {
        return Err(format!(
            "Voice {} missing: {} (+ .json).",
            options.voice_name,
            model.display()
        ));
    }

    Ok(PiperInstallation {
        executable_path: executable,
        espeak_data_directory: espeak_data,
        model_path: model,
        model_config_path: config,
        voice_name: options.voice_name.clone(),
    })
}

fn find_root(options: &PiperLocatorOptions) -> Option<PathBuf> {
    if let Some(root) = &options.explicit_root {
        return root
            .is_dir()
            .then(|| std::path::absolute(root).unwrap_or_else(|_| root.clone()));
    }

    options
        .packaged_directories
        .iter()
        .chain(std::iter::once(&options.base_directory))
        .map(|dir| dir.join("piper"))
        .find(|dir| dir.is_dir())
        .or_else(|| {
            options
                .base_directory
                .ancestors()
                .map(|dir| dir.join("vendor").join("piper"))
                .find(|dir| dir.is_dir())
        })
}
