//! Doubles and halves (SUB000122): the facts of the programme and the half of an even number by
//! decomposing it (46 = 40 + 6, so 23).

use super::family::{number_choices, pick_value, Draw, Position};
use super::{Answer, Exercise, Input, Token};
use crate::random::SeededRng;
use crate::skills::{AnswerMode, ExerciseFormat, Skill};
use crate::Grade;
use rand::Rng;

/// How the half of a number is decomposed in the explanation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HalfSplit {
    /// By rank: 56 = 50 + 6, so 25 + 3 (CE1: the half of 50 is a fact to know).
    ByRank,
    /// Into halves known at CP (SUB012206): an even number of tens and the rest, at most 20:
    /// 56 = 40 + 16, so 20 + 8.
    KnownHalves,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fact {
    Double(i32),
    Half(i32),
    TimesTwentyFive(i32),
}

pub fn draw(
    skill: &Skill,
    format: ExerciseFormat,
    mode: AnswerMode,
    _position: Position,
    random: &mut SeededRng,
) -> Option<Draw> {
    let params = &skill.params;
    let doubles = params
        .doubles
        .as_ref()
        .map(|r| r.allowed_values())
        .unwrap_or_default();
    let halves: Vec<i32> = params
        .halves
        .as_ref()
        .map(|r| r.allowed_values())
        .unwrap_or_default()
        .into_iter()
        .filter(|n| n % 2 == 0)
        .collect();
    let times = &params.times_twenty_five;

    // Each kind of fact in proportion of how many there are, the multiples of 25 a little less.
    let total = doubles.len() + halves.len() + times.len();
    if total == 0 {
        return None;
    }
    let pick = random.random_range(0..total);
    let fact = if pick < doubles.len() {
        Fact::Double(pick_value(&doubles, random)?)
    } else if pick < doubles.len() + halves.len() {
        Fact::Half(pick_value(&halves, random)?)
    } else {
        Fact::TimesTwentyFive(pick_value(times, random)?)
    };

    let split = if skill.grade == Grade::CP {
        HalfSplit::KnownHalves
    } else {
        HalfSplit::ByRank
    };
    let exercise = build(fact, format, mode, split, random)?;
    Some(Draw {
        difficulty: difficulty(fact),
        exercise,
    })
}

/// Halves of even tens first (46, 84), then the others (56), then beyond 100.
fn difficulty(fact: Fact) -> u8 {
    match fact {
        Fact::Half(n) => {
            let odd_tens = u8::from(n / 10 % 2 == 1);
            let big = u8::from(n > 100);
            odd_tens + 2 * big
        }
        _ => 0,
    }
}

/// The half of an even number taken digit by digit, rounding down: 56 → 23 (classic mistake).
fn digit_by_digit_half(n: i32) -> i32 {
    let mut result = 0;
    let mut rank = 1;
    let mut rest = n;
    while rest > 0 {
        result += (rest % 10) / 2 * rank;
        rest /= 10;
        rank *= 10;
    }
    result
}

/// The parts of `n` whose halves are taken: `[40, 6]` for 46; for 56, `[50, 6]` by rank or
/// `[40, 16]` into known halves.
fn half_parts(n: i32, split: HalfSplit) -> Vec<i32> {
    let tens = n / 10 % 10;
    let parts = if split == HalfSplit::KnownHalves && n < 100 && n > 20 && tens % 2 == 1 {
        [0, (tens - 1) * 10, 10 + n % 10]
    } else {
        [n / 100 * 100, tens * 10, n % 10]
    };
    parts.into_iter().filter(|p| *p > 0).collect()
}

/// « 46 = 40 + 6. La moitié de 40 est 20, la moitié de 6 est 3. La moitié de 46 est 20 + 3 = 23. »
fn half_explanation(n: i32, split: HalfSplit) -> String {
    let parts = half_parts(n, split);
    if parts.len() < 2 {
        return format!(
            "La moitié de {n}, c'est {}, car {} + {} = {n}.",
            n / 2,
            n / 2,
            n / 2
        );
    }

    let sum = parts
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" + ");
    let halves = parts
        .iter()
        .map(|p| format!("la moitié de {p} est {}", p / 2))
        .collect::<Vec<_>>()
        .join(", ");
    let half_sum = parts
        .iter()
        .map(|p| (p / 2).to_string())
        .collect::<Vec<_>>()
        .join(" + ");
    format!(
        "{n} = {sum}. {}. La moitié de {n} est {half_sum} = {}.",
        capitalize(&halves),
        n / 2
    )
}

fn capitalize(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn build(
    fact: Fact,
    format: ExerciseFormat,
    mode: AnswerMode,
    split: HalfSplit,
    random: &mut SeededRng,
) -> Option<Exercise> {
    let sign = |s: &str| Token::Sign(s.to_owned());
    let (expected, question, statement, explanation, mistakes, key) = match fact {
        Fact::Double(n) => (
            2 * n,
            format!("Quel est le double de {n} ?"),
            vec![
                Token::Text("Le double de".to_owned()),
                Token::Number(n),
                sign("="),
                Token::Slot(0),
            ],
            format!("Le double de {n}, c'est {n} + {n} = {}.", 2 * n),
            vec![n / 2, n + 2, 2 * n + 10, 2 * n - 10, 2 * n + 1],
            format!("double:{n}"),
        ),
        Fact::Half(n) => (
            n / 2,
            format!("Quelle est la moitié de {n} ?"),
            vec![
                Token::Text("La moitié de".to_owned()),
                Token::Number(n),
                sign("="),
                Token::Slot(0),
            ],
            half_explanation(n, split),
            match split {
                // CP: the half of the tens only (46 → 20), never a number of three digits.
                HalfSplit::KnownHalves => [
                    n / 10 * 10 / 2,
                    digit_by_digit_half(n),
                    2 * n,
                    n / 2 + 10,
                    n / 2 - 10,
                    n / 2 + 1,
                ]
                .into_iter()
                .filter(|m| *m < 100 && *m != n / 2)
                .collect(),
                HalfSplit::ByRank => vec![
                    digit_by_digit_half(n),
                    2 * n,
                    n / 2 + 10,
                    n / 2 - 10,
                    n / 2 + 1,
                ],
            },
            format!("half:{n}"),
        ),
        Fact::TimesTwentyFive(k) => (
            25 * k,
            format!("Combien font {k} fois 25 ?"),
            vec![
                Token::Number(k),
                sign("×"),
                Token::Number(25),
                sign("="),
                Token::Slot(0),
            ],
            format!(
                "{k} × 25 = {} : {}",
                25 * k,
                vec!["25"; k.max(1) as usize].join(" + ")
            ) + &format!(" = {}.", 25 * k),
            vec![25 + k, 25 * (k + 1), 25 * (k - 1), 20 * k],
            format!("times25:{k}"),
        ),
    };

    // CP: the numbers stay within 100 (SUB012206).
    let mistakes: Vec<i32> = match split {
        HalfSplit::KnownHalves => mistakes.into_iter().filter(|m| *m <= 100).collect(),
        HalfSplit::ByRank => mistakes,
    };
    let phrase = format == ExerciseFormat::Phrase;
    let (input, choices) = match mode {
        AnswerMode::Keyboard => (Input::Number { slots: 1 }, None),
        AnswerMode::Choice => (
            Input::Pick,
            Some(number_choices(expected, &mistakes, random)),
        ),
    };
    let (question_shown, statement) = if phrase {
        (Some(question.clone()), vec![Token::Slot(0)])
    } else {
        (None, statement)
    };
    Some(Exercise {
        index: 0,
        format,
        question: question_shown,
        statement,
        picture: None,
        input,
        expected: Answer::Number(expected),
        choices,
        spoken: question,
        explanation,
        help: None,
        key,
        calculation: None,
        word: None,
    })
}
