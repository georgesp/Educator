use crate::database::{storage, Database};
use crate::profiles::id_text;
use educator_core::sessions::{
    AnswerRecord, SessionEnd, SessionRecord, SessionStore, SessionType, SkillMastery,
};
use educator_core::skills::{AnswerMode, ExerciseFormat, Subject};
use educator_core::{Grade, Result};
use rusqlite::{OptionalExtension, Row};
use std::sync::Arc;
use uuid::Uuid;

pub(crate) const SESSION_COLUMNS: &str = r#""Id", "ProfileId", "Type", "Subject", "Grade", "SkillId", "LessonId", "AnswerMode",
    "Format", "Seed", "QuestionCount", "TimeLimitSeconds", "StartedAt", "EndedAt", "Abandoned", "CorrectCount",
    "PointsEarned""#;

pub(crate) const ANSWER_COLUMNS: &str = r#""Id", "SessionId", "Index", "SkillId", "Prompt", "Expected", "Given", "IsCorrect",
    "DurationMs", "AnsweredAt""#;

/// [`SessionStore`] backed by the `Session`, `Answer` and `SkillMastery` tables (SUB001401).
#[derive(Debug, Clone)]
pub struct SqliteSessionStore {
    database: Arc<Database>,
}

impl SqliteSessionStore {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

fn invalid(index: usize, message: String) -> rusqlite::Error {
    rusqlite::Error::FromSqlConversionFailure(index, rusqlite::types::Type::Text, message.into())
}

fn uuid_at(row: &Row<'_>, index: usize) -> rusqlite::Result<Uuid> {
    let text: String = row.get(index)?;
    Uuid::parse_str(&text).map_err(|e| invalid(index, e.to_string()))
}

pub(crate) fn read_session(row: &Row<'_>) -> rusqlite::Result<SessionRecord> {
    let session_type: String = row.get(2)?;
    let subject: String = row.get(3)?;
    let grade: String = row.get(4)?;
    let answer_mode: Option<String> = row.get(7)?;
    let format: Option<String> = row.get(8)?;
    Ok(SessionRecord {
        id: uuid_at(row, 0)?,
        profile_id: uuid_at(row, 1)?,
        session_type: session_type
            .parse::<SessionType>()
            .map_err(|e| invalid(2, e.to_string()))?,
        subject: Subject::from_name(&subject)
            .ok_or_else(|| invalid(3, format!("Unknown subject '{subject}'.")))?,
        grade: grade
            .parse::<Grade>()
            .map_err(|e| invalid(4, e.to_string()))?,
        skill_id: row.get(5)?,
        lesson_id: row.get(6)?,
        answer_mode: answer_mode
            .map(|m| {
                AnswerMode::from_name(&m).ok_or_else(|| invalid(7, format!("Unknown mode '{m}'.")))
            })
            .transpose()?,
        format: format
            .map(|f| {
                ExerciseFormat::from_name(&f)
                    .ok_or_else(|| invalid(8, format!("Unknown format '{f}'.")))
            })
            .transpose()?,
        seed: row.get(9)?,
        question_count: row.get(10)?,
        time_limit_seconds: row.get(11)?,
        started_at: row.get(12)?,
        ended_at: row.get(13)?,
        abandoned: row.get(14)?,
        correct_count: row.get(15)?,
        points_earned: row.get(16)?,
    })
}

pub(crate) fn read_answer(row: &Row<'_>) -> rusqlite::Result<AnswerRecord> {
    Ok(AnswerRecord {
        id: uuid_at(row, 0)?,
        session_id: uuid_at(row, 1)?,
        index: row.get(2)?,
        skill_id: row.get(3)?,
        prompt: row.get(4)?,
        expected: row.get(5)?,
        given: row.get(6)?,
        is_correct: row.get(7)?,
        duration_ms: row.get(8)?,
        answered_at: row.get(9)?,
    })
}

fn read_mastery(row: &Row<'_>) -> rusqlite::Result<SkillMastery> {
    Ok(SkillMastery {
        profile_id: uuid_at(row, 0)?,
        skill_id: row.get(1)?,
        score: row.get(2)?,
        attempt_count: row.get(3)?,
        last_practiced_at: row.get(4)?,
    })
}

impl SessionStore for SqliteSessionStore {
    fn add_session(&self, session: &SessionRecord) -> Result<()> {
        let now = self.database.now_ms();
        self.database
            .connection()
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
            .map(|_| ())
            .map_err(storage)
    }

    fn session(&self, id: Uuid) -> Result<Option<SessionRecord>> {
        self.database
            .connection()
            .query_row(
                &format!(r#"SELECT {SESSION_COLUMNS} FROM "Session" WHERE "Id" = ?1"#),
                [id_text(id)],
                read_session,
            )
            .optional()
            .map_err(storage)
    }

    fn add_answer(&self, answer: &AnswerRecord) -> Result<bool> {
        self.database
            .connection()
            .execute(
                &format!(
                    r#"INSERT INTO "Answer" ({ANSWER_COLUMNS}) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                       ON CONFLICT ("SessionId", "Index") DO NOTHING"#
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
            .map(|count| count > 0)
            .map_err(storage)
    }

    fn answers(&self, session_id: Uuid) -> Result<Vec<AnswerRecord>> {
        let connection = self.database.connection();
        let mut statement = connection
            .prepare(&format!(
                r#"SELECT {ANSWER_COLUMNS} FROM "Answer" WHERE "SessionId" = ?1 ORDER BY "Index""#
            ))
            .map_err(storage)?;
        let rows = statement
            .query_map([id_text(session_id)], read_answer)
            .map_err(storage)?;
        rows.collect::<rusqlite::Result<Vec<_>>>().map_err(storage)
    }

    fn end_session(&self, id: Uuid, end: &SessionEnd) -> Result<bool> {
        let now = self.database.now_ms();
        self.database
            .connection()
            .execute(
                r#"UPDATE "Session" SET "EndedAt" = ?2, "Abandoned" = ?3, "QuestionCount" = ?4, "CorrectCount" = ?5,
                   "PointsEarned" = ?6, "UpdatedAt" = ?7 WHERE "Id" = ?1 AND "EndedAt" IS NULL"#,
                rusqlite::params![
                    id_text(id),
                    end.ended_at,
                    end.abandoned,
                    end.question_count,
                    end.correct_count,
                    end.points_earned,
                    now,
                ],
            )
            .map(|count| count > 0)
            .map_err(storage)
    }

    /// The end is the last answer when there is one, otherwise the start.
    fn open_sessions(&self) -> Result<Vec<SessionRecord>> {
        let connection = self.database.connection();
        let mut statement = connection
            .prepare(&format!(
                r#"SELECT {SESSION_COLUMNS} FROM "Session" WHERE "EndedAt" IS NULL"#
            ))
            .map_err(storage)?;
        let rows = statement.query_map([], read_session).map_err(storage)?;
        rows.collect::<rusqlite::Result<Vec<_>>>().map_err(storage)
    }

    fn abandon_open_sessions(&self, now: i64) -> Result<usize> {
        self.database
            .connection()
            .execute(
                r#"UPDATE "Session" SET "Abandoned" = 1, "UpdatedAt" = ?1,
                   "EndedAt" = COALESCE((SELECT MAX("AnsweredAt") FROM "Answer" WHERE "SessionId" = "Session"."Id"), "StartedAt"),
                   "CorrectCount" = (SELECT COUNT(*) FROM "Answer" WHERE "SessionId" = "Session"."Id" AND "IsCorrect" = 1)
                   WHERE "EndedAt" IS NULL"#,
                [now],
            )
            .map_err(storage)
    }

    fn recent_results(&self, profile_id: Uuid, skill_id: &str, limit: usize) -> Result<Vec<bool>> {
        let connection = self.database.connection();
        let mut statement = connection
            .prepare(
                r#"SELECT a."IsCorrect" FROM "Answer" a JOIN "Session" s ON s."Id" = a."SessionId"
                   WHERE s."ProfileId" = ?1 AND a."SkillId" = ?2
                   AND NOT (s."Type" = 'Placement' AND s."Abandoned" = 1)
                   ORDER BY a."AnsweredAt" DESC, a."Index" DESC LIMIT ?3"#,
            )
            .map_err(storage)?;
        let limit = i64::try_from(limit).unwrap_or(i64::MAX);
        let rows = statement
            .query_map(
                rusqlite::params![id_text(profile_id), skill_id, limit],
                |row| row.get::<_, bool>(0),
            )
            .map_err(storage)?;
        rows.collect::<rusqlite::Result<Vec<_>>>().map_err(storage)
    }

    fn answer_count(&self, profile_id: Uuid, skill_id: &str) -> Result<u32> {
        self.database
            .connection()
            .query_row(
                r#"SELECT COUNT(*) FROM "Answer" a JOIN "Session" s ON s."Id" = a."SessionId"
                   WHERE s."ProfileId" = ?1 AND a."SkillId" = ?2
                   AND NOT (s."Type" = 'Placement' AND s."Abandoned" = 1)"#,
                rusqlite::params![id_text(profile_id), skill_id],
                |row| row.get(0),
            )
            .map_err(storage)
    }

    /// `UpdatedAt` only moves when a column actually changes.
    fn save_mastery(&self, mastery: &SkillMastery) -> Result<()> {
        let now = self.database.now_ms();
        self.database
            .connection()
            .execute(
                r#"INSERT INTO "SkillMastery" ("ProfileId", "SkillId", "Score", "AttemptCount", "LastPracticedAt", "CreatedAt", "UpdatedAt")
                   VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6)
                   ON CONFLICT ("ProfileId", "SkillId") DO UPDATE SET "Score" = excluded."Score",
                       "AttemptCount" = excluded."AttemptCount", "LastPracticedAt" = excluded."LastPracticedAt",
                       "UpdatedAt" = excluded."UpdatedAt"
                   WHERE "Score" <> excluded."Score" OR "AttemptCount" <> excluded."AttemptCount"
                       OR "LastPracticedAt" <> excluded."LastPracticedAt""#,
                rusqlite::params![
                    id_text(mastery.profile_id),
                    mastery.skill_id,
                    mastery.score,
                    mastery.attempt_count,
                    mastery.last_practiced_at,
                    now,
                ],
            )
            .map(|_| ())
            .map_err(storage)
    }

    fn masteries(&self, profile_id: Uuid) -> Result<Vec<SkillMastery>> {
        let connection = self.database.connection();
        let mut statement = connection
            .prepare(
                r#"SELECT "ProfileId", "SkillId", "Score", "AttemptCount", "LastPracticedAt" FROM "SkillMastery"
                   WHERE "ProfileId" = ?1 ORDER BY "SkillId""#,
            )
            .map_err(storage)?;
        let rows = statement
            .query_map([id_text(profile_id)], read_mastery)
            .map_err(storage)?;
        rows.collect::<rusqlite::Result<Vec<_>>>().map_err(storage)
    }

    fn profile_sessions(&self, profile_id: Uuid) -> Result<Vec<SessionRecord>> {
        let connection = self.database.connection();
        let mut statement = connection
            .prepare(&format!(
                r#"SELECT {SESSION_COLUMNS} FROM "Session" WHERE "ProfileId" = ?1 AND "EndedAt" IS NOT NULL
                   AND NOT ("Type" = 'Placement' AND "Abandoned" = 1)
                   ORDER BY "StartedAt", "Id""#
            ))
            .map_err(storage)?;
        let rows = statement
            .query_map([id_text(profile_id)], read_session)
            .map_err(storage)?;
        rows.collect::<rusqlite::Result<Vec<_>>>().map_err(storage)
    }

    fn profile_answers(&self, profile_id: Uuid) -> Result<Vec<AnswerRecord>> {
        let connection = self.database.connection();
        let columns = ANSWER_COLUMNS
            .split(',')
            .map(|c| format!("a.{}", c.trim()))
            .collect::<Vec<_>>()
            .join(", ");
        let mut statement = connection
            .prepare(&format!(
                r#"SELECT {columns} FROM "Answer" a JOIN "Session" s ON s."Id" = a."SessionId"
                   WHERE s."ProfileId" = ?1 AND s."EndedAt" IS NOT NULL
                   AND NOT (s."Type" = 'Placement' AND s."Abandoned" = 1)
                   ORDER BY a."AnsweredAt", a."Index""#
            ))
            .map_err(storage)?;
        let rows = statement
            .query_map([id_text(profile_id)], read_answer)
            .map_err(storage)?;
        rows.collect::<rusqlite::Result<Vec<_>>>().map_err(storage)
    }

    fn best_timed_score(
        &self,
        profile_id: Uuid,
        subject: Subject,
        time_limit_seconds: u32,
    ) -> Result<Option<u32>> {
        self.database
            .connection()
            .query_row(
                r#"SELECT MAX("CorrectCount") FROM "Session" WHERE "ProfileId" = ?1 AND "Type" = ?2 AND "Subject" = ?3
                   AND "TimeLimitSeconds" = ?4 AND "EndedAt" IS NOT NULL AND "Abandoned" = 0"#,
                rusqlite::params![
                    id_text(profile_id),
                    SessionType::Timed.as_str(),
                    subject.as_str(),
                    time_limit_seconds,
                ],
                |row| row.get(0),
            )
            .map_err(storage)
    }

    fn lesson_slide(&self, profile_id: Uuid, lesson_id: &str) -> Result<Option<u32>> {
        self.database
            .connection()
            .query_row(
                r#"SELECT "SlideIndex" FROM "LessonProgress" WHERE "ProfileId" = ?1 AND "LessonId" = ?2"#,
                rusqlite::params![id_text(profile_id), lesson_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(storage)
    }

    fn save_lesson_slide(
        &self,
        profile_id: Uuid,
        lesson_id: &str,
        slide: Option<u32>,
    ) -> Result<()> {
        let connection = self.database.connection();
        match slide {
            None => connection.execute(
                r#"DELETE FROM "LessonProgress" WHERE "ProfileId" = ?1 AND "LessonId" = ?2"#,
                rusqlite::params![id_text(profile_id), lesson_id],
            ),
            Some(slide) => connection.execute(
                r#"INSERT INTO "LessonProgress" ("ProfileId", "LessonId", "SlideIndex", "CreatedAt", "UpdatedAt")
                   VALUES (?1, ?2, ?3, ?4, ?4)
                   ON CONFLICT ("ProfileId", "LessonId") DO UPDATE SET "SlideIndex" = excluded."SlideIndex",
                       "UpdatedAt" = excluded."UpdatedAt" WHERE "SlideIndex" <> excluded."SlideIndex""#,
                rusqlite::params![id_text(profile_id), lesson_id, slide, self.database.now_ms()],
            ),
        }
        .map(|_| ())
        .map_err(storage)
    }
}
