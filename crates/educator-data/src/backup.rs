//! Backup and restore of every profile (SUB001512): the whole database in one JSON file, to keep it
//! safe or to move to another computer. The file lists the rows of each table; a restore replaces
//! every profile of this computer by those of the file, after a copy of the database
//! (`<db>.before-restore-…`). A file of an older version of the application is accepted (missing
//! columns take their default); a file of a newer version is refused.

use crate::database::{copy_to, storage, Database};
use crate::migrations::MIGRATIONS;
use educator_core::{Error, Result};
use rusqlite::types::Value;
use rusqlite::Connection;
use serde_json::{json, Map, Value as Json};

/// Name written in the file, checked when it is read.
pub const BACKUP_APPLICATION: &str = "Educator";

/// Version of the file format (not of the database).
pub const BACKUP_FORMAT: u64 = 1;

/// Tables saved, parents before children (the order of the restore).
pub const BACKUP_TABLES: [&str; 11] = [
    "AppSetting",
    "Profile",
    "Session",
    "Answer",
    "SkillMastery",
    "LessonProgress",
    "EarnedBadge",
    "UnlockedAccessory",
    "DailyGoalDay",
    "ReviewItem",
    "ProfileSetting",
];

/// What a restore brought back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreReport {
    pub profiles: usize,
    /// Copy of the database taken before the restore.
    pub safety_copy: std::path::PathBuf,
}

/// Every row of every table, as JSON.
pub fn export(database: &Database) -> Result<String> {
    let connection = database.connection();
    let mut tables = Map::new();
    for table in BACKUP_TABLES {
        tables.insert(table.to_owned(), Json::Array(rows(&connection, table)?));
    }
    let backup = json!({
        "application": BACKUP_APPLICATION,
        "format": BACKUP_FORMAT,
        "schemaVersion": MIGRATIONS.len(),
        "exportedAt": database.now_ms(),
        "tables": tables,
    });
    serde_json::to_string_pretty(&backup).map_err(storage)
}

fn rows(connection: &Connection, table: &str) -> Result<Vec<Json>> {
    let mut statement = connection
        .prepare(&format!(r#"SELECT * FROM "{table}""#))
        .map_err(storage)?;
    let names: Vec<String> = statement
        .column_names()
        .into_iter()
        .map(str::to_owned)
        .collect();
    let rows = statement
        .query_map([], |row| {
            let mut object = Map::new();
            for (index, name) in names.iter().enumerate() {
                let value: Value = row.get(index)?;
                object.insert(name.clone(), to_json(value));
            }
            Ok(Json::Object(object))
        })
        .map_err(storage)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(storage);
    rows
}

fn to_json(value: Value) -> Json {
    match value {
        Value::Null => Json::Null,
        Value::Integer(i) => json!(i),
        Value::Real(f) => json!(f),
        Value::Text(t) => Json::String(t),
        // No blob in the schema; kept as text if one ever appears.
        Value::Blob(b) => Json::String(String::from_utf8_lossy(&b).into_owned()),
    }
}

fn to_sql(value: &Json) -> Option<Value> {
    match value {
        Json::Null => Some(Value::Null),
        Json::Bool(b) => Some(Value::Integer(i64::from(*b))),
        Json::Number(n) => n
            .as_i64()
            .map(Value::Integer)
            .or_else(|| n.as_f64().map(Value::Real)),
        Json::String(s) => Some(Value::Text(s.clone())),
        _ => None,
    }
}

fn invalid(message: impl Into<String>) -> Error {
    Error::InvalidRequest(message.into())
}

/// Replaces every profile (and the settings) by those of a backup file.
pub fn restore(database: &Database, text: &str) -> Result<RestoreReport> {
    let backup: Json =
        serde_json::from_str(text).map_err(|e| invalid(format!("Not a backup file: {e}")))?;
    if backup["application"] != BACKUP_APPLICATION {
        return Err(invalid("Not an Educator backup."));
    }
    if backup["format"].as_u64() != Some(BACKUP_FORMAT) {
        return Err(invalid("Unknown backup format."));
    }
    let version = backup["schemaVersion"]
        .as_u64()
        .ok_or_else(|| invalid("Backup without version."))?;
    if version > MIGRATIONS.len() as u64 {
        return Err(invalid(
            "This backup comes from a newer version of Educator: update the application first.",
        ));
    }
    let tables = backup["tables"]
        .as_object()
        .ok_or_else(|| invalid("Backup without tables."))?;

    let mut connection = database.connection();
    let safety_copy = copy_to(
        &connection,
        database.path(),
        "before-restore",
        database.now_ms(),
    )?;
    let transaction = connection.transaction().map_err(storage)?;
    // Children first; deleting the profiles would cascade, the explicit order is clearer.
    for table in BACKUP_TABLES.iter().rev() {
        transaction
            .execute(&format!(r#"DELETE FROM "{table}""#), [])
            .map_err(storage)?;
    }
    let mut profiles = 0;
    for table in BACKUP_TABLES {
        let columns = table_columns(&transaction, table)?;
        let Some(rows) = tables.get(table).and_then(Json::as_array) else {
            continue;
        };
        for row in rows {
            let object = row
                .as_object()
                .ok_or_else(|| invalid(format!("A row of {table} is not an object.")))?;
            let present: Vec<(&String, Value)> = object
                .iter()
                .filter(|(name, _)| columns.contains(name))
                .map(|(name, value)| {
                    to_sql(value)
                        .map(|v| (name, v))
                        .ok_or_else(|| invalid(format!("Bad value for {table}.{name}.")))
                })
                .collect::<Result<_>>()?;
            if present.is_empty() {
                continue;
            }
            let names = present
                .iter()
                .map(|(n, _)| format!(r#""{n}""#))
                .collect::<Vec<_>>()
                .join(", ");
            let marks = (1..=present.len())
                .map(|i| format!("?{i}"))
                .collect::<Vec<_>>()
                .join(", ");
            transaction
                .execute(
                    &format!(r#"INSERT INTO "{table}" ({names}) VALUES ({marks})"#),
                    rusqlite::params_from_iter(present.iter().map(|(_, v)| v)),
                )
                .map_err(|e| invalid(format!("Row of {table} refused: {e}")))?;
            if table == "Profile" {
                profiles += 1;
            }
        }
    }
    // A backup older than lot 3: the avatars come from the characters.
    transaction
        .execute(
            r#"UPDATE "Profile" SET "Avatar" = json_object('character', "CharacterId", 'accessories', json('[]')) WHERE "Avatar" = ''"#,
            [],
        )
        .map_err(storage)?;
    transaction.commit().map_err(storage)?;
    Ok(RestoreReport {
        profiles,
        safety_copy,
    })
}

fn table_columns(connection: &Connection, table: &str) -> Result<Vec<String>> {
    let mut statement = connection
        .prepare(&format!(r#"PRAGMA table_info("{table}")"#))
        .map_err(storage)?;
    let columns = statement
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(storage)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(storage);
    columns
}
