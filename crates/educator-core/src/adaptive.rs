//! Adaptive difficulty (SUB001508): « Je te conseille… » on the exercises screen. The starting
//! point is the grade and the current period; the mastery of the skills ([`SkillMastery`], the last
//! answers) moves the advice: a skill well mastered leads to the next one, a skill missed too often
//! sends back to the skill before it in the same family. Only an advice: every skill stays open.
//! Thresholds in `content/progress/adaptive.json`.

use crate::sessions::SkillMastery;
use crate::skills::OfferedSkill;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdaptiveRules {
    /// From this mastery (percent), a skill is mastered: the advice moves on.
    pub raise_above: u8,
    /// Under this mastery, the skill before comes back first.
    pub lower_below: u8,
    /// Answers needed before a mastery moves the advice.
    pub min_answers: u32,
}

impl AdaptiveRules {
    pub const DEFAULT: AdaptiveRules = AdaptiveRules {
        raise_above: 85,
        lower_below: 60,
        min_answers: 10,
    };

    pub fn new(raise_above: u8, lower_below: u8, min_answers: u32) -> Result<Self, String> {
        if lower_below >= raise_above || raise_above > 100 {
            return Err("Expected lowerBelow < raiseAbove <= 100.".to_owned());
        }
        if min_answers == 0 {
            return Err("At least one answer before moving the advice.".to_owned());
        }
        Ok(Self {
            raise_above,
            lower_below,
            min_answers,
        })
    }
}

impl Default for AdaptiveRules {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Why a skill is advised.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdviceReason {
    /// Not practised enough yet: a skill of the current period.
    Start,
    /// Being learnt: keep going.
    Continue,
    /// The skill of the current period is missed too often: the one before it first.
    Consolidate,
    /// Everything of the current period is mastered: the next skill.
    Next,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Advice {
    pub skill_id: String,
    pub reason: AdviceReason,
    /// [`AdviceReason::Consolidate`]: the skill of the current period it prepares.
    pub prepares: Option<String>,
}

/// The advice for a grade (`offered`: what the grade offers, sorted by period), or `None` when
/// everything up to the next period is mastered.
pub fn advise(
    rules: &AdaptiveRules,
    offered: &[OfferedSkill<'_>],
    current_period: u8,
    masteries: &[SkillMastery],
) -> Option<Advice> {
    let mastery = |id: &str| masteries.iter().find(|m| m.skill_id == id);
    let known = |id: &str| mastery(id).filter(|m| m.attempt_count >= rules.min_answers);
    let own: Vec<&OfferedSkill<'_>> = offered
        .iter()
        .filter(|o| o.revision_from.is_none())
        .collect();
    // The period of the starting point: the current one, or the last one before it with skills.
    let period = own
        .iter()
        .map(|o| o.period)
        .filter(|p| *p <= current_period)
        .max()
        .or_else(|| own.iter().map(|o| o.period).min())?;

    for (position, skill) in offered.iter().enumerate() {
        if skill.revision_from.is_some() || skill.period != period {
            continue;
        }
        let id = &skill.skill.id;
        let Some(m) = known(id) else {
            let reason = if mastery(id).is_some() {
                AdviceReason::Continue
            } else {
                AdviceReason::Start
            };
            return Some(advice(id, reason, None));
        };
        if m.score < rules.lower_below {
            // The skill before it in its family (revisions included), when there is one.
            let category = skill.skill.category();
            let before = offered[..position]
                .iter()
                .rev()
                .find(|o| o.skill.category() == category && o.skill.id != *id);
            return Some(match before {
                Some(before) => advice(&before.skill.id, AdviceReason::Consolidate, Some(id)),
                None => advice(id, AdviceReason::Continue, None),
            });
        }
        if m.score < rules.raise_above {
            return Some(advice(id, AdviceReason::Continue, None));
        }
    }

    // Everything of the period is mastered: the first skill after it not mastered yet.
    own.iter()
        .filter(|o| o.period > period)
        .find(|o| known(&o.skill.id).is_none_or(|m| m.score < rules.raise_above))
        .map(|o| advice(&o.skill.id, AdviceReason::Next, None))
}

fn advice(skill_id: &str, reason: AdviceReason, prepares: Option<&str>) -> Advice {
    Advice {
        skill_id: skill_id.to_owned(),
        reason,
        prepares: prepares.map(str::to_owned),
    }
}
