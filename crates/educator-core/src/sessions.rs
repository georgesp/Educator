//! Record of the sessions (SUB001401): every series of exercises, lesson quiz, exam and timed
//! challenge is a [`SessionRecord`] with one [`AnswerRecord`] per question, stored locally
//! (`educator-data`). At the end of a session, the [`SkillMastery`] of each skill practised is
//! updated: the share of right answers among the last [`MASTERY_WINDOW`] answers. They feed the
//! adaptive difficulty and the parents' space (lot 3, [`crate::statistics`]), and the timed challenge (SUB001415).

use crate::skills::{AnswerMode, ExerciseFormat, Subject};
use crate::{Grade, Result};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

/// Number of last answers of a skill averaged by its mastery.
pub const MASTERY_WINDOW: usize = 20;

/// What the child did.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionType {
    /// A series of exercises of one skill.
    Exercise,
    /// An exam mixing the skills of the grade (SUB001414).
    Exam,
    /// A timed challenge (SUB001415).
    Timed,
    /// The quiz at the end of a lesson (SUB001411).
    LessonQuiz,
    /// The level assessment taken once per subject after the creation of a profile. Only a
    /// finished one counts: an abandoned one is left out of the statistics and the masteries.
    Placement,
    /// « Mes révisions » (SUB001509): the mistakes of the review queue due today.
    Review,
}

impl SessionType {
    pub const ALL: [SessionType; 6] = [
        SessionType::Exercise,
        SessionType::Exam,
        SessionType::Timed,
        SessionType::LessonQuiz,
        SessionType::Placement,
        SessionType::Review,
    ];

    /// Stored name.
    pub fn as_str(self) -> &'static str {
        match self {
            SessionType::Exercise => "Exercise",
            SessionType::Exam => "Exam",
            SessionType::Timed => "Timed",
            SessionType::LessonQuiz => "LessonQuiz",
            SessionType::Placement => "Placement",
            SessionType::Review => "Review",
        }
    }
}

/// An unknown stored name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownName(pub String);

impl fmt::Display for UnknownName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown name '{}'.", self.0)
    }
}

impl std::error::Error for UnknownName {}

impl FromStr for SessionType {
    type Err = UnknownName;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        SessionType::ALL
            .into_iter()
            .find(|t| t.as_str() == value)
            .ok_or_else(|| UnknownName(value.to_owned()))
    }
}

/// A session of a profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRecord {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub session_type: SessionType,
    pub subject: Subject,
    /// Grade of the profile when the session started.
    pub grade: Grade,
    /// The skill of a series (the main skill of a lesson); `None` for an exam or a timed challenge.
    pub skill_id: Option<String>,
    pub lesson_id: Option<String>,
    pub answer_mode: Option<AnswerMode>,
    /// Kind of exercise chosen by the child (dictation…), when the skill offers several.
    pub format: Option<ExerciseFormat>,
    /// Seed of the generation: the same seed gives the same questions.
    pub seed: u32,
    /// Questions asked (for a timed challenge, set at the end: the questions answered).
    pub question_count: u32,
    /// Duration of a timed challenge.
    pub time_limit_seconds: Option<u32>,
    pub started_at: i64,
    pub ended_at: Option<i64>,
    pub abandoned: bool,
    pub correct_count: u32,
    pub points_earned: i32,
}

/// One answer of a session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnswerRecord {
    pub id: Uuid,
    pub session_id: Uuid,
    /// Position of the question in the session, from 0.
    pub index: u32,
    /// Skill of the question (an exam mixes several).
    pub skill_id: String,
    /// The question as written, `?` for the answer (« 7 + ? = 12 », « bat___ »).
    pub prompt: String,
    pub expected: String,
    pub given: String,
    pub is_correct: bool,
    /// Time taken to answer, measured by the front.
    pub duration_ms: Option<u32>,
    pub answered_at: i64,
}

/// How well a profile masters a skill.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillMastery {
    pub profile_id: Uuid,
    pub skill_id: String,
    /// 0 to 100: share of right answers among the last [`MASTERY_WINDOW`] ones.
    pub score: u8,
    /// Every answer ever given on the skill.
    pub attempt_count: u32,
    pub last_practiced_at: i64,
}

/// End of a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionEnd {
    pub ended_at: i64,
    pub abandoned: bool,
    pub question_count: u32,
    pub correct_count: u32,
    pub points_earned: i32,
}

/// Storage of the sessions, answers and masteries (SQLite, in `educator-data`). Deleting a profile
/// deletes its sessions, answers and masteries.
pub trait SessionStore: Send + Sync {
    fn add_session(&self, session: &SessionRecord) -> Result<()>;

    fn session(&self, id: Uuid) -> Result<Option<SessionRecord>>;

    /// Adds an answer; `false` when this question of the session already has one.
    fn add_answer(&self, answer: &AnswerRecord) -> Result<bool>;

    /// The answers of a session, by index.
    fn answers(&self, session_id: Uuid) -> Result<Vec<AnswerRecord>>;

    /// Closes a session still open; `false` when it is unknown or already closed.
    fn end_session(&self, id: Uuid, end: &SessionEnd) -> Result<bool>;

    /// The sessions not closed yet.
    fn open_sessions(&self) -> Result<Vec<SessionRecord>>;

    /// Closes as abandoned every session left open (the application was closed in the middle);
    /// returns how many.
    fn abandon_open_sessions(&self, now: i64) -> Result<usize>;

    /// Whether the last answers of a profile on a skill were right, the most recent first.
    fn recent_results(&self, profile_id: Uuid, skill_id: &str, limit: usize) -> Result<Vec<bool>>;

    /// Number of answers of a profile on a skill.
    fn answer_count(&self, profile_id: Uuid, skill_id: &str) -> Result<u32>;

    fn save_mastery(&self, mastery: &SkillMastery) -> Result<()>;

    /// Every mastery of a profile, by skill id.
    fn masteries(&self, profile_id: Uuid) -> Result<Vec<SkillMastery>>;

    /// The closed sessions of a profile, oldest first (parents' space, SUB001511).
    fn profile_sessions(&self, profile_id: Uuid) -> Result<Vec<SessionRecord>>;

    /// The answers of the closed sessions of a profile, oldest first.
    fn profile_answers(&self, profile_id: Uuid) -> Result<Vec<AnswerRecord>>;

    /// Best number of right answers of the finished timed challenges of a profile, for a subject
    /// and a duration.
    fn best_timed_score(
        &self,
        profile_id: Uuid,
        subject: Subject,
        time_limit_seconds: u32,
    ) -> Result<Option<u32>>;

    /// The slide where a profile left a lesson (SUB001410), when it did not finish it.
    fn lesson_slide(&self, profile_id: Uuid, lesson_id: &str) -> Result<Option<u32>>;

    /// Keeps the slide where the profile is in a lesson; `None` once the lesson is finished.
    fn save_lesson_slide(
        &self,
        profile_id: Uuid,
        lesson_id: &str,
        slide: Option<u32>,
    ) -> Result<()>;
}

/// Score of a skill from its last results: the share of right answers, rounded (0 without any).
pub fn mastery_score(recent: &[bool]) -> u8 {
    if recent.is_empty() {
        return 0;
    }

    let right = recent.iter().filter(|r| **r).count();
    ((right * 100 + recent.len() / 2) / recent.len()).min(100) as u8
}

/// Recomputes the mastery of each skill of `skill_ids` for a profile, after a session.
/// At start-up: the sessions left open (the application was closed in the middle) are closed as
/// abandoned, and the masteries of the skills they practised take their answers into account.
/// Returns how many sessions were closed.
pub fn close_open_sessions(store: &dyn SessionStore, now: i64) -> Result<usize> {
    let open = store.open_sessions()?;
    let closed = store.abandon_open_sessions(now)?;
    for session in open {
        let practised: Vec<String> = store
            .answers(session.id)?
            .into_iter()
            .map(|a| a.skill_id)
            .collect();
        update_masteries(store, session.profile_id, &practised, now)?;
    }
    Ok(closed)
}

pub fn update_masteries(
    store: &dyn SessionStore,
    profile_id: Uuid,
    skill_ids: &[String],
    now: i64,
) -> Result<()> {
    let mut done: Vec<&str> = Vec::new();
    for skill_id in skill_ids {
        if done.contains(&skill_id.as_str()) {
            continue;
        }

        done.push(skill_id);
        let recent = store.recent_results(profile_id, skill_id, MASTERY_WINDOW)?;
        if recent.is_empty() {
            continue;
        }

        store.save_mastery(&SkillMastery {
            profile_id,
            skill_id: skill_id.clone(),
            score: mastery_score(&recent),
            attempt_count: store.answer_count(profile_id, skill_id)?,
            last_practiced_at: now,
        })?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_score_is_the_rounded_share_of_right_answers() {
        assert_eq!(0, mastery_score(&[]));
        assert_eq!(100, mastery_score(&[true; 20]));
        assert_eq!(0, mastery_score(&[false, false]));
        assert_eq!(67, mastery_score(&[true, true, false]));
        assert_eq!(50, mastery_score(&[true, false]));
    }

    #[test]
    fn session_types_round_trip() {
        for t in SessionType::ALL {
            assert_eq!(Ok(t), t.as_str().parse());
        }
        assert!("exam".parse::<SessionType>().is_err());
    }
}
