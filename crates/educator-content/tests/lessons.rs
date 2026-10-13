//! Lessons (SUB001409): the committed lessons match `lesson.schema.json` and load without problem;
//! a badly written lesson is refused with a clear message, never shown to the child.

use educator_content::{
    load_skills, parse_lesson, EducatorContent, Skills, MAX_QUIZ, MAX_SLIDES,
    MIN_INTERACTIVE_SLIDES, MIN_QUIZ, MIN_SLIDES,
};
use educator_core::exercises::Answer;
use educator_core::lessons::{
    written_columns, Animation, ChainMode, EquationTerm, LessonCatalog, Operator, SubtractionMethod,
};
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

/// SUB000125 to SUB000139: the CE1 lessons of lot 1, at the period of their main skill (the first one);
/// the other skills come in later periods.
#[test]
fn every_ce1_skill_of_lot_1_has_its_lesson() {
    let lessons = &content().lessons;
    let expected = [
        ("ce1-add-2-digits", 1, vec!["ce1-add-2-digits"]),
        ("ce1-add-9-19-29", 2, vec!["ce1-add-9-19-29"]),
        (
            "ce1-add-sub-3-digits",
            5,
            vec!["ce1-add-3-digits", "ce1-sub-3-digits"],
        ),
        (
            "ce1-add-sub-tens-hundreds",
            4,
            vec!["ce1-add-sub-tens-hundreds"],
        ),
        ("ce1-add-tables", 1, vec!["ce1-add-tables"]),
        ("ce1-sub-2-digits", 3, vec!["ce1-sub-2-digits"]),
        ("ce1-sub-9", 2, vec!["ce1-sub-9"]),
        ("ce1-sub-small", 3, vec!["ce1-sub-small"]),
        (
            "ce1-comparer-ranger-1000",
            2,
            vec!["ce1-num-compare", "ce1-num-frame-line"],
        ),
        (
            "ce1-multiplication-et-tables",
            2,
            vec![
                "ce1-mul-meaning",
                "ce1-mul-tables-2-5-10",
                "ce1-mul-tables-3-4",
                "ce1-mul-tables-6-9",
            ],
        ),
        (
            "ce1-multiplier-en-decomposant",
            5,
            vec!["ce1-mul-distributive"],
        ),
        (
            "ce1-nombres-jusqu-a-1000",
            1,
            vec!["ce1-num-hundreds", "ce1-num-read-write"],
        ),
        (
            "ce1-pair-impair-doubles-moities",
            2,
            vec!["ce1-parity", "ce1-double-half-facts", "ce1-half-even"],
        ),
        (
            "ce1-premieres-fractions",
            2,
            vec![
                "ce1-frac-unit",
                "ce1-frac-read-write",
                "ce1-frac-compare-add",
            ],
        ),
        (
            "ce1-resoudre-des-problemes",
            1,
            vec![
                "ce1-problems-additive",
                "ce1-problems-two-steps",
                "ce1-problems-multiplicative",
            ],
        ),
    ];
    for (id, period, skill_ids) in expected {
        let lesson = lessons.find(id).unwrap_or_else(|| panic!("no lesson {id}"));
        assert_eq!((Grade::CE1, period), (lesson.grade, lesson.period), "{id}");
        assert_eq!(skill_ids, lesson.skill_ids, "{id}");
        let periods: Vec<u8> = lesson
            .skill_ids
            .iter()
            .map(|skill| content().skills.find(skill).map_or(0, |s| s.period))
            .collect();
        assert_eq!(lesson.period, periods[0], "{id}: period of its main skill");
        assert!(
            periods.iter().all(|p| *p >= lesson.period),
            "{id}: no skill before the lesson"
        );
    }
    assert_eq!(
        15,
        lessons.for_grade(Subject::Math, Grade::CE1).len(),
        "every CE1 lesson is listed above"
    );
}

/// SUB000117: every animation of the catalogue (but `proportionTable`, for the CM) is used by a lesson.
#[test]
fn the_lessons_use_every_animation_of_the_catalogue() {
    let used: std::collections::BTreeSet<&str> = content()
        .lessons
        .all()
        .iter()
        .flat_map(|l| l.slides.iter().map(|s| s.animation.type_name()))
        .collect();
    let catalogue = [
        "arrayGrid",
        "barModel",
        "calcChain",
        "columnOperation",
        "compareNumbers",
        "equation",
        "fractionBar",
        "groupObjects",
        "letterTiles",
        "numberLine",
        "placeValueTable",
        "recapCard",
        "sequencePattern",
        "soundSort",
        "tenFrame",
        "tensUnits",
        "timesTable",
        "titleCard",
    ];
    assert_eq!(
        catalogue
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>(),
        used
    );
}

/// SUB000128: 608, the 0 keeps the place of the tens; 347 has 34 tens; the quiz accepts 608 only.
#[test]
fn the_ce1_lesson_on_numbers_up_to_1000_uses_the_table_of_numeration() {
    let lesson = content().lessons.find("ce1-nombres-jusqu-a-1000").unwrap();
    let tables: Vec<&educator_core::lessons::PlaceValueTable> = lesson
        .slides
        .iter()
        .filter_map(|s| match &s.animation {
            Animation::PlaceValueTable(t) => Some(t),
            _ => None,
        })
        .collect();
    assert!(tables
        .iter()
        .any(|t| t.value == 608 && t.digits() == vec![Some(6), Some(0), Some(8)]));
    assert!(lesson.interactive_slides() >= 2);
}

/// SUB000125, SUB000130: the quiz may expect a sign or a fraction, any equal fraction being right.
#[test]
fn quiz_answers_can_be_signs_fractions_or_yes_no() {
    use educator_core::exercises::{Comparison, Fraction};
    let compare = content().lessons.find("ce1-comparer-ranger-1000").unwrap();
    assert_eq!(Answer::Comparison(Comparison::Less), compare.quiz[0].answer);
    assert!(
        compare
            .check_quiz(0, &Answer::Comparison(Comparison::Less))
            .unwrap()
            .correct
    );

    let fractions = content().lessons.find("ce1-premieres-fractions").unwrap();
    let (index, _) = fractions
        .quiz
        .iter()
        .enumerate()
        .find(|(_, q)| q.answer == Answer::Fraction(Fraction::new(3, 4)))
        .expect("a question expects 3/4");
    assert!(
        fractions
            .check_quiz(index, &Answer::Fraction(Fraction::new(6, 8)))
            .unwrap()
            .correct
    );

    let tables = content()
        .lessons
        .find("ce1-multiplication-et-tables")
        .unwrap();
    assert!(tables
        .quiz
        .iter()
        .any(|q| q.answer == Answer::YesNo(true) && q.choices.len() == 2));
}

/// No carry anywhere in a written operation (SUB001412: « avec et sans retenue »).
fn without_carry(operation: &Animation) -> bool {
    let Animation::ColumnOperation(o) = operation else {
        panic!("{operation:?}");
    };
    written_columns(o.operator, &o.operands, o.method)
        .iter()
        .all(|c| c.carry == 0 && c.rewrites.is_empty() && !c.plus_ten)
}

/// SUB000132: 34 + 25 without carry, then 56 + 27 set in columns, computed column by column; 34 + 48
/// and 28 + 8 + 56 by the child.
#[test]
fn the_ce1_lesson_on_written_additions_computes_column_by_column() {
    let lesson = content().lessons.find("ce1-add-2-digits").unwrap();
    assert!(without_carry(&lesson.slides[3].animation));
    assert!(!without_carry(&lesson.slides[4].animation));
    let computed: Vec<Option<usize>> = lesson.slides[4..7]
        .iter()
        .map(|s| match &s.animation {
            Animation::ColumnOperation(o) => o.computed_columns,
            other => panic!("{other:?}"),
        })
        .collect();
    assert_eq!(vec![Some(0), Some(1), None], computed);

    let Animation::ColumnOperation(own) = &lesson.slides[9].animation else {
        panic!("slide 10 is a written operation");
    };
    assert!(own.interactive);
    assert_eq!(vec![28, 8, 56], own.operands);
    assert_eq!(92, own.result());
    assert_eq!(2, lesson.interactive_slides());
}

/// SUB000137: 68 − 25 without carry, then 74 − 28 with the two techniques, then 63 − 27 by the child
/// and the checking addition.
#[test]
fn the_ce1_lesson_on_written_subtractions_shows_both_techniques() {
    let lesson = content().lessons.find("ce1-sub-2-digits").unwrap();
    assert_eq!(12, lesson.slides.len());
    assert!(without_carry(&lesson.slides[3].animation));
    let methods: Vec<(Option<SubtractionMethod>, bool)> = lesson.slides[5..8]
        .iter()
        .map(|s| match &s.animation {
            Animation::ColumnOperation(o) => (o.method, o.both_methods),
            other => panic!("{other:?}"),
        })
        .collect();
    assert_eq!(
        vec![
            (Some(SubtractionMethod::Cassage), false),
            (Some(SubtractionMethod::Compensation), false),
            (None, true),
        ],
        methods
    );
    for method in [SubtractionMethod::Cassage, SubtractionMethod::Compensation] {
        let digits: Vec<u8> = written_columns(Operator::Subtract, &[74, 28], Some(method))
            .iter()
            .map(|c| c.digit)
            .collect();
        assert_eq!(vec![6, 4], digits);
    }

    // « À toi » : 63 − 27 by the child, the two techniques side by side.
    let Animation::ColumnOperation(own) = &lesson.slides[8].animation else {
        panic!("slide 9 is a written operation");
    };
    assert!(own.interactive && own.both_methods);

    let Animation::Equation(check) = &lesson.slides[9].animation else {
        panic!("slide 10 is an equation");
    };
    assert_eq!(
        (Some(EquationTerm::Result), Some(74)),
        (check.missing, check.missing_value())
    );
    assert!(check.interactive && check.family);
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

fn with_slide(animation: Value) -> Value {
    let mut lesson = valid();
    lesson["slides"][2]["animation"] = animation;
    lesson
}

#[test]
fn a_written_subtraction_needs_a_method_and_the_larger_number_on_top() {
    assert_refused(
        with_slide(
            json!({ "type": "columnOperation", "operation": "subtract", "operands": [74, 28] }),
        ),
        "needs a method",
    );
    assert_refused(
        with_slide(
            json!({ "type": "columnOperation", "operation": "subtract", "operands": [28, 74], "method": "cassage" }),
        ),
        "the larger on top",
    );
    assert_refused(
        with_slide(
            json!({ "type": "columnOperation", "operation": "add", "operands": [56, 27], "method": "cassage" }),
        ),
        "a method only for a subtraction",
    );
    assert!(errors_of(&with_slide(
        json!({ "type": "columnOperation", "operation": "subtract", "operands": [74, 28], "method": "both" })
    ))
    .is_empty());
}

#[test]
fn a_written_operation_cannot_compute_more_columns_than_it_has() {
    assert_refused(
        with_slide(
            json!({ "type": "columnOperation", "operation": "add", "operands": [56, 27], "computedColumns": 3 }),
        ),
        "computedColumns",
    );
    assert_refused(
        with_slide(
            json!({ "type": "columnOperation", "operation": "add", "operands": [56, 27], "computedColumns": 1, "interactive": true }),
        ),
        "computed by the child",
    );
}

#[test]
fn an_interactive_equation_needs_its_box_among_the_choices() {
    assert_refused(
        with_slide(
            json!({ "type": "equation", "left": 7, "operation": "add", "right": 8, "missing": "right", "interactive": true, "choices": [6, 7, 9] }),
        ),
        "the missing term must be among the choices",
    );
    assert_refused(
        with_slide(
            json!({ "type": "equation", "left": 7, "operation": "add", "right": 8, "interactive": true, "choices": [6, 7, 8] }),
        ),
        "needs a missing term",
    );
    assert_refused(
        with_slide(json!({ "type": "equation", "left": 7, "operation": "subtract", "right": 8 })),
        "between 0 and 1 000 000",
    );
}

#[test]
fn a_quiz_answer_must_be_a_number_a_sign_yes_no_or_a_fraction() {
    let mut lesson = valid();
    lesson["quiz"][0]["answer"] = json!("vingt");
    assert_refused(lesson, "answer 'vingt' must be");

    let mut lesson = valid();
    lesson["quiz"][1]["answer"] = json!("<");
    lesson["quiz"][1]["choices"] = json!(["<", 2, "="]);
    assert_refused(lesson, "the same kind as the answer");

    let mut lesson = valid();
    lesson["quiz"][1]["answer"] = json!("1/2");
    lesson["quiz"][1]["choices"] = json!(["1/2", "2/4", "1/3"]);
    assert_refused(lesson, "exactly one choice must be right");

    let mut lesson = valid();
    lesson["quiz"][1]["answer"] = json!("oui");
    lesson["quiz"][1]["choices"] = json!(["oui", "non"]);
    assert!(errors_of(&lesson).is_empty(), "{:?}", errors_of(&lesson));

    let mut lesson = valid();
    lesson["quiz"][0]["answer"] = json!(">");
    lesson["quiz"][0].as_object_mut().unwrap().remove("choices");
    assert_refused(lesson, "only a number or a word can be typed");
}

#[test]
fn a_pick_chain_has_exactly_one_right_path() {
    let chains = |crossed: [bool; 3]| {
        json!({ "type": "calcChain", "mode": "pick", "chains": [
            { "start": 42, "steps": [10, 1], "crossed": crossed[0] },
            { "start": 42, "steps": [10, -1], "crossed": crossed[1] },
            { "start": 42, "steps": [9, -1], "crossed": crossed[2] }
        ] })
    };
    assert!(errors_of(&with_slide(chains([true, false, true]))).is_empty());
    assert_refused(
        with_slide(chains([false, false, true])),
        "only one of them right",
    );
    assert_refused(
        with_slide(chains([true, true, true])),
        "only one of them right",
    );
}

#[test]
fn number_line_flags_stand_on_graduations() {
    let line = |flags: Value| json!({ "type": "numberLine", "from": 0, "to": 100, "step": 10, "start": 30, "flags": flags });
    assert!(errors_of(&with_slide(line(json!([30, 40])))).is_empty());
    assert_refused(with_slide(line(json!([35]))), "each on a graduation");
}

#[test]
fn the_new_animations_refuse_inconsistent_parameters() {
    let cases = [
        (
            json!({ "type": "tensUnits", "value": 47, "add": 28, "remove": 3 }),
            "not two of them",
        ),
        (
            json!({ "type": "tensUnits", "value": 47, "share": 2 }),
            "the value divides",
        ),
        (
            json!({ "type": "sequencePattern", "values": [3, 5, 8] }),
            "not be already in order",
        ),
        (
            json!({ "type": "arrayGrid", "rows": 3, "cols": 5, "split": 3 }),
            "split between 1 and rows",
        ),
        (
            json!({ "type": "timesTable", "operation": "multiply", "size": 10, "highlights": [{ "kind": "result", "value": 10 }] }),
            "a result of the addition table",
        ),
        (
            json!({ "type": "placeValueTable", "value": 47, "countOf": "hundreds" }),
            "countOf a rank",
        ),
        (
            json!({ "type": "groupObjects", "mode": "join", "counts": [15, 8] }),
            "20 objects at most",
        ),
        (
            json!({ "type": "groupObjects", "mode": "sortParity", "numbers": [2, 4, 6] }),
            "even and odd numbers",
        ),
        (
            json!({ "type": "tenFrame", "filled": 7, "added": 5 }),
            "1 car to all the places",
        ),
        (
            json!({ "type": "fractionBar", "bars": [{ "denominator": 4, "numerators": [3, 2] }] }),
            "fit in the bar",
        ),
        (
            json!({ "type": "barModel", "kind": "groups", "groups": 4, "size": 6, "unknown": "part1" }),
            "unknown 'part1'",
        ),
        (
            json!({ "type": "equation", "left": 3, "operation": "multiply", "right": 7, "family": true, "missing": "left", "interactive": true, "choices": [2, 3, 21] }),
            "",
        ),
        (
            json!({ "type": "columnOperation", "operation": "multiply", "operands": [23, 4] }),
            "the written multiplication comes in CE2",
        ),
        (
            json!({ "type": "compareNumbers", "a": 52000, "b": 42000 }),
            "between 0 and 9 999",
        ),
        (
            json!({ "type": "placeValueTable", "value": 347, "words": true, "interactive": true }),
            "an interactive table has no countOf",
        ),
        (
            json!({ "type": "arrayGrid", "rows": 13, "cols": 7, "split": 10, "showSum": true }),
            "a split grid is not summed",
        ),
        (
            json!({ "type": "groupObjects", "mode": "join", "counts": [2, 6], "commute": true, "interactive": true }),
            "commute only on a join shown by itself",
        ),
        (
            json!({ "type": "barModel", "kind": "parts", "parts": [28, 15], "unknown": "whole", "labels": ["Tom"] }),
            "labels name the two bars of a comparison only",
        ),
    ];
    for (animation, expected) in cases {
        let errors = errors_of(&with_slide(animation.clone()));
        if expected.is_empty() {
            assert!(errors.is_empty(), "{animation}: {errors:?}");
        } else {
            assert!(
                errors.iter().any(|e| e.contains(expected)),
                "{animation}: expected « {expected} » in {errors:?}"
            );
        }
    }
}

// ── Spelling lessons (SUB001410, SUB001413) ──────────────────────────────────────────────────

/// One lesson per spelling skill, each with at least one `letterTiles` or `soundSort`.
#[test]
fn every_spelling_skill_has_its_lesson_with_a_spelling_animation() {
    let lessons = &content().lessons;
    for skill in content()
        .skills
        .all()
        .iter()
        .filter(|s| s.subject == Subject::Spelling)
    {
        let lesson = lessons
            .all()
            .iter()
            .find(|l| l.skill_ids.first() == Some(&skill.id))
            .unwrap_or_else(|| panic!("no lesson for {}", skill.id));
        assert_eq!(Subject::Spelling, lesson.subject);
        assert!(
            lesson.slides.iter().any(|s| matches!(
                s.animation,
                Animation::LetterTiles(_) | Animation::SoundSort(_)
            )),
            "{}",
            lesson.id
        );
    }
}

#[test]
fn letter_tiles_and_sound_sort_are_checked() {
    let tiles = |word: &str, tiles: Value, distractors: Value| json!({ "type": "letterTiles", "word": word, "tiles": tiles, "distractors": distractors });
    let with_slide = |animation: Value| {
        let mut lesson = valid();
        lesson["slides"][1]["animation"] = animation;
        lesson
    };
    assert_refused(
        with_slide(tiles("bateau", json!(["b", "a", "t", "o"]), json!([]))),
        "the tiles, in order, must make the word",
    );
    assert_refused(
        with_slide(tiles(
            "bateau",
            json!(["b", "a", "t", "eau"]),
            json!(["eau"]),
        )),
        "none equal to a tile",
    );
    let sort = |words: Value| json!({ "type": "soundSort", "groups": ["o", "au"], "words": words });
    assert_refused(
        with_slide(sort(json!([
            { "word": "vélo", "group": "o" },
            { "word": "moto", "group": "o" },
            { "word": "jaune", "group": "au" },
            { "word": "domino", "group": "au" }
        ]))),
        "soundSort: 'au' is not in",
    );
    assert_refused(
        with_slide(sort(json!([
            { "word": "vélo", "group": "o" },
            { "word": "moto", "group": "o" },
            { "word": "robe", "group": "o" },
            { "word": "domino", "group": "o" }
        ]))),
        "every group gets at least one word",
    );
}

#[test]
fn a_spelling_quiz_expects_words_without_tolerance_on_accents() {
    let lesson = content().lessons.find("ce1-son-o-au-eau").unwrap();
    let typed = lesson
        .quiz
        .iter()
        .position(|q| q.choices.is_empty())
        .unwrap();
    assert_eq!(
        Answer::Spelling("vélo".to_owned()),
        lesson.quiz[typed].answer
    );
    assert!(
        lesson
            .check_quiz(typed, &Answer::Text("Vélo".to_owned()))
            .unwrap()
            .correct
    );
    assert!(
        !lesson
            .check_quiz(typed, &Answer::Text("velo".to_owned()))
            .unwrap()
            .correct
    );
    let Animation::SoundSort(sort) = &lesson.slides[6].animation else {
        panic!("slide 7 sorts words");
    };
    assert_eq!(vec!["o", "au", "eau"], sort.groups);
    let gateau = sort.words.iter().find(|w| w.word == "gâteau").unwrap();
    assert_eq!((2, 3, 3), (gateau.group, gateau.start, gateau.length));
}

#[test]
fn a_spelling_quiz_showing_the_word_to_write_is_refused() {
    let mut lesson = read_json(&content_dir().join("lessons/spelling/ce1-son-o-au-eau.json"));
    let typed = lesson["quiz"]
        .as_array()
        .unwrap()
        .iter()
        .position(|q| q.get("choices").is_none())
        .unwrap();
    lesson["quiz"][typed]["question"] = json!("Écris le mot « vélo ».");
    let errors = parse_lesson(&lesson.to_string(), Some("ce1-son-o-au-eau"), skills())
        .err()
        .unwrap_or_default();
    assert!(
        errors.iter().any(|e| e.contains("must not be shown")),
        "{errors:?}"
    );
}

/// SUB001410: the child makes packets of 10 only; a grouping by 2 or of too many objects is shown
/// only.
#[test]
fn an_interactive_grouping_makes_packets_of_ten() {
    let grouping = |counts: Value, size: i64| {
        let mut lesson = valid();
        lesson["slides"][1]["animation"] = json!({
            "type": "groupObjects", "mode": "groupBy", "counts": counts, "size": size, "interactive": true
        });
        errors_of(&lesson)
    };
    let refused = |errors: Vec<String>| errors.iter().any(|e| e.contains("groupObjects groupBy"));

    assert!(!refused(grouping(json!([23]), 10)));
    assert!(!refused(grouping(json!([17]), 10)));
    assert!(refused(grouping(json!([8]), 2)));
    assert!(refused(grouping(json!([45]), 10)));
    assert!(refused(grouping(json!([23, 12]), 10)));
}

/// SUB001412: the complements to 100, through the next ten (65 → 70 → 100).
#[test]
fn the_ce1_lesson_on_tens_and_hundreds_goes_up_to_one_hundred() {
    let lesson = content().lessons.find("ce1-add-sub-tens-hundreds").unwrap();
    let to_100 = lesson.slides.iter().find_map(|s| match &s.animation {
        Animation::NumberLine { start, jumps, .. }
            if educator_core::lessons::landings(*start, jumps).last() == Some(&100) =>
        {
            Some((*start, jumps.clone()))
        }
        _ => None,
    });
    assert_eq!(Some((65, vec![5, 30])), to_100);
}
