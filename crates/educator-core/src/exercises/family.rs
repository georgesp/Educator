//! Series of the families other than the calculations (numeration, doubles and halves, fractions,
//! problems): the common loop (formats, no duplicate, progression) and the choice helpers.

use super::{double_half, fractions, groups, numeration, problems, Answer, Exercise, CHOICE_COUNT};
use crate::random::SeededRng;
use crate::skills::{AnswerMode, ExerciseFormat, Progression, Skill, SkillOperation};
use crate::{Error, Result};
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::collections::HashSet;

/// Where an exercise stands in its series (the problems use smaller numbers at the start).
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub index: usize,
    pub length: usize,
}

impl Position {
    /// In the first part of the series (60 %), where the problems keep the smaller numbers.
    pub fn is_early(&self) -> bool {
        self.index * 10 < self.length * 6
    }
}

/// A drawn exercise and its difficulty (0 = easiest), used by [`Progression::EasyFirst`].
#[derive(Debug, Clone)]
pub struct Draw {
    pub exercise: Exercise,
    pub difficulty: u8,
}

/// Draws one exercise of a family for a format, or `None` when this draw failed (another is tried).
type DrawFn = fn(&Skill, ExerciseFormat, AnswerMode, Position, &mut SeededRng) -> Option<Draw>;

fn draw_function(operation: SkillOperation) -> Option<DrawFn> {
    match operation {
        SkillOperation::Numeration => Some(numeration::draw),
        SkillOperation::DoubleHalf => Some(double_half::draw),
        SkillOperation::Groups => Some(groups::draw),
        SkillOperation::Fraction => Some(fractions::draw),
        SkillOperation::Problem => Some(problems::draw),
        SkillOperation::Spelling => Some(crate::spelling::draw),
        _ => None,
    }
}

/// Draws tried for one exercise before the series is declared impossible.
const DRAWS_PER_EXERCISE: usize = 400;

/// Generates a series of a non-calculation family. Same rules as the calculations (SUB000112): no
/// duplicate, never twice in a row the same format when the skill has several, balanced formats.
pub fn generate(
    skill: &Skill,
    seed: u64,
    answer_mode: AnswerMode,
    length: usize,
    formats: &[ExerciseFormat],
) -> Result<Vec<Exercise>> {
    let draw = draw_function(skill.operation)
        .ok_or_else(|| Error::InvalidSkill(format!("Skill {} is not a family skill.", skill.id)))?;
    if formats.is_empty() {
        return Err(Error::InvalidSkill(format!(
            "Skill {} has no format for the answer mode {answer_mode:?}.",
            skill.id
        )));
    }

    let mut random = SeededRng::seed_from_u64(seed);
    let formats = plan_formats(formats, length, &mut random);
    let mut keys: HashSet<String> = HashSet::new();
    let mut draws: Vec<Draw> = Vec::with_capacity(length);
    for (index, &format) in formats.iter().enumerate() {
        let position = Position { index, length };
        let mut found = None;
        for _ in 0..DRAWS_PER_EXERCISE {
            if let Some(candidate) = draw(skill, format, answer_mode, position, &mut random) {
                if !keys.contains(&candidate.exercise.key) {
                    found = Some(candidate);
                    break;
                }
            }
        }

        let found = found.ok_or_else(|| {
            Error::InvalidSkill(format!(
                "Skill {} cannot produce {length} different exercises.",
                skill.id
            ))
        })?;
        keys.insert(found.exercise.key.clone());
        draws.push(found);
    }

    if skill.params.progression == Progression::EasyFirst {
        // Stable: same difficulty keeps the drawn order (and the format alternation as far as possible).
        draws.sort_by_key(|d| d.difficulty);
    }

    Ok(draws
        .into_iter()
        .enumerate()
        .map(|(index, d)| Exercise {
            index,
            ..d.exercise
        })
        .collect())
}

/// Number of different exercises a family skill can produce (keys of many draws, capped).
pub fn distinct_count(skill: &Skill) -> usize {
    const DRAWS: usize = 4000;
    let Some(draw) = draw_function(skill.operation) else {
        return 0;
    };
    let mut random = SeededRng::seed_from_u64(0x5EED);
    let mut keys = HashSet::new();
    for i in 0..DRAWS {
        let mode = skill.answer_modes[i % skill.answer_modes.len()];
        let formats = super::formats_for(skill, mode, None);
        let Some(&format) = formats.get(i / skill.answer_modes.len() % formats.len().max(1)) else {
            continue;
        };
        let position = Position {
            index: i % 10,
            length: 10,
        };
        if let Some(d) = draw(skill, format, mode, position, &mut random) {
            keys.insert(d.exercise.key);
        }
    }

    keys.len()
}

/// Formats of the series: every format as often as the others (± 1), shuffled, never twice the same
/// in a row when the skill has several.
fn plan_formats(
    formats: &[ExerciseFormat],
    length: usize,
    random: &mut impl Rng,
) -> Vec<ExerciseFormat> {
    if formats.len() == 1 {
        return vec![formats[0]; length];
    }

    let mut planned: Vec<ExerciseFormat> = Vec::with_capacity(length);
    while planned.len() < length {
        let mut round = formats.to_vec();
        round.shuffle(random);
        if planned.last() == round.first() {
            round.rotate_left(1);
        }
        planned.extend(round);
    }

    planned.truncate(length);
    planned
}

/// Four number proposals (the right one first in `candidates` order of credibility, then numbers
/// close to the answer), natural numbers only, without duplicates, shuffled.
pub fn number_choices(expected: i32, candidates: &[i32], random: &mut impl Rng) -> Vec<Answer> {
    let mut picked: Vec<i32> = Vec::with_capacity(CHOICE_COUNT);
    for &candidate in candidates {
        if picked.len() == CHOICE_COUNT - 1 {
            break;
        }
        if candidate >= 0 && candidate != expected && !picked.contains(&candidate) {
            picked.push(candidate);
        }
    }

    let mut delta = 1;
    while picked.len() < CHOICE_COUNT - 1 {
        for candidate in [expected + delta, expected - delta] {
            if picked.len() < CHOICE_COUNT - 1 && candidate >= 0 && !picked.contains(&candidate) {
                picked.push(candidate);
            }
        }
        delta += 1;
    }

    let mut choices: Vec<Answer> = std::iter::once(expected)
        .chain(picked)
        .map(Answer::Number)
        .collect();
    choices.shuffle(random);
    choices
}

/// Proposals among answers of any kind: the right one and up to three wrong ones (never accepted
/// as right), shuffled. At least `minimum` proposals, otherwise `None`.
pub fn answer_choices(
    expected: &Answer,
    candidates: Vec<Answer>,
    minimum: usize,
    random: &mut impl Rng,
) -> Option<Vec<Answer>> {
    let mut choices = vec![expected.clone()];
    for candidate in candidates {
        if choices.len() == CHOICE_COUNT {
            break;
        }
        let given = candidate.as_given();
        if !expected.accepts(&given)
            && !choices
                .iter()
                .any(|c| c.as_given().accepts(&given) || *c == candidate)
        {
            choices.push(candidate);
        }
    }

    if choices.len() < minimum {
        return None;
    }

    choices.shuffle(random);
    Some(choices)
}

/// A random allowed value of an operand rule.
pub fn pick_value(values: &[i32], random: &mut impl Rng) -> Option<i32> {
    (!values.is_empty()).then(|| values[random.random_range(0..values.len())])
}
