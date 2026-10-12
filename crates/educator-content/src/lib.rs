//! Pedagogical content of Educator (port of `Educator.Content`): the skills, lessons, school periods,
//! points scale and characters read from the `content/` folder, validated once at start-up. Invalid files
//! are skipped and listed in [`EducatorContent::problems`]: the application logs them, the tests fail
//! on them, the child never sees a broken rule.
//!
//! On the desktop, the [`speech`] module runs the Piper voice (ADR 0002). It does not exist on
//! iOS / Android, where the application uses the core's `UnavailableSpeechSynthesizer` until the
//! tablet lot.

mod characters;
mod directory;
mod lessons;
mod problems;
mod skills;
mod small;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
pub mod speech;

pub use characters::{
    validate_svg, AnchorPoint, CharacterAnchors, CharacterDefinition, Characters, REQUIRED_GROUPS,
    TOP_GROUPS,
};
pub use directory::{find_content_directory, CONTENT_DIR_VARIABLE, FOLDER_NAME};
pub use lessons::{
    load_lessons, parse_lesson, Lessons, MAX_QUIZ, MAX_SLIDES, MAX_SPOKEN_LENGTH,
    MIN_INTERACTIVE_SLIDES, MIN_QUIZ, MIN_SLIDES,
};
pub use problems::{load_problems, template_works_at, ProblemBank, KINDS, WORDS_FILE};
pub use skills::{
    load_skills, load_skills_with, parse_skill, parse_skill_with, Skills, MAX_COMBINATIONS,
    MAX_OPERAND, MAX_PROBLEM_NUMBER, MIN_DISTINCT_EXERCISES,
};
pub use small::{load_calendar, load_points};

use educator_core::points::PointsRules;
use educator_core::skills::SchoolCalendar;
use std::fmt;
use std::path::{Path, PathBuf};

/// An invalid content file: which file and why (English, for the logs and the tests).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentProblem {
    pub file: String,
    pub message: String,
}

impl ContentProblem {
    pub fn new(file: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            file: file.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for ContentProblem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.file, self.message)
    }
}

/// Result of a content load: what is valid, and the problems of what was skipped.
#[derive(Debug, Clone)]
pub struct Loaded<T> {
    pub value: T,
    pub problems: Vec<ContentProblem>,
}

/// Everything read from the `content/` folder, loaded once at start-up.
#[derive(Debug)]
pub struct EducatorContent {
    pub directory: PathBuf,
    pub skills: Skills,
    pub lessons: Lessons,
    pub calendar: SchoolCalendar,
    pub points: PointsRules,
    pub characters: Characters,
    pub problems: Vec<ContentProblem>,
}

impl EducatorContent {
    pub fn load(directory: &Path) -> Self {
        let skills = load_skills(&directory.join("skills"));
        let catalog = Skills::new(skills.value);
        let lessons = load_lessons(&directory.join("lessons"), &catalog);
        let calendar = load_calendar(&directory.join("calendar").join("periods.json"));
        let points = load_points(&directory.join("rewards").join("points.json"));
        let characters = Characters::load(&directory.join("characters"));

        let mut problems = skills.problems;
        problems.extend(lessons.problems);
        problems.extend(calendar.problems);
        problems.extend(points.problems);
        problems.extend(characters.problems);
        Self {
            directory: directory.to_path_buf(),
            skills: catalog,
            lessons: Lessons::new(lessons.value),
            calendar: calendar.value,
            points: points.value,
            characters: characters.value,
            problems,
        }
    }
}
