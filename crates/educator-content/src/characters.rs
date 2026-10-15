//! Profile characters (SUB000101): `content/characters/characters.json` plus one SVG per character,
//! loaded once. A character whose entry or SVG is invalid is skipped and reported.

use crate::{ContentProblem, Loaded};
use educator_core::characters::{CharacterCatalog, CharacterInfo, CharacterKind};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

/// Named groups every character SVG must contain (accessories of lot 3 attach to them).
pub const REQUIRED_GROUPS: [&str; 4] = ["body", "head", "face", "clothes"];

/// At least one of these groups: hair for the children, ears or mane for the animals.
pub const TOP_GROUPS: [&str; 3] = ["hair", "ears", "mane"];

const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

/// Point in the 256 × 256 viewBox.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnchorPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CharacterAnchors {
    pub head_top: AnchorPoint,
    pub eyes: AnchorPoint,
    pub neck: AnchorPoint,
}

/// A character and where its accessories attach (lot 3).
#[derive(Debug, Clone, PartialEq)]
pub struct CharacterDefinition {
    pub info: CharacterInfo,
    pub file: String,
    pub anchors: CharacterAnchors,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct CharactersFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    characters: Vec<CharacterEntry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct CharacterEntry {
    id: String,
    name: String,
    file: String,
    kind: String,
    anchors: CharacterAnchors,
}

/// [`CharacterCatalog`] over the characters of the content.
#[derive(Debug, Clone, Default)]
pub struct Characters {
    infos: Vec<CharacterInfo>,
    definitions: Vec<CharacterDefinition>,
    svgs: HashMap<String, String>,
}

impl Characters {
    pub fn definitions(&self) -> &[CharacterDefinition] {
        &self.definitions
    }

    pub fn load(directory: &Path) -> Loaded<Characters> {
        const INDEX: &str = "characters.json";
        let mut problems = Vec::new();
        let mut catalog = Characters::default();

        let index = std::fs::read_to_string(directory.join(INDEX))
            .map_err(|e| e.to_string())
            .and_then(|json| {
                serde_json::from_str::<CharactersFile>(&json).map_err(|e| e.to_string())
            });
        let entries = match index {
            Ok(file) => file.characters,
            Err(message) => {
                problems.push(ContentProblem::new(INDEX, message));
                Vec::new()
            }
        };

        for entry in entries {
            if catalog.definitions.iter().any(|d| d.info.id == entry.id) {
                problems.push(ContentProblem::new(
                    INDEX,
                    format!("Duplicate character id {}.", entry.id),
                ));
                continue;
            }

            let kind = match entry.kind.as_str() {
                "animal" => CharacterKind::Animal,
                "child" => CharacterKind::Child,
                _ => {
                    problems.push(ContentProblem::new(
                        INDEX,
                        format!(
                            "{}: kind must be animal or child, name is required.",
                            entry.id
                        ),
                    ));
                    continue;
                }
            };
            if entry.name.trim().is_empty() {
                problems.push(ContentProblem::new(
                    INDEX,
                    format!(
                        "{}: kind must be animal or child, name is required.",
                        entry.id
                    ),
                ));
                continue;
            }

            let svg = match validate_svg(&directory.join(&entry.file)) {
                Ok(svg) => svg,
                Err(problem) => {
                    problems.push(ContentProblem::new(entry.file.clone(), problem));
                    continue;
                }
            };

            let info = CharacterInfo::new(entry.id.clone(), entry.name, kind);
            catalog.infos.push(info.clone());
            catalog.definitions.push(CharacterDefinition {
                info,
                file: entry.file,
                anchors: entry.anchors,
            });
            catalog.svgs.insert(entry.id, svg);
        }

        Loaded {
            value: catalog,
            problems,
        }
    }
}

impl CharacterCatalog for Characters {
    fn all(&self) -> &[CharacterInfo] {
        &self.infos
    }

    fn svg(&self, id: &str) -> Option<&str> {
        self.svgs.get(id).map(String::as_str)
    }

    fn anchors(&self, id: &str) -> Option<educator_core::characters::CharacterAnchors> {
        let point = |p: AnchorPoint| educator_core::characters::Point::new(p.x, p.y);
        self.definitions.iter().find(|d| d.info.id == id).map(|d| {
            educator_core::characters::CharacterAnchors {
                head_top: point(d.anchors.head_top),
                eyes: point(d.anchors.eyes),
                neck: point(d.anchors.neck),
            }
        })
    }
}

/// Checks one SVG file: well-formed, 256 × 256 viewBox, named groups, no script or external
/// reference. Returns the SVG source, or why it is refused.
pub fn validate_svg(path: &Path) -> Result<String, String> {
    let svg = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let problem = {
        let document = roxmltree::Document::parse(&svg).map_err(|e| e.to_string())?;
        check_svg(&document)
    };
    match problem {
        Some(problem) => Err(problem),
        None => Ok(svg),
    }
}

fn check_svg(document: &roxmltree::Document<'_>) -> Option<String> {
    let root = document.root_element();
    if root.tag_name().name() != "svg" || root.tag_name().namespace() != Some(SVG_NAMESPACE) {
        return Some("Root element must be <svg> in the SVG namespace.".to_owned());
    }

    if root.attribute("viewBox") != Some("0 0 256 256") {
        return Some("viewBox must be \"0 0 256 256\".".to_owned());
    }

    let ids: Vec<&str> = root
        .descendants()
        .filter(|n| {
            n.is_element()
                && n.tag_name().name() == "g"
                && n.tag_name().namespace() == Some(SVG_NAMESPACE)
        })
        .filter_map(|g| g.attribute("id"))
        .collect();
    let missing: Vec<&str> = REQUIRED_GROUPS
        .iter()
        .copied()
        .filter(|g| !ids.contains(g))
        .collect();
    if !missing.is_empty() {
        return Some(format!("Missing groups: {}.", missing.join(", ")));
    }

    if !TOP_GROUPS.iter().any(|g| ids.contains(g)) {
        return Some(format!(
            "Needs one of the groups {}.",
            TOP_GROUPS.join(", ")
        ));
    }

    let forbidden = root
        .descendants()
        .filter(roxmltree::Node::is_element)
        .find(|e| {
            matches!(
                e.tag_name().name(),
                "script" | "foreignObject" | "image" | "text" | "metadata"
            ) || e.attributes().any(|a| {
                a.name().to_ascii_lowercase().starts_with("on")
                    || (a.name() == "href" && !a.value().starts_with('#'))
            })
        });
    forbidden.map(|e| {
        format!(
            "Forbidden element or attribute in <{}>.",
            e.tag_name().name()
        )
    })
}
