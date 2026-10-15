//! Helpers shared by the tests: skills written in the tests (independent of the content folder),
//! an in-memory profile store (the SQLite one is tested in educator-data) and fake characters.
#![allow(dead_code)]

use educator_core::characters::{CharacterCatalog, CharacterInfo, CharacterKind};
use educator_core::profiles::{Profile, ProfileStore};
use educator_core::sessions::{
    AnswerRecord, SessionEnd, SessionRecord, SessionStore, SessionType, SkillMastery,
};
use educator_core::skills::{
    AnswerMode, ExerciseFormat, OperandRule, Skill, SkillConstraints, SkillOperation, SkillParams,
    Subject,
};
use educator_core::{Grade, Result};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

pub fn skill(id: &str) -> Skill {
    Skill {
        id: id.to_owned(),
        subject: Subject::Math,
        grade: Grade::CP,
        period: 1,
        title: "Test".to_owned(),
        instruction: "Calcule.".to_owned(),
        operation: SkillOperation::Addition,
        left: OperandRule::range(0, 10),
        right: OperandRule::range(0, 10),
        constraints: SkillConstraints::default(),
        formats: vec![ExerciseFormat::Result, ExerciseFormat::MissingOperand],
        answer_modes: vec![AnswerMode::Keyboard, AnswerMode::Choice],
        source: "Tests".to_owned(),
        also_offered: Vec::new(),
        params: SkillParams::default(),
    }
}

pub fn test_skill() -> Skill {
    skill("test-skill")
}

#[derive(Default)]
pub struct InMemoryProfileStore {
    profiles: Mutex<HashMap<Uuid, Profile>>,
}

impl ProfileStore for InMemoryProfileStore {
    fn list(&self) -> Result<Vec<Profile>> {
        let mut all: Vec<Profile> = self.profiles.lock().unwrap().values().cloned().collect();
        all.sort_by_key(|p| std::cmp::Reverse(p.last_used_at));
        Ok(all)
    }

    fn get(&self, id: Uuid) -> Result<Option<Profile>> {
        Ok(self.profiles.lock().unwrap().get(&id).cloned())
    }

    fn name_exists(&self, normalized_name: &str, except_id: Option<Uuid>) -> Result<bool> {
        Ok(self.profiles.lock().unwrap().values().any(|p| {
            Profile::normalize_name(&p.name) == normalized_name && Some(p.id) != except_id
        }))
    }

    fn add(&self, profile: &Profile) -> Result<Profile> {
        self.profiles
            .lock()
            .unwrap()
            .insert(profile.id, profile.clone());
        Ok(profile.clone())
    }

    fn update(&self, profile: &Profile) -> Result<Option<Profile>> {
        let mut profiles = self.profiles.lock().unwrap();
        let Some(existing) = profiles.get(&profile.id) else {
            return Ok(None);
        };

        // Points are only changed through add_points, as in the SQLite store.
        let stored = Profile {
            points: existing.points,
            ..profile.clone()
        };
        profiles.insert(profile.id, stored.clone());
        Ok(Some(stored))
    }

    fn delete(&self, id: Uuid) -> Result<bool> {
        Ok(self.profiles.lock().unwrap().remove(&id).is_some())
    }

    fn add_points(&self, id: Uuid, points: i32) -> Result<Option<i32>> {
        let mut profiles = self.profiles.lock().unwrap();
        Ok(profiles.get_mut(&id).map(|p| {
            p.points += points;
            p.points
        }))
    }
}

pub struct FakeCharacters(Vec<CharacterInfo>);

impl Default for FakeCharacters {
    fn default() -> Self {
        Self(vec![
            CharacterInfo::new("lion", "Lion", CharacterKind::Animal),
            CharacterInfo::new("ours", "Ours", CharacterKind::Animal),
            CharacterInfo::new("lapin", "Lapin", CharacterKind::Animal),
        ])
    }
}

impl CharacterCatalog for FakeCharacters {
    fn all(&self) -> &[CharacterInfo] {
        &self.0
    }

    fn svg(&self, id: &str) -> Option<&str> {
        self.0.iter().any(|c| c.id == id).then_some("<svg/>")
    }
}

/// In-memory [`SessionStore`] (the SQLite one is tested in educator-data).
#[derive(Default)]
pub struct InMemorySessionStore {
    pub sessions: Mutex<Vec<SessionRecord>>,
    pub answers: Mutex<Vec<AnswerRecord>>,
    pub masteries: Mutex<Vec<SkillMastery>>,
    pub lesson_slides: Mutex<HashMap<(Uuid, String), u32>>,
}

impl SessionStore for InMemorySessionStore {
    fn add_session(&self, session: &SessionRecord) -> Result<()> {
        self.sessions.lock().unwrap().push(session.clone());
        Ok(())
    }

    fn session(&self, id: Uuid) -> Result<Option<SessionRecord>> {
        Ok(self
            .sessions
            .lock()
            .unwrap()
            .iter()
            .find(|s| s.id == id)
            .cloned())
    }

    fn add_answer(&self, answer: &AnswerRecord) -> Result<bool> {
        let mut answers = self.answers.lock().unwrap();
        if answers
            .iter()
            .any(|a| a.session_id == answer.session_id && a.index == answer.index)
        {
            return Ok(false);
        }
        answers.push(answer.clone());
        Ok(true)
    }

    fn answers(&self, session_id: Uuid) -> Result<Vec<AnswerRecord>> {
        let mut found: Vec<AnswerRecord> = self
            .answers
            .lock()
            .unwrap()
            .iter()
            .filter(|a| a.session_id == session_id)
            .cloned()
            .collect();
        found.sort_by_key(|a| a.index);
        Ok(found)
    }

    fn end_session(&self, id: Uuid, end: &SessionEnd) -> Result<bool> {
        let mut sessions = self.sessions.lock().unwrap();
        let Some(session) = sessions
            .iter_mut()
            .find(|s| s.id == id && s.ended_at.is_none())
        else {
            return Ok(false);
        };
        session.ended_at = Some(end.ended_at);
        session.abandoned = end.abandoned;
        session.question_count = end.question_count;
        session.correct_count = end.correct_count;
        session.points_earned = end.points_earned;
        Ok(true)
    }

    fn open_sessions(&self) -> Result<Vec<SessionRecord>> {
        Ok(self
            .sessions
            .lock()
            .unwrap()
            .iter()
            .filter(|s| s.ended_at.is_none())
            .cloned()
            .collect())
    }

    fn abandon_open_sessions(&self, now: i64) -> Result<usize> {
        let mut count = 0;
        for session in self.sessions.lock().unwrap().iter_mut() {
            if session.ended_at.is_none() {
                session.ended_at = Some(now);
                session.abandoned = true;
                count += 1;
            }
        }
        Ok(count)
    }

    fn recent_results(&self, profile_id: Uuid, skill_id: &str, limit: usize) -> Result<Vec<bool>> {
        let sessions = self.sessions.lock().unwrap();
        let mut answers: Vec<AnswerRecord> = self
            .answers
            .lock()
            .unwrap()
            .iter()
            .filter(|a| {
                a.skill_id == skill_id
                    && sessions.iter().any(|s| {
                        s.id == a.session_id
                            && s.profile_id == profile_id
                            // An abandoned level assessment does not count.
                            && !(s.session_type == SessionType::Placement && s.abandoned)
                    })
            })
            .cloned()
            .collect();
        answers.sort_by_key(|a| std::cmp::Reverse((a.answered_at, a.index)));
        Ok(answers.iter().take(limit).map(|a| a.is_correct).collect())
    }

    fn answer_count(&self, profile_id: Uuid, skill_id: &str) -> Result<u32> {
        Ok(self.recent_results(profile_id, skill_id, usize::MAX)?.len() as u32)
    }

    fn save_mastery(&self, mastery: &SkillMastery) -> Result<()> {
        let mut masteries = self.masteries.lock().unwrap();
        masteries
            .retain(|m| !(m.profile_id == mastery.profile_id && m.skill_id == mastery.skill_id));
        masteries.push(mastery.clone());
        Ok(())
    }

    fn masteries(&self, profile_id: Uuid) -> Result<Vec<SkillMastery>> {
        let mut found: Vec<SkillMastery> = self
            .masteries
            .lock()
            .unwrap()
            .iter()
            .filter(|m| m.profile_id == profile_id)
            .cloned()
            .collect();
        found.sort_by(|a, b| a.skill_id.cmp(&b.skill_id));
        Ok(found)
    }

    fn profile_sessions(&self, profile_id: Uuid) -> Result<Vec<SessionRecord>> {
        let mut found: Vec<SessionRecord> = self
            .sessions
            .lock()
            .unwrap()
            .iter()
            .filter(|s| {
                s.profile_id == profile_id
                    && s.ended_at.is_some()
                    && !(s.session_type == SessionType::Placement && s.abandoned)
            })
            .cloned()
            .collect();
        found.sort_by_key(|s| s.started_at);
        Ok(found)
    }

    fn profile_answers(&self, profile_id: Uuid) -> Result<Vec<AnswerRecord>> {
        let sessions = self.profile_sessions(profile_id)?;
        let mut found: Vec<AnswerRecord> = self
            .answers
            .lock()
            .unwrap()
            .iter()
            .filter(|a| sessions.iter().any(|s| s.id == a.session_id))
            .cloned()
            .collect();
        found.sort_by_key(|a| (a.answered_at, a.index));
        Ok(found)
    }

    fn best_timed_score(
        &self,
        profile_id: Uuid,
        subject: educator_core::skills::Subject,
        time_limit_seconds: u32,
    ) -> Result<Option<u32>> {
        Ok(self
            .sessions
            .lock()
            .unwrap()
            .iter()
            .filter(|s| {
                s.profile_id == profile_id
                    && s.session_type == SessionType::Timed
                    && s.subject == subject
                    && s.time_limit_seconds == Some(time_limit_seconds)
                    && s.ended_at.is_some()
                    && !s.abandoned
            })
            .map(|s| s.correct_count)
            .max())
    }

    fn lesson_slide(&self, profile_id: Uuid, lesson_id: &str) -> Result<Option<u32>> {
        Ok(self
            .lesson_slides
            .lock()
            .unwrap()
            .get(&(profile_id, lesson_id.to_owned()))
            .copied())
    }

    fn save_lesson_slide(
        &self,
        profile_id: Uuid,
        lesson_id: &str,
        slide: Option<u32>,
    ) -> Result<()> {
        let mut slides = self.lesson_slides.lock().unwrap();
        let key = (profile_id, lesson_id.to_owned());
        match slide {
            Some(slide) => slides.insert(key, slide),
            None => slides.remove(&key),
        };
        Ok(())
    }
}

/// In-memory [`educator_core::rewards::RewardStore`].
#[derive(Default)]
pub struct InMemoryRewardStore {
    pub badges: Mutex<Vec<(Uuid, educator_core::rewards::badges::EarnedBadge)>>,
    pub accessories: Mutex<Vec<(Uuid, String)>>,
    pub days: Mutex<Vec<(Uuid, educator_core::rewards::daily_goal::GoalDay)>>,
}

impl educator_core::rewards::RewardStore for InMemoryRewardStore {
    fn earned_badges(
        &self,
        profile_id: Uuid,
    ) -> Result<Vec<educator_core::rewards::badges::EarnedBadge>> {
        Ok(self
            .badges
            .lock()
            .unwrap()
            .iter()
            .filter(|(p, _)| *p == profile_id)
            .map(|(_, b)| b.clone())
            .collect())
    }

    fn add_badge(&self, profile_id: Uuid, badge_id: &str, earned_at: i64) -> Result<bool> {
        let mut badges = self.badges.lock().unwrap();
        if badges
            .iter()
            .any(|(p, b)| *p == profile_id && b.badge_id == badge_id)
        {
            return Ok(false);
        }
        badges.push((
            profile_id,
            educator_core::rewards::badges::EarnedBadge {
                badge_id: badge_id.to_owned(),
                earned_at,
            },
        ));
        Ok(true)
    }

    fn unlocked_accessories(&self, profile_id: Uuid) -> Result<Vec<String>> {
        Ok(self
            .accessories
            .lock()
            .unwrap()
            .iter()
            .filter(|(p, _)| *p == profile_id)
            .map(|(_, a)| a.clone())
            .collect())
    }

    fn unlock_accessory(&self, profile_id: Uuid, accessory_id: &str, _at: i64) -> Result<bool> {
        let mut all = self.accessories.lock().unwrap();
        if all
            .iter()
            .any(|(p, a)| *p == profile_id && a == accessory_id)
        {
            return Ok(false);
        }
        all.push((profile_id, accessory_id.to_owned()));
        Ok(true)
    }

    fn goal_days(
        &self,
        profile_id: Uuid,
    ) -> Result<Vec<educator_core::rewards::daily_goal::GoalDay>> {
        Ok(self
            .days
            .lock()
            .unwrap()
            .iter()
            .filter(|(p, _)| *p == profile_id)
            .map(|(_, d)| *d)
            .collect())
    }

    fn add_goal_day(
        &self,
        profile_id: Uuid,
        day: &educator_core::rewards::daily_goal::GoalDay,
    ) -> Result<bool> {
        let mut days = self.days.lock().unwrap();
        if days
            .iter()
            .any(|(p, d)| *p == profile_id && d.day == day.day)
        {
            return Ok(false);
        }
        days.push((profile_id, *day));
        Ok(true)
    }
}

/// In-memory [`educator_core::reviews::ReviewStore`].
#[derive(Default)]
pub struct InMemoryReviewStore {
    pub items: Mutex<Vec<educator_core::reviews::ReviewItem>>,
}

impl educator_core::reviews::ReviewStore for InMemoryReviewStore {
    fn review_items(&self, profile_id: Uuid) -> Result<Vec<educator_core::reviews::ReviewItem>> {
        Ok(self
            .items
            .lock()
            .unwrap()
            .iter()
            .filter(|i| i.profile_id == profile_id)
            .cloned()
            .collect())
    }

    fn review_item(&self, id: Uuid) -> Result<Option<educator_core::reviews::ReviewItem>> {
        Ok(self
            .items
            .lock()
            .unwrap()
            .iter()
            .find(|i| i.id == id)
            .cloned())
    }

    fn save_review_item(&self, item: &educator_core::reviews::ReviewItem) -> Result<()> {
        let mut items = self.items.lock().unwrap();
        items.retain(|i| i.id != item.id);
        items.push(item.clone());
        Ok(())
    }

    fn delete_review_item(&self, id: Uuid) -> Result<bool> {
        let mut items = self.items.lock().unwrap();
        let before = items.len();
        items.retain(|i| i.id != id);
        Ok(items.len() < before)
    }
}

/// In-memory [`educator_core::profile_settings::ProfileSettingsStore`].
#[derive(Default)]
pub struct InMemoryProfileSettingsStore {
    pub values: Mutex<Vec<(Uuid, String, String)>>,
}

impl educator_core::profile_settings::ProfileSettingsStore for InMemoryProfileSettingsStore {
    fn profile_settings(&self, profile_id: Uuid) -> Result<Vec<(String, String)>> {
        Ok(self
            .values
            .lock()
            .unwrap()
            .iter()
            .filter(|(p, _, _)| *p == profile_id)
            .map(|(_, k, v)| (k.clone(), v.clone()))
            .collect())
    }

    fn set_profile_setting(&self, profile_id: Uuid, key: &str, value: &str) -> Result<()> {
        let mut values = self.values.lock().unwrap();
        values.retain(|(p, k, _)| !(*p == profile_id && k == key));
        values.push((profile_id, key.to_owned(), value.to_owned()));
        Ok(())
    }
}
