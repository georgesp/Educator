//! Loads the mascot (SUB001417): `content/mascot/mascot.json` (name and phrases by situation) and
//! one SVG per expression, checked like the characters. Without a valid mascot, the application
//! simply shows none.

use crate::characters::validate_svg;
use crate::{ContentProblem, Loaded};
use educator_core::mascot::{Expression, Mascot, Situation};
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

/// Phrases of a situation: at least this many, so that they do not repeat too often.
pub const MIN_MASCOT_PHRASES: usize = 3;

/// Longest phrase (shown in a bubble and read aloud).
pub const MAX_MASCOT_PHRASE: usize = 160;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct MascotFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    #[serde(rename = "comment")]
    _comment: Option<String>,
    name: String,
    phrases: BTreeMap<String, Vec<String>>,
}

/// The mascot of `directory` (`content/mascot`); `None` with the problems when it is invalid.
pub fn load_mascot(directory: &Path) -> Loaded<Option<Mascot>> {
    let mut problems = Vec::new();
    let file = "mascot/mascot.json";
    let parsed = std::fs::read_to_string(directory.join("mascot.json"))
        .map_err(|e| e.to_string())
        .and_then(|json| {
            serde_json::from_str::<MascotFile>(&json).map_err(|e| format!("Invalid JSON: {e}"))
        });
    let parsed = match parsed {
        Ok(parsed) => parsed,
        Err(e) => {
            problems.push(ContentProblem::new(file, e));
            return Loaded {
                value: None,
                problems,
            };
        }
    };

    let mut problem = |message: String| problems.push(ContentProblem::new(file, message));
    if parsed.name.trim().is_empty() || parsed.name.chars().count() > 20 {
        problem("name: 1 to 20 characters.".to_owned());
    }

    let mut phrases = HashMap::new();
    for (name, list) in &parsed.phrases {
        let Some(situation) = Situation::ALL.into_iter().find(|s| s.name() == name) else {
            problem(format!("Unknown situation '{name}'."));
            continue;
        };
        let unique: HashSet<&String> = list.iter().collect();
        if list.len() < MIN_MASCOT_PHRASES || unique.len() != list.len() {
            problem(format!(
                "{name}: at least {MIN_MASCOT_PHRASES} different phrases."
            ));
        }
        for phrase in list {
            if phrase.trim().is_empty() || phrase.chars().count() > MAX_MASCOT_PHRASE {
                problem(format!(
                    "{name}: a phrase of 1 to {MAX_MASCOT_PHRASE} characters."
                ));
            }
            if phrase.contains("{title}") != (situation == Situation::LessonStart) {
                problem(format!(
                    "{name}: {{title}} is for every phrase of lessonStart, and only for them."
                ));
            }
        }
        phrases.insert(situation, list.clone());
    }
    for situation in Situation::ALL {
        if !phrases.contains_key(&situation) {
            problem(format!("No phrase for {}.", situation.name()));
        }
    }

    let mut pictures = HashMap::new();
    for expression in Expression::ALL {
        match validate_svg(&directory.join(expression.file_name())) {
            Ok(svg) => {
                pictures.insert(expression, svg);
            }
            Err(e) => problems.push(ContentProblem::new(
                format!("mascot/{}", expression.file_name()),
                e,
            )),
        }
    }

    let valid = problems.is_empty();
    Loaded {
        value: valid.then(|| Mascot {
            name: parsed.name.trim().to_owned(),
            pictures,
            phrases,
        }),
        problems,
    }
}
