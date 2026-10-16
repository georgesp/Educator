//! Contracts of the synchronisation between the computer and the iPad (lot tablette 2, US0276,
//! SUB027603): the state shown by the screen « Synchronisation » of the parents' space, the pairing
//! code, the result of an exchange and the question « Est-ce le même enfant ? ».

use super::iso_8601;
use educator_core::sync::{NameConflict, SyncPeer};
use educator_sync::service::{SyncError, SyncOutcome};
use serde::{Deserialize, Serialize};

/// What this device is in the synchronisation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum SyncRole {
    /// The computer: it receives (server).
    Computer,
    /// The iPad: it starts the exchanges (client).
    Tablet,
}

/// A paired device: the tablets on the computer, the computer on the iPad.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SyncDeviceResponse {
    pub id: String,
    /// Neutral name (« Tablette 1 », « Ordinateur »), never a child's name.
    pub name: String,
    pub paired_at: String,
    pub last_sync_at: Option<String>,
}

impl From<&SyncPeer> for SyncDeviceResponse {
    fn from(peer: &SyncPeer) -> Self {
        Self {
            id: peer.device_id.to_string(),
            name: peer.name.clone(),
            paired_at: iso_8601(peer.paired_at),
            last_sync_at: peer.last_sync_at.map(iso_8601),
        }
    }
}

/// « Est-ce le même enfant ? » for a profile of this iPad.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SyncQuestionResponse {
    pub local_profile_id: String,
    pub local_name: String,
    pub remote_profile_id: String,
    pub remote_name: String,
    /// The character of the profile of the computer, to recognise the child.
    pub remote_character_id: String,
}

impl From<&NameConflict> for SyncQuestionResponse {
    fn from(conflict: &NameConflict) -> Self {
        Self {
            local_profile_id: conflict.local_id.to_string(),
            local_name: conflict.local_name.clone(),
            remote_profile_id: conflict.remote.id.to_string(),
            remote_name: conflict.remote.name.clone(),
            remote_character_id: conflict.remote.character_id.clone(),
        }
    }
}

/// The state of the screen « Synchronisation ».
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SyncStatusResponse {
    pub role: SyncRole,
    /// Computer: the server listens now.
    pub receiving: bool,
    /// Computer: the option « Recevoir les synchronisations » (even with the screen closed).
    pub receive_always: bool,
    /// Computer: where the tablet finds it (`192.168.1.10:47615`), while it listens.
    pub address: Option<String>,
    /// Why the computer cannot receive (port, firewall), when it cannot.
    pub problem: Option<String>,
    /// The tablets of the computer, or the computer of the iPad.
    pub devices: Vec<SyncDeviceResponse>,
    /// iPad: the questions waiting for the parent.
    pub questions: Vec<SyncQuestionResponse>,
}

/// What the computer shows to pair an iPad.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SyncPairingResponse {
    /// The QR code, as an SVG drawn by the core.
    pub qr_svg: String,
    /// Plan B without camera: `ABCD-EFGH`, with the address.
    pub code: String,
    pub address: String,
    pub expires_at: String,
}

/// How the iPad pairs: the text of the QR code, or the address and the short code.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct PairWithComputerRequest {
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub link: Option<String>,
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub address: Option<String>,
    #[serde(default)]
    #[cfg_attr(test, ts(optional))]
    pub code: Option<String>,
}

/// The result of an exchange, or why there was none (the screen says it simply to the parent).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum SyncOutcomeName {
    Done,
    NeedsDecision,
    NotPaired,
    Busy,
    Unreachable,
    CertificateMismatch,
    Unauthorized,
    Incompatible,
    PairingExpired,
    TooManyAttempts,
    InvalidLink,
    Interrupted,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct SyncResultResponse {
    pub outcome: SyncOutcomeName,
    /// Records sent and received.
    pub sent: u32,
    pub received: u32,
    pub status: SyncStatusResponse,
}

impl SyncOutcomeName {
    pub fn of(result: &Result<SyncOutcome, SyncError>) -> (Self, u32, u32) {
        let count = |n: usize| u32::try_from(n).unwrap_or(u32::MAX);
        match result {
            Ok(SyncOutcome::Done { sent, received }) => {
                (SyncOutcomeName::Done, count(*sent), count(*received))
            }
            Ok(SyncOutcome::NeedsDecision(_)) => (SyncOutcomeName::NeedsDecision, 0, 0),
            Err(error) => (Self::of_error(error), 0, 0),
        }
    }

    pub fn of_error(error: &SyncError) -> Self {
        match error {
            SyncError::NotPaired => SyncOutcomeName::NotPaired,
            SyncError::Busy => SyncOutcomeName::Busy,
            SyncError::Unreachable => SyncOutcomeName::Unreachable,
            SyncError::CertificateMismatch => SyncOutcomeName::CertificateMismatch,
            SyncError::Unauthorized => SyncOutcomeName::Unauthorized,
            SyncError::Incompatible => SyncOutcomeName::Incompatible,
            SyncError::PairingExpired => SyncOutcomeName::PairingExpired,
            SyncError::TooManyAttempts => SyncOutcomeName::TooManyAttempts,
            SyncError::InvalidLink => SyncOutcomeName::InvalidLink,
            SyncError::Interrupted => SyncOutcomeName::Interrupted,
            SyncError::Failed(_) => SyncOutcomeName::Failed,
        }
    }
}
