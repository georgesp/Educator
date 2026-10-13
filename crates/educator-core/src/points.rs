//! Points scale (SUB000110), read from `content/rewards/points.json`. Points are never taken away.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointsRules {
    /// Points per correct answer.
    pub per_correct_answer: i32,
    /// Bonus for a series answered to the end, whatever the score (rewards effort).
    pub series_completed_bonus: i32,
    /// Extra bonus for a series without any mistake.
    pub perfect_series_bonus: i32,
}

impl PointsRules {
    pub const DEFAULT: PointsRules = PointsRules::new(10, 20, 20);

    pub const fn new(
        per_correct_answer: i32,
        series_completed_bonus: i32,
        perfect_series_bonus: i32,
    ) -> Self {
        Self {
            per_correct_answer,
            series_completed_bonus,
            perfect_series_bonus,
        }
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
}

impl Default for PointsRules {
    fn default() -> Self {
        Self::DEFAULT
    }
}

fn to_i32(value: usize) -> i32 {
    i32::try_from(value).unwrap_or(i32::MAX)
}
