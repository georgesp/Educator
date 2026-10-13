use super::exam::{compose_exam, compose_timed, exam_skills, TIMED_DURATIONS, TIMED_MIN_MASTERY};
use super::{generate_series_as, Answer, Exercise, Picture, DEFAULT_LENGTH};
use crate::clock::{uuid_v7, Clock};
use crate::lessons::Lesson;
use crate::points::PointsRules;
use crate::profiles::{Profile, ProfileStore};
use crate::random::RandomSource;
use crate::sessions::{
    update_masteries, AnswerRecord, SessionEnd, SessionRecord, SessionStore, SessionType,
};
use crate::skills::{AnswerMode, ExerciseFormat, Skill, SkillCatalog, Subject};
use crate::spelling::SpellingCorrection;
use crate::{Error, Result};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};
use uuid::Uuid;

/// Answers accepted after the end of a timed challenge (the last click, the network of the window).
pub const TIMED_GRACE_MS: i64 = 5_000;

/// Series kept in memory at most (the oldest are dropped).
pub const SERIES_CAPACITY: usize = 50;

/// A series in progress: a series of exercises of one skill, an exam or a timed challenge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Series {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub session_type: SessionType,
    pub subject: Subject,
    /// The skill of a series of exercises; `None` for an exam or a timed challenge.
    pub skill_id: Option<String>,
    /// Instruction read and shown before the first exercise (the skill's one); empty for an exam, a
    /// timed challenge or a quiz, whose instructions are texts of the front.
    pub instruction: String,
    pub answer_mode: AnswerMode,
    /// Kind of exercise chosen (dictation…), when the skill offers several.
    pub format: Option<ExerciseFormat>,
    pub seed: u32,
    pub exercises: Vec<Exercise>,
    /// Skill of each exercise.
    pub skill_ids: Vec<String>,
    /// Duration of a timed challenge.
    pub time_limit_seconds: Option<u32>,
    /// The lesson of a quiz.
    pub lesson_id: Option<String>,
    /// When the series started (UTC Unix milliseconds).
    pub started_at: i64,
    answers: Vec<Option<bool>>,
    closed: bool,
}

impl Series {
    pub fn correct_count(&self) -> usize {
        self.answers.iter().filter(|a| **a == Some(true)).count()
    }

    pub fn answered_count(&self) -> usize {
        self.answers.iter().filter(|a| a.is_some()).count()
    }

    /// Right (`Some(true)`), wrong or not answered yet, for each exercise.
    pub fn results(&self) -> &[Option<bool>] {
        &self.answers
    }

    fn record(&mut self, index: usize, answer: &Answer) -> Result<(Exercise, bool, bool)> {
        if self.closed {
            return Err(Error::SeriesRequest("The series is over.".to_owned()));
        }

        if index >= self.exercises.len() {
            return Err(Error::SeriesRequest(format!(
                "No exercise {index} in this series."
            )));
        }

        if self.answers[index].is_some() {
            return Err(Error::SeriesRequest(format!(
                "Exercise {index} is already answered."
            )));
        }

        let exercise = self.exercises[index].clone();
        let correct = exercise.is_correct(answer);
        self.answers[index] = Some(correct);
        // A timed challenge ends with the time, not with the last question.
        self.closed =
            self.session_type != SessionType::Timed && self.answers.iter().all(Option::is_some);
        Ok((exercise, correct, self.closed))
    }
}

/// Result of one answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnswerOutcome {
    pub correct: bool,
    pub expected: Answer,
    /// The expected answer as written in the correction (« 3/4 », « < », « 300 et 400 »).
    pub expected_label: String,
    pub explanation: String,
    /// Picture shown with the correction (decomposition, bar model).
    pub help: Option<Picture>,
    /// Spelling: the child's word, letter by letter, against the expected one.
    pub correction: Option<SpellingCorrection>,
    /// End of series (points awarded), when this answer was the last one.
    pub summary: Option<SeriesSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeriesSummary {
    pub correct_count: usize,
    pub exercise_count: usize,
    /// Points earned by this series.
    pub points_earned: i32,
    /// New total of the profile.
    pub profile_points: i32,
    /// Right, wrong or not answered, for each exercise of the series (results by skill of an exam).
    pub results: Vec<Option<bool>>,
    /// Skill of each exercise.
    pub skill_ids: Vec<String>,
    /// Timed challenge: the best score before this one, at the same duration (none yet: `None`).
    pub previous_best: Option<u32>,
    /// Timed challenge: at least one right answer and better than [`Self::previous_best`].
    pub new_record: bool,
    /// Lesson quiz: fewer than half right answers, the lesson is worth reading again (SUB001411).
    pub review_lesson: bool,
}

/// The results of a series on one skill.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillResult {
    pub skill_id: String,
    pub correct_count: usize,
    /// Questions answered on the skill.
    pub question_count: usize,
}

impl SkillResult {
    /// A strong point of an exam (at least two thirds right); otherwise a notion to work on again.
    pub fn is_strong(&self) -> bool {
        self.question_count > 0 && self.correct_count * 3 >= self.question_count * 2
    }
}

impl SeriesSummary {
    /// Results by skill, in the order the skills first appear; the questions not answered are left out.
    pub fn by_skill(&self) -> Vec<SkillResult> {
        let mut results: Vec<SkillResult> = Vec::new();
        for (skill_id, result) in self.skill_ids.iter().zip(&self.results) {
            let Some(correct) = result else {
                continue;
            };

            let index = match results.iter().position(|r| &r.skill_id == skill_id) {
                Some(index) => index,
                None => {
                    results.push(SkillResult {
                        skill_id: skill_id.clone(),
                        correct_count: 0,
                        question_count: 0,
                    });
                    results.len() - 1
                }
            };
            results[index].question_count += 1;
            results[index].correct_count += usize::from(*correct);
        }

        results
    }
}

/// What a new series is made of.
#[derive(Debug, Clone)]
pub struct NewSeries {
    pub session_type: SessionType,
    pub subject: Subject,
    pub skill_id: Option<String>,
    /// The lesson of a quiz.
    pub lesson_id: Option<String>,
    pub instruction: String,
    pub answer_mode: AnswerMode,
    pub format: Option<ExerciseFormat>,
    pub seed: u32,
    pub exercises: Vec<Exercise>,
    pub skill_ids: Vec<String>,
    pub time_limit_seconds: Option<u32>,
}

#[derive(Default)]
struct InProgress {
    series: HashMap<Uuid, Series>,
    order: VecDeque<Uuid>,
}

/// Series of exercises (SUB000107, SUB000110, SUB001401): generation, answer checking, points at the
/// end. Every series is recorded as a session with its answers ([`SessionStore`]); the exercises of
/// the series in progress stay in memory, behind a mutex (a series does not survive a restart: the
/// sessions left open are closed as abandoned at start-up). Points are added to the profile once,
/// when the series ends (last answer) or is abandoned; the masteries of its skills are then updated.
pub struct SeriesService {
    skills: Arc<dyn SkillCatalog>,
    profiles: Arc<dyn ProfileStore>,
    sessions: Arc<dyn SessionStore>,
    points: PointsRules,
    random: Arc<dyn RandomSource>,
    clock: Arc<dyn Clock>,
    in_progress: Mutex<InProgress>,
}

impl fmt::Debug for SeriesService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SeriesService")
            .field("points", &self.points)
            .finish_non_exhaustive()
    }
}

impl SeriesService {
    pub fn new(
        skills: Arc<dyn SkillCatalog>,
        profiles: Arc<dyn ProfileStore>,
        sessions: Arc<dyn SessionStore>,
        points: PointsRules,
        random: Arc<dyn RandomSource>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            skills,
            profiles,
            sessions,
            points,
            random,
            clock,
            in_progress: Mutex::new(InProgress::default()),
        }
    }

    pub fn points(&self) -> PointsRules {
        self.points
    }

    pub fn skills(&self) -> &Arc<dyn SkillCatalog> {
        &self.skills
    }

    pub fn sessions(&self) -> &Arc<dyn SessionStore> {
        &self.sessions
    }

    /// The profile, or [`Error::ProfileNotFound`].
    pub fn profile(&self, profile_id: Uuid) -> Result<Profile> {
        self.profiles
            .get(profile_id)?
            .ok_or(Error::ProfileNotFound(profile_id))
    }

    /// A seed drawn when none is given.
    pub fn seed_or_random(&self, seed: Option<u32>) -> u32 {
        seed.unwrap_or_else(|| self.random.next(i32::MAX as u32))
    }

    /// A series of exercises of one skill. `format`: the kind of exercise chosen (dictation…), all
    /// the formats of the skill allowed by the answer mode when `None`. `seed`: fixed seed (tests,
    /// reproducing a series); random when `None`.
    pub fn start(
        &self,
        profile_id: Uuid,
        skill_id: &str,
        answer_mode: AnswerMode,
        format: Option<ExerciseFormat>,
        seed: Option<u32>,
    ) -> Result<Series> {
        self.profile(profile_id)?;
        let skill = self
            .skills
            .find(skill_id)
            .ok_or_else(|| Error::SeriesRequest(format!("Unknown skill {skill_id}.")))?;
        if !skill.answer_modes.contains(&answer_mode) {
            return Err(Error::SeriesRequest(format!(
                "Skill {} does not allow the answer mode {answer_mode:?}.",
                skill.id
            )));
        }

        if let Some(format) = format {
            if !skill.formats.contains(&format) {
                return Err(Error::SeriesRequest(format!(
                    "Skill {} does not offer the format {}.",
                    skill.id,
                    format.name()
                )));
            }
        }

        let seed = self.seed_or_random(seed);
        let exercises =
            generate_series_as(skill, u64::from(seed), answer_mode, DEFAULT_LENGTH, format)?;
        self.open(
            profile_id,
            NewSeries {
                session_type: SessionType::Exercise,
                subject: skill.subject,
                skill_id: Some(skill.id.clone()),
                lesson_id: None,
                instruction: skill.instruction.clone(),
                answer_mode,
                format,
                seed,
                skill_ids: vec![skill.id.clone(); exercises.len()],
                exercises,
                time_limit_seconds: None,
            },
        )
    }

    /// An exam (SUB001414): 15 questions of the subject's skills of the profile's grade up to the
    /// current period, the recent ones weighing more.
    pub fn start_exam(
        &self,
        profile_id: Uuid,
        subject: Subject,
        current_period: u8,
        seed: Option<u32>,
    ) -> Result<Series> {
        let profile = self.profile(profile_id)?;
        let (_, offered) = self.skills.offered_or_fallback(subject, profile.grade);
        let skills = exam_skills(&offered, current_period);
        let seed = self.seed_or_random(seed);
        let questions = compose_exam(&skills, u64::from(seed))?;
        self.open(
            profile_id,
            NewSeries {
                session_type: SessionType::Exam,
                subject,
                skill_id: None,
                lesson_id: None,
                // The front shows its own instruction for an exam, a challenge, a quiz (texts in fr.ts).
                instruction: String::new(),
                answer_mode: AnswerMode::Keyboard,
                format: None,
                seed,
                skill_ids: questions.iter().map(|q| q.skill_id.clone()).collect(),
                exercises: questions.into_iter().map(|q| q.exercise).collect(),
                time_limit_seconds: None,
            },
        )
    }

    /// The skills a profile may meet in a timed challenge: those of its grade (revisions included)
    /// mastered at [`TIMED_MIN_MASTERY`] or more.
    pub fn timed_skills(&self, profile_id: Uuid, subject: Subject) -> Result<Vec<String>> {
        let profile = self.profile(profile_id)?;
        let masteries = self.sessions.masteries(profile_id)?;
        let (_, offered) = self.skills.offered_or_fallback(subject, profile.grade);
        Ok(offered
            .iter()
            .filter(|o| {
                masteries
                    .iter()
                    .any(|m| m.skill_id == o.skill.id && m.score >= TIMED_MIN_MASTERY)
            })
            .map(|o| o.skill.id.clone())
            .collect())
    }

    /// A timed challenge (SUB001415) of `seconds` on the mastered skills; it ends with
    /// [`Self::finish`].
    pub fn start_timed(
        &self,
        profile_id: Uuid,
        subject: Subject,
        seconds: u32,
        seed: Option<u32>,
    ) -> Result<Series> {
        if !TIMED_DURATIONS.contains(&seconds) {
            return Err(Error::SeriesRequest(format!(
                "A timed challenge lasts {TIMED_DURATIONS:?} seconds, not {seconds}."
            )));
        }

        let ids = self.timed_skills(profile_id, subject)?;
        let skills: Vec<&Skill> = ids.iter().filter_map(|id| self.skills.find(id)).collect();
        let seed = self.seed_or_random(seed);
        let questions = compose_timed(&skills, u64::from(seed))?;
        self.open(
            profile_id,
            NewSeries {
                session_type: SessionType::Timed,
                subject,
                skill_id: None,
                lesson_id: None,
                instruction: String::new(),
                answer_mode: AnswerMode::Choice,
                format: None,
                seed,
                skill_ids: questions.iter().map(|q| q.skill_id.clone()).collect(),
                exercises: questions.into_iter().map(|q| q.exercise).collect(),
                time_limit_seconds: Some(seconds),
            },
        )
    }

    /// The quiz of a lesson (SUB001411), recorded as a session `LessonQuiz` on the lesson's main skill.
    pub fn start_lesson_quiz(&self, profile_id: Uuid, lesson: &Lesson) -> Result<Series> {
        let exercises = lesson.quiz_exercises();
        let skill_id = lesson
            .skill_ids
            .first()
            .cloned()
            .unwrap_or_else(|| lesson.id.clone());
        self.open(
            profile_id,
            NewSeries {
                session_type: SessionType::LessonQuiz,
                subject: lesson.subject,
                skill_id: Some(skill_id.clone()),
                lesson_id: Some(lesson.id.clone()),
                instruction: String::new(),
                answer_mode: AnswerMode::Keyboard,
                format: None,
                seed: 0,
                skill_ids: vec![skill_id; exercises.len()],
                exercises,
                time_limit_seconds: None,
            },
        )
    }

    /// Records a new series (an exam, a timed challenge, a series of exercises) and keeps it in memory.
    pub fn open(&self, profile_id: Uuid, new: NewSeries) -> Result<Series> {
        let profile = self.profile(profile_id)?;
        let now = self.clock.now_ms();
        let series = Series {
            id: uuid_v7(now),
            profile_id,
            session_type: new.session_type,
            subject: new.subject,
            skill_id: new.skill_id,
            instruction: new.instruction,
            answer_mode: new.answer_mode,
            format: new.format,
            seed: new.seed,
            answers: vec![None; new.exercises.len()],
            exercises: new.exercises,
            skill_ids: new.skill_ids,
            time_limit_seconds: new.time_limit_seconds,
            lesson_id: new.lesson_id,
            started_at: now,
            closed: false,
        };
        self.sessions.add_session(&SessionRecord {
            id: series.id,
            profile_id,
            session_type: series.session_type,
            subject: series.subject,
            grade: profile.grade,
            skill_id: series.skill_id.clone(),
            lesson_id: series.lesson_id.clone(),
            // An exam and a timed challenge mix typed and picked answers.
            answer_mode: match series.session_type {
                SessionType::Exam | SessionType::Timed => None,
                _ => Some(series.answer_mode),
            },
            format: series.format,
            seed: series.seed,
            question_count: count(series.exercises.len()),
            time_limit_seconds: series.time_limit_seconds,
            started_at: now,
            ended_at: None,
            abandoned: false,
            correct_count: 0,
            points_earned: 0,
        })?;

        let mut evicted = Vec::new();
        {
            let mut state = self.lock();
            state.series.insert(series.id, series.clone());
            state.order.push_back(series.id);
            while state.series.len() > SERIES_CAPACITY {
                match state.order.pop_front() {
                    Some(oldest) => evicted.extend(state.series.remove(&oldest)),
                    None => break,
                }
            }
        }
        // A series pushed out of the memory is over: recorded as abandoned, like at start-up. Its
        // failure must not prevent the new series.
        for old in evicted.into_iter().filter(|s| !s.closed) {
            let profile = old.profile_id;
            let _ = self.close_series(old, profile, true);
        }

        Ok(series)
    }

    /// An exercise of a series in progress (the word of a dictation, read by the voice).
    pub fn exercise(&self, series_id: Uuid, index: usize) -> Result<Exercise> {
        let state = self.lock();
        let series = state
            .series
            .get(&series_id)
            .ok_or(Error::SeriesNotFound(series_id))?;
        series
            .exercises
            .get(index)
            .cloned()
            .ok_or_else(|| Error::SeriesRequest(format!("No exercise {index} in this series.")))
    }

    /// `duration_ms`: time taken by the child, measured by the front.
    pub fn answer(
        &self,
        series_id: Uuid,
        index: usize,
        answer: &Answer,
        duration_ms: Option<u32>,
    ) -> Result<AnswerOutcome> {
        let now = self.clock.now_ms();
        let (exercise, correct, completed, profile_id, skill_id) = {
            let mut state = self.lock();
            let series = state
                .series
                .get_mut(&series_id)
                .ok_or(Error::SeriesNotFound(series_id))?;
            if let Some(seconds) = series.time_limit_seconds {
                if now > series.started_at + i64::from(seconds) * 1000 + TIMED_GRACE_MS {
                    return Err(Error::SeriesRequest("The time is over.".to_owned()));
                }
            }
            let (exercise, correct, completed) = series.record(index, answer)?;
            (
                exercise,
                correct,
                completed,
                series.profile_id,
                series.skill_ids[index].clone(),
            )
        };

        self.sessions.add_answer(&AnswerRecord {
            id: uuid_v7(now),
            session_id: series_id,
            index: count(index),
            skill_id,
            prompt: exercise.prompt(),
            expected: exercise.expected_label(),
            given: answer.label(),
            is_correct: correct,
            duration_ms,
            answered_at: now,
        })?;

        let summary = if completed {
            Some(self.close(series_id, profile_id, false)?)
        } else {
            None
        };

        Ok(AnswerOutcome {
            correct,
            expected_label: exercise.expected_label(),
            correction: exercise.correction(answer),
            expected: exercise.expected,
            explanation: exercise.explanation,
            help: exercise.help,
            summary,
        })
    }

    /// The child leaves: the correct answers already given are kept, without bonus.
    pub fn abandon(&self, series_id: Uuid) -> Result<SeriesSummary> {
        let profile_id = self.mark_closed(series_id)?;
        self.close(series_id, profile_id, true)
    }

    /// End of a timed challenge: the questions answered count, with the bonus of a finished series.
    pub fn finish(&self, series_id: Uuid) -> Result<SeriesSummary> {
        {
            let state = self.lock();
            let series = state
                .series
                .get(&series_id)
                .ok_or(Error::SeriesNotFound(series_id))?;
            if series.session_type != SessionType::Timed {
                return Err(Error::SeriesRequest(
                    "Only a timed challenge ends before its last question.".to_owned(),
                ));
            }
        }
        let profile_id = self.mark_closed(series_id)?;
        self.close(series_id, profile_id, false)
    }

    /// A profile is deleted: its series in progress are forgotten (its sessions go with it).
    pub fn forget_profile(&self, profile_id: Uuid) {
        let mut state = self.lock();
        state.series.retain(|_, s| s.profile_id != profile_id);
        let InProgress { series, order } = &mut *state;
        order.retain(|id| series.contains_key(id));
    }

    fn mark_closed(&self, series_id: Uuid) -> Result<Uuid> {
        let mut state = self.lock();
        let series = state
            .series
            .get_mut(&series_id)
            .ok_or(Error::SeriesNotFound(series_id))?;
        if series.closed {
            return Err(Error::SeriesRequest("The series is over.".to_owned()));
        }

        series.closed = true;
        Ok(series.profile_id)
    }

    /// Points, end of the session, masteries; the series leaves the memory.
    fn close(&self, series_id: Uuid, profile_id: Uuid, abandoned: bool) -> Result<SeriesSummary> {
        let series = {
            let mut state = self.lock();
            state.order.retain(|id| *id != series_id);
            state
                .series
                .remove(&series_id)
                .ok_or(Error::SeriesNotFound(series_id))?
        };
        self.close_series(series, profile_id, abandoned)
    }

    fn close_series(
        &self,
        series: Series,
        profile_id: Uuid,
        abandoned: bool,
    ) -> Result<SeriesSummary> {
        let series_id = series.id;
        let correct_count = series.correct_count();
        let answered = series.answered_count();
        let timed = series.session_type == SessionType::Timed;
        // Read before this challenge is recorded as ended.
        let previous_best = match series.time_limit_seconds {
            Some(seconds) if timed && !abandoned => {
                self.sessions
                    .best_timed_score(profile_id, series.subject, seconds)?
            }
            _ => None,
        };
        let (earned, question_count) = match series.session_type {
            // A timed challenge has more questions than a child can answer: only those answered.
            _ if abandoned => (
                self.points.for_abandoned_series(correct_count),
                if timed {
                    answered
                } else {
                    series.exercises.len()
                },
            ),
            SessionType::Timed => (
                self.points.for_timed_challenge(correct_count, answered),
                answered,
            ),
            _ => (
                self.points
                    .for_completed_series(correct_count, series.exercises.len()),
                series.exercises.len(),
            ),
        };
        let total = self
            .profiles
            .add_points(profile_id, earned)?
            .ok_or(Error::ProfileNotFound(profile_id))?;

        let now = self.clock.now_ms();
        self.sessions.end_session(
            series_id,
            &SessionEnd {
                ended_at: now,
                abandoned,
                question_count: count(question_count),
                correct_count: count(correct_count),
                points_earned: earned,
            },
        )?;
        let practised: Vec<String> = series
            .skill_ids
            .iter()
            .zip(series.results())
            .filter(|(_, r)| r.is_some())
            .map(|(s, _)| s.clone())
            .collect();
        update_masteries(self.sessions.as_ref(), profile_id, &practised, now)?;

        let (results, skill_ids) = if series.session_type == SessionType::Timed {
            // Only the questions reached.
            let reached = series.answers.iter().take_while(|a| a.is_some()).count();
            (
                series.answers[..reached].to_vec(),
                series.skill_ids[..reached].to_vec(),
            )
        } else {
            (series.answers.clone(), series.skill_ids.clone())
        };
        let new_record = timed
            && !abandoned
            && correct_count > 0
            && previous_best.is_none_or(|best| count(correct_count) > best);
        let review_lesson = series.session_type == SessionType::LessonQuiz
            && question_count > 0
            && correct_count * 2 < question_count;
        Ok(SeriesSummary {
            correct_count,
            exercise_count: question_count,
            points_earned: earned,
            profile_points: total,
            results,
            skill_ids,
            previous_best,
            new_record,
            review_lesson,
        })
    }

    fn lock(&self) -> MutexGuard<'_, InProgress> {
        self.in_progress.lock().unwrap_or_else(|e| e.into_inner())
    }
}

fn count(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}
