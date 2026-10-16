//! Lot 3 rewards (SUB001501 to SUB001507) through real series: stars, level, goal of the day,
//! badges and the accessories they unlock, the avatar.

mod common;

use common::{
    test_skill, FakeCharacters, InMemoryProfileSettingsStore, InMemoryProfileStore,
    InMemoryRewardStore, InMemorySessionStore,
};
use educator_core::clock::{unix_ms, ManualClock};
use educator_core::exercises::{Answer, SeriesService, SeriesSummary};
use educator_core::points::PointsRules;
use educator_core::profile_settings::ProfileSettingsService;
use educator_core::profiles::{Profile, ProfileStore};
use educator_core::random::ScriptedRandom;
use educator_core::rewards::accessories::{
    Accessory, AccessoryCatalog, AccessoryColor, AccessoryLayer, AccessorySlot, Point, Unlock,
    WornAccessory,
};
use educator_core::rewards::badges::{Badge, BadgeCondition};
use educator_core::rewards::daily_goal::DailyGoalRules;
use educator_core::rewards::levels::LevelScale;
use educator_core::rewards::stars::StarRules;
use educator_core::rewards::{RewardRules, RewardsDependencies, RewardsService};
use educator_core::skills::{AnswerMode, SkillList};
use educator_core::statistics::utc_date_of;
use educator_core::Grade;
use std::sync::Arc;
use uuid::Uuid;

struct Fixture {
    clock: Arc<ManualClock>,
    profiles: Arc<InMemoryProfileStore>,
    settings: Arc<ProfileSettingsService>,
    series: SeriesService,
    rewards: RewardsService,
    profile: Profile,
}

fn badge(id: &str, condition: BadgeCondition) -> Badge {
    Badge {
        id: id.to_owned(),
        title: id.to_owned(),
        description: id.to_owned(),
        icon: "<svg/>".to_owned(),
        condition,
    }
}

fn accessory(id: &str, slot: AccessorySlot, unlock: Unlock) -> Accessory {
    Accessory {
        id: id.to_owned(),
        name: id.to_owned(),
        slot,
        layer: AccessoryLayer::Front,
        anchor: Point::new(10.0, 10.0),
        content: "<path d=\"M0 0h1\"/>".to_owned(),
        colors: vec!["rouge".to_owned()],
        unlock,
    }
}

fn fixture() -> Fixture {
    let clock = Arc::new(ManualClock::new(unix_ms(2026, 10, 12, 8, 0, 0)));
    let profiles = Arc::new(InMemoryProfileStore::default());
    let profile = profiles
        .add(&Profile {
            id: Uuid::now_v7(),
            name: "Léa".to_owned(),
            grade: Grade::CP,
            character_id: "lion".to_owned(),
            accessories: Vec::new(),
            points: 0,
            placement_seconds: 20,
            exam_seconds: 0,
            created_at: 0,
            updated_at: 0,
            last_used_at: 0,
        })
        .unwrap();
    let skills = Arc::new(SkillList(vec![test_skill()]));
    let sessions = Arc::new(InMemorySessionStore::default());
    let settings = Arc::new(ProfileSettingsService::new(
        Arc::new(InMemoryProfileSettingsStore::default()),
        DailyGoalRules::new(2, 30).unwrap(),
        vec!["sport".to_owned()],
    ));
    let series = SeriesService::new(
        skills.clone(),
        profiles.clone(),
        sessions.clone(),
        PointsRules::DEFAULT,
        Arc::new(ScriptedRandom::new([5])),
        clock.clone(),
    );
    let rewards = RewardsService::new(
        RewardRules {
            levels: LevelScale::new(vec![0, 100, 250], 300).unwrap(),
            stars: StarRules::DEFAULT,
            goal: DailyGoalRules::new(2, 30).unwrap(),
        },
        vec![
            badge(
                "first",
                BadgeCondition::SeriesCompleted {
                    count: 1,
                    subject: None,
                },
            ),
            badge("perfect", BadgeCondition::PerfectSeries { count: 2 }),
            badge(
                "master",
                BadgeCondition::SkillMastery {
                    skills: vec!["test-skill".to_owned()],
                    min_score: 90,
                },
            ),
        ],
        AccessoryCatalog {
            accessories: vec![
                accessory("cap", AccessorySlot::Head, Unlock::Level(1)),
                accessory("glasses", AccessorySlot::Eyes, Unlock::Level(2)),
                accessory(
                    "medal",
                    AccessorySlot::Neck,
                    Unlock::Badge("first".to_owned()),
                ),
                accessory("crown", AccessorySlot::Head, Unlock::Level(3)),
            ],
            colors: vec![AccessoryColor {
                id: "rouge".to_owned(),
                name: "Rouge".to_owned(),
                main: "#D7262B".to_owned(),
                light: "#F07A7D".to_owned(),
            }],
        },
        RewardsDependencies {
            skills,
            characters: Arc::new(FakeCharacters::default()),
            profiles: profiles.clone(),
            sessions,
            store: Arc::new(InMemoryRewardStore::default()),
            settings: settings.clone(),
            clock: clock.clone(),
            date_of: Arc::new(utc_date_of),
        },
    );
    Fixture {
        clock,
        profiles,
        settings,
        series,
        rewards,
        profile,
    }
}

/// A series of 10 answered with `right` right answers, then its rewards.
fn play(f: &Fixture, right: usize) -> (SeriesSummary, educator_core::rewards::SessionRewards) {
    let series = f
        .series
        .start(
            f.profile.id,
            "test-skill",
            AnswerMode::Keyboard,
            None,
            Some(1),
            1,
        )
        .unwrap();
    let mut summary = None;
    for (i, exercise) in series.exercises.iter().enumerate() {
        let answer = match (&exercise.expected, i < right) {
            (Answer::Number(n), true) => Answer::Number(*n),
            (Answer::Number(n), false) => Answer::Number(n + 1),
            _ => panic!("numbers only"),
        };
        summary = f
            .series
            .answer(series.id, i, &answer, None)
            .unwrap()
            .summary;
    }
    let summary = summary.unwrap();
    let rewards = f.rewards.after_session(f.profile.id, &summary).unwrap();
    (summary, rewards)
}

#[test]
fn a_finished_series_gives_its_stars_and_the_best_is_kept() {
    let f = fixture();
    let (_, first) = play(&f, 8);
    let (_, second) = play(&f, 5);
    let (_, third) = play(&f, 10);

    assert_eq!((Some(2), 0), (first.stars, first.previous_best_stars));
    assert_eq!((Some(1), 2), (second.stars, second.previous_best_stars));
    assert_eq!((Some(3), 2), (third.stars, third.previous_best_stars));
    assert_eq!(
        3,
        f.rewards
            .best_stars(f.profile.id)
            .unwrap()
            .skill("test-skill")
    );
}

#[test]
fn the_level_rises_with_the_points_and_unlocks_its_accessories() {
    let f = fixture();
    // 10 right: 100 + 20 + 20 = 140 points, level 2 (from 100).
    let (_, rewards) = play(&f, 10);

    assert_eq!((1, 2), (rewards.level_before.level, rewards.level.level));
    let unlocked: Vec<&str> = rewards
        .new_accessories
        .iter()
        .map(|a| a.id.as_str())
        .collect();
    assert!(unlocked.contains(&"cap") && unlocked.contains(&"glasses"));
    assert!(unlocked.contains(&"medal"), "badge first unlocks the medal");
    assert!(!unlocked.contains(&"crown"));
    // 10 answers right: the series, and the skill mastered.
    assert_eq!(
        vec!["first".to_owned(), "master".to_owned()],
        rewards
            .new_badges
            .iter()
            .map(|b| b.id.clone())
            .collect::<Vec<_>>()
    );
}

#[test]
fn the_goal_of_the_day_gives_its_bonus_once_and_counts_the_days_in_a_row() {
    let f = fixture();
    let (_, one) = play(&f, 5);
    assert_eq!(
        (1, false, 0),
        (one.goal.done, one.goal.reached, one.goal_bonus)
    );

    let (summary, two) = play(&f, 5);
    assert_eq!(
        (2, true, 30, 1),
        (
            two.goal.done,
            two.goal.reached,
            two.goal_bonus,
            two.goal.streak
        )
    );
    assert_eq!(summary.profile_points + 30, two.profile_points);
    let (_, three) = play(&f, 5);
    assert_eq!(0, three.goal_bonus, "once a day");

    // The next day: the count starts at 0, the streak is kept, then grows.
    f.clock.advance(86_400_000);
    assert_eq!((0, 1), {
        let g = f.rewards.goal(f.profile.id).unwrap();
        (g.done, g.streak)
    });
    play(&f, 5);
    let (_, next) = play(&f, 5);
    assert_eq!(2, next.goal.streak);

    // A day missed: the count starts again, nothing is taken away.
    f.clock.advance(2 * 86_400_000);
    let goal = f.rewards.goal(f.profile.id).unwrap();
    assert_eq!((0, false, 0), (goal.done, goal.reached, goal.streak));
}

#[test]
fn the_parents_can_change_or_turn_off_the_goal() {
    let f = fixture();
    f.settings
        .update(f.profile.id, &[("dailyGoal".to_owned(), "0".to_owned())])
        .unwrap();
    play(&f, 5);
    let (_, rewards) = play(&f, 5);
    assert_eq!(
        (0, false, 0),
        (
            rewards.goal.target,
            rewards.goal.reached,
            rewards.goal_bonus
        )
    );
}

#[test]
fn badges_are_earned_once_and_kept() {
    let f = fixture();
    let (_, first) = play(&f, 9);
    assert!(first.new_badges.iter().any(|b| b.id == "master"));
    play(&f, 10);
    let (_, third) = play(&f, 10);
    assert_eq!(
        vec!["perfect"],
        third
            .new_badges
            .iter()
            .map(|b| b.id.as_str())
            .collect::<Vec<_>>()
    );
    let (_, fourth) = play(&f, 0);
    assert!(fourth.new_badges.is_empty(), "nothing new, nothing lost");
    let earned = f.rewards.badges(f.profile.id).unwrap();
    assert!(earned.iter().all(|b| b.earned_at.is_some()));
}

#[test]
fn the_avatar_wears_unlocked_accessories_only() {
    let f = fixture();
    let locked = f.rewards.set_avatar(
        f.profile.id,
        None,
        &[WornAccessory::new("glasses", "rouge")],
    );
    assert!(locked.is_err(), "level 2 needed");

    let dressed = f
        .rewards
        .set_avatar(
            f.profile.id,
            Some("ours"),
            &[WornAccessory::new("cap", "rouge")],
        )
        .unwrap();
    assert_eq!("ours", dressed.character_id);
    assert_eq!(
        1,
        f.profiles
            .get(f.profile.id)
            .unwrap()
            .unwrap()
            .accessories
            .len()
    );
    assert!(f
        .rewards
        .set_avatar(f.profile.id, Some("dragon"), &[])
        .is_err());

    let svg = f.rewards.avatar_svg("ours", &dressed.accessories).unwrap();
    assert!(svg.contains(r#"data-accessory="cap""#));
}
