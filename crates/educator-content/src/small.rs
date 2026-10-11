//! The small content files: school periods (`content/calendar/periods.json`) and points scale
//! (`content/rewards/points.json`). An invalid file is reported and the built-in default is used.

use crate::{ContentProblem, Loaded};
use educator_core::points::PointsRules;
use educator_core::skills::{PeriodStart, SchoolCalendar};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct CalendarFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    #[serde(rename = "comment")]
    _comment: Option<String>,
    periods: Vec<PeriodFile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PeriodFile {
    period: u8,
    month: u32,
    day: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PointsFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    per_correct_answer: i32,
    series_completed_bonus: i32,
    perfect_series_bonus: i32,
}

pub fn load_calendar(path: &Path) -> Loaded<SchoolCalendar> {
    load(path, SchoolCalendar::default(), |file: CalendarFile| {
        SchoolCalendar::new(
            file.periods
                .iter()
                .map(|p| PeriodStart::new(p.period, p.month, p.day))
                .collect(),
        )
    })
}

pub fn load_points(path: &Path) -> Loaded<PointsRules> {
    load(path, PointsRules::DEFAULT, |file: PointsFile| {
        if file.per_correct_answer < 0
            || file.series_completed_bonus < 0
            || file.perfect_series_bonus < 0
        {
            return Err("Points cannot be negative.".to_owned());
        }

        Ok(PointsRules::new(
            file.per_correct_answer,
            file.series_completed_bonus,
            file.perfect_series_bonus,
        ))
    })
}

fn load<T, F: DeserializeOwned>(
    path: &Path,
    fallback: T,
    convert: impl FnOnce(F) -> Result<T, String>,
) -> Loaded<T> {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let result = std::fs::read_to_string(path)
        .map_err(|e| e.to_string())
        .and_then(|json| serde_json::from_str::<F>(&json).map_err(|e| e.to_string()))
        .and_then(convert);
    match result {
        Ok(value) => Loaded {
            value,
            problems: Vec::new(),
        },
        Err(message) => Loaded {
            value: fallback,
            problems: vec![ContentProblem::new(name, message)],
        },
    }
}
