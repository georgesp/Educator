//! Spaced review of the mistakes (SUB001509): every wrong answer puts the calculation or the word
//! in the review queue of the profile, to see again the next day, then 3, 7 and 14 days later
//! (`content/progress/reviews.json`). In « Mes révisions », an item answered right waits longer
//! before coming back, an item missed again comes back the next day; after the last step it
//! leaves the queue. A calculation comes back as it was when it can be rebuilt (same skill, seed
//! and place in its series), otherwise as a calculation of the same skill (same parameters); a
//! word comes back as the same word.

use crate::clock::{uuid_v7, Clock};
use crate::exercises::exam::Listening;
use crate::exercises::{generate_series_as, Exercise, DEFAULT_LENGTH};
use crate::rewards::DateOf;
use crate::skills::{AnswerMode, ExerciseFormat, Skill};
use crate::statistics::day_number;
use crate::{Error, Result};
use std::fmt;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewRules {
    /// Days before each review: after a mistake, then after each review answered right.
    pub intervals: Vec<u32>,
    /// Exercises of a « Mes révisions » series at most.
    pub series_length: usize,
}

impl ReviewRules {
    pub fn new(intervals: Vec<u32>, series_length: usize) -> std::result::Result<Self, String> {
        if intervals.is_empty() || intervals.contains(&0) {
            return Err("Review intervals are at least one day.".to_owned());
        }
        if intervals.windows(2).any(|w| w[0] > w[1]) {
            return Err("Review intervals must grow.".to_owned());
        }
        if !(1..=20).contains(&series_length) {
            return Err("A review series has 1 to 20 exercises.".to_owned());
        }
        Ok(Self {
            intervals,
            series_length,
        })
    }
}

impl Default for ReviewRules {
    fn default() -> Self {
        Self {
            intervals: vec![1, 3, 7, 14],
            series_length: DEFAULT_LENGTH,
        }
    }
}

/// Where a mistake came from in a series of exercises, to rebuild the same exercise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReviewOrigin {
    pub seed: u32,
    pub answer_mode: AnswerMode,
    /// The kind of exercise chosen for the series (`None`: every kind of the skill).
    pub format: Option<ExerciseFormat>,
    /// Place of the exercise in its series.
    pub index: u32,
}

/// A calculation or a word to see again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewItem {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub skill_id: String,
    /// The exercise's key: the same mistake twice is one item.
    pub key: String,
    /// The question as written (« 7 + ? = 12 ») and its answer, for the parents.
    pub prompt: String,
    pub expected: String,
    /// Spelling: the word to write again.
    pub word: Option<String>,
    pub origin: Option<ReviewOrigin>,
    /// Reviews answered right in a row (index in [`ReviewRules::intervals`]).
    pub step: u32,
    /// Local day number when it is to be seen again.
    pub due_day: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Storage of the review queue (SQLite, in `educator-data`); deleted with the profile.
pub trait ReviewStore: Send + Sync {
    /// The items of a profile, oldest first.
    fn review_items(&self, profile_id: Uuid) -> Result<Vec<ReviewItem>>;

    fn review_item(&self, id: Uuid) -> Result<Option<ReviewItem>>;

    /// Adds or replaces an item (by id).
    fn save_review_item(&self, item: &ReviewItem) -> Result<()>;

    fn delete_review_item(&self, id: Uuid) -> Result<bool>;
}

/// The review queue of the profiles.
pub struct ReviewQueue {
    store: Arc<dyn ReviewStore>,
    rules: ReviewRules,
    clock: Arc<dyn Clock>,
    date_of: DateOf,
}

impl fmt::Debug for ReviewQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReviewQueue")
            .field("rules", &self.rules)
            .finish_non_exhaustive()
    }
}

impl ReviewQueue {
    pub fn new(
        store: Arc<dyn ReviewStore>,
        rules: ReviewRules,
        clock: Arc<dyn Clock>,
        date_of: DateOf,
    ) -> Self {
        Self {
            store,
            rules,
            clock,
            date_of,
        }
    }

    pub fn rules(&self) -> &ReviewRules {
        &self.rules
    }

    fn today(&self) -> i64 {
        day_number((self.date_of)(self.clock.now_ms()))
    }

    fn interval(&self, step: u32) -> i64 {
        let index = (step as usize).min(self.rules.intervals.len() - 1);
        i64::from(self.rules.intervals[index])
    }

    /// A wrong answer outside « Mes révisions »: the item (new, or already in the queue) comes back
    /// after the first interval.
    pub fn record_mistake(
        &self,
        profile_id: Uuid,
        skill_id: &str,
        exercise: &Exercise,
        origin: Option<ReviewOrigin>,
    ) -> Result<()> {
        let now = self.clock.now_ms();
        let due_day = self.today() + self.interval(0);
        let existing = self
            .store
            .review_items(profile_id)?
            .into_iter()
            .find(|i| i.skill_id == skill_id && i.key == exercise.key);
        let item = match existing {
            Some(mut item) => {
                item.step = 0;
                item.due_day = due_day;
                item.origin = origin.or(item.origin);
                item.updated_at = now;
                item
            }
            None => ReviewItem {
                id: uuid_v7(now),
                profile_id,
                skill_id: skill_id.to_owned(),
                key: exercise.key.clone(),
                prompt: exercise.prompt(),
                expected: exercise.expected_label(),
                word: exercise.word.as_ref().map(|w| w.word.clone()),
                origin,
                step: 0,
                due_day,
                created_at: now,
                updated_at: now,
            },
        };
        self.store.save_review_item(&item)
    }

    /// An answer in « Mes révisions »: right, the item waits longer (and leaves the queue after
    /// the last step); wrong, it comes back the next day.
    pub fn record_review(&self, item_id: Uuid, correct: bool) -> Result<()> {
        let Some(mut item) = self.store.review_item(item_id)? else {
            return Ok(());
        };
        let today = self.today();
        if correct {
            item.step += 1;
            if item.step as usize >= self.rules.intervals.len() {
                self.store.delete_review_item(item.id)?;
                return Ok(());
            }
            item.due_day = today + self.interval(item.step);
        } else {
            item.step = 0;
            item.due_day = today + self.interval(0);
        }
        item.updated_at = self.clock.now_ms();
        self.store.save_review_item(&item)
    }

    /// The items to see today (or late), the most late first; `filter` keeps some skills (a subject).
    pub fn due(&self, profile_id: Uuid, filter: &dyn Fn(&str) -> bool) -> Result<Vec<ReviewItem>> {
        let today = self.today();
        let mut due: Vec<ReviewItem> = self
            .store
            .review_items(profile_id)?
            .into_iter()
            .filter(|i| i.due_day <= today && filter(&i.skill_id))
            .collect();
        due.sort_by(|a, b| {
            a.due_day
                .cmp(&b.due_day)
                .then_with(|| a.created_at.cmp(&b.created_at))
        });
        Ok(due)
    }

    /// Every item of a profile (parents).
    pub fn all(&self, profile_id: Uuid) -> Result<Vec<ReviewItem>> {
        self.store.review_items(profile_id)
    }
}

/// The exercise of a review item: the same one when it can be rebuilt, otherwise one of the same
/// skill (the same word in spelling). `listening`: without the voice, a word is not dictated.
pub fn review_exercise(
    skill: &Skill,
    item: &ReviewItem,
    seed: u64,
    listening: Listening,
) -> Result<Exercise> {
    if let Some(origin) = item.origin {
        let format_heard = origin.format.is_some_and(ExerciseFormat::needs_listening);
        if listening == Listening::Available || !format_heard {
            if let Ok(series) = generate_series_as(
                skill,
                u64::from(origin.seed),
                origin.answer_mode,
                DEFAULT_LENGTH,
                origin.format,
            ) {
                if let Some(same) = series
                    .into_iter()
                    .nth(origin.index as usize)
                    .filter(|e| e.key == item.key)
                    .filter(|e| listening == Listening::Available || !e.format.needs_listening())
                {
                    return Ok(same);
                }
            }
        }
    }

    let narrowed;
    let skill = match (&item.word, &skill.params.words) {
        (Some(word), Some(words)) if words.words.iter().any(|w| &w.word.text == word) => {
            let mut only = skill.clone();
            if let Some(set) = only.params.words.as_mut() {
                set.words.retain(|w| &w.word.text == word);
            }
            narrowed = only;
            &narrowed
        }
        _ => skill,
    };
    let answer_mode = if skill.answer_modes.contains(&AnswerMode::Keyboard) {
        AnswerMode::Keyboard
    } else {
        *skill
            .answer_modes
            .first()
            .ok_or_else(|| Error::InvalidSkill(format!("Skill {} has no answer mode.", skill.id)))?
    };
    // Without the voice, a format where the word is only heard is left out (SUB026704).
    let format = if listening == Listening::Unavailable {
        let playable: Vec<ExerciseFormat> = skill
            .formats
            .iter()
            .copied()
            .filter(|f| !f.needs_listening())
            .collect();
        if skill.formats.iter().any(|f| f.needs_listening()) {
            Some(*playable.first().ok_or_else(|| {
                Error::SeriesRequest(format!("Skill {} needs the voice.", skill.id))
            })?)
        } else {
            None
        }
    } else {
        None
    };
    let answer_mode = match format {
        Some(ExerciseFormat::ChooseSpelling) => AnswerMode::Choice,
        _ => answer_mode,
    };
    generate_series_as(skill, seed, answer_mode, 1, format)?
        .into_iter()
        .next()
        .ok_or_else(|| Error::InvalidSkill(format!("Skill {} drew nothing.", skill.id)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_are_checked() {
        assert!(ReviewRules::new(vec![], 10).is_err());
        assert!(ReviewRules::new(vec![0, 3], 10).is_err());
        assert!(ReviewRules::new(vec![3, 1], 10).is_err());
        assert!(ReviewRules::new(vec![1, 3, 7, 14], 0).is_err());
        assert!(ReviewRules::new(vec![1, 3, 7, 14], 10).is_ok());
    }
}
