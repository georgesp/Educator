//! Word problems (SUB000124), generated from templates (`content/problems/*.json`): first names,
//! objects and numbers drawn at random, every number checked by the template's rules. After a
//! mistake, the correction comes with a bar model (or the groups of a multiplication).

use super::expression::{Expression, Operator};
use super::family::{number_choices, Draw, Position};
use super::{Answer, BarSegment, Exercise, Input, Picture, Token};
use crate::random::SeededRng;
use crate::skills::{AnswerMode, ExerciseFormat, Skill};
use crate::Grade;
use rand::Rng;
use std::collections::HashMap;

/// Kinds of problems of the programme (cycle 2, 2024).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProblemKind {
    /// Parts and whole, one step.
    PartWhole,
    /// A change (gain, loss), one step.
    Transformation,
    /// Additive comparison (« de plus », « de moins »), one step.
    Comparison,
    /// Two additive steps.
    TwoSteps,
    /// Value of the whole: 4 boxes of 6.
    Product,
    /// Equal sharing: number of parts or value of a part.
    Sharing,
    /// Two steps, one additive and one multiplicative.
    Mixed,
}

/// Range of a number of a template; `max: None` is the scale of the skill (100 or 1 000).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableRange {
    pub name: String,
    pub min: i64,
    pub max: Option<i64>,
}

/// Correction picture of a template.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProblemHelp {
    /// Bars of segments (expressions), and the total of the first bar.
    BarModel {
        bars: Vec<Vec<Expression>>,
        total: Option<Expression>,
    },
    Groups {
        groups: Expression,
        per_group: Expression,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProblemTemplate {
    pub id: String,
    pub kind: ProblemKind,
    /// Text with `{a}`, `{name}`, `{il}`, `{objects}`… (see `docs/specs/regles-de-generation.md`).
    pub text: String,
    pub variables: Vec<VariableRange>,
    /// Intermediate values, usable in the text, the checks and the answer.
    pub derived: Vec<(String, Expression)>,
    /// Every check must be true (non-zero) for a draw to be kept.
    pub checks: Vec<Expression>,
    pub answer: Expression,
    /// Written solution, e.g. `{a} + {b} = {answer}`.
    pub solution: String,
    /// Typical wrong answers (wrong operation, one step only).
    pub mistakes: Vec<Expression>,
    pub help: Option<ProblemHelp>,
    /// Grades whose skills may use the template (empty: every grade). The CP templates say
    /// « fois » instead of × (SUB012208).
    pub grades: Vec<Grade>,
}

/// A first name and whether it takes « elle ».
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FirstName {
    pub name: String,
    pub feminine: bool,
}

/// A noun: « bille » / « billes ».
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Noun {
    pub singular: String,
    pub plural: String,
}

/// Words drawn into the templates.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ProblemWords {
    pub names: Vec<FirstName>,
    pub objects: Vec<Noun>,
    pub containers: Vec<Noun>,
}

/// The problems of a skill: its templates (resolved from its kinds when loading) and the scale.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProblemSet {
    pub kinds: Vec<ProblemKind>,
    pub templates: Vec<ProblemTemplate>,
    pub words: ProblemWords,
    /// Largest number of the statements and answers.
    pub max_number: i64,
    /// Smaller scale for the first exercises of a series (100 before 1 000).
    pub start_max_number: Option<i64>,
    /// No carry in the additions and no borrowed ten in the subtractions of the answer (CP before
    /// period 3, SUB012208).
    pub no_carry: bool,
}

/// Draws of numbers tried for a template before giving up this draw.
const NUMBER_DRAWS: usize = 200;

pub fn draw(
    skill: &Skill,
    _format: ExerciseFormat,
    mode: AnswerMode,
    position: Position,
    random: &mut SeededRng,
) -> Option<Draw> {
    let set = skill.params.problems.as_ref()?;
    if set.templates.is_empty() || set.words.names.len() < 2 {
        return None;
    }

    let template = &set.templates[random.random_range(0..set.templates.len())];
    let scale = match set.start_max_number {
        Some(start) if position.is_early() => start,
        _ => set.max_number,
    };
    let values = draw_numbers_with(template, scale, set.no_carry, random)?;
    let exercise = build(template, set, &values, mode, random)?;
    Some(Draw {
        exercise,
        difficulty: 0,
    })
}

/// Draws the numbers of a template and computes the derived values and the answer
/// (`answer` and `max` are set too). `None` when no draw respects the checks.
pub fn draw_numbers(
    template: &ProblemTemplate,
    scale: i64,
    random: &mut impl Rng,
) -> Option<HashMap<String, i64>> {
    draw_numbers_with(template, scale, false, random)
}

/// Whether a calculation of `expression` carries a ten (37 + 25) or borrows one (43 − 18).
fn carries(expression: &Expression, values: &HashMap<String, i64>) -> bool {
    let Expression::Binary(operator, left, right) = expression else {
        return false;
    };
    if carries(left, values) || carries(right, values) {
        return true;
    }
    let (Some(l), Some(r)) = (left.evaluate(values), right.evaluate(values)) else {
        return false;
    };
    match operator {
        Operator::Add => l % 10 + r % 10 >= 10,
        Operator::Subtract => l % 10 < r % 10,
        _ => false,
    }
}

/// [`draw_numbers`], and with `no_carry` no carry nor borrowed ten in the derived values and the
/// answer (SUB012208).
pub fn draw_numbers_with(
    template: &ProblemTemplate,
    scale: i64,
    no_carry: bool,
    random: &mut impl Rng,
) -> Option<HashMap<String, i64>> {
    'draws: for _ in 0..NUMBER_DRAWS {
        let mut values: HashMap<String, i64> = HashMap::new();
        values.insert("max".to_owned(), scale);
        for variable in &template.variables {
            let max = variable.max.unwrap_or(scale).min(scale);
            if max < variable.min {
                return None;
            }
            values.insert(
                variable.name.clone(),
                random.random_range(variable.min..=max),
            );
        }

        for (name, expression) in &template.derived {
            let Some(value) = expression.evaluate(&values) else {
                continue 'draws;
            };
            values.insert(name.clone(), value);
        }

        let Some(answer) = template.answer.evaluate(&values) else {
            continue;
        };
        values.insert("answer".to_owned(), answer);
        let numbers_ok = values
            .iter()
            .filter(|(name, _)| name.as_str() != "max")
            .all(|(_, v)| (0..=scale).contains(v));
        let carry_ok = !no_carry
            || (!carries(&template.answer, &values)
                && template.derived.iter().all(|(_, e)| !carries(e, &values)));
        if answer > 0
            && numbers_ok
            && carry_ok
            && template
                .checks
                .iter()
                .all(|check| check.evaluate(&values).is_some_and(|v| v != 0))
        {
            return Some(values);
        }
    }

    None
}

/// Replaces `{a}`, `{answer}`, `{name}`, `{name2}`, `{il}`, `{Il}`, `{il2}`, `{Il2}`, `{ils}`,
/// `{object}`, `{objects}`, `{container}`, `{containers}` in a text.
pub fn fill(text: &str, values: &HashMap<String, i64>, words: &HashMap<&str, String>) -> String {
    let mut result = String::with_capacity(text.len() + 32);
    let mut rest = text;
    while let Some(start) = rest.find('{') {
        result.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        match after.find('}') {
            Some(end) => {
                let key = &after[..end];
                if let Some(value) = values.get(key) {
                    result.push_str(&value.to_string());
                } else if let Some(word) = words.get(key) {
                    result.push_str(word);
                } else {
                    result.push('{');
                    result.push_str(key);
                    result.push('}');
                }
                rest = &after[end + 1..];
            }
            None => {
                result.push_str(&rest[start..]);
                rest = "";
            }
        }
    }
    result.push_str(rest);
    elide(&result)
}

/// French elision after filling: « de images » → « d'images », « que Inès » → « qu'Inès ».
pub fn elide(text: &str) -> String {
    let starts_with_vowel = |word: &str| {
        word.chars()
            .next()
            .is_some_and(|c| "aeiouyéèêàâîôûAEIOUYÉÈÊÀÂÎÔÛ".contains(c))
    };
    let words: Vec<&str> = text.split(' ').collect();
    let mut result = String::with_capacity(text.len());
    let mut i = 0;
    while i < words.len() {
        let word = words[i];
        let next = words.get(i + 1).copied().unwrap_or_default();
        let elided = match word {
            "de" | "De" | "que" | "Que" if starts_with_vowel(next) => {
                Some(format!("{}'{next}", &word[..word.len() - 1]))
            }
            _ => None,
        };
        if !result.is_empty() {
            result.push(' ');
        }
        match elided {
            Some(joined) => {
                result.push_str(&joined);
                i += 2;
            }
            None => {
                result.push_str(word);
                i += 1;
            }
        }
    }
    result
}

/// The placeholders a text may use besides the numbers.
pub const WORD_PLACEHOLDERS: &[&str] = &[
    "name",
    "name2",
    "il",
    "Il",
    "il2",
    "Il2",
    "ils",
    "object",
    "objects",
    "container",
    "containers",
];

fn pick<'a, T>(items: &'a [T], random: &mut impl Rng) -> &'a T {
    &items[random.random_range(0..items.len())]
}

fn build(
    template: &ProblemTemplate,
    set: &ProblemSet,
    values: &HashMap<String, i64>,
    mode: AnswerMode,
    random: &mut SeededRng,
) -> Option<Exercise> {
    let words_set = &set.words;
    let first = pick(&words_set.names, random).clone();
    let mut second = pick(&words_set.names, random).clone();
    for _ in 0..10 {
        if second.name != first.name {
            break;
        }
        second = pick(&words_set.names, random).clone();
    }
    if second.name == first.name {
        return None;
    }

    let object = pick(&words_set.objects, random).clone();
    let container = if words_set.containers.is_empty() {
        Noun {
            singular: "boîte".to_owned(),
            plural: "boîtes".to_owned(),
        }
    } else {
        pick(&words_set.containers, random).clone()
    };
    let pronoun = |feminine: bool| if feminine { "elle" } else { "il" };
    let capital = |feminine: bool| if feminine { "Elle" } else { "Il" };
    let words: HashMap<&str, String> = HashMap::from([
        ("name", first.name.clone()),
        ("name2", second.name.clone()),
        ("il", pronoun(first.feminine).to_owned()),
        ("Il", capital(first.feminine).to_owned()),
        ("il2", pronoun(second.feminine).to_owned()),
        ("Il2", capital(second.feminine).to_owned()),
        (
            "ils",
            (if first.feminine && second.feminine {
                "elles"
            } else {
                "ils"
            })
            .to_owned(),
        ),
        ("object", object.singular.clone()),
        ("objects", object.plural.clone()),
        ("container", container.singular.clone()),
        ("containers", container.plural.clone()),
    ]);

    let text = fill(&template.text, values, &words);
    let answer = *values.get("answer")?;
    let answer = i32::try_from(answer).ok()?;
    let mut mistakes: Vec<i32> = template
        .mistakes
        .iter()
        .filter_map(|m| m.evaluate(values))
        .filter_map(|v| i32::try_from(v).ok())
        .collect();
    // A number of the statement copied as the answer.
    for variable in &template.variables {
        if let Some(v) = values
            .get(&variable.name)
            .and_then(|v| i32::try_from(*v).ok())
        {
            mistakes.push(v);
        }
    }
    mistakes.push(answer + 10);
    mistakes.push(answer - 10);
    // Within the numbers of the skill: no 190 in a problem of the CP up to 100.
    mistakes.retain(|m| i64::from(*m) <= set.max_number);

    let (input, choices) = match mode {
        AnswerMode::Keyboard => (Input::Number { slots: 1 }, None),
        AnswerMode::Choice => (Input::Pick, Some(number_choices(answer, &mistakes, random))),
    };

    let number = |e: &Expression| e.evaluate(values).and_then(|v| i32::try_from(v).ok());
    let help = template.help.as_ref().and_then(|help| match help {
        ProblemHelp::BarModel { bars, total } => Some(Picture::BarModel {
            bars: bars
                .iter()
                .map(|bar| {
                    bar.iter()
                        .map(|segment| BarSegment {
                            value: number(segment),
                        })
                        .collect()
                })
                .collect(),
            total: total.as_ref().and_then(number),
        }),
        ProblemHelp::Groups { groups, per_group } => {
            let (g, p) = (number(groups)?, number(per_group)?);
            (g <= 12 && p <= 12).then(|| Picture::Groups {
                groups: g.unsigned_abs(),
                per_group: p.unsigned_abs(),
            })
        }
    });

    let mut key_values: Vec<(String, i64)> = template
        .variables
        .iter()
        .filter_map(|v| values.get(&v.name).map(|x| (v.name.clone(), *x)))
        .collect();
    key_values.sort();
    Some(Exercise {
        index: 0,
        format: ExerciseFormat::Problem,
        question: Some(text.clone()),
        statement: vec![Token::Slot(0)],
        picture: None,
        input,
        expected: Answer::Number(answer),
        choices,
        spoken: text,
        explanation: fill(&template.solution, values, &words),
        help,
        key: format!("{}:{key_values:?}", template.id),
        calculation: None,
        word: None,
    })
}
