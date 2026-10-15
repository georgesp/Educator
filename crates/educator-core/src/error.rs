use uuid::Uuid;

/// Errors of the domain. Messages are in English (logs, diagnostics) except
/// [`Error::ProfileValidation`], whose message is a simple French sentence shown to the child.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Invalid input on a profile; `field` is `name`, `grade`, `characterId`, `placementSeconds` or
    /// `examSeconds`.
    #[error("{message}")]
    ProfileValidation {
        field: &'static str,
        message: String,
    },

    #[error("No profile with id {0}.")]
    ProfileNotFound(Uuid),

    #[error("No series in progress with id {0}.")]
    SeriesNotFound(Uuid),

    /// A request on a series that cannot be honoured (unknown skill, bad index, series over…).
    #[error("{0}")]
    SeriesRequest(String),

    /// A request that cannot be honoured outside a series (an accessory still locked, an unknown
    /// setting…): a mistake of the front, never shown to the child.
    #[error("{0}")]
    InvalidRequest(String),

    /// A skill that cannot produce a series (rule too narrow): a content error.
    #[error("{0}")]
    InvalidSkill(String),

    #[error("No character in the content.")]
    NoCharacter,

    /// The speech engine cannot synthesize (missing files, crash, timeout).
    #[error("{0}")]
    SpeechUnavailable(String),

    /// Storage failure (SQLite), with the underlying message.
    #[error("Storage error: {0}")]
    Storage(String),
}

impl Error {
    pub fn validation(field: &'static str, message: impl Into<String>) -> Self {
        Self::ProfileValidation {
            field,
            message: message.into(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
