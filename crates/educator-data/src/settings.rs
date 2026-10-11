use crate::database::{storage, Database};
use educator_core::settings::AppSettingsStore;
use educator_core::Result;
use rusqlite::OptionalExtension;
use std::sync::Arc;

/// [`AppSettingsStore`] backed by the `AppSetting` table.
#[derive(Debug, Clone)]
pub struct SqliteAppSettingsStore {
    database: Arc<Database>,
}

impl SqliteAppSettingsStore {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

impl AppSettingsStore for SqliteAppSettingsStore {
    fn get(&self, key: &str) -> Result<Option<String>> {
        self.database
            .connection()
            .query_row(
                r#"SELECT "Value" FROM "AppSetting" WHERE "Key" = ?1"#,
                [key],
                |row| row.get(0),
            )
            .optional()
            .map_err(storage)
    }

    /// `UpdatedAt` only moves when the value actually changes.
    fn set(&self, key: &str, value: &str) -> Result<()> {
        let now = self.database.now_ms();
        self.database
            .connection()
            .execute(
                r#"INSERT INTO "AppSetting" ("Key", "Value", "CreatedAt", "UpdatedAt") VALUES (?1, ?2, ?3, ?3)
                   ON CONFLICT ("Key") DO UPDATE SET "Value" = excluded."Value", "UpdatedAt" = excluded."UpdatedAt"
                   WHERE "Value" <> excluded."Value""#,
                rusqlite::params![key, value, now],
            )
            .map(|_| ())
            .map_err(storage)
    }

    fn remove(&self, key: &str) -> Result<bool> {
        self.database
            .connection()
            .execute(r#"DELETE FROM "AppSetting" WHERE "Key" = ?1"#, [key])
            .map(|count| count > 0)
            .map_err(storage)
    }
}
