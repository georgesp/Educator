use educator_content::{load_skills, parse_skill};
use educator_core::skills::{
    AnswerMode, CarryRule, ExerciseFormat, OperandRule, Placement, TrivialCase,
};
use educator_core::Grade;

const VALID: &str = r#"
{
  "id": "test-add",
  "subject": "math",
  "grade": "CE1",
  "period": 2,
  "title": "Additions",
  "instruction": "Calcule.",
  "operation": "addition",
  "operands": [ { "min": 0, "max": 10 }, { "min": 0, "max": 10, "multipleOf": 2 } ],
  "constraints": { "maxResult": 20, "carry": "forbidden", "excludeTrivial": ["plusZero"] },
  "formats": [ "result", "missingOperand" ],
  "answerModes": [ "keyboard" ],
  "source": "Tests"
}
"#;

#[test]
fn valid_file_gives_the_skill() {
    let skill = parse_skill(VALID, Some("test-add")).unwrap();

    assert_eq!(Grade::CE1, skill.grade);
    assert_eq!(2, skill.period);
    assert_eq!(OperandRule::multiples(0, 10, 2), skill.right);
    assert_eq!(CarryRule::Forbidden, skill.constraints.carry);
    assert_eq!(
        vec![TrivialCase::PlusZero],
        skill.constraints.exclude_trivial
    );
    assert_eq!(
        vec![ExerciseFormat::Result, ExerciseFormat::MissingOperand],
        skill.formats
    );
    assert_eq!(vec![AnswerMode::Keyboard], skill.answer_modes);
}

#[test]
fn also_offered_is_loaded() {
    let json = VALID.replace(
        r#""source": "Tests""#,
        r#""source": "Tests", "alsoOffered": [ { "grade": "CE2", "period": 2 } ]"#,
    );
    let skill = parse_skill(&json, Some("test-add")).unwrap();

    assert_eq!(vec![Placement::new(Grade::CE2, 2)], skill.also_offered);
    assert!(parse_skill(VALID, Some("test-add"))
        .unwrap()
        .also_offered
        .is_empty());
}

#[test]
fn invalid_file_is_reported() {
    let cases = [
        (
            r#""operation": "addition""#,
            r#""operation": "division""#,
            "operation 'division' is unknown",
        ),
        (
            r#""grade": "CE1""#,
            r#""grade": "ce1""#,
            "grade 'ce1' is unknown",
        ),
        (r#""period": 2"#, r#""period": 6"#, "period 6"),
        (
            r#""source": "Tests""#,
            r#""source": "Tests", "theme": "agreement""#,
            "theme is not used by 'addition'",
        ),
        (
            r#""source": "Tests""#,
            r#""source": "Tests", "level": 3"#,
            "Invalid JSON",
        ),
        (
            r#""source": "Tests""#,
            r#""source": """#,
            "source is required",
        ),
        (
            r#"{ "min": 0, "max": 10 },"#,
            r#"{ "min": 12, "max": 10 },"#,
            "need 0 <= min <= max",
        ),
        (r#""maxResult": 20"#, r#""maxResult": 3"#, "Rule too narrow"),
        (
            r#""answerModes": [ "keyboard" ]"#,
            r#""answerModes": []"#,
            "answerModes must not be empty",
        ),
        (
            r#"[ { "min": 0, "max": 10 }, { "min": 0, "max": 10, "multipleOf": 2 } ]"#,
            r#"[ { "min": 0, "max": 99999 }, { "min": 0, "max": 99999 } ]"#,
            "Rule too wide",
        ),
        (
            r#""source": "Tests""#,
            r#""source": "Tests", "alsoOffered": [ { "grade": "CE3", "period": 1 } ]"#,
            "alsoOffered.grade 'CE3' is unknown",
        ),
        (
            r#""source": "Tests""#,
            r#""source": "Tests", "alsoOffered": [ { "grade": "CE2", "period": 0 } ]"#,
            "alsoOffered: period 0",
        ),
        (
            r#""source": "Tests""#,
            r#""source": "Tests", "alsoOffered": [ { "grade": "CE1", "period": 4 } ]"#,
            "own grade",
        ),
        (
            r#""source": "Tests""#,
            r#""source": "Tests", "alsoOffered": [ { "grade": "CE2", "period": 1 }, { "grade": "CE2", "period": 2 } ]"#,
            "listed twice",
        ),
    ];
    for (from, to, expected) in cases {
        let json = VALID.replace(from, to);
        assert_ne!(VALID, json, "replacement of {from}");
        let errors = parse_skill(&json, Some("test-add")).expect_err(expected);
        assert!(
            errors.iter().any(|e| e.contains(expected)),
            "{expected}: {errors:?}"
        );
    }
}

#[test]
fn id_different_from_the_file_name_is_reported() {
    let errors = parse_skill(VALID, Some("other-name")).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.contains("must match the file name")));
}

#[test]
fn load_directory_skips_invalid_files_and_reports_them() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("test-add.json"), VALID).unwrap();
    std::fs::write(dir.path().join("broken.json"), "{ not json").unwrap();

    let loaded = load_skills(dir.path());

    assert_eq!(
        vec!["test-add"],
        loaded
            .value
            .iter()
            .map(|s| s.id.as_str())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec!["broken.json"],
        loaded
            .problems
            .iter()
            .map(|p| p.file.as_str())
            .collect::<Vec<_>>()
    );
}
