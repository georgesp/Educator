//! SUB001509: the mistakes go to the review queue and come back on the next day, then after 3, 7
//! and 14 days; « Mes révisions » rebuilds the same calculation. The days come from a simulated
//! clock.

mod common;

use common::{test_skill, InMemoryProfileStore, InMemoryReviewStore, InMemorySessionStore};
use educator_core::clock::{unix_ms, ManualClock};
use educator_core::exercises::{Answer, Exercise, SeriesService};
use educator_core::points::PointsRules;
use educator_core::profiles::{Profile, ProfileStore};
use educator_core::random::ScriptedRandom;
use educator_core::reviews::{ReviewQueue, ReviewRules};
use educator_core::sessions::SessionType;
use educator_core::skills::{AnswerMode, SkillList, Subject};
use educator_core::statistics::utc_date_of;
use educator_core::Grade;
use std::sync::Arc;
use uuid::Uuid;

const DAY: i64 = 86_400_000;

struct Fixture {
    clock: Arc<ManualClock>,
    store: Arc<InMemoryReviewStore>,
    queue: Arc<ReviewQueue>,
    series: SeriesService,
    profile: Uuid,
}

fn fixture() -> Fixture {
    fixture_with(test_skill())
}

/// The fixture with another skill of id `test-skill` (another period).
fn fixture_with(skill: educator_core::skills::Skill) -> Fixture {
    let clock = Arc::new(ManualClock::new(unix_ms(2026, 10, 12, 8, 0, 0)));
    let profiles = Arc::new(InMemoryProfileStore::default());
    let profile = profiles
        .add(&Profile {
            id: Uuid::now_v7(),
            name: "Léa".to_owned(),
            grade: Grade::CP,
            character_id: "lion".to_owned(),
            accessories: Vec::new(),
            points: 0,
            placement_seconds: 20,
            exam_seconds: 0,
            created_at: 0,
            updated_at: 0,
            last_used_at: 0,
        })
        .unwrap()
        .id;
    let store = Arc::new(InMemoryReviewStore::default());
    let queue = Arc::new(ReviewQueue::new(
        store.clone(),
        ReviewRules::default(),
        clock.clone(),
        Arc::new(utc_date_of),
    ));
    let series = SeriesService::new(
        Arc::new(SkillList(vec![skill])),
        profiles,
        Arc::new(InMemorySessionStore::default()),
        PointsRules::DEFAULT,
        Arc::new(ScriptedRandom::new([5])),
        clock.clone(),
    )
    .with_reviews(queue.clone());
    Fixture {
        clock,
        store,
        queue,
        series,
        profile,
    }
}

fn answer(exercise: &Exercise, right: bool) -> Answer {
    match exercise.expected {
        Answer::Number(n) => Answer::Number(if right { n } else { n + 1 }),
        _ => panic!("numbers only"),
    }
}

/// A series of the skill with the first `wrong` exercises missed; returns their keys.
fn series_with_mistakes(f: &Fixture, wrong: usize) -> Vec<String> {
    let series = f
        .series
        .start(
            f.profile,
            "test-skill",
            AnswerMode::Keyboard,
            None,
            Some(7),
            1,
        )
        .unwrap();
    for (i, exercise) in series.exercises.iter().enumerate() {
        f.series
            .answer(series.id, i, &answer(exercise, i >= wrong), None)
            .unwrap();
    }
    series.exercises[..wrong]
        .iter()
        .map(|e| e.key.clone())
        .collect()
}

/// « Mes révisions » answered right or wrong; returns the keys asked.
fn review(f: &Fixture, right: bool) -> Vec<String> {
    let series = f
        .series
        .start_review(f.profile, Subject::Math, Some(3))
        .unwrap();
    assert_eq!(SessionType::Review, series.session_type);
    for (i, exercise) in series.exercises.iter().enumerate() {
        f.series
            .answer(series.id, i, &answer(exercise, right), None)
            .unwrap();
    }
    series.exercises.iter().map(|e| e.key.clone()).collect()
}

fn due(f: &Fixture) -> usize {
    f.queue.due(f.profile, &|_| true).unwrap().len()
}

#[test]
fn mistakes_come_back_the_next_day_as_the_same_calculation() {
    let f = fixture();
    let missed = series_with_mistakes(&f, 3);
    assert_eq!(3, f.store.items.lock().unwrap().len());
    assert_eq!(0, due(&f), "not the same day");
    assert!(f
        .series
        .start_review(f.profile, Subject::Math, None)
        .is_err());

    f.clock.advance(DAY);
    assert_eq!(3, due(&f));
    let mut asked = review(&f, true);
    asked.sort();
    let mut expected = missed;
    expected.sort();
    assert_eq!(expected, asked);
}

#[test]
fn right_reviews_space_out_and_leave_the_queue_after_the_last_step() {
    let f = fixture();
    series_with_mistakes(&f, 1);
    for wait in [1, 3, 7, 14] {
        f.clock.advance((wait - 1) * DAY);
        assert_eq!(0, due(&f), "the day before: not yet");
        f.clock.advance(DAY);
        assert_eq!(1, due(&f), "after {wait} day(s)");
        review(&f, true);
    }
    assert!(f.store.items.lock().unwrap().is_empty(), "learnt");
}

#[test]
fn a_review_missed_comes_back_the_next_day() {
    let f = fixture();
    series_with_mistakes(&f, 1);
    f.clock.advance(DAY);
    review(&f, true);
    f.clock.advance(3 * DAY);
    review(&f, false);
    let item = f.store.items.lock().unwrap()[0].clone();
    assert_eq!(0, item.step);
    assert_eq!(0, due(&f));
    f.clock.advance(DAY);
    assert_eq!(1, due(&f));
}

#[test]
fn the_same_mistake_twice_is_one_item() {
    let f = fixture();
    series_with_mistakes(&f, 2);
    series_with_mistakes(&f, 2);
    assert_eq!(2, f.store.items.lock().unwrap().len());
}

/// « Mes révisions » earns the usual points, even on the mistakes of a period still to come.
#[test]
fn the_reviews_earn_the_usual_points_whatever_the_period() {
    let mut later = test_skill();
    later.period = 3;
    let f = fixture_with(later);
    series_with_mistakes(&f, 3);
    f.clock.advance(DAY);

    let series = f
        .series
        .start_review(f.profile, Subject::Math, Some(3))
        .unwrap();
    assert_eq!(100, series.points_percent);
    let mut summary = None;
    for (i, exercise) in series.exercises.iter().enumerate() {
        summary = f
            .series
            .answer(series.id, i, &answer(exercise, true), None)
            .unwrap()
            .summary;
    }
    let summary = summary.unwrap();
    // 3 right answers, the bonus of a series and of a perfect one, at 100 %.
    assert_eq!((100, 70), (summary.points_percent, summary.points_earned));
}
