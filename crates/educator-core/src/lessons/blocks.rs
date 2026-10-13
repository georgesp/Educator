//! `tensUnits` (SUB000117): hundred plates, ten bars and unit cubes, and what happens to them in a
//! lesson: a number joins (with the exchange of 10 cubes for a bar), a number is taken away (with a
//! bar broken into 10 cubes when there are not enough cubes), a number is shared in equal parts.
//! Every intermediate state is computed here; the front only draws it.

/// A number as base ten blocks. After a join, `units` (or `tens`) may be 10 or more until the
/// exchange.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Blocks {
    pub hundreds: u32,
    pub tens: u32,
    pub units: u32,
}

impl Blocks {
    /// The usual writing of `value`: 347 → 3 plates, 4 bars, 7 cubes.
    pub fn of(value: u32) -> Self {
        Self {
            hundreds: value / 100,
            tens: value / 10 % 10,
            units: value % 10,
        }
    }

    pub fn value(&self) -> u32 {
        self.hundreds * 100 + self.tens * 10 + self.units
    }

    /// No pile of 10 or more cubes or bars left to exchange.
    pub fn is_normalized(&self) -> bool {
        self.units < 10 && self.tens < 10
    }

    fn plus(self, other: Blocks) -> Blocks {
        Blocks {
            hundreds: self.hundreds + other.hundreds,
            tens: self.tens + other.tens,
            units: self.units + other.units,
        }
    }
}

/// What happens to the blocks of `value` on the slide.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockChange {
    /// A number joins (47 + 28: the 28 comes next to the 47).
    Add(u32),
    /// A number is taken away (50 − 6: a bar is broken first).
    Remove(u32),
    /// Shared in equal parts (half of 46: 2 bars and 3 cubes each).
    Share(u32),
}

/// `tensUnits`: the blocks of `value`, maybe changed on the slide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TensUnits {
    pub value: u32,
    /// The cubes missing to the next ten, in dotted lines (19 is « presque 20 »).
    pub complete_ten: bool,
    pub change: Option<BlockChange>,
    /// The child manipulates: builds `value` from nothing, adds or removes blocks until the target,
    /// or distributes the blocks in the parts of a share.
    pub interactive: bool,
}

/// The kind of a step of a `tensUnits` animation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockStepKind {
    /// The blocks of `value`.
    Start,
    /// The added number comes next to the blocks (`highlight`: the blocks it brings).
    Join,
    /// 10 cubes become a bar (`highlight`: the new bar).
    ExchangeUnits,
    /// 10 bars become a plate (`highlight`: the new plate).
    ExchangeTens,
    /// A bar is broken into 10 cubes (`highlight`: the 10 new cubes).
    BreakTen,
    /// A plate is broken into 10 bars (`highlight`: the 10 new bars).
    BreakHundred,
    /// The removed blocks go away (`highlight`: the blocks taken, drawn in dotted lines).
    TakeAway,
    /// The blocks are shared: `blocks` is one part, drawn `parts` times.
    Share,
}

/// A state of the blocks, shown one after the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockStep {
    pub kind: BlockStepKind,
    pub blocks: Blocks,
    pub highlight: Blocks,
}

impl TensUnits {
    /// Cubes missing to the next ten, shown when `complete_ten`.
    pub fn missing(&self) -> u32 {
        if self.complete_ten && self.value % 10 != 0 {
            10 - self.value % 10
        } else {
            0
        }
    }

    /// The value the blocks show at the end: the sum, the difference or one part of the share.
    pub fn result(&self) -> u32 {
        match self.change {
            None => self.value,
            Some(BlockChange::Add(n)) => self.value + n,
            Some(BlockChange::Remove(n)) => self.value.saturating_sub(n),
            Some(BlockChange::Share(parts)) => self.value / parts.max(1),
        }
    }

    /// Number of equal parts (1 without a share).
    pub fn parts(&self) -> u32 {
        match self.change {
            Some(BlockChange::Share(parts)) => parts,
            _ => 1,
        }
    }

    /// Interactive slide: the blocks the child starts with (nothing to build a number).
    pub fn interactive_start(&self) -> Blocks {
        match self.change {
            None => Blocks::default(),
            Some(_) => Blocks::of(self.value),
        }
    }

    /// Interactive slide: the blocks to reach (one part for a share).
    pub fn target(&self) -> Blocks {
        Blocks::of(self.result())
    }

    /// The states shown one after the other, from the blocks of `value`.
    pub fn steps(&self) -> Vec<BlockStep> {
        let start = Blocks::of(self.value);
        let mut steps = vec![BlockStep {
            kind: BlockStepKind::Start,
            blocks: start,
            highlight: Blocks::default(),
        }];
        match self.change {
            None => {}
            Some(BlockChange::Add(n)) => add_steps(start, n, &mut steps),
            Some(BlockChange::Remove(n)) => remove_steps(start, n, &mut steps),
            Some(BlockChange::Share(parts)) => share_steps(start, parts, &mut steps),
        }
        steps
    }
}

fn add_steps(start: Blocks, n: u32, steps: &mut Vec<BlockStep>) {
    let added = Blocks::of(n);
    let mut blocks = start.plus(added);
    steps.push(BlockStep {
        kind: BlockStepKind::Join,
        blocks,
        highlight: added,
    });
    if blocks.units >= 10 {
        blocks.units -= 10;
        blocks.tens += 1;
        steps.push(BlockStep {
            kind: BlockStepKind::ExchangeUnits,
            blocks,
            highlight: Blocks {
                tens: 1,
                ..Blocks::default()
            },
        });
    }
    if blocks.tens >= 10 {
        blocks.tens -= 10;
        blocks.hundreds += 1;
        steps.push(BlockStep {
            kind: BlockStepKind::ExchangeTens,
            blocks,
            highlight: Blocks {
                hundreds: 1,
                ..Blocks::default()
            },
        });
    }
}

fn remove_steps(start: Blocks, n: u32, steps: &mut Vec<BlockStep>) {
    let removed = Blocks::of(n);
    let mut blocks = start;
    if blocks.units < removed.units {
        if blocks.tens == 0 {
            blocks.hundreds -= 1;
            blocks.tens += 10;
            steps.push(BlockStep {
                kind: BlockStepKind::BreakHundred,
                blocks,
                highlight: Blocks {
                    tens: 10,
                    ..Blocks::default()
                },
            });
        }
        blocks.tens -= 1;
        blocks.units += 10;
        steps.push(BlockStep {
            kind: BlockStepKind::BreakTen,
            blocks,
            highlight: Blocks {
                units: 10,
                ..Blocks::default()
            },
        });
    }
    if blocks.tens < removed.tens {
        blocks.hundreds -= 1;
        blocks.tens += 10;
        steps.push(BlockStep {
            kind: BlockStepKind::BreakHundred,
            blocks,
            highlight: Blocks {
                tens: 10,
                ..Blocks::default()
            },
        });
    }
    steps.push(BlockStep {
        kind: BlockStepKind::TakeAway,
        blocks: Blocks {
            hundreds: blocks.hundreds - removed.hundreds,
            tens: blocks.tens - removed.tens,
            units: blocks.units - removed.units,
        },
        highlight: removed,
    });
}

fn share_steps(start: Blocks, parts: u32, steps: &mut Vec<BlockStep>) {
    let mut blocks = start;
    // A plate or a bar that cannot be shared is broken first (half of 56: a bar becomes 10 cubes).
    let plates_left = blocks.hundreds % parts;
    if plates_left > 0 {
        blocks.hundreds -= plates_left;
        blocks.tens += 10 * plates_left;
        steps.push(BlockStep {
            kind: BlockStepKind::BreakHundred,
            blocks,
            highlight: Blocks {
                tens: 10 * plates_left,
                ..Blocks::default()
            },
        });
    }
    let bars_left = blocks.tens % parts;
    if bars_left > 0 {
        blocks.tens -= bars_left;
        blocks.units += 10 * bars_left;
        steps.push(BlockStep {
            kind: BlockStepKind::BreakTen,
            blocks,
            highlight: Blocks {
                units: 10 * bars_left,
                ..Blocks::default()
            },
        });
    }
    steps.push(BlockStep {
        kind: BlockStepKind::Share,
        blocks: Blocks {
            hundreds: blocks.hundreds / parts,
            tens: blocks.tens / parts,
            units: blocks.units / parts,
        },
        highlight: Blocks::default(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blocks(hundreds: u32, tens: u32, units: u32) -> Blocks {
        Blocks {
            hundreds,
            tens,
            units,
        }
    }

    fn tens_units(value: u32, change: Option<BlockChange>) -> TensUnits {
        TensUnits {
            value,
            complete_ten: false,
            change,
            interactive: false,
        }
    }

    fn kinds(steps: &[BlockStep]) -> Vec<BlockStepKind> {
        steps.iter().map(|s| s.kind).collect()
    }

    #[test]
    fn a_join_exchanges_ten_cubes_for_a_bar() {
        // 47 + 28: 6 bars and 15 cubes, then 7 bars and 5 cubes.
        let steps = tens_units(47, Some(BlockChange::Add(28))).steps();
        assert_eq!(
            vec![
                BlockStepKind::Start,
                BlockStepKind::Join,
                BlockStepKind::ExchangeUnits
            ],
            kinds(&steps)
        );
        assert_eq!(blocks(0, 6, 15), steps[1].blocks);
        assert_eq!(blocks(0, 2, 8), steps[1].highlight);
        assert_eq!(blocks(0, 7, 5), steps[2].blocks);
        assert_eq!(75, steps[2].blocks.value());

        // 35 + 24: no exchange. 90 + 10: 10 bars become a plate.
        assert_eq!(2, tens_units(35, Some(BlockChange::Add(24))).steps().len());
        let plate = tens_units(90, Some(BlockChange::Add(10))).steps();
        assert_eq!(BlockStepKind::ExchangeTens, plate.last().unwrap().kind);
        assert_eq!(blocks(1, 0, 0), plate.last().unwrap().blocks);
    }

    #[test]
    fn taking_away_breaks_a_bar_when_cubes_are_missing() {
        // 50 − 6: a bar becomes 10 cubes, then 4 bars and 4 cubes remain.
        let steps = tens_units(50, Some(BlockChange::Remove(6))).steps();
        assert_eq!(
            vec![
                BlockStepKind::Start,
                BlockStepKind::BreakTen,
                BlockStepKind::TakeAway
            ],
            kinds(&steps)
        );
        assert_eq!(blocks(0, 4, 10), steps[1].blocks);
        assert_eq!(blocks(0, 4, 4), steps[2].blocks);
        assert_eq!(blocks(0, 0, 6), steps[2].highlight);

        // 15 − 3: enough cubes. 37 − 10: a bar goes. 300 − 20: a plate is broken.
        assert_eq!(
            2,
            tens_units(15, Some(BlockChange::Remove(3))).steps().len()
        );
        assert_eq!(
            blocks(0, 2, 7),
            tens_units(37, Some(BlockChange::Remove(10)))
                .steps()
                .last()
                .unwrap()
                .blocks
        );
        let plate = tens_units(300, Some(BlockChange::Remove(20))).steps();
        assert_eq!(BlockStepKind::BreakHundred, plate[1].kind);
        assert_eq!(blocks(2, 8, 0), plate.last().unwrap().blocks);
        // 100 − 1: the plate, then a bar.
        let both = tens_units(100, Some(BlockChange::Remove(1))).steps();
        assert_eq!(
            vec![
                BlockStepKind::Start,
                BlockStepKind::BreakHundred,
                BlockStepKind::BreakTen,
                BlockStepKind::TakeAway
            ],
            kinds(&both)
        );
        assert_eq!(99, both.last().unwrap().blocks.value());
    }

    #[test]
    fn a_share_breaks_what_cannot_be_shared() {
        // Half of 46: 2 bars and 3 cubes each.
        let steps = tens_units(46, Some(BlockChange::Share(2))).steps();
        assert_eq!(blocks(0, 2, 3), steps.last().unwrap().blocks);
        assert_eq!(2, steps.len());
        // Half of 56: a bar is broken, then 2 bars and 8 cubes each. Half of 300: 1 plate and 5 bars.
        let odd = tens_units(56, Some(BlockChange::Share(2))).steps();
        assert_eq!(BlockStepKind::BreakTen, odd[1].kind);
        assert_eq!(blocks(0, 2, 8), odd.last().unwrap().blocks);
        assert_eq!(
            blocks(1, 5, 0),
            tens_units(300, Some(BlockChange::Share(2)))
                .steps()
                .last()
                .unwrap()
                .blocks
        );
    }

    #[test]
    fn an_interactive_slide_has_a_start_and_a_target() {
        let build = TensUnits {
            interactive: true,
            ..tens_units(253, None)
        };
        assert_eq!(Blocks::default(), build.interactive_start());
        assert_eq!(blocks(2, 5, 3), build.target());

        let add = tens_units(36, Some(BlockChange::Add(27)));
        assert_eq!(blocks(0, 3, 6), add.interactive_start());
        assert_eq!(63, add.target().value());
        assert_eq!(42, tens_units(84, Some(BlockChange::Share(2))).result());
        assert_eq!(2, tens_units(84, Some(BlockChange::Share(2))).parts());
    }

    #[test]
    fn complete_ten_counts_the_missing_cubes() {
        let nineteen = TensUnits {
            complete_ten: true,
            ..tens_units(19, None)
        };
        assert_eq!(1, nineteen.missing());
        assert_eq!(0, tens_units(19, None).missing());
    }
}
