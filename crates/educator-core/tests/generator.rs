mod common;

use common::{skill, test_skill};
use educator_core::exercises::{
    build_choices, generate_series, ArithmeticOperation, Calculation, CalculationSpace, Exercise,
    HiddenPart, CHOICE_COUNT, DEFAULT_LENGTH,
};
use educator_core::random::SeededRng;
use educator_core::skills::{
    AnswerMode, CarryRule, ExerciseFormat, OperandRule, SkillConstraints, SkillOperation,
    TrivialCase,
};
use educator_core::Error;
use rand::SeedableRng;
use std::collections::HashSet;

fn describe(series: &[Exercise]) -> String {
    series
        .iter()
        .map(|e| format!("{} {:?}", e.prompt(), e.choices))
        .collect::<Vec<_>>()
        .join(" | ")
}

#[test]
fn same_seed_same_skill_give_the_same_series() {
    let skill = test_skill();

    let first = generate_series(&skill, 42, AnswerMode::Choice, DEFAULT_LENGTH).unwrap();
    let second = generate_series(&skill, 42, AnswerMode::Choice, DEFAULT_LENGTH).unwrap();
    let other = generate_series(&skill, 43, AnswerMode::Choice, DEFAULT_LENGTH).unwrap();

    assert_eq!(describe(&first), describe(&second));
    assert_ne!(describe(&first), describe(&other));
}

#[test]
fn series_has_ten_exercises_without_duplicates() {
    let mut skill = test_skill();
    skill.constraints = SkillConstraints {
        max_result: Some(10),
        exclude_trivial: vec![TrivialCase::PlusZero],
        ..Default::default()
    };

    for seed in 0..500 {
        let series = generate_series(&skill, seed, AnswerMode::Keyboard, DEFAULT_LENGTH).unwrap();

        assert_eq!(DEFAULT_LENGTH, series.len());
        assert_eq!(
            (0..10).collect::<Vec<_>>(),
            series.iter().map(|e| e.index).collect::<Vec<_>>()
        );
        assert_eq!(
            series.len(),
            series
                .iter()
                .map(|e| e.calculation.duplicate_key())
                .collect::<HashSet<_>>()
                .len()
        );
        assert!(series.iter().all(|e| e.choices.is_none()));
    }
}

#[test]
fn constraints_are_always_respected() {
    let mut skill = test_skill();
    skill.operation = SkillOperation::Subtraction;
    skill.left = OperandRule::multiples(20, 90, 10);
    skill.right = OperandRule::range(1, 9);
    skill.constraints = SkillConstraints {
        carry: CarryRule::Required,
        ..Default::default()
    };

    for seed in 0..1000 {
        for e in generate_series(&skill, seed, AnswerMode::Choice, DEFAULT_LENGTH).unwrap() {
            assert_eq!(ArithmeticOperation::Subtraction, e.calculation.operation);
            assert_eq!(0, e.calculation.left % 10);
            assert!((1..=9).contains(&e.calculation.right));
            assert!(e.calculation.has_carry());
            assert!(CalculationSpace::is_valid(&skill, &e.calculation));
        }
    }
}

#[test]
fn formats_follow_the_skill() {
    let mut result_only = test_skill();
    result_only.formats = vec![ExerciseFormat::Result];
    let mut missing_only = skill("missing");
    missing_only.formats = vec![ExerciseFormat::MissingOperand];
    let both = skill("both");

    let all: Vec<Exercise> = (0..200)
        .flat_map(|seed| {
            generate_series(&both, seed, AnswerMode::Keyboard, DEFAULT_LENGTH).unwrap()
        })
        .collect();

    assert!(
        generate_series(&result_only, 1, AnswerMode::Keyboard, DEFAULT_LENGTH)
            .unwrap()
            .iter()
            .all(|e| e.hidden == HiddenPart::Result)
    );
    assert!(
        generate_series(&missing_only, 1, AnswerMode::Keyboard, DEFAULT_LENGTH)
            .unwrap()
            .iter()
            .all(|e| e.hidden != HiddenPart::Result)
    );
    let result_share = all
        .iter()
        .filter(|e| e.hidden == HiddenPart::Result)
        .count() as f64
        / all.len() as f64;
    assert!((0.5..=0.7).contains(&result_share), "{result_share}");
    assert!(all.iter().any(|e| e.hidden == HiddenPart::Left));
    assert!(all.iter().any(|e| e.hidden == HiddenPart::Right));
}

#[test]
fn choices_are_four_credible_different_natural_numbers() {
    let mut small = test_skill();
    small.constraints = SkillConstraints {
        max_result: Some(10),
        ..Default::default()
    };
    let mut sub = skill("sub");
    sub.operation = SkillOperation::Subtraction;
    sub.left = OperandRule::range(0, 20);
    sub.right = OperandRule::range(0, 10);
    let mut mixed = skill("mixed");
    mixed.operation = SkillOperation::Mixed;
    mixed.left = OperandRule::range(100, 999);
    mixed.right = OperandRule::multiples(10, 90, 10);

    for skill in [small, sub, mixed] {
        for seed in 0..300 {
            for exercise in
                generate_series(&skill, seed, AnswerMode::Choice, DEFAULT_LENGTH).unwrap()
            {
                let choices = exercise.choices.clone().unwrap();
                assert_eq!(CHOICE_COUNT, choices.len());
                assert_eq!(choices.len(), choices.iter().collect::<HashSet<_>>().len());
                assert!(choices.iter().all(|c| *c >= 0));
                assert_eq!(
                    1,
                    choices
                        .iter()
                        .filter(|c| **c == exercise.expected())
                        .count()
                );
            }
        }
    }
}

#[test]
fn choices_include_the_classic_mistakes() {
    let exercise = Exercise::new(0, Calculation::addition(27, 15), HiddenPart::Result);

    for seed in 0..50 {
        let choices = build_choices(&exercise, &mut SeededRng::seed_from_u64(seed));

        assert!(choices.contains(&42));
        assert!(choices.iter().any(|c| *c == 41 || *c == 43));
        assert!(choices.iter().any(|c| *c == 32 || *c == 52));
        assert!(choices.contains(&12)); // 27 − 15: inverted operation
    }
}

#[test]
fn choices_for_a_missing_number_include_the_copied_result() {
    let exercise = Exercise::new(0, Calculation::addition(68, 2), HiddenPart::Right);

    for seed in 0..50 {
        let choices = build_choices(&exercise, &mut SeededRng::seed_from_u64(seed));

        assert!(choices.contains(&2));
        assert!(choices.contains(&70));
        assert!(choices.iter().all(|c| (0..=80).contains(c)));
    }
}

#[test]
fn narrow_skill_is_refused() {
    let mut narrow = test_skill();
    narrow.constraints = SkillConstraints {
        exact_result: Some(10),
        ..Default::default()
    };
    narrow.formats = vec![ExerciseFormat::MissingOperand];

    assert_eq!(6, CalculationSpace::of(&narrow).distinct_count()); // 0+10, 1+9 … 5+5
    assert!(matches!(
        generate_series(&narrow, 1, AnswerMode::Keyboard, DEFAULT_LENGTH),
        Err(Error::InvalidSkill(_))
    ));
}
