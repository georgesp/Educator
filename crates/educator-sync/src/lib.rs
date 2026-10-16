//! Synchronisation of Educator between the computer and the iPad on the Wi-Fi of the house
//! (lot tablette 2 — US0276, ADR 0007). The computer is the server, the iPad the client; nothing
//! leaves the local network, everything is encrypted (TLS 1.3, certificate pinned at pairing).
//!
//! - [`wire`]: the exchange format, versioned (schemas in `schemas/`);
//! - [`tls`], [`http`]: the self-signed certificate, the pinning, a minimal HTTP/1.1;
//! - [`pairing`]: the QR code, the short code, the tokens;
//! - [`server`] (desktop): the listener; [`client`]: the requests of the iPad;
//! - [`service`]: the routes of the computer and the exchange driven by the iPad;
//! - [`discovery`] (desktop): the Bonjour announce.
//!
//! The rules of the merge are in `educator-core::sync`, the storage in `educator-data`.

pub mod client;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
pub mod discovery;
pub mod http;
pub mod pairing;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
pub mod server;
pub mod service;
pub mod tls;
pub mod wire;

/// Key of the certificate of the computer in `SyncState` (PEM).
pub const CERTIFICATE_KEY: &str = "Certificate";

/// Key of its private key in `SyncState` (PEM).
pub const PRIVATE_KEY_KEY: &str = "PrivateKey";

/// Key of the last port of the server in `SyncState`.
pub const PORT_KEY: &str = "Port";

/// Key of the option « Recevoir les synchronisations » in `SyncState` (`"on"` / `"off"`).
pub const RECEIVE_KEY: &str = "Receive";

/// The certificate of this computer: the one kept, or a new one kept for next time.
pub fn server_identity(
    store: &dyn educator_core::sync::SyncStore,
) -> Result<tls::TlsIdentity, String> {
    let certificate = store.value(CERTIFICATE_KEY).map_err(|e| e.to_string())?;
    let key = store.value(PRIVATE_KEY_KEY).map_err(|e| e.to_string())?;
    if let (Some(certificate_pem), Some(key_pem)) = (certificate, key) {
        return Ok(tls::TlsIdentity {
            certificate_pem,
            key_pem,
        });
    }
    let identity = tls::TlsIdentity::generate().map_err(|e| e.to_string())?;
    store
        .set_value(CERTIFICATE_KEY, &identity.certificate_pem)
        .map_err(|e| e.to_string())?;
    store
        .set_value(PRIVATE_KEY_KEY, &identity.key_pem)
        .map_err(|e| e.to_string())?;
    log::info!("Synchronisation: certificate of this computer created.");
    Ok(identity)
}
