//! Profile characters (SUB000101), loaded from `content/characters`. Implemented in `educator-content`.

pub use crate::rewards::accessories::{CharacterAnchors, Point};

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

    /// Where the accessories attach on a character (SUB001504); the anchors of the children by
    /// default.
    fn anchors(&self, id: &str) -> Option<CharacterAnchors> {
        self.svg(id).map(|_| DEFAULT_ANCHORS)
    }
}

/// Anchors of the children characters, used when a catalog gives none.
pub const DEFAULT_ANCHORS: CharacterAnchors = CharacterAnchors {
    head_top: Point::new(128.0, 46.0),
    eyes: Point::new(128.0, 118.0),
    neck: Point::new(128.0, 180.0),
};
