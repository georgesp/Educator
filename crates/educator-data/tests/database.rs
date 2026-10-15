use educator_core::clock::{unix_ms, Clock, ManualClock, SystemClock};
use educator_core::profiles::{Profile, ProfileStore};
use educator_core::settings::AppSettingsStore;
use educator_core::{Error, Grade};
use educator_data::{
    Database, SqliteAppSettingsStore, SqliteProfileStore, DATABASE_FILE_NAME, MIGRATIONS,
};
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

/// Real SQLite database in a temporary folder, deleted at the end of the test.
struct Temporary {
    _dir: tempfile::TempDir,
    path: PathBuf,
}

fn temporary() -> Temporary {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("data").join(DATABASE_FILE_NAME);
    Temporary { _dir: dir, path }
}

fn open(
    path: &std::path::Path,
    clock: Arc<dyn Clock>,
) -> (Arc<Database>, educator_data::InitReport) {
    let (database, report) = Database::open(path, clock).unwrap();
    (Arc::new(database), report)
}

fn scalar(path: &std::path::Path, sql: &str) -> String {
    let connection = rusqlite::Connection::open(path).unwrap();
    connection
        .query_row(sql, [], |row| row.get::<_, rusqlite::types::Value>(0))
        .map(|v| format!("{v:?}"))
        .unwrap()
}

#[test]
fn new_database_applies_the_initial_migration_without_backup() {
    let db = temporary();

    let (_, report) = open(&db.path, Arc::new(SystemClock));

    assert!(db.path.is_file());
    assert_eq!(
        vec!["0001_initial", "0002_sessions", "0003_question_timers"],
        report.applied_migrations
    );
    assert_eq!(None, report.backup_path);
    assert_eq!(
        format!("Integer({})", MIGRATIONS.len()),
        scalar(&db.path, "PRAGMA user_version")
    );
}

#[test]
fn up_to_date_database_does_nothing() {
    let db = temporary();
    drop(open(&db.path, Arc::new(SystemClock)));

    let (_, report) = open(&db.path, Arc::new(SystemClock));

    assert!(report.applied_migrations.is_empty());
    assert_eq!(None, report.backup_path);
}

#[test]
fn existing_database_with_pending_migrations_is_backed_up_first() {
    let db = temporary();
    std::fs::create_dir_all(db.path.parent().unwrap()).unwrap();
    rusqlite::Connection::open(&db.path)
        .unwrap()
        .execute_batch(
            "CREATE TABLE Legacy (Id INTEGER PRIMARY KEY); INSERT INTO Legacy VALUES (42);",
        )
        .unwrap();

    let (_, report) = open(
        &db.path,
        Arc::new(ManualClock::new(unix_ms(2026, 10, 10, 20, 0, 0))),
    );

    let backup = report.backup_path.unwrap();
    assert_eq!(
        PathBuf::from(format!("{}.backup-20261010-200000", db.path.display())),
        backup
    );
    assert!(backup.is_file());
}

#[test]
fn database_of_the_dotnet_version_is_adopted_with_its_profiles() {
    let db = temporary();
    std::fs::create_dir_all(db.path.parent().unwrap()).unwrap();
    // Schema and data as left by EF Core (user_version 0, __EFMigrationsHistory).
    rusqlite::Connection::open(&db.path)
        .unwrap()
        .execute_batch(&format!(
            r#"{}
            CREATE TABLE "__EFMigrationsHistory" ("MigrationId" TEXT NOT NULL PRIMARY KEY, "ProductVersion" TEXT NOT NULL);
            INSERT INTO "Profile" VALUES ('0199D1F2-0000-7000-8000-000000000001', 'Léa', 'LÉA', 'CE1', 'lion', 30, 1, 2, 3);"#,
            MIGRATIONS[0].1
        ))
        .unwrap();

    let (database, report) = open(&db.path, Arc::new(SystemClock));

    assert_eq!(
        vec!["0001_initial", "0002_sessions", "0003_question_timers"],
        report.applied_migrations
    );
    let profiles = SqliteProfileStore::new(database).list().unwrap();
    assert_eq!(
        vec![("Léa".to_owned(), Grade::CE1, 30)],
        profiles
            .iter()
            .map(|p| (p.name.clone(), p.grade, p.points))
            .collect::<Vec<_>>()
    );
}

#[test]
fn journal_mode_is_wal() {
    let db = temporary();
    drop(open(&db.path, Arc::new(SystemClock)));

    assert_eq!("Text(\"wal\")", scalar(&db.path, "PRAGMA journal_mode"));
}

// ── Settings ──────────────────────────────────────────────────────────────────────────────────

#[test]
fn settings_unknown_key_is_none_then_set_get_and_remove() {
    let db = temporary();
    let store = SqliteAppSettingsStore::new(open(&db.path, Arc::new(SystemClock)).0);

    assert_eq!(None, store.get("missing").unwrap());
    store.set("speech.speed", "1.0").unwrap();
    store.set("speech.speed", "0.8").unwrap();
    assert_eq!(Some("0.8".to_owned()), store.get("speech.speed").unwrap());
    assert!(store.remove("speech.speed").unwrap());
    assert!(!store.remove("speech.speed").unwrap());
    assert_eq!(None, store.get("speech.speed").unwrap());
}

#[test]
fn settings_are_stamped_and_updated_at_moves_only_on_change() {
    let db = temporary();
    let created = unix_ms(2026, 10, 10, 20, 0, 0) + 123;
    let clock = Arc::new(ManualClock::new(created));
    let store = SqliteAppSettingsStore::new(open(&db.path, clock.clone()).0);

    store.set("key", "a").unwrap();
    clock.set(created + 5 * 60_000);
    store.set("key", "a").unwrap(); // unchanged value: UpdatedAt must not move
    store.set("other", "x").unwrap();
    clock.set(created + 10 * 60_000);
    store.set("key", "b").unwrap();

    assert_eq!(
        format!("Integer({created})"),
        scalar(
            &db.path,
            r#"SELECT "CreatedAt" FROM "AppSetting" WHERE "Key" = 'key'"#
        )
    );
    assert_eq!(
        format!("Integer({})", created + 10 * 60_000),
        scalar(
            &db.path,
            r#"SELECT "UpdatedAt" FROM "AppSetting" WHERE "Key" = 'key'"#
        )
    );
}

// ── Profiles ──────────────────────────────────────────────────────────────────────────────────

fn start() -> i64 {
    unix_ms(2026, 10, 11, 8, 0, 0)
}

fn profiles(db: &Temporary) -> (SqliteProfileStore, Arc<ManualClock>) {
    let clock = Arc::new(ManualClock::new(start()));
    (
        SqliteProfileStore::new(open(&db.path, clock.clone()).0),
        clock,
    )
}

fn new_profile(name: &str, grade: Grade, last_used: i64) -> Profile {
    Profile {
        id: Uuid::now_v7(),
        name: name.to_owned(),
        grade,
        character_id: "lion".to_owned(),
        points: 0,
        placement_seconds: 20,
        exam_seconds: 0,
        created_at: 0,
        updated_at: 0,
        last_used_at: last_used,
    }
}

#[test]
fn add_stores_every_field_with_timestamps() {
    let db = temporary();
    let (store, _) = profiles(&db);

    let added = store
        .add(&new_profile("Léa", Grade::CE1, start() - 5 * 60_000))
        .unwrap();
    let read = store.get(added.id).unwrap().unwrap();

    assert_eq!("Léa", read.name);
    assert_eq!(Grade::CE1, read.grade);
    assert_eq!("lion", read.character_id);
    assert_eq!(start(), read.created_at);
    assert_eq!(start(), read.updated_at);
    assert_eq!(start() - 5 * 60_000, read.last_used_at);
    let id = added.id.to_string().to_uppercase();
    assert_eq!(
        "Text(\"CE1\")",
        scalar(
            &db.path,
            &format!(r#"SELECT "Grade" FROM "Profile" WHERE "Id" = '{id}'"#)
        )
    );
}

#[test]
fn list_most_recently_used_first() {
    let db = temporary();
    let (store, _) = profiles(&db);
    let day = 86_400_000;
    store
        .add(&new_profile("Tom", Grade::CP, start() - 2 * day))
        .unwrap();
    store.add(&new_profile("Léa", Grade::CP, start())).unwrap();
    store
        .add(&new_profile("Inès", Grade::CP, start() - day))
        .unwrap();

    let names: Vec<String> = store.list().unwrap().into_iter().map(|p| p.name).collect();

    assert_eq!(vec!["Léa", "Inès", "Tom"], names);
}

#[test]
fn name_exists_ignores_case_and_the_profile_itself() {
    let db = temporary();
    let (store, _) = profiles(&db);
    let lea = store.add(&new_profile("Léa", Grade::CP, start())).unwrap();

    assert!(store
        .name_exists(&Profile::normalize_name("léa"), None)
        .unwrap());
    assert!(!store
        .name_exists(&Profile::normalize_name("léa"), Some(lea.id))
        .unwrap());
    assert!(!store
        .name_exists(&Profile::normalize_name("Lea"), None)
        .unwrap());
}

#[test]
fn duplicate_name_is_refused_by_the_database() {
    let db = temporary();
    let (store, _) = profiles(&db);
    store.add(&new_profile("Léa", Grade::CP, start())).unwrap();

    let error = store
        .add(&new_profile("LÉA", Grade::CP, start()))
        .unwrap_err();

    assert!(
        matches!(error, Error::ProfileValidation { field: "name", .. }),
        "{error:?}"
    );
}

#[test]
fn update_changes_name_grade_character_and_last_use_not_points() {
    let db = temporary();
    let (store, clock) = profiles(&db);
    let lea = store.add(&new_profile("Léa", Grade::CP, start())).unwrap();
    store.add_points(lea.id, 30).unwrap();
    clock.set(start() + 3_600_000);

    let updated = store
        .update(&Profile {
            name: "Léana".to_owned(),
            grade: Grade::CE1,
            character_id: "ours".to_owned(),
            points: 999,
            last_used_at: clock.now_ms(),
            ..lea.clone()
        })
        .unwrap()
        .unwrap();

    assert_eq!("Léana", updated.name);
    assert_eq!(Grade::CE1, updated.grade);
    assert_eq!("ours", updated.character_id);
    assert_eq!(30, updated.points);
    assert_eq!(start() + 3_600_000, updated.updated_at);
    assert_eq!(
        None,
        store
            .update(&Profile {
                id: Uuid::now_v7(),
                ..lea
            })
            .unwrap()
    );
}

#[test]
fn the_timers_of_a_profile_are_stored_and_a_new_profile_has_the_defaults() {
    let db = temporary();
    let (store, _) = profiles(&db);
    let lea = store.add(&new_profile("Léa", Grade::CP, start())).unwrap();
    assert_eq!((20, 0), (lea.placement_seconds, lea.exam_seconds));

    let updated = store
        .update(&Profile {
            placement_seconds: 45,
            exam_seconds: 30,
            ..lea.clone()
        })
        .unwrap()
        .unwrap();

    assert_eq!((45, 30), (updated.placement_seconds, updated.exam_seconds));
    let read = store.get(lea.id).unwrap().unwrap();
    assert_eq!((45, 30), (read.placement_seconds, read.exam_seconds));
}

#[test]
fn add_points_adds_atomically_and_returns_the_total() {
    let db = temporary();
    let (store, _) = profiles(&db);
    let lea = store.add(&new_profile("Léa", Grade::CP, start())).unwrap();
    let store = Arc::new(store);

    let threads: Vec<_> = (0..10)
        .map(|_| {
            let store = Arc::clone(&store);
            std::thread::spawn(move || store.add_points(lea.id, 10).unwrap())
        })
        .collect();
    threads.into_iter().for_each(|t| {
        t.join().unwrap();
    });

    assert_eq!(Some(105), store.add_points(lea.id, 5).unwrap());
    assert_eq!(None, store.add_points(Uuid::now_v7(), 5).unwrap());
}

#[test]
fn delete_removes_the_profile() {
    let db = temporary();
    let (store, _) = profiles(&db);
    let lea = store.add(&new_profile("Léa", Grade::CP, start())).unwrap();

    assert!(store.delete(lea.id).unwrap());
    assert!(!store.delete(lea.id).unwrap());
    assert_eq!(None, store.get(lea.id).unwrap());
}
