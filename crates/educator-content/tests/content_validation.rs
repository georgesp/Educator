//! The committed content must be valid: a broken rule fails these tests (and the CI), never the
//! application in front of a child (SUB000106, SUB000107, SUB000108).

use educator_content::{EducatorContent, MIN_DISTINCT_EXERCISES};
use educator_core::exercises::{
    distinct_count, generate_series, Answer, ArithmeticOperation, CalculationSpace, Input,
    DEFAULT_LENGTH,
};
use educator_core::points::PointsRules;
use educator_core::skills::{AnswerMode, Date, ExerciseFormat, SkillCatalog, Subject};
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
        ("skills/spelling", None, "skill.schema.json"),
        ("words", None, "words.schema.json"),
        ("spelling", Some("sounds.json"), "sounds.schema.json"),
        ("mascot", Some("mascot.json"), "mascot.schema.json"),
        (
            "problems",
            Some("additive.json"),
            "problem-templates.schema.json",
        ),
        (
            "problems",
            Some("two-steps.json"),
            "problem-templates.schema.json",
        ),
        (
            "problems",
            Some("multiplicative.json"),
            "problem-templates.schema.json",
        ),
        ("problems", Some("words.json"), "problem-words.schema.json"),
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
            distinct_count(skill) >= MIN_DISTINCT_EXERCISES,
            "{}",
            skill.id
        );
    }
}

/// Property test: 10 000 exercises per skill (1 000 series) respect every rule; 2 000 series for
/// the families of the CE1 complements (SUB000118 to SUB000124).
#[test]
fn every_skill_ten_thousand_draws_respect_every_constraint() {
    for skill in content().skills.all() {
        let seeds = if skill.operation.is_calculation() && skill.params.focus.is_empty() {
            1000u64
        } else {
            2000u64
        };
        for seed in 0..seeds {
            let mode = skill.answer_modes[(seed as usize) % skill.answer_modes.len()];
            let series = generate_series(skill, seed, mode, DEFAULT_LENGTH).unwrap();
            assert_eq!(DEFAULT_LENGTH, series.len(), "{}", skill.id);
            assert_eq!(
                series.len(),
                series.iter().map(|e| &e.key).collect::<HashSet<_>>().len(),
                "{} seed {seed}: duplicate",
                skill.id
            );
            for exercise in &series {
                assert!(
                    skill.formats.contains(&exercise.format),
                    "{} seed {seed}: format {:?}",
                    skill.id,
                    exercise.format
                );
                assert!(
                    exercise.is_correct(&exercise.expected.as_given()),
                    "{} seed {seed}",
                    skill.id
                );
                assert!(!exercise.explanation.is_empty() && !exercise.spoken.is_empty());
                if let Some(c) = &exercise.calculation {
                    assert!(
                        CalculationSpace::is_valid(skill, &c.calculation),
                        "{} seed {seed}: {}",
                        skill.id,
                        c.calculation
                    );
                    assert!(c.expected() >= 0);
                }
                if let Answer::Number(n) = exercise.expected {
                    assert!(n >= 0, "{} seed {seed}", skill.id);
                }
                if let Some(choices) = &exercise.choices {
                    assert_eq!(Input::Pick, exercise.input);
                    assert!(choices.len() >= 3, "{} seed {seed}", skill.id);
                    let right = choices
                        .iter()
                        .filter(|c| exercise.is_correct(&c.as_given()))
                        .count();
                    assert_eq!(1, right, "{} seed {seed}: {choices:?}", skill.id);
                    let labels: HashSet<String> = choices.iter().map(Answer::label).collect();
                    assert_eq!(choices.len(), labels.len(), "{} seed {seed}", skill.id);
                    assert!(choices
                        .iter()
                        .all(|c| !matches!(c, Answer::Number(n) if *n < 0)));
                    if matches!(exercise.expected, Answer::Number(_)) {
                        assert_eq!(4, choices.len());
                    }
                } else {
                    assert!(
                        mode == AnswerMode::Keyboard
                            || exercise.input != Input::Number { slots: 1 }
                    );
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
            let c = exercise.calculation.unwrap().calculation;
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
            .filter(|e| calculation(e).operation == ArithmeticOperation::Subtraction)
            .count();
        assert!(subtractions >= 4, "seed {seed}: {subtractions}");
        assert!(series.iter().all(|e| calculation(e).left <= 12));
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

fn calculation(
    exercise: &educator_core::exercises::Exercise,
) -> educator_core::exercises::Calculation {
    exercise.calculation.as_ref().unwrap().calculation
}

fn read_json(path: &Path) -> serde_json::Value {
    serde_json::from_str(&std::fs::read_to_string(path).unwrap())
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// SUB001402, SUB001403: every spelling skill has at least 30 words; CP and CE1 have spelling skills.
#[test]
fn every_spelling_skill_has_at_least_thirty_words() {
    let skills: Vec<_> = content()
        .skills
        .all()
        .iter()
        .filter(|s| s.subject == Subject::Spelling)
        .collect();
    for grade in [Grade::CP, Grade::CE1] {
        assert!(skills.iter().any(|s| s.grade == grade), "{grade}");
    }
    for skill in skills {
        let words = skill.params.words.as_ref().unwrap();
        assert!(
            words.words.len() >= educator_content::MIN_SKILL_WORDS,
            "{}: {} words",
            skill.id,
            words.words.len()
        );
        assert!(words.words.iter().all(|w| !w.word.source.is_empty()));
    }
}

/// SUB001408: the wrong spellings proposed are never real words of the bank; SUB001407: the
/// proposals of the missing letters are graphemes of the sound or its confusions.
#[test]
fn spelling_proposals_are_never_real_words_and_compete_on_the_same_sound() {
    let bank = &content().words;
    for skill in content()
        .skills
        .all()
        .iter()
        .filter(|s| s.subject == Subject::Spelling)
    {
        let words = skill.params.words.as_ref().unwrap();
        for seed in 0..200u64 {
            for exercise in
                generate_series(skill, seed, AnswerMode::Choice, DEFAULT_LENGTH).unwrap()
            {
                let word = exercise.word.as_ref().unwrap();
                let choices = exercise.choices.as_ref().unwrap();
                assert_eq!(3, choices.len(), "{} seed {seed}", skill.id);
                match exercise.format {
                    ExerciseFormat::ChooseSpelling => {
                        for choice in choices {
                            let text = choice.label();
                            assert!(
                                text == word.word || bank.find(&text).is_none(),
                                "{}: « {text} » is a real word",
                                skill.id
                            );
                        }
                    }
                    ExerciseFormat::MissingLetters => {
                        let Answer::Spelling(expected) = &exercise.expected else {
                            panic!("letters expected");
                        };
                        // The sounds the hidden letters write: the proposals are their other
                        // graphemes, or their usual confusions for a sound written one way only.
                        let sounds: Vec<_> = words
                            .sounds
                            .iter()
                            .filter(|s| s.graphemes.contains(expected))
                            .collect();
                        assert!(!sounds.is_empty(), "{}: {expected}", skill.id);
                        let (before, after) = word.blank.clone().unwrap();
                        for choice in choices {
                            let letters = choice.label();
                            let competing = letters.is_empty()
                                || sounds.iter().any(|s| {
                                    s.graphemes.contains(&letters)
                                        || s.confusions.contains(&letters)
                                });
                            assert!(
                                competing,
                                "{}: « {letters} » does not compete with « {expected} »",
                                skill.id
                            );
                            let made = format!("{before}{letters}{after}");
                            assert!(
                                &letters == expected || bank.find(&made).is_none(),
                                "{}: « {made} » is a real word",
                                skill.id
                            );
                        }
                    }
                    other => panic!("{other:?} in choice mode"),
                }
            }
        }
    }
}

/// SUB001414: an exam has 15 questions of the grade's own skills up to the current period, the
/// recent skills more often, never twice the same question.
#[test]
fn exams_are_composed_from_the_skills_reached_without_duplicates() {
    use educator_core::exercises::exam::{compose_exam, exam_skills, EXAM_LENGTH};
    let skills = &content().skills;
    for subject in [Subject::Math, Subject::Spelling] {
        for grade in [Grade::CP, Grade::CE1] {
            let offered = skills.offered_to(subject, grade);
            for period in 1..=5u8 {
                let weighted = exam_skills(&offered, period);
                assert!(!weighted.is_empty(), "{subject:?} {grade} P{period}");
                assert!(weighted
                    .iter()
                    .all(|(s, _)| s.grade == grade && s.period <= period.max(1)));
                let (mut recent, mut old) = (0usize, 0usize);
                for seed in 0..40u64 {
                    let exam = compose_exam(&weighted, seed).unwrap();
                    assert_eq!(EXAM_LENGTH, exam.len());
                    let keys: HashSet<String> = exam
                        .iter()
                        .map(|q| format!("{}|{}", q.exercise.prompt(), q.exercise.key))
                        .collect();
                    assert_eq!(exam.len(), keys.len(), "duplicate in {subject:?} {grade}");
                    for (i, q) in exam.iter().enumerate() {
                        assert_eq!(i, q.exercise.index);
                        assert!(q.exercise.is_correct(&q.exercise.expected.as_given()));
                        let skill = skills.find(&q.skill_id).unwrap();
                        if skill.period == period {
                            recent += 1;
                        } else {
                            old += 1;
                        }
                    }
                }
                // Per skill, the current period weighs more than the older ones.
                let count = |p: &dyn Fn(u8) -> bool| {
                    weighted.iter().filter(|(s, _)| p(s.period)).count().max(1)
                };
                if period > 2
                    && weighted.iter().any(|(s, _)| s.period == period)
                    && weighted.iter().any(|(s, _)| s.period + 2 <= period)
                {
                    let recent_per_skill = recent as f64 / count(&|p| p == period) as f64;
                    let old_per_skill = old as f64 / count(&|p| p != period) as f64;
                    assert!(
                        recent_per_skill > old_per_skill,
                        "{subject:?} {grade} P{period}"
                    );
                }
            }
        }
    }
}

/// SUB001418: a level assessment asks 2 or 3 questions per skill of the grade up to the current
/// period, 30 at most, never twice the same question, whatever the grade, period and seed.
#[test]
fn placements_are_composed_from_the_skills_reached_without_duplicates() {
    use educator_core::exercises::exam::{
        compose_placement, exam_skills, PLACEMENT_MAX_LENGTH, PLACEMENT_MAX_PER_SKILL,
    };
    let skills = &content().skills;
    for subject in [Subject::Math, Subject::Spelling] {
        for grade in [Grade::CP, Grade::CE1] {
            let offered = skills.offered_to(subject, grade);
            for period in 1..=5u8 {
                let weighted = exam_skills(&offered, period);
                assert!(!weighted.is_empty(), "{subject:?} {grade} P{period}");
                for seed in 0..40u64 {
                    let placement = compose_placement(&weighted, seed).unwrap();
                    assert!(
                        !placement.is_empty() && placement.len() <= PLACEMENT_MAX_LENGTH,
                        "{subject:?} {grade} P{period}: {} questions",
                        placement.len()
                    );
                    let keys: HashSet<String> = placement
                        .iter()
                        .map(|q| format!("{}|{}", q.exercise.prompt(), q.exercise.key))
                        .collect();
                    assert_eq!(
                        placement.len(),
                        keys.len(),
                        "duplicate in {subject:?} {grade} P{period}"
                    );
                    for (i, q) in placement.iter().enumerate() {
                        assert_eq!(i, q.exercise.index);
                        assert!(q.exercise.is_correct(&q.exercise.expected.as_given()));
                        let skill = skills.find(&q.skill_id).unwrap();
                        assert!(skill.grade == grade && skill.period <= period);
                    }
                    for (skill, _) in &weighted {
                        let asked = placement.iter().filter(|q| q.skill_id == skill.id).count();
                        assert!(asked <= PLACEMENT_MAX_PER_SKILL, "{} asked {asked} times", skill.id);
                    }
                }
            }
        }
    }
}

/// SUB001417: the mascot has her four expressions and phrases for every situation.
#[test]
fn the_mascot_has_four_expressions_and_phrases_for_every_situation() {
    use educator_core::mascot::{Expression, Situation};
    let mascot = content().mascot.as_ref().expect("the mascot loads");
    assert_eq!("Plume", mascot.name);
    for expression in Expression::ALL {
        assert!(mascot.pictures[&expression].contains("id=\"face\""));
    }
    for situation in Situation::ALL {
        assert!(mascot.phrases[&situation].len() >= 3, "{situation:?}");
    }
}

/// SUB001402: a badly written word (here, a grapheme at the wrong place) is refused with a clear
/// message, and never reaches the child.
#[test]
fn a_badly_written_word_is_refused() {
    use educator_content::{load_words, SOUNDS_FILE, WORDS_FOLDER};
    let dir = tempfile::tempdir().unwrap();
    let sounds = dir.path().join(SOUNDS_FILE);
    std::fs::create_dir_all(sounds.parent().unwrap()).unwrap();
    std::fs::copy(content_dir().join(SOUNDS_FILE), &sounds).unwrap();
    std::fs::create_dir_all(dir.path().join(WORDS_FOLDER)).unwrap();
    let word = |position: u32| {
        format!(
            r#"{{"word": "bateau", "grades": ["CP"], "sounds": ["o"],
                "graphemes": [{{"text": "eau", "sound": "o", "position": {position}}}],
                "frequency": 1, "category": "nom", "source": "test"}}"#
        )
    };
    let write = |position: u32| {
        std::fs::write(
            dir.path().join(WORDS_FOLDER).join("test.json"),
            format!(r#"{{"words": [{}]}}"#, word(position)),
        )
        .unwrap();
    };

    write(3);
    let good = load_words(dir.path());
    assert!(good.problems.is_empty(), "{:?}", good.problems);
    assert!(good.value.find("bateau").is_some());

    write(1);
    let bad = load_words(dir.path());
    assert!(
        bad.problems
            .iter()
            .any(|p| p.file == "words/test.json" && p.message.starts_with("'bateau'")),
        "{:?}",
        bad.problems
    );
    assert!(bad.value.find("bateau").is_none());
}
