//! Contracts of the lesson animations added with the lessons of numeration, ×, fractions and
//! problems (SUB000117): every value shown is computed by `educator-core::lessons`.

use super::{ComparisonSign, EqualityResponse, EquationTerm, LessonOperator};
use educator_core::exercises::Comparison;
use educator_core::lessons::{self, BlockChange, BlockStepKind, Rank};
use serde::Serialize;

pub(super) fn sign_of(comparison: Comparison) -> ComparisonSign {
    match comparison {
        Comparison::Less => ComparisonSign::Less,
        Comparison::Greater => ComparisonSign::Greater,
        Comparison::Equal => ComparisonSign::Equal,
    }
}

fn count(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

// ── tensUnits ──────────────────────────────────────────────────────────────────────────────

/// Plates of a hundred, bars of ten, unit cubes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct BlocksResponse {
    pub hundreds: u32,
    pub tens: u32,
    pub units: u32,
    pub value: u32,
}

impl From<lessons::Blocks> for BlocksResponse {
    fn from(blocks: lessons::Blocks) -> Self {
        Self {
            hundreds: blocks.hundreds,
            tens: blocks.tens,
            units: blocks.units,
            value: blocks.value(),
        }
    }
}

/// What a step of `tensUnits` shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum BlockStepName {
    Start,
    Join,
    ExchangeUnits,
    ExchangeTens,
    BreakTen,
    BreakHundred,
    TakeAway,
    Share,
}

/// A state of the blocks: `blocks`, with `highlight` drawn in racing red (the blocks that come,
/// the new bar or cubes) or in dotted lines (the blocks taken away).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct BlockStepResponse {
    pub kind: BlockStepName,
    pub blocks: BlocksResponse,
    pub highlight: BlocksResponse,
}

/// What the child does on an interactive `tensUnits`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum BlockTask {
    /// Build the target from nothing.
    Build,
    /// Add blocks to the start until the target (exchange 10 cubes for a bar if needed).
    Add,
    /// Remove blocks from the start until the target (break a bar if needed).
    Remove,
    /// Distribute the blocks of the start in `parts` equal parts.
    Share,
}

pub(super) fn tens_units_response(t: &lessons::TensUnits) -> super::AnimationResponse {
    let start = lessons::Blocks::of(t.value);
    super::AnimationResponse::TensUnits {
        value: t.value,
        hundreds: start.hundreds,
        tens: start.tens,
        units: start.units,
        missing: t.missing(),
        steps: t
            .steps()
            .into_iter()
            .map(|s| BlockStepResponse {
                kind: match s.kind {
                    BlockStepKind::Start => BlockStepName::Start,
                    BlockStepKind::Join => BlockStepName::Join,
                    BlockStepKind::ExchangeUnits => BlockStepName::ExchangeUnits,
                    BlockStepKind::ExchangeTens => BlockStepName::ExchangeTens,
                    BlockStepKind::BreakTen => BlockStepName::BreakTen,
                    BlockStepKind::BreakHundred => BlockStepName::BreakHundred,
                    BlockStepKind::TakeAway => BlockStepName::TakeAway,
                    BlockStepKind::Share => BlockStepName::Share,
                },
                blocks: s.blocks.into(),
                highlight: s.highlight.into(),
            })
            .collect(),
        change: t.change.map(|c| match c {
            BlockChange::Add(n) => n,
            BlockChange::Remove(n) => n,
            BlockChange::Share(parts) => parts,
        }),
        result: t.result(),
        parts: t.parts(),
        interactive: t.interactive,
        task: t.interactive.then_some(match t.change {
            None => BlockTask::Build,
            Some(BlockChange::Add(_)) => BlockTask::Add,
            Some(BlockChange::Remove(_)) => BlockTask::Remove,
            Some(BlockChange::Share(_)) => BlockTask::Share,
        }),
        interactive_start: t.interactive_start().into(),
        target: t.target().into(),
    }
}

// ── compareNumbers, placeValueTable ────────────────────────────────────────────────────────

/// A rank of the decimal writing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum RankName {
    Units,
    Tens,
    Hundreds,
    Thousands,
}

impl From<Rank> for RankName {
    fn from(rank: Rank) -> Self {
        match rank {
            Rank::Units => RankName::Units,
            Rank::Tens => RankName::Tens,
            Rank::Hundreds => RankName::Hundreds,
            Rank::Thousands => RankName::Thousands,
        }
    }
}

/// The digits of the two compared numbers at one rank (null: the number is too short).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct RankDigitsResponse {
    pub rank: RankName,
    pub a: Option<u8>,
    pub b: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct CompareNumbersResponse {
    pub a: u32,
    pub b: u32,
    pub sign: ComparisonSign,
    /// From the highest rank (on the left) to the units.
    pub ranks: Vec<RankDigitsResponse>,
    /// The first rank, from the left, where the digits differ (null: equal numbers).
    pub decisive_rank: Option<RankName>,
    /// The child picks the sign.
    pub interactive: bool,
}

impl From<&lessons::CompareNumbers> for CompareNumbersResponse {
    fn from(c: &lessons::CompareNumbers) -> Self {
        Self {
            a: c.a,
            b: c.b,
            sign: sign_of(c.sign()),
            ranks: c
                .ranks()
                .into_iter()
                .map(|rank| RankDigitsResponse {
                    rank: rank.into(),
                    a: lessons::digit_of(c.a, rank),
                    b: lessons::digit_of(c.b, rank),
                })
                .collect(),
            decisive_rank: c.decisive_rank().map(Into::into),
            interactive: c.interactive,
        }
    }
}

/// « 347 : 34 dizaines ».
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct RankCountResponse {
    pub rank: RankName,
    pub count: u32,
}

/// The number in words, traditional spelling and 1990 one (may be the same).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SpellingsResponse {
    pub traditional: String,
    pub rectified: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct PlaceValueTableResponse {
    pub value: u32,
    /// The columns, from the left.
    pub ranks: Vec<RankName>,
    /// The digit in each column (null: nothing on the left of the number).
    pub digits: Vec<Option<u8>>,
    pub focus: Option<RankName>,
    pub count_of: Option<RankCountResponse>,
    /// 300 + 40 + 7 (empty when not shown).
    pub decomposition: Vec<u32>,
    pub words: Option<SpellingsResponse>,
    /// The child writes the number, digit by digit from the left.
    pub interactive: bool,
}

impl From<&lessons::PlaceValueTable> for PlaceValueTableResponse {
    fn from(t: &lessons::PlaceValueTable) -> Self {
        Self {
            value: t.value,
            ranks: t.ranks().into_iter().map(Into::into).collect(),
            digits: t.digits(),
            focus: t.focus.map(Into::into),
            count_of: t.count_of.map(|rank| RankCountResponse {
                rank: rank.into(),
                count: t.count(rank),
            }),
            decomposition: if t.decomposition {
                t.decomposition_terms()
            } else {
                Vec::new()
            },
            words: t.words.then(|| {
                let (traditional, rectified) = t.spellings();
                SpellingsResponse {
                    traditional,
                    rectified,
                }
            }),
            interactive: t.interactive,
        }
    }
}

// ── sequencePattern ────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum SequenceMode {
    /// A counting sequence with hidden numbers.
    Count,
    /// Numbers to put in order.
    Order,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SequencePatternResponse {
    pub mode: SequenceMode,
    /// Count: the sequence. Order: the numbers as given (not in order).
    pub items: Vec<i32>,
    /// Count: the sequence. Order: the numbers in the order asked.
    pub ordered: Vec<i32>,
    /// Count: the positions (from 0) hidden in a box, then revealed.
    pub hidden: Vec<u32>,
    /// Count: the step (negative: backwards).
    pub step: i32,
    /// Order: from the largest.
    pub descending: bool,
    /// Order: the child clicks the numbers in order.
    pub interactive: bool,
}

impl From<&lessons::SequencePattern> for SequencePatternResponse {
    fn from(s: &lessons::SequencePattern) -> Self {
        // Every number is between 0 and 1 000 000 (checked by the content loader).
        let ordered: Vec<i32> = s.values().into_iter().map(|v| v as i32).collect();
        match s {
            lessons::SequencePattern::Count { step, hidden, .. } => Self {
                mode: SequenceMode::Count,
                items: ordered.clone(),
                ordered,
                hidden: hidden.iter().map(|h| count(*h)).collect(),
                step: *step,
                descending: false,
                interactive: false,
            },
            lessons::SequencePattern::Order {
                values,
                descending,
                interactive,
            } => Self {
                mode: SequenceMode::Order,
                items: values.iter().map(|v| *v as i32).collect(),
                ordered,
                hidden: Vec::new(),
                step: 0,
                descending: *descending,
                interactive: *interactive,
            },
        }
    }
}

// ── arrayGrid ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct GridPartResponse {
    pub rows: u32,
    pub cols: u32,
    pub product: u32,
}

/// What the child does on an interactive grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum GridTask {
    /// Place the rows one by one.
    Build,
    /// Click where to cut the grid.
    Split,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct ArrayGridResponse {
    pub rows: u32,
    pub cols: u32,
    pub total: u32,
    /// The rows after which the grid is cut.
    pub split: Option<u32>,
    /// From the top: one part, or two when cut.
    pub parts: Vec<GridPartResponse>,
    /// The rows added one by one (5 + 5 + 5); empty when not shown.
    pub repeated: Vec<u32>,
    pub rotate: bool,
    pub removed: u32,
    pub remaining: u32,
    pub task: Option<GridTask>,
}

impl From<&lessons::ArrayGrid> for ArrayGridResponse {
    fn from(g: &lessons::ArrayGrid) -> Self {
        Self {
            rows: g.rows,
            cols: g.cols,
            total: g.total(),
            split: g.split,
            parts: g
                .parts()
                .into_iter()
                .map(|p| GridPartResponse {
                    rows: p.rows,
                    cols: p.cols,
                    product: p.product,
                })
                .collect(),
            repeated: g.repeated_addition(),
            rotate: g.rotate,
            removed: g.removed,
            remaining: g.remaining(),
            task: g.interactive.then_some(if g.split.is_some() {
                GridTask::Split
            } else {
                GridTask::Build
            }),
        }
    }
}

// ── timesTable ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum TableHighlightKind {
    Row,
    Doubles,
    Result,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct TableHighlightResponse {
    pub kind: TableHighlightKind,
    pub value: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct TableCellResponse {
    pub row: u32,
    pub col: u32,
    pub value: u32,
    /// Index of the highlight group (in `highlights`) that lights the cell.
    pub group: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct TimesTableResponse {
    pub operator: LessonOperator,
    pub size: u32,
    /// Rows from 1 to `size`.
    pub cells: Vec<Vec<TableCellResponse>>,
    pub highlights: Vec<TableHighlightResponse>,
}

impl From<&lessons::TimesTable> for TimesTableResponse {
    fn from(t: &lessons::TimesTable) -> Self {
        Self {
            operator: t.operator.into(),
            size: t.size,
            cells: t
                .cells()
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|c| TableCellResponse {
                            row: c.row,
                            col: c.col,
                            value: c.value,
                            group: c.group.map(count),
                        })
                        .collect()
                })
                .collect(),
            highlights: t
                .highlights
                .iter()
                .map(|h| match h {
                    lessons::TableHighlight::Row(n) => TableHighlightResponse {
                        kind: TableHighlightKind::Row,
                        value: Some(*n),
                    },
                    lessons::TableHighlight::Doubles => TableHighlightResponse {
                        kind: TableHighlightKind::Doubles,
                        value: None,
                    },
                    lessons::TableHighlight::Result(n) => TableHighlightResponse {
                        kind: TableHighlightKind::Result,
                        value: Some(*n),
                    },
                })
                .collect(),
        }
    }
}

// ── groupObjects ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum GroupModeName {
    Join,
    Remove,
    GroupBy,
    Share,
    SortParity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum RaceObjectName {
    Car,
    Helmet,
    Tire,
    Flag,
    Cone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct GroupingResponse {
    pub count: u32,
    pub groups: u32,
    pub rest: u32,
}

/// The parameters that the mode does not use are 0 or empty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct GroupObjectsResponse {
    pub mode: GroupModeName,
    pub object: RaceObjectName,
    /// Join: the two groups. GroupBy: the collections.
    pub counts: Vec<u32>,
    /// Remove, share: the objects at the start.
    pub total: u32,
    pub removed: u32,
    /// GroupBy: size of a group. Share: number of parts.
    pub size: u32,
    /// The total, what remains, one part.
    pub result: u32,
    pub groupings: Vec<GroupingResponse>,
    /// SortParity: the numbers and whether each one is even.
    pub numbers: Vec<u32>,
    pub parities: Vec<bool>,
    /// Join: the groups swap places.
    pub commute: bool,
    pub interactive: bool,
    /// Interactive join or removal: proposals for the result.
    pub choices: Vec<u32>,
}

impl From<&lessons::GroupObjects> for GroupObjectsResponse {
    fn from(g: &lessons::GroupObjects) -> Self {
        use lessons::GroupMode as M;
        let (mode, counts, total, removed, size, numbers, commute) = match &g.mode {
            M::Join { counts, commute } => (
                GroupModeName::Join,
                counts.to_vec(),
                0,
                0,
                0,
                Vec::new(),
                *commute,
            ),
            M::Remove { total, removed } => (
                GroupModeName::Remove,
                Vec::new(),
                *total,
                *removed,
                0,
                Vec::new(),
                false,
            ),
            M::GroupBy { counts, size } => (
                GroupModeName::GroupBy,
                counts.clone(),
                0,
                0,
                *size,
                Vec::new(),
                false,
            ),
            M::Share { total, parts } => (
                GroupModeName::Share,
                Vec::new(),
                *total,
                0,
                *parts,
                Vec::new(),
                false,
            ),
            M::SortParity { numbers } => (
                GroupModeName::SortParity,
                Vec::new(),
                0,
                0,
                0,
                numbers.clone(),
                false,
            ),
        };
        Self {
            mode,
            object: match g.object {
                lessons::RaceObject::Car => RaceObjectName::Car,
                lessons::RaceObject::Helmet => RaceObjectName::Helmet,
                lessons::RaceObject::Tire => RaceObjectName::Tire,
                lessons::RaceObject::Flag => RaceObjectName::Flag,
                lessons::RaceObject::Cone => RaceObjectName::Cone,
            },
            counts,
            total,
            removed,
            size,
            result: g.result(),
            groupings: g
                .groupings()
                .into_iter()
                .map(|x| GroupingResponse {
                    count: x.count,
                    groups: x.groups,
                    rest: x.rest,
                })
                .collect(),
            numbers,
            parities: g.parities(),
            commute,
            interactive: g.interactive,
            choices: g.choices(),
        }
    }
}

// ── tenFrame ───────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum FrameSlotName {
    Empty,
    Filled,
    Added,
    Removed,
}

/// Two friends of 10: `a + b = 10`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct FriendPairResponse {
    pub a: u32,
    pub b: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct TenFrameResponse {
    pub frames: u32,
    /// The places, garage after garage, in reading order (2 rows of 5).
    pub slots: Vec<FrameSlotName>,
    pub filled: u32,
    pub added: u32,
    pub removed: u32,
    /// The cars in the garages at the end.
    pub total: u32,
    pub friends: Vec<FriendPairResponse>,
    /// The child parks the added cars, or makes the removed ones leave.
    pub interactive: bool,
}

impl From<&lessons::TenFrame> for TenFrameResponse {
    fn from(f: &lessons::TenFrame) -> Self {
        Self {
            frames: f.frames,
            slots: f
                .slots()
                .into_iter()
                .map(|s| match s {
                    lessons::FrameSlot::Empty => FrameSlotName::Empty,
                    lessons::FrameSlot::Filled => FrameSlotName::Filled,
                    lessons::FrameSlot::Added => FrameSlotName::Added,
                    lessons::FrameSlot::Removed => FrameSlotName::Removed,
                })
                .collect(),
            filled: f.filled,
            added: f.added,
            removed: f.removed,
            total: f.total(),
            friends: f
                .friend_pairs()
                .into_iter()
                .map(|(a, b)| FriendPairResponse { a, b })
                .collect(),
            interactive: f.interactive,
        }
    }
}

// ── fractionBar ────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct FractionStripResponse {
    pub denominator: u32,
    /// Colored parts, one color per number.
    pub numerators: Vec<u32>,
    pub colored: u32,
    /// « 3/4 ».
    pub label: String,
    /// « trois quarts ».
    pub words: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct FractionBarResponse {
    pub bars: Vec<FractionStripResponse>,
    /// Two bars: the sign between their colored fractions.
    pub comparison: Option<ComparisonSign>,
    pub unequal: bool,
    pub notation: bool,
    /// The child colors the parts of the only bar.
    pub interactive: bool,
}

impl From<&lessons::FractionBar> for FractionBarResponse {
    fn from(f: &lessons::FractionBar) -> Self {
        Self {
            bars: f
                .strips
                .iter()
                .map(|s| FractionStripResponse {
                    denominator: s.denominator,
                    numerators: s.numerators.clone(),
                    colored: s.colored(),
                    label: s.fraction().to_string(),
                    words: s.words(),
                })
                .collect(),
            comparison: f.comparison().map(sign_of),
            unequal: f.unequal,
            notation: f.notation,
            interactive: f.interactive,
        }
    }
}

// ── barModel ───────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum BarKindName {
    Parts,
    Compare,
    Groups,
}

/// A value of the diagram: the whole (`part` null) or a part (compare: 0 the smaller quantity,
/// 1 the difference; groups: 0 the size of a group).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct BarValueResponse {
    pub part: Option<u32>,
    pub value: u32,
    /// Shown as « ? » until solved.
    pub unknown: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct BarModelResponse {
    pub kind: BarKindName,
    /// The whole first, then the parts.
    pub values: Vec<BarValueResponse>,
    /// Groups: the number of equal parts.
    pub groups: u32,
    pub labels: Vec<String>,
    pub answer: u32,
    /// The calculation that finds the « ? ».
    pub solution: EqualityResponse,
    /// For « 4 × ? = 24 »: the term hidden in the solution.
    pub solution_missing: Option<EquationTerm>,
    pub solve: bool,
    /// The child places the known numbers on the diagram.
    pub interactive: bool,
}

impl From<&lessons::BarModel> for BarModelResponse {
    fn from(m: &lessons::BarModel) -> Self {
        let (solution, missing) = m.solution();
        Self {
            kind: match m.kind {
                lessons::BarKind::Parts { .. } => BarKindName::Parts,
                lessons::BarKind::Compare { .. } => BarKindName::Compare,
                lessons::BarKind::Groups { .. } => BarKindName::Groups,
            },
            values: m
                .values()
                .into_iter()
                .map(|v| BarValueResponse {
                    part: match v.role {
                        lessons::BarUnknown::Whole => None,
                        lessons::BarUnknown::Part(i) => Some(count(i)),
                    },
                    value: v.value,
                    unknown: v.unknown,
                })
                .collect(),
            groups: match m.kind {
                lessons::BarKind::Groups { groups, .. } => groups,
                _ => 0,
            },
            labels: m.labels.clone(),
            answer: m.answer(),
            solution: EqualityResponse {
                left: solution.left,
                operator: solution.operator.into(),
                right: solution.right,
                result: solution.result,
            },
            solution_missing: missing.map(|term| match term {
                lessons::EquationTerm::Left => EquationTerm::Left,
                lessons::EquationTerm::Right => EquationTerm::Right,
                lessons::EquationTerm::Result => EquationTerm::Result,
            }),
            solve: m.solve,
            interactive: m.interactive,
        }
    }
}
