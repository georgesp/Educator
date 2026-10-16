//! Types exchanged with the front (port of `Educator.Host/Api/Contracts.cs`), serialized in
//! camelCase. The TypeScript types (`web/src/api/types.ts`) are generated from them by the test
//! `bindings::types_ts_is_up_to_date` (`npm run api:generate`): never edit that file by hand.
//!
//! Enums keep the names used by the .NET API (`"Keyboard"`, `"CE1"`), so the screens do not change.

use educator_core::characters::{self, CharacterInfo};
use educator_core::exercises::{self, Answer, Exercise, SeriesSummary, TableLine};
use educator_core::lessons::{self, Lesson};
use educator_core::mascot;
use educator_core::parent_profile::ParentProfile;
use educator_core::profiles::Profile;
use educator_core::sessions;
use educator_core::skills;
use educator_core::spelling::{LetterKind, SpellingCorrection, SpokenWord};
use educator_core::statistics;
use serde::{Deserialize, Serialize};

mod figures;
mod motivation;
mod sync;

pub use figures::*;
pub use motivation::*;
pub use sync::*;

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

/// School subject (lot 2: mathematics and spelling).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum Subject {
    Math,
    Spelling,
}

impl From<skills::Subject> for Subject {
    fn from(subject: skills::Subject) -> Self {
        match subject {
            skills::Subject::Math => Subject::Math,
            skills::Subject::Spelling => Subject::Spelling,
        }
    }
}

impl From<Subject> for skills::Subject {
    fn from(subject: Subject) -> Self {
        match subject {
            Subject::Math => skills::Subject::Math,
            Subject::Spelling => skills::Subject::Spelling,
        }
    }
}

/// Kind of spelling exercise chosen by the child (SUB001406 to SUB001408).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum SpellingFormat {
    /// « Dictée » : the word is read, the child writes it.
    Dictation,
    /// « Mots à trous » : the letters hidden in the word, typed or picked.
    MissingLetters,
    /// « Choisis la bonne écriture » : one spelling out of three.
    ChooseSpelling,
}

impl SpellingFormat {
    pub fn from_format(format: skills::ExerciseFormat) -> Option<Self> {
        match format {
            skills::ExerciseFormat::Dictation => Some(SpellingFormat::Dictation),
            skills::ExerciseFormat::MissingLetters => Some(SpellingFormat::MissingLetters),
            skills::ExerciseFormat::ChooseSpelling => Some(SpellingFormat::ChooseSpelling),
            _ => None,
        }
    }
}

impl From<SpellingFormat> for skills::ExerciseFormat {
    fn from(format: SpellingFormat) -> Self {
        match format {
            SpellingFormat::Dictation => skills::ExerciseFormat::Dictation,
            SpellingFormat::MissingLetters => skills::ExerciseFormat::MissingLetters,
            SpellingFormat::ChooseSpelling => skills::ExerciseFormat::ChooseSpelling,
        }
    }
}

/// What a session was (SUB001401).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum SessionType {
    Exercise,
    Exam,
    Timed,
    LessonQuiz,
    Placement,
    Review,
}

impl From<sessions::SessionType> for SessionType {
    fn from(session_type: sessions::SessionType) -> Self {
        match session_type {
            sessions::SessionType::Exercise => SessionType::Exercise,
            sessions::SessionType::Exam => SessionType::Exam,
            sessions::SessionType::Timed => SessionType::Timed,
            sessions::SessionType::LessonQuiz => SessionType::LessonQuiz,
            sessions::SessionType::Placement => SessionType::Placement,
            sessions::SessionType::Review => SessionType::Review,
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
    /// No French voice is installed on the device: the parent can install one (iPad).
    pub voice_to_install: bool,
}

// ── Skills (SUB000106) ───────────────────────────────────────────────────────────────────────

/// The skills offered to a grade, sorted by period (its own skills before the revisions).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SkillsResponse {
    pub subject: Subject,
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
    /// Family shown to the child: a color and a symbol on the card.
    pub category: SkillCategory,
    /// Spelling: the kinds of exercise offered (dictation, missing letters, the right spelling).
    pub spelling_formats: Vec<SpellingFormat>,
    /// Spelling: the kinds of exercise where the word is only heard, which cannot be played
    /// without the voice (SUB026704).
    pub listening_formats: Vec<SpellingFormat>,
    /// Mastery of the skill by the profile (0 to 100, SUB001401), when it was practised.
    pub mastery: Option<u8>,
    /// Best stars of the profile on the skill (0 to 3, SUB001502).
    pub stars: u8,
    /// Percent of the points of a series started today: 100, more for a period still to come
    /// (a bonus shown on the card), less for a past period already mastered.
    pub points_percent: u32,
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
    Spelling,
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
            O::Spelling => SkillFamily::Spelling,
        }
    }
}

/// Family of a skill shown to the child (color and symbol of the cards of the exercises and the
/// lessons): from the operation in math, from the theme in spelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum SkillCategory {
    Numbers,
    AddSubtract,
    Multiply,
    FractionsHalves,
    Problems,
    LettersAndSounds,
    SpellingRules,
    SilentLetters,
    Agreement,
    WordsToKnow,
}

impl From<skills::SkillCategory> for SkillCategory {
    fn from(category: skills::SkillCategory) -> Self {
        use skills::SkillCategory as C;
        match category {
            C::Numbers => SkillCategory::Numbers,
            C::AddSubtract => SkillCategory::AddSubtract,
            C::Multiply => SkillCategory::Multiply,
            C::FractionsHalves => SkillCategory::FractionsHalves,
            C::Problems => SkillCategory::Problems,
            C::LettersAndSounds => SkillCategory::LettersAndSounds,
            C::SpellingRules => SkillCategory::SpellingRules,
            C::SilentLetters => SkillCategory::SilentLetters,
            C::Agreement => SkillCategory::Agreement,
            C::WordsToKnow => SkillCategory::WordsToKnow,
        }
    }
}

impl SkillResponse {
    pub fn offered(
        offered: &skills::OfferedSkill<'_>,
        current_period: u8,
        mastery: Option<u8>,
        stars: u8,
        points_percent: u32,
    ) -> Self {
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
            category: skill.category().into(),
            spelling_formats: skill
                .formats
                .iter()
                .filter_map(|f| SpellingFormat::from_format(*f))
                .collect(),
            listening_formats: skill
                .formats
                .iter()
                .filter(|f| f.needs_listening())
                .filter_map(|f| SpellingFormat::from_format(*f))
                .collect(),
            mastery,
            stars,
            points_percent,
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
    /// The accessories worn by the avatar (SUB001504).
    pub accessories: Vec<WornAccessoryResponse>,
    pub points: i32,
    /// Seconds given per question of the level assessment.
    pub placement_seconds: u32,
    /// Seconds given per question of an exam; 0 is no timer.
    pub exam_seconds: u32,
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
            accessories: profile.accessories.iter().map(Into::into).collect(),
            points: profile.points,
            placement_seconds: profile.placement_seconds,
            exam_seconds: profile.exam_seconds,
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
    /// Seconds per question of the level assessment (15, 20, 30, 45 or 60).
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub placement_seconds: Option<u32>,
    /// Seconds per question of an exam (0 for no timer, 20, 30, 45 or 60).
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub exam_seconds: Option<u32>,
}

// ── Series (SUB000107, SUB000110) ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct StartSeriesRequest {
    pub profile_id: String,
    pub skill_id: String,
    pub answer_mode: AnswerMode,
    /// Spelling: the kind of exercise chosen; every kind allowed by the answer mode when absent.
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub format: Option<SpellingFormat>,
    /// Fixed seed to replay a series (tests, debugging); random when absent.
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub seed: Option<u32>,
}

/// A new series (exercises of a skill, an exam, a timed challenge). The expected answers are not sent.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SeriesResponse {
    pub series_id: String,
    pub session_type: SessionType,
    pub subject: Subject,
    pub seed: u32,
    /// The skill of a series of exercises; null for an exam or a timed challenge.
    pub skill_id: Option<String>,
    /// The skill's instruction; empty for an exam, a timed challenge or a quiz (the front has its own).
    pub instruction: String,
    pub answer_mode: AnswerMode,
    /// Duration of a timed challenge, in seconds.
    pub time_limit_seconds: Option<u32>,
    /// Seconds given per question (level assessment, exam with a timer); null without a timer.
    pub question_seconds: Option<u32>,
    pub exercises: Vec<ExerciseResponse>,
}

impl From<&exercises::Series> for SeriesResponse {
    fn from(series: &exercises::Series) -> Self {
        Self {
            series_id: series.id.to_string(),
            session_type: series.session_type.into(),
            subject: series.subject.into(),
            seed: series.seed,
            skill_id: series.skill_id.clone(),
            instruction: series.instruction.clone(),
            answer_mode: series.answer_mode.into(),
            time_limit_seconds: series.time_limit_seconds,
            question_seconds: series.question_seconds,
            exercises: series.exercises.iter().map(Into::into).collect(),
        }
    }
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
    /// French text to read aloud (send it to `speak`). For a spelling exercise, play the word
    /// with `speak_series_word` instead.
    pub spoken_prompt: String,
    /// Spelling: the word is read (`speak_series_word`) and, for the missing letters, the letters
    /// around the blank. The word itself is not sent.
    pub word: Option<WordPromptResponse>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct WordPromptResponse {
    /// Letters before the blank (missing letters), otherwise null.
    pub before: Option<String>,
    /// Letters after the blank (missing letters), otherwise null.
    pub after: Option<String>,
    /// Missing letters: how many letters the blank hides (shown as dashes), otherwise null.
    pub missing: Option<u32>,
    /// The word can also be heard in a short sentence.
    pub has_example: bool,
    /// A noun: its article (« un », « une »), shown before the answer; the child writes the noun
    /// only. Null for the other words.
    pub article: Option<String>,
}

impl From<&SpokenWord> for WordPromptResponse {
    fn from(word: &SpokenWord) -> Self {
        Self {
            before: word.blank.as_ref().map(|(b, _)| b.clone()),
            after: word.blank.as_ref().map(|(_, a)| a.clone()),
            missing: word
                .missing_count()
                .map(|n| u32::try_from(n).unwrap_or(u32::MAX)),
            has_example: word.example.is_some(),
            article: word.article.map(|a| a.as_str().to_owned()),
        }
    }
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
            // The word to write is never sent: it is played by `speak_series_word`.
            spoken_prompt: match &exercise.word {
                Some(word) if word.word == exercise.spoken => String::new(),
                _ => exercise.spoken.clone(),
            },
            word: exercise.word.as_ref().map(Into::into),
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
    /// Letters typed (keyboard or on-screen keyboard with the accented letters), in the blank of the
    /// word for the missing letters.
    Spelling { max_length: u32 },
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
            I::Spelling { max_length } => InputResponse::Spelling {
                max_length: u32::try_from(*max_length).unwrap_or(u32::MAX),
            },
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
    /// The time given for the question ran out: no answer, always wrong.
    TimedOut,
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
            Answer::Text(value) | Answer::Spelling(value) => AnswerPayload::Text { value },
            Answer::TimedOut => AnswerPayload::TimedOut,
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
            AnswerPayload::TimedOut => Answer::TimedOut,
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
    /// Spelling: the child's word letter by letter, to highlight the mistakes (SUB001405).
    pub correction: Option<SpellingCorrectionResponse>,
    /// Set when this answer ended the series: points awarded.
    pub summary: Option<SeriesSummaryResponse>,
}

/// A written word compared with the expected one.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SpellingCorrectionResponse {
    pub correct: bool,
    pub letters: Vec<LetterResponse>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum LetterKindName {
    /// The right letter.
    Right,
    /// `given` written instead of `expected`.
    Wrong,
    /// `expected` forgotten.
    Missing,
    /// `given` too many.
    Extra,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct LetterResponse {
    pub kind: LetterKindName,
    pub given: Option<String>,
    pub expected: Option<String>,
}

impl From<&SpellingCorrection> for SpellingCorrectionResponse {
    fn from(correction: &SpellingCorrection) -> Self {
        Self {
            correct: correction.correct,
            letters: correction
                .letters
                .iter()
                .map(|l| LetterResponse {
                    kind: match l.kind {
                        LetterKind::Right => LetterKindName::Right,
                        LetterKind::Wrong => LetterKindName::Wrong,
                        LetterKind::Missing => LetterKindName::Missing,
                        LetterKind::Extra => LetterKindName::Extra,
                    },
                    given: l.given.map(String::from),
                    expected: l.expected.map(String::from),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SeriesSummaryResponse {
    pub correct_count: u32,
    pub exercise_count: u32,
    pub points_earned: i32,
    /// Percent applied to the points: 100, more ahead of the school year (a bonus shown to the
    /// child), less on a past period already mastered (never shown).
    pub points_percent: u32,
    /// New total of the profile.
    pub profile_points: i32,
    /// Results by skill, in the order of the series (an exam mixes several skills).
    pub skills: Vec<SkillResultResponse>,
    /// What the mascot says at the end (SUB001417): a party, happy or encouraging.
    pub mascot_situation: MascotSituation,
    /// Timed challenge (SUB001415): the best score before this one at the same duration.
    pub previous_best: Option<u32>,
    /// Timed challenge: a new personal record, to celebrate.
    pub new_record: bool,
    /// Lesson quiz (SUB001411): fewer than half right answers, « Revoir la leçon » comes first.
    pub review_lesson: bool,
    /// Stars, level, goal of the day, badges and accessories brought by the session (lot 3).
    pub rewards: Option<SessionRewardsResponse>,
}

/// The results of a series on one skill.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SkillResultResponse {
    pub skill_id: String,
    pub title: String,
    pub correct_count: u32,
    pub question_count: u32,
    /// Lesson of the skill (« Revoir la leçon »), when there is one.
    pub lesson_id: Option<String>,
    /// A strong point of an exam (at least two thirds right); otherwise a notion to work on again.
    pub strong: bool,
}

// ── Mascot (SUB001417) ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum MascotExpression {
    Happy,
    Thinking,
    Encouraging,
    Party,
}

impl From<mascot::Expression> for MascotExpression {
    fn from(expression: mascot::Expression) -> Self {
        match expression {
            mascot::Expression::Happy => MascotExpression::Happy,
            mascot::Expression::Thinking => MascotExpression::Thinking,
            mascot::Expression::Encouraging => MascotExpression::Encouraging,
            mascot::Expression::Party => MascotExpression::Party,
        }
    }
}

/// When the mascot speaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum MascotSituation {
    LessonStart,
    SeriesPerfect,
    SeriesWell,
    SeriesEncourage,
    ExamEnd,
    TimedEnd,
    LevelUp,
}

impl From<mascot::Situation> for MascotSituation {
    fn from(situation: mascot::Situation) -> Self {
        use mascot::Situation as S;
        match situation {
            S::LessonStart => MascotSituation::LessonStart,
            S::SeriesPerfect => MascotSituation::SeriesPerfect,
            S::SeriesWell => MascotSituation::SeriesWell,
            S::SeriesEncourage => MascotSituation::SeriesEncourage,
            S::ExamEnd => MascotSituation::ExamEnd,
            S::TimedEnd => MascotSituation::TimedEnd,
            S::LevelUp => MascotSituation::LevelUp,
        }
    }
}

impl From<MascotSituation> for mascot::Situation {
    fn from(situation: MascotSituation) -> Self {
        use mascot::Situation as S;
        match situation {
            MascotSituation::LessonStart => S::LessonStart,
            MascotSituation::SeriesPerfect => S::SeriesPerfect,
            MascotSituation::SeriesWell => S::SeriesWell,
            MascotSituation::SeriesEncourage => S::SeriesEncourage,
            MascotSituation::ExamEnd => S::ExamEnd,
            MascotSituation::TimedEnd => S::TimedEnd,
            MascotSituation::LevelUp => S::LevelUp,
        }
    }
}

/// The mascot: her name and the SVG of each expression.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct MascotResponse {
    pub name: String,
    pub pictures: Vec<MascotPictureResponse>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct MascotPictureResponse {
    pub expression: MascotExpression,
    pub svg: String,
}

/// A phrase of the mascot, shown in a bubble and read aloud, with her expression.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct MascotPhraseResponse {
    pub text: String,
    pub expression: MascotExpression,
}

/// What the timed challenge offers to a profile (SUB001415).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct TimedChallengeResponse {
    pub subject: Subject,
    /// Skills mastered enough (60 % or more) to enter the challenge; none: no challenge yet.
    pub skill_count: u32,
    /// The durations offered, in seconds, and the best score of each.
    pub durations: Vec<TimedDurationResponse>,
    pub default_seconds: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct TimedDurationResponse {
    pub seconds: u32,
    /// Best number of right answers, when the profile already played this duration.
    pub best: Option<u32>,
}

/// The level assessment of a profile: which subjects can still be taken.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct PlacementStatusResponse {
    /// One entry per subject, in the order of the menu.
    pub subjects: Vec<PlacementSubjectResponse>,
    /// Seconds given per question, from the profile's options.
    pub question_seconds: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct PlacementSubjectResponse {
    pub subject: Subject,
    /// Not done yet and the grade has skills in the subject.
    pub available: bool,
    /// Finished: an abandoned assessment does not count.
    pub done: bool,
}

impl SeriesSummaryResponse {
    /// `skill`: title and lesson of a skill id.
    pub fn new(summary: &SeriesSummary, skill: impl Fn(&str) -> (String, Option<String>)) -> Self {
        let count = |n: usize| u32::try_from(n).unwrap_or(u32::MAX);
        Self {
            correct_count: count(summary.correct_count),
            exercise_count: count(summary.exercise_count),
            points_earned: summary.points_earned,
            points_percent: summary.points_percent,
            profile_points: summary.profile_points,
            previous_best: summary.previous_best,
            new_record: summary.new_record,
            review_lesson: summary.review_lesson,
            rewards: None,
            mascot_situation: mascot::Situation::after_series(
                summary.correct_count,
                summary.exercise_count,
            )
            .into(),
            skills: summary
                .by_skill()
                .into_iter()
                .map(|r| {
                    let (title, lesson_id) = skill(&r.skill_id);
                    SkillResultResponse {
                        title,
                        lesson_id,
                        strong: r.is_strong(),
                        correct_count: count(r.correct_count),
                        question_count: count(r.question_count),
                        skill_id: r.skill_id,
                    }
                })
                .collect(),
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

// ── Lessons (SUB001409, SUB001410, SUB001411, SUB000117) ────────────────────────────────────

/// The lessons offered to a grade, sorted by period.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct LessonsResponse {
    pub subject: Subject,
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
    /// Family of the main skill of the lesson (its first one), `null` when it is not loaded.
    pub category: Option<SkillCategory>,
    /// Best stars of the profile on the quiz of the lesson (0 to 3, SUB001502).
    pub stars: u8,
    /// The main skill of the lesson (its first one): the progress map links it (SUB001506).
    pub skill_id: Option<String>,
    /// Percent of the points of its quiz taken today, as for the skills: 100, more for a period
    /// still to come, less for a past period already mastered.
    pub points_percent: u32,
}

impl LessonSummaryResponse {
    /// `main_skill`: the first skill of the lesson, which gives its family.
    pub fn new(
        lesson: &Lesson,
        current_period: u8,
        main_skill: Option<&skills::Skill>,
        stars: u8,
        points_percent: u32,
    ) -> Self {
        Self {
            id: lesson.id.clone(),
            title: lesson.title.clone(),
            period: lesson.period,
            is_current_period: lesson.period == current_period,
            duration_minutes: lesson.duration_minutes,
            category: main_skill.map(|s| s.category().into()),
            stars,
            skill_id: lesson.skill_ids.first().cloned(),
            points_percent,
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
    pub subject: Subject,
    pub grade: Grade,
    /// Skills practised after the lesson (« Je m'entraîne »), the first one being the main one.
    pub skill_ids: Vec<String>,
    pub slides: Vec<SlideResponse>,
    /// The questions of the quiz (answered through `start_lesson_quiz` and `answer_series`).
    pub quiz: Vec<QuizQuestionResponse>,
    /// The slide where the profile left the lesson, when it did not finish it (SUB001410).
    pub resume_slide: Option<u32>,
}

impl From<&Lesson> for LessonResponse {
    fn from(lesson: &Lesson) -> Self {
        Self {
            id: lesson.id.clone(),
            title: lesson.title.clone(),
            subject: lesson.subject.into(),
            grade: lesson.grade.into(),
            resume_slide: None,
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
        /// Graduations marked with a checkered flag.
        flags: Vec<i32>,
    },
    /// `value` as hundred plates, ten bars and unit cubes; `missing` dotted cubes up to the next
    /// ten. `steps`: the states shown one after the other (a number joins, is taken away, shared).
    TensUnits {
        value: u32,
        hundreds: u32,
        tens: u32,
        units: u32,
        missing: u32,
        steps: Vec<BlockStepResponse>,
        /// The number added or removed, or the number of parts of a share.
        change: Option<u32>,
        /// The sum, the difference, or one part of the share.
        result: u32,
        parts: u32,
        interactive: bool,
        task: Option<BlockTask>,
        /// Interactive: the blocks the child starts with, and the blocks to reach.
        interactive_start: BlocksResponse,
        target: BlocksResponse,
    },
    /// A written operation, one layout per technique drawn (two side by side: cassage, compensation).
    ColumnOperation {
        operator: LessonOperator,
        operands: Vec<u32>,
        result: u32,
        layouts: Vec<WrittenOperationResponse>,
        /// Columns the animation computes, from the units (0: the numbers are only set in columns).
        computed_columns: u32,
        /// The child writes the digit of each column of the result, from the units.
        interactive: bool,
    },
    /// `left operator right = result`, maybe with a hidden term, then the rest of its family.
    Equation {
        left: u32,
        operator: LessonOperator,
        right: u32,
        result: u32,
        missing: Option<EquationTerm>,
        /// The value in the box.
        missing_value: Option<u32>,
        /// The equality first, then the other equalities of its family.
        equalities: Vec<EqualityResponse>,
        /// The child picks the missing term among `choices`.
        interactive: bool,
        choices: Vec<u32>,
    },
    CompareNumbers(CompareNumbersResponse),
    SequencePattern(SequencePatternResponse),
    ArrayGrid(ArrayGridResponse),
    TimesTable(TimesTableResponse),
    PlaceValueTable(PlaceValueTableResponse),
    GroupObjects(GroupObjectsResponse),
    TenFrame(TenFrameResponse),
    FractionBar(FractionBarResponse),
    BarModel(BarModelResponse),
    /// Tiles to put in order to make `word`: `tiles` in the right order, then the `distractors`;
    /// `order`: the tiles as shown, mixed up (indexes, tiles first then distractors).
    LetterTiles {
        word: String,
        tiles: Vec<String>,
        distractors: Vec<String>,
        order: Vec<u32>,
        interactive: bool,
    },
    /// Words to put in the box of their grapheme.
    SoundSort {
        groups: Vec<String>,
        words: Vec<SortedWordResponse>,
        interactive: bool,
    },
}

/// A word of a `soundSort`: its box, and the letters of the grapheme to color.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SortedWordResponse {
    pub word: String,
    pub group: u32,
    pub start: u32,
    pub length: u32,
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
                    lessons::ChainMode::Pick => ChainMode::Pick,
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
                flags,
            } => AnimationResponse::NumberLine {
                from: *from,
                to: *to,
                step: *step,
                start: *start,
                jumps: jumps_of(*start, jumps),
                interactive: *interactive,
                hover_values: *hover_values,
                flags: flags.clone(),
            },
            A::TensUnits(blocks) => tens_units_response(blocks),
            A::ColumnOperation(operation) => {
                let layouts: Vec<WrittenOperationResponse> = operation
                    .methods()
                    .into_iter()
                    .map(|method| WrittenOperationResponse {
                        method: method.map(Into::into),
                        columns: lessons::written_columns(
                            operation.operator,
                            &operation.operands,
                            method,
                        )
                        .iter()
                        .map(Into::into)
                        .collect(),
                    })
                    .collect();
                let width = layouts.first().map_or(0, |l| l.columns.len());
                AnimationResponse::ColumnOperation {
                    operator: operation.operator.into(),
                    operands: operation.operands.clone(),
                    result: operation.result(),
                    computed_columns: operation.computed_columns.unwrap_or(width) as u32,
                    layouts,
                    interactive: operation.interactive,
                }
            }
            A::Equation(equation) => AnimationResponse::Equation {
                left: equation.left,
                operator: equation.operator.into(),
                right: equation.right,
                result: equation.result(),
                missing: equation.missing.map(|term| match term {
                    lessons::EquationTerm::Left => EquationTerm::Left,
                    lessons::EquationTerm::Right => EquationTerm::Right,
                    lessons::EquationTerm::Result => EquationTerm::Result,
                }),
                missing_value: equation.missing_value(),
                equalities: equation
                    .equalities()
                    .into_iter()
                    .map(|e| EqualityResponse {
                        left: e.left,
                        operator: e.operator.into(),
                        right: e.right,
                        result: e.result,
                    })
                    .collect(),
                interactive: equation.interactive,
                choices: equation.choices.clone(),
            },
            A::CompareNumbers(c) => AnimationResponse::CompareNumbers(c.into()),
            A::SequencePattern(s) => AnimationResponse::SequencePattern(s.into()),
            A::ArrayGrid(g) => AnimationResponse::ArrayGrid(g.into()),
            A::TimesTable(t) => AnimationResponse::TimesTable(t.into()),
            A::PlaceValueTable(t) => AnimationResponse::PlaceValueTable(t.into()),
            A::GroupObjects(g) => AnimationResponse::GroupObjects(g.into()),
            A::TenFrame(f) => AnimationResponse::TenFrame(f.into()),
            A::FractionBar(f) => AnimationResponse::FractionBar(f.into()),
            A::BarModel(m) => AnimationResponse::BarModel(m.into()),
            A::LetterTiles(t) => AnimationResponse::LetterTiles {
                word: t.word.clone(),
                tiles: t.tiles.clone(),
                distractors: t.distractors.clone(),
                order: t
                    .shuffled()
                    .into_iter()
                    .map(|i| u32::try_from(i).unwrap_or(0))
                    .collect(),
                interactive: t.interactive,
            },
            A::SoundSort(sort) => AnimationResponse::SoundSort {
                groups: sort.groups.clone(),
                words: sort
                    .words
                    .iter()
                    .map(|w| SortedWordResponse {
                        word: w.word.clone(),
                        group: u32::try_from(w.group).unwrap_or(0),
                        start: u32::try_from(w.start).unwrap_or(0),
                        length: u32::try_from(w.length).unwrap_or(0),
                    })
                    .collect(),
                interactive: sort.interactive,
            },
        }
    }
}

/// The operation of a written operation, of an equality or of a table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum LessonOperator {
    Add,
    Subtract,
    Multiply,
}

impl From<lessons::Operator> for LessonOperator {
    fn from(operator: lessons::Operator) -> Self {
        match operator {
            lessons::Operator::Add => LessonOperator::Add,
            lessons::Operator::Subtract => LessonOperator::Subtract,
            lessons::Operator::Multiply => LessonOperator::Multiply,
        }
    }
}

/// The two techniques of the written subtraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum SubtractionMethod {
    Cassage,
    Compensation,
}

impl From<lessons::SubtractionMethod> for SubtractionMethod {
    fn from(method: lessons::SubtractionMethod) -> Self {
        match method {
            lessons::SubtractionMethod::Cassage => SubtractionMethod::Cassage,
            lessons::SubtractionMethod::Compensation => SubtractionMethod::Compensation,
        }
    }
}

/// A written operation drawn with one technique (`method` null for an addition).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct WrittenOperationResponse {
    pub method: Option<SubtractionMethod>,
    /// From the units to the left.
    pub columns: Vec<WrittenColumnResponse>,
}

/// A column of a written operation, computed by the core.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct WrittenColumnResponse {
    /// The digit of each number, from top to bottom (null: the number has no digit here).
    pub digits: Vec<Option<u8>>,
    /// Addition: carry above the column. Compensation: 1 ten added to the bottom number here.
    pub carry: u8,
    /// Cassage: the top digit crossed out and rewritten, with the column computed at that moment.
    pub rewrites: Vec<RewriteResponse>,
    /// Compensation: 10 units added to the top digit.
    pub plus_ten: bool,
    /// The numbers the column adds (carry first), or the top value and the value removed.
    pub terms: Vec<u32>,
    pub value: u32,
    /// The digit written in the result.
    pub digit: u8,
    /// False for a zero on the left of the result, not written.
    pub written: bool,
}

impl From<&lessons::WrittenColumn> for WrittenColumnResponse {
    fn from(column: &lessons::WrittenColumn) -> Self {
        Self {
            digits: column.digits.clone(),
            carry: column.carry,
            rewrites: column
                .rewrites
                .iter()
                .map(|r| RewriteResponse {
                    at_column: r.at_column as u32,
                    value: r.value,
                })
                .collect(),
            plus_ten: column.plus_ten,
            terms: column.terms.clone(),
            value: column.value,
            digit: column.digit,
            written: column.written,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct RewriteResponse {
    pub at_column: u32,
    pub value: u8,
}

/// The term of an equality shown as a box.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum EquationTerm {
    Left,
    Right,
    Result,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct EqualityResponse {
    pub left: u32,
    pub operator: LessonOperator,
    pub right: u32,
    pub result: u32,
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
/// `Pick`: the child clicks the only right chain (the crossed ones are classic mistakes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum ChainMode {
    Show,
    Match,
    Pick,
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

// ── Parents' space (SUB001510, SUB001511) ───────────────────────────────────────────────────

/// The « adult » gate: `left × right` to compute, drawn again at every opening.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct ParentGateResponse {
    /// To send back with the answer (`check_parent_gate`).
    pub id: String,
    pub left: u32,
    pub right: u32,
}

/// The parents' card of the profile choice: created with default values, editable.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct ParentProfileResponse {
    pub name: String,
    pub character_id: String,
}

impl From<&ParentProfile> for ParentProfileResponse {
    fn from(profile: &ParentProfile) -> Self {
        Self {
            name: profile.name.clone(),
            character_id: profile.character_id.clone(),
        }
    }
}

/// Only the given fields change.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct UpdateParentProfileRequest {
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub name: Option<String>,
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub character_id: Option<String>,
}

/// How well an exercise is mastered: from the share of right answers among the last 20.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum MasteryStateName {
    /// Fewer than 5 answers: too early to judge.
    TooEarly,
    /// 80 % or more.
    Mastered,
    InProgress,
    /// Under 50 %.
    ToReview,
}

impl From<statistics::MasteryState> for MasteryStateName {
    fn from(state: statistics::MasteryState) -> Self {
        match state {
            statistics::MasteryState::TooEarly => MasteryStateName::TooEarly,
            statistics::MasteryState::Mastered => MasteryStateName::Mastered,
            statistics::MasteryState::InProgress => MasteryStateName::InProgress,
            statistics::MasteryState::ToReview => MasteryStateName::ToReview,
        }
    }
}

/// The points of the last series of an exercise compared with the ones before (±10 % or more).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum TrendName {
    /// Fewer than 4 series.
    Unknown,
    Up,
    Stable,
    Down,
}

impl From<statistics::Trend> for TrendName {
    fn from(trend: statistics::Trend) -> Self {
        match trend {
            statistics::Trend::Unknown => TrendName::Unknown,
            statistics::Trend::Up => TrendName::Up,
            statistics::Trend::Stable => TrendName::Stable,
            statistics::Trend::Down => TrendName::Down,
        }
    }
}

/// Statistics of a profile for the parents' space. Instants are ISO 8601 (UTC), days are local
/// dates `YYYY-MM-DD`, rates are percentages (null without any answer).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct ProfileStatisticsResponse {
    pub profile_id: String,
    pub summary: StatisticsSummaryResponse,
    /// The last 14 days, today last.
    pub days: Vec<DayActivityResponse>,
    /// The last 8 weeks (Monday to Sunday), the current week last.
    pub weeks: Vec<WeekActivityResponse>,
    /// Every exercise practised, the most recently practised first.
    pub skills: Vec<SkillStatisticsResponse>,
    /// Up to 3 exercises to review first (ids of `skills`), the weakest first.
    pub to_review: Vec<String>,
    /// Exams and timed challenges, the most recent first.
    pub tests: Vec<TestResultResponse>,
    pub frequent_mistakes: Vec<FrequentMistakeResponse>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct StatisticsSummaryResponse {
    pub session_count: u32,
    /// Time spent in the exercises, exams, challenges and lesson quizzes.
    pub total_seconds: u32,
    pub points_earned: i32,
    pub answer_count: u32,
    pub correct_count: u32,
    pub success_rate: Option<u32>,
    /// Days with an activity among the last 14.
    pub active_days: u32,
    pub last_activity_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct DayActivityResponse {
    pub date: String,
    pub seconds: u32,
    pub session_count: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct WeekActivityResponse {
    /// The Monday.
    pub start: String,
    pub seconds: u32,
    pub session_count: u32,
    pub answer_count: u32,
    pub correct_count: u32,
    pub success_rate: Option<u32>,
    pub points_earned: i32,
}

/// An exercise practised by the profile.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SkillStatisticsResponse {
    pub skill_id: String,
    /// Title of the skill (its id when the skill no longer exists).
    pub title: String,
    pub subject: Subject,
    /// Grade of the skill, when it still exists.
    pub grade: Option<Grade>,
    /// Series answered to the end.
    pub completed_count: u32,
    /// Series stopped before the end.
    pub abandoned_count: u32,
    /// Every answer, exams and timed challenges included.
    pub answer_count: u32,
    pub correct_count: u32,
    /// Share of right answers among the last 20 (the score shown to the child).
    pub mastery: u32,
    pub state: MasteryStateName,
    /// Points of the series of this exercise (an exam mixes exercises: its points are not split).
    pub points_earned: i32,
    pub average_points: Option<u32>,
    pub best_points: Option<i32>,
    pub trend: TrendName,
    pub last_practiced_at: String,
    /// The last 20 completed series, oldest first: the points chart.
    pub history: Vec<SeriesPointResponse>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SeriesPointResponse {
    pub started_at: String,
    pub date: String,
    pub session_type: SessionType,
    pub question_count: u32,
    pub correct_count: u32,
    pub points_earned: i32,
}

/// An exam or a timed challenge.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct TestResultResponse {
    pub session_type: SessionType,
    pub subject: Subject,
    pub started_at: String,
    pub date: String,
    pub question_count: u32,
    pub correct_count: u32,
    pub points_earned: i32,
    pub time_limit_seconds: Option<u32>,
    pub abandoned: bool,
}

/// A question often missed, with the wrong answer given most often.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct FrequentMistakeResponse {
    pub skill_id: String,
    pub skill_title: String,
    /// The question as written, `?` for the answer.
    pub prompt: String,
    pub expected: String,
    pub mistake_count: u32,
    pub attempt_count: u32,
    pub usual_answer: String,
    pub last_at: String,
}

/// A local date as `YYYY-MM-DD`.
pub fn local_date(date: skills::Date) -> String {
    format!("{:04}-{:02}-{:02}", date.year, date.month, date.day)
}

fn clamp_i32(value: i64) -> i32 {
    i32::try_from(value).unwrap_or(if value < 0 { i32::MIN } else { i32::MAX })
}

impl ProfileStatisticsResponse {
    /// `skill`: title and grade of a skill id, when it still exists.
    pub fn new(
        profile_id: String,
        statistics: &statistics::ProfileStatistics,
        skill: &dyn Fn(&str) -> Option<(String, Grade)>,
    ) -> Self {
        let title = |id: &str| skill(id).map(|(t, _)| t).unwrap_or_else(|| id.to_owned());
        let summary = &statistics.summary;
        Self {
            profile_id,
            summary: StatisticsSummaryResponse {
                session_count: summary.session_count,
                total_seconds: summary.total_seconds,
                points_earned: clamp_i32(summary.points_earned),
                answer_count: summary.answer_count,
                correct_count: summary.correct_count,
                success_rate: summary.success_rate.map(u32::from),
                active_days: summary.active_days,
                last_activity_at: summary.last_activity_at.map(iso_8601),
            },
            days: statistics
                .days
                .iter()
                .map(|d| DayActivityResponse {
                    date: local_date(d.date),
                    seconds: d.seconds,
                    session_count: d.session_count,
                })
                .collect(),
            weeks: statistics
                .weeks
                .iter()
                .map(|w| WeekActivityResponse {
                    start: local_date(w.start),
                    seconds: w.seconds,
                    session_count: w.session_count,
                    answer_count: w.answer_count,
                    correct_count: w.correct_count,
                    success_rate: w.success_rate.map(u32::from),
                    points_earned: clamp_i32(w.points_earned),
                })
                .collect(),
            skills: statistics
                .skills
                .iter()
                .map(|s| {
                    let found = skill(&s.skill_id);
                    SkillStatisticsResponse {
                        skill_id: s.skill_id.clone(),
                        title: found
                            .as_ref()
                            .map(|(t, _)| t.clone())
                            .unwrap_or_else(|| s.skill_id.clone()),
                        subject: s.subject.into(),
                        grade: found.map(|(_, g)| g),
                        completed_count: s.completed_count,
                        abandoned_count: s.abandoned_count,
                        answer_count: s.answer_count,
                        correct_count: s.correct_count,
                        mastery: u32::from(s.mastery),
                        state: s.state.into(),
                        points_earned: clamp_i32(s.points_earned),
                        average_points: s.average_points,
                        best_points: s.best_points,
                        trend: s.trend.into(),
                        last_practiced_at: iso_8601(s.last_practiced_at),
                        history: s
                            .history
                            .iter()
                            .map(|p| SeriesPointResponse {
                                started_at: iso_8601(p.started_at),
                                date: local_date(p.date),
                                session_type: p.session_type.into(),
                                question_count: p.question_count,
                                correct_count: p.correct_count,
                                points_earned: p.points_earned,
                            })
                            .collect(),
                    }
                })
                .collect(),
            to_review: statistics.to_review.clone(),
            tests: statistics
                .tests
                .iter()
                .map(|t| TestResultResponse {
                    session_type: t.session_type.into(),
                    subject: t.subject.into(),
                    started_at: iso_8601(t.started_at),
                    date: local_date(t.date),
                    question_count: t.question_count,
                    correct_count: t.correct_count,
                    points_earned: t.points_earned,
                    time_limit_seconds: t.time_limit_seconds,
                    abandoned: t.abandoned,
                })
                .collect(),
            frequent_mistakes: statistics
                .frequent_mistakes
                .iter()
                .map(|m| FrequentMistakeResponse {
                    skill_id: m.skill_id.clone(),
                    skill_title: title(&m.skill_id),
                    prompt: m.prompt.clone(),
                    expected: m.expected.clone(),
                    mistake_count: m.mistake_count,
                    attempt_count: m.attempt_count,
                    usual_answer: m.usual_answer.clone(),
                    last_at: iso_8601(m.last_at),
                })
                .collect(),
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
