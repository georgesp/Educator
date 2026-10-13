use crate::database::{storage, Database};
use educator_core::profiles::{Profile, ProfileStore, NAME_TAKEN};
use educator_core::{Error, Grade, Result};
use rusqlite::{Connection, OptionalExtension, Row};
use std::sync::Arc;
use uuid::Uuid;

const COLUMNS: &str =
    r#""Id", "Name", "Grade", "CharacterId", "Points", "CreatedAt", "UpdatedAt", "LastUsedAt""#;

/// [`ProfileStore`] backed by the `Profile` table.
#[derive(Debug, Clone)]
pub struct SqliteProfileStore {
    database: Arc<Database>,
}

impl SqliteProfileStore {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

/// Upper-case hyphenated text, as EF Core stores a Guid in SQLite.
pub(crate) fn id_text(id: Uuid) -> String {
    id.hyphenated()
        .encode_upper(&mut Uuid::encode_buffer())
        .to_owned()
}

fn read(row: &Row<'_>) -> rusqlite::Result<Profile> {
    let id: String = row.get(0)?;
    let grade: String = row.get(2)?;
    let invalid = |index: usize, e: Box<dyn std::error::Error + Send + Sync>| {
        rusqlite::Error::FromSqlConversionFailure(index, rusqlite::types::Type::Text, e)
    };
    Ok(Profile {
        id: Uuid::parse_str(&id).map_err(|e| invalid(0, Box::new(e)))?,
        name: row.get(1)?,
        grade: grade
            .parse::<Grade>()
            .map_err(|e| invalid(2, Box::new(e)))?,
        character_id: row.get(3)?,
        points: row.get(4)?,
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
        last_used_at: row.get(7)?,
    })
}

fn get(connection: &Connection, id: Uuid) -> Result<Option<Profile>> {
    connection
        .query_row(
            &format!(r#"SELECT {COLUMNS} FROM "Profile" WHERE "Id" = ?1"#),
            [id_text(id)],
            read,
        )
        .optional()
        .map_err(storage)
}

/// A unique-name race lost against another request reads as a validation error.
fn write_error(error: rusqlite::Error) -> Error {
    match &error {
        rusqlite::Error::SqliteFailure(e, message)
            if e.code == rusqlite::ErrorCode::ConstraintViolation
                && message
                    .as_deref()
                    .is_some_and(|m| m.contains("NormalizedName")) =>
        {
            Error::validation("name", NAME_TAKEN)
        }
        _ => storage(error),
    }
}

impl ProfileStore for SqliteProfileStore {
    fn list(&self) -> Result<Vec<Profile>> {
        let connection = self.database.connection();
        let mut statement = connection
            .prepare(&format!(
                r#"SELECT {COLUMNS} FROM "Profile" ORDER BY "LastUsedAt" DESC, "Name""#
            ))
            .map_err(storage)?;
        let profiles = statement
            .query_map([], read)
            .map_err(storage)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(storage);
        profiles
    }

    fn get(&self, id: Uuid) -> Result<Option<Profile>> {
        get(&self.database.connection(), id)
    }

    fn name_exists(&self, normalized_name: &str, except_id: Option<Uuid>) -> Result<bool> {
        self.database
            .connection()
            .query_row(
                r#"SELECT EXISTS (SELECT 1 FROM "Profile" WHERE "NormalizedName" = ?1 AND (?2 IS NULL OR "Id" <> ?2))"#,
                rusqlite::params![normalized_name, except_id.map(id_text)],
                |row| row.get(0),
            )
            .map_err(storage)
    }

    fn add(&self, profile: &Profile) -> Result<Profile> {
        let now = self.database.now_ms();
        let connection = self.database.connection();
        connection
            .execute(
                r#"INSERT INTO "Profile" ("Id", "Name", "NormalizedName", "Grade", "CharacterId", "Points", "CreatedAt", "UpdatedAt", "LastUsedAt")
                   VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7, ?8)"#,
                rusqlite::params![
                    id_text(profile.id),
                    profile.name,
                    Profile::normalize_name(&profile.name),
                    profile.grade.as_str(),
                    profile.character_id,
                    profile.points,
                    now,
                    profile.last_used_at,
                ],
            )
            .map_err(write_error)?;
        get(&connection, profile.id)?.ok_or_else(|| storage("Profile not found after insert."))
    }

    /// `UpdatedAt` only moves when a column actually changed. Points are never written here.
    fn update(&self, profile: &Profile) -> Result<Option<Profile>> {
        let now = self.database.now_ms();
        let connection = self.database.connection();
        let Some(current) = get(&connection, profile.id)? else {
            return Ok(None);
        };

        let changed = current.name != profile.name
            || current.grade != profile.grade
            || current.character_id != profile.character_id
            || current.last_used_at != profile.last_used_at;
        if changed {
            connection
                .execute(
                    r#"UPDATE "Profile" SET "Name" = ?2, "NormalizedName" = ?3, "Grade" = ?4, "CharacterId" = ?5,
                       "LastUsedAt" = ?6, "UpdatedAt" = ?7 WHERE "Id" = ?1"#,
                    rusqlite::params![
                        id_text(profile.id),
                        profile.name,
                        Profile::normalize_name(&profile.name),
                        profile.grade.as_str(),
                        profile.character_id,
                        profile.last_used_at,
                        now,
                    ],
                )
                .map_err(write_error)?;
        }

        get(&connection, profile.id)
    }

    fn delete(&self, id: Uuid) -> Result<bool> {
        self.database
            .connection()
            .execute(r#"DELETE FROM "Profile" WHERE "Id" = ?1"#, [id_text(id)])
            .map(|count| count > 0)
            .map_err(storage)
    }

    fn add_points(&self, id: Uuid, points: i32) -> Result<Option<i32>> {
        if points < 0 {
            return Err(storage("Points are never taken away."));
        }

        self.database
            .connection()
            .query_row(
                r#"UPDATE "Profile" SET "Points" = "Points" + ?2 WHERE "Id" = ?1 RETURNING "Points""#,
                rusqlite::params![id_text(id), points],
                |row| row.get(0),
            )
            .optional()
            .map_err(storage)
    }
}
