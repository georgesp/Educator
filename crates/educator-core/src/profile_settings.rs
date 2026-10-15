//! Settings of a child (SUB001512, SUB001513): what the child chooses in Options (sounds, speed of
//! the voice, size of the text, font for dyslexia, theme) and what the parents choose in their
//! space (goal of the day, timed challenge, advised length of a session). Stored per profile, key
//! by key (table `ProfileSetting`): a missing or unknown value reads as its default.

use crate::exercises::exam::TIMED_DURATIONS;
use crate::rewards::daily_goal::{DailyGoalRules, GOAL_CHOICES};
use crate::speech::speed;
use crate::{Error, Result};
use uuid::Uuid;

/// Storage of the settings of the profiles (SQLite, in `educator-data`); deleted with the profile.
pub trait ProfileSettingsStore: Send + Sync {
    /// Every `(key, value)` of a profile.
    fn profile_settings(&self, profile_id: Uuid) -> Result<Vec<(String, String)>>;

    fn set_profile_setting(&self, profile_id: Uuid, key: &str, value: &str) -> Result<()>;
}

/// Speed of the voice chosen by the child, around the speeds chosen for the children.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VoiceSpeed {
    Slower,
    #[default]
    Normal,
    Faster,
}

impl VoiceSpeed {
    pub const ALL: [VoiceSpeed; 3] = [VoiceSpeed::Slower, VoiceSpeed::Normal, VoiceSpeed::Faster];

    pub fn as_str(self) -> &'static str {
        match self {
            VoiceSpeed::Slower => "slower",
            VoiceSpeed::Normal => "normal",
            VoiceSpeed::Faster => "faster",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|s| s.as_str() == name)
    }

    /// A speed of the voice (instructions, dictated words) adjusted to the choice of the child.
    /// « Plus lentement » ([`speed::SLOW`]) stays as it is: it is already the slowest.
    pub fn apply(self, base: f64) -> f64 {
        if base <= speed::SLOW {
            return base;
        }
        let adjusted = match self {
            VoiceSpeed::Slower => base - 0.1,
            VoiceSpeed::Normal => base,
            VoiceSpeed::Faster => base + 0.1,
        };
        // Two decimals: the same speed always reaches the same Piper process and cache entry.
        ((adjusted * 100.0).round() / 100.0).clamp(speed::MIN, speed::MAX)
    }
}

/// Size of the texts (the whole interface is zoomed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextSize {
    #[default]
    Normal,
    Large,
    Larger,
}

impl TextSize {
    pub const ALL: [TextSize; 3] = [TextSize::Normal, TextSize::Large, TextSize::Larger];

    pub fn as_str(self) -> &'static str {
        match self {
            TextSize::Normal => "normal",
            TextSize::Large => "large",
            TextSize::Larger => "larger",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|s| s.as_str() == name)
    }
}

/// Advised length of a session (minutes) before a gentle reminder to take a break; 0: none.
pub const SESSION_MINUTES: [u32; 5] = [0, 15, 20, 30, 45];

/// Default theme (the sport theme of lot 1, SUB000105).
pub const DEFAULT_THEME: &str = "sport";

/// A theme of the application (`content/themes/themes.json`, SUB001513): its colors, fonts and
/// patterns are a `[data-theme="id"]` block of the front's styles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    pub id: String,
    /// French name (« Grand Prix »).
    pub name: String,
    /// One sentence shown under the name.
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileSettings {
    /// Sounds and voice on.
    pub sound: bool,
    /// The mascot shown (SUB001417).
    pub mascot: bool,
    pub voice_speed: VoiceSpeed,
    pub text_size: TextSize,
    pub dyslexia_font: bool,
    /// Theme of the application (SUB001513), from `content/themes/themes.json`.
    pub theme: String,
    /// Series a day (0: no goal); the default of `content/rewards/daily-goal.json` when not chosen.
    pub daily_goal: u32,
    /// The timed challenge offered (SUB001415).
    pub timed_enabled: bool,
    /// Duration of the timed challenge chosen by the parents; 0: the child chooses.
    pub timed_seconds: u32,
    /// [`SESSION_MINUTES`].
    pub session_minutes: u32,
}

impl ProfileSettings {
    pub fn defaults(goal: &DailyGoalRules) -> Self {
        Self {
            sound: true,
            mascot: true,
            voice_speed: VoiceSpeed::Normal,
            text_size: TextSize::Normal,
            dyslexia_font: false,
            theme: DEFAULT_THEME.to_owned(),
            daily_goal: goal.default_series,
            timed_enabled: true,
            timed_seconds: 0,
            session_minutes: 0,
        }
    }

    /// The stored values over the defaults; unknown keys and values are ignored.
    pub fn from_pairs(
        pairs: &[(String, String)],
        goal: &DailyGoalRules,
        themes: &[String],
    ) -> Self {
        let mut settings = Self::defaults(goal);
        for (key, value) in pairs {
            let _ = settings.apply(key, value, themes);
        }
        settings
    }

    /// Changes one setting from its stored name and value (`Err` when either is unknown).
    pub fn apply(&mut self, key: &str, value: &str, themes: &[String]) -> Result<()> {
        let invalid = || Error::validation("settings", format!("Invalid value {value} for {key}."));
        let flag = || match value {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(invalid()),
        };
        let number = || value.parse::<u32>().map_err(|_| invalid());
        match key {
            keys::SOUND => self.sound = flag()?,
            keys::MASCOT => self.mascot = flag()?,
            keys::VOICE_SPEED => {
                self.voice_speed = VoiceSpeed::from_name(value).ok_or_else(invalid)?
            }
            keys::TEXT_SIZE => self.text_size = TextSize::from_name(value).ok_or_else(invalid)?,
            keys::DYSLEXIA_FONT => self.dyslexia_font = flag()?,
            keys::THEME => {
                if !themes.iter().any(|t| t == value) {
                    return Err(invalid());
                }
                self.theme = value.to_owned();
            }
            keys::DAILY_GOAL => {
                let goal = number()?;
                if !GOAL_CHOICES.contains(&goal) {
                    return Err(invalid());
                }
                self.daily_goal = goal;
            }
            keys::TIMED_ENABLED => self.timed_enabled = flag()?,
            keys::TIMED_SECONDS => {
                let seconds = number()?;
                if seconds != 0 && !TIMED_DURATIONS.contains(&seconds) {
                    return Err(invalid());
                }
                self.timed_seconds = seconds;
            }
            keys::SESSION_MINUTES => {
                let minutes = number()?;
                if !SESSION_MINUTES.contains(&minutes) {
                    return Err(invalid());
                }
                self.session_minutes = minutes;
            }
            _ => {
                return Err(Error::validation(
                    "settings",
                    format!("Unknown setting {key}."),
                ))
            }
        }
        Ok(())
    }

    /// Every setting as stored.
    pub fn pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            (keys::SOUND, self.sound.to_string()),
            (keys::MASCOT, self.mascot.to_string()),
            (keys::VOICE_SPEED, self.voice_speed.as_str().to_owned()),
            (keys::TEXT_SIZE, self.text_size.as_str().to_owned()),
            (keys::DYSLEXIA_FONT, self.dyslexia_font.to_string()),
            (keys::THEME, self.theme.clone()),
            (keys::DAILY_GOAL, self.daily_goal.to_string()),
            (keys::TIMED_ENABLED, self.timed_enabled.to_string()),
            (keys::TIMED_SECONDS, self.timed_seconds.to_string()),
            (keys::SESSION_MINUTES, self.session_minutes.to_string()),
        ]
    }
}

/// Stored names of the settings.
pub mod keys {
    pub const SOUND: &str = "sound";
    pub const MASCOT: &str = "mascot";
    pub const VOICE_SPEED: &str = "voiceSpeed";
    pub const TEXT_SIZE: &str = "textSize";
    pub const DYSLEXIA_FONT: &str = "dyslexiaFont";
    pub const THEME: &str = "theme";
    pub const DAILY_GOAL: &str = "dailyGoal";
    pub const TIMED_ENABLED: &str = "timedEnabled";
    pub const TIMED_SECONDS: &str = "timedSeconds";
    pub const SESSION_MINUTES: &str = "sessionMinutes";
}

/// Reads and changes the settings of a profile.
pub struct ProfileSettingsService {
    store: std::sync::Arc<dyn ProfileSettingsStore>,
    goal: DailyGoalRules,
    themes: Vec<String>,
}

impl std::fmt::Debug for ProfileSettingsService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProfileSettingsService")
            .field("themes", &self.themes)
            .finish_non_exhaustive()
    }
}

impl ProfileSettingsService {
    pub fn new(
        store: std::sync::Arc<dyn ProfileSettingsStore>,
        goal: DailyGoalRules,
        themes: Vec<String>,
    ) -> Self {
        Self {
            store,
            goal,
            themes,
        }
    }

    pub fn themes(&self) -> &[String] {
        &self.themes
    }

    pub fn get(&self, profile_id: Uuid) -> Result<ProfileSettings> {
        Ok(ProfileSettings::from_pairs(
            &self.store.profile_settings(profile_id)?,
            &self.goal,
            &self.themes,
        ))
    }

    /// Changes the given settings (all checked before any is saved).
    pub fn update(
        &self,
        profile_id: Uuid,
        changes: &[(String, String)],
    ) -> Result<ProfileSettings> {
        let mut settings = self.get(profile_id)?;
        for (key, value) in changes {
            settings.apply(key, value, &self.themes)?;
        }
        for (key, value) in changes {
            self.store.set_profile_setting(profile_id, key, value)?;
        }
        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_are_read_over_the_defaults_and_checked() {
        let themes = vec!["sport".to_owned(), "cosmos".to_owned()];
        let pairs = vec![
            ("theme".to_owned(), "cosmos".to_owned()),
            ("dailyGoal".to_owned(), "5".to_owned()),
            ("textSize".to_owned(), "huge".to_owned()),
            ("unknown".to_owned(), "1".to_owned()),
        ];
        let settings = ProfileSettings::from_pairs(&pairs, &DailyGoalRules::DEFAULT, &themes);
        assert_eq!("cosmos", settings.theme);
        assert_eq!(5, settings.daily_goal);
        assert_eq!(TextSize::Normal, settings.text_size);

        let mut changed = settings.clone();
        assert!(changed.apply("theme", "jungle", &themes).is_err());
        assert!(changed.apply("dailyGoal", "50", &themes).is_err());
        assert!(changed.apply("timedSeconds", "45", &themes).is_err());
        assert!(changed.apply("timedSeconds", "120", &themes).is_ok());
        assert_eq!(120, changed.timed_seconds);
    }

    #[test]
    fn the_voice_speed_moves_the_speeds_but_not_the_slow_listening() {
        assert_eq!(0.8, VoiceSpeed::Slower.apply(speed::INSTRUCTIONS));
        assert_eq!(0.95, VoiceSpeed::Faster.apply(speed::WORDS));
        assert_eq!(0.85, VoiceSpeed::Normal.apply(speed::WORDS));
        assert_eq!(speed::SLOW, VoiceSpeed::Faster.apply(speed::SLOW));
    }
}
