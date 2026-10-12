//! Loads and validates the word problem templates and words (`content/problems/`, SUB000124).
//! An invalid template is skipped and reported, like an invalid skill.

use crate::{ContentProblem, Loaded};
use educator_core::exercises::expression::Expression;
use educator_core::exercises::{
    draw_numbers, FirstName, Noun, ProblemHelp, ProblemKind, ProblemTemplate, ProblemWords,
    VariableRange, WORD_PLACEHOLDERS,
};
use educator_core::random::SeededRng;
use rand::SeedableRng;
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

/// File of the words drawn into the problems, in the problems folder.
pub const WORDS_FILE: &str = "words.json";

/// Templates and words read from `content/problems/`.
#[derive(Debug, Clone, Default)]
pub struct ProblemBank {
    pub templates: Vec<ProblemTemplate>,
    pub words: ProblemWords,
}

impl ProblemBank {
    pub fn of_kind(&self, kind: ProblemKind) -> impl Iterator<Item = &ProblemTemplate> {
        self.templates.iter().filter(move |t| t.kind == kind)
    }
}

pub const KINDS: &[(&str, ProblemKind)] = &[
    ("partWhole", ProblemKind::PartWhole),
    ("transformation", ProblemKind::Transformation),
    ("comparison", ProblemKind::Comparison),
    ("twoSteps", ProblemKind::TwoSteps),
    ("product", ProblemKind::Product),
    ("sharing", ProblemKind::Sharing),
    ("mixed", ProblemKind::Mixed),
];

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TemplatesFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    templates: Vec<TemplateFile>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TemplateFile {
    id: String,
    kind: String,
    text: String,
    variables: BTreeMap<String, RangeFile>,
    #[serde(default)]
    derived: BTreeMap<String, String>,
    #[serde(default)]
    checks: Vec<String>,
    answer: String,
    solution: String,
    #[serde(default)]
    mistakes: Vec<String>,
    help: Option<HelpFile>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RangeFile {
    min: i64,
    max: Option<i64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct HelpFile {
    bars: Option<Vec<Vec<String>>>,
    total: Option<String>,
    groups: Option<String>,
    per_group: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct WordsFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    names: Vec<NameFile>,
    objects: Vec<NounFile>,
    containers: Vec<NounFile>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct NameFile {
    name: String,
    feminine: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct NounFile {
    singular: String,
    plural: String,
}

/// Every `*.json` of `directory` except [`WORDS_FILE`] holds templates.
pub fn load_problems(directory: &Path) -> Loaded<ProblemBank> {
    let mut bank = ProblemBank::default();
    let mut problems = Vec::new();
    if !directory.is_dir() {
        // No problems folder: the problem skills will report it.
        return Loaded {
            value: bank,
            problems,
        };
    }

    match std::fs::read_to_string(directory.join(WORDS_FILE))
        .map_err(|e| e.to_string())
        .and_then(|json| parse_words(&json))
    {
        Ok(words) => bank.words = words,
        Err(error) => problems.push(ContentProblem::new(format!("problems/{WORDS_FILE}"), error)),
    }

    let mut files: Vec<_> = std::fs::read_dir(directory)
        .map(|entries| {
            entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| {
                    p.extension().is_some_and(|e| e == "json")
                        && p.file_name().is_some_and(|n| n != WORDS_FILE)
                })
                .collect()
        })
        .unwrap_or_default();
    files.sort();
    let mut ids = HashSet::new();
    for file in files {
        let name = format!(
            "problems/{}",
            file.file_name().unwrap_or_default().to_string_lossy()
        );
        let parsed: Result<TemplatesFile, String> = std::fs::read_to_string(&file)
            .map_err(|e| e.to_string())
            .and_then(|json| serde_json::from_str(&json).map_err(|e| format!("Invalid JSON: {e}")));
        let file_content = match parsed {
            Ok(content) => content,
            Err(error) => {
                problems.push(ContentProblem::new(name, error));
                continue;
            }
        };

        for template in file_content.templates {
            let id = template.id.clone();
            match parse_template(template) {
                Ok(_) if !ids.insert(id.clone()) => {
                    problems.push(ContentProblem::new(
                        name.clone(),
                        format!("Duplicate template id {id}."),
                    ));
                }
                Ok(template) => bank.templates.push(template),
                Err(errors) => problems.extend(
                    errors
                        .into_iter()
                        .map(|e| ContentProblem::new(name.clone(), format!("{id}: {e}"))),
                ),
            }
        }
    }

    Loaded {
        value: bank,
        problems,
    }
}

fn parse_words(json: &str) -> Result<ProblemWords, String> {
    let file: WordsFile = serde_json::from_str(json).map_err(|e| format!("Invalid JSON: {e}"))?;
    let blank = |s: &str| s.trim().is_empty();
    if file.names.len() < 2
        || file.objects.is_empty()
        || file.containers.is_empty()
        || file.names.iter().any(|n| blank(&n.name))
        || file
            .objects
            .iter()
            .chain(&file.containers)
            .any(|n| blank(&n.singular) || blank(&n.plural))
    {
        return Err(
            "At least two names, one object and one container, none of them blank.".to_owned(),
        );
    }

    let names: HashSet<&str> = file.names.iter().map(|n| n.name.as_str()).collect();
    if names.len() != file.names.len() {
        return Err("A first name is listed twice.".to_owned());
    }

    let noun = |n: NounFile| Noun {
        singular: n.singular.trim().to_owned(),
        plural: n.plural.trim().to_owned(),
    };
    Ok(ProblemWords {
        names: file
            .names
            .into_iter()
            .map(|n| FirstName {
                name: n.name.trim().to_owned(),
                feminine: n.feminine,
            })
            .collect(),
        objects: file.objects.into_iter().map(noun).collect(),
        containers: file.containers.into_iter().map(noun).collect(),
    })
}

/// Placeholders `{...}` of a text.
fn placeholders(text: &str) -> Result<Vec<String>, String> {
    let mut found = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find('{') {
        let after = &rest[start + 1..];
        let end = after
            .find('}')
            .ok_or_else(|| format!("Unclosed '{{' in '{text}'."))?;
        found.push(after[..end].to_owned());
        rest = &after[end + 1..];
    }
    Ok(found)
}

/// Parses and validates one template: expressions, names used, and a few draws at both scales.
fn parse_template(file: TemplateFile) -> Result<ProblemTemplate, Vec<String>> {
    let mut errors = Vec::new();
    let expression = |text: &str, errors: &mut Vec<String>| match Expression::parse(text) {
        Ok(e) => Some(e),
        Err(e) => {
            errors.push(e);
            None
        }
    };

    if !is_id(&file.id) {
        errors.push("id must be lowercase words separated by hyphens.".to_owned());
    }

    let kind = KINDS
        .iter()
        .find(|(name, _)| *name == file.kind)
        .map(|(_, k)| *k);
    if kind.is_none() {
        errors.push(format!("kind '{}' is unknown.", file.kind));
    }

    if file.variables.is_empty() {
        errors.push("At least one variable.".to_owned());
    }

    let mut known: Vec<String> = vec!["max".to_owned()];
    let mut variables = Vec::new();
    for (name, range) in &file.variables {
        if !is_variable(name) || ["max", "answer"].contains(&name.as_str()) {
            errors.push(format!("Variable name '{name}' is not allowed."));
        }
        if range.min < 0 || range.max.is_some_and(|max| max < range.min) {
            errors.push(format!("Variable {name}: need 0 <= min <= max."));
        }
        known.push(name.clone());
        variables.push(VariableRange {
            name: name.clone(),
            min: range.min,
            max: range.max,
        });
    }

    let check_names = |e: &Expression, known: &[String], errors: &mut Vec<String>| {
        for variable in e.variables() {
            if !known.contains(&variable) {
                errors.push(format!("Unknown variable '{variable}'."));
            }
        }
    };

    let mut derived = Vec::new();
    for (name, text) in &file.derived {
        if let Some(e) = expression(text, &mut errors) {
            check_names(&e, &known, &mut errors);
            derived.push((name.clone(), e));
        }
        known.push(name.clone());
    }

    let answer = expression(&file.answer, &mut errors);
    if let Some(answer) = &answer {
        check_names(answer, &known, &mut errors);
    }
    known.push("answer".to_owned());

    let mut checks = Vec::new();
    for text in &file.checks {
        if let Some(e) = expression(text, &mut errors) {
            check_names(&e, &known, &mut errors);
            checks.push(e);
        }
    }

    let mut mistakes = Vec::new();
    for text in &file.mistakes {
        if let Some(e) = expression(text, &mut errors) {
            check_names(&e, &known, &mut errors);
            mistakes.push(e);
        }
    }

    for (field, text) in [("text", &file.text), ("solution", &file.solution)] {
        match placeholders(text) {
            Ok(found) => {
                for placeholder in found {
                    let number = known.contains(&placeholder) && placeholder != "max";
                    if !number && !WORD_PLACEHOLDERS.contains(&placeholder.as_str()) {
                        errors.push(format!("{field}: unknown placeholder {{{placeholder}}}."));
                    }
                    if field == "text" && placeholder == "answer" {
                        errors.push("text must not give the answer.".to_owned());
                    }
                }
            }
            Err(e) => errors.push(e),
        }
    }

    let help = match file.help {
        None => None,
        Some(HelpFile {
            bars: Some(bars),
            total,
            groups: None,
            per_group: None,
        }) => {
            let mut parsed_bars = Vec::new();
            for bar in &bars {
                let mut segments = Vec::new();
                for segment in bar {
                    if let Some(e) = expression(segment, &mut errors) {
                        check_names(&e, &known, &mut errors);
                        segments.push(e);
                    }
                }
                parsed_bars.push(segments);
            }
            let total = total.and_then(|t| expression(&t, &mut errors));
            if bars.is_empty() || bars.iter().any(Vec::is_empty) {
                errors.push("help.bars: bars of at least one segment.".to_owned());
            }
            Some(ProblemHelp::BarModel {
                bars: parsed_bars,
                total,
            })
        }
        Some(HelpFile {
            bars: None,
            total: None,
            groups: Some(groups),
            per_group: Some(per_group),
        }) => {
            let groups = expression(&groups, &mut errors);
            let per_group = expression(&per_group, &mut errors);
            match (groups, per_group) {
                (Some(groups), Some(per_group)) => Some(ProblemHelp::Groups { groups, per_group }),
                _ => None,
            }
        }
        Some(_) => {
            errors.push("help: either bars (and total) or groups and perGroup.".to_owned());
            None
        }
    };

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(ProblemTemplate {
        id: file.id,
        kind: kind.expect("checked"),
        text: file.text.trim().to_owned(),
        variables,
        derived,
        checks,
        answer: answer.expect("checked"),
        solution: file.solution.trim().to_owned(),
        mistakes,
        help,
    })
}

/// Whether a template gives numbers at this scale (a few hundred draws).
pub fn template_works_at(template: &ProblemTemplate, scale: i64) -> bool {
    let mut random = SeededRng::seed_from_u64(7);
    (0..20).any(|_| draw_numbers(template, scale, &mut random).is_some())
}

fn is_id(id: &str) -> bool {
    !id.is_empty()
        && id.split('-').all(|part| {
            !part.is_empty()
                && part
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        })
}

fn is_variable(name: &str) -> bool {
    !name.is_empty()
        && name.bytes().next().is_some_and(|b| b.is_ascii_lowercase())
        && name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
}
