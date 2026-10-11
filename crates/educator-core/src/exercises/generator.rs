use super::{
    ArithmeticOperation, Calculation, CalculationSpace, DuplicateKey, Exercise, HiddenPart,
};
use crate::random::SeededRng;
use crate::skills::{AnswerMode, ExerciseFormat, Skill};
use crate::{Error, Result};
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::collections::{HashMap, HashSet};

pub const DEFAULT_LENGTH: usize = 10;

/// Share of [`ExerciseFormat::Result`] when the skill allows both formats and the previous exercise
/// does not force the format (see [`generate_series`]).
pub const RESULT_FORMAT_SHARE: f64 = 0.6;

pub const CHOICE_COUNT: usize = 4;

/// A number should not come back more than this many times as an operand in one series (SUB000112).
pub const MAX_OPERAND_REPEATS: usize = 3;

/// Operand rules with fewer allowed values than this are not counted by [`MAX_OPERAND_REPEATS`]:
/// « ajouter 1 ou 2 » necessarily repeats 1 and 2.
const COUNTED_OPERAND_MIN_VALUES: i64 = 4;

/// Shape of an exercise: its operation and its format (`7 + 5 = ?` and `8 + 3 = ?` have the same shape).
type Shape = (ArithmeticOperation, ExerciseFormat);

/// Generates a series of exercises from a skill (SUB000107). Pure and deterministic: the same skill,
/// seed, length and answer mode always give the same series (debugging, tests).
///
/// Variety rules (SUB000112):
/// - never twice the same calculation (commuted additions count as the same);
/// - a mixed skill gets as many additions as subtractions (± 1);
/// - never twice in a row the same shape (operation and format) when the skill offers several, so a
///   skill with both formats alternates « résultat » and « nombre manquant »;
/// - a number does not come back more than [`MAX_OPERAND_REPEATS`] times as an operand, when the
///   rule leaves enough other numbers.
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
    let mut best = generate_attempt(skill, &space, answer_mode, length, &mut random);
    // The picking is greedy and can paint itself into a corner (every small number already used
    // three times when the sums must stay under 10): a few more attempts, same seed, still deterministic.
    for _ in 0..VARIETY_ATTEMPTS {
        if !best.1 {
            break;
        }

        best = generate_attempt(skill, &space, answer_mode, length, &mut random);
    }

    Ok(best.0)
}

/// Attempts at a series respecting every variety rule before keeping one that relaxes the operand rule.
const VARIETY_ATTEMPTS: usize = 30;

/// One series, and whether the operand repetition rule had to be relaxed.
fn generate_attempt(
    skill: &Skill,
    space: &CalculationSpace,
    answer_mode: AnswerMode,
    length: usize,
    random: &mut SeededRng,
) -> (Vec<Exercise>, bool) {
    let operations = plan_operations(space.calculations(), &skill.formats, length, random);
    let mut picker = CalculationPicker::new(skill, space.calculations());
    let mut previous: Option<Shape> = None;
    let mut exercises = Vec::with_capacity(length);
    for (index, &operation) in operations.iter().enumerate() {
        let calculation = picker.pick(operation, random);
        let format = pick_format(&skill.formats, calculation.operation, previous, random);
        let hidden = hidden_part(format, random);
        let mut exercise = Exercise::new(index, calculation, hidden);
        if answer_mode == AnswerMode::Choice {
            exercise.choices = Some(build_choices(&exercise, random));
        }

        previous = Some((calculation.operation, format));
        exercises.push(exercise);
    }

    (exercises, picker.relaxed)
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

/// The operation of each exercise. A skill with one operation (or whose constraints leave only one)
/// gets it everywhere. A mixed skill gets half additions, half subtractions: alternating when the
/// skill has a single format (otherwise two exercises in a row would have the same shape), shuffled
/// otherwise (the format then changes between two exercises of the same operation).
fn plan_operations(
    calculations: &[Calculation],
    formats: &[ExerciseFormat],
    length: usize,
    random: &mut impl Rng,
) -> Vec<ArithmeticOperation> {
    let has = |operation| calculations.iter().any(|c| c.operation == operation);
    let (add, sub) = (
        ArithmeticOperation::Addition,
        ArithmeticOperation::Subtraction,
    );
    match (has(add), has(sub)) {
        (true, false) => return vec![add; length],
        (false, true) => return vec![sub; length],
        _ => {}
    }

    let (first, second) = if random.random_range(0..2) == 0 {
        (add, sub)
    } else {
        (sub, add)
    };
    let mut operations: Vec<ArithmeticOperation> = (0..length)
        .map(|i| if i % 2 == 0 { first } else { second })
        .collect();
    if formats.len() > 1 {
        operations.shuffle(random);
    }

    operations
}

/// Picks the calculations of a series: never twice the same, and avoiding operands already used
/// [`MAX_OPERAND_REPEATS`] times.
struct CalculationPicker<'a> {
    calculations: &'a [Calculation],
    used: HashSet<DuplicateKey>,
    operand_uses: HashMap<i32, usize>,
    count_left: bool,
    count_right: bool,
    /// The operand repetition rule could not be respected for at least one calculation.
    relaxed: bool,
}

impl<'a> CalculationPicker<'a> {
    fn new(skill: &Skill, calculations: &'a [Calculation]) -> Self {
        Self {
            calculations,
            used: HashSet::new(),
            operand_uses: HashMap::new(),
            count_left: skill.left.allowed_count() >= COUNTED_OPERAND_MIN_VALUES,
            count_right: skill.right.allowed_count() >= COUNTED_OPERAND_MIN_VALUES,
            relaxed: false,
        }
    }

    /// Operands of `calculation` counted for the repetition rule (once each).
    fn counted_operands(&self, calculation: &Calculation) -> Vec<i32> {
        let mut operands = Vec::with_capacity(2);
        if self.count_left {
            operands.push(calculation.left);
        }

        if self.count_right && !operands.contains(&calculation.right) {
            operands.push(calculation.right);
        }

        operands
    }

    fn pick(&mut self, operation: ArithmeticOperation, random: &mut impl Rng) -> Calculation {
        let fresh = |c: &Calculation| !self.used.contains(&c.duplicate_key());
        let planned = |c: &Calculation| c.operation == operation && fresh(c);
        let varied = |c: &Calculation| {
            planned(c)
                && self
                    .counted_operands(c)
                    .iter()
                    .all(|v| self.operand_uses.get(v).copied().unwrap_or(0) < MAX_OPERAND_REPEATS)
        };

        // From the strictest wish to the only hard rule (no duplicate); the distinct count was
        // checked before generating, so the last one always finds a calculation.
        let strict = find_random(self.calculations, &varied, random);
        let calculation = match strict {
            Some(calculation) => calculation,
            None => {
                self.relaxed = true;
                find_random(self.calculations, &planned, random)
                    .or_else(|| find_random(self.calculations, &fresh, random))
                    .expect("the distinct count was checked before generating")
            }
        };

        self.used.insert(calculation.duplicate_key());
        for value in self.counted_operands(&calculation) {
            *self.operand_uses.entry(value).or_insert(0) += 1;
        }

        calculation
    }
}

/// A random calculation satisfying `accept`: random draws first, then a scan from a random start,
/// which guarantees termination when few calculations are left.
fn find_random(
    calculations: &[Calculation],
    accept: &impl Fn(&Calculation) -> bool,
    random: &mut impl Rng,
) -> Option<Calculation> {
    for _ in 0..64 {
        let candidate = calculations[random.random_range(0..calculations.len())];
        if accept(&candidate) {
            return Some(candidate);
        }
    }

    let start = random.random_range(0..calculations.len());
    (0..calculations.len())
        .map(|offset| calculations[(start + offset) % calculations.len()])
        .find(|candidate| accept(candidate))
}

/// The format of the next exercise: drawn ([`RESULT_FORMAT_SHARE`]) when the skill allows both,
/// except that it changes when the previous exercise had the same operation and the drawn format.
fn pick_format(
    formats: &[ExerciseFormat],
    operation: ArithmeticOperation,
    previous: Option<Shape>,
    random: &mut impl Rng,
) -> ExerciseFormat {
    let result = formats.contains(&ExerciseFormat::Result);
    let missing = formats.contains(&ExerciseFormat::MissingOperand);
    if !(result && missing) {
        return if result {
            ExerciseFormat::Result
        } else {
            ExerciseFormat::MissingOperand
        };
    }

    let drawn = if random.random::<f64>() < RESULT_FORMAT_SHARE {
        ExerciseFormat::Result
    } else {
        ExerciseFormat::MissingOperand
    };
    match previous {
        Some(shape) if shape == (operation, drawn) => match drawn {
            ExerciseFormat::Result => ExerciseFormat::MissingOperand,
            ExerciseFormat::MissingOperand => ExerciseFormat::Result,
        },
        _ => drawn,
    }
}

fn hidden_part(format: ExerciseFormat, random: &mut impl Rng) -> HiddenPart {
    match format {
        ExerciseFormat::Result => HiddenPart::Result,
        ExerciseFormat::MissingOperand if random.random_range(0..2) == 0 => HiddenPart::Left,
        ExerciseFormat::MissingOperand => HiddenPart::Right,
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
