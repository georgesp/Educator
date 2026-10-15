//! Loads and validates the lessons (`content/lessons/<subject>/*.json`, SUB001409). The rules
//! mirror `content/schemas/lesson.schema.json` and add what a schema cannot say: linked skills that
//! exist, values that stay on the number line, texts short enough for the voice, a title first and a
//! « Je retiens » last. An invalid file is skipped and reported: the child never sees a broken lesson.

mod figures;

use crate::{ContentProblem, Loaded};
use educator_core::exercises::{Answer, Comparison, Fraction};
use educator_core::lessons::{
    grapheme_position, landings, written_columns, Animation, Chain, ChainMode, ColumnOperation,
    Equation, EquationTerm, Lesson, LessonCatalog, LessonSource, LetterTiles, Operator,
    QuizQuestion, Slide, SortedWord, SoundSort, SubtractionMethod,
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
/// Largest number of a written operation (four columns at most).
pub const MAX_WRITTEN_NUMBER: i64 = 9_999;

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

/// Gives each word to write of the quizzes its word of the bank, when it is one: it is then read
/// with its article, its pronunciation and its sentence (SUB026706).
pub fn attach_words(lessons: &mut [Lesson], words: &crate::WordBank) {
    for question in lessons.iter_mut().flat_map(|l| l.quiz.iter_mut()) {
        if let (Answer::Spelling(word), true) = (&question.answer, question.choices.is_empty()) {
            question.word = words.find(word).cloned();
        }
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
        #[serde(default)]
        flags: Vec<i32>,
    },
    TensUnits(figures::TensUnitsFile),
    ColumnOperation {
        operation: String,
        operands: Vec<i64>,
        #[serde(default)]
        method: Option<String>,
        #[serde(default)]
        computed_columns: Option<i64>,
        #[serde(default)]
        interactive: bool,
    },
    Equation {
        left: i64,
        operation: String,
        right: i64,
        #[serde(default)]
        missing: Option<String>,
        #[serde(default)]
        family: bool,
        #[serde(default)]
        interactive: bool,
        #[serde(default)]
        choices: Vec<i64>,
    },
    CompareNumbers(figures::CompareNumbersFile),
    SequencePattern(figures::SequencePatternFile),
    ArrayGrid(figures::ArrayGridFile),
    TimesTable(figures::TimesTableFile),
    PlaceValueTable(figures::PlaceValueTableFile),
    GroupObjects(figures::GroupObjectsFile),
    TenFrame(figures::TenFrameFile),
    FractionBar(figures::FractionBarFile),
    BarModel(figures::BarModelFile),
    LetterTiles {
        word: String,
        /// The tiles in order; the letters of the word when absent.
        #[serde(default)]
        tiles: Option<Vec<String>>,
        #[serde(default)]
        distractors: Vec<String>,
        #[serde(default = "yes")]
        interactive: bool,
    },
    SoundSort {
        groups: Vec<String>,
        words: Vec<SortedWordFile>,
        #[serde(default = "yes")]
        interactive: bool,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SortedWordFile {
    word: String,
    group: String,
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
    answer: QuizValueFile,
    #[serde(default)]
    choices: Vec<QuizValueFile>,
    explanation: String,
}

/// A quiz answer as written in the file: a number, or a text for the other kinds of answers
/// (« < », « oui », « 3/4 »).
#[derive(Deserialize, Clone)]
#[serde(untagged)]
enum QuizValueFile {
    Number(i64),
    Text(String),
}

/// The answer a quiz value stands for: a number from 0 to 1 000 000, a sign (`<`, `>`, `=`),
/// `oui` / `non`, or a fraction `a/b` (any equal fraction is then accepted).
fn quiz_answer(value: &QuizValueFile) -> Option<Answer> {
    match value {
        QuizValueFile::Number(n) => (0..=i64::from(MAX_NUMBER))
            .contains(n)
            .then_some(Answer::Number(*n as i32)),
        QuizValueFile::Text(text) => match text.as_str() {
            "<" => Some(Answer::Comparison(Comparison::Less)),
            ">" => Some(Answer::Comparison(Comparison::Greater)),
            "=" => Some(Answer::Comparison(Comparison::Equal)),
            "oui" => Some(Answer::YesNo(true)),
            "non" => Some(Answer::YesNo(false)),
            _ => {
                let (numerator, denominator) = text.split_once('/')?;
                let numerator: u32 = numerator.parse().ok()?;
                let denominator: u32 = denominator.parse().ok()?;
                (denominator >= 1 && numerator <= 100 && denominator <= 100)
                    .then(|| Answer::Fraction(Fraction::new(numerator, denominator)))
            }
        },
    }
}

/// Same kind of answer (two numbers, two fractions…).
fn same_kind(a: &Answer, b: &Answer) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

fn quiz_label(value: &QuizValueFile) -> String {
    match value {
        QuizValueFile::Number(n) => n.to_string(),
        QuizValueFile::Text(text) => text.clone(),
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SourceFile {
    title: String,
    url: String,
}

const SUBJECTS: &[(&str, Subject)] = &[("math", Subject::Math), ("spelling", Subject::Spelling)];

/// A quiz answer of a spelling lesson: a word or letters to write or pick (no tolerance on accents).
fn spelling_answer(value: &QuizValueFile) -> Option<Answer> {
    match value {
        QuizValueFile::Text(text) if !text.trim().is_empty() && text.trim() == text => {
            Some(Answer::Spelling(text.clone()))
        }
        _ => None,
    }
}

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
        // A spelling lesson asks for words; « oui » and « non » keep their meaning.
        let read = |value: &QuizValueFile| {
            if subject == Some(Subject::Spelling)
                && !matches!(value, QuizValueFile::Text(t) if t == "oui" || t == "non")
            {
                spelling_answer(value)
            } else {
                quiz_answer(value)
            }
        };
        let answer = read(&question.answer);
        check(
            answer.is_some(),
            format!(
                "quiz {n}: answer '{}' must be a number from 0 to 1 000 000, <, >, =, oui, non or a fraction a/b (a word in a spelling lesson).",
                quiz_label(&question.answer)
            ),
        );
        let choices: Vec<Option<Answer>> = question.choices.iter().map(read).collect();
        if !choices.is_empty() {
            let labels: HashSet<String> = question.choices.iter().map(quiz_label).collect();
            let yes_no = matches!(answer, Some(Answer::YesNo(_)));
            check(
                labels.len() == choices.len()
                    && if yes_no {
                        choices.len() == 2
                    } else {
                        (3..=4).contains(&choices.len())
                    },
                format!(
                    "quiz {n}: 3 or 4 different choices (oui and non for a yes / no question)."
                ),
            );
            check(
                choices.iter().all(Option::is_some),
                format!("quiz {n}: every choice must be a valid answer."),
            );
            if let Some(answer) = &answer {
                check(
                    choices
                        .iter()
                        .flatten()
                        .all(|choice| same_kind(choice, answer)),
                    format!("quiz {n}: the choices must be of the same kind as the answer."),
                );
                check(
                    question
                        .choices
                        .iter()
                        .any(|c| quiz_label(c) == quiz_label(&question.answer)),
                    format!("quiz {n}: the answer must be among the choices."),
                );
                // Only one choice may be right (2/4 is also right when 1/2 is expected).
                check(
                    choices
                        .iter()
                        .flatten()
                        .filter(|choice| answer.accepts(choice))
                        .count()
                        == 1,
                    format!("quiz {n}: exactly one choice must be right."),
                );
            }
        } else {
            check(
                matches!(
                    answer,
                    None | Some(Answer::Number(_)) | Some(Answer::Spelling(_))
                ),
                format!(
                    "quiz {n}: only a number or a word can be typed; give choices for the other answers."
                ),
            );
            // A word to write is heard, never shown: the child would only copy it.
            if let Some(Answer::Spelling(word)) = &answer {
                check(
                    !question
                        .question
                        .to_lowercase()
                        .contains(&word.to_lowercase()),
                    format!(
                        "quiz {n}: the word to write ('{word}') must not be shown in the question, only spoken."
                    ),
                );
            }
        }
        quiz.push(QuizQuestion {
            question: question.question,
            spoken: question.spoken,
            answer: answer.unwrap_or(Answer::Number(0)),
            choices: choices.into_iter().flatten().collect(),
            explanation: question.explanation,
            word: None,
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
                Some("pick") => ChainMode::Pick,
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
            if mode == ChainMode::Pick {
                ensure(
                    chains.len() >= 2 && chains.iter().filter(|c| !c.crossed).count() == 1,
                    "calcChain pick: 2 to 4 chains, only one of them right (the others crossed).",
                )?;
                ensure(
                    chains.iter().all(|c| c.start == chains[0].start),
                    "calcChain pick: every chain starts from the same number.",
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
            flags,
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
            let distinct: HashSet<i32> = flags.iter().copied().collect();
            ensure(
                flags.len() <= 3
                    && distinct.len() == flags.len()
                    && flags.iter().all(|f| on_line(*f)),
                "numberLine: at most 3 different flags, each on a graduation.",
            )?;
            Ok(Animation::NumberLine {
                from,
                to,
                step,
                start,
                jumps,
                interactive,
                hover_values,
                flags,
            })
        }
        AnimationFile::TensUnits(file) => figures::tens_units(file),
        AnimationFile::ColumnOperation {
            operation,
            operands,
            method,
            computed_columns,
            interactive,
        } => column_operation(
            &operation,
            &operands,
            method.as_deref(),
            computed_columns,
            interactive,
        ),
        AnimationFile::Equation {
            left,
            operation,
            right,
            missing,
            family,
            interactive,
            choices,
        } => {
            let operator = operator(&operation, "equation")?;
            ensure(
                in_range(left) && in_range(right),
                "equation: numbers between 0 and 1 000 000.",
            )?;
            let (left, right) = (left as u32, right as u32);
            ensure(
                operator
                    .apply(left, right)
                    .is_some_and(|r| in_range(i64::from(r))),
                "equation: the result must be between 0 and 1 000 000.",
            )?;
            let missing = match missing.as_deref() {
                None => None,
                Some("left") => Some(EquationTerm::Left),
                Some("right") => Some(EquationTerm::Right),
                Some("result") => Some(EquationTerm::Result),
                Some(other) => return Err(format!("equation: unknown missing term '{other}'.")),
            };
            let equation = Equation {
                left,
                operator,
                right,
                missing,
                family,
                interactive,
                choices: choices
                    .iter()
                    .map(|c| u32::try_from(*c).unwrap_or(u32::MAX))
                    .collect(),
            };
            if interactive {
                let distinct: HashSet<u32> = equation.choices.iter().copied().collect();
                ensure(
                    equation.missing.is_some(),
                    "equation: an interactive equation needs a missing term.",
                )?;
                ensure(
                    (3..=4).contains(&equation.choices.len())
                        && distinct.len() == equation.choices.len()
                        && choices.iter().all(|c| in_range(*c)),
                    "equation: 3 or 4 different choices between 0 and 1 000 000.",
                )?;
                ensure(
                    equation
                        .missing_value()
                        .is_some_and(|v| equation.choices.contains(&v)),
                    "equation: the missing term must be among the choices.",
                )?;
            } else {
                ensure(
                    choices.is_empty(),
                    "equation: choices only for an interactive equation.",
                )?;
            }
            Ok(Animation::Equation(equation))
        }
        AnimationFile::CompareNumbers(file) => figures::compare_numbers(file),
        AnimationFile::SequencePattern(file) => figures::sequence_pattern(file),
        AnimationFile::ArrayGrid(file) => figures::array_grid(file),
        AnimationFile::TimesTable(file) => figures::times_table(file),
        AnimationFile::PlaceValueTable(file) => figures::place_value_table(file),
        AnimationFile::GroupObjects(file) => figures::group_objects(file),
        AnimationFile::TenFrame(file) => figures::ten_frame(file),
        AnimationFile::FractionBar(file) => figures::fraction_bar(file),
        AnimationFile::BarModel(file) => figures::bar_model(file),
        AnimationFile::LetterTiles {
            word,
            tiles,
            distractors,
            interactive,
        } => letter_tiles(word, tiles, distractors, interactive),
        AnimationFile::SoundSort {
            groups,
            words,
            interactive,
        } => sound_sort(groups, words, interactive),
    }
}

/// The operation of an equality: addition, subtraction or multiplication.
/// A word of a spelling animation: lower case, NFC, 1 to 20 letters.
fn is_word(word: &str) -> bool {
    let count = word.chars().count();
    (1..=20).contains(&count)
        && word == word.trim()
        && word == educator_core::spelling::nfc(word)
        && word == word.to_lowercase()
}

fn letter_tiles(
    word: String,
    tiles: Option<Vec<String>>,
    distractors: Vec<String>,
    interactive: bool,
) -> Result<Animation, String> {
    ensure(
        is_word(&word),
        "letterTiles: a word of 1 to 20 lower-case letters.",
    )?;
    let tiles = tiles.unwrap_or_else(|| word.chars().map(String::from).collect());
    ensure(
        (2..=10).contains(&tiles.len()) && tiles.iter().all(|t| !t.is_empty()),
        "letterTiles: 2 to 10 tiles, none empty.",
    )?;
    ensure(
        tiles.concat() == word,
        "letterTiles: the tiles, in order, must make the word.",
    )?;
    ensure(
        distractors.len() <= 4
            && distractors
                .iter()
                .all(|d| !d.is_empty() && !tiles.contains(d))
            && distractors.iter().collect::<HashSet<_>>().len() == distractors.len(),
        "letterTiles: at most 4 different distractors, none equal to a tile.",
    )?;
    Ok(Animation::LetterTiles(LetterTiles {
        word,
        tiles,
        distractors,
        interactive,
    }))
}

fn sound_sort(
    groups: Vec<String>,
    words: Vec<SortedWordFile>,
    interactive: bool,
) -> Result<Animation, String> {
    ensure(
        (2..=4).contains(&groups.len())
            && groups.iter().all(|g| !g.is_empty())
            && groups.iter().collect::<HashSet<_>>().len() == groups.len(),
        "soundSort: 2 to 4 different groups.",
    )?;
    ensure(
        (4..=9).contains(&words.len()),
        "soundSort: 4 to 9 words to sort.",
    )?;
    let mut sorted = Vec::new();
    for file in words {
        ensure(
            is_word(&file.word),
            "soundSort: words of 1 to 20 lower-case letters.",
        )?;
        let group = groups
            .iter()
            .position(|g| *g == file.group)
            .ok_or_else(|| format!("soundSort: '{}' is not a group.", file.group))?;
        let start = grapheme_position(&file.word, &file.group)
            .ok_or_else(|| format!("soundSort: '{}' is not in '{}'.", file.group, file.word))?;
        sorted.push(SortedWord {
            length: file.group.chars().count(),
            word: file.word,
            group,
            start,
        });
    }
    ensure(
        (0..groups.len()).all(|g| sorted.iter().any(|w| w.group == g)),
        "soundSort: every group gets at least one word.",
    )?;
    ensure(
        sorted.iter().map(|w| &w.word).collect::<HashSet<_>>().len() == sorted.len(),
        "soundSort: a word only once.",
    )?;
    Ok(Animation::SoundSort(SoundSort {
        groups,
        words: sorted,
        interactive,
    }))
}

fn operator(name: &str, animation: &str) -> Result<Operator, String> {
    match name {
        "add" => Ok(Operator::Add),
        "subtract" => Ok(Operator::Subtract),
        "multiply" => Ok(Operator::Multiply),
        other => Err(format!(
            "{animation}: unknown operation '{other}' (add, subtract or multiply)."
        )),
    }
}

fn column_operation(
    operation: &str,
    operands: &[i64],
    method: Option<&str>,
    computed_columns: Option<i64>,
    interactive: bool,
) -> Result<Animation, String> {
    let operator = operator(operation, "columnOperation")?;
    ensure(
        operator != Operator::Multiply,
        "columnOperation: add or subtract (the written multiplication comes in CE2).",
    )?;
    ensure(
        operands
            .iter()
            .all(|n| (1..=MAX_WRITTEN_NUMBER).contains(n)),
        "columnOperation: numbers between 1 and 9 999.",
    )?;
    let numbers: Vec<u32> = operands.iter().map(|n| *n as u32).collect();
    let (method, both_methods) = match operator {
        Operator::Add => {
            ensure(
                (2..=3).contains(&numbers.len()),
                "columnOperation: an addition of 2 or 3 numbers.",
            )?;
            ensure(
                i64::from(numbers.iter().sum::<u32>()) <= MAX_WRITTEN_NUMBER,
                "columnOperation: a sum up to 9 999.",
            )?;
            ensure(
                method.is_none(),
                "columnOperation: a method only for a subtraction.",
            )?;
            (None, false)
        }
        Operator::Subtract | Operator::Multiply => {
            ensure(
                numbers.len() == 2 && numbers[0] >= numbers[1],
                "columnOperation: a subtraction of 2 numbers, the larger on top.",
            )?;
            match method {
                Some("cassage") => (Some(SubtractionMethod::Cassage), false),
                Some("compensation") => (Some(SubtractionMethod::Compensation), false),
                Some("both") => (None, true),
                Some(other) => {
                    return Err(format!("columnOperation: unknown method '{other}'."));
                }
                None => {
                    return Err(
                        "columnOperation: a subtraction needs a method (cassage, compensation or both)."
                            .to_owned(),
                    );
                }
            }
        }
    };
    let columns = written_columns(
        operator,
        &numbers,
        method.or(Some(SubtractionMethod::Cassage)),
    );
    ensure(
        computed_columns.is_none_or(|c| (0..=columns.len() as i64).contains(&c)),
        "columnOperation: computedColumns between 0 and the number of columns.",
    )?;
    ensure(
        !interactive || computed_columns.is_none(),
        "columnOperation: an interactive operation is computed by the child (no computedColumns).",
    )?;
    Ok(Animation::ColumnOperation(ColumnOperation {
        operator,
        operands: numbers,
        method,
        both_methods,
        computed_columns: computed_columns.map(|c| c as usize),
        interactive,
    }))
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
