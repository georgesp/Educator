//! What the front shows for an exercise: the statement as a row of tokens, an optional picture and
//! the kind of input. The front only draws them; every number comes from the engine.

use super::Fraction;

/// One piece of the written statement, e.g. `7`, `+`, `?`, `=`, `12`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Number(i32),
    /// `+`, `−` (U+2212), `×`, `=`, `<`, `>`.
    Sign(String),
    Text(String),
    Fraction(Fraction),
    /// Where the child answers. Two slots for a framing (`… < 347 < …`), numbered from 0.
    Slot(usize),
}

/// Shape of a fraction picture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FractionShape {
    Bar,
    Disc,
}

/// A segment of a bar model: a value, or an unknown shown as `?`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BarSegment {
    pub value: Option<i32>,
}

/// Pictures drawn by the front (SUB000117 animation library: `tensUnits`, `fractionBar`,
/// `numberLine`, `arrayGrid`, `barModel`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Picture {
    /// Hundred squares, ten bars and unit cubes.
    BaseTen {
        hundreds: u32,
        tens: u32,
        units: u32,
    },
    /// A bar or a disc cut into `parts` equal parts, the first `colored` ones colored.
    FractionShape {
        shape: FractionShape,
        parts: u32,
        colored: u32,
    },
    /// A graduated half-line: `ticks` graduations from `start`, every `step`. Graduations listed in
    /// `labeled` (indexes) show their number; `arrow` points at a graduation.
    NumberLine {
        start: i32,
        step: i32,
        ticks: u32,
        labeled: Vec<u32>,
        arrow: Option<u32>,
    },
    /// `rows` rows of `columns` dots; `split`: a line after that column (13 × 7 = 10 × 7 + 3 × 7).
    ArrayGrid {
        rows: u32,
        columns: u32,
        split: Option<u32>,
    },
    /// Bars of segments, one under the other, and the total of the first bar when known.
    BarModel {
        bars: Vec<Vec<BarSegment>>,
        total: Option<i32>,
    },
    /// `groups` groups of `per_group` objects (4 boxes of 6).
    Groups { groups: u32, per_group: u32 },
}

/// How the child answers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Input {
    /// Types numbers in the slots (on-screen keypad or keyboard): one slot, or two for a framing.
    Number { slots: usize },
    /// Types a numerator and a denominator.
    Fraction,
    /// Three big buttons `<`, `>`, `=`.
    Comparison,
    /// Two big buttons Oui / Non.
    YesNo,
    /// Puts the numbers in order (drag and drop), shown first in this order.
    Order { items: Vec<i32>, ascending: bool },
    /// Picks every right number among these.
    MultiChoice { items: Vec<i32> },
    /// Picks one of the proposals ([`super::Exercise::choices`]).
    Pick,
    /// Clicks a graduation of the number line of the picture.
    NumberLine,
}
