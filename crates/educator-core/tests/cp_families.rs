//! CP complements (lot 7, US0122): the families of the CE1 used with the numbers and the
//! procedures of the CP, on skills written in the tests.

mod common;

use common::skill;
use educator_core::exercises::{generate_series, Answer, Exercise, Picture, DEFAULT_LENGTH};
use educator_core::numbers::{spell, Spelling};
use educator_core::skills::{
    AnswerMode, ExerciseFormat, GroupKind, GroupsRule, OperandRule, Skill, SkillOperation,
};
use educator_core::Grade;
use std::collections::HashSet;

fn family(id: &str, operation: SkillOperation, formats: Vec<ExerciseFormat>) -> Skill {
    let mut s = skill(id);
    s.grade = Grade::CP;
    s.operation = operation;
    s.formats = formats;
    s
}

fn every_mode(skill: &Skill, seeds: u64, check: impl Fn(&Exercise)) {
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

/// SUB012206: at CP, the half of 52 is explained with halves known at CP (40 + 12), never with the
/// half of an odd number of tens (50 + 2 would need the half of 50).
#[test]
fn cp_halves_are_explained_with_known_halves() {
    let mut s = family(
        "half",
        SkillOperation::DoubleHalf,
        vec![ExerciseFormat::Phrase],
    );
    s.params.halves = Some(OperandRule::multiples(22, 98, 2));
    every_mode(&s, 100, |e| {
        let Answer::Number(half) = e.expected else {
            panic!()
        };
        let n = 2 * half;
        if n % 10 == 0 {
            return;
        }
        if n / 10 % 2 == 1 {
            let tens = (n / 10 - 1) * 10;
            let rest = 10 + n % 10;
            assert!(
                e.explanation
                    .starts_with(&format!("{n} = {tens} + {rest}.")),
                "{}",
                e.explanation
            );
            for odd_tens in [30, 50, 70, 90] {
                assert!(
                    !e.explanation.contains(&format!("la moitié de {odd_tens} ")),
                    "{}",
                    e.explanation
                );
            }
        } else {
            assert!(
                e.explanation
                    .starts_with(&format!("{n} = {} + {}.", n / 10 * 10, n % 10)),
                "{}",
                e.explanation
            );
        }
        assert!(
            e.explanation.ends_with(&format!("= {half}.")),
            "{}",
            e.explanation
        );
    });
}

/// The CE1 keeps its decomposition by rank (the half of 50 is a CE1 fact).
#[test]
fn ce1_halves_keep_the_decomposition_by_rank() {
    let mut s = family(
        "half",
        SkillOperation::DoubleHalf,
        vec![ExerciseFormat::Phrase],
    );
    s.grade = Grade::CE1;
    s.params.halves = Some(OperandRule {
        values: Some(vec![56]),
        ..OperandRule::range(56, 56)
    });
    let series = generate_series(&s, 1, AnswerMode::Keyboard, 1).unwrap();
    assert!(
        series[0].explanation.starts_with("56 = 50 + 6."),
        "{}",
        series[0].explanation
    );
}

fn numeration(numbers: OperandRule, formats: Vec<ExerciseFormat>) -> Skill {
    let mut s = family("num", SkillOperation::Numeration, formats);
    s.params.numbers = Some(numbers);
    s
}

/// SUB012202: the lines of the CP are graduated 1 by 1, between two tens, and the proposals stay
/// within the numbers worked on.
#[test]
fn cp_number_lines_go_one_by_one_between_two_tens() {
    let mut s = numeration(
        OperandRule::range(1, 99),
        vec![ExerciseFormat::NumberLinePoint],
    );
    s.params.line_steps = vec![1];
    every_mode(&s, 200, |e| {
        let Some(Picture::NumberLine { start, step, .. }) = &e.picture else {
            panic!("{e:?}")
        };
        assert_eq!(1, *step);
        assert_eq!(0, start % 10);
        for choice in e.choices.iter().flatten() {
            if let Answer::Number(n) = choice {
                assert!(*n <= 99, "{choice:?}");
            }
        }
    });
}

/// SUB012205: the number of a dictation is never written; it is heard on click, in words.
#[test]
fn a_number_dictation_never_shows_the_number() {
    let s = numeration(
        OperandRule::range(10, 99),
        vec![ExerciseFormat::NumberDictation],
    );
    every_mode(&s, 100, |e| {
        let Answer::Number(n) = e.expected else {
            panic!()
        };
        assert!(ExerciseFormat::NumberDictation.needs_listening());
        assert!(!e.prompt().contains(&n.to_string()), "{}", e.prompt());
        assert!(
            !e.prompt().chars().any(|c| c.is_ascii_digit()),
            "{}",
            e.prompt()
        );
        assert!(
            e.spoken.contains(&spell(n, Spelling::Traditional)),
            "{}",
            e.spoken
        );
        assert!(
            !e.spoken.chars().any(|c| c.is_ascii_digit()),
            "{}",
            e.spoken
        );
    });
}

/// SUB012205: with two-digit numbers, no proposal has three digits (no « centaine » at CP).
#[test]
fn two_digit_numbers_never_propose_hundreds() {
    let s = numeration(
        OperandRule::range(10, 99),
        vec![
            ExerciseFormat::Recompose,
            ExerciseFormat::Decompose,
            ExerciseFormat::PlaceValue,
            ExerciseFormat::NumberDictation,
        ],
    );
    every_mode(&s, 300, |e| {
        assert!(!e.prompt().contains("centaine"), "{}", e.prompt());
        assert!(!e.explanation.contains(" 1 dizaines"), "{}", e.explanation);
        assert!(!e.explanation.contains(" 0 unité"), "{}", e.explanation);
        let Answer::Number(expected) = e.expected else {
            panic!()
        };
        // « soixante-douze » heard as 60 and 12: 612 (815 for 95) is the only three-digit proposal.
        let (t, u) = (expected / 10, expected % 10);
        let heard = (t - 1) * 100 + 10 + u;
        for choice in e.choices.iter().flatten() {
            if let Answer::Number(n) = choice {
                assert!(
                    *n < 100
                        || (e.format == ExerciseFormat::NumberDictation
                            && (t == 7 || t == 9)
                            && *n == heard),
                    "{}: {choice:?}",
                    e.prompt()
                );
            }
        }
    });
}

/// « soixante-douze » heard as 60 and 12: 612 is among the proposals of the dictation.
#[test]
fn seventy_and_ninety_propose_the_heard_mistake() {
    let s = numeration(
        OperandRule {
            values: Some(vec![72]),
            ..OperandRule::range(72, 72)
        },
        vec![ExerciseFormat::NumberDictation],
    );
    let series = generate_series(&s, 3, AnswerMode::Choice, 1).unwrap();
    let choices = series[0].choices.as_ref().unwrap();
    assert!(choices.contains(&Answer::Number(612)), "{choices:?}");
}

/// Comparisons of the CP: the equality stays rare, the reason is written in the singular for 1.
#[test]
fn cp_comparisons_are_rarely_equal() {
    let s = numeration(OperandRule::range(1, 100), vec![ExerciseFormat::Compare]);
    let mut equal = 0;
    let mut total = 0;
    for seed in 0..200 {
        for e in generate_series(&s, seed, AnswerMode::Keyboard, DEFAULT_LENGTH).unwrap() {
            total += 1;
            if e.expected == Answer::Comparison(educator_core::exercises::Comparison::Equal) {
                equal += 1;
            }
            assert!(!e.explanation.contains(" 1 dizaines"), "{}", e.explanation);
            assert!(!e.explanation.contains(" 1 unités"), "{}", e.explanation);
        }
    }
    assert!(equal * 100 < total * 20, "{equal} / {total}");
}

fn groups(kinds: Vec<GroupKind>, max_total: i32) -> Skill {
    let mut s = family(
        "groups",
        SkillOperation::Groups,
        vec![ExerciseFormat::Phrase, ExerciseFormat::Result],
    );
    s.params.groups = Some(GroupsRule {
        kinds,
        count: OperandRule::range(2, 6),
        size: OperandRule::range(2, 10),
        max_total,
    });
    s
}

/// SUB012204, SUB012207: « fois », shares and groups of the CP never show × or ÷, stay within their
/// total, and their answer is checked by the repeated addition of the correction.
#[test]
fn equal_groups_never_use_the_signs_times_or_divide() {
    let s = groups(
        vec![GroupKind::Times, GroupKind::Share, GroupKind::GroupBy],
        30,
    );
    let mut kinds = HashSet::new();
    every_mode(&s, 300, |e| {
        for text in [e.prompt(), e.explanation.clone(), e.spoken.clone()] {
            assert!(!text.contains('×') && !text.contains('÷'), "{text}");
        }
        let Answer::Number(answer) = e.expected else {
            panic!()
        };
        assert!(answer > 0);
        // « 6 + 6 + 6 = 18 » : the terms of the correction add up to the total.
        let sum: i32 = e
            .explanation
            .split(" = ")
            .find(|part| part.contains(" + "))
            .map(|part| {
                part.rsplit(", c'est ")
                    .next()
                    .unwrap()
                    .rsplit(" : ")
                    .next()
                    .unwrap()
                    .split(" + ")
                    .map(|n| n.trim().parse::<i32>().unwrap())
                    .sum()
            })
            .unwrap();
        assert!(sum <= 30, "{}", e.explanation);
        assert!(
            e.explanation.contains(&format!("= {sum}")),
            "{}",
            e.explanation
        );
        // The key gives the calculation: times:3:5, share:18:3, group:24:4.
        let parts: Vec<&str> = e.key.split(':').collect();
        let (x, y): (i32, i32) = (parts[1].parse().unwrap(), parts[2].parse().unwrap());
        match parts[0] {
            "times" => assert_eq!(x * y, answer, "{}", e.key),
            "share" | "group" => assert_eq!(x, answer * y, "{}", e.key),
            other => panic!("{other}"),
        }
    });
    for seed in 0..50 {
        for e in generate_series(&s, seed, AnswerMode::Keyboard, DEFAULT_LENGTH).unwrap() {
            kinds.insert(e.key.split(':').next().unwrap().to_owned());
        }
    }
    assert_eq!(3, kinds.len(), "{kinds:?}");
}

/// « 3 fois 20 = 23 » (the two numbers added) is among the proposals.
#[test]
fn times_propose_the_two_numbers_added() {
    let mut s = groups(vec![GroupKind::Times], 60);
    s.params.groups.as_mut().unwrap().count = OperandRule::range(3, 3);
    s.params.groups.as_mut().unwrap().size = OperandRule {
        values: Some(vec![20]),
        ..OperandRule::range(20, 20)
    };
    s.formats = vec![ExerciseFormat::Result];
    let series = generate_series(&s, 1, AnswerMode::Choice, 1).unwrap();
    assert_eq!(Answer::Number(60), series[0].expected);
    assert!(series[0]
        .choices
        .as_ref()
        .unwrap()
        .contains(&Answer::Number(23)));
    assert_eq!("3 fois 20 = ?", series[0].prompt());
}
