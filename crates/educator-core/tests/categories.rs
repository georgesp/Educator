//! Family shown on the cards of the exercises and the lessons: from the operation in math, from the
//! theme in spelling.

mod common;

use educator_core::skills::{SkillCategory, SkillOperation, SpellingTheme, Subject};

#[test]
fn math_families_come_from_the_operation() {
    let cases = [
        (SkillOperation::Numeration, SkillCategory::Numbers),
        (SkillOperation::Addition, SkillCategory::AddSubtract),
        (SkillOperation::Subtraction, SkillCategory::AddSubtract),
        (SkillOperation::Mixed, SkillCategory::AddSubtract),
        (SkillOperation::Multiplication, SkillCategory::Multiply),
        (SkillOperation::Fraction, SkillCategory::FractionsHalves),
        (SkillOperation::DoubleHalf, SkillCategory::FractionsHalves),
        (SkillOperation::Problem, SkillCategory::Problems),
    ];
    for (operation, expected) in cases {
        let mut skill = common::test_skill();
        skill.operation = operation;
        assert_eq!(expected, skill.category(), "{operation:?}");
    }
}

#[test]
fn spelling_families_come_from_the_theme() {
    let cases = [
        (
            SpellingTheme::LettersAndSounds,
            SkillCategory::LettersAndSounds,
        ),
        (SpellingTheme::SpellingRules, SkillCategory::SpellingRules),
        (SpellingTheme::SilentLetters, SkillCategory::SilentLetters),
        (SpellingTheme::Agreement, SkillCategory::Agreement),
        (SpellingTheme::WordsToKnow, SkillCategory::WordsToKnow),
    ];
    for (theme, expected) in cases {
        let mut skill = common::test_skill();
        skill.subject = Subject::Spelling;
        skill.operation = SkillOperation::Spelling;
        skill.params.theme = Some(theme);
        assert_eq!(expected, skill.category(), "{theme:?}");
    }
}

#[test]
fn every_category_is_listed_once() {
    let mut all = SkillCategory::ALL.to_vec();
    all.sort();
    all.dedup();
    assert_eq!(SkillCategory::ALL.len(), all.len());
}
