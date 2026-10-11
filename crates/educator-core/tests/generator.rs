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

// Variety of a series (SUB000112).

fn shape(exercise: &Exercise) -> (ArithmeticOperation, ExerciseFormat) {
    (exercise.calculation.operation, exercise.format())
}

fn variety_skills() -> Vec<educator_core::skills::Skill> {
    let mut addition = skill("addition-both-formats");
    addition.constraints = SkillConstraints {
        max_result: Some(10),
        exclude_trivial: vec![TrivialCase::PlusZero],
        ..Default::default()
    };

    let mut mixed_result = skill("mixed-result-only");
    mixed_result.operation = SkillOperation::Mixed;
    mixed_result.left = OperandRule::range(0, 12);
    mixed_result.right = OperandRule::values(1, 2, vec![1, 2]);
    mixed_result.formats = vec![ExerciseFormat::Result];

    let mut mixed_both = skill("mixed-both-formats");
    mixed_both.operation = SkillOperation::Mixed;
    mixed_both.left = OperandRule::range(0, 20);
    mixed_both.right = OperandRule::range(1, 9);

    let mut subtraction = skill("subtraction-within-10");
    subtraction.operation = SkillOperation::Subtraction;
    subtraction.left = OperandRule::range(2, 10);
    subtraction.right = OperandRule::range(1, 9);
    subtraction.constraints = SkillConstraints {
        exclude_trivial: vec![TrivialCase::MinusZero, TrivialCase::MinusSelf],
        ..Default::default()
    };

    vec![addition, mixed_result, mixed_both, subtraction]
}

#[test]
fn no_duplicate_calculation_in_a_series() {
    for skill in variety_skills() {
        for seed in 0..2000 {
            let series = generate_series(&skill, seed, AnswerMode::Choice, DEFAULT_LENGTH).unwrap();
            let keys: HashSet<_> = series
                .iter()
                .map(|e| e.calculation.duplicate_key())
                .collect();
            assert_eq!(series.len(), keys.len(), "{} seed {seed}", skill.id);
        }
    }
}

#[test]
fn never_twice_the_same_shape_in_a_row_when_the_skill_offers_several() {
    for skill in variety_skills() {
        let shapes = CalculationSpace::of(&skill)
            .calculations()
            .iter()
            .map(|c| c.operation)
            .collect::<HashSet<_>>()
            .len()
            * skill.formats.len();
        if shapes < 2 {
            continue;
        }

        for seed in 0..2000 {
            let series =
                generate_series(&skill, seed, AnswerMode::Keyboard, DEFAULT_LENGTH).unwrap();
            for pair in series.windows(2) {
                assert_ne!(
                    shape(&pair[0]),
                    shape(&pair[1]),
                    "{} seed {seed}: {}",
                    skill.id,
                    describe(&series)
                );
            }
        }
    }
}

#[test]
fn both_formats_alternate_in_a_single_operation_skill() {
    let skill = &variety_skills()[0];
    for seed in 0..500 {
        let series = generate_series(skill, seed, AnswerMode::Keyboard, DEFAULT_LENGTH).unwrap();
        for pair in series.windows(2) {
            assert_ne!(pair[0].format(), pair[1].format(), "seed {seed}");
        }
    }
}

#[test]
fn a_mixed_skill_has_as_many_subtractions_as_additions() {
    for skill in &variety_skills()[1..3] {
        for seed in 0..2000 {
            let series =
                generate_series(skill, seed, AnswerMode::Keyboard, DEFAULT_LENGTH).unwrap();
            let subtractions = series
                .iter()
                .filter(|e| e.calculation.operation == ArithmeticOperation::Subtraction)
                .count();
            assert_eq!(5, subtractions, "{} seed {seed}", skill.id);
        }
    }
}

#[test]
fn an_operand_comes_back_at_most_three_times_when_the_rule_allows_it() {
    for skill in variety_skills() {
        let count_right = skill.right.allowed_count() >= 4;
        for seed in 0..2000 {
            let series =
                generate_series(&skill, seed, AnswerMode::Keyboard, DEFAULT_LENGTH).unwrap();
            let mut uses = std::collections::HashMap::<i32, usize>::new();
            for e in &series {
                let mut operands = vec![e.calculation.left];
                if count_right && e.calculation.right != e.calculation.left {
                    operands.push(e.calculation.right);
                }
                for value in operands {
                    *uses.entry(value).or_default() += 1;
                }
            }

            let most = uses.values().copied().max().unwrap();
            assert!(
                most <= educator_core::exercises::MAX_OPERAND_REPEATS,
                "{} seed {seed}: {}",
                skill.id,
                describe(&series)
            );
        }
    }
}
