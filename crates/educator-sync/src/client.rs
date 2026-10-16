//! The HTTPS client of the iPad (SUB027602): it tries each known address of the computer, pins
//! its certificate, and reads the answers (refusals included) of the four routes.

use crate::http::{read_response, write_request, MAX_BODY};
use crate::tls::{client_config, PinnedVerifier, SERVER_NAME};
use crate::wire::{read_message, ErrorResponse, MessageError};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt;
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use std::time::Duration;

/// Where the computer is.
#[derive(Clone, PartialEq, Eq)]
pub struct ComputerAddress {
    pub addresses: Vec<String>,
    pub port: u16,
    /// SHA-256 of its certificate; `None`: pairing by the short code (first certificate kept).
    pub fingerprint: Option<String>,
}

impl fmt::Debug for ComputerAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ComputerAddress")
            .field("addresses", &self.addresses.len())
            .field("port", &self.port)
            .finish_non_exhaustive()
    }
}

/// Why a request did not get its answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientError {
    /// No address answered (computer off, other network, reception off, firewall).
    Unreachable,
    /// Another certificate than the one paired: another machine.
    CertificateMismatch,
    /// Refused by the computer, with the code of its [`ErrorResponse`].
    Refused {
        status: u16,
        code: String,
    },
    /// Another version of the protocol.
    Incompatible,
    Malformed(String),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::Unreachable => f.write_str("Computer unreachable."),
            ClientError::CertificateMismatch => f.write_str("Certificate of another machine."),
            ClientError::Refused { status, code } => write!(f, "Refused ({status} {code})."),
            ClientError::Incompatible => f.write_str("Another version of the protocol."),
            ClientError::Malformed(m) => write!(f, "Malformed answer: {m}"),
        }
    }
}

impl std::error::Error for ClientError {}

/// An answer, the address that gave it and the certificate seen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reply<T> {
    pub value: T,
    pub address: String,
    pub fingerprint: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub computer: ComputerAddress,
    pub connect_timeout: Duration,
    pub timeout: Duration,
}

impl Client {
    pub fn new(computer: ComputerAddress) -> Self {
        Self {
            computer,
            connect_timeout: Duration::from_secs(3),
            timeout: Duration::from_secs(30),
        }
    }

    /// `POST path` with `body`, to the first address that answers.
    pub fn post<Q: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        token: Option<&str>,
        body: &Q,
    ) -> Result<Reply<R>, ClientError> {
        let body = serde_json::to_vec(body).map_err(|e| ClientError::Malformed(e.to_string()))?;
        for address in &self.computer.addresses {
            // Only the local network (ADR 0007): nothing is ever sent elsewhere.
            let Ok(ip) = address.parse::<std::net::IpAddr>() else {
                continue;
            };
            if !crate::http::is_local_address(ip) {
                continue;
            }
            let target = SocketAddr::new(ip, self.computer.port);
            let Ok(stream) = TcpStream::connect_timeout(&target, self.connect_timeout) else {
                continue;
            };
            let verifier = Arc::new(PinnedVerifier::new(self.computer.fingerprint.clone()));
            match self.exchange(stream, Arc::clone(&verifier), path, token, &body) {
                Ok((status, response)) => {
                    let fingerprint = verifier.seen();
                    return if status == 200 {
                        read_message::<R>(&response)
                            .map(|value| Reply {
                                value,
                                address: address.clone(),
                                fingerprint,
                            })
                            .map_err(|e| match e {
                                MessageError::Incompatible(_) => ClientError::Incompatible,
                                MessageError::Malformed(m) => ClientError::Malformed(m),
                            })
                    } else {
                        let code = serde_json::from_slice::<ErrorResponse>(&response)
                            .map(|e| e.code)
                            .unwrap_or_default();
                        if status == 409 && code == "incompatible" {
                            Err(ClientError::Incompatible)
                        } else {
                            Err(ClientError::Refused { status, code })
                        }
                    };
                }
                Err(e) => {
                    if verifier.mismatch() {
                        return Err(ClientError::CertificateMismatch);
                    }
                    log::debug!("Synchronisation: exchange failed: {e}");
                }
            }
        }
        Err(ClientError::Unreachable)
    }

    fn exchange(
        &self,
        stream: TcpStream,
        verifier: Arc<PinnedVerifier>,
        path: &str,
        token: Option<&str>,
        body: &[u8],
    ) -> Result<(u16, Vec<u8>), String> {
        stream
            .set_read_timeout(Some(self.timeout))
            .and_then(|()| stream.set_write_timeout(Some(self.timeout)))
            .map_err(|e| e.to_string())?;
        let config = client_config(verifier).map_err(|e| e.to_string())?;
        let name =
            rustls::pki_types::ServerName::try_from(SERVER_NAME).map_err(|e| e.to_string())?;
        let connection = rustls::ClientConnection::new(config, name).map_err(|e| e.to_string())?;
        let mut tls = rustls::StreamOwned::new(connection, stream);
        write_request(&mut tls, path, token, body).map_err(|e| e.to_string())?;
        let response = read_response(&mut tls, MAX_BODY).map_err(|e| e.to_string())?;
        Ok((response.status, response.body))
    }
}
