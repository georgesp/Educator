//! The synchronisation of bout en bout (SUB027601, SUB027602, SUB027606, SUB027610): a real server
//! on this machine, two real SQLite databases, the client of the iPad. No external network.

use educator_core::clock::{uuid_v7, Clock, SystemClock};
use educator_core::profiles::{Profile, ProfileStore};
use educator_core::sessions::{
    update_masteries, AnswerRecord, SessionEnd, SessionRecord, SessionStore, SessionType,
};
use educator_core::skills::{AnswerMode, Subject};
use educator_core::sync::{PeerRole, SyncStore};
use educator_core::Grade;
use educator_data::{
    Database, SqliteProfileStore, SqliteRewardStore, SqliteSessionStore, SqliteSyncStore,
    DATABASE_FILE_NAME,
};
use educator_sync::client::{Client, ClientError, ComputerAddress};
use educator_sync::pairing::{PairingLink, PairingOffer};
use educator_sync::server::{RunningServer, ServerOptions};
use educator_sync::service::{ClientSync, ServerHandler, SyncDependencies, SyncError, SyncOutcome};
use educator_sync::tls::TlsIdentity;
use educator_sync::wire::*;
use std::sync::{Arc, Mutex, Once};
use uuid::Uuid;

// ----- Logs: nothing that identifies a child --------------------------------------------------

static LOGS: Mutex<Vec<String>> = Mutex::new(Vec::new());

struct Capture;

impl log::Log for Capture {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        LOGS.lock().unwrap().push(record.args().to_string());
    }

    fn flush(&self) {}
}

fn capture_logs() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        log::set_logger(&Capture).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
    });
}

// ----- Devices --------------------------------------------------------------------------------

struct Device {
    _dir: tempfile::TempDir,
    database: Arc<Database>,
    profiles: Arc<SqliteProfileStore>,
    sessions: Arc<SqliteSessionStore>,
    rewards: Arc<SqliteRewardStore>,
    sync: Arc<SqliteSyncStore>,
}

impl Device {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let (database, _) = Database::open(
            &dir.path().join(DATABASE_FILE_NAME),
            Arc::new(SystemClock) as Arc<dyn Clock>,
        )
        .unwrap();
        let database = Arc::new(database);
        Device {
            _dir: dir,
            profiles: Arc::new(SqliteProfileStore::new(database.clone())),
            sessions: Arc::new(SqliteSessionStore::new(database.clone())),
            rewards: Arc::new(SqliteRewardStore::new(database.clone())),
            sync: Arc::new(SqliteSyncStore::new(database.clone())),
            database,
        }
    }

    fn deps(&self) -> SyncDependencies {
        SyncDependencies {
            store: self.sync.clone(),
            profiles: self.profiles.clone(),
            sessions: self.sessions.clone(),
            rewards: self.rewards.clone(),
            clock: Arc::new(SystemClock),
        }
    }

    fn add_profile(&self, name: &str) -> Profile {
        let now = SystemClock.now_ms();
        self.profiles
            .add(&Profile {
                id: uuid_v7(now),
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

    fn play(&self, profile: Uuid, answers: usize) {
        let now = SystemClock.now_ms();
        let id = Uuid::now_v7();
        self.sessions
            .add_session(&SessionRecord {
                id,
                profile_id: profile,
                session_type: SessionType::Exercise,
                subject: Subject::Math,
                grade: Grade::CE1,
                skill_id: Some("ce1-add".to_owned()),
                lesson_id: None,
                answer_mode: Some(AnswerMode::Keyboard),
                format: None,
                seed: 3,
                question_count: answers as u32,
                time_limit_seconds: None,
                started_at: now,
                ended_at: None,
                abandoned: false,
                correct_count: 0,
                points_earned: 0,
            })
            .unwrap();
        for index in 0..answers {
            self.sessions
                .add_answer(&AnswerRecord {
                    id: Uuid::now_v7(),
                    session_id: id,
                    index: index as u32,
                    skill_id: "ce1-add".to_owned(),
                    prompt: "2 + 2 = ?".to_owned(),
                    expected: "4".to_owned(),
                    given: "4".to_owned(),
                    is_correct: true,
                    duration_ms: Some(900),
                    answered_at: now + index as i64,
                })
                .unwrap();
        }
        let points = 10 * answers as i32 + 40;
        self.profiles.add_points(profile, points).unwrap();
        self.sessions
            .end_session(
                id,
                &SessionEnd {
                    ended_at: now + answers as i64 + 1,
                    abandoned: false,
                    question_count: answers as u32,
                    correct_count: answers as u32,
                    points_earned: points,
                },
            )
            .unwrap();
        update_masteries(
            self.sessions.as_ref(),
            profile,
            &["ce1-add".to_owned()],
            now,
        )
        .unwrap();
    }

    /// What the parents' space shows: profiles, points, sessions, masteries.
    fn summary(&self) -> Vec<Summary> {
        let mut profiles = self.profiles.list().unwrap();
        profiles.sort_by_key(|p| p.id);
        profiles
            .into_iter()
            .map(|p| {
                (
                    p.id,
                    p.name.clone(),
                    p.points,
                    self.sessions.profile_sessions(p.id).unwrap().len(),
                    self.sessions
                        .masteries(p.id)
                        .unwrap()
                        .into_iter()
                        .map(|m| (m.skill_id, m.score, m.attempt_count))
                        .collect(),
                )
            })
            .collect()
    }
}

/// A profile as the parents' space shows it: id, name, points, sessions, masteries.
type Summary = (Uuid, String, i32, usize, Vec<(String, u8, u32)>);

/// The computer with its server listening on this machine.
struct Computer {
    device: Device,
    handler: Arc<ServerHandler>,
    server: RunningServer,
    identity: TlsIdentity,
}

impl Computer {
    fn start() -> Self {
        let device = Device::new();
        let identity = educator_sync::server_identity(device.sync.as_ref()).unwrap();
        let handler = Arc::new(ServerHandler::new(device.deps()));
        let server = RunningServer::start(
            &identity,
            ServerOptions::default(),
            handler.clone() as Arc<dyn educator_sync::server::Handler>,
        )
        .unwrap();
        Computer {
            device,
            handler,
            server,
            identity,
        }
    }

    fn link(&self, offer: &PairingOffer) -> String {
        PairingLink {
            addresses: vec!["127.0.0.1".to_owned()],
            port: self.server.port(),
            fingerprint: self.identity.fingerprint().unwrap(),
            secret: offer.secret.clone(),
        }
        .to_uri()
    }

    fn address(&self) -> ComputerAddress {
        ComputerAddress {
            addresses: vec!["127.0.0.1".to_owned()],
            port: self.server.port(),
            fingerprint: Some(self.identity.fingerprint().unwrap()),
        }
    }
}

fn never() -> bool {
    false
}

fn paired() -> (Computer, Device, ClientSync) {
    let computer = Computer::start();
    let tablet = Device::new();
    let client = ClientSync::new(tablet.deps());
    let offer = computer.handler.open_pairing();
    client.pair_with_link(&computer.link(&offer)).unwrap();
    (computer, tablet, client)
}

// ----- Tests ----------------------------------------------------------------------------------

#[test]
fn a_tablet_pairs_with_the_qr_code_and_the_secret_serves_once() {
    let computer = Computer::start();
    let tablet = Device::new();
    let client = ClientSync::new(tablet.deps());
    let offer = computer.handler.open_pairing();
    let link = computer.link(&offer);

    let peer = client.pair_with_link(&link).unwrap();

    assert_eq!(computer.device.sync.device_id().unwrap(), peer.device_id);
    assert_eq!(PeerRole::Computer, peer.role);
    let tablets = computer.device.sync.peers().unwrap();
    assert_eq!(
        vec!["Tablette 1"],
        tablets.iter().map(|p| p.name.as_str()).collect::<Vec<_>>()
    );
    // The computer keeps only the hash of the token.
    assert_eq!(None, tablets[0].token);
    assert_ne!(peer.token, tablets[0].token_hash);

    // The QR code photographed and replayed by another device: refused.
    let other = Device::new();
    let intruder = ClientSync::new(other.deps());
    assert_eq!(
        Err(SyncError::PairingExpired),
        intruder.pair_with_link(&link).map(|_| ())
    );
    assert_eq!(1, computer.device.sync.peers().unwrap().len());
}

#[test]
fn an_expired_offer_or_a_foreign_qr_code_is_refused() {
    let computer = Computer::start();
    let tablet = Device::new();
    let client = ClientSync::new(tablet.deps());
    let offer = computer.handler.open_pairing();
    computer.handler.close_pairing();

    assert_eq!(
        Err(SyncError::PairingExpired),
        client.pair_with_link(&computer.link(&offer)).map(|_| ())
    );
    assert_eq!(
        Err(SyncError::InvalidLink),
        client
            .pair_with_link("https://example.com/?p=1")
            .map(|_| ())
    );
}

#[test]
fn the_short_code_pairs_without_camera() {
    let computer = Computer::start();
    let tablet = Device::new();
    let client = ClientSync::new(tablet.deps());
    let address = format!("127.0.0.1:{}", computer.server.port());
    let offer = computer.handler.open_pairing();

    // A typing mistake costs one try, not the offer.
    assert_eq!(
        Err(SyncError::PairingExpired),
        client.pair_with_code(&address, "0000-0000").map(|_| ())
    );
    let peer = client
        .pair_with_code(&address, &offer.display_code().to_lowercase())
        .unwrap();

    // The certificate seen at the first answer is kept and pinned from then on.
    assert_eq!(
        peer.fingerprint,
        Some(computer.identity.fingerprint().unwrap())
    );
    assert_eq!(
        Err(SyncError::InvalidLink),
        client
            .pair_with_code("8.8.8.8:443", "AAAA-AAAA")
            .map(|_| ())
    );
}

#[test]
fn both_devices_end_with_the_same_profiles_points_and_statistics() {
    capture_logs();
    let (computer, tablet, client) = paired();
    let lea = computer.device.add_profile("Léaprénomsecret");
    computer.device.play(lea.id, 3);
    let tom = tablet.add_profile("Tomprénomsecret");
    tablet.play(tom.id, 5);

    let outcome = client.synchronise(&never, &never).unwrap();

    assert!(matches!(outcome, SyncOutcome::Done { sent, received } if sent > 0 && received > 0));
    assert_eq!(computer.device.summary(), tablet.summary());
    assert_eq!(2, tablet.summary().len());

    // Offline work on both, then again.
    tablet.play(lea.id, 2);
    computer.device.play(lea.id, 4);
    client.synchronise(&never, &never).unwrap();
    assert_eq!(computer.device.summary(), tablet.summary());
    let lea_points = tablet
        .summary()
        .into_iter()
        .find(|p| p.0 == lea.id)
        .unwrap()
        .2;
    assert_eq!(70 + 60 + 80, lea_points);

    // Repeated: nothing doubled.
    let before = tablet.summary();
    assert_eq!(
        SyncOutcome::Done {
            sent: 0,
            received: 0
        },
        client.synchronise(&never, &never).unwrap()
    );
    assert_eq!(before, tablet.summary());
    assert!(client.computer().unwrap().unwrap().last_sync_at.is_some());
    assert!(computer.device.sync.peers().unwrap()[0]
        .last_sync_at
        .is_some());

    // Nothing in the logs identifies a child.
    let logs = LOGS.lock().unwrap().join("\n");
    assert!(!logs.contains("prénomsecret"), "{logs}");
    assert!(!logs.contains(&lea.id.to_string()));
}

#[test]
fn the_same_name_on_both_devices_waits_for_the_parent() {
    let (computer, tablet, client) = paired();
    let on_computer = computer.device.add_profile("Léa");
    computer.device.play(on_computer.id, 2);
    let on_tablet = tablet.add_profile("Léa");
    tablet.play(on_tablet.id, 3);
    let other = tablet.add_profile("Zoé");
    let twin = computer.device.add_profile("Zoé");

    let SyncOutcome::NeedsDecision(questions) = client.synchronise(&never, &never).unwrap() else {
        panic!("a question is expected");
    };
    assert_eq!(2, questions.len());
    // Nothing was sent before the answer.
    assert_eq!(2, computer.device.summary().len());

    client.resolve(on_tablet.id, true).unwrap();
    client.resolve(other.id, false).unwrap();
    client.synchronise(&never, &never).unwrap();

    let names: Vec<String> = computer.device.summary().into_iter().map(|p| p.1).collect();
    assert_eq!(3, names.len());
    assert!(names.contains(&"Zoé 2".to_owned()));
    let lea = computer
        .device
        .summary()
        .into_iter()
        .find(|p| p.0 == on_computer.id)
        .unwrap();
    assert_eq!((60 + 70, 2), (lea.2, lea.3));
    assert_eq!(computer.device.summary(), tablet.summary());
    let _ = twin;
}

#[test]
fn a_removed_tablet_or_an_invalid_token_is_refused_at_once() {
    let (computer, tablet, client) = paired();
    tablet.add_profile("Léa");
    client.synchronise(&never, &never).unwrap();

    let mut peer = computer.device.sync.peers().unwrap().remove(0);
    peer.revoked = true;
    computer.device.sync.save_peer(&peer).unwrap();

    assert_eq!(
        Err(SyncError::Unauthorized),
        client.synchronise(&never, &never)
    );

    let raw = Client::new(computer.address());
    let refused = raw.post::<_, HelloResponse>(
        "/v1/hello",
        Some("00"),
        &HelloRequest {
            protocol: PROTOCOL_VERSION,
            device_id: tablet.sync.device_id().unwrap(),
        },
    );
    assert!(matches!(
        refused,
        Err(ClientError::Refused { status: 401, .. })
    ));
}

#[test]
fn another_certificate_is_refused_and_named() {
    let (computer, _tablet, client) = paired();
    // A machine that pretends to be the computer: same address, another certificate.
    let mut peer = client.computer().unwrap().unwrap();
    peer.fingerprint = Some("00".repeat(32));
    _tablet.sync.save_peer(&peer).unwrap();

    assert_eq!(
        Err(SyncError::CertificateMismatch),
        client.synchronise(&never, &never)
    );
    drop(computer);
}

#[test]
fn a_computer_off_is_simply_unreachable_and_nothing_starts_during_a_series() {
    let (computer, tablet, client) = paired();
    tablet.add_profile("Léa");
    let busy = || true;
    assert_eq!(Err(SyncError::Busy), client.synchronise(&busy, &never));

    drop(computer);

    assert_eq!(
        Err(SyncError::Unreachable),
        client.synchronise(&never, &never)
    );
    // The tablet keeps working: its profile is still there, to send next time.
    assert_eq!(1, tablet.summary().len());
}

#[test]
fn an_exchange_stopped_in_the_middle_resumes_without_doubles() {
    let (computer, tablet, client) = paired();
    let lea = computer.device.add_profile("Léa");
    for _ in 0..450 {
        computer.device.play(lea.id, 10);
    }
    let calls = std::sync::atomic::AtomicUsize::new(0);
    // Interrupted after the first pull (the application goes to the background).
    let interrupted = || calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst) >= 3;

    assert_eq!(
        Err(SyncError::Interrupted),
        client.synchronise(&never, &interrupted)
    );
    let partial = tablet.sessions.profile_sessions(lea.id).unwrap().len();
    assert!(partial > 0 && partial < 450, "{partial}");

    client.synchronise(&never, &never).unwrap();
    assert_eq!(450, tablet.sessions.profile_sessions(lea.id).unwrap().len());
    assert_eq!(computer.device.summary(), tablet.summary());
}

#[test]
fn a_message_of_another_version_or_malformed_is_refused_clearly() {
    let (computer, tablet, _client) = paired();
    let raw = Client::new(computer.address());
    let device_id = tablet.sync.device_id().unwrap();

    let other_version = raw.post::<_, PullResponse>(
        "/v1/pull",
        None,
        &serde_json::json!({ "protocol": 2, "deviceId": device_id, "after": 0 }),
    );
    assert_eq!(Err(ClientError::Incompatible), other_version.map(|_| ()));

    let malformed = raw.post::<_, PullResponse>(
        "/v1/pull",
        None,
        &serde_json::json!({ "protocol": 1, "after": "zéro" }),
    );
    assert!(matches!(
        malformed,
        Err(ClientError::Refused { status: 400, ref code }) if code == "invalid"
    ));
}

#[test]
fn the_server_is_not_reachable_once_stopped() {
    let mut computer = Computer::start();
    let address = computer.address();
    computer.server.stop();

    let raw = Client::new(address);
    let result = raw.post::<_, PairResponse>(
        "/v1/pair",
        None,
        &PairRequest {
            protocol: PROTOCOL_VERSION,
            device_id: Uuid::now_v7(),
            secret: "00".repeat(16),
        },
    );
    assert_eq!(Err(ClientError::Unreachable), result.map(|_| ()));
}

#[test]
fn repeated_failures_from_one_address_are_slowed_down() {
    let computer = Computer::start();
    let raw = Client::new(computer.address());
    let hello = HelloRequest {
        protocol: PROTOCOL_VERSION,
        device_id: Uuid::now_v7(),
    };
    for _ in 0..5 {
        let refused = raw.post::<_, HelloResponse>("/v1/hello", Some("bad"), &hello);
        assert!(matches!(
            refused,
            Err(ClientError::Refused { status: 401, .. })
        ));
    }
    let limited = raw.post::<_, HelloResponse>("/v1/hello", Some("bad"), &hello);
    assert!(matches!(
        limited,
        Err(ClientError::Refused { status: 429, .. })
    ));
}

#[test]
fn the_tablet_forgets_the_computer_and_pairs_again() {
    let (computer, tablet, client) = paired();
    client.forget().unwrap();
    assert_eq!(
        Err(SyncError::NotPaired),
        client.synchronise(&never, &never)
    );

    let offer = computer.handler.open_pairing();
    client.pair_with_link(&computer.link(&offer)).unwrap();
    tablet.add_profile("Léa");
    client.synchronise(&never, &never).unwrap();
    assert_eq!(1, computer.device.summary().len());
    // Paired again with the same tablet: still one tablet, the same name.
    assert_eq!(1, computer.device.sync.peers().unwrap().len());
}

#[test]
fn a_profile_removed_on_the_tablet_is_no_question_and_its_new_namesake_is_kept() {
    let (computer, tablet, client) = paired();
    let first = computer.device.add_profile("Léa");
    client.synchronise(&never, &never).unwrap();

    // On the tablet: the old Léa deleted, a new Léa created who plays, before any synchronisation.
    tablet.profiles.delete(first.id).unwrap();
    let new = tablet.add_profile("Léa");
    tablet.play(new.id, 3);

    let outcome = client.synchronise(&never, &never).unwrap();

    assert!(matches!(outcome, SyncOutcome::Done { .. }), "{outcome:?}");
    let names: Vec<(Uuid, String)> = computer
        .device
        .summary()
        .into_iter()
        .map(|p| (p.0, p.1))
        .collect();
    assert_eq!(vec![(new.id, "Léa".to_owned())], names);
    assert_eq!(computer.device.summary(), tablet.summary());
    assert_eq!(1, tablet.summary()[0].3);
}

#[test]
fn a_backup_restored_on_the_tablet_gets_back_what_the_computer_has() {
    let (computer, tablet, client) = paired();
    tablet.add_profile("Tom");
    let backup = educator_data::backup::export(&tablet.database).unwrap();
    computer.device.add_profile("Léa");
    client.synchronise(&never, &never).unwrap();
    assert_eq!(2, tablet.summary().len());

    educator_data::backup::restore(&tablet.database, &backup).unwrap();
    assert_eq!(1, tablet.summary().len());
    client.synchronise(&never, &never).unwrap();

    assert_eq!(2, tablet.summary().len());
    assert_eq!(computer.device.summary(), tablet.summary());
}

#[test]
fn a_backup_restored_on_the_computer_gets_back_what_the_tablet_has_and_what_it_restores() {
    let (computer, tablet, client) = paired();
    let max = computer.device.add_profile("Max");
    computer.device.add_profile("Léa");
    let backup = educator_data::backup::export(&computer.device.database).unwrap();
    tablet.add_profile("Tom");
    client.synchronise(&never, &never).unwrap();
    // Max is removed on the tablet, after the backup of the computer.
    tablet.profiles.delete(max.id).unwrap();
    client.synchronise(&never, &never).unwrap();
    assert_eq!(2, computer.device.summary().len());

    educator_data::backup::restore(&computer.device.database, &backup).unwrap();
    client.synchronise(&never, &never).unwrap();

    // Tom comes back from the tablet; Max, restored by the parent, comes back on both devices
    // (under a new identifier: his old one stays removed).
    let mut names: Vec<String> = computer.device.summary().into_iter().map(|p| p.1).collect();
    names.sort();
    assert_eq!(vec!["Léa", "Max", "Tom"], names);
    assert!(computer.device.summary().iter().all(|p| p.0 != max.id));
    assert_eq!(computer.device.summary(), tablet.summary());
}

#[test]
fn nothing_is_ever_sent_outside_the_local_network() {
    let client = Client::new(ComputerAddress {
        addresses: vec!["203.0.113.7".to_owned(), "8.8.8.8".to_owned()],
        port: 443,
        fingerprint: Some("00".repeat(32)),
    });
    let started = std::time::Instant::now();
    let result = client.post::<_, HelloResponse>(
        "/v1/hello",
        None,
        &HelloRequest {
            protocol: PROTOCOL_VERSION,
            device_id: Uuid::now_v7(),
        },
    );
    assert_eq!(Err(ClientError::Unreachable), result.map(|_| ()));
    // Not even tried.
    assert!(started.elapsed() < std::time::Duration::from_millis(500));
}
