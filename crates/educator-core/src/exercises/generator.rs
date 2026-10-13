use super::{
    family, ArithmeticOperation, Calculation, CalculationExercise, CalculationSpace, DuplicateKey,
    Exercise, HiddenPart, Picture,
};
use crate::random::SeededRng;
use crate::skills::{AnswerMode, ExerciseFormat, Progression, Skill, SkillPicture};
use crate::{Error, Result};
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::collections::{HashMap, HashSet};

pub const DEFAULT_LENGTH: usize = 10;

/// Share of [`ExerciseFormat::Result`] when the skill allows both formats and the previous exercise
/// does not force the format (see [`generate_calculations`]).
pub const RESULT_FORMAT_SHARE: f64 = 0.6;

pub const CHOICE_COUNT: usize = 4;

/// A number should not come back more than this many times as an operand in one series (SUB000112).
pub const MAX_OPERAND_REPEATS: usize = 3;

/// Share of the exercises on the tables worked on (`focus`), the others revise the other tables.
pub const FOCUS_SHARE: f64 = 0.7;

/// Share of the multiplications with a missing factor written the other way round (`30 = 3 × ?`).
pub const REVERSED_SHARE: f64 = 0.35;

/// Operand rules with fewer allowed values than this are not counted by [`MAX_OPERAND_REPEATS`]:
/// « ajouter 1 ou 2 » necessarily repeats 1 and 2.
const COUNTED_OPERAND_MIN_VALUES: i64 = 4;

/// Shape of an exercise: its operation and its format (`7 + 5 = ?` and `8 + 3 = ?` have the same shape).
type Shape = (ArithmeticOperation, ExerciseFormat);

/// Generates a series of exercises from a skill, whatever its family (calculations, numeration,
/// doubles and halves, fractions, problems). Pure and deterministic: the same skill, seed, length
/// and answer mode always give the same series (debugging, tests).
pub fn generate_series(
    skill: &Skill,
    seed: u64,
    answer_mode: AnswerMode,
    length: usize,
) -> Result<Vec<Exercise>> {
    generate_series_as(skill, seed, answer_mode, length, None)
}

/// The formats of a skill a series may use: `format` alone when given (the kind of exercise chosen
/// by the child), otherwise every format of the skill the answer mode allows. A dictation is
/// written, « Choisis la bonne écriture » is picked, every other format suits both modes.
pub fn formats_for(
    skill: &Skill,
    answer_mode: AnswerMode,
    format: Option<ExerciseFormat>,
) -> Vec<ExerciseFormat> {
    let allowed = |f: &ExerciseFormat| match f {
        ExerciseFormat::Dictation => answer_mode == AnswerMode::Keyboard,
        ExerciseFormat::ChooseSpelling => answer_mode == AnswerMode::Choice,
        _ => true,
    };
    skill
        .formats
        .iter()
        .copied()
        .filter(|f| format.is_none_or(|wanted| wanted == *f))
        .filter(allowed)
        .collect()
}

/// [`generate_series`] of one kind of exercise (`format`), or of every format of the skill allowed
/// by the answer mode.
pub fn generate_series_as(
    skill: &Skill,
    seed: u64,
    answer_mode: AnswerMode,
    length: usize,
    format: Option<ExerciseFormat>,
) -> Result<Vec<Exercise>> {
    if !skill.operation.is_calculation() {
        return family::generate(
            skill,
            seed,
            answer_mode,
            length,
            &formats_for(skill, answer_mode, format),
        );
    }

    let picture = skill.params.picture;
    Ok(generate_calculations(skill, seed, answer_mode, length)?
        .iter()
        .map(|e| {
            let shown = match picture {
                Some(SkillPicture::ArrayGrid) if e.format == ExerciseFormat::Result => {
                    Some(Picture::ArrayGrid {
                        rows: e.calculation.left.unsigned_abs(),
                        columns: e.calculation.right.unsigned_abs(),
                        split: None,
                    })
                }
                _ => None,
            };
            e.to_exercise(shown)
        })
        .collect())
}

/// Number of different exercises a skill can produce (commuted calculations counted once).
pub fn distinct_count(skill: &Skill) -> usize {
    if skill.operation.is_calculation() {
        CalculationSpace::of(skill).distinct_count()
    } else {
        family::distinct_count(skill)
    }
}

/// Generates a series of calculations from a skill (SUB000107).
///
/// Variety rules (SUB000112):
/// - never twice the same calculation (commuted additions and multiplications count as the same);
/// - a mixed skill gets as many additions as subtractions (± 1);
/// - never twice in a row the same shape (operation and format) when the skill offers several, so a
///   skill with both formats alternates « résultat » and « nombre manquant »;
/// - a number does not come back more than [`MAX_OPERAND_REPEATS`] times as an operand, when the
///   rule leaves enough other numbers (the tables worked on, `focus`, are not counted).
///
/// Multiplications (SUB000119, SUB000120): [`FOCUS_SHARE`] of the series on the tables worked on,
/// the factors shown in any order, a missing factor sometimes written `30 = 3 × ?`, and with
/// [`Progression::EasyFirst`] the easiest products first.
///
/// Fails with [`Error::InvalidSkill`] when the skill allows fewer calculations than `length`.
pub fn generate_calculations(
    skill: &Skill,
    seed: u64,
    answer_mode: AnswerMode,
    length: usize,
) -> Result<Vec<CalculationExercise>> {
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
) -> (Vec<CalculationExercise>, bool) {
    let operations = plan_operations(space.calculations(), &skill.formats, length, random);
    let focus = plan_focus(skill, space.calculations(), length, random);
    let mut picker = CalculationPicker::new(skill, space.calculations());

    // Calculations first when they are reordered from the easiest; otherwise picked one by one with
    // their format (the historical order of the draws, kept so that a seed gives the same series).
    let easy_first = skill.params.progression == Progression::EasyFirst;
    let mut calculations: Vec<Calculation> = Vec::new();
    if easy_first {
        for (index, &operation) in operations.iter().enumerate() {
            calculations.push(picker.pick(operation, focus.get(index).copied().flatten(), random));
        }
        calculations.sort_by_key(difficulty);
    }

    let mut previous: Option<Shape> = None;
    let mut exercises = Vec::with_capacity(length);
    for (index, &operation) in operations.iter().enumerate() {
        let calculation = if easy_first {
            calculations[index]
        } else {
            picker.pick(operation, focus.get(index).copied().flatten(), random)
        };
        let format = pick_format(&skill.formats, calculation.operation, previous, random);
        let hidden = hidden_part(format, random);
        let mut exercise = CalculationExercise::new(index, calculation, hidden);
        exercise.format = format;
        if calculation.operation == ArithmeticOperation::Multiplication {
            shape_multiplication(skill, &mut exercise, random);
        }

        if answer_mode == AnswerMode::Choice {
            exercise.choices = Some(build_choices(&exercise, random));
        }

        previous = Some((calculation.operation, format));
        exercises.push(exercise);
    }

    (exercises, picker.relaxed)
}

/// Factors in any order (unless the picture draws the rows), missing factor sometimes written the
/// other way round.
fn shape_multiplication(skill: &Skill, exercise: &mut CalculationExercise, random: &mut SeededRng) {
    let keeps_order = skill.params.picture.is_some() || exercise.format == ExerciseFormat::Phrase;
    if !keeps_order && random.random_bool(0.5) {
        exercise.calculation = exercise.calculation.commuted();
        exercise.hidden = match exercise.hidden {
            HiddenPart::Left => HiddenPart::Right,
            HiddenPart::Right => HiddenPart::Left,
            HiddenPart::Result => HiddenPart::Result,
        };
    }

    if exercise.format == ExerciseFormat::MissingOperand {
        if exercise.calculation.result() == 0 {
            // `0 × ? = 0` has every number as answer: the product is asked instead.
            exercise.format = ExerciseFormat::Result;
            exercise.hidden = HiddenPart::Result;
        } else {
            exercise.reversed = random.random_bool(REVERSED_SHARE);
        }
    }
}

/// Easiest products first (SUB000119, SUB000120): × 0, × 1, × 2, × 5, × 10, then a small factor
/// (up to 5), then the others. Other operations keep their order.
fn difficulty(calculation: &Calculation) -> u8 {
    if calculation.operation != ArithmeticOperation::Multiplication {
        return 0;
    }

    let easy = |n: i32| matches!(n, 0 | 1 | 2 | 5 | 10);
    let (a, b) = (calculation.left, calculation.right);
    if easy(a) || easy(b) {
        0
    } else if a.min(b) <= 5 {
        1
    } else {
        2
    }
}

/// Which exercises are on the tables worked on (`Some(true)`), and which revise the others
/// (`Some(false)`); empty when the skill has no focus or no revision is possible.
fn plan_focus(
    skill: &Skill,
    calculations: &[Calculation],
    length: usize,
    random: &mut SeededRng,
) -> Vec<Option<bool>> {
    let focus = &skill.params.focus;
    if focus.is_empty() {
        return Vec::new();
    }

    let on_focus = |c: &Calculation| focus.contains(&c.left) || focus.contains(&c.right);
    if !calculations.iter().any(|c| !on_focus(c)) {
        return Vec::new();
    }

    let focused = ((length as f64) * FOCUS_SHARE).round() as usize;
    let mut plan: Vec<Option<bool>> = (0..length).map(|i| Some(i < focused)).collect();
    plan.shuffle(random);
    plan
}

/// Four proposals: the right answer and three credible mistakes, never negative, never twice the
/// same, in a random order. Additions and subtractions: ±1, carry error ±10, inverted operation or
/// copied number. Multiplications: the sum instead of the product, the neighbouring table, ±1; for
/// 13 × 7, a forgotten part (70), the units only (21), a wrong sum (±10).
pub fn build_choices(exercise: &CalculationExercise, random: &mut impl Rng) -> Vec<i32> {
    let expected = exercise.expected();
    let groups: [Vec<i32>; 3] =
        if exercise.calculation.operation == ArithmeticOperation::Multiplication {
            multiplication_mistakes(exercise, random)
        } else {
            [
                shuffled(vec![expected + 1, expected - 1], random),
                shuffled(vec![expected + 10, expected - 10], random),
                vec![inverted_operation(exercise)],
            ]
        };

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

fn multiplication_mistakes(exercise: &CalculationExercise, random: &mut impl Rng) -> [Vec<i32>; 3] {
    let c = &exercise.calculation;
    let (a, b, product) = (c.left, c.right, c.result());
    let expected = exercise.expected();
    match exercise.hidden {
        HiddenPart::Result => {
            let teens = |n: i32| (11..=19).contains(&n);
            if teens(a) || teens(b) {
                let (big, small) = if teens(a) { (a, b) } else { (b, a) };
                [
                    vec![10 * small],
                    vec![big % 10 * small, 10 * small + big % 10],
                    shuffled(vec![product + 10, product - 10], random),
                ]
            } else {
                [
                    vec![a + b],
                    shuffled(
                        vec![product + a, product - a, product + b, product - b],
                        random,
                    ),
                    shuffled(vec![product + 1, product - 1], random),
                ]
            }
        }
        // A missing factor: the product copied, a neighbouring factor.
        _ => [
            vec![product],
            shuffled(vec![expected + 1, expected - 1], random),
            vec![if exercise.hidden == HiddenPart::Left {
                b
            } else {
                a
            }],
        ],
    }
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
    let (add, sub, mul) = (
        ArithmeticOperation::Addition,
        ArithmeticOperation::Subtraction,
        ArithmeticOperation::Multiplication,
    );
    if has(mul) {
        return vec![mul; length];
    }

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
    focus: &'a [i32],
    used: HashSet<DuplicateKey>,
    operand_uses: HashMap<i32, usize>,
    count_left: bool,
    count_right: bool,
    /// The operand repetition rule could not be respected for at least one calculation.
    relaxed: bool,
}

impl<'a> CalculationPicker<'a> {
    fn new(skill: &'a Skill, calculations: &'a [Calculation]) -> Self {
        Self {
            calculations,
            focus: &skill.params.focus,
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
        if self.count_left && !self.focus.contains(&calculation.left) {
            operands.push(calculation.left);
        }

        if self.count_right
            && !operands.contains(&calculation.right)
            && !self.focus.contains(&calculation.right)
        {
            operands.push(calculation.right);
        }

        operands
    }

    /// `focus`: on the tables worked on (`Some(true)`), on the others (`Some(false)`), or any.
    fn pick(
        &mut self,
        operation: ArithmeticOperation,
        focus: Option<bool>,
        random: &mut impl Rng,
    ) -> Calculation {
        let on_focus = |c: &Calculation| {
            focus.is_none_or(|wanted| {
                (self.focus.contains(&c.left) || self.focus.contains(&c.right)) == wanted
            })
        };
        let fresh = |c: &Calculation| !self.used.contains(&c.duplicate_key());
        let planned = |c: &Calculation| c.operation == operation && fresh(c);
        let varied = |c: &Calculation| {
            planned(c)
                && on_focus(c)
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
                let focused = match focus {
                    Some(_) => {
                        find_random(self.calculations, &|c| planned(c) && on_focus(c), random)
                    }
                    None => None,
                };
                focused
                    .or_else(|| find_random(self.calculations, &planned, random))
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
/// With the repeated addition (`Phrase`), each format is drawn equally, never twice in a row.
fn pick_format(
    formats: &[ExerciseFormat],
    operation: ArithmeticOperation,
    previous: Option<Shape>,
    random: &mut impl Rng,
) -> ExerciseFormat {
    let calculation_formats: Vec<ExerciseFormat> = formats
        .iter()
        .copied()
        .filter(|f| {
            matches!(
                f,
                ExerciseFormat::Result | ExerciseFormat::MissingOperand | ExerciseFormat::Phrase
            )
        })
        .collect();
    if calculation_formats.contains(&ExerciseFormat::Phrase) {
        if calculation_formats.len() == 1 {
            return ExerciseFormat::Phrase;
        }
        let others: Vec<ExerciseFormat> = calculation_formats
            .iter()
            .copied()
            .filter(|f| previous.is_none_or(|(op, format)| op != operation || format != *f))
            .collect();
        return others[random.random_range(0..others.len())];
    }

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
            _ => ExerciseFormat::Result,
        },
        _ => drawn,
    }
}

fn hidden_part(format: ExerciseFormat, random: &mut impl Rng) -> HiddenPart {
    match format {
        ExerciseFormat::Result => HiddenPart::Result,
        _ if random.random_range(0..2) == 0 => HiddenPart::Left,
        _ => HiddenPart::Right,
    }
}

/// A classic mistake: the other operation when the result is asked (27 + 15 → 12); when a number is
/// missing, copying the result (68 + ? = 70 → 70) or, for `? − b = r`, subtracting instead of adding.
fn inverted_operation(exercise: &CalculationExercise) -> i32 {
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
