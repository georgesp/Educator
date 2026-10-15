//! Domain of Educator (lot 1bis, port of `Educator.Core`): skills and their generation rules, the
//! exercise engine, lessons, profiles, points and the text read by the voice.
//!
//! Pure code: no file, database, process or UI access. Storage, content loading and speech
//! synthesis are traits implemented by the other crates (`educator-data`, `educator-content`).
//! The traits are synchronous: SQLite and Piper are blocking, the Tauri commands run them on
//! worker threads.

pub mod adaptive;
pub mod characters;
pub mod clock;
mod error;
pub mod exercises;
mod grade;
pub mod lessons;
pub mod mascot;
pub mod numbers;
pub mod parent_gate;
pub mod parent_profile;
pub mod points;
pub mod profile_settings;
pub mod profiles;
pub mod random;
pub mod reviews;
pub mod rewards;
pub mod sessions;
pub mod settings;
pub mod skills;
pub mod speech;
pub mod spelling;
pub mod statistics;

pub use error::{Error, Result};
pub use grade::Grade;
