//! CE1 complements (SUB000118 to SUB000124): numbers in words, answers, fractions, expressions and
//! the generation of the new families, on skills written in the tests.

mod common;

use common::skill;
use educator_core::exercises::expression::Expression;
use educator_core::exercises::{
    elide, generate_series, Answer, Comparison, Fraction, Input, Picture, Token, DEFAULT_LENGTH,
};
use educator_core::numbers::{fraction_words, is_correct_spelling, misspellings, spell, Spelling};
use educator_core::skills::{
    AnswerMode, ExerciseFormat, OperandRule, Progression, Skill, SkillOperation,
};
use std::collections::{HashMap, HashSet};

#[test]
fn numbers_are_spelled_traditionally_and_in_1990() {
    let cases = [
        (71, "soixante et onze", "soixante-et-onze"),
        (80, "quatre-vingts", "quatre-vingts"),
        (81, "quatre-vingt-un", "quatre-vingt-un"),
        (200, "deux cents", "deux-cents"),
        (201, "deux cent un", "deux-cent-un"),
        (280, "deux cent quatre-vingts", "deux-cent-quatre-vingts"),
        (1000, "mille", "mille"),
    ];
    for (n, traditional, rectified) in cases {
        assert_eq!(traditional, spell(n, Spelling::Traditional));
        assert_eq!(rectified, spell(n, Spelling::Rectified1990));
        assert!(is_correct_spelling(traditional, n));
        assert!(is_correct_spelling(rectified, n), "{rectified}");
        assert!(is_correct_spelling(&traditional.to_uppercase(), n));
    }
    assert!(!is_correct_spelling("deux cents quatre-vingt", 280));
}

#[test]
fn misspellings_are_never_a_right_spelling() {
    for n in 0..=1000 {
        for wrong in misspellings(n) {
            assert!(!is_correct_spelling(&wrong, n), "{n}: {wrong}");
        }
    }
    let wrong = misspellings(280);
    assert!(
        wrong.contains(&"deux cents quatre-vingt".to_owned()),
        "{wrong:?}"
    );
    assert!(
        wrong.contains(&"deux cent quatre-vingt".to_owned()),
        "{wrong:?}"
    );
}

#[test]
fn fractions_in_words() {
    let cases = [
        (1, 2, "un demi"),
        (1, 3, "un tiers"),
        (2, 3, "deux tiers"),
        (3, 4, "trois quarts"),
        (1, 5, "un cinquième"),
        (5, 8, "cinq huitièmes"),
        (7, 10, "sept dixièmes"),
        (1, 9, "un neuvième"),
    ];
    for (a, b, words) in cases {
        assert_eq!(words, fraction_words(a, b));
    }
}

#[test]
fn any_equal_fraction_is_accepted() {
    let half = Answer::Fraction(Fraction::new(1, 2));
    assert!(half.accepts(&Answer::Fraction(Fraction::new(2, 4))));
    assert!(half.accepts(&Answer::Fraction(Fraction::new(5, 10))));
    assert!(!half.accepts(&Answer::Fraction(Fraction::new(2, 1))));
    assert!(!half.accepts(&Answer::Fraction(Fraction::new(1, 0))));
    assert!(!half.accepts(&Answer::Number(1)));
    assert_eq!(
        Comparison::Greater,
        Fraction::new(1, 3).compare(&Fraction::new(1, 5))
    );
}

#[test]
fn answers_of_every_kind() {
    assert!(Answer::Set(vec![4, 2]).accepts(&Answer::Set(vec![2, 4])));
    assert!(!Answer::Sequence(vec![2, 4]).accepts(&Answer::Sequence(vec![4, 2])));
    assert!(Answer::Words(280).accepts(&Answer::Text("deux-cent-quatre-vingts".to_owned())));
    assert!(!Answer::Words(280).accepts(&Answer::Text("deux cent quatre-vingt".to_owned())));
    assert_eq!("300 et 400", Answer::Sequence(vec![300, 400]).label());
    assert_eq!("<", Answer::Comparison(Comparison::Less).label());
}

#[test]
fn expressions() {
    let values = HashMap::from([("a".to_owned(), 12), ("b".to_owned(), 5)]);
    let eval = |text: &str| Expression::parse(text).unwrap().evaluate(&values);
    assert_eq!(Some(17), eval("a + b"));
    assert_eq!(Some(60), eval("a × b"));
    assert_eq!(Some(2), eval("(a - 2) / b"));
    assert_eq!(None, eval("a / b"));
    assert_eq!(Some(1), eval("b < a"));
    assert_eq!(Some(0), eval("a <= b"));
    assert!(Expression::parse("a +").is_err());
    assert!(Expression::parse("a $ b").is_err());
}

#[test]
fn elision() {
    assert_eq!("Combien d'images ?", elide("Combien de images ?"));
    assert_eq!("plus qu'Inès", elide("plus que Inès"));
    assert_eq!("de billes", elide("de billes"));
}

fn family(id: &str, operation: SkillOperation, formats: Vec<ExerciseFormat>) -> Skill {
    let mut s = skill(id);
    s.operation = operation;
    s.formats = formats;
    s
}

fn every_mode(skill: &Skill, seeds: u64, check: impl Fn(&educator_core::exercises::Exercise)) {
    for seed in 0..seeds {
        for mode in [AnswerMode::Keyboard, AnswerMode::Choice] {
            let series = generate_series(skill, seed, mode, DEFAULT_LENGTH).unwrap();
            assert_eq!(DEFAULT_LENGTH, series.len());
            let keys: HashSet<&String> = series.iter().map(|e| &e.key).collect();
            assert_eq!(DEFAULT_LENGTH, keys.len(), "duplicate in seed {seed}");
            for exercise in &series {
                assert!(exercise.is_correct(&exercise.expected.as_given()));
                check(exercise);
            }
        }
    }
}

#[test]
fn numeration_place_value_and_writing() {
    let mut s = family(
        "num",
        SkillOperation::Numeration,
        vec![
            ExerciseFormat::Recompose,
            ExerciseFormat::Decompose,
            ExerciseFormat::PlaceValue,
            ExerciseFormat::WriteNumber,
            ExerciseFormat::ReadNumber,
        ],
    );
    s.params.numbers = Some(OperandRule::range(100, 999));
    every_mode(&s, 300, |e| {
        if let (
            Some(Picture::BaseTen {
                hundreds,
                tens,
                units,
            }),
            Answer::Number(n),
        ) = (&e.picture, &e.expected)
        {
            assert_eq!(*n as u32, hundreds * 100 + tens * 10 + units);
        }
        if e.format == ExerciseFormat::ReadNumber {
            let choices = e.choices.as_ref().unwrap();
            assert_eq!(
                1,
                choices
                    .iter()
                    .filter(|c| e.is_correct(&c.as_given()))
                    .count()
            );
        }
    });
}

#[test]
fn comparisons_orders_frames_and_lines() {
    let mut s = family(
        "cmp",
        SkillOperation::Numeration,
        vec![
            ExerciseFormat::Compare,
            ExerciseFormat::Order,
            ExerciseFormat::Frame,
            ExerciseFormat::NumberLinePoint,
        ],
    );
    s.params.numbers = Some(OperandRule::range(0, 1000));
    every_mode(&s, 300, |e| {
        match (&e.input, &e.expected, e.statement.as_slice()) {
            (
                Input::Comparison,
                Answer::Comparison(c),
                [Token::Number(a), Token::Slot(0), Token::Number(b)],
            ) => {
                assert_eq!(*c, Comparison::of(a, b));
            }
            (Input::Order { items, ascending }, Answer::Sequence(sorted), _) => {
                let mut expected = items.clone();
                expected.sort_unstable();
                if !ascending {
                    expected.reverse();
                }
                assert_eq!(&expected, sorted);
                assert_ne!(items, sorted, "already ranked");
                assert!((3..=5).contains(&items.len()));
            }
            (
                _,
                Answer::Sequence(bounds),
                [Token::Slot(0), _, Token::Number(n), _, Token::Slot(1)],
            ) => {
                assert!(bounds[0] < *n && *n < bounds[1]);
                assert!(bounds[1] - bounds[0] == 10 || bounds[1] - bounds[0] == 100);
                assert_eq!(0, bounds[0] % (bounds[1] - bounds[0]));
            }
            (_, Answer::Number(value), _) => {
                let Some(Picture::NumberLine {
                    start,
                    step,
                    ticks,
                    arrow,
                    ..
                }) = &e.picture
                else {
                    panic!("number line expected: {e:?}");
                };
                let last = start + step * (*ticks as i32 - 1);
                assert!((*start..=last).contains(value) && (value - start) % step == 0);
                if let Some(k) = arrow {
                    assert_eq!(*value, start + step * *k as i32);
                }
            }
            other => panic!("unexpected {other:?}"),
        }
    });
}

#[test]
fn parity() {
    let mut s = family(
        "par",
        SkillOperation::Numeration,
        vec![ExerciseFormat::YesNo, ExerciseFormat::MultiChoice],
    );
    s.params.numbers = Some(OperandRule::range(0, 1000));
    every_mode(&s, 300, |e| match (&e.input, &e.expected) {
        (Input::MultiChoice { items }, Answer::Set(right)) => {
            let even_asked = e.question.as_ref().unwrap().contains("pairs")
                && !e.question.as_ref().unwrap().contains("impairs");
            let expected: Vec<i32> = items
                .iter()
                .copied()
                .filter(|n| (n % 2 == 0) == even_asked)
                .collect();
            assert_eq!(&expected, right);
        }
        (Input::YesNo, Answer::YesNo(_)) => {}
        other => panic!("unexpected {other:?}"),
    });
}

#[test]
fn doubles_and_halves() {
    let mut s = family(
        "dh",
        SkillOperation::DoubleHalf,
        vec![ExerciseFormat::Phrase, ExerciseFormat::Result],
    );
    s.params.doubles = Some(OperandRule::range(1, 15));
    s.params.halves = Some(OperandRule::multiples(2, 100, 2));
    s.params.times_twenty_five = vec![1, 2, 3, 4];
    every_mode(&s, 300, |e| {
        let question = e.spoken.clone();
        let Answer::Number(answer) = e.expected else {
            panic!()
        };
        let number: i32 = question
            .split(|c: char| !c.is_ascii_digit())
            .find(|w| !w.is_empty())
            .unwrap()
            .parse()
            .unwrap();
        if question.contains("double") {
            assert_eq!(2 * number, answer);
        } else if question.contains("moitié") {
            assert_eq!(number, 2 * answer);
        } else {
            assert_eq!(25 * number, answer);
        }
    });
}

#[test]
fn halves_of_even_tens_come_first() {
    let mut s = family(
        "half",
        SkillOperation::DoubleHalf,
        vec![ExerciseFormat::Phrase],
    );
    s.params.halves = Some(OperandRule::multiples(22, 98, 2));
    s.params.progression = Progression::EasyFirst;
    for seed in 0..200 {
        let series = generate_series(&s, seed, AnswerMode::Keyboard, DEFAULT_LENGTH).unwrap();
        let odd_tens: Vec<bool> = series
            .iter()
            .map(|e| match e.expected {
                Answer::Number(half) => (2 * half) / 10 % 2 == 1,
                _ => panic!(),
            })
            .collect();
        let mut sorted = odd_tens.clone();
        sorted.sort();
        assert_eq!(sorted, odd_tens, "seed {seed}");
    }
}

#[test]
fn fractions() {
    let mut s = family(
        "frac",
        SkillOperation::Fraction,
        vec![
            ExerciseFormat::FractionPicture,
            ExerciseFormat::FractionWords,
            ExerciseFormat::Compare,
            ExerciseFormat::Result,
        ],
    );
    s.params.denominators = vec![2, 3, 4, 5, 6, 8, 10];
    every_mode(&s, 300, |e| {
        match (&e.picture, &e.expected) {
            (Some(Picture::FractionShape { parts, colored, .. }), Answer::Fraction(f)) => {
                assert_eq!((*colored, *parts), (f.numerator, f.denominator));
            }
            (_, Answer::Fraction(f)) => {
                assert!(f.numerator >= 1 && f.numerator <= f.denominator, "{f}");
            }
            (_, Answer::Comparison(_) | Answer::Text(_)) => {}
            other => panic!("unexpected {other:?}"),
        }
        if let Some(choices) = &e.choices {
            assert_eq!(
                1,
                choices
                    .iter()
                    .filter(|c| e.is_correct(&c.as_given()))
                    .count(),
                "{choices:?}"
            );
        }
    });
}

#[test]
fn multiplications_focus_on_the_tables_worked_on() {
    let mut s = skill("mul");
    s.operation = SkillOperation::Multiplication;
    s.left = OperandRule::values(2, 10, vec![2, 3, 4, 5, 10]);
    s.right = OperandRule::range(1, 10);
    s.params.focus = vec![3, 4];
    for seed in 0..500 {
        let series = generate_series(&s, seed, AnswerMode::Choice, DEFAULT_LENGTH).unwrap();
        let focused = series
            .iter()
            .filter(|e| {
                let c = e.calculation.as_ref().unwrap().calculation;
                [3, 4].contains(&c.left) || [3, 4].contains(&c.right)
            })
            .count();
        assert_eq!(7, focused, "seed {seed}");
    }
}

#[test]
fn a_missing_factor_is_never_ambiguous() {
    let mut s = skill("mul0");
    s.operation = SkillOperation::Multiplication;
    s.left = OperandRule::values(2, 10, vec![2, 5, 10]);
    s.right = OperandRule::range(0, 10);
    s.formats = vec![ExerciseFormat::Result, ExerciseFormat::MissingOperand];
    for seed in 0..500 {
        for e in generate_series(&s, seed, AnswerMode::Keyboard, DEFAULT_LENGTH).unwrap() {
            let c = e.calculation.as_ref().unwrap();
            if c.format == ExerciseFormat::MissingOperand {
                assert_ne!(0, c.calculation.result(), "{}", e.prompt());
            }
        }
    }
}

#[test]
fn products_by_decomposition_go_from_easy_to_hard_with_their_help() {
    let mut s = skill("dist");
    s.operation = SkillOperation::Multiplication;
    s.left = OperandRule::range(11, 19);
    s.right = OperandRule::range(2, 9);
    s.formats = vec![ExerciseFormat::Result];
    s.params.progression = Progression::EasyFirst;
    for seed in 0..300 {
        let series = generate_series(&s, seed, AnswerMode::Keyboard, DEFAULT_LENGTH).unwrap();
        let smalls: Vec<i32> = series
            .iter()
            .map(|e| {
                let c = e.calculation.as_ref().unwrap().calculation;
                c.left.min(c.right)
            })
            .collect();
        let rank = |n: i32| {
            if [2, 5].contains(&n) {
                0
            } else if n <= 5 {
                1
            } else {
                2
            }
        };
        let ranks: Vec<i32> = smalls.iter().map(|n| rank(*n)).collect();
        let mut sorted = ranks.clone();
        sorted.sort();
        assert_eq!(sorted, ranks, "seed {seed}");
        for e in &series {
            assert!(matches!(
                e.help,
                Some(Picture::ArrayGrid {
                    split: Some(10),
                    ..
                })
            ));
            assert!(e.explanation.contains(" + "), "{}", e.explanation);
        }
    }
}

#[test]
fn repeated_additions() {
    let mut s = skill("meaning");
    s.operation = SkillOperation::Multiplication;
    s.left = OperandRule::range(2, 5);
    s.right = OperandRule::range(2, 10);
    s.formats = vec![ExerciseFormat::Result, ExerciseFormat::Phrase];
    for seed in 0..200 {
        for e in generate_series(&s, seed, AnswerMode::Keyboard, DEFAULT_LENGTH).unwrap() {
            if e.format == ExerciseFormat::Phrase {
                let c = e.calculation.as_ref().unwrap().calculation;
                let pluses = e
                    .statement
                    .iter()
                    .filter(|t| **t == Token::Sign("+".to_owned()))
                    .count();
                assert_eq!(c.left as usize - 1, pluses, "{}", e.prompt());
            }
        }
    }
}
