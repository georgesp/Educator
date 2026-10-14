//! The parents' card of the profile choice (SUB001510).

use educator_core::characters::{CharacterCatalog, CharacterInfo, CharacterKind};
use educator_core::parent_profile::{
    ParentProfile, ParentProfileService, DEFAULT_PARENT_NAME, PARENT_NAME_REQUIRED,
};
use educator_core::settings::AppSettingsStore;
use educator_core::{Error, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct Settings(Mutex<HashMap<String, String>>);

impl AppSettingsStore for Settings {
    fn get(&self, key: &str) -> Result<Option<String>> {
        Ok(self.0.lock().unwrap().get(key).cloned())
    }

    fn set(&self, key: &str, value: &str) -> Result<()> {
        self.0
            .lock()
            .unwrap()
            .insert(key.to_owned(), value.to_owned());
        Ok(())
    }

    fn remove(&self, key: &str) -> Result<bool> {
        Ok(self.0.lock().unwrap().remove(key).is_some())
    }
}

struct Characters(Vec<CharacterInfo>);

impl CharacterCatalog for Characters {
    fn all(&self) -> &[CharacterInfo] {
        &self.0
    }

    fn svg(&self, _id: &str) -> Option<&str> {
        None
    }
}

fn characters(ids: &[&str]) -> Arc<Characters> {
    Arc::new(Characters(
        ids.iter()
            .map(|id| CharacterInfo::new(*id, *id, CharacterKind::Animal))
            .collect(),
    ))
}

fn service(settings: Arc<Settings>, ids: &[&str]) -> ParentProfileService {
    ParentProfileService::new(settings, characters(ids))
}

#[test]
fn the_card_is_created_with_the_default_name_and_the_bear() {
    let settings = Arc::new(Settings::default());
    let parents = service(Arc::clone(&settings), &["lion", "ours"]);

    let card = parents.get().unwrap();

    assert_eq!(
        ParentProfile {
            name: DEFAULT_PARENT_NAME.to_owned(),
            character_id: "ours".to_owned(),
        },
        card
    );
    // Stored at once: the next start finds the same card.
    assert_eq!(card, service(settings, &["lion", "ours"]).get().unwrap());
}

#[test]
fn without_the_bear_the_first_character_is_taken() {
    let parents = service(Arc::new(Settings::default()), &["lion", "renard"]);
    assert_eq!("lion", parents.get().unwrap().character_id);

    let empty = service(Arc::new(Settings::default()), &[]);
    assert_eq!(Err(Error::NoCharacter), empty.get());
}

#[test]
fn the_name_and_the_character_change_one_at_a_time() {
    let settings = Arc::new(Settings::default());
    let parents = service(Arc::clone(&settings), &["lion", "ours"]);

    let renamed = parents.update(Some("  Papa   et  Maman "), None).unwrap();
    assert_eq!("Papa et Maman", renamed.name);
    assert_eq!("ours", renamed.character_id);

    let changed = parents.update(None, Some("lion")).unwrap();
    assert_eq!(
        ("Papa et Maman", "lion"),
        (changed.name.as_str(), changed.character_id.as_str())
    );
    assert_eq!(changed, service(settings, &["lion", "ours"]).get().unwrap());
}

#[test]
fn a_wrong_name_or_character_changes_nothing() {
    let parents = service(Arc::new(Settings::default()), &["lion", "ours"]);

    assert_eq!(
        Err(Error::validation("name", PARENT_NAME_REQUIRED)),
        parents.update(Some("   "), Some("lion"))
    );
    assert!(matches!(
        parents.update(Some("Un nom beaucoup trop long"), None),
        Err(Error::ProfileValidation { field: "name", .. })
    ));
    assert!(matches!(
        parents.update(Some("Papa"), Some("dragon")),
        Err(Error::ProfileValidation {
            field: "characterId",
            ..
        })
    ));
    assert_eq!(DEFAULT_PARENT_NAME, parents.get().unwrap().name);
    assert_eq!("ours", parents.get().unwrap().character_id);
}

#[test]
fn a_character_gone_from_the_content_is_replaced() {
    let settings = Arc::new(Settings::default());
    service(Arc::clone(&settings), &["lion", "ours", "dragon"])
        .update(None, Some("dragon"))
        .unwrap();

    let card = service(settings, &["lion", "ours"]).get().unwrap();

    assert_eq!("ours", card.character_id);
}
