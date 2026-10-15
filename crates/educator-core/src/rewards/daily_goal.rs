//! The goal of the day (SUB001507): a few series a day (3 by default, chosen by the parents), a
//! bonus of points the day it is reached, and the number of days in a row it was reached. A missed
//! day only starts the count again: nothing is taken away, nothing negative is said.

use crate::sessions::{SessionRecord, SessionType};
use crate::skills::Date;
use crate::statistics::day_number;

/// Choices offered to the parents (0: no goal).
pub const GOAL_CHOICES: [u32; 7] = [0, 1, 2, 3, 4, 5, 6];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DailyGoalRules {
    /// Series a day when the parents chose nothing.
    pub default_series: u32,
    /// Points added the day the goal is reached.
    pub bonus_points: i32,
}

impl DailyGoalRules {
    pub const DEFAULT: DailyGoalRules = DailyGoalRules {
        default_series: 3,
        bonus_points: 30,
    };

    pub fn new(default_series: u32, bonus_points: i32) -> Result<Self, String> {
        if !GOAL_CHOICES.contains(&default_series) || default_series == 0 {
            return Err(format!(
                "The default goal is one of {:?} (not 0).",
                &GOAL_CHOICES[1..]
            ));
        }
        if bonus_points < 0 {
            return Err("The bonus cannot be negative.".to_owned());
        }
        Ok(Self {
            default_series,
            bonus_points,
        })
    }
}

impl Default for DailyGoalRules {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// A day when a profile reached its goal (table `DailyGoalDay`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GoalDay {
    /// Local day number (days since 1970-01-01, [`day_number`]).
    pub day: i64,
    pub reached_at: i64,
    pub bonus_points: i32,
}

/// The goal of today.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DailyGoalProgress {
    /// Series asked; 0 when the parents turned the goal off.
    pub target: u32,
    /// Series finished today.
    pub done: u32,
    pub reached: bool,
    /// Days in a row the goal was reached, ending today (or yesterday while today is not reached).
    pub streak: u32,
    pub bonus_points: i32,
}

/// Whether a session counts for the goal: anything finished (an abandoned series, a level
/// assessment do not count).
pub fn counts_for_goal(session: &SessionRecord) -> bool {
    session.ended_at.is_some()
        && !session.abandoned
        && session.session_type != SessionType::Placement
}

/// Series finished on a local day.
pub fn done_on(sessions: &[SessionRecord], day: i64, date_of: &dyn Fn(i64) -> Date) -> u32 {
    let count = sessions
        .iter()
        .filter(|s| counts_for_goal(s))
        .filter(|s| day_number(date_of(s.ended_at.unwrap_or(s.started_at))) == day)
        .count();
    u32::try_from(count).unwrap_or(u32::MAX)
}

/// Days in a row ending `today`, or yesterday when today is not in `days`.
pub fn streak(days: &[GoalDay], today: i64) -> u32 {
    let reached = |day: i64| days.iter().any(|d| d.day == day);
    let mut day = if reached(today) { today } else { today - 1 };
    let mut count = 0;
    while reached(day) {
        count += 1;
        day -= 1;
    }
    count
}

/// The longest run of days in a row (badges).
pub fn best_streak(days: &[GoalDay]) -> u32 {
    let mut sorted: Vec<i64> = days.iter().map(|d| d.day).collect();
    sorted.sort_unstable();
    sorted.dedup();
    let (mut best, mut run) = (0, 0);
    for (i, day) in sorted.iter().enumerate() {
        run = if i > 0 && sorted[i - 1] + 1 == *day {
            run + 1
        } else {
            1
        };
        best = best.max(run);
    }
    best
}

pub fn progress(
    target: u32,
    bonus_points: i32,
    sessions: &[SessionRecord],
    days: &[GoalDay],
    today: i64,
    date_of: &dyn Fn(i64) -> Date,
) -> DailyGoalProgress {
    let done = done_on(sessions, today, date_of);
    DailyGoalProgress {
        target,
        done,
        reached: days.iter().any(|d| d.day == today) || (target > 0 && done >= target),
        streak: streak(days, today),
        bonus_points,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(day: i64) -> GoalDay {
        GoalDay {
            day,
            reached_at: 0,
            bonus_points: 30,
        }
    }

    #[test]
    fn the_streak_counts_the_days_in_a_row_and_starts_again_after_a_gap() {
        let days = [day(10), day(11), day(12), day(14)];
        assert_eq!(1, streak(&days, 14));
        assert_eq!(1, streak(&days, 15));
        assert_eq!(0, streak(&days, 16));
        assert_eq!(3, streak(&days, 12));
        assert_eq!(3, streak(&days, 13));
        assert_eq!(3, best_streak(&days));
        assert_eq!(0, best_streak(&[]));
    }

    #[test]
    fn rules_are_checked() {
        assert!(DailyGoalRules::new(0, 30).is_err());
        assert!(DailyGoalRules::new(3, -1).is_err());
        assert!(DailyGoalRules::new(3, 30).is_ok());
    }
}
