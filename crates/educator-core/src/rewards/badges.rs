//! Badges (SUB001503): described in `content/rewards/badges.json` (title, condition explained to
//! the child, SVG icon, condition), checked at the end of every session. A badge earned is kept for
//! good (table `EarnedBadge`), even when the mastery that gave it goes down later.

use super::daily_goal::{best_streak, counts_for_goal, GoalDay};
use super::stars::BestStars;
use crate::sessions::{SessionRecord, SessionType, SkillMastery};
use crate::skills::{Date, SkillCatalog, SkillCategory, Subject};
use crate::statistics::day_number;
use std::collections::HashSet;

/// Answers needed on a skill before its mastery counts for a badge (a lucky first series does not).
pub const MASTERY_MIN_ANSWERS: u32 = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Badge {
    pub id: String,
    /// « Champion des compléments à 10 ».
    pub title: String,
    /// The condition said simply to the child: « Réussis presque tout aux compléments à 10. ».
    pub description: String,
    /// SVG source of the icon.
    pub icon: String,
    pub condition: BadgeCondition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BadgeCondition {
    /// One of `skills` mastered at `min_score` or more (with [`MASTERY_MIN_ANSWERS`] answers).
    SkillMastery { skills: Vec<String>, min_score: u8 },
    /// `count` skills of a family mastered at `min_score` or more.
    CategoryMastery {
        category: SkillCategory,
        count: u32,
        min_score: u8,
    },
    /// Series of exercises answered to the end (of a subject when given).
    SeriesCompleted {
        count: u32,
        subject: Option<Subject>,
    },
    /// Series of exercises without any mistake.
    PerfectSeries { count: u32 },
    /// Different days with at least one finished session.
    ActiveDays { count: u32 },
    /// Days in a row with the goal of the day reached.
    GoalStreak { days: u32 },
    /// Lessons whose quiz was answered to the end.
    LessonsFinished { count: u32 },
    /// Best stars of the skills and lessons, added up.
    Stars { count: u32 },
    /// Level reached.
    Level { level: u32 },
}

/// What a profile did, read from its records when its badges are checked.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BadgeFacts {
    pub masteries: Vec<SkillMastery>,
    pub series_completed: u32,
    pub math_series_completed: u32,
    pub spelling_series_completed: u32,
    pub perfect_series: u32,
    pub active_days: u32,
    pub best_goal_streak: u32,
    pub lessons_finished: u32,
    pub stars: u32,
    pub level: u32,
}

impl BadgeFacts {
    pub fn collect(
        sessions: &[SessionRecord],
        masteries: Vec<SkillMastery>,
        goal_days: &[GoalDay],
        stars: &BestStars,
        level: u32,
        date_of: &dyn Fn(i64) -> Date,
    ) -> Self {
        let finished_series: Vec<&SessionRecord> = sessions
            .iter()
            .filter(|s| s.session_type == SessionType::Exercise && counts_for_goal(s))
            .collect();
        let count = |n: usize| u32::try_from(n).unwrap_or(u32::MAX);
        let days: HashSet<i64> = sessions
            .iter()
            .filter(|s| counts_for_goal(s))
            .map(|s| day_number(date_of(s.ended_at.unwrap_or(s.started_at))))
            .collect();
        let lessons: HashSet<&String> = sessions
            .iter()
            .filter(|s| s.session_type == SessionType::LessonQuiz && counts_for_goal(s))
            .filter_map(|s| s.lesson_id.as_ref())
            .collect();
        Self {
            masteries,
            series_completed: count(finished_series.len()),
            math_series_completed: count(
                finished_series
                    .iter()
                    .filter(|s| s.subject == Subject::Math)
                    .count(),
            ),
            spelling_series_completed: count(
                finished_series
                    .iter()
                    .filter(|s| s.subject == Subject::Spelling)
                    .count(),
            ),
            perfect_series: count(
                finished_series
                    .iter()
                    .filter(|s| s.question_count > 0 && s.correct_count == s.question_count)
                    .count(),
            ),
            active_days: count(days.len()),
            best_goal_streak: best_streak(goal_days),
            lessons_finished: count(lessons.len()),
            stars: stars.total(),
            level,
        }
    }

    fn mastered(&self, skill_id: &str, min_score: u8) -> bool {
        self.masteries.iter().any(|m| {
            m.skill_id == skill_id && m.score >= min_score && m.attempt_count >= MASTERY_MIN_ANSWERS
        })
    }
}

impl BadgeCondition {
    pub fn is_met(&self, facts: &BadgeFacts, skills: &dyn SkillCatalog) -> bool {
        match self {
            BadgeCondition::SkillMastery {
                skills: ids,
                min_score,
            } => ids.iter().any(|id| facts.mastered(id, *min_score)),
            BadgeCondition::CategoryMastery {
                category,
                count,
                min_score,
            } => {
                let mastered = skills
                    .all()
                    .iter()
                    .filter(|s| s.category() == *category && facts.mastered(&s.id, *min_score))
                    .count();
                u32::try_from(mastered).unwrap_or(u32::MAX) >= *count
            }
            BadgeCondition::SeriesCompleted { count, subject } => {
                let done = match subject {
                    None => facts.series_completed,
                    Some(Subject::Math) => facts.math_series_completed,
                    Some(Subject::Spelling) => facts.spelling_series_completed,
                };
                done >= *count
            }
            BadgeCondition::PerfectSeries { count } => facts.perfect_series >= *count,
            BadgeCondition::ActiveDays { count } => facts.active_days >= *count,
            BadgeCondition::GoalStreak { days } => facts.best_goal_streak >= *days,
            BadgeCondition::LessonsFinished { count } => facts.lessons_finished >= *count,
            BadgeCondition::Stars { count } => facts.stars >= *count,
            BadgeCondition::Level { level } => facts.level >= *level,
        }
    }

    /// Skills a condition names (checked against the content when it is loaded).
    pub fn skill_ids(&self) -> &[String] {
        match self {
            BadgeCondition::SkillMastery { skills, .. } => skills,
            _ => &[],
        }
    }
}

/// A badge of a profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EarnedBadge {
    pub badge_id: String,
    pub earned_at: i64,
}

/// The badges whose condition is met and that are not earned yet, in the order of the content.
pub fn newly_met<'a>(
    badges: &'a [Badge],
    earned: &[EarnedBadge],
    facts: &BadgeFacts,
    skills: &dyn SkillCatalog,
) -> Vec<&'a Badge> {
    badges
        .iter()
        .filter(|b| !earned.iter().any(|e| e.badge_id == b.id))
        .filter(|b| b.condition.is_met(facts, skills))
        .collect()
}
