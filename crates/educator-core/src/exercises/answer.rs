use crate::numbers::{fraction_words, is_correct_spelling, normalize_words, spell, Spelling};
use crate::spelling::is_right_spelling;
use std::fmt;

/// A fraction `numerator / denominator` with natural numbers (SUB000123). Never stored as a float:
/// two fractions are equal when their cross products are ([`Fraction::same_value`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fraction {
    pub numerator: u32,
    pub denominator: u32,
}

impl Fraction {
    pub const fn new(numerator: u32, denominator: u32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    /// Same value: 2/4 and 1/2. A zero denominator has no value.
    pub fn same_value(&self, other: &Fraction) -> bool {
        self.denominator != 0
            && other.denominator != 0
            && u64::from(self.numerator) * u64::from(other.denominator)
                == u64::from(other.numerator) * u64::from(self.denominator)
    }

    /// Compares the values (cross products).
    pub fn compare(&self, other: &Fraction) -> Comparison {
        let left = u64::from(self.numerator) * u64::from(other.denominator);
        let right = u64::from(other.numerator) * u64::from(self.denominator);
        Comparison::of(left, right)
    }

    /// « trois quarts ».
    pub fn words(&self) -> String {
        fraction_words(self.numerator, self.denominator)
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

/// Result of a comparison: the sign written between two numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Comparison {
    Less,
    Greater,
    Equal,
}

impl Comparison {
    pub fn of<T: Ord>(left: T, right: T) -> Self {
        match left.cmp(&right) {
            std::cmp::Ordering::Less => Comparison::Less,
            std::cmp::Ordering::Greater => Comparison::Greater,
            std::cmp::Ordering::Equal => Comparison::Equal,
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Comparison::Less => "<",
            Comparison::Greater => ">",
            Comparison::Equal => "=",
        }
    }

    /// The sign the other way round (classic mistake).
    pub fn inverted(&self) -> Self {
        match self {
            Comparison::Less => Comparison::Greater,
            Comparison::Greater => Comparison::Less,
            Comparison::Equal => Comparison::Equal,
        }
    }
}

/// An answer: the one expected by an exercise, or the one given by the child.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Answer {
    Number(i32),
    /// Any equal fraction is accepted (decision of 2026-10-11): 2/4 for 1/2.
    Fraction(Fraction),
    Comparison(Comparison),
    YesNo(bool),
    /// Numbers in a given order: ranking (« range ») or framing (lower bound, upper bound).
    Sequence(Vec<i32>),
    /// Numbers in any order: « clique sur tous les nombres pairs ».
    Set(Vec<i32>),
    /// A number written in words: the traditional spelling is shown, the 1990 one is accepted too.
    Words(i32),
    /// A text to pick, compared after normalisation (fraction words).
    Text(String),
    /// A word or letters to write (spelling, SUB001405): no tolerance on accents, capitals
    /// tolerated. The child's answer is a [`Answer::Text`].
    Spelling(String),
}

impl Answer {
    /// Whether `given` is right when `self` is expected. A given answer of another kind is wrong.
    pub fn accepts(&self, given: &Answer) -> bool {
        match (self, given) {
            (Answer::Number(a), Answer::Number(b)) => a == b,
            (Answer::Fraction(a), Answer::Fraction(b)) => a.same_value(b),
            (Answer::Comparison(a), Answer::Comparison(b)) => a == b,
            (Answer::YesNo(a), Answer::YesNo(b)) => a == b,
            (Answer::Sequence(a), Answer::Sequence(b)) => a == b,
            (Answer::Set(a), Answer::Set(b)) => sorted(a) == sorted(b),
            (Answer::Words(n), Answer::Text(text)) => is_correct_spelling(text, *n),
            (Answer::Words(a), Answer::Words(b)) => a == b,
            (Answer::Text(a), Answer::Text(b)) => normalize_words(a) == normalize_words(b),
            (Answer::Spelling(a), Answer::Text(b) | Answer::Spelling(b)) => is_right_spelling(a, b),
            _ => false,
        }
    }

    /// Short written form, shown on a choice tile and in the correction: « 3/4 », « < », « oui »,
    /// « 300 et 400 », « deux cent quatre-vingts ».
    pub fn label(&self) -> String {
        match self {
            Answer::Number(n) => n.to_string(),
            Answer::Fraction(f) => f.to_string(),
            Answer::Comparison(c) => c.symbol().to_owned(),
            Answer::YesNo(true) => "oui".to_owned(),
            Answer::YesNo(false) => "non".to_owned(),
            Answer::Sequence(values) | Answer::Set(values) => join_numbers(values),
            Answer::Words(n) => spell(*n, Spelling::Traditional),
            Answer::Text(text) | Answer::Spelling(text) => text.clone(),
        }
    }

    /// The answer as given by the child for a pick: a [`Answer::Words`] proposal is sent as text.
    pub fn as_given(&self) -> Answer {
        match self {
            Answer::Words(n) => Answer::Text(spell(*n, Spelling::Traditional)),
            Answer::Spelling(text) => Answer::Text(text.clone()),
            other => other.clone(),
        }
    }
}

fn sorted(values: &[i32]) -> Vec<i32> {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    sorted
}

/// « 3, 4 et 5 ».
pub fn join_numbers(values: &[i32]) -> String {
    match values {
        [] => String::new(),
        [only] => only.to_string(),
        [rest @ .., last] => format!(
            "{} et {last}",
            rest.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}
