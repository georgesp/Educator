//! Source of randomness, injectable so that the tests control the draws (`IRandomSource` in C#).
//!
//! The series themselves are generated from a seed with [`SeededRng`] (ChaCha8): the same seed gives
//! the same series on every platform and with every version of `rand_chacha` 0.9.

use rand::Rng;
use std::fmt;

/// Draws for the services (first character of a profile, seed of a new series).
pub trait RandomSource: Send + Sync {
    /// A number in `[0, max_exclusive)`. `max_exclusive` is at least 1.
    fn next(&self, max_exclusive: u32) -> u32;
}

/// [`RandomSource`] backed by the thread-local generator of `rand`.
#[derive(Debug, Default, Clone, Copy)]
pub struct ThreadRandom;

impl RandomSource for ThreadRandom {
    fn next(&self, max_exclusive: u32) -> u32 {
        rand::rng().random_range(0..max_exclusive.max(1))
    }
}

/// Deterministic generator used by the exercise engine.
pub type SeededRng = rand_chacha::ChaCha8Rng;

/// Draws the given numbers in a loop (tests).
pub struct ScriptedRandom {
    values: Vec<u32>,
    next: std::sync::atomic::AtomicUsize,
}

impl ScriptedRandom {
    pub fn new(values: impl Into<Vec<u32>>) -> Self {
        let values = values.into();
        assert!(
            !values.is_empty(),
            "ScriptedRandom needs at least one value"
        );
        Self {
            values,
            next: std::sync::atomic::AtomicUsize::new(0),
        }
    }
}

impl fmt::Debug for ScriptedRandom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScriptedRandom")
            .field("values", &self.values)
            .finish()
    }
}

impl RandomSource for ScriptedRandom {
    fn next(&self, max_exclusive: u32) -> u32 {
        let index = self.next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.values[index % self.values.len()] % max_exclusive.max(1)
    }
}
