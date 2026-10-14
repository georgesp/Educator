//! Statistics of the parents' space (SUB001511), computed from the sessions and answers recorded
//! since lot 2 (SUB001401): time spent, every exercise done with its points and their evolution,
//! the success by week, the exams and timed challenges, the frequent mistakes and the exercises to
//! review first.
//!
//! Pure computation: the caller gives the records of one profile, the local date of today and how
//! to turn an instant into a local date (the time zone is not known here).

use crate::clock::days_from_civil;
use crate::sessions::{mastery_score, AnswerRecord, SessionRecord, SessionType, MASTERY_WINDOW};
use crate::skills::{Date, Subject};
use std::collections::HashMap;

/// Days shown by the daily time chart (today included).
pub const DAYS_SHOWN: usize = 14;

/// Weeks shown by the weekly charts (the current week included); weeks start on Monday.
pub const WEEKS_SHOWN: usize = 8;

/// A session counts at most this long: a window left open does not inflate the time spent.
pub const MAX_SESSION_SECONDS: i64 = 30 * 60;

/// Series kept in the history of an exercise (its points chart).
pub const HISTORY_LENGTH: usize = 20;

/// Under this many answers, an exercise has no state yet: too early to judge.
pub const MIN_ANSWERS_FOR_STATE: u32 = 5;

/// Mastery from which an exercise is mastered.
pub const MASTERED_FROM: u8 = 80;

/// Mastery under which an exercise is to review.
pub const TO_REVIEW_UNDER: u8 = 50;

/// Series compared by the trend: the last ones against as many before them.
pub const TREND_WINDOW: usize = 5;

/// Change of the points (in percent of the series before) that makes a trend.
pub const TREND_THRESHOLD: i64 = 10;

/// Exercises suggested to review first.
pub const TO_REVIEW_COUNT: usize = 3;

/// Frequent mistakes listed.
pub const MISTAKES_SHOWN: usize = 10;

/// Exams and timed challenges listed.
pub const TESTS_SHOWN: usize = 20;

/// How well an exercise is mastered, from the share of right answers among the last
/// [`MASTERY_WINDOW`] ones (the score shown to the child).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MasteryState {
    /// Fewer than [`MIN_ANSWERS_FOR_STATE`] answers.
    TooEarly,
    /// [`MASTERED_FROM`] or more.
    Mastered,
    InProgress,
    /// Under [`TO_REVIEW_UNDER`].
    ToReview,
}

impl MasteryState {
    pub fn of(mastery: u8, answer_count: u32) -> Self {
        if answer_count < MIN_ANSWERS_FOR_STATE {
            MasteryState::TooEarly
        } else if mastery >= MASTERED_FROM {
            MasteryState::Mastered
        } else if mastery < TO_REVIEW_UNDER {
            MasteryState::ToReview
        } else {
            MasteryState::InProgress
        }
    }
}

/// How the points of the last series of an exercise compare with the ones before.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Trend {
    /// Not enough series to compare (at least 4): see [`trend_of`].
    Unknown,
    Up,
    Stable,
    Down,
}

/// Totals of a profile.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StatisticsSummary {
    /// Every session (abandoned ones included).
    pub session_count: u32,
    pub total_seconds: u32,
    pub points_earned: i64,
    pub answer_count: u32,
    pub correct_count: u32,
    /// Share of right answers, rounded; `None` without any answer.
    pub success_rate: Option<u8>,
    /// Days with at least one session among the last [`DAYS_SHOWN`].
    pub active_days: u32,
    pub last_activity_at: Option<i64>,
}

/// Activity of one local day.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayActivity {
    pub date: Date,
    pub seconds: u32,
    pub session_count: u32,
}

/// Activity of one week (Monday to Sunday).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeekActivity {
    /// The Monday.
    pub start: Date,
    pub seconds: u32,
    pub session_count: u32,
    pub answer_count: u32,
    pub correct_count: u32,
    pub success_rate: Option<u8>,
    pub points_earned: i64,
}

/// One finished series of an exercise, in its history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeriesPoint {
    pub started_at: i64,
    pub date: Date,
    pub session_type: SessionType,
    pub question_count: u32,
    pub correct_count: u32,
    pub points_earned: i32,
}

/// Everything about one exercise (skill) practised by the profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillStatistics {
    pub skill_id: String,
    pub subject: Subject,
    /// Series answered to the end (exercises and lesson quizzes on this skill).
    pub completed_count: u32,
    /// Series stopped before the end.
    pub abandoned_count: u32,
    /// Every answer on the skill, exams and timed challenges included.
    pub answer_count: u32,
    pub correct_count: u32,
    /// Share of right answers among the last [`MASTERY_WINDOW`] answers.
    pub mastery: u8,
    pub state: MasteryState,
    /// Points of the series of this skill (exams and challenges mix skills: their points are not split).
    pub points_earned: i64,
    /// Average points of a completed series.
    pub average_points: Option<u32>,
    pub best_points: Option<i32>,
    pub trend: Trend,
    pub last_practiced_at: i64,
    /// The last [`HISTORY_LENGTH`] completed series, oldest first.
    pub history: Vec<SeriesPoint>,
}

/// An exam or a timed challenge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestResult {
    pub session_type: SessionType,
    pub subject: Subject,
    pub started_at: i64,
    pub date: Date,
    pub question_count: u32,
    pub correct_count: u32,
    pub points_earned: i32,
    pub time_limit_seconds: Option<u32>,
    pub abandoned: bool,
}

/// A question often missed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrequentMistake {
    pub skill_id: String,
    pub prompt: String,
    pub expected: String,
    /// Wrong answers to this question.
    pub mistake_count: u32,
    /// Times the question was asked.
    pub attempt_count: u32,
    /// The wrong answer given most often.
    pub usual_answer: String,
    pub last_at: i64,
}

/// Statistics of one profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileStatistics {
    pub summary: StatisticsSummary,
    /// The last [`DAYS_SHOWN`] days, oldest first, today last.
    pub days: Vec<DayActivity>,
    /// The last [`WEEKS_SHOWN`] weeks, oldest first, the current week last.
    pub weeks: Vec<WeekActivity>,
    /// Exercises practised, the most recently practised first.
    pub skills: Vec<SkillStatistics>,
    /// The skill ids to review first (at most [`TO_REVIEW_COUNT`]), the weakest first.
    pub to_review: Vec<String>,
    /// Exams and timed challenges, the most recent first.
    pub tests: Vec<TestResult>,
    pub frequent_mistakes: Vec<FrequentMistake>,
}

/// Where the statistics are computed: today's local date and the local date of an instant.
pub struct LocalCalendar<'a> {
    pub today: Date,
    pub date_of: &'a dyn Fn(i64) -> Date,
}

impl std::fmt::Debug for LocalCalendar<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalCalendar")
            .field("today", &self.today)
            .finish_non_exhaustive()
    }
}

/// Computes the statistics of a profile from its sessions and their answers (the open sessions are
/// ignored: they are closed at the next start).
pub fn profile_statistics(
    sessions: &[SessionRecord],
    answers: &[AnswerRecord],
    calendar: &LocalCalendar<'_>,
) -> ProfileStatistics {
    let mut sessions: Vec<&SessionRecord> =
        sessions.iter().filter(|s| s.ended_at.is_some()).collect();
    sessions.sort_by_key(|s| s.started_at);
    let session_ids: std::collections::HashSet<_> = sessions.iter().map(|s| s.id).collect();
    let mut answers: Vec<&AnswerRecord> = answers
        .iter()
        .filter(|a| session_ids.contains(&a.session_id))
        .collect();
    answers.sort_by_key(|a| (a.answered_at, a.index));

    let today = day_number(calendar.today);
    let day_of = |instant: i64| day_number((calendar.date_of)(instant));

    // Days and weeks.
    let first_day = today - (DAYS_SHOWN as i64 - 1);
    let mut days: Vec<DayActivity> = (first_day..=today)
        .map(|d| DayActivity {
            date: date_of_day(d),
            seconds: 0,
            session_count: 0,
        })
        .collect();
    let this_monday = monday_of(today);
    let first_monday = this_monday - 7 * (WEEKS_SHOWN as i64 - 1);
    let mut weeks: Vec<WeekActivity> = (0..WEEKS_SHOWN as i64)
        .map(|w| WeekActivity {
            start: date_of_day(first_monday + 7 * w),
            seconds: 0,
            session_count: 0,
            answer_count: 0,
            correct_count: 0,
            success_rate: None,
            points_earned: 0,
        })
        .collect();
    let week_index = |day: i64| -> Option<usize> {
        let monday = monday_of(day);
        (monday >= first_monday && monday <= this_monday)
            .then(|| ((monday - first_monday) / 7) as usize)
    };

    let mut summary = StatisticsSummary::default();
    for session in &sessions {
        let seconds = session_seconds(session);
        let day = day_of(session.started_at);
        summary.session_count += 1;
        summary.total_seconds += seconds;
        summary.points_earned += i64::from(session.points_earned);
        summary.last_activity_at = Some(
            summary
                .last_activity_at
                .unwrap_or(i64::MIN)
                .max(session.ended_at.unwrap_or(session.started_at)),
        );
        if day >= first_day && day <= today {
            let entry = &mut days[(day - first_day) as usize];
            entry.seconds += seconds;
            entry.session_count += 1;
        }
        if let Some(w) = week_index(day) {
            weeks[w].seconds += seconds;
            weeks[w].session_count += 1;
            weeks[w].points_earned += i64::from(session.points_earned);
        }
    }

    for answer in &answers {
        summary.answer_count += 1;
        summary.correct_count += u32::from(answer.is_correct);
        if let Some(w) = week_index(day_of(answer.answered_at)) {
            weeks[w].answer_count += 1;
            weeks[w].correct_count += u32::from(answer.is_correct);
        }
    }
    summary.success_rate = rate(summary.correct_count, summary.answer_count);
    summary.active_days = days.iter().filter(|d| d.session_count > 0).count() as u32;
    for week in &mut weeks {
        week.success_rate = rate(week.correct_count, week.answer_count);
    }

    let skills = skill_statistics(&sessions, &answers, &day_of);
    let to_review = to_review(&skills);
    let tests = sessions
        .iter()
        .rev()
        .filter(|s| matches!(s.session_type, SessionType::Exam | SessionType::Timed))
        .take(TESTS_SHOWN)
        .map(|s| TestResult {
            session_type: s.session_type,
            subject: s.subject,
            started_at: s.started_at,
            date: date_of_day(day_of(s.started_at)),
            question_count: s.question_count,
            correct_count: s.correct_count,
            points_earned: s.points_earned,
            time_limit_seconds: s.time_limit_seconds,
            abandoned: s.abandoned,
        })
        .collect();

    ProfileStatistics {
        summary,
        days,
        weeks,
        skills,
        to_review,
        tests,
        frequent_mistakes: frequent_mistakes(&answers),
    }
}

fn skill_statistics(
    sessions: &[&SessionRecord],
    answers: &[&AnswerRecord],
    day_of: &dyn Fn(i64) -> i64,
) -> Vec<SkillStatistics> {
    let subject_of_session: HashMap<_, _> = sessions.iter().map(|s| (s.id, s.subject)).collect();
    let mut by_skill: HashMap<&str, SkillStatistics> = HashMap::new();
    let mut results: HashMap<&str, Vec<bool>> = HashMap::new();

    for answer in answers {
        let entry = by_skill.entry(answer.skill_id.as_str()).or_insert_with(|| {
            empty_skill(&answer.skill_id, subject_of_session[&answer.session_id])
        });
        entry.answer_count += 1;
        entry.correct_count += u32::from(answer.is_correct);
        entry.last_practiced_at = entry.last_practiced_at.max(answer.answered_at);
        results
            .entry(answer.skill_id.as_str())
            .or_default()
            .push(answer.is_correct);
    }

    for session in sessions {
        if !matches!(
            session.session_type,
            SessionType::Exercise | SessionType::LessonQuiz
        ) {
            continue;
        }
        let Some(skill_id) = session.skill_id.as_deref() else {
            continue;
        };
        let entry = by_skill
            .entry(skill_id)
            .or_insert_with(|| empty_skill(skill_id, session.subject));
        entry.points_earned += i64::from(session.points_earned);
        entry.last_practiced_at = entry
            .last_practiced_at
            .max(session.ended_at.unwrap_or(session.started_at));
        if session.abandoned {
            entry.abandoned_count += 1;
        } else {
            entry.completed_count += 1;
            entry.history.push(SeriesPoint {
                started_at: session.started_at,
                date: date_of_day(day_of(session.started_at)),
                session_type: session.session_type,
                question_count: session.question_count,
                correct_count: session.correct_count,
                points_earned: session.points_earned,
            });
        }
    }

    let mut skills: Vec<SkillStatistics> = by_skill
        .into_values()
        .map(|mut skill| {
            let recent: Vec<bool> = results
                .get(skill.skill_id.as_str())
                .map(|r| r.iter().rev().take(MASTERY_WINDOW).copied().collect())
                .unwrap_or_default();
            skill.mastery = mastery_score(&recent);
            skill.state = MasteryState::of(skill.mastery, skill.answer_count);
            skill.trend = trend_of(&skill.history);
            if !skill.history.is_empty() {
                let total: i64 = skill
                    .history
                    .iter()
                    .map(|p| i64::from(p.points_earned))
                    .sum();
                let count = skill.history.len() as i64;
                skill.average_points = u32::try_from((total + count / 2) / count).ok();
                skill.best_points = skill.history.iter().map(|p| p.points_earned).max();
            }
            if skill.history.len() > HISTORY_LENGTH {
                skill.history.drain(..skill.history.len() - HISTORY_LENGTH);
            }
            skill
        })
        .collect();
    skills.sort_by(|a, b| {
        b.last_practiced_at
            .cmp(&a.last_practiced_at)
            .then_with(|| a.skill_id.cmp(&b.skill_id))
    });
    skills
}

fn empty_skill(skill_id: &str, subject: Subject) -> SkillStatistics {
    SkillStatistics {
        skill_id: skill_id.to_owned(),
        subject,
        completed_count: 0,
        abandoned_count: 0,
        answer_count: 0,
        correct_count: 0,
        mastery: 0,
        state: MasteryState::TooEarly,
        points_earned: 0,
        average_points: None,
        best_points: None,
        trend: Trend::Unknown,
        last_practiced_at: i64::MIN,
        history: Vec::new(),
    }
}

/// Compares the points of the last series (up to [`TREND_WINDOW`]) with as many series before
/// them, like the points chart of the exercise; at least 2 series on each side are needed. A change
/// of [`TREND_THRESHOLD`] percent or more makes a trend.
pub fn trend_of(history: &[SeriesPoint]) -> Trend {
    let window = (history.len() / 2).min(TREND_WINDOW);
    if window < 2 {
        return Trend::Unknown;
    }

    // Both sides have as many series: their totals compare like their averages.
    let total =
        |points: &[SeriesPoint]| -> i64 { points.iter().map(|p| i64::from(p.points_earned)).sum() };
    let last = total(&history[history.len() - window..]);
    let before = total(&history[history.len() - 2 * window..history.len() - window]);
    if last > before && last * 100 >= before * (100 + TREND_THRESHOLD) {
        Trend::Up
    } else if last < before && last * 100 <= before * (100 - TREND_THRESHOLD) {
        Trend::Down
    } else {
        Trend::Stable
    }
}

/// The exercises to review first: those with a state, under [`MASTERED_FROM`], the weakest first.
fn to_review(skills: &[SkillStatistics]) -> Vec<String> {
    let mut weak: Vec<&SkillStatistics> = skills
        .iter()
        .filter(|s| matches!(s.state, MasteryState::ToReview | MasteryState::InProgress))
        .collect();
    weak.sort_by(|a, b| {
        a.mastery
            .cmp(&b.mastery)
            .then_with(|| b.answer_count.cmp(&a.answer_count))
            .then_with(|| a.skill_id.cmp(&b.skill_id))
    });
    weak.into_iter()
        .take(TO_REVIEW_COUNT)
        .map(|s| s.skill_id.clone())
        .collect()
}

fn frequent_mistakes(answers: &[&AnswerRecord]) -> Vec<FrequentMistake> {
    struct Tally<'a> {
        attempts: u32,
        mistakes: u32,
        given: HashMap<&'a str, u32>,
        last_at: i64,
    }

    let mut tallies: HashMap<(&str, &str, &str), Tally<'_>> = HashMap::new();
    for answer in answers {
        let tally = tallies
            .entry((&answer.skill_id, &answer.prompt, &answer.expected))
            .or_insert_with(|| Tally {
                attempts: 0,
                mistakes: 0,
                given: HashMap::new(),
                last_at: i64::MIN,
            });
        tally.attempts += 1;
        if !answer.is_correct {
            tally.mistakes += 1;
            *tally.given.entry(answer.given.as_str()).or_default() += 1;
            tally.last_at = tally.last_at.max(answer.answered_at);
        }
    }

    let mut mistakes: Vec<FrequentMistake> = tallies
        .into_iter()
        .filter(|(_, t)| t.mistakes > 0)
        .map(|((skill_id, prompt, expected), t)| {
            let usual_answer = t
                .given
                .iter()
                .max_by(|a, b| a.1.cmp(b.1).then_with(|| b.0.cmp(a.0)))
                .map(|(given, _)| (*given).to_owned())
                .unwrap_or_default();
            FrequentMistake {
                skill_id: skill_id.to_owned(),
                prompt: prompt.to_owned(),
                expected: expected.to_owned(),
                mistake_count: t.mistakes,
                attempt_count: t.attempts,
                usual_answer,
                last_at: t.last_at,
            }
        })
        .collect();
    mistakes.sort_by(|a, b| {
        b.mistake_count
            .cmp(&a.mistake_count)
            .then_with(|| b.last_at.cmp(&a.last_at))
            .then_with(|| a.prompt.cmp(&b.prompt))
    });
    mistakes.truncate(MISTAKES_SHOWN);
    mistakes
}

/// Duration of a closed session, capped at [`MAX_SESSION_SECONDS`].
fn session_seconds(session: &SessionRecord) -> u32 {
    let end = session.ended_at.unwrap_or(session.started_at);
    let seconds = ((end - session.started_at) / 1000).clamp(0, MAX_SESSION_SECONDS);
    seconds as u32
}

/// Rounded share in percent; `None` when `total` is 0.
fn rate(part: u32, total: u32) -> Option<u8> {
    (total > 0)
        .then(|| ((u64::from(part) * 100 + u64::from(total) / 2) / u64::from(total)).min(100) as u8)
}

/// Days since 1970-01-01 (proleptic Gregorian calendar).
pub fn day_number(date: Date) -> i64 {
    days_from_civil(date.year, date.month, date.day)
}

/// The date of a day number (inverse of [`day_number`]).
pub fn date_of_day(day: i64) -> Date {
    let z = day + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let mp = (5 * day_of_year + 2) / 153;
    let day = (day_of_year - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    Date::new(year as i32, month, day)
}

/// The UTC date of an instant in Unix milliseconds (tests, and a fallback without time zone).
pub fn utc_date_of(instant_ms: i64) -> Date {
    date_of_day(instant_ms.div_euclid(86_400_000))
}

/// The Monday of the week of a day (1970-01-01 was a Thursday).
fn monday_of(day: i64) -> i64 {
    day - (day + 3).rem_euclid(7)
}
