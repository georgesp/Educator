//! TLS 1.3 between the computer and the iPad (SUB027610, ADR 0007 section 7): the computer has a
//! self-signed certificate made at the first start of its server; the iPad pins the SHA-256 of
//! that certificate (given by the QR code). No certificate authority.

use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::crypto::{verify_tls12_signature, verify_tls13_signature, CryptoProvider};
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, ServerName, UnixTime};
use rustls::{CertificateError, DigitallySignedStruct, SignatureScheme};
use sha2::{Digest, Sha256};
use std::fmt;
use std::sync::{Arc, Mutex};

/// Name written in the certificate and asked by the iPad (the certificate is pinned, the name is
/// not what is checked).
pub const SERVER_NAME: &str = "educator.local";

/// The certificate of the computer and its private key, as PEM (kept in `SyncState`).
#[derive(Clone, PartialEq, Eq)]
pub struct TlsIdentity {
    pub certificate_pem: String,
    pub key_pem: String,
}

impl fmt::Debug for TlsIdentity {
    /// Without the key.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TlsIdentity").finish_non_exhaustive()
    }
}

/// A TLS problem (certificate unreadable, configuration refused).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlsError(pub String);

impl fmt::Display for TlsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TLS: {}", self.0)
    }
}

impl std::error::Error for TlsError {}

fn tls_error(e: impl fmt::Display) -> TlsError {
    TlsError(e.to_string())
}

pub(crate) fn provider() -> Arc<CryptoProvider> {
    Arc::new(rustls::crypto::ring::default_provider())
}

impl TlsIdentity {
    /// A new self-signed certificate (ECDSA P-256).
    pub fn generate() -> Result<Self, TlsError> {
        let certified =
            rcgen::generate_simple_self_signed(vec![SERVER_NAME.to_owned()]).map_err(tls_error)?;
        Ok(Self {
            certificate_pem: certified.cert.pem(),
            key_pem: certified.signing_key.serialize_pem(),
        })
    }

    fn certificate(&self) -> Result<CertificateDer<'static>, TlsError> {
        CertificateDer::from_pem_slice(self.certificate_pem.as_bytes()).map_err(tls_error)
    }

    /// SHA-256 of the certificate, in upper-case hexadecimal: what the QR code carries.
    pub fn fingerprint(&self) -> Result<String, TlsError> {
        Ok(fingerprint(self.certificate()?.as_ref()))
    }

    /// The configuration of the server: TLS 1.3 only, no client certificate.
    pub fn server_config(&self) -> Result<Arc<rustls::ServerConfig>, TlsError> {
        let key = PrivateKeyDer::from_pem_slice(self.key_pem.as_bytes()).map_err(tls_error)?;
        let config = rustls::ServerConfig::builder_with_provider(provider())
            .with_protocol_versions(&[&rustls::version::TLS13])
            .map_err(tls_error)?
            .with_no_client_auth()
            .with_single_cert(vec![self.certificate()?], key)
            .map_err(tls_error)?;
        Ok(Arc::new(config))
    }
}

/// SHA-256 of a certificate (DER), upper-case hexadecimal.
pub fn fingerprint(der: &[u8]) -> String {
    hex(&Sha256::digest(der))
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect()
}

/// Compares two secrets in constant time (tokens, fingerprints).
pub(crate) fn same_secret(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

/// The iPad side: accepts only the certificate whose fingerprint was given at pairing; with
/// `None` (pairing by the short code), accepts the first one and remembers it.
pub(crate) struct PinnedVerifier {
    expected: Option<String>,
    seen: Mutex<Option<String>>,
    mismatch: Mutex<bool>,
    provider: Arc<CryptoProvider>,
}

impl fmt::Debug for PinnedVerifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PinnedVerifier").finish_non_exhaustive()
    }
}

impl PinnedVerifier {
    pub(crate) fn new(expected: Option<String>) -> Self {
        Self {
            expected,
            seen: Mutex::new(None),
            mismatch: Mutex::new(false),
            provider: provider(),
        }
    }

    pub(crate) fn seen(&self) -> Option<String> {
        self.seen.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    pub(crate) fn mismatch(&self) -> bool {
        *self.mismatch.lock().unwrap_or_else(|e| e.into_inner())
    }
}

impl ServerCertVerifier for PinnedVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        let found = fingerprint(end_entity.as_ref());
        *self.seen.lock().unwrap_or_else(|e| e.into_inner()) = Some(found.clone());
        match &self.expected {
            Some(expected) if !same_secret(&expected.to_ascii_uppercase(), &found) => {
                *self.mismatch.lock().unwrap_or_else(|e| e.into_inner()) = true;
                Err(rustls::Error::InvalidCertificate(
                    CertificateError::ApplicationVerificationFailure,
                ))
            }
            _ => Ok(ServerCertVerified::assertion()),
        }
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls12_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls13_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.provider
            .signature_verification_algorithms
            .supported_schemes()
    }
}

pub(crate) fn client_config(
    verifier: Arc<PinnedVerifier>,
) -> Result<Arc<rustls::ClientConfig>, TlsError> {
    let config = rustls::ClientConfig::builder_with_provider(provider())
        .with_protocol_versions(&[&rustls::version::TLS13])
        .map_err(tls_error)?
        .dangerous()
        .with_custom_certificate_verifier(verifier)
        .with_no_client_auth();
    Ok(Arc::new(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_generated_identity_has_a_stable_fingerprint_and_a_working_configuration() {
        let identity = TlsIdentity::generate().unwrap();
        let fingerprint = identity.fingerprint().unwrap();
        assert_eq!(64, fingerprint.len());
        assert_eq!(fingerprint, identity.fingerprint().unwrap());
        assert!(identity.server_config().is_ok());
        assert_ne!(
            fingerprint,
            TlsIdentity::generate().unwrap().fingerprint().unwrap()
        );
        assert!(!format!("{identity:?}").contains("PRIVATE"));
    }

    #[test]
    fn secrets_are_compared_whole() {
        assert!(same_secret("ABC", "ABC"));
        assert!(!same_secret("ABC", "ABD"));
        assert!(!same_secret("ABC", "ABCD"));
    }
}
