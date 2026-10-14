//! Sessions, answers and masteries (SUB001401) on a real SQLite file in a temporary folder.

use educator_core::clock::{unix_ms, Clock, ManualClock};
use educator_core::profiles::{Profile, ProfileStore};
use educator_core::sessions::{
    update_masteries, AnswerRecord, SessionEnd, SessionRecord, SessionStore, SessionType,
    SkillMastery,
};
use educator_core::skills::{AnswerMode, ExerciseFormat, Subject};
use educator_core::Grade;
use educator_data::{Database, SqliteProfileStore, SqliteSessionStore, DATABASE_FILE_NAME};
use std::sync::Arc;
use uuid::Uuid;

struct Fixture {
    _dir: tempfile::TempDir,
    path: std::path::PathBuf,
    clock: Arc<ManualClock>,
    profiles: SqliteProfileStore,
    sessions: SqliteSessionStore,
    profile: Profile,
}

fn fixture() -> Fixture {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(DATABASE_FILE_NAME);
    let clock = Arc::new(ManualClock::new(unix_ms(2026, 10, 13, 9, 0, 0)));
    let (database, _) = Database::open(&path, clock.clone() as Arc<dyn Clock>).unwrap();
    let database = Arc::new(database);
    let profiles = SqliteProfileStore::new(database.clone());
    let sessions = SqliteSessionStore::new(database);
    let profile = add_profile(&profiles, "Léa", clock.now_ms());
    Fixture {
        _dir: dir,
        path,
        clock,
        profiles,
        sessions,
        profile,
    }
}

fn add_profile(profiles: &SqliteProfileStore, name: &str, now: i64) -> Profile {
    profiles
        .add(&Profile {
            id: Uuid::now_v7(),
            name: name.to_owned(),
            grade: Grade::CE1,
            character_id: "lion".to_owned(),
            points: 0,
            created_at: 0,
            updated_at: 0,
            last_used_at: now,
        })
        .unwrap()
}

fn session(profile_id: Uuid, session_type: SessionType, started_at: i64) -> SessionRecord {
    SessionRecord {
        id: Uuid::now_v7(),
        profile_id,
        session_type,
        subject: Subject::Spelling,
        grade: Grade::CE1,
        skill_id: Some("ce1-son-o-au-eau".to_owned()),
        lesson_id: None,
        answer_mode: Some(AnswerMode::Keyboard),
        format: Some(ExerciseFormat::Dictation),
        seed: 42,
        question_count: 10,
        time_limit_seconds: None,
        started_at,
        ended_at: None,
        abandoned: false,
        correct_count: 0,
        points_earned: 0,
    }
}

fn answer(session_id: Uuid, index: u32, skill: &str, correct: bool, at: i64) -> AnswerRecord {
    AnswerRecord {
        id: Uuid::now_v7(),
        session_id,
        index,
        skill_id: skill.to_owned(),
        prompt: "Dictée".to_owned(),
        expected: "bateau".to_owned(),
        given: if correct { "bateau" } else { "bato" }.to_owned(),
        is_correct: correct,
        duration_ms: Some(2500),
        answered_at: at,
    }
}

fn count(path: &std::path::Path, table: &str) -> i64 {
    rusqlite::Connection::open(path)
        .unwrap()
        .query_row(&format!(r#"SELECT COUNT(*) FROM "{table}""#), [], |r| {
            r.get(0)
        })
        .unwrap()
}

#[test]
fn a_session_and_its_answers_are_stored_and_read_back() {
    let f = fixture();
    let now = f.clock.now_ms();
    let s = session(f.profile.id, SessionType::Exercise, now);
    f.sessions.add_session(&s).unwrap();

    assert_eq!(Some(s.clone()), f.sessions.session(s.id).unwrap());
    assert!(f
        .sessions
        .add_answer(&answer(s.id, 1, "ce1-son-o-au-eau", false, now + 2))
        .unwrap());
    assert!(f
        .sessions
        .add_answer(&answer(s.id, 0, "ce1-son-o-au-eau", true, now + 1))
        .unwrap());
    // The same question twice: refused.
    assert!(!f
        .sessions
        .add_answer(&answer(s.id, 0, "ce1-son-o-au-eau", false, now + 3))
        .unwrap());

    let answers = f.sessions.answers(s.id).unwrap();
    assert_eq!(
        vec![0, 1],
        answers.iter().map(|a| a.index).collect::<Vec<_>>()
    );
    assert_eq!("bato", answers[1].given);
    assert_eq!(Some(2500), answers[1].duration_ms);

    let end = SessionEnd {
        ended_at: now + 10,
        abandoned: false,
        question_count: 10,
        correct_count: 1,
        points_earned: 30,
    };
    assert!(f.sessions.end_session(s.id, &end).unwrap());
    // Already closed.
    assert!(!f.sessions.end_session(s.id, &end).unwrap());
    let stored = f.sessions.session(s.id).unwrap().unwrap();
    assert_eq!(
        (Some(now + 10), 1, 30),
        (stored.ended_at, stored.correct_count, stored.points_earned)
    );
}

#[test]
fn sessions_left_open_are_closed_as_abandoned_with_their_answers() {
    let f = fixture();
    let now = f.clock.now_ms();
    let open = session(f.profile.id, SessionType::Exercise, now);
    let empty = session(f.profile.id, SessionType::Exam, now + 5);
    f.sessions.add_session(&open).unwrap();
    f.sessions.add_session(&empty).unwrap();
    f.sessions
        .add_answer(&answer(open.id, 0, "a", true, now + 100))
        .unwrap();
    f.sessions
        .add_answer(&answer(open.id, 1, "a", true, now + 200))
        .unwrap();
    f.sessions
        .add_answer(&answer(open.id, 2, "a", false, now + 300))
        .unwrap();

    assert_eq!(2, f.sessions.open_sessions().unwrap().len());
    assert_eq!(
        2,
        educator_core::sessions::close_open_sessions(&f.sessions, now + 1000).unwrap()
    );
    // The answers given before the application was closed count in the mastery.
    let mastery = f.sessions.masteries(f.profile.id).unwrap();
    assert_eq!(
        vec![("a".to_owned(), 67, 3)],
        mastery
            .iter()
            .map(|m| (m.skill_id.clone(), m.score, m.attempt_count))
            .collect::<Vec<_>>()
    );
    assert!(f.sessions.open_sessions().unwrap().is_empty());

    let closed = f.sessions.session(open.id).unwrap().unwrap();
    assert!(closed.abandoned);
    assert_eq!(
        (Some(now + 300), 2),
        (closed.ended_at, closed.correct_count)
    );
    assert_eq!(
        Some(now + 5),
        f.sessions.session(empty.id).unwrap().unwrap().ended_at
    );
    assert_eq!(3, f.sessions.answers(open.id).unwrap().len());
    assert_eq!(0, f.sessions.abandon_open_sessions(now + 2000).unwrap());
}

#[test]
fn mastery_averages_the_last_twenty_answers_of_the_profile() {
    let f = fixture();
    let other = add_profile(&f.profiles, "Tom", f.clock.now_ms());
    let now = f.clock.now_ms();
    let s = session(f.profile.id, SessionType::Exercise, now);
    f.sessions.add_session(&s).unwrap();
    // 10 old wrong answers, then 20 right ones: only the last 20 count.
    for i in 0..30u32 {
        f.sessions
            .add_answer(&answer(s.id, i, "skill-a", i >= 10, now + i64::from(i)))
            .unwrap();
    }
    // Another profile's answers never count.
    let t = session(other.id, SessionType::Exercise, now);
    f.sessions.add_session(&t).unwrap();
    f.sessions
        .add_answer(&answer(t.id, 0, "skill-a", false, now + 100))
        .unwrap();

    update_masteries(
        &f.sessions,
        f.profile.id,
        &[
            "skill-a".to_owned(),
            "skill-a".to_owned(),
            "skill-b".to_owned(),
        ],
        now + 500,
    )
    .unwrap();

    assert_eq!(
        vec![SkillMastery {
            profile_id: f.profile.id,
            skill_id: "skill-a".to_owned(),
            score: 100,
            attempt_count: 30,
            last_practiced_at: now + 500,
        }],
        f.sessions.masteries(f.profile.id).unwrap()
    );

    // A new wrong answer: 19 right out of the last 20.
    f.sessions
        .add_answer(&answer(s.id, 30, "skill-a", false, now + 600))
        .unwrap();
    update_masteries(
        &f.sessions,
        f.profile.id,
        &["skill-a".to_owned()],
        now + 700,
    )
    .unwrap();
    let mastery = &f.sessions.masteries(f.profile.id).unwrap()[0];
    assert_eq!((95, 31), (mastery.score, mastery.attempt_count));
}

#[test]
fn best_timed_score_counts_finished_challenges_of_the_same_duration() {
    let f = fixture();
    let now = f.clock.now_ms();
    for (limit, correct, abandoned) in [
        (120, 12, false),
        (120, 17, true),
        (60, 30, false),
        (120, 9, false),
    ] {
        let mut s = session(f.profile.id, SessionType::Timed, now);
        s.time_limit_seconds = Some(limit);
        f.sessions.add_session(&s).unwrap();
        f.sessions
            .end_session(
                s.id,
                &SessionEnd {
                    ended_at: now + 1,
                    abandoned,
                    question_count: correct + 2,
                    correct_count: correct,
                    points_earned: 0,
                },
            )
            .unwrap();
    }

    assert_eq!(
        Some(12),
        f.sessions
            .best_timed_score(f.profile.id, Subject::Spelling, 120)
            .unwrap()
    );
    assert_eq!(
        None,
        f.sessions
            .best_timed_score(f.profile.id, Subject::Math, 120)
            .unwrap()
    );
}

#[test]
fn deleting_a_profile_deletes_its_sessions_answers_and_masteries() {
    let f = fixture();
    let other = add_profile(&f.profiles, "Tom", f.clock.now_ms());
    let now = f.clock.now_ms();
    for profile in [f.profile.id, other.id] {
        let s = session(profile, SessionType::Exercise, now);
        f.sessions.add_session(&s).unwrap();
        f.sessions
            .add_answer(&answer(s.id, 0, "a", true, now))
            .unwrap();
        update_masteries(&f.sessions, profile, &["a".to_owned()], now).unwrap();
    }

    assert!(f.profiles.delete(f.profile.id).unwrap());

    assert_eq!(1, count(&f.path, "Session"));
    assert_eq!(1, count(&f.path, "Answer"));
    assert_eq!(1, count(&f.path, "SkillMastery"));
    assert!(f.sessions.masteries(f.profile.id).unwrap().is_empty());
    assert_eq!(1, f.sessions.masteries(other.id).unwrap().len());
}

#[test]
fn the_slide_of_an_unfinished_lesson_is_kept_until_it_is_finished() {
    let f = fixture();
    assert_eq!(
        None,
        f.sessions
            .lesson_slide(f.profile.id, "ce1-son-o-au-eau")
            .unwrap()
    );
    f.sessions
        .save_lesson_slide(f.profile.id, "ce1-son-o-au-eau", Some(3))
        .unwrap();
    f.sessions
        .save_lesson_slide(f.profile.id, "ce1-son-o-au-eau", Some(5))
        .unwrap();
    assert_eq!(
        Some(5),
        f.sessions
            .lesson_slide(f.profile.id, "ce1-son-o-au-eau")
            .unwrap()
    );
    f.sessions
        .save_lesson_slide(f.profile.id, "ce1-son-o-au-eau", None)
        .unwrap();
    assert_eq!(
        None,
        f.sessions
            .lesson_slide(f.profile.id, "ce1-son-o-au-eau")
            .unwrap()
    );

    f.sessions
        .save_lesson_slide(f.profile.id, "cp-son-ou", Some(2))
        .unwrap();
    f.profiles.delete(f.profile.id).unwrap();
    assert_eq!(0, count(&f.path, "LessonProgress"));
}

#[test]
fn the_parents_space_reads_the_closed_sessions_and_answers_of_one_profile() {
    let f = fixture();
    let now = f.clock.now_ms();
    let other = add_profile(&f.profiles, "Tom", now);
    let end = |at: i64| SessionEnd {
        ended_at: at,
        abandoned: false,
        question_count: 10,
        correct_count: 1,
        points_earned: 30,
    };

    let later = session(f.profile.id, SessionType::Exercise, now + 100);
    let earlier = session(f.profile.id, SessionType::Exam, now);
    let open = session(f.profile.id, SessionType::Exercise, now + 200);
    let of_other = session(other.id, SessionType::Exercise, now);
    for s in [&later, &earlier, &open, &of_other] {
        f.sessions.add_session(s).unwrap();
    }
    f.sessions
        .add_answer(&answer(later.id, 0, "ce1-son-o-au-eau", true, now + 101))
        .unwrap();
    f.sessions
        .add_answer(&answer(earlier.id, 0, "ce1-son-o-au-eau", false, now + 1))
        .unwrap();
    f.sessions
        .add_answer(&answer(open.id, 0, "ce1-son-o-au-eau", true, now + 201))
        .unwrap();
    f.sessions
        .add_answer(&answer(of_other.id, 0, "ce1-son-o-au-eau", true, now + 2))
        .unwrap();
    f.sessions.end_session(later.id, &end(now + 150)).unwrap();
    f.sessions.end_session(earlier.id, &end(now + 50)).unwrap();
    f.sessions.end_session(of_other.id, &end(now + 50)).unwrap();

    let sessions = f.sessions.profile_sessions(f.profile.id).unwrap();
    assert_eq!(
        vec![earlier.id, later.id],
        sessions.iter().map(|s| s.id).collect::<Vec<_>>()
    );
    assert_eq!(Some(now + 50), sessions[0].ended_at);

    let answers = f.sessions.profile_answers(f.profile.id).unwrap();
    assert_eq!(
        vec![earlier.id, later.id],
        answers.iter().map(|a| a.session_id).collect::<Vec<_>>()
    );
    assert_eq!("bato", answers[0].given);
    assert_eq!(Some(2500), answers[0].duration_ms);

    let fresh = add_profile(&f.profiles, "Zoé", now);
    assert!(f.sessions.profile_sessions(fresh.id).unwrap().is_empty());
    assert!(f.sessions.profile_answers(fresh.id).unwrap().is_empty());
}
