//! Domain of Educator (lot 1bis, port of `Educator.Core`): skills and their generation rules, the
//! exercise engine, lessons, profiles, points and the text read by the voice.
//!
//! Pure code: no file, database, process or UI access. Storage, content loading and speech
//! synthesis are traits implemented by the other crates (`educator-data`, `educator-content`).
//! The traits are synchronous: SQLite and Piper are blocking, the Tauri commands run them on
//! worker threads.

pub mod characters;
pub mod clock;
mod error;
pub mod exercises;
mod grade;
pub mod lessons;
pub mod numbers;
pub mod points;
pub mod profiles;
pub mod random;
pub mod settings;
pub mod skills;
pub mod speech;

pub use error::{Error, Result};
pub use grade::Grade;
