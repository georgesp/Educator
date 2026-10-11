mod common;

use common::{FakeCharacters, InMemoryProfileStore};
use educator_core::clock::{unix_ms, ManualClock};
use educator_core::profiles::ProfileService;
use educator_core::random::{RandomSource, ScriptedRandom};
use educator_core::{Error, Grade};
use std::sync::Arc;
use uuid::Uuid;

fn start() -> i64 {
    unix_ms(2026, 10, 11, 8, 0, 0)
}

struct Fixture {
    clock: Arc<ManualClock>,
    service: ProfileService,
}

fn fixture(random: impl RandomSource + 'static) -> Fixture {
    let clock = Arc::new(ManualClock::new(start()));
    let service = ProfileService::new(
        Arc::new(InMemoryProfileStore::default()),
        Arc::new(FakeCharacters::default()),
        Arc::new(random),
        clock.clone(),
    );
    Fixture { clock, service }
}

fn service() -> ProfileService {
    fixture(ScriptedRandom::new([0])).service
}

fn validation_message(result: Result<impl std::fmt::Debug, Error>) -> (&'static str, String) {
    match result {
        Err(Error::ProfileValidation { field, message }) => (field, message),
        other => panic!("expected a validation error, got {other:?}"),
    }
}

#[test]
fn create_draws_a_character_at_random() {
    let service = fixture(ScriptedRandom::new([1])).service;

    let profile = service.create("  Léa  ", Grade::CP, None).unwrap();

    assert_eq!("Léa", profile.name);
    assert_eq!(Grade::CP, profile.grade);
    assert_eq!("ours", profile.character_id);
    assert_eq!(0, profile.points);
    assert_eq!(start(), profile.last_used_at);
    assert_eq!(7, profile.id.get_version_num());
}

#[test]
fn create_invalid_name_says_why_in_french() {
    for (name, message) in [
        ("", "Écris ton prénom."),
        ("   ", "Écris ton prénom."),
        (
            "Maximilien-Alexandre-Jean",
            "Ton prénom est trop long : 20 lettres au maximum.",
        ),
    ] {
        let (field, actual) = validation_message(service().create(name, Grade::CP, None));
        assert_eq!("name", field);
        assert_eq!(message, actual);
    }
}

#[test]
fn create_name_already_taken_ignoring_case() {
    let service = service();
    service.create("Léa", Grade::CP, None).unwrap();

    let (_, message) = validation_message(service.create("LÉA", Grade::CE1, None));

    assert_eq!("Ce prénom est déjà pris. Choisis-en un autre.", message);
}

#[test]
fn create_keeps_the_character_shown_on_screen_if_it_exists() {
    let service = service();

    let profile = service.create("Léa", Grade::CP, Some("lapin")).unwrap();

    assert_eq!("lapin", profile.character_id);
    let (field, _) = validation_message(service.create("Tom", Grade::CP, Some("dragon")));
    assert_eq!("characterId", field);
}

#[test]
fn draw_character_never_gives_the_excluded_one() {
    let service = fixture(ScriptedRandom::new([0, 1, 2, 3, 4])).service;

    for _ in 0..20 {
        assert_ne!("ours", service.draw_character(Some("ours")).unwrap().id);
    }
}

#[test]
fn reroll_gives_another_character() {
    let service = fixture(ScriptedRandom::new([0, 0, 1, 0])).service;
    let mut profile = service.create("Tom", Grade::CE1, None).unwrap();

    for _ in 0..5 {
        let before = profile.character_id.clone();
        profile = service.reroll_character(profile.id).unwrap();
        assert_ne!(before, profile.character_id);
    }
}

#[test]
fn update_changes_only_the_given_fields() {
    let service = service();
    let profile = service.create("Tom", Grade::CP, None).unwrap();

    let updated = service
        .update(profile.id, None, Some(Grade::CE1), Some("lapin"))
        .unwrap();

    assert_eq!("Tom", updated.name);
    assert_eq!(Grade::CE1, updated.grade);
    assert_eq!("lapin", updated.character_id);
    validation_message(service.update(profile.id, None, None, Some("dragon")));
}

#[test]
fn update_keeping_its_own_name_is_allowed() {
    let service = service();
    let profile = service.create("Tom", Grade::CP, None).unwrap();

    let updated = service.update(profile.id, Some("TOM"), None, None).unwrap();

    assert_eq!("TOM", updated.name);
}

#[test]
fn select_updates_last_use() {
    let Fixture { clock, service } = fixture(ScriptedRandom::new([0]));
    let profile = service.create("Tom", Grade::CP, None).unwrap();
    clock.advance(2 * 3_600_000);

    let selected = service.select(profile.id).unwrap();

    assert_eq!(start() + 2 * 3_600_000, selected.last_used_at);
}

#[test]
fn unknown_profile_is_reported() {
    let service = service();

    assert!(matches!(
        service.select(Uuid::now_v7()),
        Err(Error::ProfileNotFound(_))
    ));
    assert!(matches!(
        service.delete(Uuid::now_v7()),
        Err(Error::ProfileNotFound(_))
    ));
}

#[test]
fn unknown_grade_reads_choose_your_grade() {
    let (field, message) = validation_message(educator_core::profiles::parse_grade("CM3"));
    assert_eq!("grade", field);
    assert_eq!("Choisis ta classe.", message);
    assert_eq!(
        Grade::CE2,
        educator_core::profiles::parse_grade("CE2").unwrap()
    );
}

/// SUB000115: a profile edited afterwards follows the creation rules and keeps its points.
#[test]
fn update_checks_the_name_like_the_creation_and_keeps_the_points() {
    let service = service();
    service.create("Léa", Grade::CP, None).unwrap();
    let tom = service.create("Tom", Grade::CP, None).unwrap();

    for (name, message) in [
        ("  ", "Écris ton prénom."),
        (
            "Maximilien-Alexandre-Jean",
            "Ton prénom est trop long : 20 lettres au maximum.",
        ),
        ("léa", "Ce prénom est déjà pris. Choisis-en un autre."),
    ] {
        let (field, actual) = validation_message(service.update(tom.id, Some(name), None, None));
        assert_eq!(("name", message), (field, actual.as_str()));
    }

    let unchanged = service.select(tom.id).unwrap();
    assert_eq!("Tom", unchanged.name);
    assert!(matches!(
        service.update(Uuid::now_v7(), Some("Zoé"), None, None),
        Err(Error::ProfileNotFound(_))
    ));
}
