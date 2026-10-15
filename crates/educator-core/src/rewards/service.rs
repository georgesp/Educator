use super::accessories::{
    check_outfit, compose_avatar, wearable, Accessory, AccessoryCatalog, WornAccessory,
};
use super::badges::{newly_met, Badge, BadgeFacts, EarnedBadge};
use super::daily_goal::{self, DailyGoalProgress, DailyGoalRules, GoalDay};
use super::levels::{LevelProgress, LevelScale};
use super::stars::{BestStars, StarRules};
use crate::characters::CharacterCatalog;
use crate::clock::Clock;
use crate::exercises::SeriesSummary;
use crate::profile_settings::ProfileSettingsService;
use crate::profiles::{Profile, ProfileStore, UNKNOWN_CHARACTER};
use crate::sessions::{SessionStore, SessionType};
use crate::skills::{Date, SkillCatalog};
use crate::statistics::day_number;
use crate::{Error, Result};
use std::fmt;
use std::sync::Arc;
use uuid::Uuid;

/// The local date of an instant (the time zone of the computer).
pub type DateOf = Arc<dyn Fn(i64) -> Date + Send + Sync>;

/// Storage of the rewards of the profiles (SQLite, in `educator-data`); deleted with the profile.
pub trait RewardStore: Send + Sync {
    fn earned_badges(&self, profile_id: Uuid) -> Result<Vec<EarnedBadge>>;

    /// `false` when the profile already has the badge.
    fn add_badge(&self, profile_id: Uuid, badge_id: &str, earned_at: i64) -> Result<bool>;

    /// Ids of the accessories unlocked by a profile.
    fn unlocked_accessories(&self, profile_id: Uuid) -> Result<Vec<String>>;

    /// `false` when the accessory was already unlocked.
    fn unlock_accessory(&self, profile_id: Uuid, accessory_id: &str, at: i64) -> Result<bool>;

    /// The days a profile reached its goal of the day, oldest first.
    fn goal_days(&self, profile_id: Uuid) -> Result<Vec<GoalDay>>;

    /// `false` when the day is already recorded (the bonus is given once a day).
    fn add_goal_day(&self, profile_id: Uuid, day: &GoalDay) -> Result<bool>;
}

/// The rules of the content.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RewardRules {
    pub levels: LevelScale,
    pub stars: StarRules,
    pub goal: DailyGoalRules,
}

/// What a session brought, shown at its end.
#[derive(Debug, Clone, PartialEq)]
pub struct SessionRewards {
    /// Stars of this series or quiz (`None` for an exam, a challenge, revisions).
    pub stars: Option<u8>,
    /// Best stars of the skill or the lesson before this session.
    pub previous_best_stars: u8,
    pub level_before: LevelProgress,
    pub level: LevelProgress,
    pub goal: DailyGoalProgress,
    /// Bonus of the goal of the day given by this session (0 when it was not reached now).
    pub goal_bonus: i32,
    /// Total of the profile, bonus included.
    pub profile_points: i32,
    pub new_badges: Vec<Badge>,
    pub new_accessories: Vec<Accessory>,
}

/// The rewards of a profile at a glance (menu).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RewardsOverview {
    pub level: LevelProgress,
    pub goal: DailyGoalProgress,
    pub badges_earned: u32,
    pub badges_total: u32,
}

/// A badge and when the profile earned it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BadgeState {
    pub badge: Badge,
    pub earned_at: Option<i64>,
}

/// An accessory and whether the profile unlocked it.
#[derive(Debug, Clone, PartialEq)]
pub struct AccessoryState {
    pub accessory: Accessory,
    pub unlocked: bool,
}

/// Levels, stars, badges, accessories and the goal of the day (lot 3).
pub struct RewardsService {
    rules: RewardRules,
    badges: Arc<Vec<Badge>>,
    accessories: Arc<AccessoryCatalog>,
    skills: Arc<dyn SkillCatalog>,
    characters: Arc<dyn CharacterCatalog>,
    profiles: Arc<dyn ProfileStore>,
    sessions: Arc<dyn SessionStore>,
    store: Arc<dyn RewardStore>,
    settings: Arc<ProfileSettingsService>,
    clock: Arc<dyn Clock>,
    date_of: DateOf,
}

impl fmt::Debug for RewardsService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RewardsService")
            .field("rules", &self.rules)
            .finish_non_exhaustive()
    }
}

/// The collaborators of [`RewardsService`].
pub struct RewardsDependencies {
    pub skills: Arc<dyn SkillCatalog>,
    pub characters: Arc<dyn CharacterCatalog>,
    pub profiles: Arc<dyn ProfileStore>,
    pub sessions: Arc<dyn SessionStore>,
    pub store: Arc<dyn RewardStore>,
    pub settings: Arc<ProfileSettingsService>,
    pub clock: Arc<dyn Clock>,
    pub date_of: DateOf,
}

impl fmt::Debug for RewardsDependencies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RewardsDependencies")
            .finish_non_exhaustive()
    }
}

impl RewardsService {
    pub fn new(
        rules: RewardRules,
        badges: Vec<Badge>,
        accessories: AccessoryCatalog,
        dependencies: RewardsDependencies,
    ) -> Self {
        Self {
            rules,
            badges: Arc::new(badges),
            accessories: Arc::new(accessories),
            skills: dependencies.skills,
            characters: dependencies.characters,
            profiles: dependencies.profiles,
            sessions: dependencies.sessions,
            store: dependencies.store,
            settings: dependencies.settings,
            clock: dependencies.clock,
            date_of: dependencies.date_of,
        }
    }

    pub fn rules(&self) -> &RewardRules {
        &self.rules
    }

    pub fn accessory_catalog(&self) -> &AccessoryCatalog {
        &self.accessories
    }

    pub fn badge_list(&self) -> &[Badge] {
        &self.badges
    }

    fn profile(&self, profile_id: Uuid) -> Result<Profile> {
        self.profiles
            .get(profile_id)?
            .ok_or(Error::ProfileNotFound(profile_id))
    }

    fn today(&self) -> i64 {
        day_number((self.date_of)(self.clock.now_ms()))
    }

    /// The best stars of a profile, by skill and by lesson.
    pub fn best_stars(&self, profile_id: Uuid) -> Result<BestStars> {
        Ok(BestStars::of(
            &self.rules.stars,
            &self.sessions.profile_sessions(profile_id)?,
        ))
    }

    /// The goal of today of a profile.
    pub fn goal(&self, profile_id: Uuid) -> Result<DailyGoalProgress> {
        let target = self.settings.get(profile_id)?.daily_goal;
        Ok(daily_goal::progress(
            target,
            self.rules.goal.bonus_points,
            &self.sessions.profile_sessions(profile_id)?,
            &self.store.goal_days(profile_id)?,
            self.today(),
            &*self.date_of,
        ))
    }

    pub fn overview(&self, profile_id: Uuid) -> Result<RewardsOverview> {
        let profile = self.profile(profile_id)?;
        // A profile created before lot 3, or a scale changed: what its level gives is unlocked.
        self.sync_unlocks(&profile)?;
        let earned = self.store.earned_badges(profile_id)?;
        let count = |n: usize| u32::try_from(n).unwrap_or(u32::MAX);
        Ok(RewardsOverview {
            level: self.rules.levels.progress(profile.points),
            goal: self.goal(profile_id)?,
            badges_earned: count(
                earned
                    .iter()
                    .filter(|e| self.badges.iter().any(|b| b.id == e.badge_id))
                    .count(),
            ),
            badges_total: count(self.badges.len()),
        })
    }

    /// After a session: the goal of the day (and its bonus), the level, the new badges and the
    /// accessories they unlock.
    pub fn after_session(
        &self,
        profile_id: Uuid,
        summary: &SeriesSummary,
    ) -> Result<SessionRewards> {
        let profile = self.profile(profile_id)?;
        let sessions = self.sessions.profile_sessions(profile_id)?;
        let today = self.today();
        let now = self.clock.now_ms();

        // The goal of the day: reached by this session, the bonus is given once.
        let target = self.settings.get(profile_id)?.daily_goal;
        let mut goal_days = self.store.goal_days(profile_id)?;
        let done = daily_goal::done_on(&sessions, today, &*self.date_of);
        let mut goal_bonus = 0;
        let mut points = profile.points;
        if target > 0 && done >= target && !goal_days.iter().any(|d| d.day == today) {
            let day = GoalDay {
                day: today,
                reached_at: now,
                bonus_points: self.rules.goal.bonus_points,
            };
            if self.store.add_goal_day(profile_id, &day)? {
                goal_bonus = day.bonus_points;
                points = self
                    .profiles
                    .add_points(profile_id, goal_bonus)?
                    .ok_or(Error::ProfileNotFound(profile_id))?;
                goal_days.push(day);
            }
        }
        let goal = daily_goal::progress(
            target,
            self.rules.goal.bonus_points,
            &sessions,
            &goal_days,
            today,
            &*self.date_of,
        );

        // Stars of this series, against the best before it.
        let (stars, previous_best_stars) = match summary.session_type {
            SessionType::Exercise | SessionType::LessonQuiz => {
                let earlier: Vec<_> = sessions
                    .iter()
                    .filter(|s| s.id != summary.series_id)
                    .cloned()
                    .collect();
                let before = BestStars::of(&self.rules.stars, &earlier);
                let previous = match summary.session_type {
                    SessionType::Exercise => {
                        summary.skill_id.as_deref().map_or(0, |id| before.skill(id))
                    }
                    _ => summary
                        .lesson_id
                        .as_deref()
                        .map_or(0, |id| before.lesson(id)),
                };
                (
                    Some(self.rules.stars.stars(
                        u32::try_from(summary.correct_count).unwrap_or(u32::MAX),
                        u32::try_from(summary.exercise_count).unwrap_or(u32::MAX),
                        !summary.abandoned,
                    )),
                    previous,
                )
            }
            _ => (None, 0),
        };

        let level_before = self
            .rules
            .levels
            .progress(points - goal_bonus - summary.points_earned);
        let level = self.rules.levels.progress(points);

        // Badges, from everything recorded.
        let best = BestStars::of(&self.rules.stars, &sessions);
        let facts = BadgeFacts::collect(
            &sessions,
            self.sessions.masteries(profile_id)?,
            &goal_days,
            &best,
            level.level,
            &*self.date_of,
        );
        let earned = self.store.earned_badges(profile_id)?;
        let mut new_badges = Vec::new();
        for badge in newly_met(&self.badges, &earned, &facts, self.skills.as_ref()) {
            if self.store.add_badge(profile_id, &badge.id, now)? {
                new_badges.push(badge.clone());
            }
        }

        let mut updated = profile;
        updated.points = points;
        let new_accessories = self.sync_unlocks(&updated)?;
        Ok(SessionRewards {
            stars,
            previous_best_stars,
            level_before,
            level,
            goal,
            goal_bonus,
            profile_points: points,
            new_badges,
            new_accessories,
        })
    }

    /// Unlocks what the level and the badges of a profile give; returns the accessories unlocked now.
    fn sync_unlocks(&self, profile: &Profile) -> Result<Vec<Accessory>> {
        let level = self.rules.levels.level_of(profile.points);
        let badges: Vec<String> = self
            .store
            .earned_badges(profile.id)?
            .into_iter()
            .map(|e| e.badge_id)
            .collect();
        let unlocked = self.store.unlocked_accessories(profile.id)?;
        let now = self.clock.now_ms();
        let mut new = Vec::new();
        for accessory in self.accessories.unlocked_by(level, &badges) {
            if !unlocked.contains(&accessory.id)
                && self
                    .store
                    .unlock_accessory(profile.id, &accessory.id, now)?
            {
                new.push(accessory.clone());
            }
        }
        Ok(new)
    }

    /// Every badge of the content, earned or not.
    pub fn badges(&self, profile_id: Uuid) -> Result<Vec<BadgeState>> {
        self.profile(profile_id)?;
        let earned = self.store.earned_badges(profile_id)?;
        Ok(self
            .badges
            .iter()
            .map(|badge| BadgeState {
                earned_at: earned
                    .iter()
                    .find(|e| e.badge_id == badge.id)
                    .map(|e| e.earned_at),
                badge: badge.clone(),
            })
            .collect())
    }

    /// Every accessory of the content, unlocked or not.
    pub fn accessories(&self, profile_id: Uuid) -> Result<Vec<AccessoryState>> {
        let profile = self.profile(profile_id)?;
        self.sync_unlocks(&profile)?;
        let unlocked = self.store.unlocked_accessories(profile_id)?;
        Ok(self
            .accessories
            .accessories
            .iter()
            .map(|a| AccessoryState {
                unlocked: unlocked.contains(&a.id),
                accessory: a.clone(),
            })
            .collect())
    }

    /// Dresses the avatar of a profile (SUB001505): its character and the accessories it wears.
    /// Saved at once; an accessory still locked, or two in one slot, is refused.
    pub fn set_avatar(
        &self,
        profile_id: Uuid,
        character_id: Option<&str>,
        worn: &[WornAccessory],
    ) -> Result<Profile> {
        let mut profile = self.profile(profile_id)?;
        if let Some(id) = character_id {
            if !self.characters.all().iter().any(|c| c.id == id) {
                return Err(Error::validation("characterId", UNKNOWN_CHARACTER));
            }
            profile.character_id = id.to_owned();
        }
        self.sync_unlocks(&profile)?;
        let unlocked = self.store.unlocked_accessories(profile_id)?;
        check_outfit(&self.accessories, worn, &unlocked)
            .map_err(|e| Error::InvalidRequest(e.to_string()))?;
        profile.accessories = worn.to_vec();
        self.profiles
            .update(&profile)?
            .ok_or(Error::ProfileNotFound(profile_id))
    }

    /// The avatar as one SVG; `None` when the character is unknown.
    pub fn avatar_svg(&self, character_id: &str, worn: &[WornAccessory]) -> Option<String> {
        let svg = self.characters.svg(character_id)?;
        let anchors = self.characters.anchors(character_id)?;
        Some(compose_avatar(
            svg,
            &anchors,
            &wearable(&self.accessories, worn),
        ))
    }
}
