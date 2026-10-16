//! Points scale (SUB000110), read from `content/rewards/points.json`. Points are never taken away.
//! A series of exercises or a lesson quiz ahead of the school year earns more, a past period
//! already mastered earns less ([`Pace`]): the percents are in the same file.

use crate::adaptive::AdaptiveRules;
use crate::lessons::Lesson;
use crate::sessions::SkillMastery;
use crate::skills::OfferedSkill;
use crate::Grade;

/// The usual points: 100 %.
pub const USUAL_PERCENT: u32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointsRules {
    /// Points per correct answer.
    pub per_correct_answer: i32,
    /// Bonus for a series answered to the end, whatever the score (rewards effort).
    pub series_completed_bonus: i32,
    /// Extra bonus for a series without any mistake.
    pub perfect_series_bonus: i32,
    /// Percent of the points on a past period already mastered ([`Pace::Revision`]).
    pub past_period_percent: u32,
    /// Percent of the points on a period still to come ([`Pace::Ahead`]).
    pub next_period_percent: u32,
}

impl PointsRules {
    pub const DEFAULT: PointsRules = PointsRules::new(10, 20, 20);

    /// The scale with the default percents of the periods (50 % and 150 %).
    pub const fn new(
        per_correct_answer: i32,
        series_completed_bonus: i32,
        perfect_series_bonus: i32,
    ) -> Self {
        Self {
            per_correct_answer,
            series_completed_bonus,
            perfect_series_bonus,
            past_period_percent: 50,
            next_period_percent: 150,
        }
    }

    /// The same scale with other percents for the past and the next periods.
    pub const fn with_period_percents(
        mut self,
        past_period_percent: u32,
        next_period_percent: u32,
    ) -> Self {
        self.past_period_percent = past_period_percent;
        self.next_period_percent = next_period_percent;
        self
    }

    /// Points of a series answered to the end.
    pub fn for_completed_series(&self, correct_answers: usize, exercises: usize) -> i32 {
        assert!(
            correct_answers <= exercises,
            "more correct answers than exercises"
        );
        to_i32(correct_answers) * self.per_correct_answer
            + self.series_completed_bonus
            + if correct_answers == exercises {
                self.perfect_series_bonus
            } else {
                0
            }
    }

    /// Points of a timed challenge (SUB001415): the right answers, and the bonus of a finished
    /// series when at least one question was answered; the perfect bonus needs no mistake. A wrong
    /// answer costs nothing.
    pub fn for_timed_challenge(&self, correct_answers: usize, answered: usize) -> i32 {
        if answered == 0 {
            return 0;
        }

        self.for_completed_series(correct_answers.min(answered), answered)
    }

    /// Points of an abandoned series: the correct answers already given, without any bonus.
    pub fn for_abandoned_series(&self, correct_answers: usize) -> i32 {
        to_i32(correct_answers) * self.per_correct_answer
    }

    /// Percent of the points for a pace.
    pub fn percent(&self, pace: Pace) -> u32 {
        match pace {
            Pace::Current => USUAL_PERCENT,
            Pace::Revision => self.past_period_percent,
            Pace::Ahead => self.next_period_percent,
        }
    }
}

impl Default for PointsRules {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// `points` at `percent`, rounded to the nearest point.
pub fn apply_percent(points: i32, percent: u32) -> i32 {
    let scaled = (i64::from(points) * i64::from(percent) + 50).div_euclid(100);
    i32::try_from(scaled).unwrap_or(if scaled < 0 { i32::MIN } else { i32::MAX })
}

/// Where a skill or a lesson stands against the school year, for its points.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pace {
    /// The current period, a revision of the grade before, another grade, or a past period not
    /// mastered yet: the usual points.
    Current,
    /// A past period already mastered: the points of a revision.
    Revision,
    /// A period still to come: a bonus.
    Ahead,
}

/// The pace of what is offered to the profile's grade at `period` (`None`: not one of the
/// grade's own skills or lessons, a revision of the grade before or another grade). `mastered`:
/// all its skills are mastered.
pub fn pace(period: Option<u8>, current_period: u8, mastered: bool) -> Pace {
    match period {
        Some(period) if period > current_period => Pace::Ahead,
        Some(period) if period < current_period && mastered => Pace::Revision,
        _ => Pace::Current,
    }
}

/// Whether every skill of `skill_ids` is mastered, as « Je te conseille… » sees it (enough answers,
/// a score of [`AdaptiveRules::raise_above`] or more). No skill: not mastered.
pub fn all_mastered(
    skill_ids: &[String],
    masteries: &[SkillMastery],
    rules: &AdaptiveRules,
) -> bool {
    !skill_ids.is_empty()
        && skill_ids.iter().all(|id| {
            masteries.iter().any(|m| {
                m.skill_id == *id
                    && m.attempt_count >= rules.min_answers
                    && m.score >= rules.raise_above
            })
        })
}

/// What the pace of a skill or a lesson depends on, for one profile today.
#[derive(Debug, Clone, Copy)]
pub struct PaceContext<'a> {
    /// The profile's grade: only its own skills and lessons follow the periods.
    pub grade: Grade,
    pub current_period: u8,
    /// The masteries of the profile.
    pub masteries: &'a [SkillMastery],
    /// When a skill is mastered (`content/progress/adaptive.json`).
    pub rules: &'a AdaptiveRules,
}

impl PaceContext<'_> {
    /// The pace of a skill among `offered`, what is offered to `shown` (the grade returned by
    /// [`crate::skills::SkillCatalog::offered_or_fallback`]). The CE1 skills shown to a grade
    /// without skills yet are another grade's: the usual points.
    pub fn skill(&self, shown: Grade, offered: &[OfferedSkill<'_>], skill_id: &str) -> Pace {
        let period = offered
            .iter()
            .find(|o| o.skill.id == skill_id && o.revision_from.is_none())
            .filter(|_| shown == self.grade)
            .map(|o| o.period);
        pace(
            period,
            self.current_period,
            all_mastered(&[skill_id.to_owned()], self.masteries, self.rules),
        )
    }

    /// The pace of a lesson: only the lessons of the profile's grade follow the periods (the CE1
    /// lessons shown to a grade without lessons yet earn the usual points).
    pub fn lesson(&self, lesson: &Lesson) -> Pace {
        let period = (lesson.grade == self.grade).then_some(lesson.period);
        pace(
            period,
            self.current_period,
            all_mastered(&lesson.skill_ids, self.masteries, self.rules),
        )
    }
}

fn to_i32(value: usize) -> i32 {
    i32::try_from(value).unwrap_or(i32::MAX)
}
