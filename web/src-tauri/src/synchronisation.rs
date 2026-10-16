//! The screen « Synchronisation » of the parents' space and the exchanges with the other device
//! (lot tablette 2, US0276, ADR 0007): thin adapters over `educator-sync` (routes, client, pairing)
//! and `educator-data` (storage). On the computer: the server, started while the screen is open or
//! while the option « Recevoir les synchronisations » is on. On the iPad: pairing, « Synchroniser
//! maintenant », the automatic attempt at the profile choice, the questions « Est-ce le même
//! enfant ? ». No rule here.

use crate::contracts::*;
use educator_core::sync::{PeerRole, SyncStore};
use educator_core::Error;
use educator_sync::service::{ClientSync, SyncDependencies, SyncError, SyncOutcome};
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use uuid::Uuid;

/// Longest name of a paired device.
const MAX_DEVICE_NAME: usize = 30;

pub struct Synchronisation {
    role: SyncRole,
    store: Arc<dyn SyncStore>,
    client: ClientSync,
    /// Asked by the front when the application goes to the background (SUB023811).
    interrupted: AtomicBool,
    #[cfg(desktop)]
    server: desktop::ServerSide,
}

impl fmt::Debug for Synchronisation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Synchronisation")
            .field("role", &self.role)
            .finish_non_exhaustive()
    }
}

fn failed(error: Error) -> CommandError {
    log::error!("Synchronisation: {error}");
    CommandError::new(500, "Synchronisation error", error.to_string())
}

fn parse_id(id: &str) -> Result<Uuid, CommandError> {
    Uuid::parse_str(id).map_err(|_| CommandError::new(400, "Invalid request", "Unknown device."))
}

impl Synchronisation {
    /// The role of this platform: the computer receives, the iPad starts the exchanges.
    pub fn platform_role() -> SyncRole {
        if cfg!(mobile) {
            SyncRole::Tablet
        } else {
            SyncRole::Computer
        }
    }

    pub fn new(role: SyncRole, deps: SyncDependencies) -> Self {
        let this = Self {
            role,
            store: Arc::clone(&deps.store),
            #[cfg(desktop)]
            server: desktop::ServerSide::new(deps.clone()),
            client: ClientSync::new(deps),
            interrupted: AtomicBool::new(false),
        };
        // The computer receives from the start when the parent asked for it.
        #[cfg(desktop)]
        if role == SyncRole::Computer && this.receive_always() {
            this.server.ensure_running(this.store.as_ref());
        }
        this
    }

    #[cfg_attr(not(desktop), allow(dead_code))]
    fn receive_always(&self) -> bool {
        matches!(
            self.store.value(educator_sync::RECEIVE_KEY),
            Ok(Some(value)) if value == "on"
        )
    }

    fn not_here(&self) -> CommandError {
        CommandError::new(409, "Not on this device", format!("{:?}", self.role))
    }

    pub fn status(&self) -> Result<SyncStatusResponse, CommandError> {
        let peers = self.store.peers().map_err(failed)?;
        let wanted = match self.role {
            SyncRole::Computer => PeerRole::Tablet,
            SyncRole::Tablet => PeerRole::Computer,
        };
        let devices = peers
            .iter()
            .filter(|p| p.role == wanted && !p.revoked)
            .map(SyncDeviceResponse::from)
            .collect();
        #[allow(unused_mut)]
        let mut status = SyncStatusResponse {
            role: self.role,
            receiving: false,
            receive_always: false,
            address: None,
            problem: None,
            devices,
            questions: self
                .client
                .pending_conflicts()
                .iter()
                .map(SyncQuestionResponse::from)
                .collect(),
        };
        #[cfg(desktop)]
        if self.role == SyncRole::Computer {
            status.receive_always = self.receive_always();
            let (receiving, address, problem) = self.server.state();
            status.receiving = receiving;
            status.address = address;
            status.problem = problem;
            status.questions.clear();
        }
        Ok(status)
    }

    /// The screen « Synchronisation » is open: the computer listens (pairing, exchanges).
    pub fn open_screen(&self) -> Result<SyncStatusResponse, CommandError> {
        #[cfg(desktop)]
        if self.role == SyncRole::Computer {
            self.server.screen_opened(self.store.as_ref());
        }
        self.status()
    }

    /// The screen is closed: the computer stops listening unless the option is on.
    pub fn close_screen(&self) -> Result<SyncStatusResponse, CommandError> {
        #[cfg(desktop)]
        if self.role == SyncRole::Computer {
            self.server.screen_closed(self.receive_always());
        }
        self.status()
    }

    /// « Recevoir les synchronisations » even with the screen closed.
    pub fn set_receive(&self, enabled: bool) -> Result<SyncStatusResponse, CommandError> {
        if self.role != SyncRole::Computer {
            return Err(self.not_here());
        }
        self.store
            .set_value(
                educator_sync::RECEIVE_KEY,
                if enabled { "on" } else { "off" },
            )
            .map_err(failed)?;
        #[cfg(desktop)]
        if enabled {
            self.server.ensure_running(self.store.as_ref());
        } else {
            // Still listening while the screen is open.
            self.server.stop_if_idle();
        }
        self.status()
    }

    /// « Associer une tablette »: a new QR code and short code, valid 5 minutes.
    pub fn start_pairing(&self) -> Result<SyncPairingResponse, CommandError> {
        if self.role != SyncRole::Computer {
            return Err(self.not_here());
        }
        #[cfg(desktop)]
        {
            self.server.start_pairing(self.store.as_ref())
        }
        #[cfg(not(desktop))]
        {
            Err(self.not_here())
        }
    }

    pub fn cancel_pairing(&self) -> Result<SyncStatusResponse, CommandError> {
        #[cfg(desktop)]
        self.server.cancel_pairing();
        self.status()
    }

    pub fn rename_device(&self, id: &str, name: &str) -> Result<SyncStatusResponse, CommandError> {
        let name = name.split_whitespace().collect::<Vec<_>>().join(" ");
        if name.is_empty() || name.chars().count() > MAX_DEVICE_NAME {
            return Err(CommandError::new(
                400,
                "Invalid request",
                "Bad device name.",
            ));
        }
        let mut peer = self
            .store
            .peer(parse_id(id)?)
            .map_err(failed)?
            .ok_or_else(|| CommandError::new(404, "Unknown device", id.to_owned()))?;
        peer.name = name;
        self.store.save_peer(&peer).map_err(failed)?;
        self.status()
    }

    /// Computer: « Retirer cette tablette » (its token is refused at once). iPad: « Dissocier ».
    pub fn remove_device(&self, id: &str) -> Result<SyncStatusResponse, CommandError> {
        let device_id = parse_id(id)?;
        match self.role {
            SyncRole::Computer => {
                if let Some(mut peer) = self.store.peer(device_id).map_err(failed)? {
                    peer.revoked = true;
                    peer.token_hash = None;
                    self.store.save_peer(&peer).map_err(failed)?;
                    log::info!(
                        "Synchronisation: tablet {} removed.",
                        educator_core::sync::short(device_id)
                    );
                }
            }
            SyncRole::Tablet => self.client.forget().map_err(failed)?,
        }
        self.status()
    }

    fn result(
        &self,
        result: Result<SyncOutcome, SyncError>,
    ) -> Result<SyncResultResponse, CommandError> {
        if let Err(SyncError::Failed(message)) = &result {
            log::warn!("Synchronisation failed: {message}");
        }
        let (outcome, sent, received) = SyncOutcomeName::of(&result);
        Ok(SyncResultResponse {
            outcome,
            sent,
            received,
            status: self.status()?,
        })
    }

    /// iPad: pairs with the computer (QR code, or address and short code).
    pub fn pair(
        &self,
        request: &PairWithComputerRequest,
    ) -> Result<SyncResultResponse, CommandError> {
        if self.role != SyncRole::Tablet {
            return Err(self.not_here());
        }
        let paired = match (&request.link, &request.address, &request.code) {
            (Some(link), _, _) => self.client.pair_with_link(link),
            (None, Some(address), Some(code)) => self.client.pair_with_code(address, code),
            _ => Err(SyncError::InvalidLink),
        };
        self.result(paired.map(|_| SyncOutcome::Done {
            sent: 0,
            received: 0,
        }))
    }

    /// iPad: « Synchroniser maintenant », or the automatic attempt (`automatic`: nothing happens
    /// without a computer or while a question waits). `busy`: a series is in progress.
    pub fn sync_now(
        &self,
        automatic: bool,
        busy: &dyn Fn() -> bool,
    ) -> Result<SyncResultResponse, CommandError> {
        if self.role != SyncRole::Tablet {
            return Err(self.not_here());
        }
        if automatic && !self.client.pending_conflicts().is_empty() {
            return self.result(Ok(SyncOutcome::NeedsDecision(
                self.client.pending_conflicts(),
            )));
        }
        self.interrupted.store(false, Ordering::SeqCst);
        let interrupted = || self.interrupted.load(Ordering::SeqCst);
        self.result(self.client.synchronise(busy, &interrupted))
    }

    /// iPad: the parent's answer to « Est-ce le même enfant ? ».
    pub fn answer(
        &self,
        local_profile_id: &str,
        same_child: bool,
    ) -> Result<SyncStatusResponse, CommandError> {
        if self.role != SyncRole::Tablet {
            return Err(self.not_here());
        }
        let local = Uuid::parse_str(local_profile_id)
            .map_err(|_| CommandError::new(400, "Invalid request", "Unknown profile."))?;
        self.client.resolve(local, same_child).map_err(|e| {
            log::warn!("Synchronisation: answer not applied: {e}");
            CommandError::new(409, "Question not answered", e.to_string())
        })?;
        self.status()
    }

    /// The application goes to the background: the exchange stops after its current batch.
    pub fn interrupt(&self) {
        self.interrupted.store(true, Ordering::SeqCst);
    }

    /// Tests: the computer is reached on this machine.
    #[cfg(all(test, desktop))]
    pub fn use_loopback(&self) {
        self.server.use_addresses(|| vec!["127.0.0.1".to_owned()]);
    }
}

#[cfg(desktop)]
mod desktop {
    use crate::contracts::{CommandError, SyncPairingResponse};
    use educator_core::sync::SyncStore;
    use educator_sync::discovery::Announce;
    use educator_sync::pairing::{qr_svg, PairingLink};
    use educator_sync::server::{local_addresses, Handler, RunningServer, ServerOptions};
    use educator_sync::service::{ServerHandler, SyncDependencies};
    use std::sync::{Arc, Mutex, MutexGuard};

    /// Preferred port, when the computer has none kept yet (any other if it is taken).
    const DEFAULT_PORT: u16 = 47615;

    #[derive(Default)]
    struct State {
        server: Option<RunningServer>,
        announce: Option<Announce>,
        screens: usize,
        problem: Option<String>,
        fingerprint: Option<String>,
    }

    pub(super) struct ServerSide {
        handler: Arc<ServerHandler>,
        state: Mutex<State>,
        /// The private addresses of the computer (replaced by the loopback in the tests).
        addresses: Mutex<fn() -> Vec<String>>,
    }

    impl ServerSide {
        pub(super) fn new(deps: SyncDependencies) -> Self {
            Self {
                handler: Arc::new(ServerHandler::new(deps)),
                state: Mutex::new(State::default()),
                addresses: Mutex::new(local_addresses),
            }
        }

        fn addresses(&self) -> Vec<String> {
            (self.addresses.lock().unwrap_or_else(|e| e.into_inner()))()
        }

        #[cfg(test)]
        pub(super) fn use_addresses(&self, addresses: fn() -> Vec<String>) {
            *self.addresses.lock().unwrap_or_else(|e| e.into_inner()) = addresses;
        }

        fn lock(&self) -> MutexGuard<'_, State> {
            self.state.lock().unwrap_or_else(|e| e.into_inner())
        }

        /// (listening, address, problem)
        pub(super) fn state(&self) -> (bool, Option<String>, Option<String>) {
            let state = self.lock();
            let address = state.server.as_ref().and_then(|server| {
                self.addresses()
                    .first()
                    .map(|ip| format!("{ip}:{}", server.port()))
            });
            (state.server.is_some(), address, state.problem.clone())
        }

        pub(super) fn ensure_running(&self, store: &dyn SyncStore) {
            let mut state = self.lock();
            if state.server.is_some() {
                return;
            }
            let identity = match educator_sync::server_identity(store) {
                Ok(identity) => identity,
                Err(e) => {
                    log::error!("Synchronisation: {e}");
                    state.problem = Some(e);
                    return;
                }
            };
            let port = store
                .value(educator_sync::PORT_KEY)
                .ok()
                .flatten()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_PORT);
            let handler: Arc<dyn Handler> = self.handler.clone();
            match RunningServer::start(
                &identity,
                ServerOptions {
                    port,
                    ..ServerOptions::default()
                },
                handler,
            ) {
                Ok(server) => {
                    let _ = store.set_value(educator_sync::PORT_KEY, &server.port().to_string());
                    let addresses = self.addresses();
                    state.announce = store.device_id().ok().and_then(|id| {
                        Announce::start(id, server.port(), &addresses)
                            .map_err(|e| log::warn!("Synchronisation: no Bonjour announce: {e}"))
                            .ok()
                    });
                    state.fingerprint = identity.fingerprint().ok();
                    state.problem = None;
                    state.server = Some(server);
                }
                Err(e) => {
                    log::error!("{e}");
                    state.problem = Some(e.to_string());
                }
            }
        }

        fn stop(state: &mut State, handler: &ServerHandler) {
            handler.close_pairing();
            state.announce = None;
            if let Some(mut server) = state.server.take() {
                server.stop();
            }
        }

        pub(super) fn screen_opened(&self, store: &dyn SyncStore) {
            self.lock().screens += 1;
            self.ensure_running(store);
        }

        /// Stops listening when no screen « Synchronisation » is open.
        pub(super) fn stop_if_idle(&self) {
            let mut state = self.lock();
            if state.screens == 0 {
                Self::stop(&mut state, &self.handler);
            }
        }

        pub(super) fn screen_closed(&self, keep_receiving: bool) {
            let mut state = self.lock();
            state.screens = state.screens.saturating_sub(1);
            self.handler.close_pairing();
            if state.screens == 0 && !keep_receiving {
                Self::stop(&mut state, &self.handler);
            }
        }

        pub(super) fn start_pairing(
            &self,
            store: &dyn SyncStore,
        ) -> Result<SyncPairingResponse, CommandError> {
            self.ensure_running(store);
            let state = self.lock();
            let (Some(server), Some(fingerprint)) = (&state.server, &state.fingerprint) else {
                return Err(CommandError::new(
                    503,
                    "Not receiving",
                    state.problem.clone().unwrap_or_default(),
                ));
            };
            let addresses = self.addresses();
            if addresses.is_empty() {
                return Err(CommandError::new(
                    503,
                    "No local network",
                    "No private address.",
                ));
            }
            let offer = self.handler.open_pairing();
            let link = PairingLink {
                addresses: addresses.clone(),
                port: server.port(),
                fingerprint: fingerprint.clone(),
                secret: offer.secret.clone(),
            };
            Ok(SyncPairingResponse {
                qr_svg: qr_svg(&link.to_uri()).unwrap_or_default(),
                code: offer.display_code(),
                address: format!("{}:{}", addresses[0], server.port()),
                expires_at: crate::contracts::iso_8601(offer.expires_at),
            })
        }

        pub(super) fn cancel_pairing(&self) {
            self.handler.close_pairing();
        }
    }

    impl Drop for ServerSide {
        fn drop(&mut self) {
            let mut state = self.lock();
            Self::stop(&mut state, &self.handler);
        }
    }
}

#[cfg(all(test, desktop))]
mod tests {
    use super::*;
    use crate::services::{Locations, Services};
    use educator_core::clock::SystemClock;
    use educator_core::random::ThreadRandom;
    use educator_core::skills::Date;
    use educator_core::speech::UnavailableSpeechSynthesizer;
    use std::path::Path;

    struct Device {
        _dir: tempfile::TempDir,
        services: Services,
    }

    fn device(role: SyncRole) -> Device {
        let dir = tempfile::tempdir().unwrap();
        let services = Services::start_as(
            role,
            &Locations {
                content_directory: Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content"),
                database_path: dir.path().join("educator.db"),
            },
            Arc::new(UnavailableSpeechSynthesizer::new("tests")),
            Arc::new(ThreadRandom),
            Arc::new(SystemClock),
            Box::new(|| Date::new(2026, 10, 15)),
            Arc::new(educator_core::statistics::utc_date_of),
        )
        .unwrap();
        services.synchronisation().use_loopback();
        Device {
            _dir: dir,
            services,
        }
    }

    fn create(services: &Services, name: &str) -> ProfileResponse {
        services
            .create_profile(CreateProfileRequest {
                name: name.to_owned(),
                grade: Grade::CP,
                character_id: None,
            })
            .unwrap()
    }

    fn names(services: &Services) -> Vec<String> {
        let mut names: Vec<String> = services
            .list_profiles()
            .unwrap()
            .into_iter()
            .map(|p| p.name)
            .collect();
        names.sort();
        names
    }

    fn pair(computer: &Device, tablet: &Device) {
        let pairing = computer.services.synchronisation().start_pairing().unwrap();
        let port = pairing.address.rsplit(':').next().unwrap();
        let paired = tablet
            .services
            .synchronisation()
            .pair(&PairWithComputerRequest {
                link: None,
                address: Some(format!("127.0.0.1:{port}")),
                code: Some(pairing.code.clone()),
            })
            .unwrap();
        assert_eq!(SyncOutcomeName::Done, paired.outcome);
    }

    #[test]
    fn the_computer_listens_while_the_screen_is_open_or_the_option_is_on() {
        let computer = device(SyncRole::Computer);
        let sync = computer.services.synchronisation();
        assert!(!sync.status().unwrap().receiving);

        assert!(sync.open_screen().unwrap().receiving);
        let pairing = sync.start_pairing().unwrap();
        assert!(pairing.qr_svg.contains("<svg"));
        assert_eq!(9, pairing.code.len());
        assert!(!sync.close_screen().unwrap().receiving);

        assert!(sync.set_receive(true).unwrap().receiving);
        assert!(sync.close_screen().unwrap().receiving);
        let status = sync.set_receive(false).unwrap();
        assert!(!status.receiving && !status.receive_always);
        // Turning the option off while the screen is open keeps the screen working.
        sync.open_screen().unwrap();
        sync.set_receive(true).unwrap();
        assert!(sync.set_receive(false).unwrap().receiving);
        assert!(!sync.close_screen().unwrap().receiving);
        // Tablet commands are not offered by the computer.
        assert_eq!(409, sync.sync_now(false, &|| false).unwrap_err().status);
    }

    #[test]
    fn a_tablet_pairs_synchronises_and_is_removed() {
        let computer = device(SyncRole::Computer);
        let tablet = device(SyncRole::Tablet);
        computer.services.synchronisation().open_screen().unwrap();
        create(&computer.services, "Léa");
        create(&tablet.services, "Tom");

        pair(&computer, &tablet);
        let result = tablet.services.sync_now(false).unwrap();

        assert_eq!(SyncOutcomeName::Done, result.outcome);
        assert_eq!(vec!["Léa", "Tom"], names(&computer.services));
        assert_eq!(names(&computer.services), names(&tablet.services));
        let tablets = computer
            .services
            .synchronisation()
            .status()
            .unwrap()
            .devices;
        assert_eq!(
            vec!["Tablette 1"],
            tablets.iter().map(|d| d.name.as_str()).collect::<Vec<_>>()
        );
        assert!(tablets[0].last_sync_at.is_some());
        assert_eq!(1, result.status.devices.len());

        // « Retirer cette tablette »: refused at once.
        computer
            .services
            .synchronisation()
            .remove_device(&tablets[0].id)
            .unwrap();
        let refused = tablet.services.sync_now(false).unwrap();
        assert_eq!(SyncOutcomeName::Unauthorized, refused.outcome);
        assert!(computer
            .services
            .synchronisation()
            .status()
            .unwrap()
            .devices
            .is_empty());
    }

    #[test]
    fn the_same_name_asks_the_parent_and_nothing_starts_during_a_series() {
        let computer = device(SyncRole::Computer);
        let tablet = device(SyncRole::Tablet);
        computer.services.synchronisation().open_screen().unwrap();
        create(&computer.services, "Léa");
        let lea = create(&tablet.services, "Léa");
        pair(&computer, &tablet);

        // The automatic attempt at the profile choice stops at the question.
        let asked = tablet.services.sync_now(true).unwrap();
        assert_eq!(SyncOutcomeName::NeedsDecision, asked.outcome);
        assert_eq!(1, asked.status.questions.len());
        assert_eq!(lea.id, asked.status.questions[0].local_profile_id);
        let again = tablet.services.sync_now(true).unwrap();
        assert_eq!(SyncOutcomeName::NeedsDecision, again.outcome);

        let status = tablet
            .services
            .synchronisation()
            .answer(&lea.id, true)
            .unwrap();
        assert!(status.questions.is_empty());

        // A series in progress: nothing starts.
        let profile = tablet.services.list_profiles().unwrap().remove(0);
        tablet
            .services
            .start_series(StartSeriesRequest {
                profile_id: profile.id.clone(),
                skill_id: "cp-add-sum-10".to_owned(),
                answer_mode: AnswerMode::Keyboard,
                format: None,
                seed: Some(1),
            })
            .unwrap();
        assert_eq!(
            SyncOutcomeName::Busy,
            tablet.services.sync_now(false).unwrap().outcome
        );
    }

    #[test]
    fn a_tablet_without_computer_or_with_the_computer_off_keeps_working() {
        let tablet = device(SyncRole::Tablet);
        assert_eq!(
            SyncOutcomeName::NotPaired,
            tablet.services.sync_now(true).unwrap().outcome
        );

        let computer = device(SyncRole::Computer);
        computer.services.synchronisation().open_screen().unwrap();
        pair(&computer, &tablet);
        computer.services.synchronisation().close_screen().unwrap();

        let result = tablet.services.sync_now(true).unwrap();
        assert_eq!(SyncOutcomeName::Unreachable, result.outcome);
        create(&tablet.services, "Tom");
        assert_eq!(vec!["Tom"], names(&tablet.services));
        let status = tablet
            .services
            .synchronisation()
            .remove_device(&result.status.devices[0].id)
            .unwrap();
        assert!(status.devices.is_empty());
    }
}
