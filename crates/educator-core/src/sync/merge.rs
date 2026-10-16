//! The merge engine (SUB027608, ADR 0007 section 4): pure code that reads the local state through
//! [`LocalView`] and turns a received [`ChangeBatch`] into the writes to apply ([`MergePlan`]).
//! The storage applies a plan in one transaction (`educator-data`).
//!
//! Rules:
//! - sessions and answers: union by identifier, never changed;
//! - profiles: the last writer wins field by field, `created_at` the smallest, `last_used_at` the
//!   greatest; two profiles with the same name: the one with the greater identifier is renamed
//!   « Prénom 2 » (the same choice on every device);
//! - removals win over everything; a removal « merged into » sends the records to its target;
//! - lesson progress, review items, settings: the most recent clock wins;
//! - badges, accessories, days of the goal: union, the earliest date wins.
//!
//! Applying the same batch twice changes nothing, and the order of two batches does not matter.

use super::changes::*;
use super::Hlc;
use crate::profiles::{Profile, MAX_NAME_LENGTH};
use crate::rewards::daily_goal::GoalDay;
use crate::{Error, Result};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use uuid::Uuid;

/// How the merge engine reads the local state (implemented by `educator-data` on the open
/// transaction, and in memory by the tests).
pub trait LocalView {
    /// The local profile with the clock of each field.
    fn profile(&self, id: Uuid) -> Result<Option<ProfileState>>;

    /// The tombstone of a profile removed here (or received).
    fn removal(&self, id: Uuid) -> Result<Option<ProfileRemoval>>;

    /// Identifier and name of every local profile.
    fn profile_names(&self) -> Result<Vec<(Uuid, String)>>;

    fn has_session(&self, id: Uuid) -> Result<bool>;

    /// Clock of a keyed record (lesson progress, review item, setting), removed or not.
    fn clock_of(&self, entity: Entity, key: &str) -> Result<Option<Hlc>>;

    /// When the profile earned the badge, if it did.
    fn badge(&self, profile_id: Uuid, badge_id: &str) -> Result<Option<i64>>;

    /// When the profile unlocked the accessory, if it did.
    fn accessory(&self, profile_id: Uuid, accessory_id: &str) -> Result<Option<i64>>;

    fn goal_day(&self, profile_id: Uuid, day: i64) -> Result<Option<GoalDay>>;
}

/// A removal to apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlannedRemoval {
    pub removal: ProfileRemoval,
    /// The profile exists here: it is deleted (with its records, or after moving them).
    pub exists: bool,
    /// Where its records go (« same child »), resolved and present here.
    pub move_to: Option<Uuid>,
}

/// The writes of a merge, in the order of application: profiles (final names), renames (a new
/// local clock for the name), removals, then the records. `parked`: the profiles whose name is
/// set aside first, so that the unique index of the names never fails in between.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MergePlan {
    pub parked: Vec<Uuid>,
    pub profiles: Vec<ProfileState>,
    pub renames: Vec<(Uuid, String)>,
    pub removals: Vec<PlannedRemoval>,
    pub sessions: Vec<SessionChange>,
    pub lesson_progress: Vec<LessonProgressChange>,
    pub badges: Vec<BadgeChange>,
    pub accessories: Vec<AccessoryChange>,
    pub goal_days: Vec<GoalDayChange>,
    pub reviews: Vec<ReviewChange>,
    pub settings: Vec<SettingChange>,
    /// Profiles whose masteries and points are to be recomputed.
    pub touched_profiles: Vec<Uuid>,
    /// Greatest clock received (the local clock moves past it).
    pub max_clock: Option<Hlc>,
}

impl MergePlan {
    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
            && self.renames.is_empty()
            && self.removals.is_empty()
            && self.sessions.is_empty()
            && self.lesson_progress.is_empty()
            && self.badges.is_empty()
            && self.accessories.is_empty()
            && self.goal_days.is_empty()
            && self.reviews.is_empty()
            && self.settings.is_empty()
    }
}

/// Merges a profile received with the local one: field by field.
pub fn merge_profile(local: Option<&ProfileState>, incoming: &ProfileState) -> ProfileState {
    let Some(local) = local else {
        return incoming.clone();
    };
    let mut merged = local.clone();
    let newer = |field: ProfileField| incoming.clocks.get(field) > local.clocks.get(field);
    if newer(ProfileField::Name) {
        merged.name = incoming.name.clone();
        merged.clocks.name = incoming.clocks.name;
    }
    if newer(ProfileField::Grade) {
        merged.grade = incoming.grade;
        merged.clocks.grade = incoming.clocks.grade;
    }
    if newer(ProfileField::Avatar) {
        merged.character_id = incoming.character_id.clone();
        merged.accessories = incoming.accessories.clone();
        merged.clocks.avatar = incoming.clocks.avatar;
    }
    if newer(ProfileField::PlacementSeconds) {
        merged.placement_seconds = incoming.placement_seconds;
        merged.clocks.placement_seconds = incoming.clocks.placement_seconds;
    }
    if newer(ProfileField::ExamSeconds) {
        merged.exam_seconds = incoming.exam_seconds;
        merged.clocks.exam_seconds = incoming.clocks.exam_seconds;
    }
    // The greatest instant, and with it the clock that brought it (ties: the greatest clock).
    if (incoming.last_used_at, incoming.clocks.last_used_at)
        > (local.last_used_at, local.clocks.last_used_at)
    {
        merged.last_used_at = incoming.last_used_at;
        merged.clocks.last_used_at = incoming.clocks.last_used_at;
    }
    merged.created_at = local.created_at.min(incoming.created_at);
    merged
}

/// A free name built on `base`: « Léa 2 », « Léa 3 »… within [`MAX_NAME_LENGTH`] (UTF-16, like
/// the validation of the names). `taken`: the normalized names already used.
pub fn distinct_name(base: &str, taken: &HashSet<String>) -> String {
    let base = Profile::clean_name(base);
    for n in 2u32.. {
        let suffix = format!(" {n}");
        let room = MAX_NAME_LENGTH.saturating_sub(suffix.encode_utf16().count());
        let mut stem = String::new();
        for c in base.chars() {
            if stem.encode_utf16().count() + c.len_utf16() > room {
                break;
            }
            stem.push(c);
        }
        let candidate = format!("{}{suffix}", stem.trim_end());
        if !taken.contains(&Profile::normalize_name(&candidate)) {
            return candidate;
        }
    }
    unreachable!("an unbounded range always finds a free name")
}

/// Where the records of a profile go.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Resolution {
    /// To this profile (itself, or the target of its removal).
    To(Uuid),
    /// Nowhere: the profile was removed.
    Dropped,
}

struct Merger<'a> {
    view: &'a dyn LocalView,
    /// Removals of the batch, by profile.
    incoming_removals: HashMap<Uuid, ProfileRemoval>,
    /// Profiles of the batch after merging, by identifier.
    planned: BTreeMap<Uuid, ProfileState>,
    local_profiles: HashMap<Uuid, Option<ProfileState>>,
}

impl Merger<'_> {
    fn local_profile(&mut self, id: Uuid) -> Result<Option<ProfileState>> {
        if let Some(found) = self.local_profiles.get(&id) {
            return Ok(found.clone());
        }
        let found = self.view.profile(id)?;
        self.local_profiles.insert(id, found.clone());
        Ok(found)
    }

    fn removal_of(&self, id: Uuid) -> Result<Option<ProfileRemoval>> {
        match self.view.removal(id)? {
            Some(local) => Ok(Some(local)),
            None => Ok(self.incoming_removals.get(&id).copied()),
        }
    }

    /// Follows the removals « merged into » (a few steps at most).
    fn resolve(&mut self, id: Uuid) -> Result<Resolution> {
        let mut current = id;
        for _ in 0..8 {
            match self.removal_of(current)? {
                None => return Ok(Resolution::To(current)),
                Some(removal) => match removal.merged_into {
                    Some(target) if target != current => current = target,
                    _ => return Ok(Resolution::Dropped),
                },
            }
        }
        Ok(Resolution::Dropped)
    }

    /// The profile exists here after the merge (local and not removed, or received).
    fn will_exist(&mut self, id: Uuid) -> Result<bool> {
        if self.removal_of(id)?.is_some() {
            return Ok(false);
        }
        Ok(self.planned.contains_key(&id) || self.local_profile(id)?.is_some())
    }

    /// The profile a record of `profile_id` goes to; `None`: dropped. An unknown profile is a
    /// malformed batch (a batch carries the profiles of its records).
    fn owner(&mut self, profile_id: Uuid) -> Result<Option<Uuid>> {
        match self.resolve(profile_id)? {
            Resolution::Dropped => Ok(None),
            Resolution::To(id) => {
                if self.will_exist(id)? {
                    Ok(Some(id))
                } else {
                    Err(Error::InvalidRequest(format!(
                        "Synchronisation: a record refers to an unknown profile {}.",
                        short(profile_id)
                    )))
                }
            }
        }
    }
}

/// First characters of an identifier: enough for the logs, never the whole of it (ADR 0007).
pub fn short(id: Uuid) -> String {
    id_key(id).chars().take(8).collect()
}

/// The writes that merge `batch` into the local state.
pub fn merge(view: &dyn LocalView, batch: &ChangeBatch) -> Result<MergePlan> {
    let mut incoming_removals: HashMap<Uuid, ProfileRemoval> = HashMap::new();
    for removal in &batch.removals {
        // Two removals of one profile in a batch: the greatest clock.
        let keep = incoming_removals
            .get(&removal.profile_id)
            .is_none_or(|r| removal.clock > r.clock);
        if keep {
            incoming_removals.insert(removal.profile_id, *removal);
        }
    }
    let mut merger = Merger {
        view,
        incoming_removals,
        planned: BTreeMap::new(),
        local_profiles: HashMap::new(),
    };
    let mut plan = MergePlan {
        max_clock: batch.max_clock(),
        ..MergePlan::default()
    };
    let mut touched: BTreeSet<Uuid> = BTreeSet::new();

    // Profiles: field by field, unless removed (here or in the batch).
    for incoming in &batch.profiles {
        if merger.removal_of(incoming.id)?.is_some() {
            continue;
        }
        let base = match merger.planned.get(&incoming.id) {
            Some(planned) => Some(planned.clone()),
            None => merger.local_profile(incoming.id)?,
        };
        let merged = merge_profile(base.as_ref(), incoming);
        merger.planned.insert(incoming.id, merged);
    }
    // Only what changes is written.
    let mut changed: BTreeMap<Uuid, ProfileState> = BTreeMap::new();
    let planned: Vec<ProfileState> = merger.planned.values().cloned().collect();
    for state in planned {
        if merger.local_profile(state.id)?.as_ref() != Some(&state) {
            changed.insert(state.id, state);
        }
    }

    // Removals.
    let mut removals: Vec<ProfileRemoval> = merger.incoming_removals.values().copied().collect();
    removals.sort_by_key(|r| r.profile_id);
    for removal in removals {
        if view.removal(removal.profile_id)?.is_some() {
            continue;
        }
        let exists = merger.local_profile(removal.profile_id)?.is_some();
        let move_to = match removal.merged_into {
            Some(target) if exists => match merger.resolve(target)? {
                Resolution::To(id) if merger.will_exist(id)? => Some(id),
                _ => None,
            },
            _ => None,
        };
        if let Some(target) = move_to {
            touched.insert(target);
        }
        plan.removals.push(PlannedRemoval {
            removal,
            exists,
            move_to,
        });
    }

    // Names: one profile per normalized name; the greater identifier is renamed.
    let removed_here: HashSet<Uuid> = plan
        .removals
        .iter()
        .filter(|r| r.exists)
        .map(|r| r.removal.profile_id)
        .collect();
    let mut names: BTreeMap<Uuid, String> = BTreeMap::new();
    for (id, name) in view.profile_names()? {
        if !removed_here.contains(&id) {
            names.insert(id, name);
        }
    }
    for (id, state) in &merger.planned {
        names.insert(*id, state.name.clone());
    }
    let mut groups: BTreeMap<String, Vec<Uuid>> = BTreeMap::new();
    for (id, name) in &names {
        groups
            .entry(Profile::normalize_name(name))
            .or_default()
            .push(*id);
    }
    let mut taken: HashSet<String> = groups.keys().cloned().collect();
    let mut renamed: BTreeMap<Uuid, String> = BTreeMap::new();
    for ids in groups.values() {
        for id in ids.iter().skip(1) {
            let new_name = distinct_name(&names[id], &taken);
            taken.insert(Profile::normalize_name(&new_name));
            renamed.insert(*id, new_name);
        }
    }
    for (id, new_name) in &renamed {
        if let Some(state) = changed.get_mut(id) {
            state.name = new_name.clone();
        } else if let Some(state) = merger.planned.get(id) {
            // Planned but unchanged: written with its new name.
            let mut state = state.clone();
            state.name = new_name.clone();
            changed.insert(*id, state);
        }
        plan.renames.push((*id, new_name.clone()));
    }

    // Profiles whose name is set aside before the writes: every existing one written, renamed or
    // removed.
    let mut parked: BTreeSet<Uuid> = BTreeSet::new();
    for id in changed.keys().chain(renamed.keys()) {
        if merger.local_profile(*id)?.is_some() {
            parked.insert(*id);
        }
    }
    parked.extend(removed_here.iter().copied());
    plan.parked = parked.into_iter().collect();
    plan.profiles = changed.into_values().collect();

    // Sessions: union by identifier.
    let mut seen_sessions: HashSet<Uuid> = HashSet::new();
    for change in &batch.sessions {
        let session = &change.session;
        if !seen_sessions.insert(session.id) || view.has_session(session.id)? {
            continue;
        }
        if session.ended_at.is_none() {
            return Err(Error::InvalidRequest(
                "Synchronisation: a session still open was sent.".to_owned(),
            ));
        }
        let mut indexes = HashSet::new();
        for answer in &change.answers {
            if answer.session_id != session.id || !indexes.insert(answer.index) {
                return Err(Error::InvalidRequest(
                    "Synchronisation: an answer does not belong to its session.".to_owned(),
                ));
            }
        }
        let Some(owner) = merger.owner(session.profile_id)? else {
            continue;
        };
        let mut change = change.clone();
        change.session.profile_id = owner;
        touched.insert(owner);
        plan.sessions.push(change);
    }

    // Keyed records: the most recent clock wins (removed ones included).
    let mut best: HashMap<(Entity, String), Hlc> = HashMap::new();
    let mut newer =
        |merger: &mut Merger<'_>, entity: Entity, key: String, clock: Hlc| -> Result<bool> {
            let current = match best.get(&(entity, key.clone())) {
                Some(clock) => Some(*clock),
                None => merger.view.clock_of(entity, &key)?,
            };
            if current.is_some_and(|c| c >= clock) {
                return Ok(false);
            }
            best.insert((entity, key), clock);
            Ok(true)
        };

    let mut lessons: BTreeMap<String, LessonProgressChange> = BTreeMap::new();
    for change in &batch.lesson_progress {
        let Some(owner) = merger.owner(change.profile_id)? else {
            continue;
        };
        let key = record_key(owner, &[&change.lesson_id]);
        if newer(
            &mut merger,
            Entity::LessonProgress,
            key.clone(),
            change.clock,
        )? {
            let mut change = change.clone();
            change.profile_id = owner;
            lessons.insert(key, change);
        }
    }
    plan.lesson_progress = lessons.into_values().collect();

    let mut reviews: BTreeMap<String, ReviewChange> = BTreeMap::new();
    for change in &batch.reviews {
        let Some(owner) = merger.owner(change.profile_id)? else {
            continue;
        };
        let key = record_key(owner, &[&change.skill_id, &change.key]);
        if newer(&mut merger, Entity::Review, key.clone(), change.clock)? {
            let mut change = change.clone();
            change.profile_id = owner;
            if let Some(item) = change.item.as_mut() {
                item.profile_id = owner;
                item.skill_id = change.skill_id.clone();
                item.key = change.key.clone();
            }
            reviews.insert(key, change);
        }
    }
    plan.reviews = reviews.into_values().collect();

    let mut settings: BTreeMap<String, SettingChange> = BTreeMap::new();
    for change in &batch.settings {
        let Some(owner) = merger.owner(change.profile_id)? else {
            continue;
        };
        let key = record_key(owner, &[&change.key]);
        if newer(&mut merger, Entity::Setting, key.clone(), change.clock)? {
            let mut change = change.clone();
            change.profile_id = owner;
            settings.insert(key, change);
        }
    }
    plan.settings = settings.into_values().collect();

    // Union: the earliest date wins.
    let mut badges: BTreeMap<(Uuid, String), BadgeChange> = BTreeMap::new();
    for change in &batch.badges {
        let Some(owner) = merger.owner(change.profile_id)? else {
            continue;
        };
        let id = (owner, change.badge_id.clone());
        let current = match badges.get(&id) {
            Some(c) => Some(c.earned_at),
            None => view.badge(owner, &change.badge_id)?,
        };
        if current.is_none_or(|at| change.earned_at < at) {
            let mut change = change.clone();
            change.profile_id = owner;
            badges.insert(id, change);
        }
    }
    plan.badges = badges.into_values().collect();

    let mut accessories: BTreeMap<(Uuid, String), AccessoryChange> = BTreeMap::new();
    for change in &batch.accessories {
        let Some(owner) = merger.owner(change.profile_id)? else {
            continue;
        };
        let id = (owner, change.accessory_id.clone());
        let current = match accessories.get(&id) {
            Some(c) => Some(c.unlocked_at),
            None => view.accessory(owner, &change.accessory_id)?,
        };
        if current.is_none_or(|at| change.unlocked_at < at) {
            let mut change = change.clone();
            change.profile_id = owner;
            accessories.insert(id, change);
        }
    }
    plan.accessories = accessories.into_values().collect();

    let mut days: BTreeMap<(Uuid, i64), GoalDayChange> = BTreeMap::new();
    for change in &batch.goal_days {
        let Some(owner) = merger.owner(change.profile_id)? else {
            continue;
        };
        let id = (owner, change.day.day);
        let current = match days.get(&id) {
            Some(c) => Some(c.day),
            None => view.goal_day(owner, change.day.day)?,
        };
        // The earliest; for the same instant, the smallest bonus (the same on every device).
        let earlier = current.is_none_or(|day| {
            (change.day.reached_at, change.day.bonus_points) < (day.reached_at, day.bonus_points)
        });
        if earlier {
            let mut change = change.clone();
            change.profile_id = owner;
            touched.insert(owner);
            days.insert(id, change);
        }
    }
    plan.goal_days = days.into_values().collect();

    plan.touched_profiles = touched.into_iter().collect();
    Ok(plan)
}
