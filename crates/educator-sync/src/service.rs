//! The synchronisation on each side (ADR 0007): the routes of the computer ([`ServerHandler`]) and
//! the exchange driven by the iPad ([`ClientSync`]): pairing, « Est-ce le même enfant ? »,
//! push, pull, push again, cursors saved after each batch, masteries and points recomputed.

use crate::client::{Client, ClientError, ComputerAddress};
use crate::http::{Request, Response};
use crate::pairing::{new_token, token_hash, Pairing, PairingError, PairingLink, PairingOffer};
use crate::tls::same_secret;
use crate::wire::*;
use educator_core::clock::Clock;
use educator_core::profiles::{Profile, ProfileStore};
use educator_core::rewards::RewardStore;
use educator_core::sessions::SessionStore;
use educator_core::sync::{
    distinct_name, name_conflicts, refresh_pending, short, ChangeBatch, ExportRequest,
    NameConflict, PeerRole, SyncPeer, SyncStore,
};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Failed attempts (pairing, token) from one address in a minute before it is refused for a while.
pub const MAX_FAILURES_PER_MINUTE: usize = 5;

/// What both sides need.
#[derive(Clone)]
pub struct SyncDependencies {
    pub store: Arc<dyn SyncStore>,
    pub profiles: Arc<dyn ProfileStore>,
    pub sessions: Arc<dyn SessionStore>,
    pub rewards: Arc<dyn RewardStore>,
    pub clock: Arc<dyn Clock>,
}

impl fmt::Debug for SyncDependencies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SyncDependencies").finish_non_exhaustive()
    }
}

impl SyncDependencies {
    fn refresh(&self) -> educator_core::Result<usize> {
        refresh_pending(
            self.sessions.as_ref(),
            self.rewards.as_ref(),
            self.store.as_ref(),
            self.clock.now_ms(),
        )
    }
}

fn refusal(status: u16, code: &str, message: impl Into<String>) -> Response {
    Response::json(
        status,
        &ErrorResponse {
            protocol: PROTOCOL_VERSION,
            code: code.to_owned(),
            message: message.into(),
        },
    )
}

fn unavailable(e: impl fmt::Display) -> Response {
    log::error!("Synchronisation: {e}");
    refusal(503, "unavailable", "The computer could not do it.")
}

// ----- Computer -------------------------------------------------------------------------------

/// The routes of the computer: `/v1/pair`, `/v1/hello`, `/v1/push`, `/v1/pull`.
pub struct ServerHandler {
    deps: SyncDependencies,
    pairing: Mutex<Pairing>,
    failures: Mutex<HashMap<IpAddr, Vec<i64>>>,
}

impl fmt::Debug for ServerHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServerHandler").finish_non_exhaustive()
    }
}

impl ServerHandler {
    pub fn new(deps: SyncDependencies) -> Self {
        Self {
            deps,
            pairing: Mutex::new(Pairing::default()),
            failures: Mutex::new(HashMap::new()),
        }
    }

    fn pairing(&self) -> std::sync::MutexGuard<'_, Pairing> {
        self.pairing.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// A new pairing offer (« Associer une tablette »).
    pub fn open_pairing(&self) -> PairingOffer {
        self.pairing().open(self.deps.clock.now_ms())
    }

    /// The offer still valid, if any.
    pub fn current_pairing(&self) -> Option<PairingOffer> {
        self.pairing().current(self.deps.clock.now_ms())
    }

    pub fn close_pairing(&self) {
        self.pairing().close();
    }

    fn failures(&self) -> std::sync::MutexGuard<'_, HashMap<IpAddr, Vec<i64>>> {
        self.failures.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn limited(&self, from: IpAddr, now: i64) -> bool {
        let mut failures = self.failures();
        let recent = failures.entry(from).or_default();
        recent.retain(|at| now - at < 60_000);
        recent.len() >= MAX_FAILURES_PER_MINUTE
    }

    fn failed(&self, from: IpAddr, now: i64) {
        self.failures().entry(from).or_default().push(now);
    }

    /// The tablet presenting this token, if it is paired and not removed.
    fn authenticate(&self, device_id: Uuid, token: Option<&str>) -> Option<SyncPeer> {
        let token = token?;
        let peer = self.deps.store.peer(device_id).ok()??;
        let expected = peer.token_hash.as_deref()?;
        (peer.role == PeerRole::Tablet
            && !peer.revoked
            && same_secret(&token_hash(token), expected))
        .then_some(peer)
    }

    /// « Tablette 1 », « Tablette 2 »…: the first free.
    fn tablet_name(&self) -> educator_core::Result<String> {
        let taken: HashSet<String> = self
            .deps
            .store
            .peers()?
            .into_iter()
            .filter(|p| p.role == PeerRole::Tablet && !p.revoked)
            .map(|p| p.name)
            .collect();
        Ok((1..)
            .map(|n| format!("Tablette {n}"))
            .find(|name| !taken.contains(name))
            .unwrap_or_default())
    }

    fn pair(&self, request: &PairRequest, from: IpAddr, now: i64) -> Response {
        let outcome = self.pairing().consume(&request.secret, now);
        match outcome {
            Ok(()) => {}
            Err(PairingError::TooMany) => {
                self.failed(from, now);
                return refusal(429, "tooMany", "Too many wrong codes.");
            }
            Err(_) => {
                self.failed(from, now);
                return refusal(403, "expired", "No valid pairing offer for this secret.");
            }
        }
        let token = new_token();
        let existing = self.deps.store.peer(request.device_id).ok().flatten();
        let name = match existing {
            Some(peer) if !peer.revoked => peer.name,
            _ => match self.tablet_name() {
                Ok(name) => name,
                Err(e) => return unavailable(e),
            },
        };
        let peer = SyncPeer {
            device_id: request.device_id,
            role: PeerRole::Tablet,
            name,
            token_hash: Some(token_hash(&token)),
            token: None,
            fingerprint: None,
            addresses: Vec::new(),
            port: None,
            paired_at: now,
            last_sync_at: None,
            pushed_seq: 0,
            pulled_seq: 0,
            revoked: false,
            epoch: None,
        };
        if let Err(e) = self.deps.store.save_peer(&peer) {
            return unavailable(e);
        }
        let device_id = match self.deps.store.device_id() {
            Ok(id) => id,
            Err(e) => return unavailable(e),
        };
        log::info!(
            "Synchronisation: tablet {} paired.",
            short(request.device_id)
        );
        Response::json(
            200,
            &PairResponse {
                protocol: PROTOCOL_VERSION,
                device_id,
                token,
            },
        )
    }

    fn hello(&self) -> Response {
        let (device_id, epoch, profiles) = match (
            self.deps.store.device_id(),
            self.deps.store.epoch(),
            self.deps.store.profile_states(),
        ) {
            (Ok(id), Ok(epoch), Ok(profiles)) => (id, epoch, profiles),
            (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => return unavailable(e),
        };
        Response::json(
            200,
            &HelloResponse {
                protocol: PROTOCOL_VERSION,
                device_id,
                epoch,
                profiles: profiles.iter().map(WireProfile::from).collect(),
            },
        )
    }

    fn push(&self, request: &PushRequest, peer: &SyncPeer) -> Response {
        let batch = match ChangeBatch::try_from(&request.batch) {
            Ok(batch) => batch,
            Err(e) => return refusal(400, "invalid", e.to_string()),
        };
        match self.deps.store.apply(&batch, None) {
            Ok(report) => {
                log::info!(
                    "Synchronisation: from tablet {}: {batch}; {} session(s) added.",
                    short(peer.device_id),
                    report.sessions_added
                );
            }
            Err(educator_core::Error::InvalidRequest(m)) => return refusal(400, "invalid", m),
            Err(e) => return unavailable(e),
        }
        if let Err(e) = self.deps.refresh() {
            return unavailable(e);
        }
        Response::json(
            200,
            &PushResponse {
                protocol: PROTOCOL_VERSION,
                acknowledged: request.last_seq,
            },
        )
    }

    fn pull(&self, request: &PullRequest, peer: &SyncPeer, now: i64) -> Response {
        let page = match self
            .deps
            .store
            .export(&ExportRequest::after(request.after, Some(peer.device_id)))
        {
            Ok(page) => page,
            Err(e) => return unavailable(e),
        };
        // A targeted update: a tablet removed meanwhile stays removed.
        if !page.more {
            if let Err(e) = self.deps.store.record_sync(peer.device_id, now, None) {
                return unavailable(e);
            }
        }
        log::info!(
            "Synchronisation: to tablet {}: {}.",
            short(peer.device_id),
            page.batch
        );
        Response::json(
            200,
            &PullResponse {
                protocol: PROTOCOL_VERSION,
                last_seq: page.last_seq,
                more: page.more,
                batch: WireBatch::from(&page.batch),
            },
        )
    }

    fn route<T: for<'de> serde::Deserialize<'de>>(
        &self,
        request: &Request,
        from: IpAddr,
        now: i64,
        answer: impl FnOnce(&T, Option<SyncPeer>) -> Response,
        device_of: impl Fn(&T) -> Uuid,
        authenticated: bool,
    ) -> Response {
        let message = match read_message::<T>(&request.body) {
            Ok(message) => message,
            Err(MessageError::Incompatible(_)) => {
                return refusal(409, "incompatible", "Another version of the protocol.")
            }
            Err(MessageError::Malformed(m)) => return refusal(400, "invalid", m),
        };
        if !authenticated {
            return answer(&message, None);
        }
        match self.authenticate(device_of(&message), request.bearer()) {
            Some(peer) => answer(&message, Some(peer)),
            None => {
                self.failed(from, now);
                refusal(401, "unauthorized", "This tablet is not paired.")
            }
        }
    }
}

impl crate::http::Handler for ServerHandler {
    fn handle(&self, request: &Request, from: IpAddr) -> Response {
        let now = self.deps.clock.now_ms();
        if self.limited(from, now) {
            return refusal(429, "tooMany", "Too many failed attempts.");
        }
        if request.method != "POST" {
            return refusal(404, "invalid", "Unknown route.");
        }
        match request.path.as_str() {
            "/v1/pair" => self.route::<PairRequest>(
                request,
                from,
                now,
                |m, _| self.pair(m, from, now),
                |m| m.device_id,
                false,
            ),
            "/v1/hello" => self.route::<HelloRequest>(
                request,
                from,
                now,
                |_, _| self.hello(),
                |m| m.device_id,
                true,
            ),
            "/v1/push" => self.route::<PushRequest>(
                request,
                from,
                now,
                |m, peer| match peer {
                    Some(peer) => self.push(m, &peer),
                    None => refusal(401, "unauthorized", "Not paired."),
                },
                |m| m.device_id,
                true,
            ),
            "/v1/pull" => self.route::<PullRequest>(
                request,
                from,
                now,
                |m, peer| match peer {
                    Some(peer) => self.pull(m, &peer, now),
                    None => refusal(401, "unauthorized", "Not paired."),
                },
                |m| m.device_id,
                true,
            ),
            _ => refusal(404, "invalid", "Unknown route."),
        }
    }
}

// ----- iPad -----------------------------------------------------------------------------------

/// Why a synchronisation did not happen (shown to the parent by the front, never to the child).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncError {
    /// No computer paired.
    NotPaired,
    /// A series, an exam, a quiz or a timed challenge is in progress.
    Busy,
    /// The computer did not answer (off, other Wi-Fi, reception off, firewall).
    Unreachable,
    /// Another machine answered with another certificate.
    CertificateMismatch,
    /// The computer removed this tablet (or never knew it).
    Unauthorized,
    /// The other device runs another version of the protocol.
    Incompatible,
    /// The QR code or the short code is no longer valid.
    PairingExpired,
    /// Too many wrong codes.
    TooManyAttempts,
    /// Not a QR code of Educator.
    InvalidLink,
    /// Stopped in the middle (application in the background): resumed next time.
    Interrupted,
    Failed(String),
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyncError::Failed(m) => write!(f, "Synchronisation failed: {m}"),
            other => write!(f, "Synchronisation: {other:?}"),
        }
    }
}

impl std::error::Error for SyncError {}

impl From<ClientError> for SyncError {
    fn from(e: ClientError) -> Self {
        match e {
            ClientError::Unreachable => SyncError::Unreachable,
            ClientError::CertificateMismatch => SyncError::CertificateMismatch,
            ClientError::Incompatible => SyncError::Incompatible,
            ClientError::Refused { status: 401, .. } => SyncError::Unauthorized,
            ClientError::Refused { code, .. } if code == "expired" => SyncError::PairingExpired,
            ClientError::Refused { code, .. } if code == "tooMany" => SyncError::TooManyAttempts,
            other => SyncError::Failed(other.to_string()),
        }
    }
}

impl From<educator_core::Error> for SyncError {
    fn from(e: educator_core::Error) -> Self {
        SyncError::Failed(e.to_string())
    }
}

/// What a synchronisation did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncOutcome {
    Done {
        sent: usize,
        received: usize,
    },
    /// The parent says first whether these profiles are the same children.
    NeedsDecision(Vec<NameConflict>),
}

/// The synchronisation driven by the iPad.
pub struct ClientSync {
    deps: SyncDependencies,
    /// The questions « Est-ce le même enfant ? » of the last attempt.
    pending: Mutex<Vec<NameConflict>>,
    /// The names of the profiles of the computer at the last attempt (a name made distinct must be
    /// free there too).
    remote_names: Mutex<Vec<String>>,
    /// One synchronisation at a time.
    running: Mutex<()>,
}

impl fmt::Debug for ClientSync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientSync").finish_non_exhaustive()
    }
}

/// Name given to the computer on the iPad.
pub const COMPUTER_NAME: &str = "Ordinateur";

impl ClientSync {
    pub fn new(deps: SyncDependencies) -> Self {
        Self {
            deps,
            pending: Mutex::new(Vec::new()),
            remote_names: Mutex::new(Vec::new()),
            running: Mutex::new(()),
        }
    }

    fn pending(&self) -> std::sync::MutexGuard<'_, Vec<NameConflict>> {
        self.pending.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// The computer this iPad is paired with.
    pub fn computer(&self) -> educator_core::Result<Option<SyncPeer>> {
        Ok(self
            .deps
            .store
            .peers()?
            .into_iter()
            .find(|p| p.role == PeerRole::Computer && !p.revoked))
    }

    /// Pairs with the computer of a QR code.
    pub fn pair_with_link(&self, uri: &str) -> Result<SyncPeer, SyncError> {
        let link = PairingLink::parse(uri).ok_or(SyncError::InvalidLink)?;
        self.pair(
            ComputerAddress {
                addresses: link.addresses.clone(),
                port: link.port,
                fingerprint: Some(link.fingerprint.clone()),
            },
            &link.secret,
        )
    }

    /// Plan B without camera: the address shown by the computer (`192.168.1.10:47615`) and the
    /// short code. The certificate of the first answer is kept.
    pub fn pair_with_code(&self, address: &str, code: &str) -> Result<SyncPeer, SyncError> {
        let target: std::net::SocketAddr = address
            .trim()
            .replace(' ', "")
            .parse()
            .map_err(|_| SyncError::InvalidLink)?;
        if !crate::http::is_local_address(target.ip()) {
            return Err(SyncError::InvalidLink);
        }
        self.pair(
            ComputerAddress {
                addresses: vec![target.ip().to_string()],
                port: target.port(),
                fingerprint: None,
            },
            code,
        )
    }

    fn pair(&self, computer: ComputerAddress, secret: &str) -> Result<SyncPeer, SyncError> {
        let device_id = self.deps.store.device_id()?;
        let client = Client::new(computer.clone());
        let reply = client.post::<_, PairResponse>(
            "/v1/pair",
            None,
            &PairRequest {
                protocol: PROTOCOL_VERSION,
                device_id,
                secret: secret.to_owned(),
            },
        )?;
        let fingerprint = computer
            .fingerprint
            .or(reply.fingerprint)
            .ok_or_else(|| SyncError::Failed("No certificate seen.".to_owned()))?;
        let previous = self.deps.store.peer(reply.value.device_id)?;
        // One computer at a time: another one paired before is forgotten.
        for other in self.deps.store.peers()? {
            if other.role == PeerRole::Computer && other.device_id != reply.value.device_id {
                self.deps.store.delete_peer(other.device_id)?;
            }
        }
        let mut addresses = vec![reply.address.clone()];
        addresses.extend(
            computer
                .addresses
                .into_iter()
                .filter(|a| *a != reply.address),
        );
        let peer = SyncPeer {
            device_id: reply.value.device_id,
            role: PeerRole::Computer,
            name: previous
                .as_ref()
                .map_or_else(|| COMPUTER_NAME.to_owned(), |p| p.name.clone()),
            token_hash: None,
            token: Some(reply.value.token),
            fingerprint: Some(fingerprint),
            addresses,
            port: Some(computer.port),
            paired_at: self.deps.clock.now_ms(),
            last_sync_at: previous.as_ref().and_then(|p| p.last_sync_at),
            pushed_seq: previous.as_ref().map_or(0, |p| p.pushed_seq),
            pulled_seq: previous.as_ref().map_or(0, |p| p.pulled_seq),
            revoked: false,
            epoch: previous.as_ref().and_then(|p| p.epoch.clone()),
        };
        self.deps.store.save_peer(&peer)?;
        log::info!(
            "Synchronisation: paired with computer {}.",
            short(peer.device_id)
        );
        Ok(peer)
    }

    /// « Dissocier »: forgets the computer (its data stay here).
    pub fn forget(&self) -> educator_core::Result<()> {
        if let Some(peer) = self.computer()? {
            self.deps.store.delete_peer(peer.device_id)?;
        }
        self.pending().clear();
        Ok(())
    }

    /// The questions waiting for the parent.
    pub fn pending_conflicts(&self) -> Vec<NameConflict> {
        self.pending().clone()
    }

    /// The parent's answer to « Est-ce le même enfant ? » for a local profile.
    pub fn resolve(&self, local_id: Uuid, same_child: bool) -> Result<(), SyncError> {
        let conflict = self
            .pending()
            .iter()
            .find(|c| c.local_id == local_id)
            .cloned()
            .ok_or_else(|| SyncError::Failed("No such question.".to_owned()))?;
        if same_child {
            self.deps.store.link_profile(local_id, &conflict.remote)?;
            self.deps.refresh()?;
        } else {
            let mut profile = self
                .deps
                .profiles
                .get(local_id)?
                .ok_or(educator_core::Error::ProfileNotFound(local_id))?;
            let mut taken: HashSet<String> = self
                .deps
                .profiles
                .list()?
                .iter()
                .map(|p| Profile::normalize_name(&p.name))
                .collect();
            for name in self
                .remote_names
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .iter()
            {
                taken.insert(Profile::normalize_name(name));
            }
            profile.name = distinct_name(&profile.name, &taken);
            self.deps.profiles.update(&profile)?;
        }
        self.pending().retain(|c| c.local_id != local_id);
        Ok(())
    }

    /// Push, pull, push: everything the two devices have that the other has not. `busy`: a series
    /// is in progress (nothing starts); `interrupted`: asked between two batches (the application
    /// goes to the background), the exchange stops and resumes next time.
    pub fn synchronise(
        &self,
        busy: &dyn Fn() -> bool,
        interrupted: &dyn Fn() -> bool,
    ) -> Result<SyncOutcome, SyncError> {
        let Ok(_running) = self.running.try_lock() else {
            return Err(SyncError::Busy);
        };
        if busy() {
            return Err(SyncError::Busy);
        }
        let mut peer = self.computer()?.ok_or(SyncError::NotPaired)?;
        let token = peer.token.clone().ok_or(SyncError::NotPaired)?;
        let device_id = self.deps.store.device_id()?;
        let client = Client::new(ComputerAddress {
            addresses: peer.addresses.clone(),
            port: peer.port.ok_or(SyncError::NotPaired)?,
            fingerprint: peer.fingerprint.clone(),
        });

        let hello = client.post::<_, HelloResponse>(
            "/v1/hello",
            Some(&token),
            &HelloRequest {
                protocol: PROTOCOL_VERSION,
                device_id,
            },
        )?;
        if hello.value.device_id != peer.device_id {
            return Err(SyncError::CertificateMismatch);
        }
        let mut remote =
            profile_states(&hello.value.profiles).map_err(|e| SyncError::Failed(e.to_string()))?;
        *self.remote_names.lock().unwrap_or_else(|e| e.into_inner()) =
            remote.iter().map(|p| p.name.clone()).collect();
        // A profile removed here is no question: its removal will reach the computer.
        let mut kept = Vec::with_capacity(remote.len());
        for profile in remote {
            if !self.deps.store.is_removed(profile.id)? {
                kept.push(profile);
            }
        }
        remote = kept;
        // The computer restored a backup (or this is the first time): everything again, both ways.
        if peer.epoch.as_deref() != Some(hello.value.epoch.as_str()) {
            self.deps
                .store
                .reset_cursors(peer.device_id, &hello.value.epoch)?;
            peer.pushed_seq = 0;
            peer.pulled_seq = 0;
            peer.epoch = Some(hello.value.epoch.clone());
        }
        let local: Vec<(Uuid, String)> = self
            .deps
            .store
            .profile_states()?
            .into_iter()
            .map(|p| (p.id, p.name))
            .collect();
        let conflicts = name_conflicts(&local, &remote);
        *self.pending() = conflicts.clone();
        if !conflicts.is_empty() {
            return Ok(SyncOutcome::NeedsDecision(conflicts));
        }
        // The address that answered goes first next time.
        peer.addresses.retain(|a| *a != hello.address);
        peer.addresses.insert(0, hello.address.clone());

        let mut sent = self.push(&client, &token, &mut peer, device_id, interrupted)?;
        let mut received = 0;
        loop {
            if interrupted() {
                return Err(SyncError::Interrupted);
            }
            let reply = client.post::<_, PullResponse>(
                "/v1/pull",
                Some(&token),
                &PullRequest {
                    protocol: PROTOCOL_VERSION,
                    device_id,
                    after: peer.pulled_seq,
                },
            )?;
            let page = reply.value;
            let batch =
                ChangeBatch::try_from(&page.batch).map_err(|e| SyncError::Failed(e.to_string()))?;
            received += batch.item_count();
            // The batch and the cursor together: all or nothing.
            self.deps
                .store
                .apply(&batch, Some((peer.device_id, page.last_seq)))?;
            peer.pulled_seq = page.last_seq;
            self.deps.refresh()?;
            if !page.more {
                break;
            }
        }
        // What the pull changed here (a name made distinct…) goes back at once.
        sent += self.push(&client, &token, &mut peer, device_id, interrupted)?;
        // Targeted updates: a computer forgotten meanwhile stays forgotten.
        self.deps.store.record_sync(
            peer.device_id,
            self.deps.clock.now_ms(),
            Some(&peer.addresses),
        )?;
        log::info!("Synchronisation done: {sent} record(s) sent, {received} received.");
        Ok(SyncOutcome::Done { sent, received })
    }

    fn push(
        &self,
        client: &Client,
        token: &str,
        peer: &mut SyncPeer,
        device_id: Uuid,
        interrupted: &dyn Fn() -> bool,
    ) -> Result<usize, SyncError> {
        let mut sent = 0;
        loop {
            if interrupted() {
                return Err(SyncError::Interrupted);
            }
            let page = self
                .deps
                .store
                .export(&ExportRequest::after(peer.pushed_seq, Some(peer.device_id)))?;
            if !page.batch.is_empty() {
                let reply = client.post::<_, PushResponse>(
                    "/v1/push",
                    Some(token),
                    &PushRequest {
                        protocol: PROTOCOL_VERSION,
                        device_id,
                        last_seq: page.last_seq,
                        batch: WireBatch::from(&page.batch),
                    },
                )?;
                if reply.value.acknowledged != page.last_seq {
                    return Err(SyncError::Failed("Unexpected acknowledgement.".to_owned()));
                }
                sent += page.batch.item_count();
            }
            peer.pushed_seq = page.last_seq;
            self.deps.store.record_push(peer.device_id, page.last_seq)?;
            if !page.more {
                return Ok(sent);
            }
        }
    }
}
