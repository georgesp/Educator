//! Lot 3 storage: the badges, accessories and goal days of the profiles (`EarnedBadge`,
//! `UnlockedAccessory`, `DailyGoalDay`), the review queue (`ReviewItem`) and the settings of each
//! child (`ProfileSetting`). Everything goes with its profile (foreign keys, `ON DELETE CASCADE`).

use crate::database::{storage, Database};
use crate::profiles::id_text;
use educator_core::profile_settings::ProfileSettingsStore;
use educator_core::reviews::{ReviewItem, ReviewOrigin, ReviewStore};
use educator_core::rewards::badges::EarnedBadge;
use educator_core::rewards::daily_goal::GoalDay;
use educator_core::rewards::RewardStore;
use educator_core::skills::{AnswerMode, ExerciseFormat};
use educator_core::Result;
use rusqlite::{OptionalExtension, Row};
use std::sync::Arc;
use uuid::Uuid;

/// [`RewardStore`] backed by the `EarnedBadge`, `UnlockedAccessory` and `DailyGoalDay` tables.
#[derive(Debug, Clone)]
pub struct SqliteRewardStore {
    database: Arc<Database>,
}

impl SqliteRewardStore {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

impl RewardStore for SqliteRewardStore {
    fn earned_badges(&self, profile_id: Uuid) -> Result<Vec<EarnedBadge>> {
        let connection = self.database.connection();
        let mut statement = connection
            .prepare(
                r#"SELECT "BadgeId", "EarnedAt" FROM "EarnedBadge" WHERE "ProfileId" = ?1 ORDER BY "EarnedAt", "BadgeId""#,
            )
            .map_err(storage)?;
        let badges = statement
            .query_map([id_text(profile_id)], |row| {
                Ok(EarnedBadge {
                    badge_id: row.get(0)?,
                    earned_at: row.get(1)?,
                })
            })
            .map_err(storage)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(storage);
        badges
    }

    fn add_badge(&self, profile_id: Uuid, badge_id: &str, earned_at: i64) -> Result<bool> {
        let now = self.database.now_ms();
        self.database
            .connection()
            .execute(
                r#"INSERT INTO "EarnedBadge" ("ProfileId", "BadgeId", "EarnedAt", "CreatedAt", "UpdatedAt")
                   VALUES (?1, ?2, ?3, ?4, ?4) ON CONFLICT DO NOTHING"#,
                rusqlite::params![id_text(profile_id), badge_id, earned_at, now],
            )
            .map(|count| count > 0)
            .map_err(storage)
    }

    fn unlocked_accessories(&self, profile_id: Uuid) -> Result<Vec<String>> {
        let connection = self.database.connection();
        let mut statement = connection
            .prepare(
                r#"SELECT "AccessoryId" FROM "UnlockedAccessory" WHERE "ProfileId" = ?1 ORDER BY "UnlockedAt", "AccessoryId""#,
            )
            .map_err(storage)?;
        let ids = statement
            .query_map([id_text(profile_id)], |row| row.get(0))
            .map_err(storage)?
            .collect::<rusqlite::Result<Vec<String>>>()
            .map_err(storage);
        ids
    }

    fn unlock_accessory(&self, profile_id: Uuid, accessory_id: &str, at: i64) -> Result<bool> {
        let now = self.database.now_ms();
        self.database
            .connection()
            .execute(
                r#"INSERT INTO "UnlockedAccessory" ("ProfileId", "AccessoryId", "UnlockedAt", "CreatedAt", "UpdatedAt")
                   VALUES (?1, ?2, ?3, ?4, ?4) ON CONFLICT DO NOTHING"#,
                rusqlite::params![id_text(profile_id), accessory_id, at, now],
            )
            .map(|count| count > 0)
            .map_err(storage)
    }

    fn goal_days(&self, profile_id: Uuid) -> Result<Vec<GoalDay>> {
        let connection = self.database.connection();
        let mut statement = connection
            .prepare(
                r#"SELECT "Day", "ReachedAt", "BonusPoints" FROM "DailyGoalDay" WHERE "ProfileId" = ?1 ORDER BY "Day""#,
            )
            .map_err(storage)?;
        let days = statement
            .query_map([id_text(profile_id)], |row| {
                Ok(GoalDay {
                    day: row.get(0)?,
                    reached_at: row.get(1)?,
                    bonus_points: row.get(2)?,
                })
            })
            .map_err(storage)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(storage);
        days
    }

    fn add_goal_day(&self, profile_id: Uuid, day: &GoalDay) -> Result<bool> {
        let now = self.database.now_ms();
        self.database
            .connection()
            .execute(
                r#"INSERT INTO "DailyGoalDay" ("ProfileId", "Day", "ReachedAt", "BonusPoints", "CreatedAt", "UpdatedAt")
                   VALUES (?1, ?2, ?3, ?4, ?5, ?5) ON CONFLICT DO NOTHING"#,
                rusqlite::params![
                    id_text(profile_id),
                    day.day,
                    day.reached_at,
                    day.bonus_points,
                    now
                ],
            )
            .map(|count| count > 0)
            .map_err(storage)
    }
}

/// [`ReviewStore`] backed by the `ReviewItem` table (SUB001509).
#[derive(Debug, Clone)]
pub struct SqliteReviewStore {
    database: Arc<Database>,
}

impl SqliteReviewStore {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

const REVIEW_COLUMNS: &str = r#""Id", "ProfileId", "SkillId", "Key", "Prompt", "Expected", "Word", "OriginSeed",
    "OriginAnswerMode", "OriginFormat", "OriginIndex", "Step", "DueDay", "CreatedAt", "UpdatedAt""#;

fn invalid(index: usize, message: String) -> rusqlite::Error {
    rusqlite::Error::FromSqlConversionFailure(index, rusqlite::types::Type::Text, message.into())
}

fn uuid_at(row: &Row<'_>, index: usize) -> rusqlite::Result<Uuid> {
    let text: String = row.get(index)?;
    Uuid::parse_str(&text).map_err(|e| invalid(index, e.to_string()))
}

fn read_review(row: &Row<'_>) -> rusqlite::Result<ReviewItem> {
    let seed: Option<u32> = row.get(7)?;
    let mode: Option<String> = row.get(8)?;
    let format: Option<String> = row.get(9)?;
    let index: Option<u32> = row.get(10)?;
    let origin = match (seed, mode.as_deref().and_then(AnswerMode::from_name), index) {
        (Some(seed), Some(answer_mode), Some(index)) => Some(ReviewOrigin {
            seed,
            answer_mode,
            format: format.as_deref().and_then(ExerciseFormat::from_name),
            index,
        }),
        _ => None,
    };
    Ok(ReviewItem {
        id: uuid_at(row, 0)?,
        profile_id: uuid_at(row, 1)?,
        skill_id: row.get(2)?,
        key: row.get(3)?,
        prompt: row.get(4)?,
        expected: row.get(5)?,
        word: row.get(6)?,
        origin,
        step: row.get(11)?,
        due_day: row.get(12)?,
        created_at: row.get(13)?,
        updated_at: row.get(14)?,
    })
}

impl ReviewStore for SqliteReviewStore {
    fn review_items(&self, profile_id: Uuid) -> Result<Vec<ReviewItem>> {
        let connection = self.database.connection();
        let mut statement = connection
            .prepare(&format!(
                r#"SELECT {REVIEW_COLUMNS} FROM "ReviewItem" WHERE "ProfileId" = ?1 ORDER BY "CreatedAt", "Id""#
            ))
            .map_err(storage)?;
        let items = statement
            .query_map([id_text(profile_id)], read_review)
            .map_err(storage)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(storage);
        items
    }

    fn review_item(&self, id: Uuid) -> Result<Option<ReviewItem>> {
        self.database
            .connection()
            .query_row(
                &format!(r#"SELECT {REVIEW_COLUMNS} FROM "ReviewItem" WHERE "Id" = ?1"#),
                [id_text(id)],
                read_review,
            )
            .optional()
            .map_err(storage)
    }

    fn save_review_item(&self, item: &ReviewItem) -> Result<()> {
        let now = self.database.now_ms();
        let origin = item.origin;
        self.database
            .connection()
            .execute(
                &format!(
                    r#"INSERT INTO "ReviewItem" ({REVIEW_COLUMNS})
                       VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?14)
                       ON CONFLICT ("Id") DO UPDATE SET "Prompt" = excluded."Prompt", "Expected" = excluded."Expected",
                       "Word" = excluded."Word", "OriginSeed" = excluded."OriginSeed",
                       "OriginAnswerMode" = excluded."OriginAnswerMode", "OriginFormat" = excluded."OriginFormat",
                       "OriginIndex" = excluded."OriginIndex", "Step" = excluded."Step", "DueDay" = excluded."DueDay",
                       "UpdatedAt" = excluded."UpdatedAt""#
                ),
                rusqlite::params![
                    id_text(item.id),
                    id_text(item.profile_id),
                    item.skill_id,
                    item.key,
                    item.prompt,
                    item.expected,
                    item.word,
                    origin.map(|o| o.seed),
                    origin.map(|o| o.answer_mode.as_str()),
                    origin.and_then(|o| o.format).map(|f| f.name()),
                    origin.map(|o| o.index),
                    item.step,
                    item.due_day,
                    now,
                ],
            )
            .map(|_| ())
            .map_err(storage)
    }

    fn delete_review_item(&self, id: Uuid) -> Result<bool> {
        self.database
            .connection()
            .execute(r#"DELETE FROM "ReviewItem" WHERE "Id" = ?1"#, [id_text(id)])
            .map(|count| count > 0)
            .map_err(storage)
    }
}

/// [`ProfileSettingsStore`] backed by the `ProfileSetting` table (SUB001512).
#[derive(Debug, Clone)]
pub struct SqliteProfileSettingsStore {
    database: Arc<Database>,
}

impl SqliteProfileSettingsStore {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

impl ProfileSettingsStore for SqliteProfileSettingsStore {
    fn profile_settings(&self, profile_id: Uuid) -> Result<Vec<(String, String)>> {
        let connection = self.database.connection();
        let mut statement = connection
            .prepare(
                r#"SELECT "Key", "Value" FROM "ProfileSetting" WHERE "ProfileId" = ?1 ORDER BY "Key""#,
            )
            .map_err(storage)?;
        let pairs = statement
            .query_map([id_text(profile_id)], |row| Ok((row.get(0)?, row.get(1)?)))
            .map_err(storage)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(storage);
        pairs
    }

    /// `UpdatedAt` only moves when the value actually changes.
    fn set_profile_setting(&self, profile_id: Uuid, key: &str, value: &str) -> Result<()> {
        let now = self.database.now_ms();
        self.database
            .connection()
            .execute(
                r#"INSERT INTO "ProfileSetting" ("ProfileId", "Key", "Value", "CreatedAt", "UpdatedAt") VALUES (?1, ?2, ?3, ?4, ?4)
                   ON CONFLICT ("ProfileId", "Key") DO UPDATE SET "Value" = excluded."Value", "UpdatedAt" = excluded."UpdatedAt"
                   WHERE "Value" <> excluded."Value""#,
                rusqlite::params![id_text(profile_id), key, value, now],
            )
            .map(|_| ())
            .map_err(storage)
    }
}
