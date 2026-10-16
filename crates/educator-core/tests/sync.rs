//! Pure rules of the synchronisation (ADR 0007): field-by-field merge of a profile, free names,
//! the question « Est-ce le même enfant ? » and the points recomputed from the sessions. The merge
//! of whole databases is tested on SQLite (`educator-data/tests/sync.rs`).

use educator_core::profiles::MAX_NAME_LENGTH;
use educator_core::rewards::daily_goal::GoalDay;
use educator_core::sessions::{SessionRecord, SessionType};
use educator_core::skills::Subject;
use educator_core::sync::*;
use educator_core::Grade;
use std::collections::HashSet;
use uuid::Uuid;

fn device(n: u128) -> Uuid {
    Uuid::from_u128(n)
}

fn state(id: u128, name: &str, clock: Hlc) -> ProfileState {
    ProfileState {
        id: Uuid::from_u128(id),
        name: name.to_owned(),
        grade: Grade::CE1,
        character_id: "lion".to_owned(),
        accessories: Vec::new(),
        placement_seconds: 20,
        exam_seconds: 0,
        created_at: 100,
        last_used_at: 200,
        clocks: ProfileClocks::all(clock),
    }
}

#[test]
fn a_profile_is_merged_field_by_field_whatever_the_order() {
    let mut local = state(1, "Léa", Hlc::new(10, 0, device(1)));
    local.grade = Grade::CE2;
    local.clocks.grade = Hlc::new(30, 0, device(1));
    local.last_used_at = 900;
    let mut incoming = state(1, "Léana", Hlc::new(20, 0, device(2)));
    incoming.created_at = 50;

    let merged = merge_profile(Some(&local), &incoming);

    assert_eq!("Léana", merged.name);
    assert_eq!(Grade::CE2, merged.grade);
    assert_eq!(50, merged.created_at);
    assert_eq!(900, merged.last_used_at);
    assert_eq!(merged, merge_profile(Some(&incoming), &local));
    assert_eq!(merged, merge_profile(Some(&merged), &incoming));
}

#[test]
fn a_free_name_gets_a_number_and_stays_within_the_limit() {
    let taken: HashSet<String> = ["LÉA", "LÉA 2"].iter().map(|s| s.to_string()).collect();
    assert_eq!("Léa 3", distinct_name("Léa", &taken));

    let long = "Marie-Charlotte Anne";
    assert_eq!(MAX_NAME_LENGTH, long.encode_utf16().count());
    let free = distinct_name(long, &HashSet::new());
    assert_eq!("Marie-Charlotte An 2", free);
    assert!(free.encode_utf16().count() <= MAX_NAME_LENGTH);
}

#[test]
fn only_two_identifiers_with_the_same_name_ask_whether_it_is_the_same_child() {
    let clock = Hlc::new(1, 0, device(9));
    let remote = vec![
        state(1, "LÉA", clock),
        state(2, "Tom", clock),
        state(3, "Zoé", clock),
    ];
    let local = vec![
        (Uuid::from_u128(10), "Léa".to_owned()),
        (Uuid::from_u128(2), "Tom".to_owned()),
        (Uuid::from_u128(11), "Inès".to_owned()),
    ];

    let conflicts = name_conflicts(&local, &remote);

    assert_eq!(1, conflicts.len());
    assert_eq!(Uuid::from_u128(10), conflicts[0].local_id);
    assert_eq!(Uuid::from_u128(1), conflicts[0].remote.id);
}

#[test]
fn the_points_are_those_of_the_closed_sessions_and_the_goal_bonuses() {
    let session = |points: i32, ended: bool| SessionRecord {
        id: Uuid::now_v7(),
        profile_id: Uuid::from_u128(1),
        session_type: SessionType::Exercise,
        subject: Subject::Math,
        grade: Grade::CE1,
        skill_id: None,
        lesson_id: None,
        answer_mode: None,
        format: None,
        seed: 1,
        question_count: 1,
        time_limit_seconds: None,
        started_at: 1,
        ended_at: ended.then_some(2),
        abandoned: false,
        correct_count: 0,
        points_earned: points,
    };
    let goal = GoalDay {
        day: 1,
        reached_at: 1,
        bonus_points: 30,
    };

    assert_eq!(
        40 + 20 + 30,
        profile_points(
            &[session(40, true), session(20, true), session(99, false)],
            &[goal]
        )
    );
    assert_eq!(0, profile_points(&[], &[]));
}

#[test]
fn a_batch_tells_its_greatest_clock_and_its_size_without_names() {
    let batch = ChangeBatch {
        profiles: vec![state(1, "Léa", Hlc::new(5, 0, device(1)))],
        removals: vec![ProfileRemoval {
            profile_id: Uuid::from_u128(2),
            clock: Hlc::new(9, 1, device(2)),
            merged_into: None,
        }],
        ..ChangeBatch::default()
    };

    assert_eq!(Some(Hlc::new(9, 1, device(2))), batch.max_clock());
    assert_eq!(2, batch.item_count());
    assert!(!batch.to_string().contains("Léa"));
}

#[test]
fn record_keys_round_trip() {
    let profile = Uuid::now_v7();
    let key = record_key(profile, &["ce1-add", "7+5"]);
    assert_eq!(Some((profile, "ce1-add/7+5")), split_record_key(&key));
    assert_eq!(None, split_record_key("not-a-key"));
}
