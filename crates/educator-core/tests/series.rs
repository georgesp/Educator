mod common;

use common::{skill, test_skill, InMemoryProfileStore, InMemorySessionStore};
use educator_core::clock::{unix_ms, ManualClock};
use educator_core::exercises::{Answer, Exercise, SeriesService};
use educator_core::points::PointsRules;
use educator_core::profiles::{Profile, ProfileStore};
use educator_core::random::ScriptedRandom;
use educator_core::sessions::{SessionStore, SessionType};
use educator_core::skills::{
    AnswerMode, Placement, SkillCatalog, SkillConstraints, SkillList, Subject,
};
use educator_core::{Error, Grade};
use std::sync::Arc;
use uuid::Uuid;

struct Fixture {
    profiles: Arc<InMemoryProfileStore>,
    sessions: Arc<InMemorySessionStore>,
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
    let sessions = Arc::new(InMemorySessionStore::default());
    let service = SeriesService::new(
        Arc::new(SkillList(skills)),
        profiles.clone(),
        sessions.clone(),
        PointsRules::DEFAULT,
        Arc::new(ScriptedRandom::new([5])),
        Arc::new(ManualClock::new(now)),
    );
    Fixture {
        profiles,
        sessions,
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
        .start(
            f.profile.id,
            "test-skill",
            AnswerMode::Keyboard,
            None,
            Some(7),
        )
        .unwrap();

    let mut last = None;
    for exercise in &series.exercises {
        let outcome = f
            .service
            .answer(series.id, exercise.index, &exercise.expected, None)
            .unwrap();
        assert!(outcome.correct);
        assert_eq!(exercise.index == 9, outcome.summary.is_some());
        last = outcome.summary;
    }

    let last = last.unwrap();
    assert_eq!(
        (10, 10, 140, 140),
        (
            last.correct_count,
            last.exercise_count,
            last.points_earned,
            last.profile_points
        )
    );
    assert_eq!(vec![Some(true); 10], last.results);
    assert_eq!(vec!["test-skill".to_owned(); 10], last.skill_ids);
    assert_eq!(140, points(&f));

    // Recorded: the session, closed, and one answer per exercise; the mastery of the skill.
    let session = f.sessions.session(series.id).unwrap().unwrap();
    assert_eq!(SessionType::Exercise, session.session_type);
    assert_eq!(Some("test-skill"), session.skill_id.as_deref());
    assert_eq!(Some(7), Some(session.seed));
    assert_eq!(
        (10, 140, false),
        (
            session.correct_count,
            session.points_earned,
            session.abandoned
        )
    );
    assert!(session.ended_at.is_some());
    assert_eq!(10, f.sessions.answers(series.id).unwrap().len());
    let mastery = f.sessions.masteries(f.profile.id).unwrap();
    assert_eq!(1, mastery.len());
    assert_eq!((100, 10), (mastery[0].score, mastery[0].attempt_count));
}

#[test]
fn mistakes_are_corrected_without_losing_points() {
    let f = fixture(vec![]);
    let series = f
        .service
        .start(
            f.profile.id,
            "test-skill",
            AnswerMode::Choice,
            None,
            Some(7),
        )
        .unwrap();

    let wrong = f
        .service
        .answer(series.id, 0, &wrong_answer(&series.exercises[0]), None)
        .unwrap();
    for exercise in series.exercises.iter().skip(1) {
        f.service
            .answer(series.id, exercise.index, &exercise.expected, None)
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
        .start(
            f.profile.id,
            "test-skill",
            AnswerMode::Keyboard,
            None,
            Some(3),
        )
        .unwrap();
    f.service
        .answer(series.id, 0, &series.exercises[0].expected, None)
        .unwrap();
    f.service
        .answer(series.id, 1, &series.exercises[1].expected, None)
        .unwrap();
    f.service
        .answer(series.id, 2, &Answer::Number(-1), None)
        .unwrap();

    let summary = f.service.abandon(series.id).unwrap();

    assert_eq!(
        (2, 10, 20, 20),
        (
            summary.correct_count,
            summary.exercise_count,
            summary.points_earned,
            summary.profile_points
        )
    );
    // Recorded as abandoned, with the answers already given; mastery on those 3 answers.
    let session = f.sessions.session(series.id).unwrap().unwrap();
    assert!(session.abandoned);
    assert_eq!(2, session.correct_count);
    let answers = f.sessions.answers(series.id).unwrap();
    assert_eq!(
        vec![true, true, false],
        answers.iter().map(|a| a.is_correct).collect::<Vec<_>>()
    );
    assert_eq!("-1", answers[2].given);
    assert_eq!(67, f.sessions.masteries(f.profile.id).unwrap()[0].score);
    assert!(matches!(
        f.service.answer(series.id, 3, &Answer::Number(0), None),
        Err(Error::SeriesNotFound(_))
    ));
}

#[test]
fn same_exercise_cannot_be_answered_twice() {
    let f = fixture(vec![]);
    let series = f
        .service
        .start(f.profile.id, "test-skill", AnswerMode::Keyboard, None, None)
        .unwrap();
    f.service
        .answer(series.id, 0, &Answer::Number(1), None)
        .unwrap();

    assert!(matches!(
        f.service.answer(series.id, 0, &Answer::Number(1), None),
        Err(Error::SeriesRequest(_))
    ));
    assert!(matches!(
        f.service.answer(series.id, 10, &Answer::Number(1), None),
        Err(Error::SeriesRequest(_))
    ));
}

#[test]
fn start_checks_profile_skill_and_mode() {
    let mut keyboard_only = skill("keyboard-only");
    keyboard_only.answer_modes = vec![AnswerMode::Keyboard];
    let f = fixture(vec![keyboard_only]);

    assert!(matches!(
        f.service.start(
            Uuid::now_v7(),
            "test-skill",
            AnswerMode::Keyboard,
            None,
            None
        ),
        Err(Error::ProfileNotFound(_))
    ));
    assert!(matches!(
        f.service
            .start(f.profile.id, "unknown", AnswerMode::Keyboard, None, None),
        Err(Error::SeriesRequest(_))
    ));
    assert!(matches!(
        f.service.start(
            f.profile.id,
            "keyboard-only",
            AnswerMode::Choice,
            None,
            None
        ),
        Err(Error::SeriesRequest(_))
    ));
}

#[test]
fn same_seed_replays_the_same_series() {
    let f = fixture(vec![]);

    let first = f
        .service
        .start(
            f.profile.id,
            "test-skill",
            AnswerMode::Choice,
            None,
            Some(99),
        )
        .unwrap();
    let second = f
        .service
        .start(
            f.profile.id,
            "test-skill",
            AnswerMode::Choice,
            None,
            Some(99),
        )
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

// Exams and timed challenges (SUB001414, SUB001415).

fn fixture_with_clock(extra: Vec<educator_core::skills::Skill>) -> (Fixture, Arc<ManualClock>) {
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
    let clock = Arc::new(ManualClock::new(now));
    let sessions = Arc::new(InMemorySessionStore::default());
    let mut skills = vec![test_skill()];
    skills.extend(extra);
    let service = SeriesService::new(
        Arc::new(SkillList(skills)),
        profiles.clone(),
        sessions.clone(),
        PointsRules::DEFAULT,
        Arc::new(ScriptedRandom::new([5])),
        clock.clone(),
    );
    (
        Fixture {
            profiles,
            sessions,
            service,
            profile,
        },
        clock,
    )
}

#[test]
fn an_exam_mixes_the_skills_reached_and_is_recorded() {
    let mut later = skill("later");
    later.period = 4;
    let mut second = skill("second");
    second.right = educator_core::skills::OperandRule::range(0, 5);
    let (f, _) = fixture_with_clock(vec![second, later]);

    let exam = f
        .service
        .start_exam(f.profile.id, Subject::Math, 1, Some(3))
        .unwrap();

    assert_eq!(SessionType::Exam, exam.session_type);
    assert_eq!(None, exam.skill_id);
    // Two skills of period 1 share the 15 questions; never the period 4 one.
    assert_eq!(15, exam.exercises.len());
    assert!(exam.skill_ids.iter().all(|s| s != "later"));
    let mut summary = None;
    for e in &exam.exercises {
        summary = f
            .service
            .answer(exam.id, e.index, &e.expected, Some(900))
            .unwrap()
            .summary;
    }
    let summary = summary.unwrap();
    let by_skill = summary.by_skill();
    assert_eq!(2, by_skill.len());
    assert!(by_skill.iter().all(|r| r.is_strong()));
    let session = f.sessions.session(exam.id).unwrap().unwrap();
    assert_eq!(
        (SessionType::Exam, 15),
        (session.session_type, session.correct_count)
    );
    assert_eq!(2, f.sessions.masteries(f.profile.id).unwrap().len());
}

#[test]
fn a_timed_challenge_needs_mastered_skills_and_ends_with_the_time() {
    let (f, clock) = fixture_with_clock(vec![]);

    // Nothing practised yet: no challenge.
    assert!(matches!(
        f.service
            .start_timed(f.profile.id, Subject::Math, 120, Some(1)),
        Err(Error::SeriesRequest(_))
    ));
    assert!(matches!(
        f.service
            .start_timed(f.profile.id, Subject::Math, 100, Some(1)),
        Err(Error::SeriesRequest(_))
    ));

    // A series all right: the skill is mastered.
    let series = f
        .service
        .start(
            f.profile.id,
            "test-skill",
            AnswerMode::Keyboard,
            None,
            Some(7),
        )
        .unwrap();
    for e in &series.exercises {
        f.service
            .answer(series.id, e.index, &e.expected, None)
            .unwrap();
    }
    assert_eq!(
        vec!["test-skill".to_owned()],
        f.service.timed_skills(f.profile.id, Subject::Math).unwrap()
    );
    let before = points(&f);

    let timed = f
        .service
        .start_timed(f.profile.id, Subject::Math, 60, Some(2))
        .unwrap();
    assert_eq!(Some(60), timed.time_limit_seconds);
    assert!(timed.exercises.len() >= 20);
    // A wrong answer costs nothing and goes on.
    let wrong = f
        .service
        .answer(timed.id, 0, &wrong_answer(&timed.exercises[0]), None)
        .unwrap();
    assert!(!wrong.correct && wrong.summary.is_none());
    for e in &timed.exercises[1..4] {
        f.service
            .answer(timed.id, e.index, &e.expected, None)
            .unwrap();
    }
    // After the time (and its grace), answers are refused; the end counts the questions answered.
    clock.advance(66_000);
    assert!(matches!(
        f.service
            .answer(timed.id, 4, &timed.exercises[4].expected, None),
        Err(Error::SeriesRequest(_))
    ));
    let summary = f.service.finish(timed.id).unwrap();
    assert_eq!((3, 4), (summary.correct_count, summary.exercise_count));
    assert_eq!(30 + 20, summary.points_earned);
    assert_eq!(before + 50, points(&f));
    assert_eq!(4, summary.results.len());
    assert_eq!(
        Some(3),
        f.sessions
            .best_timed_score(f.profile.id, Subject::Math, 60)
            .unwrap()
    );
    // The first challenge at this duration is a record; the session mixes the answer modes.
    assert_eq!((None, true), (summary.previous_best, summary.new_record));
    assert!(!summary.review_lesson);
    assert_eq!(
        None,
        f.sessions.session(timed.id).unwrap().unwrap().answer_mode
    );

    // A second one with fewer right answers: no record, the best score is given.
    let again = f
        .service
        .start_timed(f.profile.id, Subject::Math, 60, Some(3))
        .unwrap();
    f.service
        .answer(again.id, 0, &again.exercises[0].expected, None)
        .unwrap();
    let summary = f.service.finish(again.id).unwrap();
    assert_eq!(
        (Some(3), false),
        (summary.previous_best, summary.new_record)
    );

    // An abandoned challenge counts only the questions answered.
    let left = f
        .service
        .start_timed(f.profile.id, Subject::Math, 60, Some(4))
        .unwrap();
    f.service
        .answer(left.id, 0, &left.exercises[0].expected, None)
        .unwrap();
    let summary = f.service.abandon(left.id).unwrap();
    assert_eq!(1, summary.exercise_count);
    let session = f.sessions.session(left.id).unwrap().unwrap();
    assert_eq!((true, 1), (session.abandoned, session.question_count));
}

#[test]
fn only_a_timed_challenge_ends_before_its_last_question() {
    let f = fixture(vec![]);
    let series = f
        .service
        .start(
            f.profile.id,
            "test-skill",
            AnswerMode::Keyboard,
            None,
            Some(7),
        )
        .unwrap();
    f.service
        .answer(series.id, 0, &series.exercises[0].expected, None)
        .unwrap();

    assert!(matches!(
        f.service.finish(series.id),
        Err(Error::SeriesRequest(_))
    ));
    // Still in progress: the child goes on.
    assert!(f
        .service
        .answer(series.id, 1, &series.exercises[1].expected, None)
        .is_ok());
}

#[test]
fn a_series_pushed_out_of_the_memory_is_recorded_as_abandoned() {
    use educator_core::exercises::SERIES_CAPACITY;
    let f = fixture(vec![]);
    let start = |seed| {
        f.service
            .start(
                f.profile.id,
                "test-skill",
                AnswerMode::Keyboard,
                None,
                Some(seed),
            )
            .unwrap()
    };
    let first = start(1);
    f.service
        .answer(first.id, 0, &first.exercises[0].expected, None)
        .unwrap();
    for seed in 0..SERIES_CAPACITY as u32 {
        start(100 + seed);
    }

    let session = f.sessions.session(first.id).unwrap().unwrap();
    assert!(session.ended_at.is_some() && session.abandoned);
    assert_eq!(1, session.correct_count);
    assert!(matches!(
        f.service
            .answer(first.id, 1, &first.exercises[1].expected, None),
        Err(Error::SeriesNotFound(_))
    ));
}

#[test]
fn the_series_of_a_deleted_profile_are_forgotten() {
    let f = fixture(vec![]);
    let series = f
        .service
        .start(
            f.profile.id,
            "test-skill",
            AnswerMode::Keyboard,
            None,
            Some(7),
        )
        .unwrap();

    f.service.forget_profile(f.profile.id);

    assert!(matches!(
        f.service
            .answer(series.id, 0, &series.exercises[0].expected, None),
        Err(Error::SeriesNotFound(_))
    ));
}
