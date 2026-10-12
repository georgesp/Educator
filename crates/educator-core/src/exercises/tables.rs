//! « Mes tables » (SUB000119): a multiplication table shown in large and read aloud.

/// The tables go from × 0 to × 10, like the programme (A × B with A and B from 0 to 10).
pub const TABLE_LAST_FACTOR: i32 = 10;

/// One line of a table: `3 × 7 = 21`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableLine {
    pub left: i32,
    pub right: i32,
    pub result: i32,
    /// « 3 fois 7, 21 » read aloud.
    pub spoken: String,
}

/// The table of `number` (from 0 to 10), `None` outside 0 to 10.
pub fn multiplication_table(number: i32) -> Option<Vec<TableLine>> {
    if !(0..=TABLE_LAST_FACTOR).contains(&number) {
        return None;
    }

    Some(
        (0..=TABLE_LAST_FACTOR)
            .map(|factor| TableLine {
                left: number,
                right: factor,
                result: number * factor,
                spoken: format!("{number} fois {factor}, {}.", number * factor),
            })
            .collect(),
    )
}
