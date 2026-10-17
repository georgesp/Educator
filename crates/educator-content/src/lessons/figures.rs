//! File shapes and validation of the animations added for the lessons of numeration, ×,
//! fractions and problems (SUB000117): `tensUnits` with a change, `compareNumbers`,
//! `sequencePattern`, `arrayGrid`, `timesTable`, `placeValueTable`, `groupObjects`, `tenFrame`,
//! `fractionBar` and `barModel`. Each function returns the animation or the first error found.

use super::{ensure, MAX_NUMBER};
use educator_core::lessons::{
    Animation, ArrayGrid, BarKind, BarModel, BarUnknown, BlockChange, CompareNumbers, FractionBar,
    FractionStrip, GroupMode, GroupObjects, Operator, PlaceValueTable, RaceObject, Rank,
    SequencePattern, TableHighlight, TenFrame, TensUnits, TimesTable,
};
use serde::Deserialize;
use std::collections::HashSet;

/// Largest number of base ten blocks (9 plates, 9 bars, 9 cubes).
pub const MAX_BLOCKS: i64 = 999;
/// Objects drawn one by one: beyond, the child cannot count them.
pub const MAX_OBJECTS: i64 = 20;

/// Objects of an interactive grouping by 10: up to 3 packets of 10 and some left over.
pub const MAX_GROUPED_BY_TEN: i64 = 39;
/// Cars of a starting grid.
pub const MAX_GRID_CARS: i64 = 200;

fn in_range(n: i64) -> bool {
    (0..=i64::from(MAX_NUMBER)).contains(&n)
}

fn to_u32(n: i64) -> u32 {
    u32::try_from(n).unwrap_or(0)
}

// ── tensUnits ──────────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TensUnitsFile {
    value: i64,
    #[serde(default)]
    complete_ten: bool,
    #[serde(default)]
    add: Option<i64>,
    #[serde(default)]
    remove: Option<i64>,
    #[serde(default)]
    share: Option<i64>,
    #[serde(default)]
    interactive: bool,
}

pub fn tens_units(file: TensUnitsFile) -> Result<Animation, String> {
    ensure(
        (1..=MAX_BLOCKS).contains(&file.value),
        "tensUnits: value from 1 to 999.",
    )?;
    let changes = [file.add, file.remove, file.share]
        .iter()
        .filter(|c| c.is_some())
        .count();
    ensure(
        changes <= 1,
        "tensUnits: add, remove or share, not two of them.",
    )?;
    ensure(
        !file.complete_ten || (file.value % 10 != 0 && changes == 0),
        "tensUnits: completeTen needs a value that is not a whole ten, and no change.",
    )?;
    let change = if let Some(n) = file.add {
        ensure(
            n >= 1 && file.value + n <= MAX_BLOCKS,
            "tensUnits: add at least 1, the sum up to 999.",
        )?;
        Some(BlockChange::Add(to_u32(n)))
    } else if let Some(n) = file.remove {
        ensure(
            (1..=file.value).contains(&n),
            "tensUnits: remove from 1 to the value.",
        )?;
        Some(BlockChange::Remove(to_u32(n)))
    } else if let Some(parts) = file.share {
        ensure(
            (2..=4).contains(&parts) && file.value % parts == 0,
            "tensUnits: share in 2 to 4 equal parts (the value divides).",
        )?;
        Some(BlockChange::Share(to_u32(parts)))
    } else {
        None
    };
    ensure(
        !file.interactive || !file.complete_ten,
        "tensUnits: an interactive slide has no completeTen.",
    )?;
    Ok(Animation::TensUnits(TensUnits {
        value: to_u32(file.value),
        complete_ten: file.complete_ten,
        change,
        interactive: file.interactive,
    }))
}

// ── compareNumbers ─────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompareNumbersFile {
    a: i64,
    b: i64,
    #[serde(default)]
    interactive: bool,
}

pub fn compare_numbers(file: CompareNumbersFile) -> Result<Animation, String> {
    // The front shows four ranks at most (thousands to units).
    ensure(
        (0..=9_999).contains(&file.a) && (0..=9_999).contains(&file.b),
        "compareNumbers: numbers between 0 and 9 999.",
    )?;
    Ok(Animation::CompareNumbers(CompareNumbers {
        a: to_u32(file.a),
        b: to_u32(file.b),
        interactive: file.interactive,
    }))
}

// ── sequencePattern ────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SequencePatternFile {
    #[serde(default)]
    start: Option<i64>,
    #[serde(default)]
    step: Option<i64>,
    #[serde(default)]
    count: Option<i64>,
    #[serde(default)]
    hidden: Vec<i64>,
    #[serde(default)]
    values: Option<Vec<i64>>,
    #[serde(default)]
    descending: bool,
    #[serde(default)]
    interactive: bool,
}

pub fn sequence_pattern(file: SequencePatternFile) -> Result<Animation, String> {
    if let Some(values) = file.values {
        ensure(
            file.start.is_none()
                && file.step.is_none()
                && file.count.is_none()
                && file.hidden.is_empty(),
            "sequencePattern: values (to order) or start, step and count (to count), not both.",
        )?;
        let distinct: HashSet<i64> = values.iter().copied().collect();
        ensure(
            (3..=6).contains(&values.len()) && distinct.len() == values.len(),
            "sequencePattern: 3 to 6 different values to order.",
        )?;
        ensure(
            values.iter().all(|v| in_range(*v)),
            "sequencePattern: values between 0 and 1 000 000.",
        )?;
        let sorted = {
            let mut s = values.clone();
            s.sort_unstable();
            s
        };
        ensure(
            sorted != values && sorted.iter().rev().copied().collect::<Vec<_>>() != values,
            "sequencePattern: the values must not be already in order.",
        )?;
        return Ok(Animation::SequencePattern(SequencePattern::Order {
            values: values.into_iter().map(to_u32).collect(),
            descending: file.descending,
            interactive: file.interactive,
        }));
    }

    let (Some(start), Some(step), Some(count)) = (file.start, file.step, file.count) else {
        return Err(
            "sequencePattern: values (to order) or start, step and count (to count).".to_owned(),
        );
    };
    ensure(
        !file.interactive && !file.descending,
        "sequencePattern: interactive and descending only to order values.",
    )?;
    ensure(
        step != 0 && (3..=10).contains(&count),
        "sequencePattern: a step other than 0, 3 to 10 numbers.",
    )?;
    ensure(
        in_range(start) && in_range(start + (count - 1) * step),
        "sequencePattern: every number between 0 and 1 000 000.",
    )?;
    let distinct: HashSet<i64> = file.hidden.iter().copied().collect();
    ensure(
        distinct.len() == file.hidden.len()
            && file.hidden.iter().all(|h| (0..count).contains(h))
            && (file.hidden.len() as i64) < count,
        "sequencePattern: hidden positions between 0 and count − 1, at least one number shown.",
    )?;
    Ok(Animation::SequencePattern(SequencePattern::Count {
        start: start as i32,
        step: step as i32,
        count: count as usize,
        hidden: file.hidden.into_iter().map(|h| h as usize).collect(),
    }))
}

// ── arrayGrid ──────────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArrayGridFile {
    rows: i64,
    cols: i64,
    #[serde(default)]
    split: Option<i64>,
    #[serde(default)]
    rotate: bool,
    #[serde(default)]
    show_sum: bool,
    #[serde(default)]
    removed: i64,
    #[serde(default)]
    interactive: bool,
}

pub fn array_grid(file: ArrayGridFile) -> Result<Animation, String> {
    ensure(
        (1..=20).contains(&file.rows)
            && (1..=10).contains(&file.cols)
            && file.rows * file.cols <= MAX_GRID_CARS,
        "arrayGrid: 1 to 20 rows of 1 to 10 cars, 200 cars at most.",
    )?;
    ensure(
        file.split.is_none_or(|s| (1..file.rows).contains(&s)),
        "arrayGrid: split between 1 and rows − 1.",
    )?;
    ensure(
        !file.rotate || (file.split.is_none() && !file.show_sum && file.removed == 0),
        "arrayGrid: a rotating grid is not split, summed or emptied.",
    )?;
    ensure(
        file.split.is_none() || (!file.show_sum && file.removed == 0),
        "arrayGrid: a split grid is not summed or emptied.",
    )?;
    ensure(
        !file.show_sum || file.removed == 0,
        "arrayGrid: showSum or removed, not both.",
    )?;
    ensure(
        !file.show_sum || file.rows <= 10,
        "arrayGrid: showSum for 10 rows at most.",
    )?;
    ensure(
        (0..=file.rows * file.cols).contains(&file.removed),
        "arrayGrid: removed between 0 and the number of cars.",
    )?;
    if file.interactive {
        ensure(
            file.removed == 0 && !file.rotate && !file.show_sum,
            "arrayGrid: an interactive grid is not rotated, summed or emptied.",
        )?;
        ensure(
            file.split.is_some() || file.rows <= 10,
            "arrayGrid: the child places 10 rows at most.",
        )?;
    }
    Ok(Animation::ArrayGrid(ArrayGrid {
        rows: to_u32(file.rows),
        cols: to_u32(file.cols),
        split: file.split.map(to_u32),
        rotate: file.rotate,
        show_sum: file.show_sum,
        removed: to_u32(file.removed),
        interactive: file.interactive,
    }))
}

// ── timesTable ─────────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HighlightFile {
    kind: String,
    #[serde(default)]
    value: Option<i64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TimesTableFile {
    operation: String,
    size: i64,
    highlights: Vec<HighlightFile>,
}

pub fn times_table(file: TimesTableFile) -> Result<Animation, String> {
    let operator = match file.operation.as_str() {
        "add" => Operator::Add,
        "multiply" => Operator::Multiply,
        other => {
            return Err(format!(
                "timesTable: unknown operation '{other}' (add or multiply)."
            ))
        }
    };
    ensure(
        (5..=10).contains(&file.size),
        "timesTable: size from 5 to 10.",
    )?;
    ensure(
        (1..=3).contains(&file.highlights.len()),
        "timesTable: 1 to 3 highlights.",
    )?;
    let mut highlights = Vec::new();
    for highlight in file.highlights {
        let h = match (highlight.kind.as_str(), highlight.value) {
            ("row", Some(n)) if (1..=file.size).contains(&n) => TableHighlight::Row(to_u32(n)),
            ("doubles", None) => TableHighlight::Doubles,
            ("result", Some(n)) if operator == Operator::Add && (2..=2 * file.size).contains(&n) => {
                TableHighlight::Result(to_u32(n))
            }
            _ => {
                return Err(
                    "timesTable: a highlight is a row (value from 1 to size), the doubles, or a result of the addition table."
                        .to_owned(),
                )
            }
        };
        highlights.push(h);
    }
    Ok(Animation::TimesTable(TimesTable {
        operator,
        size: to_u32(file.size),
        highlights,
    }))
}

// ── placeValueTable ────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlaceValueTableFile {
    value: i64,
    #[serde(default)]
    focus: Option<String>,
    #[serde(default)]
    count_of: Option<String>,
    #[serde(default)]
    decomposition: bool,
    #[serde(default)]
    words: bool,
    #[serde(default)]
    interactive: bool,
}

fn rank(name: &str) -> Result<Rank, String> {
    match name {
        "units" => Ok(Rank::Units),
        "tens" => Ok(Rank::Tens),
        "hundreds" => Ok(Rank::Hundreds),
        "thousands" => Ok(Rank::Thousands),
        other => Err(format!(
            "placeValueTable: unknown rank '{other}' (units, tens, hundreds, thousands)."
        )),
    }
}

pub fn place_value_table(file: PlaceValueTableFile) -> Result<Animation, String> {
    ensure(
        (1..=9_999).contains(&file.value),
        "placeValueTable: value from 1 to 9 999.",
    )?;
    let focus = file.focus.as_deref().map(rank).transpose()?;
    let count_of = file.count_of.as_deref().map(rank).transpose()?;
    let table = PlaceValueTable {
        value: to_u32(file.value),
        focus,
        count_of,
        decomposition: file.decomposition,
        words: file.words,
        interactive: file.interactive,
    };
    ensure(
        !file.interactive || (count_of.is_none() && !file.decomposition && !file.words),
        "placeValueTable: an interactive table has no countOf, decomposition or words.",
    )?;
    let ranks = table.ranks();
    ensure(
        focus.is_none_or(|r| ranks.contains(&r)),
        "placeValueTable: focus on a rank of the table.",
    )?;
    ensure(
        count_of.is_none_or(|r| r != Rank::Units && ranks.contains(&r) && table.count(r) > 0),
        "placeValueTable: countOf a rank above the units that the number has.",
    )?;
    Ok(Animation::PlaceValueTable(table))
}

// ── groupObjects ───────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GroupObjectsFile {
    mode: String,
    #[serde(default)]
    object: Option<String>,
    #[serde(default)]
    counts: Vec<i64>,
    #[serde(default)]
    total: Option<i64>,
    #[serde(default)]
    removed: Option<i64>,
    #[serde(default)]
    size: Option<i64>,
    #[serde(default)]
    parts: Option<i64>,
    #[serde(default)]
    numbers: Vec<i64>,
    #[serde(default)]
    commute: bool,
    #[serde(default)]
    interactive: bool,
}

pub fn group_objects(file: GroupObjectsFile) -> Result<Animation, String> {
    let object = match file.object.as_deref() {
        None | Some("car") => RaceObject::Car,
        Some("helmet") => RaceObject::Helmet,
        Some("tire") => RaceObject::Tire,
        Some("flag") => RaceObject::Flag,
        Some("cone") => RaceObject::Cone,
        Some(other) => {
            return Err(format!(
                "groupObjects: unknown object '{other}' (car, helmet, tire, flag, cone)."
            ))
        }
    };
    let drawable = |n: i64| (1..=MAX_OBJECTS).contains(&n);
    let unused = |ok: bool| ensure(ok, "groupObjects: a parameter that this mode does not use.");
    let mode = match file.mode.as_str() {
        "join" => {
            unused(
                file.total.is_none()
                    && file.removed.is_none()
                    && file.size.is_none()
                    && file.parts.is_none()
                    && file.numbers.is_empty(),
            )?;
            ensure(
                file.counts.len() == 2
                    && file.counts.iter().all(|c| drawable(*c))
                    && drawable(file.counts[0] + file.counts[1]),
                "groupObjects join: 2 counts, 20 objects at most in all.",
            )?;
            GroupMode::Join {
                counts: [to_u32(file.counts[0]), to_u32(file.counts[1])],
                commute: file.commute,
            }
        }
        "remove" => {
            unused(
                file.counts.is_empty()
                    && file.size.is_none()
                    && file.parts.is_none()
                    && file.numbers.is_empty()
                    && !file.commute,
            )?;
            let (Some(total), Some(removed)) = (file.total, file.removed) else {
                return Err("groupObjects remove: total and removed.".to_owned());
            };
            ensure(
                drawable(total) && (1..=total).contains(&removed),
                "groupObjects remove: 1 to 20 objects, 1 to all of them removed.",
            )?;
            GroupMode::Remove {
                total: to_u32(total),
                removed: to_u32(removed),
            }
        }
        "groupBy" => {
            unused(
                file.total.is_none()
                    && file.removed.is_none()
                    && file.parts.is_none()
                    && file.numbers.is_empty()
                    && !file.commute,
            )?;
            let size = file.size.unwrap_or(0);
            ensure(
                (1..=2).contains(&file.counts.len())
                    && file.counts.iter().all(|c| drawable(*c))
                    && (2..=10).contains(&size),
                "groupObjects groupBy: 1 or 2 counts of 1 to 20 objects, groups of 2 to 10.",
            )
            .or_else(|e| {
                // The child makes the packets of 10 himself: up to 3 tens and some units (SUB001410).
                ensure(
                    file.interactive
                        && size == 10
                        && file.counts.len() == 1
                        && (11..=MAX_GROUPED_BY_TEN).contains(&file.counts[0]),
                    &e,
                )
            })?;
            ensure(
                !file.interactive
                    || (size == 10 && file.counts.len() == 1 && file.counts[0] >= size),
                "groupObjects groupBy: the child makes packets of 10, of 1 collection of 10 objects or more.",
            )?;
            GroupMode::GroupBy {
                counts: file.counts.into_iter().map(to_u32).collect(),
                size: to_u32(size),
            }
        }
        "share" => {
            unused(
                file.counts.is_empty()
                    && file.removed.is_none()
                    && file.size.is_none()
                    && file.numbers.is_empty()
                    && !file.commute,
            )?;
            let (Some(total), Some(parts)) = (file.total, file.parts) else {
                return Err("groupObjects share: total and parts.".to_owned());
            };
            ensure(
                (1..=30).contains(&total) && (2..=6).contains(&parts) && total % parts == 0,
                "groupObjects share: 1 to 30 objects in 2 to 6 equal parts.",
            )?;
            GroupMode::Share {
                total: to_u32(total),
                parts: to_u32(parts),
            }
        }
        "sortParity" => {
            unused(
                file.counts.is_empty()
                    && file.total.is_none()
                    && file.removed.is_none()
                    && file.size.is_none()
                    && file.parts.is_none()
                    && !file.commute,
            )?;
            let distinct: HashSet<i64> = file.numbers.iter().copied().collect();
            ensure(
                (3..=6).contains(&file.numbers.len())
                    && distinct.len() == file.numbers.len()
                    && file.numbers.iter().all(|n| in_range(*n)),
                "groupObjects sortParity: 3 to 6 different numbers.",
            )?;
            ensure(
                file.numbers.iter().any(|n| n % 2 == 0) && file.numbers.iter().any(|n| n % 2 == 1),
                "groupObjects sortParity: even and odd numbers.",
            )?;
            GroupMode::SortParity {
                numbers: file.numbers.into_iter().map(to_u32).collect(),
            }
        }
        other => {
            return Err(format!(
                "groupObjects: unknown mode '{other}' (join, remove, groupBy, share, sortParity)."
            ))
        }
    };
    ensure(
        !file.interactive
            || matches!(
                mode,
                GroupMode::Join { .. }
                    | GroupMode::Remove { .. }
                    | GroupMode::GroupBy { .. }
                    | GroupMode::Share { .. }
                    | GroupMode::SortParity { .. }
            ),
        "groupObjects: interactive only to join, remove, groupBy, share or sortParity.",
    )?;
    ensure(
        !(file.interactive && file.commute),
        "groupObjects: commute only on a join shown by itself.",
    )?;
    Ok(Animation::GroupObjects(GroupObjects {
        mode,
        object,
        interactive: file.interactive,
    }))
}

// ── tenFrame ───────────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TenFrameFile {
    #[serde(default)]
    filled: i64,
    #[serde(default)]
    added: i64,
    #[serde(default)]
    removed: i64,
    #[serde(default = "one")]
    frames: i64,
    #[serde(default)]
    friends: bool,
    #[serde(default)]
    interactive: bool,
}

fn one() -> i64 {
    1
}

pub fn ten_frame(file: TenFrameFile) -> Result<Animation, String> {
    ensure((1..=2).contains(&file.frames), "tenFrame: 1 or 2 garages.")?;
    if file.friends {
        ensure(
            file.filled == 0
                && file.added == 0
                && file.removed == 0
                && !file.interactive
                && file.frames == 1,
            "tenFrame: the friends of 10 come alone.",
        )?;
    } else {
        ensure(
            file.filled >= 0 && file.added >= 0 && file.removed >= 0,
            "tenFrame: no negative number of cars.",
        )?;
        ensure(
            file.filled + file.added >= 1 && file.filled + file.added <= 10 * file.frames,
            "tenFrame: 1 car to all the places of the garages.",
        )?;
        ensure(
            file.added == 0 || file.removed == 0,
            "tenFrame: cars come in or leave, not both.",
        )?;
        ensure(
            file.removed <= file.filled,
            "tenFrame: removed up to the parked cars.",
        )?;
        ensure(
            !file.interactive || file.added > 0 || file.removed > 0,
            "tenFrame: an interactive garage needs cars to add or remove.",
        )?;
    }
    Ok(Animation::TenFrame(TenFrame {
        filled: to_u32(file.filled),
        added: to_u32(file.added),
        removed: to_u32(file.removed),
        frames: to_u32(file.frames),
        friends: file.friends,
        interactive: file.interactive,
    }))
}

// ── fractionBar ────────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StripFile {
    denominator: i64,
    #[serde(default)]
    numerators: Vec<i64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FractionBarFile {
    bars: Vec<StripFile>,
    #[serde(default)]
    unequal: bool,
    #[serde(default)]
    notation: bool,
    #[serde(default)]
    interactive: bool,
}

pub fn fraction_bar(file: FractionBarFile) -> Result<Animation, String> {
    ensure(
        (1..=6).contains(&file.bars.len()),
        "fractionBar: 1 to 6 bars.",
    )?;
    let mut strips = Vec::new();
    for bar in &file.bars {
        ensure(
            (1..=12).contains(&bar.denominator)
                && bar.numerators.len() <= 2
                && bar.numerators.iter().all(|n| *n >= 1)
                && bar.numerators.iter().sum::<i64>() <= bar.denominator,
            "fractionBar: 1 to 12 parts, at most 2 colored groups that fit in the bar.",
        )?;
        strips.push(FractionStrip {
            denominator: to_u32(bar.denominator),
            numerators: bar.numerators.iter().map(|n| to_u32(*n)).collect(),
        });
    }
    ensure(
        !file.unequal || (strips.len() == 1 && (2..=4).contains(&strips[0].denominator)),
        "fractionBar: unequal parts on a single bar of 2 to 4 parts.",
    )?;
    if file.interactive {
        ensure(
            strips.len() == 1 && strips[0].colored() >= 1 && !file.unequal,
            "fractionBar: the child colors the parts of a single bar.",
        )?;
    }
    Ok(Animation::FractionBar(FractionBar {
        strips,
        unequal: file.unequal,
        notation: file.notation,
        interactive: file.interactive,
    }))
}

// ── barModel ───────────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BarModelFile {
    kind: String,
    #[serde(default)]
    parts: Vec<i64>,
    #[serde(default)]
    small: Option<i64>,
    #[serde(default)]
    difference: Option<i64>,
    #[serde(default)]
    groups: Option<i64>,
    #[serde(default)]
    size: Option<i64>,
    unknown: String,
    #[serde(default)]
    labels: Vec<String>,
    #[serde(default)]
    solve: bool,
    #[serde(default)]
    interactive: bool,
}

/// Largest value of a bar model.
const MAX_BAR_VALUE: i64 = 1_000;

pub fn bar_model(file: BarModelFile) -> Result<Animation, String> {
    let value = |n: i64| (1..=MAX_BAR_VALUE).contains(&n);
    let (kind, unknown) = match file.kind.as_str() {
        "parts" => {
            ensure(
                (2..=3).contains(&file.parts.len()) && file.parts.iter().all(|p| value(*p)),
                "barModel parts: 2 or 3 parts from 1 to 1 000.",
            )?;
            let unknown = match file.unknown.as_str() {
                "whole" => BarUnknown::Whole,
                "part1" => BarUnknown::Part(0),
                "part2" => BarUnknown::Part(1),
                "part3" if file.parts.len() == 3 => BarUnknown::Part(2),
                other => {
                    return Err(format!(
                        "barModel parts: unknown '{other}' (whole, part1, part2, part3)."
                    ))
                }
            };
            (
                BarKind::Parts {
                    parts: file.parts.iter().map(|p| to_u32(*p)).collect(),
                },
                unknown,
            )
        }
        "compare" => {
            let (Some(small), Some(difference)) = (file.small, file.difference) else {
                return Err("barModel compare: small and difference.".to_owned());
            };
            ensure(
                value(small) && value(difference) && value(small + difference),
                "barModel compare: values from 1 to 1 000.",
            )?;
            let unknown = match file.unknown.as_str() {
                "big" => BarUnknown::Whole,
                "small" => BarUnknown::Part(0),
                "difference" => BarUnknown::Part(1),
                other => {
                    return Err(format!(
                        "barModel compare: unknown '{other}' (big, small, difference)."
                    ))
                }
            };
            (
                BarKind::Compare {
                    small: to_u32(small),
                    difference: to_u32(difference),
                },
                unknown,
            )
        }
        "groups" => {
            let (Some(groups), Some(size)) = (file.groups, file.size) else {
                return Err("barModel groups: groups and size.".to_owned());
            };
            ensure(
                (2..=10).contains(&groups) && value(size) && value(groups * size),
                "barModel groups: 2 to 10 groups, values up to 1 000.",
            )?;
            let unknown = match file.unknown.as_str() {
                "whole" => BarUnknown::Whole,
                "size" => BarUnknown::Part(0),
                other => return Err(format!("barModel groups: unknown '{other}' (whole, size).")),
            };
            (
                BarKind::Groups {
                    groups: to_u32(groups),
                    size: to_u32(size),
                },
                unknown,
            )
        }
        other => {
            return Err(format!(
                "barModel: unknown kind '{other}' (parts, compare, groups)."
            ))
        }
    };
    let unused = match kind {
        BarKind::Parts { .. } => {
            file.small.is_none()
                && file.difference.is_none()
                && file.groups.is_none()
                && file.size.is_none()
        }
        BarKind::Compare { .. } => {
            file.parts.is_empty() && file.groups.is_none() && file.size.is_none()
        }
        BarKind::Groups { .. } => {
            file.parts.is_empty() && file.small.is_none() && file.difference.is_none()
        }
    };
    ensure(unused, "barModel: a parameter that this kind does not use.")?;
    ensure(
        file.labels.is_empty() || matches!(kind, BarKind::Compare { .. }),
        "barModel: labels name the two bars of a comparison only.",
    )?;
    ensure(
        file.labels.len() <= 2
            && file
                .labels
                .iter()
                .all(|l| !l.trim().is_empty() && l.chars().count() <= 20),
        "barModel: at most 2 labels of 1 to 20 characters.",
    )?;
    Ok(Animation::BarModel(BarModel {
        kind,
        unknown,
        labels: file.labels,
        solve: file.solve,
        interactive: file.interactive,
    }))
}
