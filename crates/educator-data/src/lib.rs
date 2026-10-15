//! Local SQLite storage of Educator (port of `Educator.Data`): the `Profile` and `AppSetting`
//! tables, the sessions of lot 2 (`Session`, `Answer`, `SkillMastery`), versioned migrations,
//! the rewards, review queue and settings of lot 3 (`EarnedBadge`, `UnlockedAccessory`, `DailyGoalDay`,
//! `ReviewItem`, `ProfileSetting`), the backup of everything (`backup`), timestamps and the location
//! of the data files.
//!
//! Conventions (ADR 0001): tables in singular PascalCase, identifiers Guid v7 stored as upper-case
//! text (as EF Core did, so that a database created by the .NET version is kept), dates as UTC Unix
//! milliseconds (`INTEGER`), enums as text, `CreatedAt` / `UpdatedAt` filled from the injected clock.

pub mod backup;
mod database;
mod migrations;
mod motivation;
mod paths;
mod profiles;
mod sessions;
mod settings;

pub use database::{Database, InitReport};
pub use migrations::MIGRATIONS;
pub use motivation::{SqliteProfileSettingsStore, SqliteReviewStore, SqliteRewardStore};
pub use paths::{app_data_directory, DATABASE_FILE_NAME, DATA_DIR_VARIABLE, LOGS_FOLDER_NAME};
pub use profiles::SqliteProfileStore;
pub use sessions::SqliteSessionStore;
pub use settings::SqliteAppSettingsStore;
