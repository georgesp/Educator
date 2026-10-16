//! Hybrid logical clock (ADR 0007, section 3): wall-clock milliseconds, a counter for the changes
//! made within the same millisecond (or while the clock is behind the last one seen), and the
//! device as the last tie-breaker. Two changes always compare the same way on every device, and a
//! device whose clock is a few minutes late still orders its changes after those it has received.

use std::fmt;
use uuid::Uuid;

/// Widest counter written in the text form; beyond it the clock moves to the next millisecond.
const MAX_COUNTER: u32 = 999_999;

/// Latest wall clock accepted (year 5138): far beyond any real clock, and the text stays 16 digits
/// wide even when [`Hlc::next`] moves past it.
pub const MAX_WALL_MS: i64 = 99_999_999_999_999;

/// A point of the hybrid logical clock. The derived order (wall, counter, device) is the order of
/// the text form ([`Hlc::encode`]): the database compares the texts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Hlc {
    pub wall_ms: i64,
    pub counter: u32,
    pub device: Uuid,
}

impl Hlc {
    pub const fn new(wall_ms: i64, counter: u32, device: Uuid) -> Self {
        Self {
            wall_ms,
            counter,
            device,
        }
    }

    /// The smallest clock: older than any change.
    pub const ZERO: Hlc = Hlc::new(0, 0, Uuid::nil());

    /// The clock of a new change made on `device` at `now_ms`, after `last` (the greatest clock
    /// known on this device, local or received).
    pub fn next(last: Option<Hlc>, now_ms: i64, device: Uuid) -> Hlc {
        let now_ms = now_ms.clamp(0, MAX_WALL_MS);
        match last {
            Some(last) if last.wall_ms >= now_ms => {
                if last.counter >= MAX_COUNTER {
                    Hlc::new(last.wall_ms + 1, 0, device)
                } else {
                    Hlc::new(last.wall_ms, last.counter + 1, device)
                }
            }
            _ => Hlc::new(now_ms, 0, device),
        }
    }

    /// `0001792095649001-000003-0199F1A2-…`: fixed width, so that texts sort like clocks.
    pub fn encode(&self) -> String {
        let device = self
            .device
            .hyphenated()
            .encode_upper(&mut Uuid::encode_buffer())
            .to_owned();
        format!(
            "{:016}-{:06}-{device}",
            self.wall_ms.max(0),
            self.counter.min(MAX_COUNTER)
        )
    }

    /// The clock of a text written by [`Hlc::encode`]; `None` when malformed.
    pub fn parse(text: &str) -> Option<Hlc> {
        let (wall, rest) = text.split_once('-')?;
        let (counter, device) = rest.split_once('-')?;
        if wall.len() != 16 || counter.len() != 6 {
            return None;
        }
        if !wall.bytes().all(|b| b.is_ascii_digit()) || !counter.bytes().all(|b| b.is_ascii_digit())
        {
            return None;
        }
        let wall: i64 = wall.parse().ok()?;
        if wall > MAX_WALL_MS {
            return None;
        }
        Some(Hlc::new(
            wall,
            counter.parse().ok()?,
            Uuid::parse_str(device).ok()?,
        ))
    }
}

impl fmt::Display for Hlc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.encode())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn device(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    #[test]
    fn a_new_change_follows_the_wall_clock_when_it_moves_on() {
        let last = Hlc::new(1_000, 4, device(1));
        assert_eq!(
            Hlc::new(2_000, 0, device(2)),
            Hlc::next(Some(last), 2_000, device(2))
        );
        assert_eq!(Hlc::new(5, 0, device(2)), Hlc::next(None, 5, device(2)));
    }

    #[test]
    fn a_late_clock_still_orders_its_changes_after_the_last_one_seen() {
        let last = Hlc::new(10_000, 2, device(9));
        let next = Hlc::next(Some(last), 4_000, device(1));
        assert_eq!(Hlc::new(10_000, 3, device(1)), next);
        assert!(next > last);
    }

    #[test]
    fn the_counter_never_overflows_its_text() {
        let last = Hlc::new(7, MAX_COUNTER, device(1));
        assert_eq!(
            Hlc::new(8, 0, device(1)),
            Hlc::next(Some(last), 7, device(1))
        );
    }

    #[test]
    fn the_text_round_trips_and_sorts_like_the_clock() {
        let clocks = [
            Hlc::new(1_792_095_649_001, 3, device(0xABC)),
            Hlc::new(1_792_095_649_001, 3, device(0xABD)),
            Hlc::new(1_792_095_649_001, 12, device(1)),
            Hlc::new(1_792_095_649_002, 0, device(0)),
            Hlc::new(99, 0, device(u128::MAX)),
        ];
        for clock in clocks {
            assert_eq!(Some(clock), Hlc::parse(&clock.encode()));
        }
        let mut by_clock = clocks.to_vec();
        by_clock.sort();
        let mut by_text = clocks.to_vec();
        by_text.sort_by_key(Hlc::encode);
        assert_eq!(by_clock, by_text);
    }

    #[test]
    fn malformed_texts_are_refused() {
        for text in [
            "",
            "12-000001-x",
            "000000000000000a-000001-00000000-0000-0000-0000-000000000000",
            // A clock from far in the future (a broken peer): refused rather than overflowing.
            "9999999999999999-000001-00000000-0000-0000-0000-000000000000",
        ] {
            assert_eq!(None, Hlc::parse(text));
        }
    }
}
