//! The « adult » gate of the parents' space (SUB001510): a multiplication of a two-digit number by
//! a one-digit number, drawn again at every opening. Not a password, just a guard against a click
//! by the child.

use crate::clock::{uuid_v7, Clock};
use crate::random::RandomSource;
use std::fmt;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Smallest and largest two-digit factor drawn; a multiple of 10 is moved to the next numbers (too
/// easy), so the factor ends between 12 and 99.
const LEFT_MIN: u32 = 12;
const LEFT_MAX: u32 = 98;
/// Smallest and largest one-digit factor.
const RIGHT_MIN: u32 = 3;
const RIGHT_MAX: u32 = 9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParentGate {
    pub left: u32,
    pub right: u32,
}

impl ParentGate {
    /// A new question, e.g. 27 × 6.
    pub fn draw(random: &dyn RandomSource) -> Self {
        let mut left = LEFT_MIN + random.next(LEFT_MAX - LEFT_MIN + 1);
        if left % 10 == 0 {
            left += 1 + random.next(9);
        }
        let right = RIGHT_MIN + random.next(RIGHT_MAX - RIGHT_MIN + 1);
        Self { left, right }
    }

    pub fn answer(&self) -> u32 {
        self.left * self.right
    }

    pub fn is_answered_by(&self, given: u32) -> bool {
        given == self.answer()
    }
}

/// The gate waiting for its answer: one at a time (a new one replaces it), used up by any answer,
/// right or wrong, so that a wrong answer always gets a new question.
pub struct ParentGateKeeper {
    random: Arc<dyn RandomSource>,
    clock: Arc<dyn Clock>,
    waiting: Mutex<Option<(Uuid, ParentGate)>>,
}

impl fmt::Debug for ParentGateKeeper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParentGateKeeper").finish_non_exhaustive()
    }
}

impl ParentGateKeeper {
    pub fn new(random: Arc<dyn RandomSource>, clock: Arc<dyn Clock>) -> Self {
        Self {
            random,
            clock,
            waiting: Mutex::new(None),
        }
    }

    /// A new gate and its id, to send back with the answer.
    pub fn open(&self) -> (Uuid, ParentGate) {
        let gate = ParentGate::draw(self.random.as_ref());
        let id = uuid_v7(self.clock.now_ms());
        *self.waiting.lock().unwrap_or_else(|e| e.into_inner()) = Some((id, gate));
        (id, gate)
    }

    /// Whether `answer` opens the gate `id` (as text, from the front). The gate is used up either way.
    pub fn check(&self, id: &str, answer: u32) -> bool {
        let waiting = self
            .waiting
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take();
        matches!(
            (waiting, Uuid::parse_str(id)),
            (Some((expected, gate)), Ok(given)) if expected == given && gate.is_answered_by(answer)
        )
    }
}
