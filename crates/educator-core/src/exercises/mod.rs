//! Exercise engine (SUB000107): calculations, the space of valid calculations of a skill,
//! series generation and the series in progress. The families of the CE1 complements
//! (SUB000118 to SUB000124) produce the same [`Exercise`]: numeration, doubles and halves,
//! fractions and word problems.

mod answer;
mod calculation;
mod calculation_exercise;
mod double_half;
mod exercise;
pub mod expression;
mod family;
mod fractions;
mod generator;
mod numeration;
mod presentation;
mod problems;
mod series;
mod space;
mod tables;

pub use answer::{join_numbers, Answer, Comparison, Fraction};
pub use calculation::{ArithmeticOperation, Calculation, DuplicateKey};
pub use calculation_exercise::{CalculationExercise, HiddenPart};
pub use exercise::{Exercise, PLACEHOLDER};
pub use generator::{
    build_choices, distinct_count, generate_calculations, generate_series, CHOICE_COUNT,
    DEFAULT_LENGTH, FOCUS_SHARE, MAX_OPERAND_REPEATS, RESULT_FORMAT_SHARE,
};
pub use presentation::{BarSegment, FractionShape, Input, Picture, Token};
pub use problems::{
    draw_numbers, elide, fill, FirstName, Noun, ProblemHelp, ProblemKind, ProblemSet,
    ProblemTemplate, ProblemWords, VariableRange, WORD_PLACEHOLDERS,
};
pub use series::{AnswerOutcome, Series, SeriesService, SeriesSummary, SERIES_CAPACITY};
pub use space::CalculationSpace;
pub use tables::{multiplication_table, TableLine, TABLE_LAST_FACTOR};
