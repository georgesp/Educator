use crate::migrations::MIGRATIONS;
use educator_core::clock::{uuid_v7, Clock};
use educator_core::sync::Hlc;
use educator_core::{Error, Result};
use rusqlite::functions::FunctionFlags;
use rusqlite::{Connection, OptionalExtension};
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
    sync_clock: Arc<SyncClock>,
}

/// First migration of the synchronisation (ADR 0007): from it on, the device has an identity and
/// every write of a followed table goes through `educator_hlc()`.
const SYNC_MIGRATION: usize = 5;

/// Key of the identifier of this device in `SyncState`.
pub(crate) const DEVICE_ID_KEY: &str = "DeviceId";

/// Key of the epoch of this device in `SyncState`: a new one at every restore of a backup.
pub(crate) const EPOCH_KEY: &str = "Epoch";

/// The hybrid logical clock of this device (ADR 0007, section 3), shared by the SQL function
/// `educator_hlc()` of the triggers and the merge.
#[derive(Debug, Default)]
pub(crate) struct SyncClock(Mutex<ClockState>);

#[derive(Debug, Default)]
struct ClockState {
    device: Option<uuid::Uuid>,
    last: Option<Hlc>,
}

impl SyncClock {
    fn state(&self) -> MutexGuard<'_, ClockState> {
        self.0.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// The clock of a new change.
    pub(crate) fn tick(&self, now_ms: i64) -> std::result::Result<Hlc, String> {
        let mut state = self.state();
        let device = state
            .device
            .ok_or_else(|| "The identity of the device is not ready.".to_owned())?;
        let next = Hlc::next(state.last, now_ms, device);
        state.last = Some(next);
        Ok(next)
    }

    /// A clock received: the next local changes come after it.
    pub(crate) fn observe(&self, clock: Hlc) {
        let mut state = self.state();
        if state.last.is_none_or(|last| clock > last) {
            state.last = Some(clock);
        }
    }

    pub(crate) fn device(&self) -> Option<uuid::Uuid> {
        self.state().device
    }

    fn set_device(&self, device: uuid::Uuid) {
        self.state().device = Some(device);
    }
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
        let sync_clock = Arc::new(SyncClock::default());
        register_clock(&connection, &sync_clock, &clock)?;

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
        if applied >= SYNC_MIGRATION {
            ensure_identity(&mut connection, &sync_clock, clock.now_ms())?;
        }
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
                if applied + offset + 1 == SYNC_MIGRATION {
                    ensure_identity(&mut connection, &sync_clock, clock.now_ms())?;
                }
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
                sync_clock,
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

    pub(crate) fn sync_clock(&self) -> &SyncClock {
        &self.sync_clock
    }
}

/// `educator_hlc()`: the clock of a new change, called by the triggers of migration 0005.
fn register_clock(
    connection: &Connection,
    sync_clock: &Arc<SyncClock>,
    clock: &Arc<dyn Clock>,
) -> Result<()> {
    let sync_clock = Arc::clone(sync_clock);
    let clock = Arc::clone(clock);
    connection
        .create_scalar_function("educator_hlc", 0, FunctionFlags::SQLITE_UTF8, move |_| {
            sync_clock
                .tick(clock.now_ms())
                .map(|hlc| hlc.encode())
                .map_err(|e| rusqlite::Error::UserFunctionError(e.into()))
        })
        .map_err(storage)
}

/// Reads the identity of the device, or creates it (first opening after migration 0005): a new
/// identifier, and every record already there marked « to send » with a clock taken from its
/// last update. The clock of the device starts after the greatest clock known.
fn ensure_identity(connection: &mut Connection, sync_clock: &SyncClock, now: i64) -> Result<()> {
    let existing: Option<String> = connection
        .query_row(
            r#"SELECT "Value" FROM "SyncState" WHERE "Key" = ?1"#,
            [DEVICE_ID_KEY],
            |row| row.get(0),
        )
        .optional()
        .map_err(storage)?;
    let device = match existing {
        Some(text) => uuid::Uuid::parse_str(&text).map_err(storage)?,
        None => {
            let device = uuid_v7(now);
            let transaction = connection.transaction().map_err(storage)?;
            transaction
                .execute(
                    r#"INSERT INTO "SyncState" ("Key", "Value", "CreatedAt", "UpdatedAt") VALUES (?1, ?2, ?3, ?3)"#,
                    rusqlite::params![DEVICE_ID_KEY, crate::profiles::id_text(device), now],
                )
                .map_err(storage)?;
            transaction
                .execute(
                    r#"INSERT INTO "SyncState" ("Key", "Value", "CreatedAt", "UpdatedAt") VALUES (?1, ?2, ?3, ?3)"#,
                    rusqlite::params![EPOCH_KEY, uuid_v7(now).to_string(), now],
                )
                .map_err(storage)?;
            let seeded = seed(&transaction, device)?;
            transaction.commit().map_err(storage)?;
            log::info!(
                "Synchronisation: identity of this device created, {seeded} record(s) to send."
            );
            device
        }
    };
    sync_clock.set_device(device);
    let last: Option<String> = connection
        .query_row(r#"SELECT MAX("Hlc") FROM "SyncRow""#, [], |row| row.get(0))
        .map_err(storage)?;
    if let Some(last) = last.as_deref().and_then(Hlc::parse) {
        sync_clock.observe(last);
    }
    Ok(())
}

/// Marks every record of the database « to send », oldest first.
fn seed(connection: &Connection, device: uuid::Uuid) -> Result<usize> {
    const PROFILE_FIELDS: [&str; 5] =
        ["name", "grade", "avatar", "placementSeconds", "examSeconds"];
    // (entity, key, field, instant)
    let mut rows: Vec<(String, String, String, i64)> = Vec::new();
    let mut collect = |sql: &str, entity: &str, fields: &[&str]| -> Result<()> {
        let mut statement = connection.prepare(sql).map_err(storage)?;
        let found = statement
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
            })
            .map_err(storage)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(storage)?;
        for (key, at) in found {
            for field in fields {
                rows.push((entity.to_owned(), key.clone(), (*field).to_owned(), at));
            }
        }
        Ok(())
    };
    collect(
        r#"SELECT "Id", "UpdatedAt" FROM "Profile""#,
        "Profile",
        &PROFILE_FIELDS,
    )?;
    collect(
        r#"SELECT "Id", "LastUsedAt" FROM "Profile""#,
        "Profile",
        &["lastUsedAt"],
    )?;
    collect(
        r#"SELECT "Id", "UpdatedAt" FROM "Session" WHERE "EndedAt" IS NOT NULL"#,
        "Session",
        &[""],
    )?;
    collect(
        r#"SELECT "ProfileId" || '/' || "LessonId", "UpdatedAt" FROM "LessonProgress""#,
        "LessonProgress",
        &[""],
    )?;
    collect(
        r#"SELECT "ProfileId" || '/' || "BadgeId", "CreatedAt" FROM "EarnedBadge""#,
        "Badge",
        &[""],
    )?;
    collect(
        r#"SELECT "ProfileId" || '/' || "AccessoryId", "CreatedAt" FROM "UnlockedAccessory""#,
        "Accessory",
        &[""],
    )?;
    collect(
        r#"SELECT "ProfileId" || '/' || "Day", "CreatedAt" FROM "DailyGoalDay""#,
        "GoalDay",
        &[""],
    )?;
    collect(
        r#"SELECT "ProfileId" || '/' || "SkillId" || '/' || "Key", "UpdatedAt" FROM "ReviewItem""#,
        "Review",
        &[""],
    )?;
    collect(
        r#"SELECT "ProfileId" || '/' || "Key", "UpdatedAt" FROM "ProfileSetting""#,
        "Setting",
        &[""],
    )?;
    rows.sort_by(|a, b| (a.3, &a.0, &a.1, &a.2).cmp(&(b.3, &b.0, &b.1, &b.2)));
    let mut statement = connection
        .prepare(
            r#"INSERT OR IGNORE INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted") VALUES (?1, ?2, ?3, ?4, ?5, 0)"#,
        )
        .map_err(storage)?;
    for (seq, (entity, key, field, at)) in rows.iter().enumerate() {
        statement
            .execute(rusqlite::params![
                entity,
                key,
                field,
                Hlc::new(*at, 0, device).encode(),
                i64::try_from(seq).unwrap_or(i64::MAX) + 1
            ])
            .map_err(storage)?;
    }
    connection
        .execute(
            r#"UPDATE "SyncCounter" SET "Seq" = ?1"#,
            [i64::try_from(rows.len()).unwrap_or(i64::MAX)],
        )
        .map_err(storage)?;
    Ok(rows.len())
}

/// `VACUUM INTO` produces a consistent copy, WAL content included: `<db>.backup-yyyyMMdd-HHmmss`.
fn backup(connection: &Connection, path: &Path, now_ms: i64) -> Result<PathBuf> {
    copy_to(connection, path, "backup", now_ms)
}

/// A consistent copy of the database next to it: `<db>.<label>-yyyyMMdd-HHmmss`.
pub(crate) fn copy_to(
    connection: &Connection,
    path: &Path,
    label: &str,
    now_ms: i64,
) -> Result<PathBuf> {
    let backup_path = PathBuf::from(format!("{}.{label}-{}", path.display(), stamp(now_ms)));
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
