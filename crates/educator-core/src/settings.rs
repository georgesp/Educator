//! Key / value application settings stored locally (table `AppSetting`, SUB006004).
//! Keys are case-sensitive; values are plain strings (JSON when structured).

use crate::Result;

pub trait AppSettingsStore: Send + Sync {
    fn get(&self, key: &str) -> Result<Option<String>>;

    fn set(&self, key: &str, value: &str) -> Result<()>;

    /// Returns `true` when the key existed.
    fn remove(&self, key: &str) -> Result<bool>;
}
