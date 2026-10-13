//! Objects and pictures of the lessons (SUB000117): `groupObjects`, `tenFrame`, `fractionBar` and
//! `barModel`. The values they show are computed here.

use super::{Equality, EquationTerm, Operator};
use crate::exercises::{Comparison, Fraction};
use crate::numbers::fraction_words;

// ── groupObjects ───────────────────────────────────────────────────────────────────────────

/// The racing objects drawn (palette of the characters).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RaceObject {
    #[default]
    Car,
    Helmet,
    Tire,
    Flag,
    Cone,
}

/// What happens to the objects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupMode {
    /// Two groups join (3 red cars and 2 blue ones). `commute`: they then swap places (2 + 6 =
    /// 6 + 2). Interactive: the child moves each object onto the grid, then picks the total.
    Join { counts: [u32; 2], commute: bool },
    /// `removed` of the `total` objects leave. Interactive: the child makes them leave, then
    /// picks what remains.
    Remove { total: u32, removed: u32 },
    /// Each collection is put in groups of `size` (in pairs: is the number even?).
    GroupBy { counts: Vec<u32>, size: u32 },
    /// `total` objects shared in `parts` equal parts.
    Share { total: u32, parts: u32 },
    /// Numbers to park in the « pair » or « impair » garage. Interactive: the child parks each one.
    SortParity { numbers: Vec<u32> },
}

/// `groupObjects`: racing objects to join, take away, group or share.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupObjects {
    pub mode: GroupMode,
    pub object: RaceObject,
    pub interactive: bool,
}

/// A collection grouped by `size`: the full groups and the objects left alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grouping {
    pub count: u32,
    pub groups: u32,
    pub rest: u32,
}

impl GroupObjects {
    /// The number found at the end: the total, what remains, one part of the share.
    pub fn result(&self) -> u32 {
        match &self.mode {
            GroupMode::Join { counts, .. } => counts[0] + counts[1],
            GroupMode::Remove { total, removed } => total.saturating_sub(*removed),
            GroupMode::GroupBy { counts, .. } => counts.first().copied().unwrap_or(0),
            GroupMode::Share { total, parts } => total / (*parts).max(1),
            GroupMode::SortParity { numbers } => {
                numbers.iter().filter(|n| *n % 2 == 0).count() as u32
            }
        }
    }

    /// GroupBy: each collection in groups of `size`.
    pub fn groupings(&self) -> Vec<Grouping> {
        match &self.mode {
            GroupMode::GroupBy { counts, size } => counts
                .iter()
                .map(|count| Grouping {
                    count: *count,
                    groups: count / size,
                    rest: count % size,
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    /// SortParity: whether each number is even.
    pub fn parities(&self) -> Vec<bool> {
        match &self.mode {
            GroupMode::SortParity { numbers } => numbers.iter().map(|n| n % 2 == 0).collect(),
            _ => Vec::new(),
        }
    }

    /// Interactive join or removal: the numbers proposed for the total (or what remains), around
    /// the right one.
    pub fn choices(&self) -> Vec<u32> {
        if !self.interactive {
            return Vec::new();
        }
        match self.mode {
            GroupMode::Join { .. } | GroupMode::Remove { .. } => {
                let result = self.result();
                if result == 0 {
                    vec![0, 1, 2]
                } else {
                    vec![result - 1, result, result + 1]
                }
            }
            // Grouped by 10: the tens and the units swapped (23 / 32) and one ten more or less.
            GroupMode::GroupBy { .. } => {
                let count = self.result();
                let swapped = (count % 10) * 10 + count / 10;
                let mut choices = vec![count];
                for other in [swapped, count.saturating_sub(10), count + 10] {
                    if choices.len() < 3 && other > 0 && !choices.contains(&other) {
                        choices.push(other);
                    }
                }
                choices.sort_unstable();
                choices
            }
            _ => Vec::new(),
        }
    }
}

// ── tenFrame ───────────────────────────────────────────────────────────────────────────────

/// A place of the « garage de 10 places ».
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameSlot {
    Empty,
    /// A car already parked.
    Filled,
    /// A car that comes in (second color).
    Added,
    /// A parked car that leaves.
    Removed,
}

/// `tenFrame`: one or two garages of 10 places (2 rows of 5), filled in reading order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TenFrame {
    pub filled: u32,
    pub added: u32,
    pub removed: u32,
    pub frames: u32,
    /// The friends of 10 (1 and 9, 2 and 8… 5 and 5), one small garage each.
    pub friends: bool,
    /// The child parks the `added` cars, or makes the `removed` ones leave.
    pub interactive: bool,
}

/// Places of one garage.
pub const FRAME_SIZE: u32 = 10;

impl TenFrame {
    /// The places, garage after garage. The added cars follow the parked ones; the cars leaving
    /// are the last parked.
    pub fn slots(&self) -> Vec<FrameSlot> {
        let size = (FRAME_SIZE * self.frames) as usize;
        let mut slots = vec![FrameSlot::Empty; size];
        let filled = (self.filled as usize).min(size);
        let added = (self.added as usize).min(size - filled);
        let removed = (self.removed as usize).min(filled);
        for slot in slots.iter_mut().take(filled) {
            *slot = FrameSlot::Filled;
        }
        for slot in slots.iter_mut().skip(filled - removed).take(removed) {
            *slot = FrameSlot::Removed;
        }
        for slot in slots.iter_mut().skip(filled).take(added) {
            *slot = FrameSlot::Added;
        }
        slots
    }

    /// The cars in the garages at the end.
    pub fn total(&self) -> u32 {
        (self.filled + self.added).saturating_sub(self.removed)
    }

    /// The friends of 10 shown: (1, 9), (2, 8)… (5, 5).
    pub fn friend_pairs(&self) -> Vec<(u32, u32)> {
        if self.friends {
            (1..=5).map(|n| (n, FRAME_SIZE - n)).collect()
        } else {
            Vec::new()
        }
    }
}

// ── fractionBar ────────────────────────────────────────────────────────────────────────────

/// A strip of track shared in `denominator` equal parts; `numerators` colored parts, one color
/// per number (2/8 + 3/8: two colors).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FractionStrip {
    pub denominator: u32,
    pub numerators: Vec<u32>,
}

impl FractionStrip {
    pub fn colored(&self) -> u32 {
        self.numerators.iter().sum()
    }

    pub fn fraction(&self) -> Fraction {
        Fraction::new(self.colored(), self.denominator)
    }

    /// « trois quarts ».
    pub fn words(&self) -> String {
        fraction_words(self.colored(), self.denominator)
    }
}

/// `fractionBar`: strips of track shared in equal parts, stacked to compare them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FractionBar {
    pub strips: Vec<FractionStrip>,
    /// A strip cut in parts that are not equal (they are not halves): drawn crossed out.
    pub unequal: bool,
    /// The fraction is written next to the strip, with « numérateur » and « dénominateur ».
    pub notation: bool,
    /// The child colors the parts of the only strip.
    pub interactive: bool,
}

impl FractionBar {
    /// Two strips: how the first colored fraction compares to the second (1/5 < 1/3).
    pub fn comparison(&self) -> Option<Comparison> {
        match self.strips.as_slice() {
            [a, b] => Some(a.fraction().compare(&b.fraction())),
            _ => None,
        }
    }
}

// ── barModel ───────────────────────────────────────────────────────────────────────────────

/// The value of a bar model that is looked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarUnknown {
    /// The whole (parts, groups) or the larger quantity (comparison).
    Whole,
    /// A part (from 0), or in a comparison 0 = the smaller quantity, 1 = the difference; in groups,
    /// the size of a group.
    Part(usize),
}

/// The shape of a bar model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BarKind {
    /// A whole made of 2 or 3 parts.
    Parts { parts: Vec<u32> },
    /// Two quantities: `small`, and `small + difference` (« 12 de plus »).
    Compare { small: u32, difference: u32 },
    /// `groups` equal parts of `size`.
    Groups { groups: u32, size: u32 },
}

/// `barModel`: the bar diagram of a problem (« schéma en barres »).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarModel {
    pub kind: BarKind,
    pub unknown: BarUnknown,
    /// Names written on the bars (« Tom », « Léa ») — at most 3.
    pub labels: Vec<String>,
    /// After the diagram, the calculation that finds the « ? ».
    pub solve: bool,
    /// The child places the known numbers on the diagram.
    pub interactive: bool,
}

/// A value of a bar model: a segment of a bar or the whole.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BarValue {
    /// Whole or part index (see [`BarUnknown`]).
    pub role: BarUnknown,
    pub value: u32,
    /// Shown as « ? » until solved.
    pub unknown: bool,
}

impl BarModel {
    /// The whole: sum of the parts, larger quantity, or all the groups.
    pub fn whole(&self) -> u32 {
        match &self.kind {
            BarKind::Parts { parts } => parts.iter().sum(),
            BarKind::Compare { small, difference } => small + difference,
            BarKind::Groups { groups, size } => groups * size,
        }
    }

    /// The parts: the given parts; the smaller quantity and the difference; the size of a group.
    pub fn parts(&self) -> Vec<u32> {
        match &self.kind {
            BarKind::Parts { parts } => parts.clone(),
            BarKind::Compare { small, difference } => vec![*small, *difference],
            BarKind::Groups { size, .. } => vec![*size],
        }
    }

    /// Every value of the diagram, the whole first.
    pub fn values(&self) -> Vec<BarValue> {
        let whole = BarValue {
            role: BarUnknown::Whole,
            value: self.whole(),
            unknown: self.unknown == BarUnknown::Whole,
        };
        std::iter::once(whole)
            .chain(
                self.parts()
                    .into_iter()
                    .enumerate()
                    .map(|(i, value)| BarValue {
                        role: BarUnknown::Part(i),
                        value,
                        unknown: self.unknown == BarUnknown::Part(i),
                    }),
            )
            .collect()
    }

    /// The value looked for.
    pub fn answer(&self) -> u32 {
        self.values()
            .into_iter()
            .find(|v| v.unknown)
            .map_or(0, |v| v.value)
    }

    /// The calculation that finds the « ? »: add the parts for the whole, subtract for a part,
    /// multiply for groups (« 4 × ? = 24 » for the size of a group).
    pub fn solution(&self) -> (Equality, Option<EquationTerm>) {
        let whole = self.whole();
        let parts = self.parts();
        match (&self.kind, self.unknown) {
            (BarKind::Groups { groups, size }, BarUnknown::Whole) => (
                Equality {
                    left: *groups,
                    operator: Operator::Multiply,
                    right: *size,
                    result: whole,
                },
                None,
            ),
            (BarKind::Groups { groups, size }, BarUnknown::Part(_)) => (
                Equality {
                    left: *groups,
                    operator: Operator::Multiply,
                    right: *size,
                    result: whole,
                },
                Some(EquationTerm::Right),
            ),
            (_, BarUnknown::Whole) => (
                Equality {
                    left: parts[0],
                    operator: Operator::Add,
                    right: parts[1..].iter().sum(),
                    result: whole,
                },
                None,
            ),
            (_, BarUnknown::Part(i)) => {
                let known: u32 = parts
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, v)| v)
                    .sum();
                (
                    Equality {
                        left: whole,
                        operator: Operator::Subtract,
                        right: known,
                        result: parts.get(i).copied().unwrap_or(0),
                    },
                    None,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn objects_join_leave_and_group() {
        let join = GroupObjects {
            mode: GroupMode::Join {
                counts: [4, 3],
                commute: false,
            },
            object: RaceObject::Car,
            interactive: true,
        };
        assert_eq!(7, join.result());
        assert_eq!(vec![6, 7, 8], join.choices());

        let remove = GroupObjects {
            mode: GroupMode::Remove {
                total: 7,
                removed: 3,
            },
            object: RaceObject::Helmet,
            interactive: false,
        };
        assert_eq!(4, remove.result());
        assert!(remove.choices().is_empty());

        let pairs = GroupObjects {
            mode: GroupMode::GroupBy {
                counts: vec![8, 7],
                size: 2,
            },
            object: RaceObject::Car,
            interactive: false,
        };
        let groupings = pairs.groupings();
        assert_eq!((4, 0), (groupings[0].groups, groupings[0].rest));
        assert_eq!((3, 1), (groupings[1].groups, groupings[1].rest));
        assert!(pairs.choices().is_empty());

        // The child makes the packets of 10, then picks the number: tens and units swapped as a trap.
        let tens = |count| GroupObjects {
            mode: GroupMode::GroupBy {
                counts: vec![count],
                size: 10,
            },
            object: RaceObject::Car,
            interactive: true,
        };
        assert_eq!(vec![13, 23, 32], tens(23).choices());
        assert_eq!(vec![7, 17, 71], tens(17).choices());
        assert_eq!(vec![2, 10, 20], tens(20).choices());
        assert_eq!(vec![1, 11, 21], tens(11).choices());

        let share = GroupObjects {
            mode: GroupMode::Share {
                total: 24,
                parts: 4,
            },
            object: RaceObject::Tire,
            interactive: false,
        };
        assert_eq!(6, share.result());

        let parity = GroupObjects {
            mode: GroupMode::SortParity {
                numbers: vec![768, 773, 40],
            },
            object: RaceObject::Car,
            interactive: true,
        };
        assert_eq!(vec![true, false, true], parity.parities());
    }

    #[test]
    fn a_garage_fills_in_reading_order() {
        // 8 + 5 with two garages: 2 cars complete the first one, 3 go in the second.
        let frame = TenFrame {
            filled: 8,
            added: 5,
            removed: 0,
            frames: 2,
            friends: false,
            interactive: false,
        };
        let slots = frame.slots();
        assert_eq!(20, slots.len());
        assert_eq!(FrameSlot::Filled, slots[7]);
        assert_eq!(FrameSlot::Added, slots[8]);
        assert_eq!(FrameSlot::Added, slots[12]);
        assert_eq!(FrameSlot::Empty, slots[13]);
        assert_eq!(13, frame.total());

        // 8 − 5: the last 5 parked cars leave.
        let leave = TenFrame {
            filled: 8,
            added: 0,
            removed: 5,
            frames: 1,
            ..frame.clone()
        };
        let slots = leave.slots();
        assert_eq!(FrameSlot::Filled, slots[2]);
        assert_eq!(FrameSlot::Removed, slots[3]);
        assert_eq!(FrameSlot::Removed, slots[7]);
        assert_eq!(3, leave.total());

        let friends = TenFrame {
            friends: true,
            filled: 0,
            added: 0,
            removed: 0,
            frames: 1,
            interactive: false,
        };
        assert_eq!(
            vec![(1, 9), (2, 8), (3, 7), (4, 6), (5, 5)],
            friends.friend_pairs()
        );
    }

    #[test]
    fn fraction_strips_are_named_and_compared() {
        let bar = FractionBar {
            strips: vec![
                FractionStrip {
                    denominator: 3,
                    numerators: vec![1],
                },
                FractionStrip {
                    denominator: 5,
                    numerators: vec![1],
                },
            ],
            unequal: false,
            notation: false,
            interactive: false,
        };
        assert_eq!(Some(Comparison::Greater), bar.comparison());
        assert_eq!("un tiers", bar.strips[0].words());

        let sum = FractionStrip {
            denominator: 8,
            numerators: vec![2, 3],
        };
        assert_eq!(5, sum.colored());
        assert_eq!("5/8", sum.fraction().to_string());
        assert_eq!("cinq huitièmes", sum.words());
    }

    #[test]
    fn a_bar_model_gives_its_calculation() {
        // Tom has 35, Léa 12 more: 35 + 12 = 47.
        let compare = BarModel {
            kind: BarKind::Compare {
                small: 35,
                difference: 12,
            },
            unknown: BarUnknown::Whole,
            labels: vec!["Tom".into(), "Léa".into()],
            solve: true,
            interactive: false,
        };
        assert_eq!(47, compare.answer());
        let (equality, missing) = compare.solution();
        assert_eq!(
            (35, Operator::Add, 12, 47),
            (
                equality.left,
                equality.operator,
                equality.right,
                equality.result
            )
        );
        assert_eq!(None, missing);

        // 12 helmets, 5 fall: 12 − 5 = 7.
        let parts = BarModel {
            kind: BarKind::Parts { parts: vec![5, 7] },
            unknown: BarUnknown::Part(1),
            labels: Vec::new(),
            solve: true,
            interactive: true,
        };
        assert_eq!(12, parts.whole());
        assert_eq!(7, parts.answer());
        let (equality, _) = parts.solution();
        assert_eq!(
            (12, Operator::Subtract, 5, 7),
            (
                equality.left,
                equality.operator,
                equality.right,
                equality.result
            )
        );
        assert_eq!(3, parts.values().len());
        assert!(parts.values()[2].unknown);

        // 4 boxes of 6 tires: 4 × 6 = 24; 24 tires for 4 cars: 4 × ? = 24.
        let groups = BarModel {
            kind: BarKind::Groups { groups: 4, size: 6 },
            unknown: BarUnknown::Whole,
            labels: Vec::new(),
            solve: true,
            interactive: false,
        };
        assert_eq!(24, groups.answer());
        let share = BarModel {
            unknown: BarUnknown::Part(0),
            ..groups
        };
        assert_eq!(6, share.answer());
        assert_eq!(Some(EquationTerm::Right), share.solution().1);
    }
}
