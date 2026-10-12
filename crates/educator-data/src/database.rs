use crate::migrations::MIGRATIONS;
use educator_core::clock::Clock;
use educator_core::{Error, Result};
use rusqlite::Connection;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

/// What [`Database::open`] did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InitReport {
    /// Migrations applied by this opening.
    pub applied_migrations: Vec<String>,
    /// Backup taken before migrating an existing database, if any.
    pub backup_path: Option<PathBuf>,
}

/// The SQLite database file, opened once and shared by the stores (one connection behind a mutex:
/// a single child uses the application at a time, the queries are tiny).
pub struct Database {
    path: PathBuf,
    connection: Mutex<Connection>,
    clock: Arc<dyn Clock>,
}

impl fmt::Debug for Database {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Database")
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}

impl Database {
    /// Prepares the database at start-up: creates the folder, backs up an existing database before
    /// applying pending migrations (`VACUUM INTO`), applies them, and switches the journal to WAL.
    pub fn open(path: &Path, clock: Arc<dyn Clock>) -> Result<(Database, InitReport)> {
        if let Some(directory) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
            std::fs::create_dir_all(directory)
                .map_err(|e| storage(format!("{}: {e}", directory.display())))?;
        }

        let existed = path.is_file();
        let mut connection = Connection::open(path).map_err(storage)?;
        connection
            .busy_timeout(Duration::from_secs(30))
            .map_err(storage)?;
        connection
            .pragma_update(None, "foreign_keys", true)
            .map_err(storage)?;

        let version: i64 = connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .map_err(storage)?;
        let applied = usize::try_from(version).unwrap_or(0);
        if applied > MIGRATIONS.len() {
            return Err(storage(format!(
                "Database version {applied} is newer than this application ({}): update Educator.",
                MIGRATIONS.len()
            )));
        }

        let mut report = InitReport::default();
        let pending = &MIGRATIONS[applied..];
        if !pending.is_empty() {
            if existed {
                report.backup_path = Some(backup(&connection, path, clock.now_ms())?);
            }

            for (offset, (name, sql)) in pending.iter().enumerate() {
                let transaction = connection.transaction().map_err(storage)?;
                transaction
                    .execute_batch(sql)
                    .map_err(|e| storage(format!("Migration {name}: {e}")))?;
                transaction
                    .pragma_update(None, "user_version", (applied + offset + 1) as i64)
                    .map_err(storage)?;
                transaction.commit().map_err(storage)?;
                log::info!("Database migration {name} applied.");
                report.applied_migrations.push((*name).to_owned());
            }
        }

        // Persistent setting stored in the database file: readers never block the writer.
        let _: String = connection
            .pragma_update_and_check(None, "journal_mode", "WAL", |row| row.get(0))
            .map_err(storage)?;

        Ok((
            Database {
                path: path.to_path_buf(),
                connection: Mutex::new(connection),
                clock,
            },
            report,
        ))
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn connection(&self) -> MutexGuard<'_, Connection> {
        self.connection.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Now, truncated to the millisecond (the precision of the columns).
    pub(crate) fn now_ms(&self) -> i64 {
        self.clock.now_ms()
    }
}

/// `VACUUM INTO` produces a consistent copy, WAL content included: `<db>.backup-yyyyMMdd-HHmmss`.
fn backup(connection: &Connection, path: &Path, now_ms: i64) -> Result<PathBuf> {
    let backup_path = PathBuf::from(format!("{}.backup-{}", path.display(), stamp(now_ms)));
    connection
        .execute("VACUUM INTO ?1", [backup_path.to_string_lossy().as_ref()])
        .map_err(|e| storage(format!("Backup before migration: {e}")))?;
    log::info!(
        "Database backed up to {} before migrating.",
        backup_path.display()
    );
    Ok(backup_path)
}

/// `yyyyMMdd-HHmmss` in UTC.
fn stamp(now_ms: i64) -> String {
    let seconds = now_ms.div_euclid(1000);
    let (days, rest) = (seconds.div_euclid(86_400), seconds.rem_euclid(86_400));
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}{month:02}{day:02}-{:02}{:02}{:02}",
        rest / 3600,
        rest % 3600 / 60,
        rest % 60
    )
}

/// Inverse of `days_from_civil` (Howard Hinnant's algorithm).
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    (yoe + era * 400 + i64::from(month <= 2), month, day)
}

pub(crate) fn storage(error: impl fmt::Display) -> Error {
    Error::Storage(error.to_string())
}
