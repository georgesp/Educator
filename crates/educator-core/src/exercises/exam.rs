//! Composition of an exam (SUB001414) and of a timed challenge (SUB001415): questions drawn from
//! several skills of the grade, with the generators of the families.

use super::{distinct_count, generate_series_as, Exercise};
use crate::random::SeededRng;
use crate::skills::{AnswerMode, OfferedSkill, Skill};
use crate::{Error, Result};
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::collections::HashSet;

/// Questions of an exam.
pub const EXAM_LENGTH: usize = 15;

/// Most questions an exam asks on one skill.
pub const EXAM_MAX_PER_SKILL: usize = 3;

/// Questions prepared for a timed challenge: more than a child answers in 3 minutes.
pub const TIMED_LENGTH: usize = 90;

/// Mastery (0 to 100) from which a skill enters the timed challenge: no pressure on what is not
/// mastered yet (SUB001415).
pub const TIMED_MIN_MASTERY: u8 = 60;

/// The durations offered for a timed challenge, in seconds (2 minutes by default).
pub const TIMED_DURATIONS: [u32; 3] = [60, 120, 180];

pub const DEFAULT_TIMED_DURATION: u32 = 120;

/// A question of an exam or a challenge, with its skill.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposedQuestion {
    pub skill_id: String,
    pub exercise: Exercise,
}

/// Weight of a skill in an exam: the skills of the current period count three times, those of the
/// period before twice, the older ones once (« pondération plus forte sur les plus récentes »).
pub fn exam_weight(skill_period: u8, current_period: u8) -> u32 {
    3u32.saturating_sub(u32::from(current_period.saturating_sub(skill_period)))
        .max(1)
}

/// The skills of an exam: the grade's own skills up to the current period (the revisions of the
/// grade before are left out); all the grade's skills of period 1 at worst.
pub fn exam_skills<'a>(offered: &[OfferedSkill<'a>], current_period: u8) -> Vec<(&'a Skill, u32)> {
    let own: Vec<&OfferedSkill<'a>> = offered
        .iter()
        .filter(|o| o.revision_from.is_none())
        .collect();
    let reached: Vec<&OfferedSkill<'a>> = own
        .iter()
        .copied()
        .filter(|o| o.period <= current_period)
        .collect();
    let chosen = if reached.is_empty() {
        let first = own.iter().map(|o| o.period).min().unwrap_or(1);
        own.into_iter().filter(|o| o.period == first).collect()
    } else {
        reached
    };
    chosen
        .into_iter()
        .map(|o| (o.skill, exam_weight(o.period, current_period)))
        .collect()
}

/// How many questions each skill gets: proportional to its weight (largest remainders), at most
/// [`EXAM_MAX_PER_SKILL`] each — more when there are few skills, so that the exam always has
/// `length` questions. With more skills than questions, the skills are drawn (weighted) first.
pub fn plan_questions(weights: &[u32], length: usize, random: &mut impl Rng) -> Vec<usize> {
    let mut counts = vec![0usize; weights.len()];
    if weights.is_empty() || length == 0 {
        return counts;
    }

    let max_per_skill = EXAM_MAX_PER_SKILL.max(length.div_ceil(weights.len()));
    let mut candidates: Vec<usize> = (0..weights.len()).collect();
    if weights.len() > length {
        // Weighted draw without replacement of `length` skills, one question each.
        let mut keys: Vec<(f64, usize)> = candidates
            .iter()
            .map(|&i| {
                let u: f64 = random.random_range(f64::EPSILON..1.0);
                (u.powf(1.0 / f64::from(weights[i].max(1))), i)
            })
            .collect();
        keys.sort_by(|a, b| b.0.total_cmp(&a.0));
        candidates = keys.into_iter().take(length).map(|(_, i)| i).collect();
    }

    let total: u32 = candidates.iter().map(|&i| weights[i].max(1)).sum();
    let mut remainders: Vec<(f64, usize)> = Vec::new();
    for &i in &candidates {
        let exact = length as f64 * f64::from(weights[i].max(1)) / f64::from(total);
        counts[i] = (exact.floor() as usize).clamp(1, max_per_skill);
        remainders.push((exact - exact.floor(), i));
    }

    remainders.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    // Add to the largest remainders, then anywhere there is room; remove from the largest counts.
    let mut assigned: usize = counts.iter().sum();
    let mut order: Vec<usize> = remainders.iter().map(|(_, i)| *i).collect();
    while assigned < length {
        let room = |i: &usize| counts[*i] < max_per_skill;
        if !order.iter().any(room) {
            // A new round over every skill with room, the heaviest first.
            order = candidates.iter().copied().filter(room).collect();
            order.sort_by_key(|&i| (std::cmp::Reverse(weights[i]), i));
        }
        let Some(&i) = order.iter().find(|i| room(i)) else {
            break;
        };
        counts[i] += 1;
        assigned += 1;
        order.retain(|&j| j != i);
    }
    while assigned > length {
        let i = (0..counts.len())
            .filter(|&i| counts[i] > 1)
            .min_by_key(|&i| (weights[i], std::cmp::Reverse(counts[i])))
            .unwrap_or(0);
        counts[i] -= 1;
        assigned -= 1;
    }

    counts
}

/// The answer mode of the questions of a skill: drawn among its modes, so that an exam mixes the
/// typed and the picked answers (« types d'exercices variés »).
fn mode_for(skill: &Skill, prefer_choice: bool, random: &mut impl Rng) -> AnswerMode {
    let has = |m| skill.answer_modes.contains(&m);
    match (has(AnswerMode::Keyboard), has(AnswerMode::Choice)) {
        (true, true) if prefer_choice => AnswerMode::Choice,
        (true, true) => {
            if random.random_bool(0.5) {
                AnswerMode::Keyboard
            } else {
                AnswerMode::Choice
            }
        }
        (false, true) => AnswerMode::Choice,
        _ => AnswerMode::Keyboard,
    }
}

/// Composes the questions: `counts[i]` questions of `skills[i]`, never twice the same question
/// (same written prompt), shuffled, numbered from 0.
fn compose(
    skills: &[&Skill],
    counts: &[usize],
    seed: u64,
    prefer_choice: bool,
) -> Result<Vec<ComposedQuestion>> {
    let mut random = SeededRng::seed_from_u64(seed);
    let mut questions: Vec<ComposedQuestion> = Vec::new();
    let mut prompts: HashSet<String> = HashSet::new();
    for (skill, &count) in skills.iter().zip(counts) {
        if count == 0 {
            continue;
        }

        let mode = mode_for(skill, prefer_choice, &mut random);
        // A few more than needed, to skip a question already asked by another skill; counting the
        // different questions of the skill (costly: thousands of draws) only when it has too few.
        let generated = [count + 4, count]
            .into_iter()
            .find_map(|n| generate_series_as(skill, random.random(), mode, n, None).ok())
            .map_or_else(
                || {
                    let available = distinct_count(skill).max(1);
                    generate_series_as(skill, random.random(), mode, count.min(available), None)
                },
                Ok,
            )?;
        let mut taken = 0;
        for exercise in generated {
            if taken == count {
                break;
            }

            let prompt = format!("{}|{}", exercise.prompt(), exercise.key);
            if prompts.insert(prompt) {
                questions.push(ComposedQuestion {
                    skill_id: skill.id.clone(),
                    exercise,
                });
                taken += 1;
            }
        }
    }

    questions.shuffle(&mut random);
    for (index, question) in questions.iter_mut().enumerate() {
        question.exercise.index = index;
    }
    Ok(questions)
}

/// The 15 questions of an exam (SUB001414) from the weighted skills.
pub fn compose_exam(skills: &[(&Skill, u32)], seed: u64) -> Result<Vec<ComposedQuestion>> {
    if skills.is_empty() {
        return Err(Error::SeriesRequest("No skill for an exam.".to_owned()));
    }

    let mut random = SeededRng::seed_from_u64(seed ^ 0xE8A7);
    let weights: Vec<u32> = skills.iter().map(|(_, w)| *w).collect();
    let counts = plan_questions(&weights, EXAM_LENGTH, &mut random);
    let list: Vec<&Skill> = skills.iter().map(|(s, _)| *s).collect();
    compose(&list, &counts, seed, false)
}

/// The questions of a timed challenge (SUB001415): the skills already mastered, in equal shares,
/// picked answers when the skill allows them (faster).
pub fn compose_timed(skills: &[&Skill], seed: u64) -> Result<Vec<ComposedQuestion>> {
    if skills.is_empty() {
        return Err(Error::SeriesRequest(
            "No skill mastered enough for a timed challenge.".to_owned(),
        ));
    }

    // Rounds of questions until there are enough: a question may come back in a later round when
    // the skills have few different ones.
    let per_skill = TIMED_LENGTH.div_ceil(skills.len()).min(30);
    let counts = vec![per_skill; skills.len()];
    let mut questions: Vec<ComposedQuestion> = Vec::new();
    for round in 0..8u64 {
        if questions.len() >= TIMED_LENGTH {
            break;
        }
        questions.extend(compose(
            skills,
            &counts,
            seed.wrapping_add(round * 7919),
            true,
        )?);
    }
    questions.truncate(TIMED_LENGTH);
    for (index, question) in questions.iter_mut().enumerate() {
        question.exercise.index = index;
    }
    Ok(questions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recent_skills_weigh_more() {
        assert_eq!(3, exam_weight(3, 3));
        assert_eq!(2, exam_weight(2, 3));
        assert_eq!(1, exam_weight(1, 3));
        assert_eq!(1, exam_weight(1, 5));
    }

    #[test]
    fn the_plan_gives_fifteen_questions_three_at_most_per_skill() {
        let mut random = SeededRng::seed_from_u64(1);
        for weights in [
            vec![3, 2, 1, 1, 1],
            vec![3],
            vec![3, 3],
            vec![1; 12],
            vec![3, 3, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ] {
            let counts = plan_questions(&weights, EXAM_LENGTH, &mut random);
            let total: usize = counts.iter().sum();
            assert_eq!(EXAM_LENGTH, total, "{weights:?} → {counts:?}");
            let cap = EXAM_MAX_PER_SKILL.max(EXAM_LENGTH.div_ceil(weights.len()));
            assert!(counts.iter().all(|&c| c <= cap), "{weights:?} → {counts:?}");
        }

        // Five skills: every skill asked, the recent ones more.
        let counts = plan_questions(&[3, 3, 2, 1, 1], EXAM_LENGTH, &mut random);
        assert!(counts.iter().all(|&c| c >= 1), "{counts:?}");
        assert!(
            counts[0] >= counts[3] && counts[1] >= counts[4],
            "{counts:?}"
        );
    }
}
