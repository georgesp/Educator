//! Stars of a series and of a lesson (SUB001502): 1 star for a series answered to the end, 2 from
//! a share of right answers, 3 from a higher one (thresholds in `content/rewards/stars.json`). The
//! best result is kept by profile and by skill (series of exercises), and by lesson (its quiz): it
//! is read from the recorded sessions, so nothing more is stored.

use crate::sessions::{SessionRecord, SessionType};
use std::collections::HashMap;

pub const MAX_STARS: u8 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StarRules {
    /// Share of right answers (percent) for 2 stars.
    pub two_stars: u8,
    /// Share of right answers (percent) for 3 stars.
    pub three_stars: u8,
}

impl StarRules {
    pub const DEFAULT: StarRules = StarRules {
        two_stars: 70,
        three_stars: 90,
    };

    pub fn new(two_stars: u8, three_stars: u8) -> Result<Self, String> {
        if two_stars == 0 || two_stars >= three_stars || three_stars > 100 {
            return Err("Expected 0 < twoStars < threeStars <= 100.".to_owned());
        }
        Ok(Self {
            two_stars,
            three_stars,
        })
    }

    /// Stars of a series: 0 when it was left before the end.
    pub fn stars(&self, correct: u32, questions: u32, finished: bool) -> u8 {
        if !finished || questions == 0 {
            return 0;
        }
        let share =
            |threshold: u8| u64::from(correct) * 100 >= u64::from(threshold) * u64::from(questions);
        if share(self.three_stars) {
            3
        } else if share(self.two_stars) {
            2
        } else {
            1
        }
    }

    /// Stars of a recorded session: series of exercises and lesson quizzes only.
    pub fn of_session(&self, session: &SessionRecord) -> Option<u8> {
        matches!(
            session.session_type,
            SessionType::Exercise | SessionType::LessonQuiz
        )
        .then(|| {
            self.stars(
                session.correct_count,
                session.question_count,
                session.ended_at.is_some() && !session.abandoned,
            )
        })
    }
}

impl Default for StarRules {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// The best stars of a profile, by skill and by lesson (absent: never finished).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BestStars {
    pub skills: HashMap<String, u8>,
    pub lessons: HashMap<String, u8>,
}

impl BestStars {
    /// From the sessions of a profile.
    pub fn of(rules: &StarRules, sessions: &[SessionRecord]) -> Self {
        let mut best = BestStars::default();
        for session in sessions {
            let Some(stars) = rules.of_session(session).filter(|s| *s > 0) else {
                continue;
            };
            let (map, key) = match session.session_type {
                SessionType::Exercise => (&mut best.skills, session.skill_id.as_ref()),
                _ => (&mut best.lessons, session.lesson_id.as_ref()),
            };
            if let Some(key) = key {
                let entry = map.entry(key.clone()).or_insert(0);
                *entry = (*entry).max(stars);
            }
        }
        best
    }

    pub fn skill(&self, id: &str) -> u8 {
        self.skills.get(id).copied().unwrap_or(0)
    }

    pub fn lesson(&self, id: &str) -> u8 {
        self.lessons.get(id).copied().unwrap_or(0)
    }

    /// Every best star of the skills and the lessons.
    pub fn total(&self) -> u32 {
        self.skills
            .values()
            .chain(self.lessons.values())
            .map(|s| u32::from(*s))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stars_follow_the_thresholds() {
        let rules = StarRules::DEFAULT;
        assert_eq!(0, rules.stars(10, 10, false));
        assert_eq!(1, rules.stars(0, 10, true));
        assert_eq!(1, rules.stars(6, 10, true));
        assert_eq!(2, rules.stars(7, 10, true));
        assert_eq!(2, rules.stars(8, 10, true));
        assert_eq!(3, rules.stars(9, 10, true));
        assert_eq!(3, rules.stars(10, 10, true));
        assert_eq!(0, rules.stars(0, 0, true));
    }

    #[test]
    fn bad_thresholds_are_refused() {
        assert!(StarRules::new(90, 70).is_err());
        assert!(StarRules::new(0, 70).is_err());
        assert!(StarRules::new(70, 101).is_err());
        assert!(StarRules::new(70, 90).is_ok());
    }
}
