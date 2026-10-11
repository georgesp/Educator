//! Types exchanged with the front (port of `Educator.Host/Api/Contracts.cs`), serialized in
//! camelCase. The TypeScript types (`web/src/api/types.ts`) are generated from them by the test
//! `bindings::types_ts_is_up_to_date` (`npm run api:generate`): never edit that file by hand.
//!
//! Enums keep the names used by the .NET API (`"Keyboard"`, `"CE1"`), so the screens do not change.

use educator_core::characters::{self, CharacterInfo};
use educator_core::exercises::{self, Exercise, SeriesSummary};
use educator_core::profiles::Profile;
use educator_core::skills;
use serde::{Deserialize, Serialize};

// ── Enums ────────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum Grade {
    CP,
    CE1,
    CE2,
    CM1,
    CM2,
}

impl From<educator_core::Grade> for Grade {
    fn from(grade: educator_core::Grade) -> Self {
        match grade {
            educator_core::Grade::CP => Grade::CP,
            educator_core::Grade::CE1 => Grade::CE1,
            educator_core::Grade::CE2 => Grade::CE2,
            educator_core::Grade::CM1 => Grade::CM1,
            educator_core::Grade::CM2 => Grade::CM2,
        }
    }
}

impl From<Grade> for educator_core::Grade {
    fn from(grade: Grade) -> Self {
        match grade {
            Grade::CP => educator_core::Grade::CP,
            Grade::CE1 => educator_core::Grade::CE1,
            Grade::CE2 => educator_core::Grade::CE2,
            Grade::CM1 => educator_core::Grade::CM1,
            Grade::CM2 => educator_core::Grade::CM2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum AnswerMode {
    Keyboard,
    Choice,
}

impl From<skills::AnswerMode> for AnswerMode {
    fn from(mode: skills::AnswerMode) -> Self {
        match mode {
            skills::AnswerMode::Keyboard => AnswerMode::Keyboard,
            skills::AnswerMode::Choice => AnswerMode::Choice,
        }
    }
}

impl From<AnswerMode> for skills::AnswerMode {
    fn from(mode: AnswerMode) -> Self {
        match mode {
            AnswerMode::Keyboard => skills::AnswerMode::Keyboard,
            AnswerMode::Choice => skills::AnswerMode::Choice,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum HiddenPart {
    Left,
    Right,
    Result,
}

impl From<exercises::HiddenPart> for HiddenPart {
    fn from(hidden: exercises::HiddenPart) -> Self {
        match hidden {
            exercises::HiddenPart::Left => HiddenPart::Left,
            exercises::HiddenPart::Right => HiddenPart::Right,
            exercises::HiddenPart::Result => HiddenPart::Result,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum CharacterKind {
    Animal,
    Child,
}

impl From<characters::CharacterKind> for CharacterKind {
    fn from(kind: characters::CharacterKind) -> Self {
        match kind {
            characters::CharacterKind::Animal => CharacterKind::Animal,
            characters::CharacterKind::Child => CharacterKind::Child,
        }
    }
}

// ── Start-up and status ──────────────────────────────────────────────────────────────────────

/// State of the application core (content and database), polled by the front at start-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(
    tag = "state",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum StartupStatus {
    Starting,
    Ready,
    Failed { message: String },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct StatusResponse {
    pub version: String,
    pub database: DatabaseStatusResponse,
    pub speech: SpeechStatusResponse,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct DatabaseStatusResponse {
    /// True once the database is created and migrated.
    pub is_ready: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SpeechStatusResponse {
    pub is_available: bool,
    /// Voice name, when available (e.g. `fr_FR-siwis-medium`).
    pub voice: Option<String>,
    /// Why the voice is unavailable (English, diagnostics only).
    pub problem: Option<String>,
}

// ── Skills (SUB000106) ───────────────────────────────────────────────────────────────────────

/// The skills offered to a grade, sorted by period (its own skills before the revisions).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SkillsResponse {
    /// Grade asked for.
    pub grade: Grade,
    /// Grade of the returned skills: the asked one, or CE1 when the asked grade has no skill yet.
    pub skills_grade: Grade,
    /// School period of today (1 to 5), from `content/calendar/periods.json`.
    pub current_period: u8,
    pub skills: Vec<SkillResponse>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SkillResponse {
    pub id: String,
    pub title: String,
    pub instruction: String,
    /// Period where the skill is offered to this grade.
    pub period: u8,
    /// True for the skills of the current period (revisions excepted).
    pub is_current_period: bool,
    /// Grade the skill comes from when it is offered as a revision (SUB000113), otherwise `null`.
    pub revision_from: Option<Grade>,
    pub answer_modes: Vec<AnswerMode>,
}

impl SkillResponse {
    pub fn offered(offered: &skills::OfferedSkill<'_>, current_period: u8) -> Self {
        let skill = offered.skill;
        Self {
            id: skill.id.clone(),
            title: skill.title.clone(),
            instruction: skill.instruction.clone(),
            period: offered.period,
            is_current_period: offered.revision_from.is_none() && offered.period == current_period,
            revision_from: offered.revision_from.map(Into::into),
            answer_modes: skill.answer_modes.iter().map(|m| (*m).into()).collect(),
        }
    }
}

// ── Characters (SUB000101) ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct CharacterResponse {
    pub id: String,
    pub name: String,
    pub kind: CharacterKind,
}

impl From<&CharacterInfo> for CharacterResponse {
    fn from(character: &CharacterInfo) -> Self {
        Self {
            id: character.id.clone(),
            name: character.name.clone(),
            kind: character.kind.into(),
        }
    }
}

// ── Profiles (SUB000102) ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct ProfileResponse {
    pub id: String,
    pub name: String,
    pub grade: Grade,
    pub character_id: String,
    pub points: i32,
    /// ISO 8601 instant in UTC, e.g. `2026-10-11T08:00:00.000Z`.
    pub last_used_at: String,
}

impl From<&Profile> for ProfileResponse {
    fn from(profile: &Profile) -> Self {
        Self {
            id: profile.id.to_string(),
            name: profile.name.clone(),
            grade: profile.grade.into(),
            character_id: profile.character_id.clone(),
            points: profile.points,
            last_used_at: iso_8601(profile.last_used_at),
        }
    }
}

/// UTC Unix milliseconds as ISO 8601.
pub fn iso_8601(unix_ms: i64) -> String {
    chrono::DateTime::from_timestamp_millis(unix_ms)
        .unwrap_or_default()
        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
        .to_string()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct CreateProfileRequest {
    pub name: String,
    pub grade: Grade,
    /// Character shown on the creation screen (`draw_character`); drawn at random when absent.
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub character_id: Option<String>,
}

/// Only the given fields change.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct UpdateProfileRequest {
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub name: Option<String>,
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub grade: Option<Grade>,
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub character_id: Option<String>,
}

// ── Series (SUB000107, SUB000110) ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct StartSeriesRequest {
    pub profile_id: String,
    pub skill_id: String,
    pub answer_mode: AnswerMode,
    /// Fixed seed to replay a series (tests, debugging); random when absent.
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub seed: Option<u32>,
}

/// A new series. The expected answers are not sent.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SeriesResponse {
    pub series_id: String,
    pub seed: u32,
    pub skill_id: String,
    pub instruction: String,
    pub answer_mode: AnswerMode,
    pub exercises: Vec<ExerciseResponse>,
}

/// One exercise: `left operator right = result` with the asked number (`hidden`) set to null.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct ExerciseResponse {
    pub index: u32,
    pub left: Option<i32>,
    /// `+` or `−` (U+2212).
    pub operator: String,
    pub right: Option<i32>,
    pub result: Option<i32>,
    pub hidden: HiddenPart,
    /// Written form with `?` for the asked number, e.g. `7 + ? = 12`.
    pub prompt: String,
    /// French question to read aloud (send it to `speak`).
    pub spoken_prompt: String,
    /// Four proposals in QCM mode, otherwise null.
    pub choices: Option<Vec<i32>>,
}

impl From<&Exercise> for ExerciseResponse {
    fn from(exercise: &Exercise) -> Self {
        let c = &exercise.calculation;
        let shown =
            |part: exercises::HiddenPart, value: i32| (exercise.hidden != part).then_some(value);
        Self {
            index: u32::try_from(exercise.index).unwrap_or(u32::MAX),
            left: shown(exercises::HiddenPart::Left, c.left),
            operator: c.symbol().to_owned(),
            right: shown(exercises::HiddenPart::Right, c.right),
            result: shown(exercises::HiddenPart::Result, c.result()),
            hidden: exercise.hidden.into(),
            prompt: exercise.prompt(),
            spoken_prompt: exercise.spoken_prompt(),
            choices: exercise.choices.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SeriesAnswerResponse {
    pub correct: bool,
    pub expected: i32,
    /// Complete equality shown and read after a mistake, e.g. `7 + 5 = 12`.
    pub explanation: String,
    /// Set when this answer ended the series: points awarded.
    pub summary: Option<SeriesSummaryResponse>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SeriesSummaryResponse {
    pub correct_count: u32,
    pub exercise_count: u32,
    pub points_earned: i32,
    /// New total of the profile.
    pub profile_points: i32,
}

impl From<SeriesSummary> for SeriesSummaryResponse {
    fn from(summary: SeriesSummary) -> Self {
        let count = |n: usize| u32::try_from(n).unwrap_or(u32::MAX);
        Self {
            correct_count: count(summary.correct_count),
            exercise_count: count(summary.exercise_count),
            points_earned: summary.points_earned,
            profile_points: summary.profile_points,
        }
    }
}

// ── Errors ───────────────────────────────────────────────────────────────────────────────────

/// Error of a command, shaped like the RFC 7807 problems of the former HTTP API: `status` keeps
/// the HTTP meaning (400 invalid input, 404 unknown, 409 conflict, 503 unavailable), `detail` is a
/// simple French sentence for the child on profile errors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct CommandError {
    pub status: u16,
    pub title: String,
    pub detail: Option<String>,
    /// Request field in error (`name`, `grade`, `characterId`), for the profile errors.
    pub field: Option<String>,
}

impl CommandError {
    pub fn new(status: u16, title: &str, detail: impl Into<String>) -> Self {
        Self {
            status,
            title: title.to_owned(),
            detail: Some(detail.into()),
            field: None,
        }
    }
}
