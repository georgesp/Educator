//! Lessons (SUB001409, SUB001410, SUB001411): short animated lessons made of slides, followed by a
//! quiz. A lesson is content (`content/lessons/<subject>/*.json`, loaded and validated by
//! `educator-content`); this module holds its model, the values the animations show (computed here,
//! never in the front) and the checking of the quiz answers.

mod blocks;
mod figures;
mod objects;
mod words;

pub use blocks::{BlockChange, BlockStep, BlockStepKind, Blocks, TensUnits};
pub use figures::{
    digit_count, digit_of, ArrayGrid, CompareNumbers, GridPart, PlaceValueTable, Rank,
    SequencePattern, TableCell, TableHighlight, TimesTable,
};
pub use objects::{
    BarKind, BarModel, BarUnknown, BarValue, FractionBar, FractionStrip, FrameSlot, GroupMode,
    GroupObjects, Grouping, RaceObject, TenFrame, FRAME_SIZE,
};
pub use words::{grapheme_position, LetterTiles, SortedWord, SoundSort};

use crate::exercises::{Answer, Exercise, Input};
use crate::skills::ExerciseFormat;
use crate::skills::Subject;
use crate::spelling::SpokenWord;
use crate::Grade;

/// A lesson: slides to watch and manipulate, then a short quiz, then the linked exercises.
#[derive(Debug, Clone, PartialEq)]
pub struct Lesson {
    pub id: String,
    pub subject: Subject,
    pub grade: Grade,
    /// School period (1 to 5) where the lesson is offered.
    pub period: u8,
    /// Skills practised after the lesson (« Je m'entraîne »), the first one being the main one.
    pub skill_ids: Vec<String>,
    pub title: String,
    pub duration_minutes: u8,
    pub slides: Vec<Slide>,
    pub quiz: Vec<QuizQuestion>,
    pub sources: Vec<LessonSource>,
}

/// One slide: a short text shown, a longer text read aloud, an animation.
#[derive(Debug, Clone, PartialEq)]
pub struct Slide {
    /// Short heading (« Rappel », « À toi »).
    pub title: String,
    /// Text shown to the child: one or two short sentences.
    pub text: String,
    /// Text read by the voice (may say more than `text`).
    pub speech: String,
    pub animation: Animation,
}

impl Slide {
    /// Whether the child has to do something before going on (click, match).
    pub fn is_interactive(&self) -> bool {
        self.animation.is_interactive()
    }
}

/// The animations of the library (SUB000117), with their parameters. The values they show are
/// computed by this module and its submodules, never by the front.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Animation {
    /// Title screen: checkered strip, title, subtitle.
    TitleCard {
        title: String,
        subtitle: Option<String>,
    },
    /// « Je retiens »: the rule in one sentence, with an example.
    RecapCard {
        text: String,
        example: Option<String>,
    },
    /// Mental calculation as chains of arrows (47 → + 3 → 50 → + 5 → 55).
    CalcChain {
        chains: Vec<Chain>,
        /// `Show`: the chains appear one step after the other. `Match`: the child links each
        /// calculation (`start + total`) to its chain of steps. `Pick`: the child clicks the right one.
        mode: ChainMode,
    },
    /// A graduated track: the car jumps from `start`, one jump per step.
    NumberLine {
        from: i32,
        to: i32,
        /// Gap between two graduations.
        step: i32,
        start: i32,
        jumps: Vec<i32>,
        /// The child clicks where the car lands after each jump.
        interactive: bool,
        /// On an interactive line, the value of a graduation shows when the pointer is over it. Off
        /// when finding a number on the line is what the lesson teaches.
        hover_values: bool,
        /// Graduations marked with a checkered flag (the bounds of a framing: 300 and 400).
        flags: Vec<i32>,
    },
    /// Base ten blocks (plates, bars, cubes), maybe joined, taken away or shared.
    TensUnits(TensUnits),
    /// A written operation (« opération posée »), computed column after column from the units.
    ColumnOperation(ColumnOperation),
    /// An equality (`left operator right = result`), maybe with a hidden term and its family.
    Equation(Equation),
    /// Two numbers face to face, compared rank after rank.
    CompareNumbers(CompareNumbers),
    /// A counting sequence with hidden numbers, or numbers to put in order.
    SequencePattern(SequencePattern),
    /// The starting grid: rows of cars, for the multiplication.
    ArrayGrid(ArrayGrid),
    /// An addition or multiplication table whose cells light up.
    TimesTable(TimesTable),
    /// The table of numeration.
    PlaceValueTable(PlaceValueTable),
    /// Racing objects to join, take away, group, share or sort.
    GroupObjects(GroupObjects),
    /// The « garage de 10 places ».
    TenFrame(TenFrame),
    /// Strips shared in equal parts (fractions).
    FractionBar(FractionBar),
    /// The bar diagram of a problem.
    BarModel(BarModel),
    /// Letters or graphemes to put in order to make a word (spelling).
    LetterTiles(LetterTiles),
    /// Words to sort by the grapheme of a sound (spelling).
    SoundSort(SoundSort),
}

impl Animation {
    pub fn is_interactive(&self) -> bool {
        match self {
            Animation::CalcChain { mode, .. } => *mode != ChainMode::Show,
            Animation::NumberLine { interactive, .. } => *interactive,
            Animation::ColumnOperation(operation) => operation.interactive,
            Animation::Equation(equation) => equation.interactive,
            Animation::TensUnits(blocks) => blocks.interactive,
            Animation::CompareNumbers(compare) => compare.interactive,
            Animation::SequencePattern(sequence) => sequence.is_interactive(),
            Animation::ArrayGrid(grid) => grid.interactive,
            Animation::PlaceValueTable(table) => table.interactive,
            Animation::GroupObjects(objects) => objects.interactive,
            Animation::TenFrame(frame) => frame.interactive,
            Animation::FractionBar(bar) => bar.interactive,
            Animation::BarModel(model) => model.interactive,
            Animation::LetterTiles(tiles) => tiles.interactive,
            Animation::SoundSort(sort) => sort.interactive,
            Animation::TitleCard { .. }
            | Animation::RecapCard { .. }
            | Animation::TimesTable(_) => false,
        }
    }

    /// Name used in the content files (`"numberLine"`).
    pub fn type_name(&self) -> &'static str {
        match self {
            Animation::TitleCard { .. } => "titleCard",
            Animation::RecapCard { .. } => "recapCard",
            Animation::CalcChain { .. } => "calcChain",
            Animation::NumberLine { .. } => "numberLine",
            Animation::TensUnits(_) => "tensUnits",
            Animation::ColumnOperation(_) => "columnOperation",
            Animation::Equation(_) => "equation",
            Animation::CompareNumbers(_) => "compareNumbers",
            Animation::SequencePattern(_) => "sequencePattern",
            Animation::ArrayGrid(_) => "arrayGrid",
            Animation::TimesTable(_) => "timesTable",
            Animation::PlaceValueTable(_) => "placeValueTable",
            Animation::GroupObjects(_) => "groupObjects",
            Animation::TenFrame(_) => "tenFrame",
            Animation::FractionBar(_) => "fractionBar",
            Animation::BarModel(_) => "barModel",
            Animation::LetterTiles(_) => "letterTiles",
            Animation::SoundSort(_) => "soundSort",
        }
    }
}

/// The operations of the lessons: addition and subtraction (in columns, in equalities), and the
/// multiplication met in CE1 (equalities, tables, bar models).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
}

impl Operator {
    /// `left operator right`; `None` when a subtraction would go below 0 (or on an overflow).
    pub fn apply(self, left: u32, right: u32) -> Option<u32> {
        match self {
            Operator::Add => left.checked_add(right),
            Operator::Subtract => left.checked_sub(right),
            Operator::Multiply => left.checked_mul(right),
        }
    }
}

/// The two techniques of the written subtraction (programme du cycle 2): the class uses one of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtractionMethod {
    /// « Cassage »: when the top digit is too small, a ten (or a hundred) of the top number is broken.
    Cassage,
    /// « Compensation »: 10 units are added to the top digit and 1 ten to the bottom number.
    Compensation,
}

/// `columnOperation`: an addition (2 or 3 numbers) or a subtraction (2 numbers), set in columns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnOperation {
    pub operator: Operator,
    /// The numbers, from top to bottom.
    pub operands: Vec<u32>,
    /// Subtraction: the technique shown. `None` for an addition, and for `both_methods`.
    pub method: Option<SubtractionMethod>,
    /// Subtraction: the two techniques side by side (cassage, then compensation).
    pub both_methods: bool,
    /// Columns computed on the slide, from the units (0: the numbers are only set in columns).
    /// `None`: all of them.
    pub computed_columns: Option<usize>,
    /// The child writes the digits of the result, from the units.
    pub interactive: bool,
}

impl ColumnOperation {
    pub fn result(&self) -> u32 {
        match self.operator {
            Operator::Add => self.operands.iter().sum(),
            Operator::Subtract => self.operands[0] - self.operands[1..].iter().sum::<u32>(),
            Operator::Multiply => self.operands.iter().product(),
        }
    }

    /// The techniques drawn: one, or the two side by side; `[None]` for an addition.
    pub fn methods(&self) -> Vec<Option<SubtractionMethod>> {
        if self.both_methods {
            vec![
                Some(SubtractionMethod::Cassage),
                Some(SubtractionMethod::Compensation),
            ]
        } else {
            vec![self.method]
        }
    }
}

/// A column of a written operation (index 0: the units).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrittenColumn {
    /// The digit of each number in this column, from top to bottom (`None`: the number is too short).
    pub digits: Vec<Option<u8>>,
    /// Addition: carry written above this column (from the previous one). Compensation: ten added
    /// to the bottom number in this column (the small 1 next to its digit). 0 otherwise.
    pub carry: u8,
    /// Cassage: the new values of the top digit, each with the column being computed when it was
    /// written (652 − 358: the 5 of the tens becomes 4 at the units, then 14 at the tens).
    pub rewrites: Vec<Rewrite>,
    /// Compensation: 10 units added to the top digit (3 becomes 13).
    pub plus_ten: bool,
    /// What the column computes: addition, the numbers added (carry first); subtraction, the top
    /// value then the value removed.
    pub terms: Vec<u32>,
    /// The sum or the difference of the column.
    pub value: u32,
    /// The digit written in the result.
    pub digit: u8,
    /// False for the zeros on the left of the result (652 − 618 = 34, not 034).
    pub written: bool,
}

/// A digit of the top number crossed out and rewritten (cassage).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rewrite {
    /// The column being computed when the digit is rewritten.
    pub at_column: usize,
    pub value: u8,
}

fn digit_at(n: u32, column: usize) -> Option<u8> {
    let digits = n.max(1).ilog10() as usize + 1;
    (column < digits).then(|| (n / 10u32.pow(column as u32) % 10) as u8)
}

/// The columns of a written operation, from the units. A subtraction needs a method; its
/// operands must not give a negative result (checked by the content loader).
pub fn written_columns(
    operator: Operator,
    operands: &[u32],
    method: Option<SubtractionMethod>,
) -> Vec<WrittenColumn> {
    match operator {
        // The written multiplication comes in CE2; the loader refuses it in a column operation.
        Operator::Add | Operator::Multiply => written_addition(operands),
        Operator::Subtract => written_subtraction(
            operands[0],
            operands[1],
            method.unwrap_or(SubtractionMethod::Cassage),
        ),
    }
}

fn written_addition(operands: &[u32]) -> Vec<WrittenColumn> {
    let sum: u32 = operands.iter().sum();
    let mut carry = 0u32;
    (0..digit_count(sum))
        .map(|column| {
            let digits: Vec<Option<u8>> = operands.iter().map(|n| digit_at(*n, column)).collect();
            let mut terms: Vec<u32> = Vec::new();
            if carry > 0 {
                terms.push(carry);
            }
            terms.extend(digits.iter().flatten().map(|d| u32::from(*d)));
            let value: u32 = terms.iter().sum();
            let written = WrittenColumn {
                digits,
                carry: carry as u8,
                rewrites: Vec::new(),
                plus_ten: false,
                terms,
                value,
                digit: (value % 10) as u8,
                written: true,
            };
            carry = value / 10;
            written
        })
        .collect()
}

fn written_subtraction(top: u32, bottom: u32, method: SubtractionMethod) -> Vec<WrittenColumn> {
    let width = digit_count(top);
    let mut tops: Vec<u32> = (0..width)
        .map(|c| u32::from(digit_at(top, c).unwrap_or(0)))
        .collect();
    let mut rewrites: Vec<Vec<Rewrite>> = vec![Vec::new(); width];
    let mut columns = Vec::with_capacity(width);
    let mut carry = 0u32;

    for column in 0..width {
        let bottom_digit = digit_at(bottom, column);
        let removed = u32::from(bottom_digit.unwrap_or(0));
        let (used_top, used_bottom, plus_ten, column_carry) = match method {
            SubtractionMethod::Cassage => {
                if tops[column] < removed {
                    // Break the first non-zero digit on the left; the zeros in between become 9.
                    let from = (column + 1..width).find(|c| tops[*c] > 0).unwrap_or(column);
                    tops[from] -= 1;
                    rewrites[from].push(Rewrite {
                        at_column: column,
                        value: tops[from] as u8,
                    });
                    for between in column + 1..from {
                        tops[between] += 9;
                        rewrites[between].push(Rewrite {
                            at_column: column,
                            value: tops[between] as u8,
                        });
                    }
                    tops[column] += 10;
                    rewrites[column].push(Rewrite {
                        at_column: column,
                        value: tops[column] as u8,
                    });
                }
                (tops[column], removed, false, 0)
            }
            SubtractionMethod::Compensation => {
                let used_bottom = removed + carry;
                let short = tops[column] < used_bottom;
                let used_top = tops[column] + if short { 10 } else { 0 };
                let column_carry = carry;
                carry = u32::from(short);
                (used_top, used_bottom, short, column_carry)
            }
        };
        let value = used_top - used_bottom;
        columns.push(WrittenColumn {
            digits: vec![digit_at(top, column), bottom_digit],
            carry: column_carry as u8,
            rewrites: Vec::new(),
            plus_ten,
            terms: vec![used_top, used_bottom],
            value,
            digit: value as u8,
            written: true,
        });
    }

    for (column, list) in rewrites.into_iter().enumerate() {
        columns[column].rewrites = list;
    }
    // No zero on the left of the result, except the units.
    for column in (1..width).rev() {
        if columns[column].digit != 0 {
            break;
        }
        columns[column].written = false;
    }
    columns
}

/// The term of an equality hidden in a box.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquationTerm {
    Left,
    Right,
    Result,
}

/// `equation`: `left operator right = result`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Equation {
    pub left: u32,
    pub operator: Operator,
    pub right: u32,
    /// The term shown as a box (« 7 + … = 15 »).
    pub missing: Option<EquationTerm>,
    /// After the equality, the other equalities of its family appear (7 + 8 = 15, 8 + 7 = 15,
    /// 15 − 7 = 8, 15 − 8 = 7).
    pub family: bool,
    /// The child picks the missing term among `choices`.
    pub interactive: bool,
    pub choices: Vec<u32>,
}

/// One equality `left operator right = result`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Equality {
    pub left: u32,
    pub operator: Operator,
    pub right: u32,
    pub result: u32,
}

impl Equation {
    /// The result; the loader refuses a subtraction below 0.
    pub fn result(&self) -> u32 {
        self.operator.apply(self.left, self.right).unwrap_or(0)
    }

    /// The value of the hidden term.
    pub fn missing_value(&self) -> Option<u32> {
        self.missing.map(|term| match term {
            EquationTerm::Left => self.left,
            EquationTerm::Right => self.right,
            EquationTerm::Result => self.result(),
        })
    }

    /// The equality, then the others of its family when `family` (no line twice: 6 + 6 = 12).
    /// A multiplication only swaps its factors (8 × 5 = 40, 5 × 8 = 40): the division comes later.
    pub fn equalities(&self) -> Vec<Equality> {
        let first = Equality {
            left: self.left,
            operator: self.operator,
            right: self.right,
            result: self.result(),
        };
        if !self.family {
            return vec![first];
        }

        // The family of a + b = c: a + b = c, b + a = c, c − a = b, c − b = a.
        let (a, b, c) = match self.operator {
            Operator::Add | Operator::Multiply => (self.left, self.right, self.result()),
            Operator::Subtract => (self.right, self.result(), self.left),
        };
        let all: Vec<(u32, Operator, u32, u32)> = if self.operator == Operator::Multiply {
            vec![(b, Operator::Multiply, a, c)]
        } else {
            vec![
                (a, Operator::Add, b, c),
                (b, Operator::Add, a, c),
                (c, Operator::Subtract, a, b),
                (c, Operator::Subtract, b, a),
            ]
        };
        let mut lines = vec![first];
        for (left, operator, right, result) in all {
            let line = Equality {
                left,
                operator,
                right,
                result,
            };
            if !lines.contains(&line) {
                lines.push(line);
            }
        }
        lines
    }
}

/// `Show`: the chains appear step by step. `Match`: the child links each calculation to its chain.
/// `Pick`: the child clicks the only right chain; the others are classic mistakes (`crossed`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainMode {
    Show,
    Match,
    Pick,
}

/// `start`, then each step (+ 20, − 1). A `crossed` chain is a classic mistake, shown crossed out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chain {
    pub start: i32,
    pub steps: Vec<i32>,
    pub crossed: bool,
}

impl Chain {
    /// The value after each step: 46, [+20, −1] → [66, 65].
    pub fn values(&self) -> Vec<i32> {
        landings(self.start, &self.steps)
    }

    /// What the whole chain adds: [+20, −1] → 19.
    pub fn total(&self) -> i32 {
        self.steps.iter().sum()
    }

    /// The final value.
    pub fn result(&self) -> i32 {
        self.start + self.total()
    }
}

/// Where the car lands after each jump.
pub fn landings(start: i32, jumps: &[i32]) -> Vec<i32> {
    jumps
        .iter()
        .scan(start, |at, jump| {
            *at += jump;
            Some(*at)
        })
        .collect()
}

/// A question of the end-of-lesson quiz: typed (`choices` empty) or picked among `choices`.
#[derive(Debug, Clone, PartialEq)]
pub struct QuizQuestion {
    /// Shown, e.g. « 46 + 19 = ? ».
    pub question: String,
    /// Read aloud, e.g. « Combien font 46 plus 19 ? ».
    pub spoken: String,
    pub answer: Answer,
    pub choices: Vec<Answer>,
    /// Shown and read after the answer, right or wrong.
    pub explanation: String,
}

/// A reference consulted to write the lesson.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonSource {
    pub title: String,
    pub url: String,
}

/// Result of a quiz answer.
#[derive(Debug, Clone, PartialEq)]
pub struct QuizOutcome {
    pub correct: bool,
    pub expected: Answer,
    pub explanation: String,
}

impl Lesson {
    /// Checks the answer to question `index`; `None` when there is no such question.
    pub fn check_quiz(&self, index: usize, given: &Answer) -> Option<QuizOutcome> {
        let question = self.quiz.get(index)?;
        Some(QuizOutcome {
            correct: question.answer.accepts(given),
            expected: question.answer.clone(),
            explanation: question.explanation.clone(),
        })
    }

    /// Number of slides where the child manipulates.
    pub fn interactive_slides(&self) -> usize {
        self.slides.iter().filter(|s| s.is_interactive()).count()
    }

    /// The quiz as exercises (SUB001411), answered and recorded like a series: a pick when the
    /// question has choices, otherwise a typed number or word (a word to write is read like a
    /// dictation).
    pub fn quiz_exercises(&self) -> Vec<Exercise> {
        self.quiz
            .iter()
            .enumerate()
            .map(|(index, question)| {
                let typed_word = match (&question.answer, question.choices.is_empty()) {
                    (Answer::Spelling(word), true) => Some(word.clone()),
                    _ => None,
                };
                let input = if !question.choices.is_empty() {
                    Input::Pick
                } else if let Some(word) = &typed_word {
                    Input::Spelling {
                        max_length: word.chars().count() + 6,
                    }
                } else {
                    Input::Number { slots: 1 }
                };
                Exercise {
                    index,
                    format: if typed_word.is_some() {
                        ExerciseFormat::Dictation
                    } else {
                        ExerciseFormat::Result
                    },
                    question: Some(question.question.clone()),
                    statement: Vec::new(),
                    picture: None,
                    input,
                    expected: question.answer.clone(),
                    choices: (!question.choices.is_empty()).then(|| question.choices.clone()),
                    spoken: question.spoken.clone(),
                    explanation: question.explanation.clone(),
                    help: None,
                    key: format!("{}-{index}", self.id),
                    calculation: None,
                    word: typed_word.map(|word| SpokenWord {
                        word,
                        example: None,
                        blank: None,
                    }),
                }
            })
            .collect()
    }
}

/// The lessons loaded at start-up.
pub trait LessonCatalog: Send + Sync {
    fn all(&self) -> &[Lesson];

    fn find(&self, id: &str) -> Option<&Lesson> {
        self.all().iter().find(|l| l.id == id)
    }

    /// The lessons of a grade, by period then in file order.
    fn for_grade(&self, subject: Subject, grade: Grade) -> Vec<&Lesson> {
        let mut lessons: Vec<&Lesson> = self
            .all()
            .iter()
            .filter(|l| l.subject == subject && l.grade == grade)
            .collect();
        lessons.sort_by_key(|l| l.period);
        lessons
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chain(start: i32, steps: &[i32]) -> Chain {
        Chain {
            start,
            steps: steps.to_vec(),
            crossed: false,
        }
    }

    #[test]
    fn a_chain_gives_the_value_after_each_step() {
        let c = chain(46, &[20, -1]);
        assert_eq!(vec![66, 65], c.values());
        assert_eq!(19, c.total());
        assert_eq!(65, c.result());
    }

    #[test]
    fn landings_follow_the_jumps() {
        assert_eq!(vec![77, 76], landings(57, &[20, -1]));
        assert!(landings(3, &[]).is_empty());
    }

    fn digits(columns: &[WrittenColumn]) -> Vec<u8> {
        columns.iter().map(|c| c.digit).collect()
    }

    #[test]
    fn a_written_addition_carries_to_the_next_column() {
        let columns = written_columns(Operator::Add, &[56, 27], None);
        assert_eq!(vec![3, 8], digits(&columns));
        assert_eq!(vec![6, 7], columns[0].terms);
        assert_eq!(13, columns[0].value);
        assert_eq!(1, columns[1].carry);
        assert_eq!(vec![1, 5, 2], columns[1].terms);

        // 378 + 456 = 834: two carries.
        let columns = written_columns(Operator::Add, &[378, 456], None);
        assert_eq!(vec![4, 3, 8], digits(&columns));
        assert_eq!(
            vec![0, 1, 1],
            columns.iter().map(|c| c.carry).collect::<Vec<_>>()
        );

        // 56 + 67 = 123: the last carry is a column of its own; 28 + 8 + 56: a short number.
        let columns = written_columns(Operator::Add, &[56, 67], None);
        assert_eq!(vec![3, 2, 1], digits(&columns));
        assert_eq!(vec![None, None], columns[2].digits);
        let columns = written_columns(Operator::Add, &[28, 8, 56], None);
        assert_eq!(vec![Some(2), None, Some(5)], columns[1].digits);
        assert_eq!(vec![2, 9], digits(&columns));
    }

    #[test]
    fn cassage_breaks_a_ten_or_a_hundred() {
        // 74 − 28: 7 tens become 6, 4 units become 14.
        let columns = written_columns(
            Operator::Subtract,
            &[74, 28],
            Some(SubtractionMethod::Cassage),
        );
        assert_eq!(vec![6, 4], digits(&columns));
        assert_eq!(vec![14, 8], columns[0].terms);
        assert_eq!(
            vec![Rewrite {
                at_column: 0,
                value: 6
            }],
            columns[1].rewrites
        );
        assert_eq!(vec![6, 2], columns[1].terms);

        // 503 − 128: no ten to break, a hundred is broken: 5 → 4, 0 → 9, 3 → 13.
        let columns = written_columns(
            Operator::Subtract,
            &[503, 128],
            Some(SubtractionMethod::Cassage),
        );
        assert_eq!(vec![5, 7, 3], digits(&columns));
        assert_eq!(
            vec![Rewrite {
                at_column: 0,
                value: 4
            }],
            columns[2].rewrites
        );
        assert_eq!(
            vec![Rewrite {
                at_column: 0,
                value: 9
            }],
            columns[1].rewrites
        );
        assert_eq!(vec![9, 2], columns[1].terms);

        // 652 − 358: the 5 of the tens becomes 4, then 14.
        let columns = written_columns(
            Operator::Subtract,
            &[652, 358],
            Some(SubtractionMethod::Cassage),
        );
        assert_eq!(vec![4, 9, 2], digits(&columns));
        let tens: Vec<u8> = columns[1].rewrites.iter().map(|r| r.value).collect();
        assert_eq!(vec![4, 14], tens);
    }

    #[test]
    fn compensation_adds_ten_on_top_and_one_ten_below() {
        // 74 − 28: 14 − 8 = 6, then 7 − (2 + 1) = 4.
        let columns = written_columns(
            Operator::Subtract,
            &[74, 28],
            Some(SubtractionMethod::Compensation),
        );
        assert_eq!(vec![6, 4], digits(&columns));
        assert!(columns[0].plus_ten);
        assert_eq!(1, columns[1].carry);
        assert_eq!(vec![7, 3], columns[1].terms);

        // 503 − 128: twice.
        let columns = written_columns(
            Operator::Subtract,
            &[503, 128],
            Some(SubtractionMethod::Compensation),
        );
        assert_eq!(vec![5, 7, 3], digits(&columns));
        assert_eq!(vec![10, 3], columns[1].terms);
        assert_eq!(vec![5, 2], columns[2].terms);
    }

    #[test]
    fn a_difference_is_written_without_zeros_on_the_left() {
        for method in [SubtractionMethod::Cassage, SubtractionMethod::Compensation] {
            let columns = written_columns(Operator::Subtract, &[652, 618], Some(method));
            assert_eq!(vec![4, 3, 0], digits(&columns));
            assert_eq!(
                vec![true, true, false],
                columns.iter().map(|c| c.written).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn an_equation_gives_its_family() {
        let equation = Equation {
            left: 7,
            operator: Operator::Add,
            right: 8,
            missing: Some(EquationTerm::Right),
            family: true,
            interactive: true,
            choices: vec![7, 8, 9],
        };
        assert_eq!(15, equation.result());
        assert_eq!(Some(8), equation.missing_value());
        let lines: Vec<(u32, Operator, u32, u32)> = equation
            .equalities()
            .iter()
            .map(|e| (e.left, e.operator, e.right, e.result))
            .collect();
        assert_eq!(
            vec![
                (7, Operator::Add, 8, 15),
                (8, Operator::Add, 7, 15),
                (15, Operator::Subtract, 7, 8),
                (15, Operator::Subtract, 8, 7),
            ],
            lines
        );

        // 74 − 28 = 46 is checked by 46 + 28 = 74; a double has 2 lines.
        let subtraction = Equation {
            left: 74,
            operator: Operator::Subtract,
            right: 28,
            missing: None,
            family: true,
            interactive: false,
            choices: Vec::new(),
        };
        assert_eq!(46, subtraction.result());
        assert_eq!(4, subtraction.equalities().len());
        assert!(subtraction.equalities().contains(&Equality {
            left: 28,
            operator: Operator::Add,
            right: 46,
            result: 74
        }));
        let double = Equation {
            left: 6,
            right: 6,
            ..equation
        };
        assert_eq!(2, double.equalities().len());
    }

    #[test]
    fn a_multiplication_only_swaps_its_factors() {
        let equation = Equation {
            left: 8,
            operator: Operator::Multiply,
            right: 5,
            missing: Some(EquationTerm::Left),
            family: true,
            interactive: false,
            choices: Vec::new(),
        };
        assert_eq!(40, equation.result());
        let lines: Vec<(u32, u32, u32)> = equation
            .equalities()
            .iter()
            .map(|e| (e.left, e.right, e.result))
            .collect();
        assert_eq!(vec![(8, 5, 40), (5, 8, 40)], lines);
        assert!(equation
            .equalities()
            .iter()
            .all(|e| e.operator == Operator::Multiply));
    }

    #[test]
    fn a_pick_chain_is_interactive() {
        let animation = Animation::CalcChain {
            chains: vec![chain(42, &[10, -1])],
            mode: ChainMode::Pick,
        };
        assert!(animation.is_interactive());
    }

    #[test]
    fn quiz_answers_are_checked_by_the_core() {
        let lesson = Lesson {
            id: "l".into(),
            subject: Subject::Math,
            grade: Grade::CE1,
            period: 2,
            skill_ids: vec!["s".into()],
            title: "t".into(),
            duration_minutes: 4,
            slides: Vec::new(),
            quiz: vec![QuizQuestion {
                question: "28 + 9 = ?".into(),
                spoken: "Combien font 28 plus 9 ?".into(),
                answer: Answer::Number(37),
                choices: Vec::new(),
                explanation: "28 + 10 = 38, puis 38 − 1 = 37.".into(),
            }],
            sources: Vec::new(),
        };

        assert!(lesson.check_quiz(0, &Answer::Number(37)).unwrap().correct);
        let wrong = lesson.check_quiz(0, &Answer::Number(39)).unwrap();
        assert!(!wrong.correct);
        assert_eq!(Answer::Number(37), wrong.expected);
        assert!(lesson.check_quiz(1, &Answer::Number(37)).is_none());
    }
}
