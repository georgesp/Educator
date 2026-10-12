//! Word problems (SUB000124): the committed templates and words, and the validation of a template.

use educator_content::{load_problems, EducatorContent};
use educator_core::exercises::{generate_series, Answer, Token, DEFAULT_LENGTH};
use educator_core::skills::{AnswerMode, SkillCatalog, SkillOperation};
use std::path::{Path, PathBuf};

fn content_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content")
}

#[test]
fn templates_and_words_load_without_problem() {
    let bank = load_problems(&content_dir().join("problems"));
    assert!(bank.problems.is_empty(), "{:?}", bank.problems);
    assert!(bank.value.templates.len() >= 25);
    let names = &bank.value.words.names;
    let feminine = names.iter().filter(|n| n.feminine).count();
    assert_eq!(names.len(), 2 * feminine, "as many girls as boys");
}

#[test]
fn problem_statements_are_complete_and_within_the_scale() {
    let content = EducatorContent::load(&content_dir());
    let skills: Vec<_> = content
        .skills
        .all()
        .iter()
        .filter(|s| s.operation == SkillOperation::Problem)
        .collect();
    assert_eq!(3, skills.len());
    for skill in skills {
        let set = skill.params.problems.as_ref().unwrap();
        for seed in 0..2000 {
            let series = generate_series(skill, seed, AnswerMode::Choice, DEFAULT_LENGTH).unwrap();
            for exercise in &series {
                let text = exercise.question.as_ref().unwrap();
                for written in [text, &exercise.explanation] {
                    assert!(
                        !written.contains('{') && !written.contains('}'),
                        "{written}"
                    );
                    assert!(
                        !written.contains(" de a") && !written.contains(" de i"),
                        "{written}"
                    );
                }
                assert_eq!(vec![Token::Slot(0)], exercise.statement);
                let Answer::Number(answer) = exercise.expected else {
                    panic!("number answer expected");
                };
                let scale = match set.start_max_number {
                    Some(start) if exercise.index < 6 => start,
                    _ => set.max_number,
                };
                assert!(
                    answer > 0 && i64::from(answer) <= scale,
                    "{} : {answer}",
                    skill.id
                );
                let numbers: Vec<i64> = text
                    .split(|c: char| !c.is_ascii_digit())
                    .filter(|w| !w.is_empty())
                    .map(|w| w.parse().unwrap())
                    .collect();
                assert!(numbers.iter().all(|n| *n <= scale), "{text}");
            }
        }
    }
}

#[test]
fn an_invalid_template_is_reported() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::copy(
        content_dir().join("problems/words.json"),
        dir.path().join("words.json"),
    )
    .unwrap();
    std::fs::write(
        dir.path().join("bad.json"),
        r#"{ "templates": [
            { "id": "ok", "kind": "partWhole", "text": "{name} a {a} {objects} et {b}.",
              "variables": { "a": { "min": 1 }, "b": { "min": 1 } }, "answer": "a + b", "solution": "{answer}" },
            { "id": "bad", "kind": "partWhole", "text": "{name} a {z} {answer}.",
              "variables": { "a": { "min": 1 } }, "answer": "a + c", "solution": "{answer}" }
        ] }"#,
    )
    .unwrap();
    let loaded = load_problems(dir.path());
    assert_eq!(1, loaded.value.templates.len());
    let messages: Vec<String> = loaded.problems.iter().map(|p| p.message.clone()).collect();
    assert!(
        messages.iter().any(|m| m.contains("Unknown variable 'c'")),
        "{messages:?}"
    );
    assert!(messages.iter().any(|m| m.contains("{z}")), "{messages:?}");
    assert!(
        messages
            .iter()
            .any(|m| m.contains("must not give the answer")),
        "{messages:?}"
    );
}
