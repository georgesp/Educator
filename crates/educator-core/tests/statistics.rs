//! Statistics of the parents' space (SUB001511) on simulated sessions.

use educator_core::clock::unix_ms;
use educator_core::sessions::{AnswerRecord, SessionRecord, SessionType};
use educator_core::skills::{Date, Subject};
use educator_core::statistics::{
    date_of_day, day_number, profile_statistics, trend_of, utc_date_of, LocalCalendar,
    MasteryState, ProfileStatistics, SeriesPoint, Trend, DAYS_SHOWN, MAX_SESSION_SECONDS,
    WEEKS_SHOWN,
};
use educator_core::Grade;
use uuid::Uuid;

const PROFILE: Uuid = Uuid::from_u128(1);
const TODAY: Date = Date::new(2026, 10, 14); // a Wednesday

struct Data {
    sessions: Vec<SessionRecord>,
    answers: Vec<AnswerRecord>,
    next_id: u128,
}

impl Data {
    fn new() -> Self {
        Self {
            sessions: Vec::new(),
            answers: Vec::new(),
            next_id: 100,
        }
    }

    fn id(&mut self) -> Uuid {
        self.next_id += 1;
        Uuid::from_u128(self.next_id)
    }

    /// A closed session of `results.len()` questions on `skill`, started at `start` and lasting
    /// `seconds`; the answers follow `results` (true: right).
    #[allow(clippy::too_many_arguments)]
    fn session(
        &mut self,
        session_type: SessionType,
        skill: Option<&str>,
        answers_skill: &str,
        start: i64,
        seconds: i64,
        results: &[bool],
        points: i32,
        abandoned: bool,
    ) -> Uuid {
        let id = self.id();
        self.sessions.push(SessionRecord {
            id,
            profile_id: PROFILE,
            session_type,
            subject: Subject::Math,
            grade: Grade::CE1,
            skill_id: skill.map(str::to_owned),
            lesson_id: None,
            answer_mode: None,
            format: None,
            seed: 1,
            question_count: results.len() as u32,
            time_limit_seconds: (session_type == SessionType::Timed).then_some(60),
            started_at: start,
            ended_at: Some(start + seconds * 1000),
            abandoned,
            correct_count: results.iter().filter(|r| **r).count() as u32,
            points_earned: points,
        });
        for (index, right) in results.iter().enumerate() {
            let answer_id = self.id();
            self.answers.push(AnswerRecord {
                id: answer_id,
                session_id: id,
                index: index as u32,
                skill_id: answers_skill.to_owned(),
                prompt: format!("{answers_skill} q{}", index % 3),
                expected: "12".to_owned(),
                given: if *right { "12" } else { "13" }.to_owned(),
                is_correct: *right,
                duration_ms: Some(2000),
                answered_at: start + index as i64 * 1000,
            });
        }
        id
    }

    fn statistics(&self) -> ProfileStatistics {
        profile_statistics(
            &self.sessions,
            &self.answers,
            &LocalCalendar {
                today: TODAY,
                date_of: &utc_date_of,
            },
        )
    }
}

fn results(right: usize, total: usize) -> Vec<bool> {
    (0..total).map(|i| i < right).collect()
}

fn at(day: u32, hour: u32) -> i64 {
    unix_ms(2026, 10, day, hour, 0, 0)
}

#[test]
fn days_and_dates_round_trip() {
    for day in [-800_000, -1, 0, 1, 20_000, 20_740, 2_000_000] {
        assert_eq!(day, day_number(date_of_day(day)));
    }
    assert_eq!(Date::new(1970, 1, 1), date_of_day(0));
    assert_eq!(
        Date::new(2024, 2, 29),
        date_of_day(day_number(Date::new(2024, 2, 29)))
    );
    assert_eq!(TODAY, utc_date_of(at(14, 23)));
}

#[test]
fn a_profile_without_any_session_gets_empty_statistics() {
    let statistics = Data::new().statistics();

    assert_eq!(0, statistics.summary.session_count);
    assert_eq!(0, statistics.summary.total_seconds);
    assert_eq!(None, statistics.summary.success_rate);
    assert_eq!(None, statistics.summary.last_activity_at);
    assert_eq!(DAYS_SHOWN, statistics.days.len());
    assert_eq!(TODAY, statistics.days.last().unwrap().date);
    assert_eq!(WEEKS_SHOWN, statistics.weeks.len());
    assert_eq!(
        Date::new(2026, 10, 12),
        statistics.weeks.last().unwrap().start
    );
    assert!(statistics.weeks.iter().all(|w| w.success_rate.is_none()));
    assert!(statistics.skills.is_empty());
    assert!(statistics.to_review.is_empty());
    assert!(statistics.tests.is_empty());
    assert!(statistics.frequent_mistakes.is_empty());
}

#[test]
fn every_exercise_counts_its_series_its_points_and_their_evolution() {
    let mut data = Data::new();
    // Additions: 6 series, better and better, and one abandoned.
    let scores = [(3, 50), (4, 60), (4, 60), (8, 100), (9, 110), (9, 130)];
    for (i, (right, points)) in scores.iter().enumerate() {
        data.session(
            SessionType::Exercise,
            Some("add"),
            "add",
            at(1 + i as u32, 17),
            300,
            &results(*right, 10),
            *points,
            false,
        );
    }
    data.session(
        SessionType::Exercise,
        Some("add"),
        "add",
        at(8, 17),
        60,
        &results(2, 2),
        20,
        true,
    );
    // Subtractions: one series, mostly wrong.
    data.session(
        SessionType::Exercise,
        Some("sub"),
        "sub",
        at(13, 17),
        300,
        &results(2, 10),
        40,
        false,
    );
    // An exam on subtractions: its points are not given to the skill.
    data.session(
        SessionType::Exam,
        None,
        "sub",
        at(14, 9),
        600,
        &results(1, 5),
        30,
        false,
    );

    let statistics = data.statistics();

    let add = statistics
        .skills
        .iter()
        .find(|s| s.skill_id == "add")
        .unwrap();
    assert_eq!(6, add.completed_count);
    assert_eq!(1, add.abandoned_count);
    assert_eq!(62, add.answer_count);
    assert_eq!(50 + 60 + 60 + 100 + 110 + 130 + 20, add.points_earned);
    assert_eq!(Some(85), add.average_points);
    assert_eq!(Some(130), add.best_points);
    assert_eq!(
        vec![50, 60, 60, 100, 110, 130],
        add.history
            .iter()
            .map(|p| p.points_earned)
            .collect::<Vec<_>>()
    );
    assert_eq!(Date::new(2026, 10, 1), add.history[0].date);
    assert_eq!(Trend::Up, add.trend);
    // The last 20 answers: 2 (abandoned) + 9 + 9 right… out of 20 → 18 of 20 right.
    assert_eq!(90, add.mastery);
    assert_eq!(MasteryState::Mastered, add.state);

    let sub = statistics
        .skills
        .iter()
        .find(|s| s.skill_id == "sub")
        .unwrap();
    assert_eq!(1, sub.completed_count);
    assert_eq!(15, sub.answer_count);
    assert_eq!(40, sub.points_earned);
    assert_eq!(20, sub.mastery);
    assert_eq!(MasteryState::ToReview, sub.state);
    assert_eq!(Trend::Unknown, sub.trend);

    // The most recently practised first; the weakest to review first.
    assert_eq!("sub", statistics.skills[0].skill_id);
    assert_eq!(vec!["sub".to_owned()], statistics.to_review);

    assert_eq!(1, statistics.tests.len());
    assert_eq!(SessionType::Exam, statistics.tests[0].session_type);
    assert_eq!(30, statistics.tests[0].points_earned);
    assert_eq!(1, statistics.tests[0].correct_count);
}

#[test]
fn a_finished_placement_is_listed_with_the_exams_and_an_abandoned_one_is_ignored() {
    let mut data = Data::new();
    data.session(
        SessionType::Placement,
        None,
        "add",
        at(12, 9),
        300,
        &results(4, 6),
        0,
        false,
    );
    // Left half way: it counts for nothing.
    data.session(
        SessionType::Placement,
        None,
        "sub",
        at(13, 9),
        120,
        &results(1, 3),
        0,
        true,
    );

    let statistics = data.statistics();

    assert_eq!(1, statistics.summary.session_count);
    assert_eq!(6, statistics.summary.answer_count);
    assert_eq!(1, statistics.tests.len());
    assert_eq!(SessionType::Placement, statistics.tests[0].session_type);
    assert_eq!((4, 6), (statistics.tests[0].correct_count, statistics.tests[0].question_count));
    assert_eq!(1, statistics.skills.len());
    assert_eq!("add", statistics.skills[0].skill_id);
    assert_eq!(6, statistics.skills[0].answer_count);
    assert_eq!(67, statistics.skills[0].mastery);
}

#[test]
fn time_is_counted_by_day_and_by_week_and_long_sessions_are_capped() {
    let mut data = Data::new();
    data.session(
        SessionType::Exercise,
        Some("add"),
        "add",
        at(14, 8),
        300,
        &results(5, 5),
        70,
        false,
    );
    // Left open for two hours: counts 30 minutes.
    data.session(
        SessionType::Exercise,
        Some("add"),
        "add",
        at(13, 8),
        7200,
        &results(5, 5),
        70,
        false,
    );
    // Three weeks ago, and long ago (outside the charts, inside the totals).
    data.session(
        SessionType::Timed,
        None,
        "add",
        at(1, 8),
        60,
        &results(3, 4),
        50,
        false,
    );
    data.session(
        SessionType::Exercise,
        Some("add"),
        "add",
        unix_ms(2026, 6, 1, 8, 0, 0),
        100,
        &results(1, 5),
        30,
        false,
    );
    // Still open: ignored.
    let mut open = data.sessions[0].clone();
    open.id = Uuid::from_u128(9_999);
    open.ended_at = None;
    data.sessions.push(open);

    let statistics = data.statistics();

    let cap = MAX_SESSION_SECONDS as u32;
    assert_eq!(4, statistics.summary.session_count);
    assert_eq!(300 + cap + 60 + 100, statistics.summary.total_seconds);
    assert_eq!(70 + 70 + 50 + 30, statistics.summary.points_earned);
    assert_eq!(3, statistics.summary.active_days);
    assert_eq!(
        Some(at(14, 8) + 300_000),
        statistics.summary.last_activity_at
    );
    assert_eq!(Some(74), statistics.summary.success_rate); // 14 right of 19

    let today = statistics.days.last().unwrap();
    assert_eq!((300, 1), (today.seconds, today.session_count));
    let yesterday = &statistics.days[DAYS_SHOWN - 2];
    assert_eq!((cap, 1), (yesterday.seconds, yesterday.session_count));

    let this_week = statistics.weeks.last().unwrap();
    assert_eq!(300 + cap, this_week.seconds);
    assert_eq!(Some(100), this_week.success_rate);
    assert_eq!(140, this_week.points_earned);
    let three_weeks_ago = &statistics.weeks[WEEKS_SHOWN - 3];
    assert_eq!(Date::new(2026, 9, 28), three_weeks_ago.start);
    assert_eq!(
        (60, Some(75)),
        (three_weeks_ago.seconds, three_weeks_ago.success_rate)
    );

    assert_eq!(SessionType::Timed, statistics.tests[0].session_type);
    assert_eq!(Some(60), statistics.tests[0].time_limit_seconds);
}

#[test]
fn frequent_mistakes_come_with_the_usual_wrong_answer() {
    let mut data = Data::new();
    let id = data.session(
        SessionType::Exercise,
        Some("sub"),
        "sub",
        at(14, 8),
        300,
        &results(0, 6),
        20,
        false,
    );
    // q0 asked twice and missed twice, as « 4 » then « 4 »… q1 missed twice with different answers.
    for answer in data.answers.iter_mut().filter(|a| a.session_id == id) {
        answer.given = match answer.index {
            0 | 3 => "4".to_owned(),
            1 => "6".to_owned(),
            _ => "8".to_owned(),
        };
    }

    let mistakes = data.statistics().frequent_mistakes;

    assert_eq!(3, mistakes.len());
    assert!(mistakes
        .iter()
        .all(|m| m.mistake_count == 2 && m.attempt_count == 2));
    let q0 = mistakes.iter().find(|m| m.prompt == "sub q0").unwrap();
    assert_eq!("4", q0.usual_answer);
    assert_eq!("12", q0.expected);
    let q2 = mistakes.iter().find(|m| m.prompt == "sub q2").unwrap();
    assert_eq!("8", q2.usual_answer);
}

#[test]
fn too_few_answers_give_no_state_and_no_review_suggestion() {
    let mut data = Data::new();
    data.session(
        SessionType::Exercise,
        Some("add"),
        "add",
        at(14, 8),
        60,
        &results(0, 3),
        20,
        false,
    );

    let statistics = data.statistics();

    assert_eq!(MasteryState::TooEarly, statistics.skills[0].state);
    assert!(statistics.to_review.is_empty());
}

#[test]
fn falling_or_steady_points_give_their_trend() {
    let mut data = Data::new();
    for (day, right) in [(1, 9), (2, 9), (3, 9), (4, 5), (5, 5), (6, 4)] {
        data.session(
            SessionType::Exercise,
            Some("add"),
            "add",
            at(day, 8),
            60,
            &results(right, 10),
            10 * right as i32 + 20,
            false,
        );
    }
    for (day, right) in [(1, 6), (2, 7), (3, 7), (4, 6)] {
        data.session(
            SessionType::Exercise,
            Some("sub"),
            "sub",
            at(day, 9),
            60,
            &results(right, 10),
            10 * right as i32 + 20,
            false,
        );
    }

    let statistics = data.statistics();
    let trend = |id: &str| {
        statistics
            .skills
            .iter()
            .find(|s| s.skill_id == id)
            .unwrap()
            .trend
    };

    assert_eq!(Trend::Down, trend("add"));
    assert_eq!(Trend::Stable, trend("sub"));
    // Both under 80 %: the weakest first.
    assert_eq!(
        vec!["add".to_owned(), "sub".to_owned()],
        statistics.to_review
    );
}

#[test]
fn the_trend_follows_the_points_like_the_chart() {
    let point = |session_type, question_count, correct_count, points_earned| SeriesPoint {
        started_at: 0,
        date: TODAY,
        session_type,
        question_count,
        correct_count,
        points_earned,
    };
    // 4 series at 7 out of 10 (90 points), then 2 lesson quizzes without a mistake (90 points
    // too): the success rises, the points do not.
    let mut history = vec![point(SessionType::Exercise, 10, 7, 90); 4];
    history.extend(vec![point(SessionType::LessonQuiz, 5, 5, 90); 2]);
    assert_eq!(Trend::Stable, trend_of(&history));

    // From nothing to something: up; 9 % less: still steady; 3 series: not enough.
    let zero = vec![point(SessionType::Exercise, 10, 0, 0); 2];
    let some = vec![point(SessionType::Exercise, 10, 1, 30); 2];
    assert_eq!(Trend::Up, trend_of(&[zero.clone(), some].concat()));
    assert_eq!(
        Trend::Stable,
        trend_of(&[
            point(SessionType::Exercise, 10, 8, 100),
            point(SessionType::Exercise, 10, 8, 100),
            point(SessionType::Exercise, 10, 7, 91),
            point(SessionType::Exercise, 10, 7, 91),
        ])
    );
    assert_eq!(
        Trend::Unknown,
        trend_of(&[zero.clone(), zero[..1].to_vec()].concat())
    );
}

#[test]
fn days_and_weeks_follow_the_local_time_of_the_computer() {
    let mut data = Data::new();
    // Sunday 11 October, 23:30 UTC: already Monday 12 at 01:30 in Paris (UTC+2).
    data.session(
        SessionType::Exercise,
        Some("add"),
        "add",
        unix_ms(2026, 10, 11, 23, 30, 0),
        120,
        &results(1, 1),
        30,
        false,
    );
    // Sunday 11 October, 21:00 UTC: still Sunday 23:00 in Paris, the week before.
    data.session(
        SessionType::Exercise,
        Some("add"),
        "add",
        unix_ms(2026, 10, 11, 21, 0, 0),
        60,
        &results(0, 1),
        20,
        false,
    );
    let paris = |instant: i64| utc_date_of(instant + 2 * 3_600_000);

    let statistics = profile_statistics(
        &data.sessions,
        &data.answers,
        &LocalCalendar {
            today: TODAY,
            date_of: &paris,
        },
    );

    let day = |date: Date| statistics.days.iter().find(|d| d.date == date).unwrap();
    assert_eq!(120, day(Date::new(2026, 10, 12)).seconds);
    assert_eq!(60, day(Date::new(2026, 10, 11)).seconds);
    let this_week = statistics.weeks.last().unwrap();
    assert_eq!(Date::new(2026, 10, 12), this_week.start);
    assert_eq!(
        (120, Some(100)),
        (this_week.seconds, this_week.success_rate)
    );
    let last_week = &statistics.weeks[WEEKS_SHOWN - 2];
    assert_eq!(Date::new(2026, 10, 5), last_week.start);
    assert_eq!((60, Some(0)), (last_week.seconds, last_week.success_rate));
}
