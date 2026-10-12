//! Local SQLite storage of Educator (port of `Educator.Data`): the `Profile` and `AppSetting`
//! tables, versioned migrations, timestamps and the location of the data files.
//!
//! Conventions (ADR 0001): tables in singular PascalCase, identifiers Guid v7 stored as upper-case
//! text (as EF Core did, so that a database created by the .NET version is kept), dates as UTC Unix
//! milliseconds (`INTEGER`), enums as text, `CreatedAt` / `UpdatedAt` filled from the injected clock.

mod database;
mod migrations;
mod paths;
mod profiles;
mod settings;

pub use database::{Database, InitReport};
pub use migrations::MIGRATIONS;
pub use paths::{app_data_directory, DATABASE_FILE_NAME, DATA_DIR_VARIABLE, LOGS_FOLDER_NAME};
pub use profiles::SqliteProfileStore;
pub use settings::SqliteAppSettingsStore;
