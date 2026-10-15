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
mod mascot;
mod problems;
mod rewards;
mod skills;
mod small;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
pub mod speech;
mod words;

pub use characters::{
    validate_svg, AnchorPoint, CharacterAnchors, CharacterDefinition, Characters, REQUIRED_GROUPS,
    TOP_GROUPS,
};
pub use directory::{find_content_directory, CONTENT_DIR_VARIABLE, FOLDER_NAME};
pub use lessons::{
    attach_words, load_lessons, parse_lesson, Lessons, MAX_QUIZ, MAX_SLIDES, MAX_SPOKEN_LENGTH,
    MIN_INTERACTIVE_SLIDES, MIN_QUIZ, MIN_SLIDES,
};
pub use mascot::{load_mascot, MAX_MASCOT_PHRASE, MIN_MASCOT_PHRASES};
pub use problems::{load_problems, template_works_at, ProblemBank, KINDS, WORDS_FILE};
pub use rewards::{
    load_accessories, load_adaptive, load_badges, load_daily_goal, load_levels, load_reviews,
    load_stars, load_themes,
};
pub use skills::{
    load_skills, load_skills_from, load_skills_with, parse_skill, parse_skill_in, parse_skill_with,
    Skills, MAX_COMBINATIONS, MAX_OPERAND, MAX_PROBLEM_NUMBER, MIN_DISTINCT_EXERCISES,
    MIN_SKILL_WORDS,
};
pub use small::{load_calendar, load_points};
pub use words::{
    load_words, WordBank, CATEGORIES, HOMOPHONES_FILE, NOUN, SOUNDS_FILE, WORDS_FOLDER,
};

use educator_core::adaptive::AdaptiveRules;
use educator_core::points::PointsRules;
use educator_core::profile_settings::Theme;
use educator_core::reviews::ReviewRules;
use educator_core::rewards::accessories::AccessoryCatalog;
use educator_core::rewards::badges::Badge;
use educator_core::rewards::RewardRules;
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
    /// The words of the spelling skills (read by the voice, ADR 0004).
    pub words: WordBank,
    pub lessons: Lessons,
    pub calendar: SchoolCalendar,
    pub points: PointsRules,
    pub characters: Characters,
    /// Lot 3: levels, stars and goal of the day.
    pub rewards: RewardRules,
    pub badges: Vec<Badge>,
    pub accessories: AccessoryCatalog,
    pub adaptive: AdaptiveRules,
    pub reviews: ReviewRules,
    pub themes: Vec<Theme>,
    /// The mascot (SUB001417), when its files are valid.
    pub mascot: Option<educator_core::mascot::Mascot>,
    pub problems: Vec<ContentProblem>,
}

impl EducatorContent {
    pub fn load(directory: &Path) -> Self {
        let bank = load_problems(&directory.join("problems"));
        let words = load_words(directory);
        let skills = load_skills_from(&directory.join("skills"), &bank.value, &words.value);
        let catalog = Skills::new(skills.value);
        let mut lessons = load_lessons(&directory.join("lessons"), &catalog);
        attach_words(&mut lessons.value, &words.value);
        let calendar = load_calendar(&directory.join("calendar").join("periods.json"));
        let points = load_points(&directory.join("rewards").join("points.json"));
        let characters = Characters::load(&directory.join("characters"));
        let mascot = load_mascot(&directory.join("mascot"));
        let rewards_dir = directory.join("rewards");
        let levels = load_levels(&rewards_dir.join("levels.json"));
        let stars = load_stars(&rewards_dir.join("stars.json"));
        let goal = load_daily_goal(&rewards_dir.join("daily-goal.json"));
        let badges = load_badges(&rewards_dir, &catalog);
        let accessories = load_accessories(&directory.join("accessories"), &badges.value);
        let adaptive = load_adaptive(&directory.join("progress").join("adaptive.json"));
        let reviews = load_reviews(&directory.join("progress").join("reviews.json"));
        let themes = load_themes(&directory.join("themes").join("themes.json"));

        let mut problems = bank.problems;
        problems.extend(words.problems);
        problems.extend(skills.problems);
        problems.extend(lessons.problems);
        problems.extend(calendar.problems);
        problems.extend(points.problems);
        problems.extend(characters.problems);
        problems.extend(mascot.problems);
        for more in [
            levels.problems,
            stars.problems,
            goal.problems,
            badges.problems,
            accessories.problems,
            adaptive.problems,
            reviews.problems,
            themes.problems,
        ] {
            problems.extend(more);
        }
        Self {
            directory: directory.to_path_buf(),
            skills: catalog,
            words: words.value,
            lessons: Lessons::new(lessons.value),
            calendar: calendar.value,
            points: points.value,
            characters: characters.value,
            rewards: RewardRules {
                levels: levels.value,
                stars: stars.value,
                goal: goal.value,
            },
            badges: badges.value,
            accessories: accessories.value,
            adaptive: adaptive.value,
            reviews: reviews.value,
            themes: themes.value,
            mascot: mascot.value,
            problems,
        }
    }
}
