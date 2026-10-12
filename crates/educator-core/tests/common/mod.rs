//! Helpers shared by the tests: skills written in the tests (independent of the content folder),
//! an in-memory profile store (the SQLite one is tested in educator-data) and fake characters.
#![allow(dead_code)]

use educator_core::characters::{CharacterCatalog, CharacterInfo, CharacterKind};
use educator_core::profiles::{Profile, ProfileStore};
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
