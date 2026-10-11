//! The committed content must be valid: a broken rule fails these tests (and the CI), never the
//! application in front of a child (SUB000106, SUB000107, SUB000108).

use educator_content::{EducatorContent, MIN_DISTINCT_EXERCISES};
use educator_core::exercises::{
    generate_series, ArithmeticOperation, CalculationSpace, DEFAULT_LENGTH,
};
use educator_core::points::PointsRules;
use educator_core::skills::{AnswerMode, Date, SkillCatalog, Subject};
use educator_core::Grade;
use std::collections::{BTreeSet, HashSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// The real `content/` folder of the repository: the tests check what is committed.
fn content_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content")
}

fn content() -> &'static EducatorContent {
    static CONTENT: OnceLock<EducatorContent> = OnceLock::new();
    CONTENT.get_or_init(|| EducatorContent::load(&content_dir()))
}

#[test]
fn content_has_no_problem() {
    let problems = &content().problems;
    assert!(
        problems.is_empty(),
        "{}",
        problems
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn every_file_matches_its_json_schema() {
    let cases = [
        ("skills/math", None, "skill.schema.json"),
        ("calendar", Some("periods.json"), "calendar.schema.json"),
        ("rewards", Some("points.json"), "points.schema.json"),
        (
            "characters",
            Some("characters.json"),
            "characters.schema.json",
        ),
        ("characters", Some("palette.json"), "palette.schema.json"),
    ];
    for (folder, file, schema_name) in cases {
        let schema: serde_json::Value = read_json(&content_dir().join("schemas").join(schema_name));
        let validator =
            jsonschema::draft202012::new(&schema).unwrap_or_else(|e| panic!("{schema_name}: {e}"));
        let files: Vec<PathBuf> = match file {
            Some(file) => vec![content_dir().join(folder).join(file)],
            None => std::fs::read_dir(content_dir().join(folder))
                .unwrap()
                .map(|e| e.unwrap().path())
                .filter(|p| p.extension().is_some_and(|e| e == "json"))
                .collect(),
        };

        assert!(!files.is_empty(), "{folder}");
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
}

#[test]
fn cp_and_ce1_have_skills_in_every_period() {
    for grade in [Grade::CP, Grade::CE1] {
        let periods: BTreeSet<u8> = content()
            .skills
            .for_grade(Subject::Math, grade)
            .iter()
            .map(|s| s.period)
            .collect();
        assert_eq!(
            vec![1, 2, 3, 4, 5],
            periods.into_iter().collect::<Vec<_>>(),
            "{grade}"
        );
    }
}

#[test]
fn every_skill_allows_at_least_twenty_different_exercises() {
    for skill in content().skills.all() {
        assert!(
            CalculationSpace::of(skill).distinct_count() >= MIN_DISTINCT_EXERCISES,
            "{}",
            skill.id
        );
    }
}

/// Property test: 10 000 exercises per skill (1 000 series) respect every rule.
#[test]
fn every_skill_ten_thousand_draws_respect_every_constraint() {
    for skill in content().skills.all() {
        for seed in 0..1000u64 {
            let mode = skill.answer_modes[(seed as usize) % skill.answer_modes.len()];
            let series = generate_series(skill, seed, mode, DEFAULT_LENGTH).unwrap();
            assert_eq!(
                series.len(),
                series
                    .iter()
                    .map(|e| e.calculation.duplicate_key())
                    .collect::<HashSet<_>>()
                    .len()
            );
            for exercise in &series {
                assert!(
                    CalculationSpace::is_valid(skill, &exercise.calculation),
                    "{} seed {seed}: {}",
                    skill.id,
                    exercise.calculation
                );
                assert!(skill.formats.contains(&exercise.format()));
                assert!(exercise.expected() >= 0);
                if let Some(choices) = &exercise.choices {
                    assert_eq!(
                        4,
                        choices
                            .iter()
                            .filter(|c| **c >= 0)
                            .collect::<HashSet<_>>()
                            .len()
                    );
                    assert!(choices.contains(&exercise.expected()));
                }
            }
        }
    }
}

/// SUB000112: subtractions with small numbers from the first period of CP.
#[test]
fn cp_offers_subtractions_within_ten_from_period_one() {
    let skill = content().skills.find("cp-sub-within-10").unwrap();
    assert_eq!((Grade::CP, 1), (skill.grade, skill.period));
    assert!(skill.source.contains("Educator"), "{}", skill.source);
    for seed in 0..1000u64 {
        for exercise in generate_series(skill, seed, AnswerMode::Keyboard, DEFAULT_LENGTH).unwrap()
        {
            let c = exercise.calculation;
            assert_eq!(ArithmeticOperation::Subtraction, c.operation);
            assert!((2..=10).contains(&c.left) && c.result() > 0, "{c}");
        }
    }
}

/// SUB000112: « ajouter ou enlever 1 ou 2 » around 10, with at least 4 subtractions out of 10.
#[test]
fn adding_or_removing_one_or_two_balances_the_operations() {
    let skill = content().skills.find("cp-add-sub-1-2").unwrap();
    assert_eq!(1, skill.period);
    for seed in 0..1000u64 {
        let series = generate_series(skill, seed, AnswerMode::Choice, DEFAULT_LENGTH).unwrap();
        let subtractions = series
            .iter()
            .filter(|e| e.calculation.operation == ArithmeticOperation::Subtraction)
            .count();
        assert!(subtractions >= 4, "seed {seed}: {subtractions}");
        assert!(series.iter().all(|e| e.calculation.left <= 12));
    }
}

/// SUB000113: in CE1 period 1, the CP period 5 skills come back as revisions, once each.
#[test]
fn ce1_revises_the_last_period_of_cp() {
    let skills = &content().skills;
    let offered = skills.offered_to(Subject::Math, Grade::CE1);
    let cp_last: BTreeSet<&str> = skills
        .for_grade(Subject::Math, Grade::CP)
        .iter()
        .filter(|s| s.period == 5)
        .map(|s| s.id.as_str())
        .collect();
    let revised: BTreeSet<&str> = offered
        .iter()
        .filter(|o| o.revision_from == Some(Grade::CP) && o.period == 1)
        .map(|o| o.skill.id.as_str())
        .collect();

    assert!(!cp_last.is_empty());
    assert_eq!(cp_last, revised);
    let ids: Vec<&str> = offered.iter().map(|o| o.skill.id.as_str()).collect();
    assert_eq!(ids.len(), ids.iter().collect::<HashSet<_>>().len());
}

#[test]
fn current_period_comes_from_the_calendar_file() {
    assert_eq!(1, content().calendar.period_of(Date::new(2026, 10, 11)));
    assert_eq!(5, content().calendar.period_of(Date::new(2027, 6, 30)));
}

#[test]
fn points_come_from_the_rewards_file() {
    assert_eq!(PointsRules::new(10, 20, 20), content().points);
}

fn read_json(path: &Path) -> serde_json::Value {
    serde_json::from_str(&std::fs::read_to_string(path).unwrap())
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}
