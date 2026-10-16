//! Storage of the synchronisation (SUB027607, SUB027608, ADR 0007): the identity of the device,
//! the changes recorded by the triggers of migration 0005 (`SyncRow`), the paired devices
//! (`SyncPeer`), the export of the changes after a cursor and the application of a merge in one
//! transaction.

use crate::database::{storage, Database, DEVICE_ID_KEY, EPOCH_KEY};
use crate::motivation::{read_review, REVIEW_COLUMNS};
use crate::profiles::{avatar_json, id_text, COLUMNS as PROFILE_COLUMNS};
use crate::sessions::{read_answer, read_session, ANSWER_COLUMNS, SESSION_COLUMNS};
use educator_core::clock::uuid_v7;
use educator_core::profiles::Profile;
use educator_core::reviews::ReviewItem;
use educator_core::rewards::daily_goal::GoalDay;
use educator_core::sessions::{AnswerRecord, SessionRecord};
use educator_core::skills::{AnswerMode, ExerciseFormat};
use educator_core::sync::*;
use educator_core::{Error, Result};
use rusqlite::{Connection, OptionalExtension};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use uuid::Uuid;

/// Prefix of the profiles waiting for their masteries and points to be recomputed (`SyncState`).
const REFRESH_PREFIX: &str = "Refresh/";

/// Rows of `SyncRow` read at once by the export.
const SCAN_PAGE: i64 = 500;

/// [`SyncStore`] backed by the tables of migration 0005.
#[derive(Debug, Clone)]
pub struct SqliteSyncStore {
    database: Arc<Database>,
}

impl SqliteSyncStore {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

fn uuid_of(text: &str) -> Result<Uuid> {
    Uuid::parse_str(text).map_err(storage)
}

fn clock_of_text(text: &str) -> Result<Hlc> {
    Hlc::parse(text).ok_or_else(|| storage(format!("Malformed clock '{text}'.")))
}

/// Writes the clock of a record (and moves it to the end of the sequence).
fn set_clock(
    connection: &Connection,
    entity: Entity,
    key: &str,
    field: &str,
    clock: Hlc,
    deleted: bool,
    target: Option<Uuid>,
) -> Result<()> {
    connection
        .execute(r#"UPDATE "SyncCounter" SET "Seq" = "Seq" + 1"#, [])
        .map_err(storage)?;
    connection
        .execute(
            r#"INSERT INTO "SyncRow" ("Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target")
               VALUES (?1, ?2, ?3, ?4, (SELECT "Seq" FROM "SyncCounter"), ?5, ?6)
               ON CONFLICT ("Entity", "Key", "Field") DO UPDATE SET "Hlc" = excluded."Hlc", "Seq" = excluded."Seq",
                   "Deleted" = excluded."Deleted", "Target" = excluded."Target""#,
            rusqlite::params![
                entity.as_str(),
                key,
                field,
                clock.encode(),
                deleted,
                target.map(id_text)
            ],
        )
        .map(|_| ())
        .map_err(storage)
}

fn profile_clocks(connection: &Connection, id: Uuid) -> Result<ProfileClocks> {
    let mut clocks = ProfileClocks::all(Hlc::ZERO);
    let mut statement = connection
        .prepare(
            r#"SELECT "Field", "Hlc" FROM "SyncRow" WHERE "Entity" = 'Profile' AND "Key" = ?1 AND "Field" <> ''"#,
        )
        .map_err(storage)?;
    let rows = statement
        .query_map([id_text(id)], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(storage)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(storage)?;
    for (field, clock) in rows {
        if let Some(field) = ProfileField::from_name(&field) {
            clocks.set(field, clock_of_text(&clock)?);
        }
    }
    Ok(clocks)
}

fn profile_state(connection: &Connection, id: Uuid) -> Result<Option<ProfileState>> {
    let profile = connection
        .query_row(
            &format!(r#"SELECT {PROFILE_COLUMNS} FROM "Profile" WHERE "Id" = ?1"#),
            [id_text(id)],
            crate::profiles::read,
        )
        .optional()
        .map_err(storage)?;
    match profile {
        None => Ok(None),
        Some(profile) => Ok(Some(ProfileState::of(
            &profile,
            profile_clocks(connection, id)?,
        ))),
    }
}

fn removal(connection: &Connection, id: Uuid) -> Result<Option<ProfileRemoval>> {
    let found: Option<(String, Option<String>)> = connection
        .query_row(
            r#"SELECT "Hlc", "Target" FROM "SyncRow" WHERE "Entity" = 'Profile' AND "Key" = ?1 AND "Field" = '' AND "Deleted" = 1"#,
            [id_text(id)],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(storage)?;
    found
        .map(|(clock, target)| {
            Ok(ProfileRemoval {
                profile_id: id,
                clock: clock_of_text(&clock)?,
                merged_into: target.as_deref().map(uuid_of).transpose()?,
            })
        })
        .transpose()
}

fn exists(connection: &Connection, sql: &str, params: impl rusqlite::Params) -> Result<bool> {
    connection
        .query_row(&format!("SELECT EXISTS ({sql})"), params, |row| row.get(0))
        .map_err(storage)
}

/// The local state read by the merge engine, on the open transaction.
struct View<'a>(&'a Connection);

impl LocalView for View<'_> {
    fn profile(&self, id: Uuid) -> Result<Option<ProfileState>> {
        profile_state(self.0, id)
    }

    fn removal(&self, id: Uuid) -> Result<Option<ProfileRemoval>> {
        removal(self.0, id)
    }

    fn profile_names(&self) -> Result<Vec<(Uuid, String)>> {
        let mut statement = self
            .0
            .prepare(r#"SELECT "Id", "Name" FROM "Profile""#)
            .map_err(storage)?;
        let rows = statement
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(storage)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(storage)?;
        rows.into_iter()
            .map(|(id, name)| Ok((uuid_of(&id)?, name)))
            .collect()
    }

    fn has_session(&self, id: Uuid) -> Result<bool> {
        exists(
            self.0,
            r#"SELECT 1 FROM "Session" WHERE "Id" = ?1"#,
            [id_text(id)],
        )
    }

    fn clock_of(&self, entity: Entity, key: &str) -> Result<Option<Hlc>> {
        let found: Option<String> = self
            .0
            .query_row(
                r#"SELECT "Hlc" FROM "SyncRow" WHERE "Entity" = ?1 AND "Key" = ?2 AND "Field" = ''"#,
                [entity.as_str(), key],
                |row| row.get(0),
            )
            .optional()
            .map_err(storage)?;
        found.as_deref().map(clock_of_text).transpose()
    }

    fn badge(&self, profile_id: Uuid, badge_id: &str) -> Result<Option<i64>> {
        self.0
            .query_row(
                r#"SELECT "EarnedAt" FROM "EarnedBadge" WHERE "ProfileId" = ?1 AND "BadgeId" = ?2"#,
                rusqlite::params![id_text(profile_id), badge_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(storage)
    }

    fn accessory(&self, profile_id: Uuid, accessory_id: &str) -> Result<Option<i64>> {
        self.0
            .query_row(
                r#"SELECT "UnlockedAt" FROM "UnlockedAccessory" WHERE "ProfileId" = ?1 AND "AccessoryId" = ?2"#,
                rusqlite::params![id_text(profile_id), accessory_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(storage)
    }

    fn goal_day(&self, profile_id: Uuid, day: i64) -> Result<Option<GoalDay>> {
        self.0
            .query_row(
                r#"SELECT "Day", "ReachedAt", "BonusPoints" FROM "DailyGoalDay" WHERE "ProfileId" = ?1 AND "Day" = ?2"#,
                rusqlite::params![id_text(profile_id), day],
                |row| {
                    Ok(GoalDay {
                        day: row.get(0)?,
                        reached_at: row.get(1)?,
                        bonus_points: row.get(2)?,
                    })
                },
            )
            .optional()
            .map_err(storage)
    }
}

fn profile_of(state: &ProfileState) -> Profile {
    Profile {
        id: state.id,
        name: state.name.clone(),
        grade: state.grade,
        character_id: state.character_id.clone(),
        accessories: state.accessories.clone(),
        points: 0,
        placement_seconds: state.placement_seconds,
        exam_seconds: state.exam_seconds,
        created_at: state.created_at,
        updated_at: 0,
        last_used_at: state.last_used_at,
    }
}

fn write_profile(connection: &Connection, state: &ProfileState, now: i64) -> Result<()> {
    let profile = profile_of(state);
    let params = rusqlite::params![
        id_text(profile.id),
        profile.name,
        Profile::normalize_name(&profile.name),
        profile.grade.as_str(),
        profile.character_id,
        profile.created_at,
        now,
        profile.last_used_at,
        profile.placement_seconds,
        profile.exam_seconds,
        avatar_json(&profile),
    ];
    let updated = connection
        .execute(
            r#"UPDATE "Profile" SET "Name" = ?2, "NormalizedName" = ?3, "Grade" = ?4, "CharacterId" = ?5,
               "CreatedAt" = ?6, "UpdatedAt" = ?7, "LastUsedAt" = ?8, "PlacementSeconds" = ?9, "ExamSeconds" = ?10,
               "Avatar" = ?11 WHERE "Id" = ?1"#,
            params,
        )
        .map_err(storage)?;
    if updated == 0 {
        connection
            .execute(
                r#"INSERT INTO "Profile" ("Id", "Name", "NormalizedName", "Grade", "CharacterId", "Points", "CreatedAt",
                   "UpdatedAt", "LastUsedAt", "PlacementSeconds", "ExamSeconds", "Avatar")
                   VALUES (?1, ?2, ?3, ?4, ?5, 0, ?6, ?7, ?8, ?9, ?10, ?11)"#,
                params,
            )
            .map_err(storage)?;
    }
    let key = id_text(state.id);
    for field in ProfileField::ALL {
        set_clock(
            connection,
            Entity::Profile,
            &key,
            field.as_str(),
            state.clocks.get(field),
            false,
            None,
        )?;
    }
    Ok(())
}

/// Sends the records of `from` to `into` (« same child »), except those `into` already has.
fn move_records(connection: &Connection, from: Uuid, into: Uuid) -> Result<()> {
    let from = id_text(from);
    let into = id_text(into);
    let statements = [
        r#"UPDATE "Session" SET "ProfileId" = ?2 WHERE "ProfileId" = ?1"#,
        r#"UPDATE "EarnedBadge" SET "ProfileId" = ?2 WHERE "ProfileId" = ?1 AND NOT EXISTS
           (SELECT 1 FROM "EarnedBadge" o WHERE o."ProfileId" = ?2 AND o."BadgeId" = "EarnedBadge"."BadgeId")"#,
        r#"UPDATE "UnlockedAccessory" SET "ProfileId" = ?2 WHERE "ProfileId" = ?1 AND NOT EXISTS
           (SELECT 1 FROM "UnlockedAccessory" o WHERE o."ProfileId" = ?2 AND o."AccessoryId" = "UnlockedAccessory"."AccessoryId")"#,
        r#"UPDATE "DailyGoalDay" SET "ProfileId" = ?2 WHERE "ProfileId" = ?1 AND NOT EXISTS
           (SELECT 1 FROM "DailyGoalDay" o WHERE o."ProfileId" = ?2 AND o."Day" = "DailyGoalDay"."Day")"#,
        r#"UPDATE "LessonProgress" SET "ProfileId" = ?2 WHERE "ProfileId" = ?1 AND NOT EXISTS
           (SELECT 1 FROM "LessonProgress" o WHERE o."ProfileId" = ?2 AND o."LessonId" = "LessonProgress"."LessonId")"#,
        r#"UPDATE "ReviewItem" SET "ProfileId" = ?2 WHERE "ProfileId" = ?1 AND NOT EXISTS
           (SELECT 1 FROM "ReviewItem" o WHERE o."ProfileId" = ?2 AND o."SkillId" = "ReviewItem"."SkillId" AND o."Key" = "ReviewItem"."Key")"#,
        r#"UPDATE "ProfileSetting" SET "ProfileId" = ?2 WHERE "ProfileId" = ?1 AND NOT EXISTS
           (SELECT 1 FROM "ProfileSetting" o WHERE o."ProfileId" = ?2 AND o."Key" = "ProfileSetting"."Key")"#,
    ];
    for sql in statements {
        connection
            .execute(sql, rusqlite::params![from, into])
            .map_err(storage)?;
    }
    Ok(())
}

fn insert_session(connection: &Connection, change: &SessionChange, now: i64) -> Result<()> {
    let session = &change.session;
    connection
        .execute(
            &format!(
                r#"INSERT INTO "Session" ({SESSION_COLUMNS}, "CreatedAt", "UpdatedAt")
                   VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?18)"#
            ),
            rusqlite::params![
                id_text(session.id),
                id_text(session.profile_id),
                session.session_type.as_str(),
                session.subject.as_str(),
                session.grade.as_str(),
                session.skill_id,
                session.lesson_id,
                session.answer_mode.map(AnswerMode::as_str),
                session.format.map(ExerciseFormat::name),
                session.seed,
                session.question_count,
                session.time_limit_seconds,
                session.started_at,
                session.ended_at,
                session.abandoned,
                session.correct_count,
                session.points_earned,
                now,
            ],
        )
        .map_err(storage)?;
    for answer in &change.answers {
        connection
            .execute(
                &format!(
                    r#"INSERT OR IGNORE INTO "Answer" ({ANSWER_COLUMNS}) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"#
                ),
                rusqlite::params![
                    id_text(answer.id),
                    id_text(answer.session_id),
                    answer.index,
                    answer.skill_id,
                    answer.prompt,
                    answer.expected,
                    answer.given,
                    answer.is_correct,
                    answer.duration_ms,
                    answer.answered_at,
                ],
            )
            .map_err(storage)?;
    }
    set_clock(
        connection,
        Entity::Session,
        &id_text(session.id),
        "",
        change.clock,
        false,
        None,
    )
}

fn write_review(connection: &Connection, item: &ReviewItem, now: i64) -> Result<()> {
    // The identifier of the item on the other device, unless it is taken here by another item.
    let id_taken = exists(
        connection,
        r#"SELECT 1 FROM "ReviewItem" WHERE "Id" = ?1 AND NOT ("ProfileId" = ?2 AND "SkillId" = ?3 AND "Key" = ?4)"#,
        rusqlite::params![
            id_text(item.id),
            id_text(item.profile_id),
            item.skill_id,
            item.key
        ],
    )?;
    let id = if id_taken { uuid_v7(now) } else { item.id };
    let origin = item.origin;
    connection
        .execute(
            &format!(
                r#"INSERT INTO "ReviewItem" ({REVIEW_COLUMNS})
                   VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
                   ON CONFLICT ("ProfileId", "SkillId", "Key") DO UPDATE SET "Prompt" = excluded."Prompt",
                   "Expected" = excluded."Expected", "Word" = excluded."Word", "OriginSeed" = excluded."OriginSeed",
                   "OriginAnswerMode" = excluded."OriginAnswerMode", "OriginFormat" = excluded."OriginFormat",
                   "OriginIndex" = excluded."OriginIndex", "Step" = excluded."Step", "DueDay" = excluded."DueDay",
                   "UpdatedAt" = excluded."UpdatedAt""#
            ),
            rusqlite::params![
                id_text(id),
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
                item.created_at,
                now,
            ],
        )
        .map(|_| ())
        .map_err(storage)
}

/// Applies a plan of the merge engine on the open transaction.
fn apply_plan(connection: &Connection, plan: &MergePlan, database: &Database) -> Result<()> {
    let now = database.now_ms();
    for id in &plan.parked {
        connection
            .execute(
                r#"UPDATE "Profile" SET "NormalizedName" = '#' || "Id" WHERE "Id" = ?1"#,
                [id_text(*id)],
            )
            .map_err(storage)?;
    }
    for state in &plan.profiles {
        write_profile(connection, state, now)?;
    }
    for (id, name) in &plan.renames {
        connection
            .execute(
                r#"UPDATE "Profile" SET "Name" = ?2, "NormalizedName" = ?3, "UpdatedAt" = ?4 WHERE "Id" = ?1"#,
                rusqlite::params![id_text(*id), name, Profile::normalize_name(name), now],
            )
            .map_err(storage)?;
        // A change of this device: a new clock, sent to the others.
        let clock = database.sync_clock().tick(now).map_err(storage)?;
        set_clock(
            connection,
            Entity::Profile,
            &id_text(*id),
            ProfileField::Name.as_str(),
            clock,
            false,
            None,
        )?;
    }
    for planned in &plan.removals {
        let removal = planned.removal;
        if planned.exists {
            if let Some(target) = planned.move_to {
                move_records(connection, removal.profile_id, target)?;
            }
            connection
                .execute(
                    r#"DELETE FROM "Profile" WHERE "Id" = ?1"#,
                    [id_text(removal.profile_id)],
                )
                .map_err(storage)?;
        }
        set_clock(
            connection,
            Entity::Profile,
            &id_text(removal.profile_id),
            "",
            removal.clock,
            true,
            removal.merged_into,
        )?;
    }
    for change in &plan.sessions {
        insert_session(connection, change, now)?;
    }
    for change in &plan.lesson_progress {
        let profile = id_text(change.profile_id);
        match change.slide {
            Some(slide) => connection.execute(
                r#"INSERT INTO "LessonProgress" ("ProfileId", "LessonId", "SlideIndex", "CreatedAt", "UpdatedAt")
                   VALUES (?1, ?2, ?3, ?4, ?4)
                   ON CONFLICT ("ProfileId", "LessonId") DO UPDATE SET "SlideIndex" = excluded."SlideIndex",
                   "UpdatedAt" = excluded."UpdatedAt""#,
                rusqlite::params![profile, change.lesson_id, slide, now],
            ),
            None => connection.execute(
                r#"DELETE FROM "LessonProgress" WHERE "ProfileId" = ?1 AND "LessonId" = ?2"#,
                rusqlite::params![profile, change.lesson_id],
            ),
        }
        .map_err(storage)?;
        set_clock(
            connection,
            Entity::LessonProgress,
            &record_key(change.profile_id, &[&change.lesson_id]),
            "",
            change.clock,
            change.slide.is_none(),
            None,
        )?;
    }
    for change in &plan.badges {
        connection
            .execute(
                r#"INSERT INTO "EarnedBadge" ("ProfileId", "BadgeId", "EarnedAt", "CreatedAt", "UpdatedAt")
                   VALUES (?1, ?2, ?3, ?4, ?4)
                   ON CONFLICT ("ProfileId", "BadgeId") DO UPDATE SET "EarnedAt" = excluded."EarnedAt",
                   "UpdatedAt" = excluded."UpdatedAt" WHERE excluded."EarnedAt" < "EarnedAt""#,
                rusqlite::params![id_text(change.profile_id), change.badge_id, change.earned_at, now],
            )
            .map_err(storage)?;
        set_clock(
            connection,
            Entity::Badge,
            &record_key(change.profile_id, &[&change.badge_id]),
            "",
            change.clock,
            false,
            None,
        )?;
    }
    for change in &plan.accessories {
        connection
            .execute(
                r#"INSERT INTO "UnlockedAccessory" ("ProfileId", "AccessoryId", "UnlockedAt", "CreatedAt", "UpdatedAt")
                   VALUES (?1, ?2, ?3, ?4, ?4)
                   ON CONFLICT ("ProfileId", "AccessoryId") DO UPDATE SET "UnlockedAt" = excluded."UnlockedAt",
                   "UpdatedAt" = excluded."UpdatedAt" WHERE excluded."UnlockedAt" < "UnlockedAt""#,
                rusqlite::params![
                    id_text(change.profile_id),
                    change.accessory_id,
                    change.unlocked_at,
                    now
                ],
            )
            .map_err(storage)?;
        set_clock(
            connection,
            Entity::Accessory,
            &record_key(change.profile_id, &[&change.accessory_id]),
            "",
            change.clock,
            false,
            None,
        )?;
    }
    for change in &plan.goal_days {
        connection
            .execute(
                r#"INSERT INTO "DailyGoalDay" ("ProfileId", "Day", "ReachedAt", "BonusPoints", "CreatedAt", "UpdatedAt")
                   VALUES (?1, ?2, ?3, ?4, ?5, ?5)
                   ON CONFLICT ("ProfileId", "Day") DO UPDATE SET "ReachedAt" = excluded."ReachedAt",
                   "BonusPoints" = excluded."BonusPoints", "UpdatedAt" = excluded."UpdatedAt""#,
                rusqlite::params![
                    id_text(change.profile_id),
                    change.day.day,
                    change.day.reached_at,
                    change.day.bonus_points,
                    now
                ],
            )
            .map_err(storage)?;
        set_clock(
            connection,
            Entity::GoalDay,
            &record_key(change.profile_id, &[&change.day.day.to_string()]),
            "",
            change.clock,
            false,
            None,
        )?;
    }
    for change in &plan.reviews {
        match &change.item {
            Some(item) => write_review(connection, item, now)?,
            None => {
                connection
                    .execute(
                        r#"DELETE FROM "ReviewItem" WHERE "ProfileId" = ?1 AND "SkillId" = ?2 AND "Key" = ?3"#,
                        rusqlite::params![id_text(change.profile_id), change.skill_id, change.key],
                    )
                    .map_err(storage)?;
            }
        }
        set_clock(
            connection,
            Entity::Review,
            &record_key(change.profile_id, &[&change.skill_id, &change.key]),
            "",
            change.clock,
            change.item.is_none(),
            None,
        )?;
    }
    for change in &plan.settings {
        connection
            .execute(
                r#"INSERT INTO "ProfileSetting" ("ProfileId", "Key", "Value", "CreatedAt", "UpdatedAt") VALUES (?1, ?2, ?3, ?4, ?4)
                   ON CONFLICT ("ProfileId", "Key") DO UPDATE SET "Value" = excluded."Value", "UpdatedAt" = excluded."UpdatedAt""#,
                rusqlite::params![id_text(change.profile_id), change.key, change.value, now],
            )
            .map_err(storage)?;
        set_clock(
            connection,
            Entity::Setting,
            &record_key(change.profile_id, &[&change.key]),
            "",
            change.clock,
            false,
            None,
        )?;
    }
    for profile_id in &plan.touched_profiles {
        connection
            .execute(
                r#"INSERT INTO "SyncState" ("Key", "Value", "CreatedAt", "UpdatedAt") VALUES (?1, '', ?2, ?2)
                   ON CONFLICT ("Key") DO NOTHING"#,
                rusqlite::params![format!("{REFRESH_PREFIX}{}", id_text(*profile_id)), now],
            )
            .map_err(storage)?;
    }
    Ok(())
}

/// A closed session and its answers, as exported.
fn session_change(connection: &Connection, id: &str, clock: Hlc) -> Result<Option<SessionChange>> {
    let session: Option<SessionRecord> = connection
        .query_row(
            &format!(
                r#"SELECT {SESSION_COLUMNS} FROM "Session" WHERE "Id" = ?1 AND "EndedAt" IS NOT NULL"#
            ),
            [id],
            read_session,
        )
        .optional()
        .map_err(storage)?;
    let Some(session) = session else {
        return Ok(None);
    };
    let mut statement = connection
        .prepare(&format!(
            r#"SELECT {ANSWER_COLUMNS} FROM "Answer" WHERE "SessionId" = ?1 ORDER BY "Index""#
        ))
        .map_err(storage)?;
    let answers = statement
        .query_map([id], read_answer)
        .map_err(storage)?
        .collect::<rusqlite::Result<Vec<AnswerRecord>>>()
        .map_err(storage)?;
    Ok(Some(SessionChange {
        session,
        answers,
        clock,
    }))
}

struct ScannedRow {
    entity: String,
    key: String,
    field: String,
    clock: String,
    seq: i64,
    deleted: bool,
    target: Option<String>,
}

fn export(connection: &Connection, request: &ExportRequest) -> Result<ExportPage> {
    let mut batch = ChangeBatch::default();
    let mut profiles: BTreeSet<Uuid> = BTreeSet::new();
    let mut items = 0usize;
    let mut last_seq = request.after_seq;
    let profile_exists = |id: Uuid| -> Result<bool> {
        exists(
            connection,
            r#"SELECT 1 FROM "Profile" WHERE "Id" = ?1"#,
            [id_text(id)],
        )
    };
    'scan: loop {
        let mut statement = connection
            .prepare(
                r#"SELECT "Entity", "Key", "Field", "Hlc", "Seq", "Deleted", "Target" FROM "SyncRow"
                   WHERE "Seq" > ?1 ORDER BY "Seq" LIMIT ?2"#,
            )
            .map_err(storage)?;
        let rows = statement
            .query_map(rusqlite::params![last_seq, SCAN_PAGE], |row| {
                Ok(ScannedRow {
                    entity: row.get(0)?,
                    key: row.get(1)?,
                    field: row.get(2)?,
                    clock: row.get(3)?,
                    seq: row.get(4)?,
                    deleted: row.get(5)?,
                    target: row.get(6)?,
                })
            })
            .map_err(storage)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(storage)?;
        if rows.is_empty() {
            break;
        }
        for row in rows {
            if batch.sessions.len() >= request.max_sessions || items >= request.max_items {
                break 'scan;
            }
            let clock = clock_of_text(&row.clock)?;
            if Some(clock.device) == request.exclude_device {
                last_seq = row.seq;
                continue;
            }
            let Ok(entity) = row.entity.parse::<Entity>() else {
                last_seq = row.seq;
                continue;
            };
            match entity {
                Entity::Profile => {
                    let id = uuid_of(&row.key)?;
                    if row.field.is_empty() && row.deleted {
                        let merged_into = row.target.as_deref().map(uuid_of).transpose()?;
                        if let Some(target) = merged_into {
                            profiles.insert(target);
                        }
                        batch.removals.push(ProfileRemoval {
                            profile_id: id,
                            clock,
                            merged_into,
                        });
                        items += 1;
                    } else if !row.field.is_empty() && profiles.insert(id) {
                        items += 1;
                    }
                }
                Entity::Session => {
                    if let Some(change) = session_change(connection, &row.key, clock)? {
                        items += 1 + change.answers.len();
                        profiles.insert(change.session.profile_id);
                        batch.sessions.push(change);
                    }
                }
                _ => {
                    let (profile_id, rest) = split_record_key(&row.key).ok_or_else(|| {
                        storage(format!("Malformed key in SyncRow ({entity:?})."))
                    })?;
                    if profile_exists(profile_id)?
                        && keyed_change(
                            connection,
                            &mut batch,
                            entity,
                            profile_id,
                            rest,
                            clock,
                            row.deleted,
                        )?
                    {
                        profiles.insert(profile_id);
                        items += 1;
                    }
                }
            }
            last_seq = row.seq;
        }
    }
    for id in profiles {
        if let Some(state) = profile_state(connection, id)? {
            batch.profiles.push(state);
        }
    }
    let more = exists(
        connection,
        r#"SELECT 1 FROM "SyncRow" WHERE "Seq" > ?1"#,
        [last_seq],
    )?;
    Ok(ExportPage {
        batch,
        last_seq,
        more,
    })
}

/// Adds the keyed record to the batch; `false` when it is gone (and not a tombstone).
fn keyed_change(
    connection: &Connection,
    batch: &mut ChangeBatch,
    entity: Entity,
    profile_id: Uuid,
    rest: &str,
    clock: Hlc,
    deleted: bool,
) -> Result<bool> {
    let profile = id_text(profile_id);
    match entity {
        Entity::LessonProgress => {
            let slide: Option<u32> = if deleted {
                None
            } else {
                let found = connection
                    .query_row(
                        r#"SELECT "SlideIndex" FROM "LessonProgress" WHERE "ProfileId" = ?1 AND "LessonId" = ?2"#,
                        rusqlite::params![profile, rest],
                        |row| row.get(0),
                    )
                    .optional()
                    .map_err(storage)?;
                if found.is_none() {
                    return Ok(false);
                }
                found
            };
            batch.lesson_progress.push(LessonProgressChange {
                profile_id,
                lesson_id: rest.to_owned(),
                slide,
                clock,
            });
        }
        Entity::Badge => {
            let Some(earned_at) = View(connection).badge(profile_id, rest)? else {
                return Ok(false);
            };
            batch.badges.push(BadgeChange {
                profile_id,
                badge_id: rest.to_owned(),
                earned_at,
                clock,
            });
        }
        Entity::Accessory => {
            let Some(unlocked_at) = View(connection).accessory(profile_id, rest)? else {
                return Ok(false);
            };
            batch.accessories.push(AccessoryChange {
                profile_id,
                accessory_id: rest.to_owned(),
                unlocked_at,
                clock,
            });
        }
        Entity::GoalDay => {
            let Ok(day) = rest.parse::<i64>() else {
                return Ok(false);
            };
            let Some(day) = View(connection).goal_day(profile_id, day)? else {
                return Ok(false);
            };
            batch.goal_days.push(GoalDayChange {
                profile_id,
                day,
                clock,
            });
        }
        Entity::Review => {
            let Some((skill_id, key)) = rest.split_once('/') else {
                return Ok(false);
            };
            let item = if deleted {
                None
            } else {
                let found = connection
                    .query_row(
                        &format!(
                            r#"SELECT {REVIEW_COLUMNS} FROM "ReviewItem" WHERE "ProfileId" = ?1 AND "SkillId" = ?2 AND "Key" = ?3"#
                        ),
                        rusqlite::params![profile, skill_id, key],
                        read_review,
                    )
                    .optional()
                    .map_err(storage)?;
                if found.is_none() {
                    return Ok(false);
                }
                found
            };
            batch.reviews.push(ReviewChange {
                profile_id,
                skill_id: skill_id.to_owned(),
                key: key.to_owned(),
                item,
                clock,
            });
        }
        Entity::Setting => {
            let value: Option<String> = connection
                .query_row(
                    r#"SELECT "Value" FROM "ProfileSetting" WHERE "ProfileId" = ?1 AND "Key" = ?2"#,
                    rusqlite::params![profile, rest],
                    |row| row.get(0),
                )
                .optional()
                .map_err(storage)?;
            let Some(value) = value else {
                return Ok(false);
            };
            batch.settings.push(SettingChange {
                profile_id,
                key: rest.to_owned(),
                value,
                clock,
            });
        }
        Entity::Profile | Entity::Session => return Ok(false),
    }
    Ok(true)
}

fn read_peer(row: &rusqlite::Row<'_>) -> rusqlite::Result<(String, SyncPeerRow)> {
    Ok((
        row.get(0)?,
        SyncPeerRow {
            role: row.get(1)?,
            name: row.get(2)?,
            token_hash: row.get(3)?,
            token: row.get(4)?,
            fingerprint: row.get(5)?,
            addresses: row.get(6)?,
            port: row.get(7)?,
            paired_at: row.get(8)?,
            last_sync_at: row.get(9)?,
            pushed_seq: row.get(10)?,
            pulled_seq: row.get(11)?,
            revoked: row.get(12)?,
            epoch: row.get(13)?,
        },
    ))
}

struct SyncPeerRow {
    role: String,
    name: String,
    token_hash: Option<String>,
    token: Option<String>,
    fingerprint: Option<String>,
    addresses: String,
    port: Option<u16>,
    paired_at: i64,
    last_sync_at: Option<i64>,
    pushed_seq: i64,
    pulled_seq: i64,
    revoked: bool,
    epoch: Option<String>,
}

const PEER_COLUMNS: &str = r#""DeviceId", "Role", "Name", "TokenHash", "Token", "Fingerprint", "Addresses", "Port",
    "PairedAt", "LastSyncAt", "PushedSeq", "PulledSeq", "Revoked", "Epoch""#;

fn peer_of((id, row): (String, SyncPeerRow)) -> Result<SyncPeer> {
    Ok(SyncPeer {
        device_id: uuid_of(&id)?,
        role: row.role.parse().map_err(storage)?,
        name: row.name,
        token_hash: row.token_hash,
        token: row.token,
        fingerprint: row.fingerprint,
        addresses: row
            .addresses
            .split(' ')
            .filter(|a| !a.is_empty())
            .map(str::to_owned)
            .collect(),
        port: row.port,
        paired_at: row.paired_at,
        last_sync_at: row.last_sync_at,
        pushed_seq: row.pushed_seq,
        pulled_seq: row.pulled_seq,
        revoked: row.revoked,
        epoch: row.epoch,
    })
}

impl SyncStore for SqliteSyncStore {
    fn device_id(&self) -> Result<Uuid> {
        self.database
            .sync_clock()
            .device()
            .ok_or_else(|| storage("The identity of the device is not ready."))
    }

    fn profile_states(&self) -> Result<Vec<ProfileState>> {
        let connection = self.database.connection();
        let ids = View(&connection).profile_names()?;
        let mut states = Vec::new();
        for (id, _) in ids {
            if let Some(state) = profile_state(&connection, id)? {
                states.push(state);
            }
        }
        states.sort_by_key(|s| s.id);
        Ok(states)
    }

    fn profile_exists(&self, id: Uuid) -> Result<bool> {
        exists(
            &self.database.connection(),
            r#"SELECT 1 FROM "Profile" WHERE "Id" = ?1"#,
            [id_text(id)],
        )
    }

    fn export(&self, request: &ExportRequest) -> Result<ExportPage> {
        let mut connection = self.database.connection();
        // A consistent picture of the database while it is read.
        let transaction = connection.transaction().map_err(storage)?;
        let page = export(&transaction, request)?;
        transaction.commit().map_err(storage)?;
        Ok(page)
    }

    fn apply(&self, batch: &ChangeBatch, pulled: Option<(Uuid, i64)>) -> Result<ApplyReport> {
        let mut connection = self.database.connection();
        let transaction = connection.transaction().map_err(storage)?;
        let plan = merge(&View(&transaction), batch)?;
        if let Some(clock) = plan.max_clock {
            self.database.sync_clock().observe(clock);
        }
        apply_plan(&transaction, &plan, &self.database)?;
        if let Some((device, seq)) = pulled {
            transaction
                .execute(
                    r#"UPDATE "SyncPeer" SET "PulledSeq" = ?2, "UpdatedAt" = ?3 WHERE "DeviceId" = ?1"#,
                    rusqlite::params![id_text(device), seq, self.database.now_ms()],
                )
                .map_err(storage)?;
        }
        transaction.commit().map_err(storage)?;
        Ok(ApplyReport {
            sessions_added: plan.sessions.len(),
            profiles_written: plan.profiles.len(),
            profiles_removed: plan.removals.iter().filter(|r| r.exists).count(),
            touched_profiles: plan.touched_profiles,
        })
    }

    fn link_profile(&self, local_id: Uuid, remote: &ProfileState) -> Result<ApplyReport> {
        if local_id == remote.id {
            return Err(Error::InvalidRequest(
                "A profile cannot be linked to itself.".to_owned(),
            ));
        }
        if !self.profile_exists(local_id)? {
            return Err(Error::ProfileNotFound(local_id));
        }
        // A profile removed here meanwhile: linking to it would delete the child's records.
        if self.is_removed(remote.id)? {
            return Err(Error::InvalidRequest(
                "The profile of the other device was removed here.".to_owned(),
            ));
        }
        let clock = self
            .database
            .sync_clock()
            .tick(self.database.now_ms())
            .map_err(storage)?;
        let batch = ChangeBatch {
            profiles: vec![remote.clone()],
            removals: vec![ProfileRemoval {
                profile_id: local_id,
                clock,
                merged_into: Some(remote.id),
            }],
            ..ChangeBatch::default()
        };
        self.apply(&batch, None)
    }

    fn pending_refresh(&self) -> Result<Vec<Uuid>> {
        let connection = self.database.connection();
        let mut statement = connection
            .prepare(r#"SELECT "Key" FROM "SyncState" WHERE "Key" LIKE 'Refresh/%' ORDER BY "Key""#)
            .map_err(storage)?;
        let keys = statement
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(storage)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(storage)?;
        keys.iter()
            .filter_map(|k| k.strip_prefix(REFRESH_PREFIX))
            .map(uuid_of)
            .collect()
    }

    fn clear_refresh(&self, profile_id: Uuid) -> Result<()> {
        self.database
            .connection()
            .execute(
                r#"DELETE FROM "SyncState" WHERE "Key" = ?1"#,
                [format!("{REFRESH_PREFIX}{}", id_text(profile_id))],
            )
            .map(|_| ())
            .map_err(storage)
    }

    fn set_points(&self, profile_id: Uuid, points: i32) -> Result<()> {
        self.database
            .connection()
            .execute(
                r#"UPDATE "Profile" SET "Points" = ?2 WHERE "Id" = ?1"#,
                rusqlite::params![id_text(profile_id), points.max(0)],
            )
            .map(|_| ())
            .map_err(storage)
    }

    fn peers(&self) -> Result<Vec<SyncPeer>> {
        let connection = self.database.connection();
        let mut statement = connection
            .prepare(&format!(
                r#"SELECT {PEER_COLUMNS} FROM "SyncPeer" ORDER BY "PairedAt", "DeviceId""#
            ))
            .map_err(storage)?;
        let rows = statement
            .query_map([], read_peer)
            .map_err(storage)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(storage)?;
        rows.into_iter().map(peer_of).collect()
    }

    fn peer(&self, device_id: Uuid) -> Result<Option<SyncPeer>> {
        let row = self
            .database
            .connection()
            .query_row(
                &format!(r#"SELECT {PEER_COLUMNS} FROM "SyncPeer" WHERE "DeviceId" = ?1"#),
                [id_text(device_id)],
                read_peer,
            )
            .optional()
            .map_err(storage)?;
        row.map(peer_of).transpose()
    }

    fn save_peer(&self, peer: &SyncPeer) -> Result<()> {
        let now = self.database.now_ms();
        self.database
            .connection()
            .execute(
                &format!(
                    r#"INSERT INTO "SyncPeer" ({PEER_COLUMNS}, "CreatedAt", "UpdatedAt")
                       VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?15)
                       ON CONFLICT ("DeviceId") DO UPDATE SET "Role" = excluded."Role", "Name" = excluded."Name",
                       "TokenHash" = excluded."TokenHash", "Token" = excluded."Token", "Fingerprint" = excluded."Fingerprint",
                       "Addresses" = excluded."Addresses", "Port" = excluded."Port", "PairedAt" = excluded."PairedAt",
                       "LastSyncAt" = excluded."LastSyncAt", "PushedSeq" = excluded."PushedSeq",
                       "PulledSeq" = excluded."PulledSeq", "Revoked" = excluded."Revoked", "Epoch" = excluded."Epoch",
                       "UpdatedAt" = excluded."UpdatedAt""#
                ),
                rusqlite::params![
                    id_text(peer.device_id),
                    peer.role.as_str(),
                    peer.name,
                    peer.token_hash,
                    peer.token,
                    peer.fingerprint,
                    peer.addresses.join(" "),
                    peer.port,
                    peer.paired_at,
                    peer.last_sync_at,
                    peer.pushed_seq,
                    peer.pulled_seq,
                    peer.revoked,
                    peer.epoch,
                    now,
                ],
            )
            .map(|_| ())
            .map_err(storage)
    }

    fn delete_peer(&self, device_id: Uuid) -> Result<bool> {
        self.database
            .connection()
            .execute(
                r#"DELETE FROM "SyncPeer" WHERE "DeviceId" = ?1"#,
                [id_text(device_id)],
            )
            .map(|n| n > 0)
            .map_err(storage)
    }

    fn record_push(&self, device_id: Uuid, seq: i64) -> Result<()> {
        self.database
            .connection()
            .execute(
                r#"UPDATE "SyncPeer" SET "PushedSeq" = ?2, "UpdatedAt" = ?3 WHERE "DeviceId" = ?1 AND "Revoked" = 0"#,
                rusqlite::params![id_text(device_id), seq, self.database.now_ms()],
            )
            .map(|_| ())
            .map_err(storage)
    }

    fn record_sync(&self, device_id: Uuid, at: i64, addresses: Option<&[String]>) -> Result<()> {
        self.database
            .connection()
            .execute(
                r#"UPDATE "SyncPeer" SET "LastSyncAt" = ?2, "Addresses" = COALESCE(?3, "Addresses"), "UpdatedAt" = ?4
                   WHERE "DeviceId" = ?1 AND "Revoked" = 0"#,
                rusqlite::params![
                    id_text(device_id),
                    at,
                    addresses.map(|a| a.join(" ")),
                    self.database.now_ms()
                ],
            )
            .map(|_| ())
            .map_err(storage)
    }

    fn reset_cursors(&self, device_id: Uuid, epoch: &str) -> Result<()> {
        self.database
            .connection()
            .execute(
                r#"UPDATE "SyncPeer" SET "PushedSeq" = 0, "PulledSeq" = 0, "Epoch" = ?2, "UpdatedAt" = ?3
                   WHERE "DeviceId" = ?1 AND "Revoked" = 0"#,
                rusqlite::params![id_text(device_id), epoch, self.database.now_ms()],
            )
            .map(|_| ())
            .map_err(storage)
    }

    fn epoch(&self) -> Result<String> {
        self.value(EPOCH_KEY)?
            .ok_or_else(|| storage("The epoch of the device is not ready."))
    }

    fn is_removed(&self, profile_id: Uuid) -> Result<bool> {
        Ok(removal(&self.database.connection(), profile_id)?.is_some())
    }

    fn value(&self, key: &str) -> Result<Option<String>> {
        self.database
            .connection()
            .query_row(
                r#"SELECT "Value" FROM "SyncState" WHERE "Key" = ?1"#,
                [key],
                |row| row.get(0),
            )
            .optional()
            .map_err(storage)
    }

    fn set_value(&self, key: &str, value: &str) -> Result<()> {
        if key == DEVICE_ID_KEY || key == EPOCH_KEY || key.starts_with(REFRESH_PREFIX) {
            return Err(Error::InvalidRequest(format!("'{key}' is reserved.")));
        }
        let now = self.database.now_ms();
        self.database
            .connection()
            .execute(
                r#"INSERT INTO "SyncState" ("Key", "Value", "CreatedAt", "UpdatedAt") VALUES (?1, ?2, ?3, ?3)
                   ON CONFLICT ("Key") DO UPDATE SET "Value" = excluded."Value", "UpdatedAt" = excluded."UpdatedAt"
                   WHERE "Value" <> excluded."Value""#,
                rusqlite::params![key, value, now],
            )
            .map(|_| ())
            .map_err(storage)
    }
}

/// Counts of the records followed, by entity (diagnostics and tests).
pub fn followed_counts(database: &Database) -> Result<BTreeMap<String, i64>> {
    let connection = database.connection();
    let mut statement = connection
        .prepare(r#"SELECT "Entity", COUNT(*) FROM "SyncRow" GROUP BY "Entity""#)
        .map_err(storage)?;
    let rows = statement
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .map_err(storage)?
        .collect::<rusqlite::Result<BTreeMap<String, i64>>>()
        .map_err(storage);
    rows
}
