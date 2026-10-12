//! Types exchanged with the front (port of `Educator.Host/Api/Contracts.cs`), serialized in
//! camelCase. The TypeScript types (`web/src/api/types.ts`) are generated from them by the test
//! `bindings::types_ts_is_up_to_date` (`npm run api:generate`): never edit that file by hand.
//!
//! Enums keep the names used by the .NET API (`"Keyboard"`, `"CE1"`), so the screens do not change.

use educator_core::characters::{self, CharacterInfo};
use educator_core::exercises::{self, Answer, Exercise, SeriesSummary, TableLine};
use educator_core::lessons::{self, Lesson};
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
    pub family: SkillFamily,
}

/// Family of a skill: the front shows « Mes tables » next to the multiplications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum SkillFamily {
    Calculation,
    Multiplication,
    Numeration,
    DoubleHalf,
    Fraction,
    Problem,
}

impl From<skills::SkillOperation> for SkillFamily {
    fn from(operation: skills::SkillOperation) -> Self {
        use skills::SkillOperation as O;
        match operation {
            O::Addition | O::Subtraction | O::Mixed => SkillFamily::Calculation,
            O::Multiplication => SkillFamily::Multiplication,
            O::Numeration => SkillFamily::Numeration,
            O::DoubleHalf => SkillFamily::DoubleHalf,
            O::Fraction => SkillFamily::Fraction,
            O::Problem => SkillFamily::Problem,
        }
    }
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
            family: skill.operation.into(),
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

/// One exercise: an optional question, the statement (with the answer slots), an optional picture
/// and the kind of input. The expected answer is not sent.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct ExerciseResponse {
    pub index: u32,
    /// Sentence shown above the statement (« Quel est le double de 35 ? », a problem).
    pub question: Option<String>,
    /// The written statement, e.g. `7`, `+`, slot, `=`, `12`.
    pub statement: Vec<TokenResponse>,
    pub picture: Option<PictureResponse>,
    pub input: InputResponse,
    /// Proposals of a pick (QCM), otherwise null.
    pub choices: Option<Vec<ChoiceResponse>>,
    /// Written form with `?` for the answer, e.g. `7 + ? = 12` (accessibility).
    pub prompt: String,
    /// French text to read aloud (send it to `speak`).
    pub spoken_prompt: String,
}

impl From<&Exercise> for ExerciseResponse {
    fn from(exercise: &Exercise) -> Self {
        Self {
            index: u32::try_from(exercise.index).unwrap_or(u32::MAX),
            question: exercise.question.clone(),
            statement: exercise.statement.iter().map(Into::into).collect(),
            picture: exercise.picture.as_ref().map(Into::into),
            input: (&exercise.input).into(),
            choices: exercise.choices.as_ref().map(|choices| {
                choices
                    .iter()
                    .map(|c| ChoiceResponse {
                        label: c.label(),
                        answer: AnswerPayload::from(&c.as_given()),
                    })
                    .collect()
            }),
            prompt: exercise.prompt(),
            spoken_prompt: exercise.spoken.clone(),
        }
    }
}

/// A piece of the statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum TokenResponse {
    Number {
        value: i32,
    },
    /// `+`, `−`, `×`, `=`, `<`, `>`, `→`.
    Sign {
        value: String,
    },
    Text {
        value: String,
    },
    Fraction {
        numerator: u32,
        denominator: u32,
    },
    /// Where the child answers (two slots for a framing, numbered from 0).
    Slot {
        slot: u32,
    },
}

impl From<&exercises::Token> for TokenResponse {
    fn from(token: &exercises::Token) -> Self {
        use exercises::Token as T;
        match token {
            T::Number(value) => TokenResponse::Number { value: *value },
            T::Sign(value) => TokenResponse::Sign {
                value: value.clone(),
            },
            T::Text(value) => TokenResponse::Text {
                value: value.clone(),
            },
            T::Fraction(f) => TokenResponse::Fraction {
                numerator: f.numerator,
                denominator: f.denominator,
            },
            T::Slot(slot) => TokenResponse::Slot {
                slot: u32::try_from(*slot).unwrap_or(0),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum FractionShape {
    Bar,
    Disc,
}

/// Pictures drawn by the front.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum PictureResponse {
    /// Hundred squares, ten bars, unit cubes.
    BaseTen {
        hundreds: u32,
        tens: u32,
        units: u32,
    },
    /// A bar or a disc in `parts` equal parts, `colored` of them colored.
    FractionShape {
        shape: FractionShape,
        parts: u32,
        colored: u32,
    },
    /// `ticks` graduations from `start`, every `step`; `labeled` graduations show their number;
    /// `arrow` points at a graduation.
    NumberLine {
        start: i32,
        step: i32,
        ticks: u32,
        labeled: Vec<u32>,
        arrow: Option<u32>,
    },
    /// `rows` rows of `columns` dots, a line after column `split`.
    ArrayGrid {
        rows: u32,
        columns: u32,
        split: Option<u32>,
    },
    /// Bars of segments (null: unknown), the total of the first bar.
    BarModel {
        bars: Vec<Vec<Option<i32>>>,
        total: Option<i32>,
    },
    /// `groups` groups of `perGroup` objects.
    Groups { groups: u32, per_group: u32 },
}

impl From<&exercises::Picture> for PictureResponse {
    fn from(picture: &exercises::Picture) -> Self {
        use exercises::Picture as P;
        match picture {
            P::BaseTen {
                hundreds,
                tens,
                units,
            } => PictureResponse::BaseTen {
                hundreds: *hundreds,
                tens: *tens,
                units: *units,
            },
            P::FractionShape {
                shape,
                parts,
                colored,
            } => PictureResponse::FractionShape {
                shape: match shape {
                    exercises::FractionShape::Bar => FractionShape::Bar,
                    exercises::FractionShape::Disc => FractionShape::Disc,
                },
                parts: *parts,
                colored: *colored,
            },
            P::NumberLine {
                start,
                step,
                ticks,
                labeled,
                arrow,
            } => PictureResponse::NumberLine {
                start: *start,
                step: *step,
                ticks: *ticks,
                labeled: labeled.clone(),
                arrow: *arrow,
            },
            P::ArrayGrid {
                rows,
                columns,
                split,
            } => PictureResponse::ArrayGrid {
                rows: *rows,
                columns: *columns,
                split: *split,
            },
            P::BarModel { bars, total } => PictureResponse::BarModel {
                bars: bars
                    .iter()
                    .map(|bar| bar.iter().map(|s| s.value).collect())
                    .collect(),
                total: *total,
            },
            P::Groups { groups, per_group } => PictureResponse::Groups {
                groups: *groups,
                per_group: *per_group,
            },
        }
    }
}

/// How the child answers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum InputResponse {
    /// Numbers typed in the slots (keypad or keyboard).
    Number { slots: u32 },
    /// A numerator and a denominator.
    Fraction,
    /// Buttons `<`, `>`, `=`.
    Comparison,
    /// Buttons Oui / Non.
    YesNo,
    /// Put the numbers in order (shown in this order).
    Order { items: Vec<i32>, ascending: bool },
    /// Pick every right number among these.
    MultiChoice { items: Vec<i32> },
    /// Pick one of the `choices`.
    Pick,
    /// Click a graduation of the number line of the picture.
    NumberLine,
}

impl From<&exercises::Input> for InputResponse {
    fn from(input: &exercises::Input) -> Self {
        use exercises::Input as I;
        match input {
            I::Number { slots } => InputResponse::Number {
                slots: u32::try_from(*slots).unwrap_or(1),
            },
            I::Fraction => InputResponse::Fraction,
            I::Comparison => InputResponse::Comparison,
            I::YesNo => InputResponse::YesNo,
            I::Order { items, ascending } => InputResponse::Order {
                items: items.clone(),
                ascending: *ascending,
            },
            I::MultiChoice { items } => InputResponse::MultiChoice {
                items: items.clone(),
            },
            I::Pick => InputResponse::Pick,
            I::NumberLine => InputResponse::NumberLine,
        }
    }
}

/// A proposal of a pick: what the tile shows, and the answer sent when it is picked.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct ChoiceResponse {
    pub label: String,
    pub answer: AnswerPayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum ComparisonSign {
    #[serde(rename = "<")]
    Less,
    #[serde(rename = ">")]
    Greater,
    #[serde(rename = "=")]
    Equal,
}

/// An answer, given by the child (`answer_series`) or expected (correction).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum AnswerPayload {
    Number {
        value: i32,
    },
    Fraction {
        numerator: u32,
        denominator: u32,
    },
    Comparison {
        sign: ComparisonSign,
    },
    YesNo {
        value: bool,
    },
    /// Numbers in order (ranking, framing).
    Sequence {
        values: Vec<i32>,
    },
    /// Numbers in any order (every even number).
    Set {
        values: Vec<i32>,
    },
    /// A written answer (a number or a fraction in words).
    Text {
        value: String,
    },
}

impl From<&Answer> for AnswerPayload {
    fn from(answer: &Answer) -> Self {
        match answer.as_given() {
            Answer::Number(value) => AnswerPayload::Number { value },
            Answer::Fraction(f) => AnswerPayload::Fraction {
                numerator: f.numerator,
                denominator: f.denominator,
            },
            Answer::Comparison(c) => AnswerPayload::Comparison {
                sign: match c {
                    exercises::Comparison::Less => ComparisonSign::Less,
                    exercises::Comparison::Greater => ComparisonSign::Greater,
                    exercises::Comparison::Equal => ComparisonSign::Equal,
                },
            },
            Answer::YesNo(value) => AnswerPayload::YesNo { value },
            Answer::Sequence(values) => AnswerPayload::Sequence { values },
            Answer::Set(values) => AnswerPayload::Set { values },
            Answer::Words(_) => AnswerPayload::Text {
                value: answer.label(),
            },
            Answer::Text(value) => AnswerPayload::Text { value },
        }
    }
}

impl From<AnswerPayload> for Answer {
    fn from(payload: AnswerPayload) -> Self {
        match payload {
            AnswerPayload::Number { value } => Answer::Number(value),
            AnswerPayload::Fraction {
                numerator,
                denominator,
            } => Answer::Fraction(exercises::Fraction::new(numerator, denominator)),
            AnswerPayload::Comparison { sign } => Answer::Comparison(match sign {
                ComparisonSign::Less => exercises::Comparison::Less,
                ComparisonSign::Greater => exercises::Comparison::Greater,
                ComparisonSign::Equal => exercises::Comparison::Equal,
            }),
            AnswerPayload::YesNo { value } => Answer::YesNo(value),
            AnswerPayload::Sequence { values } => Answer::Sequence(values),
            AnswerPayload::Set { values } => Answer::Set(values),
            AnswerPayload::Text { value } => Answer::Text(value),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SeriesAnswerResponse {
    pub correct: bool,
    pub expected: AnswerPayload,
    /// The expected answer as written in the correction (« 3/4 », « < », « 300 et 400 »).
    pub expected_label: String,
    /// Correction shown and read after a mistake, e.g. `7 + 5 = 12`.
    pub explanation: String,
    /// Picture shown with the correction (decomposition, bar model).
    pub help: Option<PictureResponse>,
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

// ── Multiplication tables (SUB000119) ───────────────────────────────────────────────────────

/// « Mes tables »: a table shown in large and read aloud.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct MultiplicationTableResponse {
    pub number: i32,
    pub lines: Vec<TableLineResponse>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct TableLineResponse {
    pub left: i32,
    pub right: i32,
    pub result: i32,
    /// « 3 fois 7, 21. »
    pub spoken: String,
}

impl From<&TableLine> for TableLineResponse {
    fn from(line: &TableLine) -> Self {
        Self {
            left: line.left,
            right: line.right,
            result: line.result,
            spoken: line.spoken.clone(),
        }
    }
}

// ── Lessons (SUB001409, SUB001410, SUB001411, SUB000133) ────────────────────────────────────

/// The lessons offered to a grade, sorted by period.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct LessonsResponse {
    /// Grade asked for.
    pub grade: Grade,
    /// Grade of the returned lessons: the asked one, or CE1 when the asked grade has no lesson yet.
    pub lessons_grade: Grade,
    /// School period of today (1 to 5).
    pub current_period: u8,
    pub lessons: Vec<LessonSummaryResponse>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct LessonSummaryResponse {
    pub id: String,
    pub title: String,
    pub period: u8,
    /// True for the lessons of the current period.
    pub is_current_period: bool,
    pub duration_minutes: u8,
}

impl LessonSummaryResponse {
    pub fn new(lesson: &Lesson, current_period: u8) -> Self {
        Self {
            id: lesson.id.clone(),
            title: lesson.title.clone(),
            period: lesson.period,
            is_current_period: lesson.period == current_period,
            duration_minutes: lesson.duration_minutes,
        }
    }
}

/// A whole lesson. The expected answers of the quiz are not sent (`answer_lesson_quiz` checks them).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct LessonResponse {
    pub id: String,
    pub title: String,
    pub grade: Grade,
    /// Skills practised after the lesson (« Je m'entraîne »), the first one being the main one.
    pub skill_ids: Vec<String>,
    pub slides: Vec<SlideResponse>,
    pub quiz: Vec<QuizQuestionResponse>,
}

impl From<&Lesson> for LessonResponse {
    fn from(lesson: &Lesson) -> Self {
        Self {
            id: lesson.id.clone(),
            title: lesson.title.clone(),
            grade: lesson.grade.into(),
            skill_ids: lesson.skill_ids.clone(),
            slides: lesson.slides.iter().map(Into::into).collect(),
            quiz: lesson
                .quiz
                .iter()
                .enumerate()
                .map(|(index, q)| QuizQuestionResponse {
                    index: u32::try_from(index).unwrap_or(u32::MAX),
                    question: q.question.clone(),
                    spoken: q.spoken.clone(),
                    choices: (!q.choices.is_empty()).then(|| {
                        q.choices
                            .iter()
                            .map(|c| ChoiceResponse {
                                label: c.label(),
                                answer: AnswerPayload::from(c),
                            })
                            .collect()
                    }),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SlideResponse {
    pub title: String,
    /// Shown.
    pub text: String,
    /// Read aloud (send it to `speak`).
    pub speech: String,
    /// The child has to do something before going on.
    pub interactive: bool,
    pub animation: AnimationResponse,
}

impl From<&lessons::Slide> for SlideResponse {
    fn from(slide: &lessons::Slide) -> Self {
        Self {
            title: slide.title.clone(),
            text: slide.text.clone(),
            speech: slide.speech.clone(),
            interactive: slide.is_interactive(),
            animation: (&slide.animation).into(),
        }
    }
}

/// An animation of the library (SUB000117), with the values it shows already computed by the core.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum AnimationResponse {
    TitleCard {
        title: String,
        subtitle: Option<String>,
    },
    /// « Je retiens ».
    RecapCard {
        text: String,
        example: Option<String>,
    },
    CalcChain {
        chains: Vec<ChainResponse>,
        mode: ChainMode,
    },
    /// Graduations from `from` to `to` every `step`; the car starts at `start` and makes the jumps.
    NumberLine {
        from: i32,
        to: i32,
        step: i32,
        start: i32,
        jumps: Vec<JumpResponse>,
        /// The child clicks each landing graduation, in order.
        interactive: bool,
        /// The value of a graduation shows under the pointer (interactive line).
        hover_values: bool,
    },
    /// `value` as hundred squares, ten bars and unit cubes; `missing` dotted cubes up to the next ten.
    TensUnits {
        value: u32,
        hundreds: u32,
        tens: u32,
        units: u32,
        missing: u32,
    },
}

impl From<&lessons::Animation> for AnimationResponse {
    fn from(animation: &lessons::Animation) -> Self {
        use lessons::Animation as A;
        match animation {
            A::TitleCard { title, subtitle } => AnimationResponse::TitleCard {
                title: title.clone(),
                subtitle: subtitle.clone(),
            },
            A::RecapCard { text, example } => AnimationResponse::RecapCard {
                text: text.clone(),
                example: example.clone(),
            },
            A::CalcChain { chains, mode } => AnimationResponse::CalcChain {
                chains: chains.iter().map(Into::into).collect(),
                mode: match mode {
                    lessons::ChainMode::Show => ChainMode::Show,
                    lessons::ChainMode::Match => ChainMode::Match,
                },
            },
            A::NumberLine {
                from,
                to,
                step,
                start,
                jumps,
                interactive,
                hover_values,
            } => AnimationResponse::NumberLine {
                from: *from,
                to: *to,
                step: *step,
                start: *start,
                jumps: jumps_of(*start, jumps),
                interactive: *interactive,
                hover_values: *hover_values,
            },
            A::TensUnits {
                value,
                complete_ten,
            } => AnimationResponse::TensUnits {
                value: *value,
                hundreds: value / 100,
                tens: value / 10 % 10,
                units: value % 10,
                missing: if *complete_ten && value % 10 != 0 {
                    10 - value % 10
                } else {
                    0
                },
            },
        }
    }
}

fn jumps_of(start: i32, deltas: &[i32]) -> Vec<JumpResponse> {
    deltas
        .iter()
        .zip(lessons::landings(start, deltas))
        .map(|(delta, value)| JumpResponse {
            delta: *delta,
            value,
        })
        .collect()
}

/// `Show`: the chains appear step by step. `Match`: the child links each calculation to its chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum ChainMode {
    Show,
    Match,
}

/// A chain of arrows: `start`, then each step with the value reached.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct ChainResponse {
    pub start: i32,
    pub steps: Vec<JumpResponse>,
    /// What the chain adds in all (+ 20 − 1 → 19): the calculation is `start + total`.
    pub total: i32,
    pub result: i32,
    /// A classic mistake, shown crossed out.
    pub crossed: bool,
}

impl From<&lessons::Chain> for ChainResponse {
    fn from(chain: &lessons::Chain) -> Self {
        Self {
            start: chain.start,
            steps: jumps_of(chain.start, &chain.steps),
            total: chain.total(),
            result: chain.result(),
            crossed: chain.crossed,
        }
    }
}

/// A step: what is added (negative: removed) and the value reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct JumpResponse {
    pub delta: i32,
    pub value: i32,
}

/// A question of the end-of-lesson quiz: typed (`choices` null) or picked.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct QuizQuestionResponse {
    pub index: u32,
    pub question: String,
    /// Read aloud (send it to `speak`).
    pub spoken: String,
    pub choices: Option<Vec<ChoiceResponse>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct QuizAnswerResponse {
    pub correct: bool,
    pub expected: AnswerPayload,
    pub expected_label: String,
    /// Shown and read after the answer, right or wrong.
    pub explanation: String,
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
