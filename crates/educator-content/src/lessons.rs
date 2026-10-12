//! Loads and validates the lessons (`content/lessons/<subject>/*.json`, SUB001409). The rules
//! mirror `content/schemas/lesson.schema.json` and add what a schema cannot say: linked skills that
//! exist, values that stay on the number line, texts short enough for the voice, a title first and a
//! « Je retiens » last. An invalid file is skipped and reported: the child never sees a broken lesson.

use crate::{ContentProblem, Loaded};
use educator_core::exercises::Answer;
use educator_core::lessons::{
    landings, Animation, Chain, ChainMode, Lesson, LessonCatalog, LessonSource, QuizQuestion, Slide,
};
use educator_core::skills::{SkillCatalog, Subject, FIRST_PERIOD, LAST_PERIOD};
use educator_core::{speech, Grade};
use serde::Deserialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// A lesson lasts 3 to 5 minutes: 8 to 12 slides (fiches « Leçon CE1 »).
pub const MIN_SLIDES: usize = 8;
pub const MAX_SLIDES: usize = 12;
/// The quiz has 3 to 5 questions (SUB001411).
pub const MIN_QUIZ: usize = 3;
pub const MAX_QUIZ: usize = 5;
/// The child manipulates on at least two slides.
pub const MIN_INTERACTIVE_SLIDES: usize = 2;
/// Longest text the `speak` command accepts, once numbers and signs are written in words (same
/// value as `MAX_SPEECH_LENGTH` of the application, checked by its tests).
pub const MAX_SPOKEN_LENGTH: usize = 300;
/// Graduations of a number line: beyond, they are too close to click.
pub const MAX_GRADUATIONS: i32 = 41;
/// Largest number shown in a lesson of the primary school.
pub const MAX_NUMBER: i32 = 1_000_000;

/// [`LessonCatalog`] over the lessons loaded at start-up.
#[derive(Debug, Clone, Default)]
pub struct Lessons {
    lessons: Vec<Lesson>,
}

impl Lessons {
    pub fn new(lessons: Vec<Lesson>) -> Self {
        Self { lessons }
    }
}

impl LessonCatalog for Lessons {
    fn all(&self) -> &[Lesson] {
        &self.lessons
    }
}

/// Every `*.json` below `lessons_directory`, in path order. `skills` checks the linked skills. A
/// missing folder is not a problem: there may be no lesson yet.
pub fn load_lessons(lessons_directory: &Path, skills: &dyn SkillCatalog) -> Loaded<Vec<Lesson>> {
    let mut lessons: Vec<Lesson> = Vec::new();
    let mut problems = Vec::new();

    let mut files = Vec::new();
    collect_json_files(lessons_directory, &mut files);
    files.sort();
    for file in files {
        let name = file
            .strip_prefix(lessons_directory)
            .unwrap_or(&file)
            .to_string_lossy()
            .replace('\\', "/");
        let stem = file.file_stem().map(|s| s.to_string_lossy().into_owned());
        let parsed = std::fs::read_to_string(&file)
            .map_err(|e| vec![e.to_string()])
            .and_then(|json| parse_lesson(&json, stem.as_deref(), skills));
        match parsed {
            Err(errors) => problems.extend(
                errors
                    .into_iter()
                    .map(|e| ContentProblem::new(format!("lessons/{name}"), e)),
            ),
            Ok(lesson) if lessons.iter().any(|l| l.id == lesson.id) => {
                problems.push(ContentProblem::new(
                    format!("lessons/{name}"),
                    format!("Duplicate lesson id {}.", lesson.id),
                ));
            }
            Ok(lesson) => lessons.push(lesson),
        }
    }

    Loaded {
        value: lessons,
        problems,
    }
}

fn collect_json_files(directory: &Path, files: &mut Vec<PathBuf>) {
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
struct LessonFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    id: String,
    subject: String,
    grade: String,
    period: i32,
    skill_ids: Vec<String>,
    title: String,
    duration_minutes: i32,
    slides: Vec<SlideFile>,
    quiz: Vec<QuizFile>,
    sources: Vec<SourceFile>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SlideFile {
    title: String,
    text: String,
    speech: String,
    animation: AnimationFile,
}

#[derive(Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
enum AnimationFile {
    TitleCard {
        title: String,
        subtitle: Option<String>,
    },
    RecapCard {
        text: String,
        example: Option<String>,
    },
    CalcChain {
        chains: Vec<ChainFile>,
        #[serde(default)]
        mode: Option<String>,
    },
    NumberLine {
        from: i32,
        to: i32,
        #[serde(default = "one")]
        step: i32,
        start: i32,
        #[serde(default)]
        jumps: Vec<i32>,
        #[serde(default)]
        interactive: bool,
        #[serde(default = "yes")]
        hover_values: bool,
    },
    TensUnits {
        value: i64,
        #[serde(default)]
        complete_ten: bool,
    },
}

fn one() -> i32 {
    1
}

fn yes() -> bool {
    true
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ChainFile {
    start: i32,
    steps: Vec<i32>,
    #[serde(default)]
    crossed: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct QuizFile {
    question: String,
    spoken: String,
    answer: i32,
    #[serde(default)]
    choices: Vec<i32>,
    explanation: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SourceFile {
    title: String,
    url: String,
}

const SUBJECTS: &[(&str, Subject)] = &[("math", Subject::Math)];

/// Parses and validates one lesson; `expected_id` is the file name. Returns every error found.
pub fn parse_lesson(
    json: &str,
    expected_id: Option<&str>,
    skills: &dyn SkillCatalog,
) -> Result<Lesson, Vec<String>> {
    let file: LessonFile =
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
    let subject = SUBJECTS
        .iter()
        .find(|(name, _)| *name == file.subject)
        .map(|(_, s)| *s);
    check(
        subject.is_some(),
        format!("Unknown subject '{}'.", file.subject),
    );
    let grade = file.grade.parse::<Grade>().ok();
    check(grade.is_some(), format!("Unknown grade '{}'.", file.grade));
    check(
        (i32::from(FIRST_PERIOD)..=i32::from(LAST_PERIOD)).contains(&file.period),
        format!("period {} must be between 1 and 5.", file.period),
    );
    check(
        is_text(&file.title, 60),
        "title must be 1 to 60 characters.".to_owned(),
    );
    check(
        (1..=10).contains(&file.duration_minutes),
        format!(
            "durationMinutes {} must be between 1 and 10.",
            file.duration_minutes
        ),
    );

    // Linked skills: they exist, in the same subject and grade (« Je m'entraîne »).
    check(
        !file.skill_ids.is_empty(),
        "skillIds must list at least one skill.".to_owned(),
    );
    for id in &file.skill_ids {
        match skills.find(id) {
            None => check(false, format!("Unknown skill '{id}' in skillIds.")),
            Some(skill) => check(
                Some(skill.grade) == grade && Some(skill.subject) == subject,
                format!(
                    "Skill '{id}' is a {} skill, not {}.",
                    skill.grade, file.grade
                ),
            ),
        }
    }

    // Slides: a title first, « Je retiens » last, enough manipulation.
    check(
        (MIN_SLIDES..=MAX_SLIDES).contains(&file.slides.len()),
        format!(
            "{} slides: a lesson has {MIN_SLIDES} to {MAX_SLIDES}.",
            file.slides.len()
        ),
    );
    let mut slides = Vec::new();
    for (i, slide) in file.slides.into_iter().enumerate() {
        let n = i + 1;
        check(
            is_text(&slide.title, 40),
            format!("slide {n}: title must be 1 to 40 characters."),
        );
        check(
            is_text(&slide.text, 220),
            format!("slide {n}: text must be 1 to 220 characters."),
        );
        check_spoken(&mut check, &format!("slide {n}: speech"), &slide.speech);
        match animation(slide.animation) {
            Ok(animation) => slides.push(Slide {
                title: slide.title,
                text: slide.text,
                speech: slide.speech,
                animation,
            }),
            Err(message) => check(false, format!("slide {n}: {message}")),
        }
    }
    if let (Some(first), Some(last)) = (slides.first(), slides.last()) {
        check(
            matches!(first.animation, Animation::TitleCard { .. }),
            "The first slide must be a titleCard.".to_owned(),
        );
        check(
            matches!(last.animation, Animation::RecapCard { .. }),
            "The last slide must be a recapCard (« Je retiens »).".to_owned(),
        );
    }
    let interactive = slides.iter().filter(|s| s.is_interactive()).count();
    check(
        interactive >= MIN_INTERACTIVE_SLIDES,
        format!(
            "{interactive} interactive slide(s): the child manipulates on at least {MIN_INTERACTIVE_SLIDES}."
        ),
    );

    // Quiz.
    check(
        (MIN_QUIZ..=MAX_QUIZ).contains(&file.quiz.len()),
        format!(
            "{} quiz questions: a quiz has {MIN_QUIZ} to {MAX_QUIZ}.",
            file.quiz.len()
        ),
    );
    let mut quiz = Vec::new();
    for (i, question) in file.quiz.into_iter().enumerate() {
        let n = i + 1;
        check(
            is_text(&question.question, 80),
            format!("quiz {n}: question must be 1 to 80 characters."),
        );
        check_spoken(&mut check, &format!("quiz {n}: spoken"), &question.spoken);
        check_spoken(
            &mut check,
            &format!("quiz {n}: explanation"),
            &question.explanation,
        );
        check(
            (0..=MAX_NUMBER).contains(&question.answer),
            format!("quiz {n}: answer {} out of range.", question.answer),
        );
        if !question.choices.is_empty() {
            let distinct: HashSet<i32> = question.choices.iter().copied().collect();
            check(
                (3..=4).contains(&question.choices.len())
                    && distinct.len() == question.choices.len(),
                format!("quiz {n}: 3 or 4 different choices."),
            );
            check(
                question.choices.contains(&question.answer),
                format!("quiz {n}: the answer must be among the choices."),
            );
            check(
                question
                    .choices
                    .iter()
                    .all(|c| (0..=MAX_NUMBER).contains(c)),
                format!("quiz {n}: choices out of range."),
            );
        }
        quiz.push(QuizQuestion {
            question: question.question,
            spoken: question.spoken,
            answer: Answer::Number(question.answer),
            choices: question.choices.into_iter().map(Answer::Number).collect(),
            explanation: question.explanation,
        });
    }

    // Sources.
    check(
        !file.sources.is_empty(),
        "sources must list the references consulted.".to_owned(),
    );
    for source in &file.sources {
        check(
            is_text(&source.title, 300) && source.url.starts_with("https://"),
            format!("source '{}': a title and an https URL.", source.title),
        );
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(Lesson {
        id: file.id,
        subject: subject.unwrap_or(Subject::Math),
        grade: grade.unwrap_or(Grade::CE1),
        period: u8::try_from(file.period).unwrap_or(FIRST_PERIOD),
        skill_ids: file.skill_ids,
        title: file.title,
        duration_minutes: u8::try_from(file.duration_minutes).unwrap_or(1),
        slides,
        quiz,
        sources: file
            .sources
            .into_iter()
            .map(|s| LessonSource {
                title: s.title,
                url: s.url,
            })
            .collect(),
    })
}

/// A text read aloud must be accepted by the `speak` command once written in words.
fn check_spoken(check: &mut impl FnMut(bool, String), what: &str, text: &str) {
    let spoken = speech::normalize(text);
    let length = spoken.encode_utf16().count();
    check(
        !text.trim().is_empty() && length <= MAX_SPOKEN_LENGTH,
        format!(
            "{what} is {length} characters once numbers are in words (1 to {MAX_SPOKEN_LENGTH})."
        ),
    );
}

fn animation(file: AnimationFile) -> Result<Animation, String> {
    let in_range = |n: i64| (0..=i64::from(MAX_NUMBER)).contains(&n);
    match file {
        AnimationFile::TitleCard { title, subtitle } => {
            ensure(
                is_text(&title, 60),
                "titleCard: title of 1 to 60 characters.",
            )?;
            ensure(
                subtitle.as_deref().is_none_or(|s| is_text(s, 120)),
                "titleCard: subtitle of 1 to 120 characters.",
            )?;
            Ok(Animation::TitleCard { title, subtitle })
        }
        AnimationFile::RecapCard { text, example } => {
            ensure(
                is_text(&text, 220),
                "recapCard: text of 1 to 220 characters.",
            )?;
            ensure(
                example.as_deref().is_none_or(|s| is_text(s, 80)),
                "recapCard: example of 1 to 80 characters.",
            )?;
            Ok(Animation::RecapCard { text, example })
        }
        AnimationFile::CalcChain { chains, mode } => {
            let mode = match mode.as_deref() {
                None | Some("show") => ChainMode::Show,
                Some("match") => ChainMode::Match,
                Some(other) => return Err(format!("calcChain: unknown mode '{other}'.")),
            };
            ensure((1..=4).contains(&chains.len()), "calcChain: 1 to 4 chains.")?;
            let chains: Vec<Chain> = chains
                .into_iter()
                .map(|c| Chain {
                    start: c.start,
                    steps: c.steps,
                    crossed: c.crossed,
                })
                .collect();
            for chain in &chains {
                ensure(
                    (1..=4).contains(&chain.steps.len()) && chain.steps.iter().all(|s| *s != 0),
                    "calcChain: 1 to 4 steps, none of them 0.",
                )?;
                ensure(
                    std::iter::once(chain.start)
                        .chain(chain.values())
                        .all(|v| in_range(i64::from(v))),
                    "calcChain: every value between 0 and 1 000 000.",
                )?;
            }
            if mode == ChainMode::Match {
                ensure(
                    chains.len() >= 2 && chains.iter().all(|c| !c.crossed),
                    "calcChain match: at least 2 chains, none crossed.",
                )?;
                let totals: HashSet<i32> = chains.iter().map(Chain::total).collect();
                ensure(
                    totals.len() == chains.len() && chains.iter().all(|c| c.total() > 0),
                    "calcChain match: each chain adds a different positive number.",
                )?;
            }
            Ok(Animation::CalcChain { chains, mode })
        }
        AnimationFile::NumberLine {
            from,
            to,
            step,
            start,
            jumps,
            interactive,
            hover_values,
        } => {
            ensure(
                in_range(i64::from(from)) && in_range(i64::from(to)) && from < to,
                "numberLine: 0 ≤ from < to ≤ 1 000 000.",
            )?;
            ensure(
                step > 0 && (to - from) % step == 0,
                "numberLine: step > 0 dividing to − from.",
            )?;
            ensure(
                (to - from) / step < MAX_GRADUATIONS,
                "numberLine: at most 41 graduations.",
            )?;
            let on_line = |v: i32| (from..=to).contains(&v) && (v - from) % step == 0;
            ensure(on_line(start), "numberLine: start must be a graduation.")?;
            ensure(
                jumps.len() <= 4 && jumps.iter().all(|j| *j != 0),
                "numberLine: at most 4 jumps, none of them 0.",
            )?;
            ensure(
                landings(start, &jumps).into_iter().all(on_line),
                "numberLine: every jump must land on a graduation.",
            )?;
            ensure(
                !interactive || !jumps.is_empty(),
                "numberLine: an interactive line needs jumps.",
            )?;
            Ok(Animation::NumberLine {
                from,
                to,
                step,
                start,
                jumps,
                interactive,
                hover_values,
            })
        }
        AnimationFile::TensUnits {
            value,
            complete_ten,
        } => {
            ensure(
                (1..=999).contains(&value),
                "tensUnits: value from 1 to 999.",
            )?;
            ensure(
                !complete_ten || value % 10 != 0,
                "tensUnits: completeTen needs a value that is not a whole ten.",
            )?;
            Ok(Animation::TensUnits {
                value: u32::try_from(value).unwrap_or(1),
                complete_ten,
            })
        }
    }
}

fn ensure(ok: bool, message: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(message.to_owned())
    }
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

fn is_text(value: &str, max: usize) -> bool {
    !value.trim().is_empty() && value.encode_utf16().count() <= max
}
