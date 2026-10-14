//! The « adult » gate of the parents' space (SUB001510).

use educator_core::clock::{unix_ms, ManualClock};
use educator_core::parent_gate::{ParentGate, ParentGateKeeper};
use educator_core::random::{RandomSource, ScriptedRandom, ThreadRandom};
use std::sync::Arc;

#[test]
fn the_gate_multiplies_a_two_digit_number_by_a_one_digit_number() {
    let random = ThreadRandom;
    for _ in 0..500 {
        let gate = ParentGate::draw(&random);
        assert!((12..=99).contains(&gate.left), "{gate:?}");
        assert_ne!(0, gate.left % 10, "{gate:?}");
        assert!((3..=9).contains(&gate.right), "{gate:?}");
        assert!(gate.is_answered_by(gate.left * gate.right));
        assert!(!gate.is_answered_by(gate.left * gate.right + 1));
    }
}

#[test]
fn a_multiple_of_ten_is_moved_off() {
    // 12 + 8 = 20 → 20 + 1 + 4 = 25; 3 + 3 = 6.
    let random = ScriptedRandom::new([8, 4, 3]);
    let gate = ParentGate::draw(&random as &dyn RandomSource);
    assert_eq!((25, 6), (gate.left, gate.right));
    assert_eq!(150, gate.answer());
}

fn keeper() -> ParentGateKeeper {
    ParentGateKeeper::new(
        Arc::new(ThreadRandom),
        Arc::new(ManualClock::new(unix_ms(2026, 10, 14, 8, 0, 0))),
    )
}

#[test]
fn the_gate_opens_with_its_product_only_once() {
    let gates = keeper();
    let (id, gate) = gates.open();
    let id = id.to_string();

    assert!(gates.check(&id.to_uppercase(), gate.answer()));
    // Used up.
    assert!(!gates.check(&id, gate.answer()));
}

#[test]
fn a_wrong_answer_uses_the_gate_up() {
    let gates = keeper();
    let (id, gate) = gates.open();
    let id = id.to_string();

    assert!(!gates.check(&id, gate.answer() + 1));
    assert!(!gates.check(&id, gate.answer()));
    // A wrong id too.
    let (_, gate) = gates.open();
    assert!(!gates.check("not-an-id", gate.answer()));
}

#[test]
fn a_new_gate_replaces_the_one_waiting() {
    let gates = keeper();
    let (older, older_gate) = gates.open();
    let (newer, _) = gates.open();
    assert_ne!(older, newer);

    assert!(!gates.check(&older.to_string(), older_gate.answer()));
    let (newest, newest_gate) = gates.open();
    assert!(gates.check(&newest.to_string(), newest_gate.answer()));
}
