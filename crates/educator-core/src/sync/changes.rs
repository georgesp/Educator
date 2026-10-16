//! What two devices exchange (ADR 0007, section 4): the state of the profiles with a clock per
//! field, the removals of profiles, the closed sessions with their answers, and the small keyed
//! records of a profile (lesson progress, badges, accessories, days of the goal, review items,
//! settings). Pure data: the wire format is in `educator-sync`, the storage in `educator-data`.

use super::Hlc;
use crate::profiles::Profile;
use crate::reviews::ReviewItem;
use crate::rewards::accessories::WornAccessory;
use crate::rewards::daily_goal::GoalDay;
use crate::sessions::{AnswerRecord, SessionRecord};
use crate::Grade;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

/// The kinds of records followed by the synchronisation (column `Entity` of `SyncRow`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Entity {
    Profile,
    Session,
    LessonProgress,
    Badge,
    Accessory,
    GoalDay,
    Review,
    Setting,
}

impl Entity {
    pub const ALL: [Entity; 8] = [
        Entity::Profile,
        Entity::Session,
        Entity::LessonProgress,
        Entity::Badge,
        Entity::Accessory,
        Entity::GoalDay,
        Entity::Review,
        Entity::Setting,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Entity::Profile => "Profile",
            Entity::Session => "Session",
            Entity::LessonProgress => "LessonProgress",
            Entity::Badge => "Badge",
            Entity::Accessory => "Accessory",
            Entity::GoalDay => "GoalDay",
            Entity::Review => "Review",
            Entity::Setting => "Setting",
        }
    }
}

impl FromStr for Entity {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Entity::ALL
            .into_iter()
            .find(|e| e.as_str() == value)
            .ok_or_else(|| format!("Unknown entity '{value}'."))
    }
}

/// Text of an identifier in the keys of `SyncRow`: upper case, as stored in the tables.
pub fn id_key(id: Uuid) -> String {
    id.hyphenated()
        .encode_upper(&mut Uuid::encode_buffer())
        .to_owned()
}

/// Key of a record of a profile in `SyncRow`: `<profile id>/<part>/<part>`.
pub fn record_key(profile_id: Uuid, parts: &[&str]) -> String {
    let mut key = id_key(profile_id);
    for part in parts {
        key.push('/');
        key.push_str(part);
    }
    key
}

/// The profile of a record key, and what follows it.
pub fn split_record_key(key: &str) -> Option<(Uuid, &str)> {
    let (profile, rest) = key.split_once('/')?;
    Some((Uuid::parse_str(profile).ok()?, rest))
}

/// The fields of a profile that follow « the last writer wins », each with its own clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProfileField {
    Name,
    Grade,
    /// The character and the accessories worn, together.
    Avatar,
    PlacementSeconds,
    ExamSeconds,
    /// The greatest wins (not the most recent clock).
    LastUsedAt,
}

impl ProfileField {
    pub const ALL: [ProfileField; 6] = [
        ProfileField::Name,
        ProfileField::Grade,
        ProfileField::Avatar,
        ProfileField::PlacementSeconds,
        ProfileField::ExamSeconds,
        ProfileField::LastUsedAt,
    ];

    /// Column `Field` of `SyncRow`.
    pub fn as_str(self) -> &'static str {
        match self {
            ProfileField::Name => "name",
            ProfileField::Grade => "grade",
            ProfileField::Avatar => "avatar",
            ProfileField::PlacementSeconds => "placementSeconds",
            ProfileField::ExamSeconds => "examSeconds",
            ProfileField::LastUsedAt => "lastUsedAt",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        ProfileField::ALL.into_iter().find(|f| f.as_str() == name)
    }
}

/// The clock of each field of a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfileClocks {
    pub name: Hlc,
    pub grade: Hlc,
    pub avatar: Hlc,
    pub placement_seconds: Hlc,
    pub exam_seconds: Hlc,
    pub last_used_at: Hlc,
}

impl ProfileClocks {
    /// Every field at the same clock.
    pub fn all(clock: Hlc) -> Self {
        Self {
            name: clock,
            grade: clock,
            avatar: clock,
            placement_seconds: clock,
            exam_seconds: clock,
            last_used_at: clock,
        }
    }

    pub fn get(&self, field: ProfileField) -> Hlc {
        match field {
            ProfileField::Name => self.name,
            ProfileField::Grade => self.grade,
            ProfileField::Avatar => self.avatar,
            ProfileField::PlacementSeconds => self.placement_seconds,
            ProfileField::ExamSeconds => self.exam_seconds,
            ProfileField::LastUsedAt => self.last_used_at,
        }
    }

    pub fn set(&mut self, field: ProfileField, clock: Hlc) {
        match field {
            ProfileField::Name => self.name = clock,
            ProfileField::Grade => self.grade = clock,
            ProfileField::Avatar => self.avatar = clock,
            ProfileField::PlacementSeconds => self.placement_seconds = clock,
            ProfileField::ExamSeconds => self.exam_seconds = clock,
            ProfileField::LastUsedAt => self.last_used_at = clock,
        }
    }

    pub fn max(&self) -> Hlc {
        ProfileField::ALL
            .into_iter()
            .map(|f| self.get(f))
            .max()
            .unwrap_or(Hlc::ZERO)
    }
}

/// A profile as exchanged: every synchronised field and its clock. The points are not exchanged:
/// they are recomputed from the sessions and the days of the goal ([`super::profile_points`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileState {
    pub id: Uuid,
    pub name: String,
    pub grade: Grade,
    pub character_id: String,
    pub accessories: Vec<WornAccessory>,
    pub placement_seconds: u32,
    pub exam_seconds: u32,
    /// The smallest wins.
    pub created_at: i64,
    pub last_used_at: i64,
    pub clocks: ProfileClocks,
}

impl ProfileState {
    pub fn of(profile: &Profile, clocks: ProfileClocks) -> Self {
        Self {
            id: profile.id,
            name: profile.name.clone(),
            grade: profile.grade,
            character_id: profile.character_id.clone(),
            accessories: profile.accessories.clone(),
            placement_seconds: profile.placement_seconds,
            exam_seconds: profile.exam_seconds,
            created_at: profile.created_at,
            last_used_at: profile.last_used_at,
            clocks,
        }
    }
}

/// A profile deleted on a device (tombstone). `merged_into`: the profile was the same child as
/// this one (« Est-ce le même enfant ? ») and everything it had went there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfileRemoval {
    pub profile_id: Uuid,
    pub clock: Hlc,
    pub merged_into: Option<Uuid>,
}

/// A closed session and its answers (never changed afterwards).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionChange {
    pub session: SessionRecord,
    pub answers: Vec<AnswerRecord>,
    pub clock: Hlc,
}

/// Where a profile is in a lesson; `None`: the lesson was finished (the progress is gone).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonProgressChange {
    pub profile_id: Uuid,
    pub lesson_id: String,
    pub slide: Option<u32>,
    pub clock: Hlc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BadgeChange {
    pub profile_id: Uuid,
    pub badge_id: String,
    pub earned_at: i64,
    pub clock: Hlc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessoryChange {
    pub profile_id: Uuid,
    pub accessory_id: String,
    pub unlocked_at: i64,
    pub clock: Hlc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoalDayChange {
    pub profile_id: Uuid,
    pub day: GoalDay,
    pub clock: Hlc,
}

/// An item of the review queue, by its natural key (profile, skill, key of the exercise: the
/// identifiers of the items differ between devices). `item`: `None` when it left the queue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewChange {
    pub profile_id: Uuid,
    pub skill_id: String,
    pub key: String,
    pub item: Option<ReviewItem>,
    pub clock: Hlc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingChange {
    pub profile_id: Uuid,
    pub key: String,
    pub value: String,
    pub clock: Hlc,
}

/// A batch of changes sent from one device to another. A batch always carries the state of the
/// profiles its records belong to (and of the targets of its removals).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChangeBatch {
    pub profiles: Vec<ProfileState>,
    pub removals: Vec<ProfileRemoval>,
    pub sessions: Vec<SessionChange>,
    pub lesson_progress: Vec<LessonProgressChange>,
    pub badges: Vec<BadgeChange>,
    pub accessories: Vec<AccessoryChange>,
    pub goal_days: Vec<GoalDayChange>,
    pub reviews: Vec<ReviewChange>,
    pub settings: Vec<SettingChange>,
}

impl ChangeBatch {
    /// Records of the batch (profiles, removals, sessions, answers and keyed records).
    pub fn item_count(&self) -> usize {
        self.profiles.len()
            + self.removals.len()
            + self.sessions.len()
            + self.sessions.iter().map(|s| s.answers.len()).sum::<usize>()
            + self.lesson_progress.len()
            + self.badges.len()
            + self.accessories.len()
            + self.goal_days.len()
            + self.reviews.len()
            + self.settings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.item_count() == 0
    }

    /// Every clock of the batch, to move the clock of the receiving device past them.
    pub fn max_clock(&self) -> Option<Hlc> {
        self.profiles
            .iter()
            .map(|p| p.clocks.max())
            .chain(self.removals.iter().map(|r| r.clock))
            .chain(self.sessions.iter().map(|s| s.clock))
            .chain(self.lesson_progress.iter().map(|c| c.clock))
            .chain(self.badges.iter().map(|c| c.clock))
            .chain(self.accessories.iter().map(|c| c.clock))
            .chain(self.goal_days.iter().map(|c| c.clock))
            .chain(self.reviews.iter().map(|c| c.clock))
            .chain(self.settings.iter().map(|c| c.clock))
            .max()
    }
}

impl fmt::Display for ChangeBatch {
    /// Counts only: never a name (logs, ADR 0007).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} profile(s), {} removal(s), {} session(s), {} other record(s)",
            self.profiles.len(),
            self.removals.len(),
            self.sessions.len(),
            self.lesson_progress.len()
                + self.badges.len()
                + self.accessories.len()
                + self.goal_days.len()
                + self.reviews.len()
                + self.settings.len()
        )
    }
}
