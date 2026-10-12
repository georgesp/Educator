//! Lessons (SUB001409): the committed lessons match `lesson.schema.json` and load without problem;
//! a badly written lesson is refused with a clear message, never shown to the child.

use educator_content::{
    load_skills, parse_lesson, EducatorContent, Skills, MAX_QUIZ, MAX_SLIDES,
    MIN_INTERACTIVE_SLIDES, MIN_QUIZ, MIN_SLIDES,
};
use educator_core::exercises::Answer;
use educator_core::lessons::{Animation, ChainMode, LessonCatalog};
use educator_core::skills::{SkillCatalog, Subject};
use educator_core::Grade;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

fn content_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content")
}

fn content() -> &'static EducatorContent {
    static CONTENT: OnceLock<EducatorContent> = OnceLock::new();
    CONTENT.get_or_init(|| EducatorContent::load(&content_dir()))
}

fn skills() -> &'static Skills {
    static SKILLS: OnceLock<Skills> = OnceLock::new();
    SKILLS.get_or_init(|| Skills::new(load_skills(&content_dir().join("skills")).value))
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&std::fs::read_to_string(path).unwrap())
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn lesson_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    for subject in std::fs::read_dir(content_dir().join("lessons")).unwrap() {
        for file in std::fs::read_dir(subject.unwrap().path()).unwrap() {
            let path = file.unwrap().path();
            if path.extension().is_some_and(|e| e == "json") {
                files.push(path);
            }
        }
    }
    files
}

#[test]
fn every_lesson_matches_the_json_schema() {
    let schema = read_json(&content_dir().join("schemas/lesson.schema.json"));
    let validator = jsonschema::draft202012::new(&schema).unwrap();
    let files = lesson_files();
    assert!(!files.is_empty());
    for path in files {
        let errors: Vec<String> = validator
            .iter_errors(&read_json(&path))
            .map(|e| format!("{}: {e}", e.instance_path))
            .collect();
        assert!(
            errors.is_empty(),
            "{}:\n{}",
            path.display(),
            errors.join("\n")
        );
    }
}

#[test]
fn every_lesson_file_is_loaded() {
    assert!(
        content()
            .problems
            .iter()
            .all(|p| !p.file.starts_with("lessons/")),
        "{:?}",
        content().problems
    );
    assert_eq!(lesson_files().len(), content().lessons.all().len());
}

#[test]
fn every_lesson_follows_the_writing_rules() {
    for lesson in content().lessons.all() {
        assert!((MIN_SLIDES..=MAX_SLIDES).contains(&lesson.slides.len()));
        assert!((MIN_QUIZ..=MAX_QUIZ).contains(&lesson.quiz.len()));
        assert!(lesson.interactive_slides() >= MIN_INTERACTIVE_SLIDES);
        assert!((3..=5).contains(&lesson.duration_minutes), "{}", lesson.id);
        for id in &lesson.skill_ids {
            assert_eq!(
                Some(lesson.grade),
                content().skills.find(id).map(|s| s.grade)
            );
        }
        for (index, question) in lesson.quiz.iter().enumerate() {
            let outcome = lesson.check_quiz(index, &question.answer).unwrap();
            assert!(outcome.correct, "{} quiz {index}", lesson.id);
        }
    }
}

/// SUB000133: « Ajouter 9, 19 ou 29 », CE1 period 2, linked to the skill of lot 1.
#[test]
fn the_ce1_lesson_on_adding_9_19_29_is_offered_in_period_two() {
    let lessons = &content().lessons;
    let lesson = lessons.find("ce1-add-9-19-29").unwrap();
    assert_eq!((Grade::CE1, 2), (lesson.grade, lesson.period));
    assert_eq!(vec!["ce1-add-9-19-29".to_owned()], lesson.skill_ids);
    assert_eq!(10, lesson.slides.len());
    assert_eq!(4, lesson.quiz.len());
    assert!(lessons
        .for_grade(Subject::Math, Grade::CE1)
        .iter()
        .any(|l| l.id == lesson.id));

    // « À toi » : 57 + 19 in two jumps lands on 77 then 76.
    let Animation::NumberLine {
        start,
        jumps,
        interactive,
        ..
    } = &lesson.slides[5].animation
    else {
        panic!("slide 6 is a number line");
    };
    assert!(*interactive);
    assert_eq!(
        vec![77, 76],
        educator_core::lessons::landings(*start, jumps)
    );

    // « Relie chaque calcul à son chemin » : 24 + 9, 24 + 19, 24 + 29.
    let Animation::CalcChain { chains, mode } = &lesson.slides[7].animation else {
        panic!("slide 8 is a chain");
    };
    assert_eq!(ChainMode::Match, *mode);
    let totals: Vec<i32> = chains.iter().map(|c| c.total()).collect();
    assert_eq!(vec![9, 19, 29], totals);

    // Quiz: 46 + 19 is a QCM 64 / 65 / 66.
    assert_eq!(
        vec![Answer::Number(64), Answer::Number(65), Answer::Number(66)],
        lesson.quiz[1].choices
    );
    assert!(!lesson.check_quiz(1, &Answer::Number(66)).unwrap().correct);
}

// ── Refused lessons ──────────────────────────────────────────────────────────────────────────

fn valid() -> Value {
    read_json(&content_dir().join("lessons/math/ce1-add-9-19-29.json"))
}

fn errors_of(lesson: &Value) -> Vec<String> {
    parse_lesson(&lesson.to_string(), Some("ce1-add-9-19-29"), skills())
        .err()
        .unwrap_or_default()
}

fn assert_refused(lesson: Value, expected: &str) {
    let errors = errors_of(&lesson);
    assert!(
        errors.iter().any(|e| e.contains(expected)),
        "expected « {expected} » in {errors:?}"
    );
}

#[test]
fn the_valid_lesson_has_no_error() {
    assert!(errors_of(&valid()).is_empty(), "{:?}", errors_of(&valid()));
}

#[test]
fn an_unknown_skill_is_refused() {
    let mut lesson = valid();
    lesson["skillIds"] = json!(["ce1-nothing"]);
    assert_refused(lesson, "Unknown skill 'ce1-nothing'");
}

#[test]
fn a_skill_of_another_grade_is_refused() {
    let mut lesson = valid();
    lesson["skillIds"] = json!(["cp-add-9"]);
    assert_refused(lesson, "is a CP skill");
}

#[test]
fn a_jump_off_the_line_is_refused() {
    let mut lesson = valid();
    lesson["slides"][5]["animation"]["jumps"] = json!([30, -1]);
    assert_refused(lesson, "every jump must land on a graduation");
}

#[test]
fn a_speech_too_long_for_the_voice_is_refused() {
    let mut lesson = valid();
    lesson["slides"][1]["speech"] = json!("123 + 456 = 579. ".repeat(10));
    assert_refused(lesson, "slide 2: speech");
}

#[test]
fn a_lesson_must_start_with_a_title_and_end_with_je_retiens() {
    let mut lesson = valid();
    lesson["slides"].as_array_mut().unwrap().swap(0, 9);
    let errors = errors_of(&lesson);
    assert!(errors.iter().any(|e| e.contains("first slide")));
    assert!(errors.iter().any(|e| e.contains("last slide")));
}

#[test]
fn a_lesson_without_manipulation_is_refused() {
    let mut lesson = valid();
    lesson["slides"][5]["animation"]["interactive"] = json!(false);
    assert_refused(lesson, "interactive slide(s)");
}

#[test]
fn a_qcm_without_its_answer_is_refused() {
    let mut lesson = valid();
    lesson["quiz"][1]["choices"] = json!([64, 66, 67]);
    assert_refused(lesson, "the answer must be among the choices");
}

#[test]
fn an_unknown_animation_is_refused() {
    let mut lesson = valid();
    lesson["slides"][2]["animation"] = json!({ "type": "rocket", "value": 3 });
    assert_refused(lesson, "Invalid JSON");
}

#[test]
fn a_matching_with_two_identical_chains_is_refused() {
    let mut lesson = valid();
    lesson["slides"][7]["animation"]["chains"][1] = json!({ "start": 24, "steps": [10, -1] });
    assert_refused(lesson, "each chain adds a different positive number");
}

#[test]
fn the_id_must_match_the_file_name() {
    let mut lesson = valid();
    lesson["id"] = json!("other-lesson");
    assert_refused(lesson, "must match the file name");
}

#[test]
fn an_unknown_animation_parameter_is_refused() {
    let mut lesson = valid();
    lesson["slides"][3]["animation"]["colour"] = json!("red");
    assert_refused(lesson, "Invalid JSON");
}

/// The value under the pointer is shown unless the lesson turns it off (finding a number on the
/// line is then what the child learns).
#[test]
fn hover_values_are_on_unless_the_lesson_turns_them_off() {
    let hover = |lesson: &Value| {
        let parsed = parse_lesson(&lesson.to_string(), Some("ce1-add-9-19-29"), skills()).unwrap();
        match &parsed.slides[5].animation {
            Animation::NumberLine { hover_values, .. } => *hover_values,
            other => panic!("{other:?}"),
        }
    };

    let mut lesson = valid();
    assert!(hover(&lesson));
    lesson["slides"][5]["animation"]
        .as_object_mut()
        .unwrap()
        .remove("hoverValues");
    assert!(hover(&lesson), "on by default");
    lesson["slides"][5]["animation"]["hoverValues"] = json!(false);
    assert!(!hover(&lesson));
}
