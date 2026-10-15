//! Contracts of lot 3 (motivation and parents' space): levels, stars, badges, avatar accessories,
//! goal of the day, advice, reviews, settings of a child and backup.

use super::{iso_8601, Subject};
use educator_core::adaptive::{Advice, AdviceReason};
use educator_core::profile_settings::{ProfileSettings, TextSize, Theme, VoiceSpeed};
use educator_core::rewards::accessories::{
    Accessory, AccessoryCatalog, AccessorySlot, Unlock, WornAccessory,
};
use educator_core::rewards::badges::Badge;
use educator_core::rewards::daily_goal::DailyGoalProgress;
use educator_core::rewards::levels::LevelProgress;
use educator_core::rewards::{AccessoryState, BadgeState, RewardsOverview, SessionRewards};
use serde::{Deserialize, Serialize};

fn count(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

// ── Levels and goal of the day (SUB001501, SUB001507) ───────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct LevelResponse {
    pub level: u32,
    pub points: i32,
    /// Points from which the current level was reached.
    pub level_start: i32,
    /// Points from which the next level is reached.
    pub next_level_at: i32,
    /// Way done to the next level, 0 to 100.
    pub percent: u8,
}

impl From<LevelProgress> for LevelResponse {
    fn from(level: LevelProgress) -> Self {
        Self {
            level: level.level,
            points: level.points,
            level_start: level.level_start,
            next_level_at: level.next_level_at,
            percent: level.percent(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct DailyGoalResponse {
    /// Series asked today; 0 when the parents turned the goal off.
    pub target: u32,
    pub done: u32,
    pub reached: bool,
    /// Days in a row the goal was reached.
    pub streak: u32,
    pub bonus_points: i32,
}

impl From<DailyGoalProgress> for DailyGoalResponse {
    fn from(goal: DailyGoalProgress) -> Self {
        Self {
            target: goal.target,
            done: goal.done,
            reached: goal.reached,
            streak: goal.streak,
            bonus_points: goal.bonus_points,
        }
    }
}

/// The menu's panel: level, goal of the day, badges, reviews due.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct ProfileOverviewResponse {
    pub level: LevelResponse,
    pub goal: DailyGoalResponse,
    pub badges_earned: u32,
    pub badges_total: u32,
    /// Items of « Mes révisions » due today, by subject (SUB001509).
    pub reviews: Vec<ReviewCountResponse>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct ReviewCountResponse {
    pub subject: Subject,
    pub due: u32,
}

impl ProfileOverviewResponse {
    pub fn new(overview: &RewardsOverview, reviews: Vec<ReviewCountResponse>) -> Self {
        Self {
            level: overview.level.into(),
            goal: overview.goal.into(),
            badges_earned: overview.badges_earned,
            badges_total: overview.badges_total,
            reviews,
        }
    }
}

// ── Badges (SUB001503) ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct BadgeResponse {
    pub id: String,
    pub title: String,
    /// The condition said simply (shown under a badge to win).
    pub description: String,
    /// SVG source of the icon.
    pub icon: String,
    pub earned: bool,
    /// ISO 8601 instant in UTC.
    pub earned_at: Option<String>,
}

impl BadgeResponse {
    pub fn earned(badge: &Badge) -> Self {
        Self::of(badge, Some(0))
    }

    fn of(badge: &Badge, earned_at: Option<i64>) -> Self {
        Self {
            id: badge.id.clone(),
            title: badge.title.clone(),
            description: badge.description.clone(),
            icon: badge.icon.clone(),
            earned: earned_at.is_some(),
            earned_at: earned_at.filter(|at| *at > 0).map(iso_8601),
        }
    }
}

impl From<&BadgeState> for BadgeResponse {
    fn from(state: &BadgeState) -> Self {
        Self::of(&state.badge, state.earned_at)
    }
}

// ── Avatar and accessories (SUB001504, SUB001505) ───────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum AccessorySlotName {
    Head,
    Eyes,
    Neck,
    Companion,
}

impl From<AccessorySlot> for AccessorySlotName {
    fn from(slot: AccessorySlot) -> Self {
        match slot {
            AccessorySlot::Head => AccessorySlotName::Head,
            AccessorySlot::Eyes => AccessorySlotName::Eyes,
            AccessorySlot::Neck => AccessorySlotName::Neck,
            AccessorySlot::Companion => AccessorySlotName::Companion,
        }
    }
}

/// An accessory worn by an avatar, in one of its colors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct WornAccessoryResponse {
    pub accessory_id: String,
    pub color_id: String,
}

impl From<&WornAccessory> for WornAccessoryResponse {
    fn from(worn: &WornAccessory) -> Self {
        Self {
            accessory_id: worn.accessory_id.clone(),
            color_id: worn.color_id.clone(),
        }
    }
}

impl From<&WornAccessoryResponse> for WornAccessory {
    fn from(worn: &WornAccessoryResponse) -> Self {
        WornAccessory::new(&worn.accessory_id, &worn.color_id)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct AccessoryColorResponse {
    pub id: String,
    pub name: String,
    /// `#RRGGBB`, for the color button.
    pub main: String,
}

/// How an accessory is unlocked: a level, or a badge (with its title).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct UnlockResponse {
    pub level: Option<u32>,
    pub badge_id: Option<String>,
    pub badge_title: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct AccessoryResponse {
    pub id: String,
    pub name: String,
    pub slot: AccessorySlotName,
    pub colors: Vec<AccessoryColorResponse>,
    pub unlocked: bool,
    pub unlock: UnlockResponse,
}

impl AccessoryResponse {
    pub fn new(
        accessory: &Accessory,
        unlocked: bool,
        catalog: &AccessoryCatalog,
        badges: &[Badge],
    ) -> Self {
        let unlock = match &accessory.unlock {
            Unlock::Level(level) => UnlockResponse {
                level: Some(*level),
                badge_id: None,
                badge_title: None,
            },
            Unlock::Badge(id) => UnlockResponse {
                level: None,
                badge_id: Some(id.clone()),
                badge_title: badges.iter().find(|b| &b.id == id).map(|b| b.title.clone()),
            },
        };
        Self {
            id: accessory.id.clone(),
            name: accessory.name.clone(),
            slot: accessory.slot.into(),
            colors: accessory
                .colors
                .iter()
                .filter_map(|c| catalog.color(c))
                .map(|c| AccessoryColorResponse {
                    id: c.id.clone(),
                    name: c.name.clone(),
                    main: c.main.clone(),
                })
                .collect(),
            unlocked,
            unlock,
        }
    }

    pub fn of_state(state: &AccessoryState, catalog: &AccessoryCatalog, badges: &[Badge]) -> Self {
        Self::new(&state.accessory, state.unlocked, catalog, badges)
    }
}

/// The new outfit of an avatar: its character (unchanged when absent) and every accessory worn.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct UpdateAvatarRequest {
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub character_id: Option<String>,
    pub accessories: Vec<WornAccessoryResponse>,
}

// ── End of a session (SUB001501 to SUB001507) ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SessionRewardsResponse {
    /// Stars of this series or quiz; `null` for an exam, a challenge, revisions.
    pub stars: Option<u8>,
    pub previous_best_stars: u8,
    pub level_before: LevelResponse,
    pub level: LevelResponse,
    pub goal: DailyGoalResponse,
    /// Bonus of the goal of the day given now (0 otherwise).
    pub goal_bonus: i32,
    pub new_badges: Vec<BadgeResponse>,
    pub new_accessories: Vec<AccessoryResponse>,
}

impl SessionRewardsResponse {
    pub fn new(rewards: &SessionRewards, catalog: &AccessoryCatalog, badges: &[Badge]) -> Self {
        Self {
            stars: rewards.stars,
            previous_best_stars: rewards.previous_best_stars,
            level_before: rewards.level_before.into(),
            level: rewards.level.into(),
            goal: rewards.goal.into(),
            goal_bonus: rewards.goal_bonus,
            new_badges: rewards
                .new_badges
                .iter()
                .map(BadgeResponse::earned)
                .collect(),
            new_accessories: rewards
                .new_accessories
                .iter()
                .map(|a| AccessoryResponse::new(a, true, catalog, badges))
                .collect(),
        }
    }
}

// ── Advice (SUB001508) ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum AdviceReasonName {
    Start,
    Continue,
    Consolidate,
    Next,
}

impl From<AdviceReason> for AdviceReasonName {
    fn from(reason: AdviceReason) -> Self {
        match reason {
            AdviceReason::Start => AdviceReasonName::Start,
            AdviceReason::Continue => AdviceReasonName::Continue,
            AdviceReason::Consolidate => AdviceReasonName::Consolidate,
            AdviceReason::Next => AdviceReasonName::Next,
        }
    }
}

/// « Je te conseille… »: a skill of the exercises screen.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct AdviceResponse {
    pub skill_id: String,
    pub title: String,
    pub reason: AdviceReasonName,
    /// Consolidate: the title of the skill it prepares.
    pub prepares: Option<String>,
}

impl AdviceResponse {
    pub fn new(advice: &Advice, title: impl Fn(&str) -> String) -> Self {
        Self {
            title: title(&advice.skill_id),
            skill_id: advice.skill_id.clone(),
            reason: advice.reason.into(),
            prepares: advice.prepares.as_deref().map(title),
        }
    }
}

// ── Settings of a child (SUB001512, SUB001513) ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum VoiceSpeedName {
    Slower,
    Normal,
    Faster,
}

impl From<VoiceSpeed> for VoiceSpeedName {
    fn from(speed: VoiceSpeed) -> Self {
        match speed {
            VoiceSpeed::Slower => VoiceSpeedName::Slower,
            VoiceSpeed::Normal => VoiceSpeedName::Normal,
            VoiceSpeed::Faster => VoiceSpeedName::Faster,
        }
    }
}

impl From<VoiceSpeedName> for VoiceSpeed {
    fn from(speed: VoiceSpeedName) -> Self {
        match speed {
            VoiceSpeedName::Slower => VoiceSpeed::Slower,
            VoiceSpeedName::Normal => VoiceSpeed::Normal,
            VoiceSpeedName::Faster => VoiceSpeed::Faster,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum TextSizeName {
    Normal,
    Large,
    Larger,
}

impl From<TextSize> for TextSizeName {
    fn from(size: TextSize) -> Self {
        match size {
            TextSize::Normal => TextSizeName::Normal,
            TextSize::Large => TextSizeName::Large,
            TextSize::Larger => TextSizeName::Larger,
        }
    }
}

impl From<TextSizeName> for TextSize {
    fn from(size: TextSizeName) -> Self {
        match size {
            TextSizeName::Normal => TextSize::Normal,
            TextSizeName::Large => TextSize::Large,
            TextSizeName::Larger => TextSize::Larger,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct ProfileSettingsResponse {
    pub sound: bool,
    pub mascot: bool,
    pub voice_speed: VoiceSpeedName,
    pub text_size: TextSizeName,
    pub dyslexia_font: bool,
    pub theme: String,
    /// Series a day; 0: no goal.
    pub daily_goal: u32,
    pub timed_enabled: bool,
    /// Duration of the timed challenge; 0: the child chooses.
    pub timed_seconds: u32,
    /// Advised length of a session before a break reminder; 0: none.
    pub session_minutes: u32,
}

impl From<&ProfileSettings> for ProfileSettingsResponse {
    fn from(settings: &ProfileSettings) -> Self {
        Self {
            sound: settings.sound,
            mascot: settings.mascot,
            voice_speed: settings.voice_speed.into(),
            text_size: settings.text_size.into(),
            dyslexia_font: settings.dyslexia_font,
            theme: settings.theme.clone(),
            daily_goal: settings.daily_goal,
            timed_enabled: settings.timed_enabled,
            timed_seconds: settings.timed_seconds,
            session_minutes: settings.session_minutes,
        }
    }
}

/// Only the given settings change.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct UpdateProfileSettingsRequest {
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub sound: Option<bool>,
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub mascot: Option<bool>,
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub voice_speed: Option<VoiceSpeedName>,
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub text_size: Option<TextSizeName>,
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub dyslexia_font: Option<bool>,
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub theme: Option<String>,
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub daily_goal: Option<u32>,
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub timed_enabled: Option<bool>,
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub timed_seconds: Option<u32>,
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub session_minutes: Option<u32>,
}

impl UpdateProfileSettingsRequest {
    /// The changes as stored `(key, value)`.
    pub fn pairs(&self) -> Vec<(String, String)> {
        use educator_core::profile_settings::keys;
        let mut pairs = Vec::new();
        let mut push = |key: &str, value: Option<String>| {
            if let Some(value) = value {
                pairs.push((key.to_owned(), value));
            }
        };
        push(keys::SOUND, self.sound.map(|v| v.to_string()));
        push(keys::MASCOT, self.mascot.map(|v| v.to_string()));
        push(
            keys::VOICE_SPEED,
            self.voice_speed
                .map(|v| VoiceSpeed::from(v).as_str().to_owned()),
        );
        push(
            keys::TEXT_SIZE,
            self.text_size
                .map(|v| TextSize::from(v).as_str().to_owned()),
        );
        push(
            keys::DYSLEXIA_FONT,
            self.dyslexia_font.map(|v| v.to_string()),
        );
        push(keys::THEME, self.theme.clone());
        push(keys::DAILY_GOAL, self.daily_goal.map(|v| v.to_string()));
        push(
            keys::TIMED_ENABLED,
            self.timed_enabled.map(|v| v.to_string()),
        );
        push(
            keys::TIMED_SECONDS,
            self.timed_seconds.map(|v| v.to_string()),
        );
        push(
            keys::SESSION_MINUTES,
            self.session_minutes.map(|v| v.to_string()),
        );
        pairs
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct ThemeResponse {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl From<&Theme> for ThemeResponse {
    fn from(theme: &Theme) -> Self {
        Self {
            id: theme.id.clone(),
            name: theme.name.clone(),
            description: theme.description.clone(),
        }
    }
}

/// What the settings offer (the same for every profile).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SettingsOptionsResponse {
    pub themes: Vec<ThemeResponse>,
    pub goal_choices: Vec<u32>,
    /// Durations of the timed challenge the parents may fix (0: the child chooses).
    pub timed_choices: Vec<u32>,
    pub session_choices: Vec<u32>,
}

// ── Backup (SUB001512) ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct BackupResponse {
    /// Profiles in the file.
    pub profiles: u32,
    /// Where the file was written or read.
    pub path: String,
}

impl BackupResponse {
    pub fn new(profiles: usize, path: &str) -> Self {
        Self {
            profiles: count(profiles),
            path: path.to_owned(),
        }
    }
}
