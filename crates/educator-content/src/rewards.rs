//! Content of lot 3: the scales of the rewards (`content/rewards`: levels, stars, goal of the day,
//! badges and their icons), the accessories of the avatars (`content/accessories`), the thresholds
//! of the adaptive difficulty and of the review queue (`content/progress`) and the themes
//! (`content/themes`). An invalid scale is reported and its built-in default used; an invalid
//! badge or accessory is skipped and reported.

use crate::small::load;
use crate::{ContentProblem, Loaded};
use educator_core::adaptive::AdaptiveRules;
use educator_core::profile_settings::{Theme, DEFAULT_THEME};
use educator_core::reviews::ReviewRules;
use educator_core::rewards::accessories::{
    Accessory, AccessoryCatalog, AccessoryColor, AccessoryLayer, AccessorySlot, Point, Unlock,
};
use educator_core::rewards::badges::{Badge, BadgeCondition};
use educator_core::rewards::daily_goal::DailyGoalRules;
use educator_core::rewards::levels::LevelScale;
use educator_core::rewards::stars::StarRules;
use educator_core::skills::{SkillCatalog, SkillCategory, Subject};
use serde::Deserialize;
use std::path::Path;

const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct LevelsFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    #[serde(rename = "comment")]
    _comment: Option<String>,
    levels: Vec<i32>,
    step_after_last: i32,
}

pub fn load_levels(path: &Path) -> Loaded<LevelScale> {
    load(path, LevelScale::default_scale(), |file: LevelsFile| {
        LevelScale::new(file.levels, file.step_after_last)
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct StarsFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    two_stars: u8,
    three_stars: u8,
}

pub fn load_stars(path: &Path) -> Loaded<StarRules> {
    load(path, StarRules::DEFAULT, |file: StarsFile| {
        StarRules::new(file.two_stars, file.three_stars)
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DailyGoalFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    default_series: u32,
    bonus_points: i32,
}

pub fn load_daily_goal(path: &Path) -> Loaded<DailyGoalRules> {
    load(path, DailyGoalRules::DEFAULT, |file: DailyGoalFile| {
        DailyGoalRules::new(file.default_series, file.bonus_points)
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AdaptiveFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    raise_above: u8,
    lower_below: u8,
    min_answers: u32,
}

pub fn load_adaptive(path: &Path) -> Loaded<AdaptiveRules> {
    load(path, AdaptiveRules::DEFAULT, |file: AdaptiveFile| {
        AdaptiveRules::new(file.raise_above, file.lower_below, file.min_answers)
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ReviewsFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    intervals: Vec<u32>,
    series_length: usize,
}

pub fn load_reviews(path: &Path) -> Loaded<ReviewRules> {
    load(path, ReviewRules::default(), |file: ReviewsFile| {
        ReviewRules::new(file.intervals, file.series_length)
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ThemesFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    themes: Vec<ThemeEntry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ThemeEntry {
    id: String,
    name: String,
    description: String,
}

/// The themes offered (SUB001513); the sport theme alone when the file is invalid.
pub fn load_themes(path: &Path) -> Loaded<Vec<Theme>> {
    let fallback = vec![Theme {
        id: DEFAULT_THEME.to_owned(),
        name: "Grand Prix".to_owned(),
        description: String::new(),
    }];
    load(path, fallback, |file: ThemesFile| {
        let themes: Vec<Theme> = file
            .themes
            .into_iter()
            .map(|t| Theme {
                id: t.id,
                name: t.name,
                description: t.description,
            })
            .collect();
        if !themes.iter().any(|t| t.id == DEFAULT_THEME) {
            return Err(format!("The theme {DEFAULT_THEME} is required."));
        }
        let mut ids: Vec<&str> = themes.iter().map(|t| t.id.as_str()).collect();
        ids.sort_unstable();
        ids.dedup();
        if ids.len() != themes.len() {
            return Err("Duplicate theme id.".to_owned());
        }
        if themes
            .iter()
            .any(|t| t.name.trim().is_empty() || !is_slug(&t.id))
        {
            return Err("A theme needs a name and an id in lower-case letters.".to_owned());
        }
        Ok(themes)
    })
}

fn is_slug(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

// ── Badges ───────────────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct BadgesFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    badges: Vec<BadgeEntry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct BadgeEntry {
    id: String,
    title: String,
    description: String,
    icon: String,
    condition: ConditionEntry,
}

#[derive(Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
enum ConditionEntry {
    SkillMastery {
        skills: Vec<String>,
        min_score: u8,
    },
    CategoryMastery {
        category: String,
        count: u32,
        min_score: u8,
    },
    SeriesCompleted {
        count: u32,
        subject: Option<String>,
    },
    PerfectSeries {
        count: u32,
    },
    ActiveDays {
        count: u32,
    },
    GoalStreak {
        days: u32,
    },
    LessonsFinished {
        count: u32,
    },
    Stars {
        count: u32,
    },
    Level {
        level: u32,
    },
}

fn category_of(name: &str) -> Option<SkillCategory> {
    use SkillCategory as C;
    Some(match name {
        "numbers" => C::Numbers,
        "addSubtract" => C::AddSubtract,
        "multiply" => C::Multiply,
        "fractionsHalves" => C::FractionsHalves,
        "problems" => C::Problems,
        "lettersAndSounds" => C::LettersAndSounds,
        "spellingRules" => C::SpellingRules,
        "silentLetters" => C::SilentLetters,
        "agreement" => C::Agreement,
        "wordsToKnow" => C::WordsToKnow,
        _ => return None,
    })
}

fn condition(entry: ConditionEntry, skills: &dyn SkillCatalog) -> Result<BadgeCondition, String> {
    let score = |s: u8| {
        if (1..=100).contains(&s) {
            Ok(s)
        } else {
            Err("minScore is 1 to 100.".to_owned())
        }
    };
    let positive = |n: u32| {
        if n > 0 {
            Ok(n)
        } else {
            Err("A count is at least 1.".to_owned())
        }
    };
    Ok(match entry {
        ConditionEntry::SkillMastery {
            skills: ids,
            min_score,
        } => {
            if ids.is_empty() {
                return Err("skillMastery needs skills.".to_owned());
            }
            if let Some(unknown) = ids.iter().find(|id| skills.find(id).is_none()) {
                return Err(format!("Unknown skill {unknown}."));
            }
            BadgeCondition::SkillMastery {
                skills: ids,
                min_score: score(min_score)?,
            }
        }
        ConditionEntry::CategoryMastery {
            category,
            count,
            min_score,
        } => BadgeCondition::CategoryMastery {
            category: category_of(&category)
                .ok_or_else(|| format!("Unknown family {category}."))?,
            count: positive(count)?,
            min_score: score(min_score)?,
        },
        ConditionEntry::SeriesCompleted { count, subject } => BadgeCondition::SeriesCompleted {
            count: positive(count)?,
            subject: subject
                .map(|s| Subject::from_name(&s).ok_or_else(|| format!("Unknown subject {s}.")))
                .transpose()?,
        },
        ConditionEntry::PerfectSeries { count } => BadgeCondition::PerfectSeries {
            count: positive(count)?,
        },
        ConditionEntry::ActiveDays { count } => BadgeCondition::ActiveDays {
            count: positive(count)?,
        },
        ConditionEntry::GoalStreak { days } => BadgeCondition::GoalStreak {
            days: positive(days)?,
        },
        ConditionEntry::LessonsFinished { count } => BadgeCondition::LessonsFinished {
            count: positive(count)?,
        },
        ConditionEntry::Stars { count } => BadgeCondition::Stars {
            count: positive(count)?,
        },
        ConditionEntry::Level { level } => BadgeCondition::Level {
            level: positive(level)?,
        },
    })
}

/// `content/rewards/badges.json` and the icons it names (`content/rewards/<icon>`).
pub fn load_badges(directory: &Path, skills: &dyn SkillCatalog) -> Loaded<Vec<Badge>> {
    const FILE: &str = "badges.json";
    let mut problems = Vec::new();
    let entries = match std::fs::read_to_string(directory.join(FILE))
        .map_err(|e| e.to_string())
        .and_then(|json| serde_json::from_str::<BadgesFile>(&json).map_err(|e| e.to_string()))
    {
        Ok(file) => file.badges,
        Err(message) => {
            return Loaded {
                value: Vec::new(),
                problems: vec![ContentProblem::new(FILE, message)],
            }
        }
    };
    let mut badges: Vec<Badge> = Vec::new();
    for entry in entries {
        let id = entry.id.clone();
        let result = (|| {
            if !is_slug(&entry.id) {
                return Err("The id is lower-case letters, digits and dashes.".to_owned());
            }
            if badges.iter().any(|b| b.id == entry.id) {
                return Err("Duplicate badge id.".to_owned());
            }
            if entry.title.trim().is_empty() || entry.description.trim().is_empty() {
                return Err("A badge needs a title and a description.".to_owned());
            }
            let icon = read_svg(&directory.join(&entry.icon), true)?;
            Ok(Badge {
                id: entry.id,
                title: entry.title,
                description: entry.description,
                icon,
                condition: condition(entry.condition, skills)?,
            })
        })();
        match result {
            Ok(badge) => badges.push(badge),
            Err(message) => problems.push(ContentProblem::new(FILE, format!("{id}: {message}"))),
        }
    }
    Loaded {
        value: badges,
        problems,
    }
}

// ── Accessories ──────────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AccessoriesFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    #[serde(rename = "comment")]
    _comment: Option<String>,
    colors: Vec<ColorEntry>,
    accessories: Vec<AccessoryEntry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ColorEntry {
    id: String,
    name: String,
    main: String,
    light: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AccessoryEntry {
    id: String,
    name: String,
    slot: String,
    #[serde(default)]
    layer: Option<String>,
    file: String,
    anchor: PointEntry,
    colors: Vec<String>,
    unlock: UnlockEntry,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PointEntry {
    x: f64,
    y: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct UnlockEntry {
    level: Option<u32>,
    badge: Option<String>,
}

fn is_color(value: &str) -> bool {
    value.len() == 7 && value.starts_with('#') && value[1..].chars().all(|c| c.is_ascii_hexdigit())
}

/// `content/accessories/accessories.json` and its SVG files. `badges`: the badges that may unlock
/// an accessory.
pub fn load_accessories(directory: &Path, badges: &[Badge]) -> Loaded<AccessoryCatalog> {
    const FILE: &str = "accessories.json";
    let mut problems = Vec::new();
    let file = match std::fs::read_to_string(directory.join(FILE))
        .map_err(|e| e.to_string())
        .and_then(|json| serde_json::from_str::<AccessoriesFile>(&json).map_err(|e| e.to_string()))
    {
        Ok(file) => file,
        Err(message) => {
            return Loaded {
                value: AccessoryCatalog::default(),
                problems: vec![ContentProblem::new(FILE, message)],
            }
        }
    };
    let mut catalog = AccessoryCatalog::default();
    for color in file.colors {
        if !is_slug(&color.id)
            || catalog.colors.iter().any(|c| c.id == color.id)
            || !is_color(&color.main)
            || !is_color(&color.light)
            || color.name.trim().is_empty()
        {
            problems.push(ContentProblem::new(
                FILE,
                format!("Color {}: unique id, a name and #RRGGBB values.", color.id),
            ));
            continue;
        }
        catalog.colors.push(AccessoryColor {
            id: color.id,
            name: color.name,
            main: color.main,
            light: color.light,
        });
    }
    for entry in file.accessories {
        let id = entry.id.clone();
        let result = (|| {
            if !is_slug(&entry.id) || catalog.find(&entry.id).is_some() {
                return Err("Unique id in lower-case letters, digits and dashes.".to_owned());
            }
            if entry.name.trim().is_empty() {
                return Err("A name is required.".to_owned());
            }
            let slot = AccessorySlot::from_name(&entry.slot)
                .ok_or_else(|| format!("Unknown slot {}.", entry.slot))?;
            let layer = match entry.layer.as_deref() {
                None | Some("front") => AccessoryLayer::Front,
                Some("back") => AccessoryLayer::Back,
                Some(other) => return Err(format!("Unknown layer {other}.")),
            };
            if entry.colors.is_empty() {
                return Err("At least one color.".to_owned());
            }
            if let Some(unknown) = entry.colors.iter().find(|c| catalog.color(c).is_none()) {
                return Err(format!("Unknown color {unknown}."));
            }
            let unlock = match (entry.unlock.level, entry.unlock.badge) {
                (Some(level), None) if level >= 1 => Unlock::Level(level),
                (None, Some(badge)) if badges.iter().any(|b| b.id == badge) => Unlock::Badge(badge),
                _ => return Err("Unlocked by a level (from 1) or a known badge.".to_owned()),
            };
            let svg = read_svg(&directory.join(&entry.file), false)?;
            Ok(Accessory {
                id: entry.id,
                name: entry.name,
                slot,
                layer,
                anchor: Point::new(entry.anchor.x, entry.anchor.y),
                content: educator_core::rewards::accessories::svg_content(&svg).to_owned(),
                colors: entry.colors,
                unlock,
            })
        })();
        match result {
            Ok(accessory) => catalog.accessories.push(accessory),
            Err(message) => problems.push(ContentProblem::new(FILE, format!("{id}: {message}"))),
        }
    }
    Loaded {
        value: catalog,
        problems,
    }
}

/// An icon or an accessory: well-formed SVG with a `viewBox`, no script, no external reference, no
/// `id` (the layers of an avatar share one document). Text is allowed in a badge only (numbers).
fn read_svg(path: &Path, allow_text: bool) -> Result<String, String> {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let svg = std::fs::read_to_string(path).map_err(|e| format!("{name}: {e}"))?;
    let document = roxmltree::Document::parse(&svg).map_err(|e| format!("{name}: {e}"))?;
    let root = document.root_element();
    if root.tag_name().name() != "svg" || root.tag_name().namespace() != Some(SVG_NAMESPACE) {
        return Err(format!("{name}: root element must be <svg>."));
    }
    if root.attribute("viewBox").is_none() {
        return Err(format!("{name}: a viewBox is required."));
    }
    let forbidden = root
        .descendants()
        .filter(roxmltree::Node::is_element)
        .find(|e| {
            matches!(
                e.tag_name().name(),
                "script" | "foreignObject" | "image" | "metadata" | "style"
            ) || (!allow_text && e.tag_name().name() == "text")
                || e.attributes().any(|a| {
                    a.name().to_ascii_lowercase().starts_with("on")
                        || a.name() == "href"
                        || a.name() == "id"
                })
        });
    if let Some(e) = forbidden {
        return Err(format!(
            "{name}: forbidden element or attribute in <{}>.",
            e.tag_name().name()
        ));
    }
    Ok(svg)
}
