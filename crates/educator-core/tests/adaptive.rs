//! SUB001508: « Je te conseille… » on simulated histories of answers.

mod common;

use common::skill;
use educator_core::adaptive::{advise, AdaptiveRules, AdviceReason};
use educator_core::sessions::{mastery_score, SkillMastery, MASTERY_WINDOW};
use educator_core::skills::{OfferedSkill, Skill, SkillCatalog, SkillList, SkillOperation};
use educator_core::Grade;
use uuid::Uuid;

/// CP: additions in periods 1, 2 and 3, a subtraction in period 2, one CP skill per period.
fn skills() -> SkillList {
    let at = |id: &str, period: u8, operation: SkillOperation| {
        let mut s: Skill = skill(id);
        s.period = period;
        s.operation = operation;
        s
    };
    SkillList(vec![
        at("add-1", 1, SkillOperation::Addition),
        at("add-2", 2, SkillOperation::Addition),
        at("sub-2", 2, SkillOperation::Subtraction),
        at("num-2", 2, SkillOperation::Numeration),
        at("add-3", 3, SkillOperation::Addition),
    ])
}

/// The mastery left by a history of answers (most recent last), as the sessions compute it.
fn history(skill_id: &str, answers: &[bool]) -> SkillMastery {
    let recent: Vec<bool> = answers.iter().rev().take(MASTERY_WINDOW).copied().collect();
    SkillMastery {
        profile_id: Uuid::nil(),
        skill_id: skill_id.to_owned(),
        score: mastery_score(&recent),
        attempt_count: answers.len() as u32,
        last_practiced_at: 0,
    }
}

fn right(n: usize) -> Vec<bool> {
    vec![true; n]
}

fn mixed(right: usize, wrong: usize) -> Vec<bool> {
    let mut answers = vec![true; right];
    answers.extend(vec![false; wrong]);
    answers
}

fn run(period: u8, masteries: &[SkillMastery]) -> Option<(String, AdviceReason)> {
    let catalog = skills();
    let offered: Vec<OfferedSkill<'_>> =
        catalog.offered_to(educator_core::skills::Subject::Math, Grade::CP);
    advise(&AdaptiveRules::DEFAULT, &offered, period, masteries).map(|a| (a.skill_id, a.reason))
}

#[test]
fn without_history_the_first_skill_of_the_current_period_is_advised() {
    assert_eq!(Some(("add-2".to_owned(), AdviceReason::Start)), run(2, &[]));
    assert_eq!(Some(("add-1".to_owned(), AdviceReason::Start)), run(1, &[]));
}

#[test]
fn a_few_answers_do_not_move_the_advice() {
    let few = [history("add-2", &right(5))];
    assert_eq!(
        Some(("add-2".to_owned(), AdviceReason::Continue)),
        run(2, &few)
    );
}

#[test]
fn a_mastered_skill_leads_to_the_next_one_of_the_period() {
    let masteries = [history("add-2", &mixed(18, 2))];
    assert_eq!(
        Some(("num-2".to_owned(), AdviceReason::Start)),
        run(2, &masteries)
    );
}

#[test]
fn a_skill_missed_too_often_sends_back_to_the_one_before_in_its_family() {
    let masteries = [history("add-2", &mixed(5, 7))];
    let catalog = skills();
    let offered = catalog.offered_to(educator_core::skills::Subject::Math, Grade::CP);
    let advice = advise(&AdaptiveRules::DEFAULT, &offered, 2, &masteries).unwrap();
    assert_eq!(
        ("add-1", AdviceReason::Consolidate),
        (advice.skill_id.as_str(), advice.reason)
    );
    assert_eq!(Some("add-2".to_owned()), advice.prepares);
}

#[test]
fn a_skill_being_learnt_is_kept() {
    let masteries = [history("add-2", &mixed(14, 6))];
    assert_eq!(
        Some(("add-2".to_owned(), AdviceReason::Continue)),
        run(2, &masteries)
    );
}

#[test]
fn a_child_who_progresses_moves_along_the_skills() {
    // Simulated weeks: add-2 learnt slowly, then the others of the period, then the next period.
    let mut answers: Vec<SkillMastery> = Vec::new();
    let mut add2 = mixed(4, 8);
    answers.push(history("add-2", &add2));
    assert_eq!(AdviceReason::Consolidate, run(2, &answers).unwrap().1);
    add2.extend(right(20));
    answers[0] = history("add-2", &add2);
    assert_eq!(
        Some(("num-2".to_owned(), AdviceReason::Start)),
        run(2, &answers)
    );
    answers.push(history("num-2", &right(12)));
    answers.push(history("sub-2", &right(12)));
    assert_eq!(
        Some(("add-3".to_owned(), AdviceReason::Next)),
        run(2, &answers)
    );
    answers.push(history("add-3", &right(12)));
    assert_eq!(
        None,
        run(2, &answers),
        "everything mastered: no advice, every skill stays open"
    );
}

#[test]
fn rules_are_checked() {
    assert!(AdaptiveRules::new(60, 85, 10).is_err());
    assert!(AdaptiveRules::new(85, 60, 0).is_err());
    assert!(AdaptiveRules::new(85, 60, 10).is_ok());
}
