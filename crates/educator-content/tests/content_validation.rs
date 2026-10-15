//! The committed content must be valid: a broken rule fails these tests (and the CI), never the
//! application in front of a child (SUB000106, SUB000107, SUB000108).

use educator_content::{EducatorContent, MIN_DISTINCT_EXERCISES};
use educator_core::exercises::{
    distinct_count, generate_series, Answer, ArithmeticOperation, CalculationSpace, Input,
    DEFAULT_LENGTH,
};
use educator_core::points::PointsRules;
use educator_core::skills::{
    AnswerMode, Date, ExerciseFormat, SkillCatalog, SkillCategory, SpellingTheme, Subject,
};
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
        (
            "spelling",
            Some("homophones.json"),
            "homophones.schema.json",
        ),
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
        ("rewards", Some("levels.json"), "levels.schema.json"),
        ("rewards", Some("stars.json"), "stars.schema.json"),
        ("rewards", Some("daily-goal.json"), "daily-goal.schema.json"),
        ("rewards", Some("badges.json"), "badges.schema.json"),
        (
            "accessories",
            Some("accessories.json"),
            "accessories.schema.json",
        ),
        ("progress", Some("adaptive.json"), "adaptive.schema.json"),
        ("progress", Some("reviews.json"), "reviews.schema.json"),
        ("themes", Some("themes.json"), "themes.schema.json"),
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

/// Every spelling skill has a theme, no math skill has one: each card shows its family.
#[test]
fn every_spelling_skill_has_a_theme_and_math_skills_none() {
    for skill in content().skills.all() {
        assert_eq!(
            skill.subject == Subject::Spelling,
            skill.params.theme.is_some(),
            "{}",
            skill.id
        );
    }
    let category = |id: &str| content().skills.find(id).unwrap().category();
    assert_eq!(SkillCategory::Numbers, category("ce1-parity"));
    assert_eq!(
        SkillCategory::FractionsHalves,
        category("ce1-double-half-facts")
    );
    assert_eq!(SkillCategory::SpellingRules, category("ce1-g-gu-ge"));
    assert_eq!(SkillCategory::WordsToKnow, category("cp-mots-outils"));
    assert_eq!(
        Some(SpellingTheme::Agreement),
        content().skills.find("ce1-pluriel-s").unwrap().params.theme
    );
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
    use educator_core::exercises::exam::{compose_exam, exam_skills, Listening, EXAM_LENGTH};
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
                    let exam = compose_exam(&weighted, seed, Listening::Available).unwrap();
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

/// SUB026704: without the voice, a spelling exam (and a level assessment) asks no question where
/// the word is only heard: no dictation, no « Choisis la bonne écriture ».
#[test]
fn without_the_voice_an_exam_asks_no_listening_question() {
    use educator_core::exercises::exam::{
        compose_exam, compose_placement, exam_skills, Listening, EXAM_LENGTH,
    };
    let skills = &content().skills;
    for grade in [Grade::CP, Grade::CE1] {
        let offered = skills.offered_to(Subject::Spelling, grade);
        for period in 1..=5u8 {
            let weighted = exam_skills(&offered, period);
            for seed in 0..20u64 {
                let with_voice = compose_exam(&weighted, seed, Listening::Available).unwrap();
                let exam = compose_exam(&weighted, seed, Listening::Unavailable).unwrap();
                let placement = compose_placement(&weighted, seed, Listening::Unavailable).unwrap();
                assert_eq!(EXAM_LENGTH, exam.len(), "{grade} P{period}");
                assert!(
                    exam.iter()
                        .chain(&placement)
                        .all(|q| !q.exercise.format.needs_listening()),
                    "{grade} P{period} seed {seed}"
                );
                if seed == 0 {
                    assert!(
                        with_voice
                            .iter()
                            .any(|q| q.exercise.format.needs_listening()),
                        "{grade} P{period}: the voice gives dictations"
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
        compose_placement, exam_skills, Listening, PLACEMENT_MAX_LENGTH, PLACEMENT_MAX_PER_SKILL,
    };
    let skills = &content().skills;
    for subject in [Subject::Math, Subject::Spelling] {
        for grade in [Grade::CP, Grade::CE1] {
            let offered = skills.offered_to(subject, grade);
            for period in 1..=5u8 {
                let weighted = exam_skills(&offered, period);
                assert!(!weighted.is_empty(), "{subject:?} {grade} P{period}");
                for seed in 0..40u64 {
                    let placement =
                        compose_placement(&weighted, seed, Listening::Available).unwrap();
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
                        assert!(
                            asked <= PLACEMENT_MAX_PER_SKILL,
                            "{} asked {asked} times",
                            skill.id
                        );
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
                "frequency": 1, "category": "nom", "article": "un", "source": "test"}}"#
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

/// A word bank of one file in a temporary content folder (SUB026706).
fn load_one_file(words: &str) -> educator_content::Loaded<educator_content::WordBank> {
    use educator_content::{load_words, HOMOPHONES_FILE, SOUNDS_FILE, WORDS_FOLDER};
    let dir = tempfile::tempdir().unwrap();
    for file in [SOUNDS_FILE, HOMOPHONES_FILE] {
        let path = dir.path().join(file);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::copy(content_dir().join(file), &path).unwrap();
    }
    std::fs::create_dir_all(dir.path().join(WORDS_FOLDER)).unwrap();
    std::fs::write(
        dir.path().join(WORDS_FOLDER).join("test.json"),
        format!(r#"{{"words": [{words}]}}"#),
    )
    .unwrap();
    load_words(dir.path())
}

fn word_json(word: &str, category: &str, extra: &str) -> String {
    format!(
        r#"{{"word": "{word}", "grades": ["CP"], "sounds": [], "graphemes": [],
            "frequency": 1, "category": "{category}", {extra} "source": "test"}}"#
    )
}

/// SUB026706: a noun is read with its article; a word with a pronunciation is read with it, and
/// the word to write stays the word.
#[test]
fn a_word_is_read_with_its_article_or_its_pronunciation() {
    let loaded = load_one_file(&format!(
        "{}, {}, {}",
        word_json("stylo", "nom", r#""article": "un","#),
        word_json(
            "fils",
            "nom",
            r#""article": "un", "pronunciation": "fisse","#
        ),
        word_json("très", "adverbe", ""),
    ));
    assert!(loaded.problems.is_empty(), "{:?}", loaded.problems);
    let bank = loaded.value;

    assert_eq!("un stylo.", bank.find("stylo").unwrap().reading());
    assert_eq!("un fisse.", bank.find("fils").unwrap().reading());
    assert_eq!("fils", bank.find("fils").unwrap().text);
    assert_eq!("Le mot : très.", bank.find("très").unwrap().reading());
}

/// SUB026706: a noun without its article, an article other than « un » / « une », an article on
/// another word and an empty pronunciation are refused.
#[test]
fn articles_and_pronunciations_are_checked() {
    for (word, problem) in [
        (word_json("stylo", "nom", ""), "a noun needs its article"),
        (
            word_json("école", "nom", r#""article": "l'","#),
            "article 'l'' is unknown",
        ),
        (
            word_json("très", "adverbe", r#""article": "un","#),
            "only a noun has an article",
        ),
        (
            word_json("très", "adverbe", r#""pronunciation": " ","#),
            "pronunciation must not be empty",
        ),
        (
            word_json("très", "adverbe", r#""pronunciation": "trè  s","#),
            "pronunciation must not be empty",
        ),
    ] {
        let loaded = load_one_file(&word);
        assert!(
            loaded.problems.iter().any(|p| p.message.contains(problem)),
            "{word}: {:?}",
            loaded.problems
        );
        assert!(loaded.value.words.is_empty(), "{word}");
    }
}

/// SUB026706: « prêt » and « près » sound the same: a homophone without its sentence is refused,
/// whether it is in the list of common homophones or read like another word of the bank.
#[test]
fn a_homophone_without_its_sentence_is_refused() {
    let listed = load_one_file(&word_json("près", "adverbe", ""));
    assert!(
        listed.problems.iter().any(|p| p
            .message
            .contains("'près': a homophone needs its example sentence")),
        "{:?}",
        listed.problems
    );

    let with_sentence = load_one_file(&word_json(
        "près",
        "adverbe",
        r#""example": "J'habite près de l'école.","#,
    ));
    assert!(
        with_sentence.problems.is_empty(),
        "{:?}",
        with_sentence.problems
    );

    // Two words of the bank read the same way.
    let read_alike = load_one_file(&format!(
        "{}, {}",
        word_json("pomme", "nom", r#""article": "une","#),
        word_json(
            "paume",
            "nom",
            r#""article": "une", "pronunciation": "pomme","#
        ),
    ));
    for word in ["pomme", "paume"] {
        assert!(
            read_alike
                .problems
                .iter()
                .any(|p| p.message.contains(&format!("'{word}': a homophone"))),
            "{word}: {:?}",
            read_alike.problems
        );
    }
}

/// SUB001503: about twenty badges, in mathematics, spelling and regularity.
#[test]
fn there_are_about_twenty_badges_of_every_kind() {
    use educator_core::rewards::badges::BadgeCondition;
    let badges = &content().badges;
    assert!(badges.len() >= 20, "{}", badges.len());
    let skill_subject = |condition: &BadgeCondition| match condition {
        BadgeCondition::SkillMastery { skills, .. } => {
            content().skills.find(&skills[0]).map(|s| s.subject)
        }
        BadgeCondition::CategoryMastery { category, .. } => content()
            .skills
            .all()
            .iter()
            .find(|s| s.category() == *category)
            .map(|s| s.subject),
        _ => None,
    };
    let math = badges
        .iter()
        .filter(|b| skill_subject(&b.condition) == Some(Subject::Math))
        .count();
    let spelling = badges
        .iter()
        .filter(|b| skill_subject(&b.condition) == Some(Subject::Spelling))
        .count();
    let regularity = badges.len() - math - spelling;
    assert!(
        math >= 5 && spelling >= 4 && regularity >= 5,
        "{math} {spelling} {regularity}"
    );
}

/// SUB001501 / SUB001502 / SUB001507 / SUB001508 / SUB001509: the scales come from their files.
#[test]
fn reward_and_progress_scales_come_from_their_files() {
    let content = content();
    assert_eq!(2, content.rewards.levels.level_of(100));
    assert_eq!(70, content.rewards.stars.two_stars);
    assert_eq!(3, content.rewards.goal.default_series);
    assert_eq!(85, content.adaptive.raise_above);
    assert_eq!(vec![1, 3, 7, 14], content.reviews.intervals);
    assert!(content.themes.iter().any(|t| t.id == "sport"));
}
