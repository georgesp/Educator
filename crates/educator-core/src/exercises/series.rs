use super::{generate_series, Exercise, DEFAULT_LENGTH};
use crate::clock::{uuid_v7, Clock};
use crate::points::PointsRules;
use crate::profiles::ProfileStore;
use crate::random::RandomSource;
use crate::skills::{AnswerMode, Skill, SkillCatalog};
use crate::{Error, Result};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};
use uuid::Uuid;

/// Series kept in memory at most (the oldest are dropped).
pub const SERIES_CAPACITY: usize = 50;

/// A series in progress.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Series {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub skill: Skill,
    pub answer_mode: AnswerMode,
    pub seed: u32,
    pub exercises: Vec<Exercise>,
    answers: Vec<Option<bool>>,
    closed: bool,
}

impl Series {
    pub fn correct_count(&self) -> usize {
        self.answers.iter().filter(|a| **a == Some(true)).count()
    }

    fn record(&mut self, index: usize, answer: i32) -> Result<(Exercise, bool, bool)> {
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
        self.closed = self.answers.iter().all(Option::is_some);
        Ok((exercise, correct, self.closed))
    }
}

/// Result of one answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnswerOutcome {
    pub correct: bool,
    pub expected: i32,
    pub explanation: String,
    /// End of series (points awarded), when this answer was the last one.
    pub summary: Option<SeriesSummary>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeriesSummary {
    pub correct_count: usize,
    pub exercise_count: usize,
    /// Points earned by this series.
    pub points_earned: i32,
    /// New total of the profile.
    pub profile_points: i32,
}

#[derive(Default)]
struct InProgress {
    series: HashMap<Uuid, Series>,
    order: VecDeque<Uuid>,
}

/// Series of exercises (SUB000107, SUB000110): generation, answer checking, points at the end.
/// The series in progress live in memory, behind a mutex (short-lived); the detailed record of
/// sessions comes with lot 2. Points are added to the profile once, when the series ends (last
/// answer) or is abandoned.
pub struct SeriesService {
    skills: Arc<dyn SkillCatalog>,
    profiles: Arc<dyn ProfileStore>,
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
        points: PointsRules,
        random: Arc<dyn RandomSource>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            skills,
            profiles,
            points,
            random,
            clock,
            in_progress: Mutex::new(InProgress::default()),
        }
    }

    pub fn points(&self) -> PointsRules {
        self.points
    }

    /// `seed`: fixed seed (tests, reproducing a series); random when `None`.
    pub fn start(
        &self,
        profile_id: Uuid,
        skill_id: &str,
        answer_mode: AnswerMode,
        seed: Option<u32>,
    ) -> Result<Series> {
        self.profiles
            .get(profile_id)?
            .ok_or(Error::ProfileNotFound(profile_id))?;
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

        let seed = seed.unwrap_or_else(|| self.random.next(i32::MAX as u32));
        let exercises = generate_series(skill, u64::from(seed), answer_mode, DEFAULT_LENGTH)?;
        let series = Series {
            id: uuid_v7(self.clock.now_ms()),
            profile_id,
            skill: skill.clone(),
            answer_mode,
            seed,
            answers: vec![None; exercises.len()],
            exercises,
            closed: false,
        };

        let mut state = self.lock();
        state.series.insert(series.id, series.clone());
        state.order.push_back(series.id);
        while state.series.len() > SERIES_CAPACITY {
            match state.order.pop_front() {
                Some(oldest) => {
                    state.series.remove(&oldest);
                }
                None => break,
            }
        }

        Ok(series)
    }

    pub fn answer(&self, series_id: Uuid, index: usize, answer: i32) -> Result<AnswerOutcome> {
        let (exercise, correct, completed, profile_id, correct_count, count) = {
            let mut state = self.lock();
            let series = state
                .series
                .get_mut(&series_id)
                .ok_or(Error::SeriesNotFound(series_id))?;
            let (exercise, correct, completed) = series.record(index, answer)?;
            (
                exercise,
                correct,
                completed,
                series.profile_id,
                series.correct_count(),
                series.exercises.len(),
            )
        };

        let summary = if completed {
            let earned = self.points.for_completed_series(correct_count, count);
            Some(self.award(series_id, profile_id, correct_count, count, earned)?)
        } else {
            None
        };

        Ok(AnswerOutcome {
            correct,
            expected: exercise.expected(),
            explanation: exercise.explanation(),
            summary,
        })
    }

    /// The child leaves: the correct answers already given are kept, without bonus.
    pub fn abandon(&self, series_id: Uuid) -> Result<SeriesSummary> {
        let (profile_id, correct_count, count) = {
            let mut state = self.lock();
            let series = state
                .series
                .get_mut(&series_id)
                .ok_or(Error::SeriesNotFound(series_id))?;
            if series.closed {
                return Err(Error::SeriesRequest("The series is over.".to_owned()));
            }

            series.closed = true;
            (
                series.profile_id,
                series.correct_count(),
                series.exercises.len(),
            )
        };

        let earned = self.points.for_abandoned_series(correct_count);
        self.award(series_id, profile_id, correct_count, count, earned)
    }

    fn award(
        &self,
        series_id: Uuid,
        profile_id: Uuid,
        correct_count: usize,
        count: usize,
        earned: i32,
    ) -> Result<SeriesSummary> {
        let total = self
            .profiles
            .add_points(profile_id, earned)?
            .ok_or(Error::ProfileNotFound(profile_id))?;
        let mut state = self.lock();
        state.series.remove(&series_id);
        state.order.retain(|id| *id != series_id);
        Ok(SeriesSummary {
            correct_count,
            exercise_count: count,
            points_earned: earned,
            profile_points: total,
        })
    }

    fn lock(&self) -> MutexGuard<'_, InProgress> {
        self.in_progress.lock().unwrap_or_else(|e| e.into_inner())
    }
}
