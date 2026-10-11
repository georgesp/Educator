//! Loads and validates the skills (`content/skills/<subject>/*.json`, SUB000106). The rules mirror
//! `content/schemas/skill.schema.json` and add what a schema cannot say (bounds in order, enough
//! different exercises). An invalid file is skipped and reported.

use crate::{ContentProblem, Loaded};
use educator_core::exercises::CalculationSpace;
use educator_core::skills::{
    AnswerMode, CarryRule, ExerciseFormat, OperandRule, Placement, Skill, SkillCatalog,
    SkillConstraints, SkillOperation, Subject, TrivialCase, FIRST_PERIOD, LAST_PERIOD,
};
use educator_core::Grade;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// A skill must allow at least this many different calculations (rule too narrow otherwise).
pub const MIN_DISTINCT_EXERCISES: usize = 20;

pub const MAX_OPERAND: i32 = 100_000;

/// Pairs of operands a rule may combine. The engine lists every valid calculation once: a wider rule
/// would make the start-up slow and the memory heavy (three-digit additions are about 640 000 pairs).
pub const MAX_COMBINATIONS: i64 = 2_000_000;

/// [`SkillCatalog`] over the skills loaded at start-up.
#[derive(Debug, Clone, Default)]
pub struct Skills {
    skills: Vec<Skill>,
    by_id: HashMap<String, usize>,
}

impl Skills {
    pub fn new(skills: Vec<Skill>) -> Self {
        let by_id = skills
            .iter()
            .enumerate()
            .map(|(i, s)| (s.id.clone(), i))
            .collect();
        Self { skills, by_id }
    }
}

impl SkillCatalog for Skills {
    fn all(&self) -> &[Skill] {
        &self.skills
    }

    fn find(&self, id: &str) -> Option<&Skill> {
        self.by_id.get(id).map(|&i| &self.skills[i])
    }
}

/// Every `*.json` below `skills_directory`, in path order.
pub fn load_skills(skills_directory: &Path) -> Loaded<Vec<Skill>> {
    let mut skills: Vec<Skill> = Vec::new();
    let mut problems = Vec::new();
    if !skills_directory.is_dir() {
        problems.push(ContentProblem::new(
            skills_directory.display().to_string(),
            "Skills folder not found.",
        ));
        return Loaded {
            value: skills,
            problems,
        };
    }

    let mut files = Vec::new();
    collect_json_files(skills_directory, &mut files);
    files.sort();
    for file in files {
        let name = file
            .strip_prefix(skills_directory)
            .unwrap_or(&file)
            .to_string_lossy()
            .replace('\\', "/");
        let stem = file.file_stem().map(|s| s.to_string_lossy().into_owned());
        let parsed = std::fs::read_to_string(&file)
            .map_err(|e| vec![e.to_string()])
            .and_then(|json| parse_skill(&json, stem.as_deref()));
        match parsed {
            Err(errors) => problems.extend(
                errors
                    .into_iter()
                    .map(|e| ContentProblem::new(name.clone(), e)),
            ),
            Ok(skill) if skills.iter().any(|s| s.id == skill.id) => {
                problems.push(ContentProblem::new(
                    name,
                    format!("Duplicate skill id {}.", skill.id),
                ));
            }
            Ok(skill) => skills.push(skill),
        }
    }

    Loaded {
        value: skills,
        problems,
    }
}

fn collect_json_files(directory: &Path, files: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_json_files(&path, files);
        } else if path.extension().is_some_and(|e| e == "json") {
            files.push(path);
        }
    }
}

// File shapes (camelCase JSON). Unknown properties are rejected.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SkillFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    id: String,
    subject: String,
    grade: String,
    period: i32,
    title: String,
    instruction: String,
    operation: String,
    operands: Vec<OperandFile>,
    constraints: ConstraintsFile,
    formats: Vec<String>,
    answer_modes: Vec<String>,
    source: String,
    also_offered: Option<Vec<PlacementFile>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PlacementFile {
    grade: String,
    period: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct OperandFile {
    min: i32,
    max: i32,
    multiple_of: Option<i32>,
    values: Option<Vec<i32>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ConstraintsFile {
    min_result: Option<i32>,
    max_result: Option<i32>,
    exact_result: Option<i32>,
    result_multiple_of: Option<i32>,
    carry: Option<String>,
    exclude_trivial: Option<Vec<String>>,
}

/// Parses and validates one skill; `expected_id` is the file name. Returns every error found.
pub fn parse_skill(json: &str, expected_id: Option<&str>) -> Result<Skill, Vec<String>> {
    let file: SkillFile =
        serde_json::from_str(json).map_err(|e| vec![format!("Invalid JSON: {e}")])?;

    let mut errors = Vec::new();
    let mut check = |ok: bool, message: String| {
        if !ok {
            errors.push(message);
        }
    };

    check(
        is_id(&file.id),
        format!(
            "id '{}' must be lowercase words separated by hyphens.",
            file.id
        ),
    );
    check(
        expected_id.is_none_or(|expected| expected == file.id),
        format!(
            "id '{}' must match the file name '{}'.",
            file.id,
            expected_id.unwrap_or_default()
        ),
    );
    check(
        (i32::from(FIRST_PERIOD)..=i32::from(LAST_PERIOD)).contains(&file.period),
        format!("period {} must be between 1 and 5.", file.period),
    );
    check(
        is_text(&file.title, 60),
        "title must be 1 to 60 characters.".to_owned(),
    );
    check(
        is_text(&file.instruction, 120),
        "instruction must be 1 to 120 characters.".to_owned(),
    );
    check(
        !file.source.trim().is_empty(),
        "source is required (official programme reference).".to_owned(),
    );

    if file.operands.len() != 2 {
        check(
            false,
            "operands must hold exactly two rules (left, right).".to_owned(),
        );
    }

    for (i, operand) in file.operands.iter().enumerate() {
        let position = if i == 0 { "left" } else { "right" };
        check(
            operand.min >= 0 && operand.max <= MAX_OPERAND && operand.min <= operand.max,
            format!("{position} operand: need 0 <= min <= max <= {MAX_OPERAND}."),
        );
        check(
            operand.multiple_of.is_none_or(|m| m > 0),
            format!("{position} operand: multipleOf must be positive."),
        );
        check(
            operand
                .values
                .as_ref()
                .is_none_or(|v| !v.is_empty() && v.iter().collect::<HashSet<_>>().len() == v.len()),
            format!("{position} operand: values must be a non-empty list without duplicates."),
        );
        check(
            operand
                .values
                .as_ref()
                .is_none_or(|v| v.iter().all(|x| *x >= operand.min && *x <= operand.max)),
            format!("{position} operand: values must lie between min and max."),
        );
    }

    let constraints = &file.constraints;
    check(
        constraints.min_result.is_none()
            || constraints.max_result.is_none()
            || constraints.min_result <= constraints.max_result,
        "constraints: minResult must not exceed maxResult.".to_owned(),
    );
    check(
        constraints.result_multiple_of.is_none_or(|m| m > 0),
        "constraints: resultMultipleOf must be positive.".to_owned(),
    );
    check(
        [
            constraints.min_result,
            constraints.max_result,
            constraints.exact_result,
        ]
        .iter()
        .all(|v| v.is_none_or(|v| v >= 0)),
        "constraints: results are natural numbers.".to_owned(),
    );

    let subject = parse_enum(&file.subject, "subject", SUBJECTS, &mut errors);
    let grade = parse_enum(&file.grade, "grade", GRADES, &mut errors);
    let operation = parse_enum(&file.operation, "operation", OPERATIONS, &mut errors);
    let formats = parse_list(&file.formats, "formats", FORMATS, false, &mut errors);
    let answer_modes = parse_list(
        &file.answer_modes,
        "answerModes",
        ANSWER_MODES,
        false,
        &mut errors,
    );
    let carry = match &constraints.carry {
        None => Some(CarryRule::Any),
        Some(carry) => parse_enum(carry, "constraints.carry", CARRY_RULES, &mut errors),
    };
    let excluded = parse_list(
        constraints.exclude_trivial.as_deref().unwrap_or_default(),
        "constraints.excludeTrivial",
        TRIVIAL_CASES,
        true,
        &mut errors,
    );

    let mut also_offered = Vec::new();
    for placement in file.also_offered.as_deref().unwrap_or_default() {
        let placement_grade =
            parse_enum(&placement.grade, "alsoOffered.grade", GRADES, &mut errors);
        if !(i32::from(FIRST_PERIOD)..=i32::from(LAST_PERIOD)).contains(&placement.period) {
            errors.push(format!(
                "alsoOffered: period {} must be between 1 and 5.",
                placement.period
            ));
        } else if let Some(placement_grade) = placement_grade {
            let period = u8::try_from(placement.period).expect("checked");
            if Some(placement_grade) == grade {
                errors.push(format!(
                    "alsoOffered: {placement_grade} is the skill's own grade; only other grades can be added."
                ));
            } else if also_offered
                .iter()
                .any(|p: &Placement| p.grade == placement_grade)
            {
                errors.push(format!("alsoOffered: {placement_grade} is listed twice."));
            } else {
                also_offered.push(Placement::new(placement_grade, period));
            }
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    let operand = |o: &OperandFile| OperandRule {
        min: o.min,
        max: o.max,
        multiple_of: o.multiple_of,
        values: o.values.clone(),
    };
    let skill = Skill {
        id: file.id.clone(),
        subject: subject.expect("checked"),
        grade: grade.expect("checked"),
        period: u8::try_from(file.period).expect("checked"),
        title: file.title.trim().to_owned(),
        instruction: file.instruction.trim().to_owned(),
        operation: operation.expect("checked"),
        left: operand(&file.operands[0]),
        right: operand(&file.operands[1]),
        constraints: SkillConstraints {
            min_result: constraints.min_result,
            max_result: constraints.max_result,
            exact_result: constraints.exact_result,
            result_multiple_of: constraints.result_multiple_of,
            carry: carry.expect("checked"),
            exclude_trivial: excluded,
        },
        formats,
        answer_modes,
        source: file.source.trim().to_owned(),
        also_offered,
    };

    let combinations = skill.combination_count();
    if combinations > MAX_COMBINATIONS {
        return Err(vec![format!(
            "Rule too wide: {combinations} pairs of operands, at most {MAX_COMBINATIONS}."
        )]);
    }

    let distinct = CalculationSpace::of(&skill).distinct_count();
    if distinct < MIN_DISTINCT_EXERCISES {
        return Err(vec![format!(
            "Rule too narrow: {distinct} different exercises, at least {MIN_DISTINCT_EXERCISES} needed."
        )]);
    }

    Ok(skill)
}

const SUBJECTS: &[(&str, Subject)] = &[("math", Subject::Math)];
const GRADES: &[(&str, Grade)] = &[
    ("CP", Grade::CP),
    ("CE1", Grade::CE1),
    ("CE2", Grade::CE2),
    ("CM1", Grade::CM1),
    ("CM2", Grade::CM2),
];
const OPERATIONS: &[(&str, SkillOperation)] = &[
    ("addition", SkillOperation::Addition),
    ("subtraction", SkillOperation::Subtraction),
    ("mixed", SkillOperation::Mixed),
];
const FORMATS: &[(&str, ExerciseFormat)] = &[
    ("result", ExerciseFormat::Result),
    ("missingOperand", ExerciseFormat::MissingOperand),
];
const ANSWER_MODES: &[(&str, AnswerMode)] = &[
    ("keyboard", AnswerMode::Keyboard),
    ("choice", AnswerMode::Choice),
];
const CARRY_RULES: &[(&str, CarryRule)] = &[
    ("any", CarryRule::Any),
    ("forbidden", CarryRule::Forbidden),
    ("required", CarryRule::Required),
];
const TRIVIAL_CASES: &[(&str, TrivialCase)] = &[
    ("plusZero", TrivialCase::PlusZero),
    ("minusZero", TrivialCase::MinusZero),
    ("minusSelf", TrivialCase::MinusSelf),
];

fn parse_enum<T: Copy>(
    value: &str,
    field: &str,
    allowed: &[(&str, T)],
    errors: &mut Vec<String>,
) -> Option<T> {
    let found = allowed
        .iter()
        .find(|(name, _)| *name == value)
        .map(|(_, v)| *v);
    if found.is_none() {
        let names = allowed
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>()
            .join(", ");
        errors.push(format!("{field} '{value}' is unknown (allowed: {names})."));
    }

    found
}

fn parse_list<T: Copy>(
    values: &[String],
    field: &str,
    allowed: &[(&str, T)],
    allow_empty: bool,
    errors: &mut Vec<String>,
) -> Vec<T> {
    if !allow_empty && values.is_empty() {
        errors.push(format!("{field} must not be empty."));
    }

    if values.iter().collect::<HashSet<_>>().len() != values.len() {
        errors.push(format!("{field} must not repeat a value."));
    }

    values
        .iter()
        .filter_map(|v| parse_enum(v, field, allowed, errors))
        .collect()
}

/// `^[a-z0-9]+(-[a-z0-9]+)*$`
fn is_id(id: &str) -> bool {
    !id.is_empty()
        && id.split('-').all(|part| {
            !part.is_empty()
                && part
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        })
}

/// Not blank and at most `max` characters (UTF-16 length, like the JSON schema validators).
fn is_text(value: &str, max: usize) -> bool {
    !value.trim().is_empty() && value.encode_utf16().count() <= max
}
