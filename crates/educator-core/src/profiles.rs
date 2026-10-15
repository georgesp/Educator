//! Children's local profiles (SUB000102). No password: every profile is open to everyone.

use crate::characters::{CharacterCatalog, CharacterInfo};
use crate::clock::{uuid_v7, Clock};
use crate::exercises::exam::{
    DEFAULT_EXAM_SECONDS, DEFAULT_PLACEMENT_SECONDS, EXAM_SECONDS, PLACEMENT_SECONDS,
};
use crate::random::RandomSource;
use crate::{Error, Grade, Result};
use std::fmt;
use std::sync::Arc;
use uuid::Uuid;

pub const MAX_NAME_LENGTH: usize = 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    pub id: Uuid,
    pub name: String,
    pub grade: Grade,
    /// Id from `characters.json`; becomes the avatar configuration in lot 3.
    pub character_id: String,
    pub points: i32,
    /// Seconds given for each question of the level assessment ([`PLACEMENT_SECONDS`]).
    pub placement_seconds: u32,
    /// Seconds given for each question of an exam ([`EXAM_SECONDS`]); 0 is no timer.
    pub exam_seconds: u32,
    /// UTC Unix milliseconds, filled by the store.
    pub created_at: i64,
    pub updated_at: i64,
    pub last_used_at: i64,
}

impl Profile {
    /// Key of the case-insensitive uniqueness of names.
    pub fn normalize_name(name: &str) -> String {
        Self::clean_name(name).to_uppercase()
    }

    /// Name as stored: trimmed, inner spaces collapsed.
    pub fn clean_name(name: &str) -> String {
        name.split_whitespace().collect::<Vec<_>>().join(" ")
    }
}

/// Storage of the profiles (SQLite, in `educator-data`).
pub trait ProfileStore: Send + Sync {
    /// Every profile, most recently used first.
    fn list(&self) -> Result<Vec<Profile>>;

    fn get(&self, id: Uuid) -> Result<Option<Profile>>;

    /// Whether another profile already has this normalized name ([`Profile::normalize_name`]).
    fn name_exists(&self, normalized_name: &str, except_id: Option<Uuid>) -> Result<bool>;

    /// Adds the profile and returns it as stored (timestamps filled). A name already taken (race
    /// lost against another request) is an [`Error::ProfileValidation`] on `name`.
    fn add(&self, profile: &Profile) -> Result<Profile>;

    /// Saves name, grade, character and last use (never the points); returns the stored profile,
    /// or `None` when unknown.
    fn update(&self, profile: &Profile) -> Result<Option<Profile>>;

    fn delete(&self, id: Uuid) -> Result<bool>;

    /// Adds points atomically; returns the new total, or `None` when the profile is unknown.
    fn add_points(&self, id: Uuid, points: i32) -> Result<Option<i32>>;
}

pub const NAME_REQUIRED: &str = "Écris ton prénom.";
pub const NAME_TAKEN: &str = "Ce prénom est déjà pris. Choisis-en un autre.";
pub const GRADE_REQUIRED: &str = "Choisis ta classe.";
pub const UNKNOWN_CHARACTER: &str = "Ce personnage n'existe pas.";
pub const UNKNOWN_TIMER: &str = "Cette durée n'est pas proposée.";

pub fn name_too_long() -> String {
    format!("Ton prénom est trop long : {MAX_NAME_LENGTH} lettres au maximum.")
}

/// Parses a grade received from the front; an unknown one reads « Choisis ta classe. ».
pub fn parse_grade(value: &str) -> Result<Grade> {
    value
        .parse()
        .map_err(|_| Error::validation("grade", GRADE_REQUIRED))
}

/// Profiles use cases (SUB000102): validation of names (French messages shown to the child),
/// random character at creation and on demand, last use.
pub struct ProfileService {
    store: Arc<dyn ProfileStore>,
    characters: Arc<dyn CharacterCatalog>,
    random: Arc<dyn RandomSource>,
    clock: Arc<dyn Clock>,
}

impl fmt::Debug for ProfileService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProfileService").finish_non_exhaustive()
    }
}

impl ProfileService {
    pub fn new(
        store: Arc<dyn ProfileStore>,
        characters: Arc<dyn CharacterCatalog>,
        random: Arc<dyn RandomSource>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            store,
            characters,
            random,
            clock,
        }
    }

    pub fn list(&self) -> Result<Vec<Profile>> {
        self.store.list()
    }

    pub fn get(&self, id: Uuid) -> Result<Profile> {
        self.store.get(id)?.ok_or(Error::ProfileNotFound(id))
    }

    /// `character_id`: character shown on the creation screen (drawn by [`Self::draw_character`]);
    /// drawn now when `None`.
    pub fn create(&self, name: &str, grade: Grade, character_id: Option<&str>) -> Result<Profile> {
        let clean_name = self.validate_name(name, None)?;
        let character = match character_id {
            None => self.draw_character(None)?.id,
            Some(id) => self.validate_character(id)?,
        };
        let now = self.clock.now_ms();
        self.store.add(&Profile {
            id: uuid_v7(now),
            name: clean_name,
            grade,
            character_id: character,
            points: 0,
            placement_seconds: DEFAULT_PLACEMENT_SECONDS,
            exam_seconds: DEFAULT_EXAM_SECONDS,
            created_at: 0,
            updated_at: 0,
            last_used_at: now,
        })
    }

    /// Only the given fields change.
    pub fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        grade: Option<Grade>,
        character_id: Option<&str>,
    ) -> Result<Profile> {
        let mut profile = self.get(id)?;
        if let Some(name) = name {
            profile.name = self.validate_name(name, Some(id))?;
        }

        if let Some(grade) = grade {
            profile.grade = grade;
        }

        if let Some(character_id) = character_id {
            profile.character_id = self.validate_character(character_id)?;
        }

        self.save(&profile)
    }

    /// Changes the seconds given per question to the level assessment and to the exams of a
    /// profile; only the given ones change. A duration that is not offered is refused.
    pub fn update_timers(
        &self,
        id: Uuid,
        placement_seconds: Option<u32>,
        exam_seconds: Option<u32>,
    ) -> Result<Profile> {
        let mut profile = self.get(id)?;
        if let Some(seconds) = placement_seconds {
            if !PLACEMENT_SECONDS.contains(&seconds) {
                return Err(Error::validation("placementSeconds", UNKNOWN_TIMER));
            }

            profile.placement_seconds = seconds;
        }

        if let Some(seconds) = exam_seconds {
            if !EXAM_SECONDS.contains(&seconds) {
                return Err(Error::validation("examSeconds", UNKNOWN_TIMER));
            }

            profile.exam_seconds = seconds;
        }

        self.save(&profile)
    }

    /// New random character, different from the current one.
    pub fn reroll_character(&self, id: Uuid) -> Result<Profile> {
        let mut profile = self.get(id)?;
        profile.character_id = self.draw_character(Some(&profile.character_id))?.id;
        self.save(&profile)
    }

    /// The child opens this profile: updates the last use.
    pub fn select(&self, id: Uuid) -> Result<Profile> {
        let mut profile = self.get(id)?;
        profile.last_used_at = self.clock.now_ms();
        self.save(&profile)
    }

    pub fn delete(&self, id: Uuid) -> Result<()> {
        if self.store.delete(id)? {
            Ok(())
        } else {
            Err(Error::ProfileNotFound(id))
        }
    }

    /// A character at random, different from `except` when there is a choice.
    pub fn draw_character(&self, except: Option<&str>) -> Result<CharacterInfo> {
        let all = self.characters.all();
        let candidates: Vec<&CharacterInfo> = all
            .iter()
            .filter(|c| Some(c.id.as_str()) != except)
            .collect();
        if candidates.is_empty() {
            return all.first().cloned().ok_or(Error::NoCharacter);
        }

        let count = u32::try_from(candidates.len()).unwrap_or(u32::MAX);
        Ok(candidates[self.random.next(count) as usize].clone())
    }

    fn save(&self, profile: &Profile) -> Result<Profile> {
        self.store
            .update(profile)?
            .ok_or(Error::ProfileNotFound(profile.id))
    }

    fn validate_name(&self, name: &str, except_id: Option<Uuid>) -> Result<String> {
        let clean = Profile::clean_name(name);
        if clean.is_empty() {
            return Err(Error::validation("name", NAME_REQUIRED));
        }

        // UTF-16 length, like the front's input and the C# version.
        if clean.encode_utf16().count() > MAX_NAME_LENGTH {
            return Err(Error::validation("name", name_too_long()));
        }

        if self
            .store
            .name_exists(&Profile::normalize_name(&clean), except_id)?
        {
            return Err(Error::validation("name", NAME_TAKEN));
        }

        Ok(clean)
    }

    fn validate_character(&self, character_id: &str) -> Result<String> {
        if self.characters.all().iter().any(|c| c.id == character_id) {
            Ok(character_id.to_owned())
        } else {
            Err(Error::validation("characterId", UNKNOWN_CHARACTER))
        }
    }
}
