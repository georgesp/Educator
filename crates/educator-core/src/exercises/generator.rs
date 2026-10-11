use super::{
    ArithmeticOperation, Calculation, CalculationSpace, DuplicateKey, Exercise, HiddenPart,
};
use crate::random::SeededRng;
use crate::skills::{AnswerMode, ExerciseFormat, Skill};
use crate::{Error, Result};
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::collections::HashSet;

pub const DEFAULT_LENGTH: usize = 10;

/// Share of [`ExerciseFormat::Result`] when the skill allows both formats.
pub const RESULT_FORMAT_SHARE: f64 = 0.6;

pub const CHOICE_COUNT: usize = 4;

/// Generates a series of exercises from a skill (SUB000107). Pure and deterministic: the same skill,
/// seed, length and answer mode always give the same series (debugging, tests).
///
/// Fails with [`Error::InvalidSkill`] when the skill allows fewer calculations than `length`.
pub fn generate_series(
    skill: &Skill,
    seed: u64,
    answer_mode: AnswerMode,
    length: usize,
) -> Result<Vec<Exercise>> {
    assert!(length >= 1, "a series has at least one exercise");

    let space = CalculationSpace::of(skill);
    if space.distinct_count() < length {
        return Err(Error::InvalidSkill(format!(
            "Skill {} allows {} different calculations, fewer than {length}.",
            skill.id,
            space.distinct_count()
        )));
    }

    let mut random = SeededRng::seed_from_u64(seed);
    let mut used = HashSet::new();
    let mut exercises = Vec::with_capacity(length);
    for index in 0..length {
        let calculation = pick_unused(space.calculations(), &used, &mut random);
        used.insert(calculation.duplicate_key());
        let hidden = pick_hidden(&skill.formats, &mut random);
        let mut exercise = Exercise::new(index, calculation, hidden);
        if answer_mode == AnswerMode::Choice {
            exercise.choices = Some(build_choices(&exercise, &mut random));
        }

        exercises.push(exercise);
    }

    Ok(exercises)
}

/// Four proposals: the right answer and three credible mistakes (±1, carry error ±10, inverted
/// operation or copied number), never negative, never twice the same, in a random order.
pub fn build_choices(exercise: &Exercise, random: &mut impl Rng) -> Vec<i32> {
    let expected = exercise.expected();
    let groups: [Vec<i32>; 3] = [
        shuffled(vec![expected + 1, expected - 1], random),
        shuffled(vec![expected + 10, expected - 10], random),
        vec![inverted_operation(exercise)],
    ];

    let mut distractors: Vec<i32> = Vec::with_capacity(CHOICE_COUNT - 1);
    let try_add = |distractors: &mut Vec<i32>, candidate: i32| {
        if distractors.len() < CHOICE_COUNT - 1
            && candidate >= 0
            && candidate != expected
            && !distractors.contains(&candidate)
        {
            distractors.push(candidate);
            true
        } else {
            false
        }
    };

    // One from each kind of mistake first, then the remaining candidates, then numbers close to the answer.
    for group in &groups {
        for &candidate in group {
            if try_add(&mut distractors, candidate) {
                break;
            }
        }
    }

    for &candidate in groups.iter().flatten() {
        try_add(&mut distractors, candidate);
    }

    let mut delta = 2;
    while distractors.len() < CHOICE_COUNT - 1 {
        try_add(&mut distractors, expected + delta);
        try_add(&mut distractors, expected - delta);
        delta += 1;
    }

    let mut choices = vec![expected];
    choices.extend(distractors);
    shuffled(choices, random)
}

fn pick_unused(
    calculations: &[Calculation],
    used: &HashSet<DuplicateKey>,
    random: &mut impl Rng,
) -> Calculation {
    // Random draws first; the scan afterwards guarantees termination when few calculations are left.
    for _ in 0..64 {
        let candidate = calculations[random.random_range(0..calculations.len())];
        if !used.contains(&candidate.duplicate_key()) {
            return candidate;
        }
    }

    let start = random.random_range(0..calculations.len());
    (0..calculations.len())
        .map(|offset| calculations[(start + offset) % calculations.len()])
        .find(|candidate| !used.contains(&candidate.duplicate_key()))
        .expect("the distinct count was checked before generating")
}

fn pick_hidden(formats: &[ExerciseFormat], random: &mut impl Rng) -> HiddenPart {
    let result = formats.contains(&ExerciseFormat::Result);
    let missing = formats.contains(&ExerciseFormat::MissingOperand);
    let asks_result = result && (!missing || random.random::<f64>() < RESULT_FORMAT_SHARE);
    if asks_result {
        HiddenPart::Result
    } else if random.random_range(0..2) == 0 {
        HiddenPart::Left
    } else {
        HiddenPart::Right
    }
}

/// A classic mistake: the other operation when the result is asked (27 + 15 → 12); when a number is
/// missing, copying the result (68 + ? = 70 → 70) or, for `? − b = r`, subtracting instead of adding.
fn inverted_operation(exercise: &Exercise) -> i32 {
    let c = &exercise.calculation;
    let addition = c.operation == ArithmeticOperation::Addition;
    match exercise.hidden {
        HiddenPart::Result if addition => (c.left - c.right).abs(),
        HiddenPart::Result => c.left + c.right,
        HiddenPart::Left if !addition => (c.result() - c.right).abs(),
        _ => c.result(),
    }
}

fn shuffled(mut values: Vec<i32>, random: &mut impl Rng) -> Vec<i32> {
    values.shuffle(random);
    values
}
