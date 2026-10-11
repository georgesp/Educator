use std::path::{Path, PathBuf};

/// Overrides the location of the content (tests, editing the content without rebuilding).
pub const CONTENT_DIR_VARIABLE: &str = "EDUCATOR_CONTENT_DIR";

pub const FOLDER_NAME: &str = "content";

/// Location of the pedagogical content: `EDUCATOR_CONTENT_DIR` when set, otherwise the first
/// candidate holding a `content/skills` folder (the Tauri resources of the packaged application),
/// otherwise the first `content/` found above `start` (development: the repository root above
/// `target/debug`).
pub fn find_content_directory(candidates: &[PathBuf], start: &Path) -> Option<PathBuf> {
    if let Some(overridden) = std::env::var_os(CONTENT_DIR_VARIABLE).filter(|v| !v.is_empty()) {
        let path = PathBuf::from(overridden);
        return Some(std::path::absolute(&path).unwrap_or(path));
    }

    let is_content = |dir: &Path| dir.join("skills").is_dir();
    candidates
        .iter()
        .find(|dir| is_content(dir))
        .cloned()
        .or_else(|| {
            start
                .ancestors()
                .map(|dir| dir.join(FOLDER_NAME))
                .find(|dir| is_content(dir))
        })
}
