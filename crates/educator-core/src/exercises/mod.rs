//! Exercise engine (SUB000107): calculations, the space of valid calculations of a skill,
//! series generation and the series in progress. The families of the CE1 complements
//! (SUB000118 to SUB000124) produce the same [`Exercise`]: numeration, doubles and halves,
//! fractions and word problems.

mod answer;
mod calculation;
mod calculation_exercise;
mod double_half;
pub mod exam;
mod exercise;
pub mod expression;
mod family;
mod fractions;
mod generator;
mod groups;
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
pub(crate) use family::{Draw, Position};
pub use generator::{
    build_choices, distinct_count, formats_for, generate_calculations, generate_series,
    generate_series_as, CHOICE_COUNT, DEFAULT_LENGTH, FOCUS_SHARE, MAX_OPERAND_REPEATS,
    RESULT_FORMAT_SHARE,
};
pub use presentation::{BarSegment, FractionShape, Input, Picture, Token};
pub use problems::{
    draw_numbers, draw_numbers_with, elide, fill, FirstName, Noun, ProblemHelp, ProblemKind,
    ProblemSet, ProblemTemplate, ProblemWords, VariableRange, WORD_PLACEHOLDERS,
};
pub use series::{
    AnswerOutcome, NewSeries, Series, SeriesService, SeriesSummary, SkillResult, SERIES_CAPACITY,
};
pub use space::CalculationSpace;
pub use tables::{multiplication_table, TableLine, TABLE_LAST_FACTOR};
