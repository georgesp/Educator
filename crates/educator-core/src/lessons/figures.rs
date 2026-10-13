//! Number figures of the lessons (SUB000117): `compareNumbers`, `sequencePattern`, `arrayGrid`,
//! `timesTable` and `placeValueTable`. The values they show are computed here.

use super::Operator;
use crate::exercises::Comparison;
use crate::numbers::{spell, Spelling};

/// A rank of the decimal writing, from the units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Units,
    Tens,
    Hundreds,
    Thousands,
}

impl Rank {
    pub const ALL: [Rank; 4] = [Rank::Units, Rank::Tens, Rank::Hundreds, Rank::Thousands];

    /// 0 for the units, 3 for the thousands.
    pub fn index(self) -> usize {
        self as usize
    }

    pub fn from_index(index: usize) -> Option<Rank> {
        Self::ALL.get(index).copied()
    }

    /// 1, 10, 100, 1 000.
    pub fn power(self) -> u32 {
        10u32.pow(self as u32)
    }
}

/// The digit of `n` at `rank`, `None` beyond its first digit (0 has one digit).
pub fn digit_of(n: u32, rank: Rank) -> Option<u8> {
    (rank.index() < digit_count(n)).then(|| (n / rank.power() % 10) as u8)
}

/// Number of digits of `n` (0 has one).
pub fn digit_count(n: u32) -> usize {
    n.max(1).ilog10() as usize + 1
}

// ── compareNumbers ─────────────────────────────────────────────────────────────────────────

/// `compareNumbers`: two numbers face to face, compared rank after rank from the left.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompareNumbers {
    pub a: u32,
    pub b: u32,
    /// The child picks the sign (<, > or =).
    pub interactive: bool,
}

impl CompareNumbers {
    /// The sign written between `a` and `b`.
    pub fn sign(&self) -> Comparison {
        Comparison::of(self.a, self.b)
    }

    /// The ranks shown, from the highest (on the left) to the units.
    pub fn ranks(&self) -> Vec<Rank> {
        let width = digit_count(self.a).max(digit_count(self.b));
        (0..width).rev().filter_map(Rank::from_index).collect()
    }

    /// The rank that decides: the first one, from the left, where the digits differ (a number with
    /// more digits wins at its first rank). `None` for two equal numbers.
    pub fn decisive_rank(&self) -> Option<Rank> {
        self.ranks()
            .into_iter()
            .find(|rank| digit_of(self.a, *rank) != digit_of(self.b, *rank))
    }
}

// ── sequencePattern ────────────────────────────────────────────────────────────────────────

/// `sequencePattern`: a counting sequence with hidden numbers, or numbers to put in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SequencePattern {
    /// `start`, then `count − 1` steps of `step` (negative: counting backwards). The numbers at
    /// `hidden` (positions from 0) appear in a box, then are revealed.
    Count {
        start: i32,
        step: i32,
        count: usize,
        hidden: Vec<usize>,
    },
    /// Cars carrying `values` go on the starting grid in increasing order (decreasing when
    /// `descending`). Interactive: the child clicks them in that order.
    Order {
        values: Vec<u32>,
        descending: bool,
        interactive: bool,
    },
}

impl SequencePattern {
    /// Count: the numbers of the sequence. Order: the numbers in the order asked.
    pub fn values(&self) -> Vec<i64> {
        match self {
            SequencePattern::Count {
                start, step, count, ..
            } => (0..*count)
                .map(|i| i64::from(*start) + i as i64 * i64::from(*step))
                .collect(),
            SequencePattern::Order {
                values, descending, ..
            } => {
                let mut sorted: Vec<i64> = values.iter().map(|v| i64::from(*v)).collect();
                sorted.sort_unstable();
                if *descending {
                    sorted.reverse();
                }
                sorted
            }
        }
    }

    pub fn is_interactive(&self) -> bool {
        matches!(
            self,
            SequencePattern::Order {
                interactive: true,
                ..
            }
        )
    }
}

// ── arrayGrid ──────────────────────────────────────────────────────────────────────────────

/// `arrayGrid`: the starting grid, `rows` rows of `cols` cars (3 × 5: 3 rows of 5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayGrid {
    pub rows: u32,
    pub cols: u32,
    /// The grid is cut after this many rows (13 × 7 = 10 × 7 + 3 × 7).
    pub split: Option<u32>,
    /// The grid turns: `cols` rows of `rows` (3 × 5 = 5 × 3).
    pub rotate: bool,
    /// The rows are added one by one: 5 + 5 + 5 = 15.
    pub show_sum: bool,
    /// Cars leaving the grid (3 rows of 8, 5 leave).
    pub removed: u32,
    /// With `split`: the child clicks where to cut. Without: the child places the rows one by one.
    pub interactive: bool,
}

/// A part of a cut grid and its product.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPart {
    pub rows: u32,
    pub cols: u32,
    pub product: u32,
}

impl ArrayGrid {
    pub fn total(&self) -> u32 {
        self.rows * self.cols
    }

    pub fn remaining(&self) -> u32 {
        self.total().saturating_sub(self.removed)
    }

    /// The parts of the grid, from the top: one without `split`.
    pub fn parts(&self) -> Vec<GridPart> {
        let part = |rows: u32| GridPart {
            rows,
            cols: self.cols,
            product: rows * self.cols,
        };
        match self.split {
            Some(first) if first > 0 && first < self.rows => {
                vec![part(first), part(self.rows - first)]
            }
            _ => vec![part(self.rows)],
        }
    }

    /// The repeated addition read row by row (`show_sum`): 3 × 5 → [5, 5, 5].
    pub fn repeated_addition(&self) -> Vec<u32> {
        if self.show_sum {
            vec![self.cols; self.rows as usize]
        } else {
            Vec::new()
        }
    }
}

// ── timesTable ─────────────────────────────────────────────────────────────────────────────

/// What lights up in a table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableHighlight {
    /// The row of `n` (the table of 2: 2 × 1, 2 × 2…).
    Row(u32),
    /// The diagonal: 4 + 4, 5 × 5.
    Doubles,
    /// The cells whose result is `n` (« les amis de 10 »).
    Result(u32),
}

/// `timesTable`: the table of an operation from 1 to `size`, whose cells light up group by group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimesTable {
    pub operator: Operator,
    pub size: u32,
    pub highlights: Vec<TableHighlight>,
}

/// A cell of a table: `row operator col = value`, in the first highlight group it belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableCell {
    pub row: u32,
    pub col: u32,
    pub value: u32,
    pub group: Option<usize>,
}

impl TimesTable {
    /// The rows of the table, from 1 to `size`.
    pub fn cells(&self) -> Vec<Vec<TableCell>> {
        (1..=self.size)
            .map(|row| {
                (1..=self.size)
                    .map(|col| {
                        let value = self.operator.apply(row, col).unwrap_or(0);
                        let group = self.highlights.iter().position(|h| match h {
                            TableHighlight::Row(n) => row == *n,
                            TableHighlight::Doubles => row == col,
                            TableHighlight::Result(n) => value == *n,
                        });
                        TableCell {
                            row,
                            col,
                            value,
                            group,
                        }
                    })
                    .collect()
            })
            .collect()
    }
}

// ── placeValueTable ────────────────────────────────────────────────────────────────────────

/// `placeValueTable`: the table of numeration (thousands, hundreds, tens, units).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceValueTable {
    pub value: u32,
    /// The rank shown in racing red (the 0 that keeps the place of the tens).
    pub focus: Option<Rank>,
    /// « Nombre de dizaines » : 347 has 34 tens (and its tens digit is 4).
    pub count_of: Option<Rank>,
    /// 347 = 300 + 40 + 7.
    pub decomposition: bool,
    /// The number in words, traditional spelling and 1990 one.
    pub words: bool,
    /// The child writes the number read by the voice, digit by digit from the left.
    pub interactive: bool,
}

impl PlaceValueTable {
    /// The ranks of the table, from the left: at least hundreds, tens, units.
    pub fn ranks(&self) -> Vec<Rank> {
        let width = digit_count(self.value).max(3);
        (0..width).rev().filter_map(Rank::from_index).collect()
    }

    /// The digit in each column of `ranks` (`None`: nothing written on the left of the number).
    pub fn digits(&self) -> Vec<Option<u8>> {
        self.ranks()
            .into_iter()
            .map(|rank| digit_of(self.value, rank))
            .collect()
    }

    /// How many `rank` there are in the number: 347 has 34 tens.
    pub fn count(&self, rank: Rank) -> u32 {
        self.value / rank.power()
    }

    /// 347 → [300, 40, 7]; 608 → [600, 8].
    pub fn decomposition_terms(&self) -> Vec<u32> {
        let mut terms: Vec<u32> = Rank::ALL
            .iter()
            .rev()
            .filter_map(|rank| {
                let digit = digit_of(self.value, *rank)?;
                (digit > 0).then(|| u32::from(digit) * rank.power())
            })
            .collect();
        if terms.is_empty() {
            terms.push(0);
        }
        terms
    }

    /// The traditional spelling, then the 1990 one.
    pub fn spellings(&self) -> (String, String) {
        let n = self.value as i32;
        (
            spell(n, Spelling::Traditional),
            spell(n, Spelling::Rectified1990),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_are_compared_from_the_left() {
        let c = CompareNumbers {
            a: 438,
            b: 483,
            interactive: false,
        };
        assert_eq!(Comparison::Less, c.sign());
        assert_eq!(vec![Rank::Hundreds, Rank::Tens, Rank::Units], c.ranks());
        assert_eq!(Some(Rank::Tens), c.decisive_rank());

        // 701 > 699: the hundreds decide. 1 000 > 999: the thousands. 52 = 52: nothing decides.
        let c = CompareNumbers {
            a: 701,
            b: 699,
            interactive: true,
        };
        assert_eq!(
            (Comparison::Greater, Some(Rank::Hundreds)),
            (c.sign(), c.decisive_rank())
        );
        let c = CompareNumbers {
            a: 1000,
            b: 999,
            interactive: false,
        };
        assert_eq!(Some(Rank::Thousands), c.decisive_rank());
        let c = CompareNumbers {
            a: 52,
            b: 52,
            interactive: false,
        };
        assert_eq!((Comparison::Equal, None), (c.sign(), c.decisive_rank()));
    }

    #[test]
    fn a_sequence_counts_forwards_or_backwards() {
        let count = SequencePattern::Count {
            start: 12,
            step: -1,
            count: 4,
            hidden: vec![2],
        };
        assert_eq!(vec![12, 11, 10, 9], count.values());
        assert!(!count.is_interactive());

        let order = SequencePattern::Order {
            values: vec![483, 348, 438],
            descending: false,
            interactive: true,
        };
        assert_eq!(vec![348, 438, 483], order.values());
        assert!(order.is_interactive());
        let down = SequencePattern::Order {
            values: vec![483, 348, 438],
            descending: true,
            interactive: false,
        };
        assert_eq!(vec![483, 438, 348], down.values());
    }

    #[test]
    fn a_grid_is_cut_in_two_products() {
        let grid = ArrayGrid {
            rows: 13,
            cols: 7,
            split: Some(10),
            rotate: false,
            show_sum: false,
            removed: 0,
            interactive: false,
        };
        assert_eq!(91, grid.total());
        let products: Vec<u32> = grid.parts().iter().map(|p| p.product).collect();
        assert_eq!(vec![70, 21], products);

        let sum = ArrayGrid {
            rows: 3,
            cols: 5,
            split: None,
            show_sum: true,
            removed: 0,
            ..grid.clone()
        };
        assert_eq!(vec![5, 5, 5], sum.repeated_addition());
        assert_eq!(1, sum.parts().len());

        let leaving = ArrayGrid {
            rows: 3,
            cols: 8,
            split: None,
            removed: 5,
            ..grid
        };
        assert_eq!(19, leaving.remaining());
    }

    #[test]
    fn a_table_lights_its_groups() {
        let table = TimesTable {
            operator: Operator::Add,
            size: 9,
            highlights: vec![TableHighlight::Doubles, TableHighlight::Result(10)],
        };
        let cells = table.cells();
        assert_eq!(9, cells.len());
        assert_eq!(8, cells[3][3].value);
        assert_eq!(Some(0), cells[3][3].group);
        // 5 + 5 is a double first; 3 + 7 a friend of 10.
        assert_eq!(Some(0), cells[4][4].group);
        assert_eq!(Some(1), cells[2][6].group);
        assert_eq!(None, cells[0][0].group.filter(|g| *g == 1));

        let two = TimesTable {
            operator: Operator::Multiply,
            size: 10,
            highlights: vec![TableHighlight::Row(2)],
        };
        let cells = two.cells();
        assert_eq!(14, cells[1][6].value);
        assert_eq!(Some(0), cells[1][6].group);
        assert_eq!(None, cells[2][1].group);
    }

    #[test]
    fn a_place_value_table_writes_and_decomposes() {
        let table = PlaceValueTable {
            value: 347,
            focus: Some(Rank::Tens),
            count_of: Some(Rank::Tens),
            decomposition: true,
            words: true,
            interactive: false,
        };
        assert_eq!(vec![Rank::Hundreds, Rank::Tens, Rank::Units], table.ranks());
        assert_eq!(vec![Some(3), Some(4), Some(7)], table.digits());
        assert_eq!(34, table.count(Rank::Tens));
        assert_eq!(vec![300, 40, 7], table.decomposition_terms());
        assert_eq!(
            (
                "trois cent quarante-sept".to_owned(),
                "trois-cent-quarante-sept".to_owned()
            ),
            table.spellings()
        );

        let zero = PlaceValueTable {
            value: 608,
            ..table.clone()
        };
        assert_eq!(vec![600, 8], zero.decomposition_terms());
        assert_eq!(vec![Some(6), Some(0), Some(8)], zero.digits());
        let small = PlaceValueTable { value: 47, ..table };
        assert_eq!(vec![None, Some(4), Some(7)], small.digits());
    }
}
