//! The parents' card of the profile choice (SUB001510): a name and a character, created with
//! default values and editable in the parents' space. Not a child profile: no grade, no points, no
//! sessions. Stored in the application settings ([`AppSettingsStore`]).

use crate::characters::CharacterCatalog;
use crate::profiles::{name_too_long, Profile, MAX_NAME_LENGTH, UNKNOWN_CHARACTER};
use crate::settings::AppSettingsStore;
use crate::{Error, Result};
use std::fmt;
use std::sync::Arc;

pub const DEFAULT_PARENT_NAME: &str = "Parents";

/// Character of the parents' card until they choose another one (the first character of the
/// content when this one is missing).
pub const DEFAULT_PARENT_CHARACTER: &str = "ours";

pub const PARENT_NAME_REQUIRED: &str = "Écris un nom pour la carte des parents.";

const NAME_KEY: &str = "ParentProfile.Name";
const CHARACTER_KEY: &str = "ParentProfile.CharacterId";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentProfile {
    pub name: String,
    pub character_id: String,
}

pub struct ParentProfileService {
    settings: Arc<dyn AppSettingsStore>,
    characters: Arc<dyn CharacterCatalog>,
}

impl fmt::Debug for ParentProfileService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParentProfileService")
            .finish_non_exhaustive()
    }
}

impl ParentProfileService {
    pub fn new(settings: Arc<dyn AppSettingsStore>, characters: Arc<dyn CharacterCatalog>) -> Self {
        Self {
            settings,
            characters,
        }
    }

    /// The parents' card; the default values are stored the first time, and a character no longer
    /// in the content is replaced by the default one.
    pub fn get(&self) -> Result<ParentProfile> {
        let name = match self.settings.get(NAME_KEY)? {
            Some(name) if !name.is_empty() => name,
            _ => {
                self.settings.set(NAME_KEY, DEFAULT_PARENT_NAME)?;
                DEFAULT_PARENT_NAME.to_owned()
            }
        };
        let character_id = match self.settings.get(CHARACTER_KEY)? {
            Some(id) if self.is_character(&id) => id,
            _ => {
                let id = self.default_character()?;
                self.settings.set(CHARACTER_KEY, &id)?;
                id
            }
        };
        Ok(ParentProfile { name, character_id })
    }

    /// Only the given fields change. The name follows the rules of the children's names (trimmed,
    /// 20 letters at most), without the uniqueness: the parents' card is apart.
    pub fn update(&self, name: Option<&str>, character_id: Option<&str>) -> Result<ParentProfile> {
        let name = name.map(validate_name).transpose()?;
        if let Some(id) = character_id {
            if !self.is_character(id) {
                return Err(Error::validation("characterId", UNKNOWN_CHARACTER));
            }
        }

        if let Some(name) = name {
            self.settings.set(NAME_KEY, &name)?;
        }
        if let Some(id) = character_id {
            self.settings.set(CHARACTER_KEY, id)?;
        }
        self.get()
    }

    fn is_character(&self, id: &str) -> bool {
        self.characters.all().iter().any(|c| c.id == id)
    }

    fn default_character(&self) -> Result<String> {
        if self.is_character(DEFAULT_PARENT_CHARACTER) {
            return Ok(DEFAULT_PARENT_CHARACTER.to_owned());
        }
        self.characters
            .all()
            .first()
            .map(|c| c.id.clone())
            .ok_or(Error::NoCharacter)
    }
}

fn validate_name(name: &str) -> Result<String> {
    let clean = Profile::clean_name(name);
    if clean.is_empty() {
        return Err(Error::validation("name", PARENT_NAME_REQUIRED));
    }
    // UTF-16 length, like the front's input and the children's names.
    if clean.encode_utf16().count() > MAX_NAME_LENGTH {
        return Err(Error::validation("name", name_too_long()));
    }
    Ok(clean)
}
