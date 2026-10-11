//! Profile characters (SUB000101), loaded from `content/characters`. Implemented in `educator-content`.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterInfo {
    /// Identifier, e.g. `lion`.
    pub id: String,
    /// French name, e.g. « Lion ».
    pub name: String,
    pub kind: CharacterKind,
}

impl CharacterInfo {
    pub fn new(id: impl Into<String>, name: impl Into<String>, kind: CharacterKind) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterKind {
    Animal,
    Child,
}

pub trait CharacterCatalog: Send + Sync {
    fn all(&self) -> &[CharacterInfo];

    /// SVG source of a character, or `None` when unknown.
    fn svg(&self, id: &str) -> Option<&str>;
}
