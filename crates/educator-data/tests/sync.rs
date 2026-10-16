//! Synchronisation storage (SUB027607, SUB027608, ADR 0007) on real SQLite files: the identity of
//! the device, the changes followed by the triggers, the export after a cursor and the merge of two
//! devices that worked apart.

use educator_core::clock::{unix_ms, Clock, ManualClock};
use educator_core::profile_settings::ProfileSettingsStore;
use educator_core::profiles::{Profile, ProfileStore};
use educator_core::reviews::{ReviewItem, ReviewStore};
use educator_core::rewards::daily_goal::GoalDay;
use educator_core::rewards::RewardStore;
use educator_core::sessions::{
    update_masteries, AnswerRecord, SessionEnd, SessionRecord, SessionStore, SessionType,
};
use educator_core::skills::{AnswerMode, Subject};
use educator_core::sync::*;
use educator_core::Grade;
use educator_data::{
    Database, SqliteProfileSettingsStore, SqliteProfileStore, SqliteReviewStore, SqliteRewardStore,
    SqliteSessionStore, SqliteSyncStore, DATABASE_FILE_NAME, MIGRATIONS,
};
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

const START: i64 = 1_792_000_000_000;

/// One device: its database and its stores.
struct Device {
    _dir: tempfile::TempDir,
    path: PathBuf,
    clock: Arc<ManualClock>,
    database: Arc<Database>,
    profiles: SqliteProfileStore,
    sessions: SqliteSessionStore,
    rewards: SqliteRewardStore,
    reviews: SqliteReviewStore,
    settings: SqliteProfileSettingsStore,
    sync: SqliteSyncStore,
}

impl Device {
    fn new(now: i64) -> Self {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(DATABASE_FILE_NAME);
        Self::open(dir, path, now)
    }

    fn open(dir: tempfile::TempDir, path: PathBuf, now: i64) -> Self {
        let clock = Arc::new(ManualClock::new(now));
        let (database, _) = Database::open(&path, clock.clone() as Arc<dyn Clock>).unwrap();
        let database = Arc::new(database);
        Device {
            _dir: dir,
            path,
            clock,
            profiles: SqliteProfileStore::new(database.clone()),
            sessions: SqliteSessionStore::new(database.clone()),
            rewards: SqliteRewardStore::new(database.clone()),
            reviews: SqliteReviewStore::new(database.clone()),
            settings: SqliteProfileSettingsStore::new(database.clone()),
            sync: SqliteSyncStore::new(database.clone()),
            database,
        }
    }

    fn id(&self) -> Uuid {
        self.sync.device_id().unwrap()
    }

    fn tick(&self) -> i64 {
        self.clock.advance(1_000);
        self.clock.now_ms()
    }

    fn add_profile(&self, name: &str) -> Profile {
        let now = self.tick();
        self.profiles
            .add(&Profile {
                id: educator_core::clock::uuid_v7(now),
                name: name.to_owned(),
                grade: Grade::CE1,
                character_id: "lion".to_owned(),
                accessories: Vec::new(),
                points: 0,
                placement_seconds: 20,
                exam_seconds: 0,
                created_at: 0,
                updated_at: 0,
                last_used_at: now,
            })
            .unwrap()
    }

    /// A series of `results.len()` questions on a skill, ended (points added, masteries updated),
    /// as the series engine records it.
    fn play(&self, profile: Uuid, skill: &str, results: &[bool]) -> Uuid {
        self.play_as(profile, skill, results, SessionType::Exercise)
    }

    fn play_as(
        &self,
        profile: Uuid,
        skill: &str,
        results: &[bool],
        session_type: SessionType,
    ) -> Uuid {
        let start = self.tick();
        let id = educator_core::clock::uuid_v7(start);
        self.sessions
            .add_session(&SessionRecord {
                id,
                profile_id: profile,
                session_type,
                subject: Subject::Math,
                grade: Grade::CE1,
                skill_id: Some(skill.to_owned()),
                lesson_id: None,
                answer_mode: Some(AnswerMode::Keyboard),
                format: None,
                seed: 7,
                question_count: results.len() as u32,
                time_limit_seconds: None,
                started_at: start,
                ended_at: None,
                abandoned: false,
                correct_count: 0,
                points_earned: 0,
            })
            .unwrap();
        for (index, right) in results.iter().enumerate() {
            let at = self.tick();
            self.sessions
                .add_answer(&AnswerRecord {
                    id: educator_core::clock::uuid_v7(at),
                    session_id: id,
                    index: index as u32,
                    skill_id: skill.to_owned(),
                    prompt: format!("{index} + ? = 9"),
                    expected: "9".to_owned(),
                    given: if *right { "9" } else { "8" }.to_owned(),
                    is_correct: *right,
                    duration_ms: Some(1200),
                    answered_at: at,
                })
                .unwrap();
        }
        let correct = results.iter().filter(|r| **r).count();
        // A level assessment earns no points.
        let points = if session_type == SessionType::Placement {
            0
        } else {
            10 * correct as i32 + 20
        };
        self.profiles.add_points(profile, points).unwrap();
        let now = self.tick();
        self.sessions
            .end_session(
                id,
                &SessionEnd {
                    ended_at: now,
                    abandoned: false,
                    question_count: results.len() as u32,
                    correct_count: correct as u32,
                    points_earned: points,
                },
            )
            .unwrap();
        update_masteries(&self.sessions, profile, &[skill.to_owned()], now).unwrap();
        id
    }

    fn reach_goal(&self, profile: Uuid, day: i64) {
        let now = self.tick();
        let goal = GoalDay {
            day,
            reached_at: now,
            bonus_points: 30,
        };
        assert!(self.rewards.add_goal_day(profile, &goal).unwrap());
        self.profiles.add_points(profile, 30).unwrap();
    }

    fn refresh(&self) {
        refresh_pending(
            &self.sessions,
            &self.rewards,
            &self.sync,
            self.clock.now_ms(),
        )
        .unwrap();
    }

    fn profile(&self, id: Uuid) -> Option<Profile> {
        self.profiles.get(id).unwrap()
    }

    /// The synchronised content of the database, without what is local to a device (row
    /// timestamps, identifiers of review items, sync tables).
    fn dump(&self) -> Vec<String> {
        let connection = rusqlite::Connection::open(&self.path).unwrap();
        let queries = [
            r#"SELECT "Id", "Name", "Grade", "CharacterId", "Avatar", "Points", "PlacementSeconds", "ExamSeconds", "CreatedAt", "LastUsedAt" FROM "Profile" ORDER BY "Id""#,
            r#"SELECT "Id", "ProfileId", "Type", "Subject", "Grade", "SkillId", "Seed", "QuestionCount", "StartedAt", "EndedAt", "Abandoned", "CorrectCount", "PointsEarned" FROM "Session" WHERE "EndedAt" IS NOT NULL ORDER BY "Id""#,
            r#"SELECT a."Id", a."SessionId", a."Index", a."SkillId", a."Given", a."IsCorrect", a."AnsweredAt" FROM "Answer" a JOIN "Session" s ON s."Id" = a."SessionId" WHERE s."EndedAt" IS NOT NULL ORDER BY a."Id""#,
            r#"SELECT "ProfileId", "SkillId", "Score", "AttemptCount" FROM "SkillMastery" ORDER BY 1, 2"#,
            r#"SELECT "ProfileId", "LessonId", "SlideIndex" FROM "LessonProgress" ORDER BY 1, 2"#,
            r#"SELECT "ProfileId", "BadgeId", "EarnedAt" FROM "EarnedBadge" ORDER BY 1, 2"#,
            r#"SELECT "ProfileId", "AccessoryId", "UnlockedAt" FROM "UnlockedAccessory" ORDER BY 1, 2"#,
            r#"SELECT "ProfileId", "Day", "ReachedAt", "BonusPoints" FROM "DailyGoalDay" ORDER BY 1, 2"#,
            r#"SELECT "ProfileId", "SkillId", "Key", "Prompt", "Expected", "Word", "Step", "DueDay" FROM "ReviewItem" ORDER BY 1, 2, 3"#,
            r#"SELECT "ProfileId", "Key", "Value" FROM "ProfileSetting" ORDER BY 1, 2"#,
        ];
        let mut lines = Vec::new();
        for sql in queries {
            let mut statement = connection.prepare(sql).unwrap();
            let count = statement.column_count();
            let rows = statement
                .query_map([], |row| {
                    let values: Vec<String> = (0..count)
                        .map(|i| format!("{:?}", row.get::<_, rusqlite::types::Value>(i).unwrap()))
                        .collect();
                    Ok(values.join("|"))
                })
                .unwrap()
                .collect::<rusqlite::Result<Vec<_>>>()
                .unwrap();
            lines.push(format!("-- {sql}"));
            lines.extend(rows);
        }
        lines
    }
}

/// The cursors of a tablet paired with a computer (kept by the tablet, ADR 0007 section 3).
#[derive(Debug, Default, Clone, Copy)]
struct Cursors {
    pushed: i64,
    pulled: i64,
}

/// Push of the tablet, pull, then push of what the pull changed here: what `educator-sync` does.
fn sync(tablet: &Device, computer: &Device, cursors: &mut Cursors) {
    push(tablet, computer, cursors);
    pull(tablet, computer, cursors, usize::MAX);
    push(tablet, computer, cursors);
    tablet.refresh();
    computer.refresh();
}

fn push(tablet: &Device, computer: &Device, cursors: &mut Cursors) {
    loop {
        let page = tablet
            .sync
            .export(&ExportRequest::after(cursors.pushed, Some(computer.id())))
            .unwrap();
        computer.sync.apply(&page.batch, None).unwrap();
        cursors.pushed = page.last_seq;
        if !page.more {
            break;
        }
    }
}

/// At most `pages` pages (to cut the exchange in the middle).
fn pull(tablet: &Device, computer: &Device, cursors: &mut Cursors, pages: usize) {
    for _ in 0..pages {
        let page = computer
            .sync
            .export(&ExportRequest::after(cursors.pulled, Some(tablet.id())))
            .unwrap();
        tablet.sync.apply(&page.batch, None).unwrap();
        cursors.pulled = page.last_seq;
        if !page.more {
            break;
        }
    }
}

fn names(device: &Device) -> Vec<String> {
    let mut names: Vec<String> = device
        .profiles
        .list()
        .unwrap()
        .into_iter()
        .map(|p| p.name)
        .collect();
    names.sort();
    names
}

#[test]
fn a_new_database_has_a_stable_identity_that_a_backup_does_not_carry() {
    let device = Device::new(START);
    let id = device.id();
    let path = device.path.clone();
    let Device { _dir, .. } = device;

    let reopened = Device::open(_dir, path, START + 10);

    assert_eq!(id, reopened.id());
    let backup = educator_data::backup::export(&reopened.database).unwrap();
    assert!(!backup.contains(&educator_core::sync::id_key(id)));
    assert!(!backup.contains("SyncRow"));
}

#[test]
fn every_write_of_a_followed_table_is_recorded_in_the_same_transaction() {
    let device = Device::new(START);
    let lea = device.add_profile("Léa");
    let open = |entity: &str| {
        educator_data::sync::followed_counts(&device.database)
            .unwrap()
            .get(entity)
            .copied()
            .unwrap_or(0)
    };
    assert_eq!(6, open("Profile"));

    device.play(lea.id, "ce1-add", &[true, false, true]);
    device.reach_goal(lea.id, 20_000);
    device.rewards.add_badge(lea.id, "first", START).unwrap();
    device
        .rewards
        .unlock_accessory(lea.id, "cap", START)
        .unwrap();
    device
        .sessions
        .save_lesson_slide(lea.id, "l1", Some(2))
        .unwrap();
    device
        .settings
        .set_profile_setting(lea.id, "theme", "cosmos")
        .unwrap();
    device
        .reviews
        .save_review_item(&review(lea.id, "ce1-add", "7+5"))
        .unwrap();

    for entity in [
        "Session",
        "GoalDay",
        "Badge",
        "Accessory",
        "LessonProgress",
        "Setting",
        "Review",
    ] {
        assert_eq!(1, open(entity), "{entity}");
    }
    let page = device.sync.export(&ExportRequest::after(0, None)).unwrap();
    assert_eq!(1, page.batch.sessions.len());
    assert_eq!(3, page.batch.sessions[0].answers.len());
    assert_eq!(1, page.batch.profiles.len());
    assert!(!page.more);

    // Finishing the lesson leaves a tombstone; removing the profile leaves one for the profile.
    device
        .sessions
        .save_lesson_slide(lea.id, "l1", None)
        .unwrap();
    let page = device
        .sync
        .export(&ExportRequest::after(page.last_seq, None))
        .unwrap();
    assert_eq!(
        vec![None],
        page.batch
            .lesson_progress
            .iter()
            .map(|l| l.slide)
            .collect::<Vec<_>>()
    );
    device.profiles.delete(lea.id).unwrap();
    let page = device
        .sync
        .export(&ExportRequest::after(page.last_seq, None))
        .unwrap();
    assert_eq!(
        vec![lea.id],
        page.batch
            .removals
            .iter()
            .map(|r| r.profile_id)
            .collect::<Vec<_>>()
    );
    assert!(page.batch.profiles.is_empty());
}

#[test]
fn a_database_of_lot_3_is_migrated_with_everything_to_send_and_its_points_recomputed() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(DATABASE_FILE_NAME);
    let connection = rusqlite::Connection::open(&path).unwrap();
    for (_, sql) in &MIGRATIONS[..4] {
        connection.execute_batch(sql).unwrap();
    }
    connection
        .execute_batch(
            r#"PRAGMA user_version = 4;
            INSERT INTO "Profile" ("Id", "Name", "NormalizedName", "Grade", "CharacterId", "Points", "CreatedAt", "UpdatedAt", "LastUsedAt", "Avatar")
            VALUES ('0199D1F2-0000-7000-8000-000000000001', 'Léa', 'LÉA', 'CE1', 'renard', 999, 1, 2, 3, '{"character":"renard","accessories":[]}');
            INSERT INTO "Session" ("Id", "ProfileId", "Type", "Subject", "Grade", "SkillId", "Seed", "QuestionCount", "StartedAt", "EndedAt", "Abandoned", "CorrectCount", "PointsEarned", "CreatedAt", "UpdatedAt")
            VALUES ('0199D1F2-0000-7000-8000-0000000000A1', '0199D1F2-0000-7000-8000-000000000001', 'Exercise', 'math', 'CE1', 'ce1-add', 1, 2, 10, 20, 0, 2, 40, 10, 20),
                   ('0199D1F2-0000-7000-8000-0000000000A2', '0199D1F2-0000-7000-8000-000000000001', 'Exercise', 'math', 'CE1', 'ce1-add', 1, 2, 30, NULL, 0, 0, 0, 30, 30);
            INSERT INTO "Answer" ("Id", "SessionId", "Index", "SkillId", "Prompt", "Expected", "Given", "IsCorrect", "DurationMs", "AnsweredAt")
            VALUES ('0199D1F2-0000-7000-8000-0000000000B1', '0199D1F2-0000-7000-8000-0000000000A1', 0, 'ce1-add', '1 + 1', '2', '2', 1, 5, 11),
                   ('0199D1F2-0000-7000-8000-0000000000B2', '0199D1F2-0000-7000-8000-0000000000A1', 1, 'ce1-add', '2 + 2', '4', '4', 1, 5, 12);
            INSERT INTO "DailyGoalDay" ("ProfileId", "Day", "ReachedAt", "BonusPoints", "CreatedAt", "UpdatedAt")
            VALUES ('0199D1F2-0000-7000-8000-000000000001', 20000, 21, 30, 21, 21);
            INSERT INTO "EarnedBadge" ("ProfileId", "BadgeId", "EarnedAt", "CreatedAt", "UpdatedAt")
            VALUES ('0199D1F2-0000-7000-8000-000000000001', 'first', 21, 21, 21);"#,
        )
        .unwrap();
    drop(connection);

    let device = Device::open(dir, path, START);

    let lea = device
        .profile(Uuid::parse_str("0199D1F2-0000-7000-8000-000000000001").unwrap())
        .unwrap();
    // 40 of the closed session + 30 of the goal: the 929 others matched no session.
    assert_eq!(70, lea.points);
    let counts = educator_data::sync::followed_counts(&device.database).unwrap();
    assert_eq!(Some(&6), counts.get("Profile"));
    // The session still open is not sent: it will be once closed.
    assert_eq!(Some(&1), counts.get("Session"));
    assert_eq!(Some(&1), counts.get("GoalDay"));
    assert_eq!(Some(&1), counts.get("Badge"));
    let page = device.sync.export(&ExportRequest::after(0, None)).unwrap();
    assert_eq!(2, page.batch.sessions[0].answers.len());

    // Closed at start-up as abandoned (as the application does): then it is to send.
    educator_core::sessions::close_open_sessions(&device.sessions, START).unwrap();
    let page = device
        .sync
        .export(&ExportRequest::after(page.last_seq, None))
        .unwrap();
    assert_eq!(1, page.batch.sessions.len());
}

#[test]
fn two_devices_that_worked_apart_end_with_the_same_data_and_statistics() {
    let computer = Device::new(START);
    let tablet = Device::new(START + 500);
    let lea = computer.add_profile("Léa");
    computer.play(lea.id, "ce1-add", &[true, true, false]);
    let tom = tablet.add_profile("Tom");
    tablet.play(tom.id, "ce1-sub", &[true; 5]);
    let mut cursors = Cursors::default();

    sync(&tablet, &computer, &mut cursors);

    assert_eq!(computer.dump(), tablet.dump());
    assert_eq!(vec!["Léa", "Tom"], names(&tablet));

    // Offline the same day on both, Léa on each device.
    computer.play(lea.id, "ce1-add", &[false, false]);
    computer.reach_goal(lea.id, 20_000);
    tablet.play(lea.id, "ce1-add", &[true, true, true, true]);
    tablet.reach_goal(lea.id, 20_000);
    tablet
        .rewards
        .add_badge(lea.id, "first", START + 9)
        .unwrap();
    computer
        .rewards
        .add_badge(lea.id, "first", START + 5)
        .unwrap();

    sync(&tablet, &computer, &mut cursors);

    assert_eq!(computer.dump(), tablet.dump());
    let sessions = computer.sessions.profile_sessions(lea.id).unwrap();
    assert_eq!(3, sessions.len());
    // One bonus for the day (ADR 0007, section 6): the points are those of a single device.
    let expected = profile_points(&sessions, &computer.rewards.goal_days(lea.id).unwrap());
    assert_eq!(expected, computer.profile(lea.id).unwrap().points);
    assert_eq!(expected, tablet.profile(lea.id).unwrap().points);
    // 40 + 20 + 60 for the three series, 30 once for the goal.
    assert_eq!(150, expected);
    assert_eq!(
        Some(START + 5),
        tablet
            .rewards
            .earned_badges(lea.id)
            .unwrap()
            .iter()
            .find(|b| b.badge_id == "first")
            .map(|b| b.earned_at)
    );
    let mastery = |d: &Device| {
        d.sessions
            .masteries(lea.id)
            .unwrap()
            .into_iter()
            .map(|m| (m.skill_id, m.score, m.attempt_count))
            .collect::<Vec<_>>()
    };
    assert_eq!(vec![("ce1-add".to_owned(), 67, 9)], mastery(&computer));
    assert_eq!(mastery(&computer), mastery(&tablet));

    // Again: nothing changes, nothing is doubled.
    let before = computer.dump();
    sync(&tablet, &computer, &mut cursors);
    assert_eq!(before, computer.dump());
    assert_eq!(before, tablet.dump());
}

#[test]
fn an_answer_of_a_skill_unknown_here_and_a_level_assessment_done_there_are_kept() {
    let computer = Device::new(START);
    let tablet = Device::new(START);
    let lea = tablet.add_profile("Léa");
    // A newer version of the tablet: a skill this computer does not know yet.
    tablet.play(lea.id, "ce9-from-the-future", &[true, false]);
    tablet.play_as(lea.id, "ce1-add", &[true; 4], SessionType::Placement);
    let mut cursors = Cursors::default();

    sync(&tablet, &computer, &mut cursors);

    let sessions = computer.sessions.profile_sessions(lea.id).unwrap();
    assert_eq!(2, sessions.len());
    // The level assessment counts as done on the computer too (SUB001418).
    assert!(sessions
        .iter()
        .any(|s| s.session_type == SessionType::Placement && !s.abandoned));
    let unknown = computer
        .sessions
        .masteries(lea.id)
        .unwrap()
        .into_iter()
        .find(|m| m.skill_id == "ce9-from-the-future")
        .unwrap();
    assert_eq!((50, 2), (unknown.score, unknown.attempt_count));
    assert_eq!(computer.dump(), tablet.dump());
}

#[test]
fn applying_the_same_batch_twice_changes_nothing() {
    let computer = Device::new(START);
    let tablet = Device::new(START);
    let lea = tablet.add_profile("Léa");
    tablet.play(lea.id, "ce1-add", &[true, false]);
    tablet
        .sessions
        .save_lesson_slide(lea.id, "l1", Some(3))
        .unwrap();
    let page = tablet
        .sync
        .export(&ExportRequest::after(0, Some(computer.id())))
        .unwrap();

    computer.sync.apply(&page.batch, None).unwrap();
    computer.refresh();
    let once = computer.dump();
    let report = computer.sync.apply(&page.batch, None).unwrap();
    computer.refresh();

    assert_eq!(once, computer.dump());
    assert_eq!(0, report.sessions_added);
    assert_eq!(0, report.profiles_written);
}

#[test]
fn a_large_history_is_cut_into_batches_and_an_interrupted_exchange_resumes_without_doubles() {
    let computer = Device::new(START);
    let tablet = Device::new(START);
    let lea = computer.add_profile("Léa");
    // 300 series of 10 answers: 3 300 records.
    for i in 0..300 {
        computer.play(lea.id, "ce1-add", &[i % 3 != 0; 10]);
    }
    let mut cursors = Cursors::default();
    let first = computer
        .sync
        .export(&ExportRequest::after(0, Some(tablet.id())))
        .unwrap();
    assert!(first.more);
    assert!(first.batch.item_count() <= MAX_BATCH_ITEMS + MAX_BATCH_SESSIONS);

    // The Wi-Fi drops after one page: the page was saved, the next one never arrived.
    pull(&tablet, &computer, &mut cursors, 1);
    assert!(tablet.sessions.profile_sessions(lea.id).unwrap().len() < 300);
    // The same page again (its acknowledgement was lost), then the rest.
    let replay = Cursors {
        pulled: 0,
        ..cursors
    };
    pull(&tablet, &computer, &mut replay.clone(), 1);
    sync(&tablet, &computer, &mut cursors);

    assert_eq!(300, tablet.sessions.profile_sessions(lea.id).unwrap().len());
    assert_eq!(computer.dump(), tablet.dump());
}

#[test]
fn a_profile_renamed_on_both_devices_keeps_the_last_name() {
    let computer = Device::new(START);
    let tablet = Device::new(START);
    let lea = computer.add_profile("Léa");
    let mut cursors = Cursors::default();
    sync(&tablet, &computer, &mut cursors);

    let mut on_computer = computer.profile(lea.id).unwrap();
    on_computer.name = "Léana".to_owned();
    on_computer.grade = Grade::CE2;
    computer.tick();
    computer.profiles.update(&on_computer).unwrap();
    // The tablet renames later (its clock is behind but it knows the computer's clock).
    let mut on_tablet = tablet.profile(lea.id).unwrap();
    on_tablet.name = "Lélé".to_owned();
    tablet.clock.set(START + 100_000);
    tablet.profiles.update(&on_tablet).unwrap();

    sync(&tablet, &computer, &mut cursors);

    // Field by field: the last name (tablet), the grade of the computer.
    let merged = computer.profile(lea.id).unwrap();
    assert_eq!(("Lélé", Grade::CE2), (merged.name.as_str(), merged.grade));
    assert_eq!(computer.dump(), tablet.dump());
}

#[test]
fn a_clock_late_by_minutes_does_not_lose_the_changes_made_after_a_sync() {
    let computer = Device::new(START + 600_000);
    let tablet = Device::new(START);
    let lea = computer.add_profile("Léa");
    let mut cursors = Cursors::default();
    sync(&tablet, &computer, &mut cursors);

    // Ten minutes late, the tablet still renames after the computer's last change.
    let mut on_tablet = tablet.profile(lea.id).unwrap();
    on_tablet.name = "Lili".to_owned();
    tablet.profiles.update(&on_tablet).unwrap();
    sync(&tablet, &computer, &mut cursors);

    assert_eq!("Lili", computer.profile(lea.id).unwrap().name);
}

#[test]
fn a_removal_wins_over_what_the_other_device_did_with_the_profile() {
    let computer = Device::new(START);
    let tablet = Device::new(START);
    let lea = computer.add_profile("Léa");
    let mut cursors = Cursors::default();
    sync(&tablet, &computer, &mut cursors);

    computer.profiles.delete(lea.id).unwrap();
    tablet.play(lea.id, "ce1-add", &[true]);
    let mut on_tablet = tablet.profile(lea.id).unwrap();
    on_tablet.name = "Léa B".to_owned();
    tablet.tick();
    tablet.profiles.update(&on_tablet).unwrap();

    sync(&tablet, &computer, &mut cursors);

    assert_eq!(None, computer.profile(lea.id));
    assert_eq!(None, tablet.profile(lea.id));
    assert_eq!(computer.dump(), tablet.dump());
}

#[test]
fn two_children_with_the_same_name_are_both_kept_one_renamed_the_same_way_everywhere() {
    let computer = Device::new(START);
    let tablet = Device::new(START);
    let first = computer.add_profile("Léa");
    let second = tablet.add_profile("léa");
    let mut cursors = Cursors::default();

    sync(&tablet, &computer, &mut cursors);

    assert_eq!(names(&computer), names(&tablet));
    let renamed = if first.id > second.id {
        first.id
    } else {
        second.id
    };
    assert_eq!(
        "léa 2",
        tablet.profile(renamed).unwrap().name.to_lowercase()
    );
    assert_eq!(computer.dump(), tablet.dump());
}

#[test]
fn the_same_child_on_both_devices_is_linked_with_everything_it_did() {
    let computer = Device::new(START);
    let tablet = Device::new(START);
    let on_computer = computer.add_profile("Léa");
    computer.play(on_computer.id, "ce1-add", &[true, true]);
    let on_tablet = tablet.add_profile("Léa");
    tablet.play(on_tablet.id, "ce1-add", &[false, true, true]);
    tablet
        .rewards
        .add_badge(on_tablet.id, "first", START)
        .unwrap();
    tablet
        .sessions
        .save_lesson_slide(on_tablet.id, "l1", Some(4))
        .unwrap();

    // « Est-ce le même enfant ? » on the tablet: yes.
    let remote = computer
        .sync
        .profile_states()
        .unwrap()
        .into_iter()
        .find(|p| p.id == on_computer.id)
        .unwrap();
    let conflicts = name_conflicts(
        &[(on_tablet.id, on_tablet.name.clone())],
        std::slice::from_ref(&remote),
    );
    assert_eq!(1, conflicts.len());
    tablet.sync.link_profile(on_tablet.id, &remote).unwrap();
    tablet.refresh();
    let mut cursors = Cursors::default();
    sync(&tablet, &computer, &mut cursors);

    assert_eq!(None, tablet.profile(on_tablet.id));
    assert_eq!(vec!["Léa"], names(&computer));
    assert_eq!(
        2,
        computer
            .sessions
            .profile_sessions(on_computer.id)
            .unwrap()
            .len()
    );
    assert_eq!(
        Some(4),
        computer
            .sessions
            .lesson_slide(on_computer.id, "l1")
            .unwrap()
    );
    assert_eq!(
        1,
        computer
            .rewards
            .earned_badges(on_computer.id)
            .unwrap()
            .len()
    );
    assert_eq!(40 + 40, computer.profile(on_computer.id).unwrap().points);
    assert_eq!(computer.dump(), tablet.dump());
}

#[test]
fn linking_to_a_profile_removed_here_is_refused_and_loses_nothing() {
    let computer = Device::new(START);
    let tablet = Device::new(START);
    let first = computer.add_profile("Léa");
    let mut cursors = Cursors::default();
    sync(&tablet, &computer, &mut cursors);
    let remote = computer
        .sync
        .profile_states()
        .unwrap()
        .into_iter()
        .find(|p| p.id == first.id)
        .unwrap();
    tablet.profiles.delete(first.id).unwrap();
    let new = tablet.add_profile("Léa");
    tablet.play(new.id, "ce1-add", &[true]);

    assert!(tablet.sync.link_profile(new.id, &remote).is_err());
    assert!(tablet.sync.is_removed(first.id).unwrap());
    assert_eq!(1, tablet.sessions.profile_sessions(new.id).unwrap().len());
}

#[test]
fn a_malformed_batch_is_refused_without_writing_anything() {
    let device = Device::new(START);
    let lea = device.add_profile("Léa");
    let before = device.dump();
    let unknown = Uuid::now_v7();
    let batch = ChangeBatch {
        profiles: vec![ProfileState::of(
            &Profile {
                name: "Tom".to_owned(),
                id: Uuid::now_v7(),
                ..device.profile(lea.id).unwrap()
            },
            ProfileClocks::all(Hlc::new(START, 0, Uuid::now_v7())),
        )],
        sessions: vec![SessionChange {
            session: SessionRecord {
                id: Uuid::now_v7(),
                profile_id: unknown,
                session_type: SessionType::Exercise,
                subject: Subject::Math,
                grade: Grade::CE1,
                skill_id: None,
                lesson_id: None,
                answer_mode: None,
                format: None,
                seed: 1,
                question_count: 0,
                time_limit_seconds: None,
                started_at: 1,
                ended_at: Some(2),
                abandoned: false,
                correct_count: 0,
                points_earned: 0,
            },
            answers: Vec::new(),
            clock: Hlc::new(START, 0, Uuid::now_v7()),
        }],
        ..ChangeBatch::default()
    };

    assert!(device.sync.apply(&batch, None).is_err());
    assert_eq!(before, device.dump());
}

#[test]
fn a_restored_backup_sends_no_removal_and_keeps_the_identity() {
    let device = Device::new(START);
    let id = device.id();
    let lea = device.add_profile("Léa");
    let backup = educator_data::backup::export(&device.database).unwrap();
    device.add_profile("Tom");
    let page = device.sync.export(&ExportRequest::after(0, None)).unwrap();

    educator_data::backup::restore(&device.database, &backup).unwrap();

    assert_eq!(id, device.id());
    let after = device
        .sync
        .export(&ExportRequest::after(page.last_seq, None))
        .unwrap();
    assert!(after.batch.removals.is_empty());
    assert_eq!(
        vec![lea.id],
        after
            .batch
            .profiles
            .iter()
            .map(|p| p.id)
            .collect::<Vec<_>>()
    );
}

#[test]
fn the_paired_devices_are_stored_without_showing_their_token() {
    let device = Device::new(START);
    let peer = SyncPeer {
        device_id: Uuid::now_v7(),
        role: PeerRole::Computer,
        name: "Ordinateur".to_owned(),
        token_hash: None,
        token: Some("secret-token".to_owned()),
        fingerprint: Some("AB".repeat(32)),
        addresses: vec!["192.168.1.10".to_owned(), "10.0.0.2".to_owned()],
        port: Some(47615),
        paired_at: START,
        last_sync_at: None,
        pushed_seq: 3,
        pulled_seq: 9,
        revoked: false,
        epoch: Some("e1".to_owned()),
    };

    device.sync.save_peer(&peer).unwrap();

    assert_eq!(
        Some(peer.clone()),
        device.sync.peer(peer.device_id).unwrap()
    );
    assert_eq!(vec![peer.clone()], device.sync.peers().unwrap());
    device.sync.record_push(peer.device_id, 12).unwrap();
    device
        .sync
        .record_sync(peer.device_id, START + 5, Some(&["10.0.0.2".to_owned()]))
        .unwrap();
    let saved = device.sync.peer(peer.device_id).unwrap().unwrap();
    assert_eq!(
        (12, Some(START + 5)),
        (saved.pushed_seq, saved.last_sync_at)
    );
    assert_eq!(vec!["10.0.0.2".to_owned()], saved.addresses);
    device.sync.reset_cursors(peer.device_id, "e2").unwrap();
    let saved = device.sync.peer(peer.device_id).unwrap().unwrap();
    assert_eq!(
        (0, 0, Some("e2")),
        (saved.pushed_seq, saved.pulled_seq, saved.epoch.as_deref())
    );
    assert!(!format!("{peer:?}").contains("secret-token"));
    assert!(device.sync.delete_peer(peer.device_id).unwrap());
    assert!(device.sync.peers().unwrap().is_empty());
    // Forgotten in the middle of a synchronisation: the late updates bring nothing back.
    device.sync.record_push(peer.device_id, 99).unwrap();
    device
        .sync
        .record_sync(peer.device_id, START, None)
        .unwrap();
    assert!(device.sync.peers().unwrap().is_empty());
    assert!(device.sync.set_value("Epoch", "x").is_err());
    assert!(device.sync.set_value("DeviceId", "x").is_err());
    device.sync.set_value("Receive", "on").unwrap();
    assert_eq!(Some("on".to_owned()), device.sync.value("Receive").unwrap());
}

fn review(profile: Uuid, skill: &str, key: &str) -> ReviewItem {
    ReviewItem {
        id: Uuid::now_v7(),
        profile_id: profile,
        skill_id: skill.to_owned(),
        key: key.to_owned(),
        prompt: "7 + ? = 12".to_owned(),
        expected: "5".to_owned(),
        word: None,
        origin: None,
        step: 0,
        due_day: 20_001,
        created_at: unix_ms(2026, 10, 15, 8, 0, 0),
        updated_at: 0,
    }
}

/// Random histories (fixed seeds): what a device does on its own between two synchronisations.
mod generated {
    use super::*;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    const NAMES: [&str; 4] = ["Léa", "Tom", "Zoé", "Inès"];

    fn work(device: &Device, rng: &mut ChaCha8Rng, steps: usize) {
        for _ in 0..steps {
            let profiles = device.profiles.list().unwrap();
            let pick = |rng: &mut ChaCha8Rng| -> Option<Profile> {
                (!profiles.is_empty())
                    .then(|| profiles[rng.random_range(0..profiles.len())].clone())
            };
            match rng.random_range(0..12) {
                0 | 1 => {
                    let name = NAMES[rng.random_range(0..NAMES.len())];
                    if !device
                        .profiles
                        .name_exists(&Profile::normalize_name(name), None)
                        .unwrap()
                    {
                        device.add_profile(name);
                    }
                }
                2..=4 => {
                    if let Some(p) = pick(rng) {
                        let results: Vec<bool> = (0..rng.random_range(1..6))
                            .map(|_| rng.random_bool(0.7))
                            .collect();
                        let skill = ["ce1-add", "ce1-sub"][rng.random_range(0..2)];
                        device.play(p.id, skill, &results);
                    }
                }
                5 => {
                    if let Some(mut p) = pick(rng) {
                        p.grade = [Grade::CP, Grade::CE1, Grade::CE2][rng.random_range(0..3)];
                        device.tick();
                        device.profiles.update(&p).unwrap();
                    }
                }
                6 => {
                    if let Some(p) = pick(rng) {
                        let day = 20_000 + rng.random_range(0..3);
                        let now = device.tick();
                        let goal = GoalDay {
                            day,
                            reached_at: now,
                            bonus_points: 30,
                        };
                        if device.rewards.add_goal_day(p.id, &goal).unwrap() {
                            device.profiles.add_points(p.id, 30).unwrap();
                        }
                    }
                }
                7 => {
                    if let Some(p) = pick(rng) {
                        let badge = ["first", "ten", "star"][rng.random_range(0..3)];
                        let at = device.tick();
                        device.rewards.add_badge(p.id, badge, at).unwrap();
                        device.rewards.unlock_accessory(p.id, badge, at).unwrap();
                    }
                }
                8 => {
                    if let Some(p) = pick(rng) {
                        device.tick();
                        let slide = rng.random_range(0..4);
                        let slide = (slide > 0).then_some(slide);
                        device
                            .sessions
                            .save_lesson_slide(p.id, "l1", slide)
                            .unwrap();
                    }
                }
                9 => {
                    if let Some(p) = pick(rng) {
                        device.tick();
                        let theme = ["sport", "cosmos"][rng.random_range(0..2)];
                        device
                            .settings
                            .set_profile_setting(p.id, "theme", theme)
                            .unwrap();
                    }
                }
                10 => {
                    if let Some(p) = pick(rng) {
                        device.tick();
                        let key = ["7+5", "8+6"][rng.random_range(0..2)];
                        let existing = device
                            .reviews
                            .review_items(p.id)
                            .unwrap()
                            .into_iter()
                            .find(|i| i.key == key);
                        match existing {
                            Some(item) if rng.random_bool(0.5) => {
                                device.reviews.delete_review_item(item.id).unwrap();
                            }
                            Some(mut item) => {
                                item.step += 1;
                                device.reviews.save_review_item(&item).unwrap();
                            }
                            None => device
                                .reviews
                                .save_review_item(&review(p.id, "ce1-add", key))
                                .unwrap(),
                        }
                    }
                }
                _ => {
                    if rng.random_bool(0.3) {
                        if let Some(p) = pick(rng) {
                            device.profiles.delete(p.id).unwrap();
                        }
                    }
                }
            }
        }
    }

    fn everything(device: &Device) -> Vec<ChangeBatch> {
        let mut batches = Vec::new();
        let mut after = 0;
        loop {
            let page = device
                .sync
                .export(&ExportRequest::after(after, None))
                .unwrap();
            batches.push(page.batch);
            after = page.last_seq;
            if !page.more {
                return batches;
            }
        }
    }

    fn receive(device: &Device, batches: &[ChangeBatch]) {
        for batch in batches {
            device.sync.apply(batch, None).unwrap();
        }
        device.refresh();
    }

    /// Commutativity (SUB027608): A then B, or B then A, gives the same state.
    #[test]
    fn the_order_of_two_merges_does_not_matter() {
        for seed in 0..12u64 {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let a = Device::new(START);
            let b = Device::new(START + 7);
            work(&a, &mut rng, 40);
            work(&b, &mut rng, 40);
            let (from_a, from_b) = (everything(&a), everything(&b));

            let first = Device::new(START + 50_000);
            receive(&first, &from_a);
            receive(&first, &from_b);
            let second = Device::new(START + 50_000);
            receive(&second, &from_b);
            receive(&second, &from_a);

            assert_eq!(first.dump(), second.dump(), "seed {seed}");
            // Idempotence: everything again, nothing changes.
            receive(&first, &from_a);
            receive(&first, &from_b);
            assert_eq!(second.dump(), first.dump(), "seed {seed}");
        }
    }

    /// Offline work on both devices, several rounds: no loss, no double, the same data.
    #[test]
    fn devices_working_apart_converge_round_after_round() {
        for seed in 100..110u64 {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let computer = Device::new(START);
            let tablet = Device::new(START - 300_000);
            let mut cursors = Cursors::default();
            for round in 0..4 {
                work(&computer, &mut rng, 15);
                work(&tablet, &mut rng, 15);
                let profiles_known: usize = [&computer, &tablet]
                    .iter()
                    .flat_map(|d| d.profiles.list().unwrap())
                    .map(|p| p.id)
                    .collect::<std::collections::BTreeSet<_>>()
                    .len();
                sync(&tablet, &computer, &mut cursors);
                assert_eq!(computer.dump(), tablet.dump(), "seed {seed}, round {round}");
                assert!(computer.profiles.list().unwrap().len() <= profiles_known);
                for profile in computer.profiles.list().unwrap() {
                    let points = profile_points(
                        &computer.sessions.profile_sessions(profile.id).unwrap(),
                        &computer.rewards.goal_days(profile.id).unwrap(),
                    );
                    assert_eq!(points, profile.points, "seed {seed}, round {round}");
                }
            }
        }
    }
}
