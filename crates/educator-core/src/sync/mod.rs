//! Synchronisation of the data between the computer and the iPad (lot tablette 2 — US0276,
//! ADR 0007). Pure code: the clock ([`Hlc`]), what is exchanged ([`ChangeBatch`]), the merge engine
//! ([`merge`]), the points and masteries recomputed after a merge, the question « Est-ce le même
//! enfant ? » ([`name_conflicts`]) and the storage the synchronisation needs ([`SyncStore`],
//! SQLite in `educator-data`). The network is in `educator-sync`.

mod changes;
mod hlc;
mod merge;

pub use changes::*;
pub use hlc::Hlc;
pub use merge::{distinct_name, merge, merge_profile, short, LocalView, MergePlan, PlannedRemoval};

use crate::profiles::Profile;
use crate::rewards::daily_goal::GoalDay;
use crate::rewards::RewardStore;
use crate::sessions::{update_masteries, SessionRecord, SessionStore};
use crate::Result;
use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

/// Version of the exchange between two devices: a different one is refused with a simple message.
pub const PROTOCOL_VERSION: u32 = 1;

/// Sessions sent in one batch at most (with their answers).
pub const MAX_BATCH_SESSIONS: usize = 200;

/// Records sent in one batch at most (answers included).
pub const MAX_BATCH_ITEMS: usize = 2_000;

/// The points of a profile (ADR 0007, section 6): the points of its closed sessions and the bonuses
/// of the days it reached its goal. Always equal to what `add_points` accumulated.
pub fn profile_points(sessions: &[SessionRecord], goal_days: &[GoalDay]) -> i32 {
    let earned: i64 = sessions
        .iter()
        .filter(|s| s.ended_at.is_some())
        .map(|s| i64::from(s.points_earned.max(0)))
        .sum();
    let bonus: i64 = goal_days
        .iter()
        .map(|d| i64::from(d.bonus_points.max(0)))
        .sum();
    i32::try_from(earned + bonus).unwrap_or(i32::MAX)
}

/// After a merge: the masteries of every skill the profile practised ([`update_masteries`]) and
/// its points ([`profile_points`]).
pub fn refresh_profile(
    sessions: &dyn SessionStore,
    rewards: &dyn RewardStore,
    sync: &dyn SyncStore,
    profile_id: Uuid,
    now: i64,
) -> Result<()> {
    let skills: BTreeSet<String> = sessions
        .profile_answers(profile_id)?
        .into_iter()
        .map(|a| a.skill_id)
        .collect();
    let skills: Vec<String> = skills.into_iter().collect();
    update_masteries(sessions, profile_id, &skills, now)?;
    let points = profile_points(
        &sessions.profile_sessions(profile_id)?,
        &rewards.goal_days(profile_id)?,
    );
    sync.set_points(profile_id, points)
}

/// Recomputes what a merge left to recompute (also at start-up: a merge interrupted after its
/// transaction is finished then). Returns how many profiles.
pub fn refresh_pending(
    sessions: &dyn SessionStore,
    rewards: &dyn RewardStore,
    sync: &dyn SyncStore,
    now: i64,
) -> Result<usize> {
    let pending = sync.pending_refresh()?;
    for profile_id in &pending {
        if sync.profile_exists(*profile_id)? {
            refresh_profile(sessions, rewards, sync, *profile_id, now)?;
        }
        sync.clear_refresh(*profile_id)?;
    }
    Ok(pending.len())
}

/// A profile of this device and a profile of the other one with the same name but two identifiers:
/// the parent says whether it is the same child (ADR 0007, section 2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameConflict {
    pub local_id: Uuid,
    pub local_name: String,
    pub remote: ProfileState,
}

/// The conflicts between the local profiles and those of the other device.
pub fn name_conflicts(local: &[(Uuid, String)], remote: &[ProfileState]) -> Vec<NameConflict> {
    let mut conflicts = Vec::new();
    for (id, name) in local {
        let normalized = Profile::normalize_name(name);
        if let Some(other) = remote
            .iter()
            .find(|r| r.id != *id && Profile::normalize_name(&r.name) == normalized)
        {
            if remote.iter().any(|r| r.id == *id) {
                // Already shared: a rename on one side, not two children.
                continue;
            }
            conflicts.push(NameConflict {
                local_id: *id,
                local_name: name.clone(),
                remote: other.clone(),
            });
        }
    }
    conflicts
}

/// The role of a paired device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PeerRole {
    /// A tablet paired with this computer (server side).
    Tablet,
    /// The computer this tablet is paired with (client side).
    Computer,
}

impl PeerRole {
    pub fn as_str(self) -> &'static str {
        match self {
            PeerRole::Tablet => "Tablet",
            PeerRole::Computer => "Computer",
        }
    }
}

impl FromStr for PeerRole {
    type Err = String;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value {
            "Tablet" => Ok(PeerRole::Tablet),
            "Computer" => Ok(PeerRole::Computer),
            _ => Err(format!("Unknown peer role '{value}'.")),
        }
    }
}

/// A paired device (table `SyncPeer`).
#[derive(Clone, PartialEq, Eq)]
pub struct SyncPeer {
    pub device_id: Uuid,
    pub role: PeerRole,
    /// Neutral name chosen by the parent (« Tablette 1 », « Ordinateur »): never a child's name.
    pub name: String,
    /// Computer side: SHA-256 of the token of the tablet (the token itself is never kept).
    pub token_hash: Option<String>,
    /// Tablet side: the token presented to the computer.
    pub token: Option<String>,
    /// Tablet side: SHA-256 of the certificate of the computer (pinned).
    pub fingerprint: Option<String>,
    /// Tablet side: the last known addresses and port of the computer.
    pub addresses: Vec<String>,
    pub port: Option<u16>,
    pub paired_at: i64,
    pub last_sync_at: Option<i64>,
    /// Tablet side: last local `Seq` acknowledged by the computer.
    pub pushed_seq: i64,
    /// Tablet side: last `Seq` of the computer received and saved.
    pub pulled_seq: i64,
    pub revoked: bool,
    /// Tablet side: the « epoch » of the computer seen at the last synchronisation. It changes when
    /// the computer restores a backup: everything is then exchanged again (ADR 0007, section 3).
    pub epoch: Option<String>,
}

impl fmt::Debug for SyncPeer {
    /// Without the token: it never reaches the logs.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SyncPeer")
            .field("device", &short(self.device_id))
            .field("role", &self.role)
            .field("revoked", &self.revoked)
            .finish_non_exhaustive()
    }
}

/// What to export: the changes after `after_seq`, except those made by `exclude_device` (it has
/// them already), in a batch of bounded size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExportRequest {
    pub after_seq: i64,
    pub exclude_device: Option<Uuid>,
    pub max_sessions: usize,
    pub max_items: usize,
}

impl ExportRequest {
    /// From `after_seq`; a cursor at 0 (first exchange, or everything again after a restore) sends
    /// everything, even what `exclude_device` already has: the merge ignores what it knows.
    pub fn after(after_seq: i64, exclude_device: Option<Uuid>) -> Self {
        let exclude_device = if after_seq == 0 { None } else { exclude_device };
        Self {
            after_seq,
            exclude_device,
            max_sessions: MAX_BATCH_SESSIONS,
            max_items: MAX_BATCH_ITEMS,
        }
    }
}

/// A page of changes. `last_seq`: the cursor to ask from next time; `more`: there is more.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExportPage {
    pub batch: ChangeBatch,
    pub last_seq: i64,
    pub more: bool,
}

/// What applying a batch did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ApplyReport {
    pub sessions_added: usize,
    pub profiles_written: usize,
    pub profiles_removed: usize,
    pub touched_profiles: Vec<Uuid>,
}

/// Storage of the synchronisation (SQLite, in `educator-data`).
pub trait SyncStore: Send + Sync {
    /// The identifier of this device (created with the database, never copied by a backup).
    fn device_id(&self) -> Result<Uuid>;

    /// The state of every local profile, with its clocks.
    fn profile_states(&self) -> Result<Vec<ProfileState>>;

    fn profile_exists(&self, id: Uuid) -> Result<bool>;

    /// The changes to send ([`ExportRequest`]).
    fn export(&self, request: &ExportRequest) -> Result<ExportPage>;

    /// Merges a batch ([`merge`]) and applies it in one transaction: all or nothing. `pulled`:
    /// the cursor of a paired device saved in the same transaction (`(device, seq)`). The profiles
    /// touched are left to recompute ([`refresh_pending`]).
    fn apply(&self, batch: &ChangeBatch, pulled: Option<(Uuid, i64)>) -> Result<ApplyReport>;

    /// « Same child »: the local profile `local_id` joins `remote` (state of the other device),
    /// with everything it has; a removal « merged into » is left for the other device.
    fn link_profile(&self, local_id: Uuid, remote: &ProfileState) -> Result<ApplyReport>;

    /// Profiles whose masteries and points are to be recomputed.
    fn pending_refresh(&self) -> Result<Vec<Uuid>>;

    fn clear_refresh(&self, profile_id: Uuid) -> Result<()>;

    /// Writes the points recomputed by [`profile_points`] (not a change to send).
    fn set_points(&self, profile_id: Uuid, points: i32) -> Result<()>;

    fn peers(&self) -> Result<Vec<SyncPeer>>;

    fn peer(&self, device_id: Uuid) -> Result<Option<SyncPeer>>;

    /// Adds or replaces a paired device.
    fn save_peer(&self, peer: &SyncPeer) -> Result<()>;

    fn delete_peer(&self, device_id: Uuid) -> Result<bool>;

    /// Tablet side: the computer acknowledged the local changes up to `seq`. Nothing happens when
    /// the device was forgotten in the meantime.
    fn record_push(&self, device_id: Uuid, seq: i64) -> Result<()>;

    /// A synchronisation with this device ended (and, tablet side, the addresses that answered
    /// first). Nothing happens when it was removed or forgotten in the meantime.
    fn record_sync(&self, device_id: Uuid, at: i64, addresses: Option<&[String]>) -> Result<()>;

    /// Tablet side: the computer restored a backup (another epoch): both cursors start again from 0.
    fn reset_cursors(&self, device_id: Uuid, epoch: &str) -> Result<()>;

    /// The epoch of this device: changed by every restore of a backup.
    fn epoch(&self) -> Result<String>;

    /// Whether the profile was removed here (a tombstone).
    fn is_removed(&self, profile_id: Uuid) -> Result<bool>;

    /// A value of the synchronisation of this device (certificate, port, options).
    fn value(&self, key: &str) -> Result<Option<String>>;

    fn set_value(&self, key: &str, value: &str) -> Result<()>;
}
