//! Exercise engine (SUB000107): calculations, the space of valid calculations of a skill,
//! series generation and the series in progress.

mod calculation;
mod exercise;
mod generator;
mod series;
mod space;

pub use calculation::{ArithmeticOperation, Calculation, DuplicateKey};
pub use exercise::{Exercise, HiddenPart, PLACEHOLDER};
pub use generator::{
    build_choices, generate_series, CHOICE_COUNT, DEFAULT_LENGTH, MAX_OPERAND_REPEATS,
    RESULT_FORMAT_SHARE,
};
pub use series::{AnswerOutcome, Series, SeriesService, SeriesSummary, SERIES_CAPACITY};
pub use space::CalculationSpace;
