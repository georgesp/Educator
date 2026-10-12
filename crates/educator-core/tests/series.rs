mod common;

use common::{skill, test_skill, InMemoryProfileStore};
use educator_core::clock::{unix_ms, ManualClock};
use educator_core::exercises::{Answer, Exercise, SeriesService, SeriesSummary};
use educator_core::points::PointsRules;
use educator_core::profiles::{Profile, ProfileStore};
use educator_core::random::ScriptedRandom;
use educator_core::skills::{
    AnswerMode, Placement, SkillCatalog, SkillConstraints, SkillList, Subject,
};
use educator_core::{Error, Grade};
use std::sync::Arc;
use uuid::Uuid;

struct Fixture {
    profiles: Arc<InMemoryProfileStore>,
    service: SeriesService,
    profile: Profile,
}

fn fixture(extra: Vec<educator_core::skills::Skill>) -> Fixture {
    let profiles = Arc::new(InMemoryProfileStore::default());
    let now = unix_ms(2026, 10, 11, 8, 0, 0);
    let profile = profiles
        .add(&Profile {
            id: Uuid::now_v7(),
            name: "Léa".to_owned(),
            grade: Grade::CP,
            character_id: "lion".to_owned(),
            points: 0,
            created_at: now,
            updated_at: now,
            last_used_at: now,
        })
        .unwrap();
    let mut main = test_skill();
    main.constraints = SkillConstraints {
        max_result: Some(10),
        ..Default::default()
    };
    let mut skills = vec![main];
    skills.extend(extra);
    let service = SeriesService::new(
        Arc::new(SkillList(skills)),
        profiles.clone(),
        PointsRules::DEFAULT,
        Arc::new(ScriptedRandom::new([5])),
        Arc::new(ManualClock::new(now)),
    );
    Fixture {
        profiles,
        service,
        profile,
    }
}

/// An answer that is surely wrong (the expected number plus one).
fn wrong_answer(exercise: &Exercise) -> Answer {
    match exercise.expected {
        Answer::Number(n) => Answer::Number(n + 1),
        _ => panic!("number exercises only"),
    }
}

fn points(fixture: &Fixture) -> i32 {
    fixture
        .profiles
        .get(fixture.profile.id)
        .unwrap()
        .unwrap()
        .points
}

#[test]
fn perfect_series_earns_answers_and_both_bonuses() {
    let f = fixture(vec![]);
    let series = f
        .service
        .start(f.profile.id, "test-skill", AnswerMode::Keyboard, Some(7))
        .unwrap();

    let mut last = None;
    for exercise in &series.exercises {
        let outcome = f
            .service
            .answer(series.id, exercise.index, &exercise.expected)
            .unwrap();
        assert!(outcome.correct);
        assert_eq!(exercise.index == 9, outcome.summary.is_some());
        last = outcome.summary;
    }

    assert_eq!(
        Some(SeriesSummary {
            correct_count: 10,
            exercise_count: 10,
            points_earned: 140,
            profile_points: 140
        }),
        last
    );
    assert_eq!(140, points(&f));
}

#[test]
fn mistakes_are_corrected_without_losing_points() {
    let f = fixture(vec![]);
    let series = f
        .service
        .start(f.profile.id, "test-skill", AnswerMode::Choice, Some(7))
        .unwrap();

    let wrong = f
        .service
        .answer(series.id, 0, &wrong_answer(&series.exercises[0]))
        .unwrap();
    for exercise in series.exercises.iter().skip(1) {
        f.service
            .answer(series.id, exercise.index, &exercise.expected)
            .unwrap();
    }

    assert!(!wrong.correct);
    assert_eq!(series.exercises[0].expected, wrong.expected);
    assert_eq!(series.exercises[0].explanation, wrong.explanation);
    assert_eq!(90 + 20, points(&f));
}

#[test]
fn abandoned_series_keeps_correct_answers_without_bonus() {
    let f = fixture(vec![]);
    let series = f
        .service
        .start(f.profile.id, "test-skill", AnswerMode::Keyboard, Some(3))
        .unwrap();
    f.service
        .answer(series.id, 0, &series.exercises[0].expected)
        .unwrap();
    f.service
        .answer(series.id, 1, &series.exercises[1].expected)
        .unwrap();
    f.service.answer(series.id, 2, &Answer::Number(-1)).unwrap();

    let summary = f.service.abandon(series.id).unwrap();

    assert_eq!(
        SeriesSummary {
            correct_count: 2,
            exercise_count: 10,
            points_earned: 20,
            profile_points: 20
        },
        summary
    );
    assert!(matches!(
        f.service.answer(series.id, 3, &Answer::Number(0)),
        Err(Error::SeriesNotFound(_))
    ));
}

#[test]
fn same_exercise_cannot_be_answered_twice() {
    let f = fixture(vec![]);
    let series = f
        .service
        .start(f.profile.id, "test-skill", AnswerMode::Keyboard, None)
        .unwrap();
    f.service.answer(series.id, 0, &Answer::Number(1)).unwrap();

    assert!(matches!(
        f.service.answer(series.id, 0, &Answer::Number(1)),
        Err(Error::SeriesRequest(_))
    ));
    assert!(matches!(
        f.service.answer(series.id, 10, &Answer::Number(1)),
        Err(Error::SeriesRequest(_))
    ));
}

#[test]
fn start_checks_profile_skill_and_mode() {
    let mut keyboard_only = skill("keyboard-only");
    keyboard_only.answer_modes = vec![AnswerMode::Keyboard];
    let f = fixture(vec![keyboard_only]);

    assert!(matches!(
        f.service
            .start(Uuid::now_v7(), "test-skill", AnswerMode::Keyboard, None),
        Err(Error::ProfileNotFound(_))
    ));
    assert!(matches!(
        f.service
            .start(f.profile.id, "unknown", AnswerMode::Keyboard, None),
        Err(Error::SeriesRequest(_))
    ));
    assert!(matches!(
        f.service
            .start(f.profile.id, "keyboard-only", AnswerMode::Choice, None),
        Err(Error::SeriesRequest(_))
    ));
}

#[test]
fn same_seed_replays_the_same_series() {
    let f = fixture(vec![]);

    let first = f
        .service
        .start(f.profile.id, "test-skill", AnswerMode::Choice, Some(99))
        .unwrap();
    let second = f
        .service
        .start(f.profile.id, "test-skill", AnswerMode::Choice, Some(99))
        .unwrap();

    assert_ne!(first.id, second.id);
    assert_eq!(
        first
            .exercises
            .iter()
            .map(|e| e.prompt())
            .collect::<Vec<_>>(),
        second
            .exercises
            .iter()
            .map(|e| e.prompt())
            .collect::<Vec<_>>()
    );
}

#[test]
fn catalog_falls_back_to_ce1_for_grades_without_skills() {
    let mut ce1 = skill("ce1-skill");
    ce1.grade = Grade::CE1;
    let catalog = SkillList(vec![test_skill(), ce1]);

    assert_eq!(
        Grade::CP,
        catalog.for_or_fallback(Subject::Math, Grade::CP).0
    );
    let (grade, skills) = catalog.for_or_fallback(Subject::Math, Grade::CM2);
    assert_eq!(Grade::CE1, grade);
    assert_eq!(
        vec!["ce1-skill"],
        skills.iter().map(|s| s.id.as_str()).collect::<Vec<_>>()
    );
}

// Revisions between grades (SUB000113).

fn placed(id: &str, grade: Grade, period: u8) -> educator_core::skills::Skill {
    let mut s = skill(id);
    s.grade = grade;
    s.period = period;
    s
}

fn describe_offer(offered: &[educator_core::skills::OfferedSkill<'_>]) -> Vec<String> {
    offered
        .iter()
        .map(|o| match o.revision_from {
            Some(from) => format!("{}@{} ({from})", o.skill.id, o.period),
            None => format!("{}@{}", o.skill.id, o.period),
        })
        .collect()
}

#[test]
fn last_period_skills_are_revised_in_the_first_period_of_the_next_grade() {
    let catalog = SkillList(vec![
        placed("cp-p1", Grade::CP, 1),
        placed("cp-p5-b", Grade::CP, 5),
        placed("cp-p5-a", Grade::CP, 5),
        placed("ce1-p1", Grade::CE1, 1),
        placed("ce1-p2", Grade::CE1, 2),
        placed("ce1-p5", Grade::CE1, 5),
    ]);

    assert_eq!(
        vec![
            "ce1-p1@1",
            "cp-p5-a@1 (CP)",
            "cp-p5-b@1 (CP)",
            "ce1-p2@2",
            "ce1-p5@5"
        ],
        describe_offer(&catalog.offered_to(Subject::Math, Grade::CE1))
    );
    // CP has no grade before it: no revision.
    assert!(catalog
        .offered_to(Subject::Math, Grade::CP)
        .iter()
        .all(|o| o.revision_from.is_none()));
}

#[test]
fn also_offered_adds_a_skill_to_another_grade_once() {
    let mut extra = placed("cp-p3", Grade::CP, 3);
    extra.also_offered = vec![Placement::new(Grade::CE1, 2)];
    // Declared and covered by the default rule: offered once, where it is declared.
    let mut both = placed("cp-p5", Grade::CP, 5);
    both.also_offered = vec![Placement::new(Grade::CE1, 1)];
    let mut far = placed("cp-p4", Grade::CP, 4);
    far.also_offered = vec![Placement::new(Grade::CE2, 3)];
    let catalog = SkillList(vec![
        placed("ce1-p1", Grade::CE1, 1),
        placed("ce1-p2", Grade::CE1, 2),
        extra,
        both,
        far,
    ]);

    assert_eq!(
        vec!["ce1-p1@1", "cp-p5@1 (CP)", "ce1-p2@2", "cp-p3@2 (CP)"],
        describe_offer(&catalog.offered_to(Subject::Math, Grade::CE1))
    );
}

#[test]
fn grades_without_skills_get_what_is_offered_to_ce1_revisions_included() {
    let mut cp_for_ce2 = placed("cp-p4", Grade::CP, 4);
    cp_for_ce2.also_offered = vec![Placement::new(Grade::CE2, 1)];
    let catalog = SkillList(vec![
        placed("cp-p5", Grade::CP, 5),
        placed("ce1-p1", Grade::CE1, 1),
        placed("ce1-p5", Grade::CE1, 5),
        cp_for_ce2,
    ]);

    // CE2 has no skill of its own yet: it gets the CE1 offer, consistent with for_or_fallback.
    let (grade, offered) = catalog.offered_or_fallback(Subject::Math, Grade::CE2);
    assert_eq!(Grade::CE1, grade);
    assert_eq!(
        vec!["ce1-p1@1", "cp-p5@1 (CP)", "ce1-p5@5"],
        describe_offer(&offered)
    );
    let (grade, _) = catalog.offered_or_fallback(Subject::Math, Grade::CE1);
    assert_eq!(Grade::CE1, grade);
}
