//! Pairing of the iPad (SUB027601, ADR 0007 section 8): the computer offers a secret for
//! [`PAIRING_SECONDS`], shown as a QR code ([`PairingLink`]) and as a short code (plan B without
//! camera). The secret serves once; a QR code photographed and replayed later is refused. The
//! iPad gets a lasting token of its own.

use crate::tls::{hex, same_secret};
use rand::RngCore;
use std::fmt;

/// How long an offer lasts.
pub const PAIRING_SECONDS: i64 = 5 * 60;

/// Wrong short codes before the offer is withdrawn.
pub const MAX_CODE_ATTEMPTS: u32 = 5;

/// Scheme of the QR code.
pub const LINK_SCHEME: &str = "educator://pair";

/// Crockford base 32 without the letters that look like digits (I, L, O, U).
const CODE_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// Random bytes as upper-case hexadecimal (secrets, tokens).
pub fn random_hex(bytes: usize) -> String {
    let mut buffer = vec![0u8; bytes];
    rand::rng().fill_bytes(&mut buffer);
    hex(&buffer)
}

/// A new token for a paired iPad (256 bits).
pub fn new_token() -> String {
    random_hex(32)
}

/// SHA-256 of a token: what the computer keeps.
pub fn token_hash(token: &str) -> String {
    crate::tls::fingerprint(token.as_bytes())
}

fn new_code() -> String {
    let mut buffer = [0u8; 8];
    rand::rng().fill_bytes(&mut buffer);
    buffer
        .iter()
        .map(|b| CODE_ALPHABET[usize::from(*b) % CODE_ALPHABET.len()] as char)
        .collect()
}

/// The short code as typed: upper case, without spaces nor dashes, `O` read as `0`, `I`/`L` as `1`.
pub fn normalize_code(code: &str) -> String {
    code.chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .map(|c| match c.to_ascii_uppercase() {
            'O' => '0',
            'I' | 'L' => '1',
            c => c,
        })
        .collect()
}

/// What the computer shows to pair an iPad.
#[derive(Clone, PartialEq, Eq)]
pub struct PairingOffer {
    /// 128 bits, in the QR code.
    pub secret: String,
    /// 8 characters, shown under the QR code as `ABCD-EFGH`.
    pub code: String,
    pub expires_at: i64,
}

impl fmt::Debug for PairingOffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PairingOffer")
            .field("expires_at", &self.expires_at)
            .finish_non_exhaustive()
    }
}

impl PairingOffer {
    /// `ABCD-EFGH`.
    pub fn display_code(&self) -> String {
        format!("{}-{}", &self.code[..4], &self.code[4..])
    }
}

/// Why a pairing is refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairingError {
    /// No offer, or it expired, or it was already used.
    Expired,
    Wrong,
    /// Too many wrong codes: the offer is withdrawn.
    TooMany,
}

#[derive(Debug, Default)]
struct Slot {
    offer: Option<PairingOffer>,
    attempts: u32,
}

/// The offer of the computer (in memory only: never written anywhere).
#[derive(Debug, Default)]
pub struct Pairing {
    slot: Slot,
}

impl Pairing {
    /// A new offer, replacing the previous one.
    pub fn open(&mut self, now: i64) -> PairingOffer {
        let offer = PairingOffer {
            secret: random_hex(16),
            code: new_code(),
            expires_at: now + PAIRING_SECONDS * 1000,
        };
        self.slot = Slot {
            offer: Some(offer.clone()),
            attempts: 0,
        };
        offer
    }

    pub fn close(&mut self) {
        self.slot = Slot::default();
    }

    /// The current offer, while it lasts.
    pub fn current(&mut self, now: i64) -> Option<PairingOffer> {
        match &self.slot.offer {
            Some(offer) if offer.expires_at > now => Some(offer.clone()),
            Some(_) => {
                self.close();
                None
            }
            None => None,
        }
    }

    /// Uses the offer with the secret of the QR code or the short code: once only.
    pub fn consume(&mut self, presented: &str, now: i64) -> Result<(), PairingError> {
        let Some(offer) = self.current(now) else {
            return Err(PairingError::Expired);
        };
        let secret_matches = same_secret(&presented.to_ascii_uppercase(), &offer.secret);
        let code_matches = same_secret(&normalize_code(presented), &offer.code);
        if secret_matches || code_matches {
            self.close();
            return Ok(());
        }
        self.slot.attempts += 1;
        if self.slot.attempts >= MAX_CODE_ATTEMPTS {
            self.close();
            return Err(PairingError::TooMany);
        }
        Err(PairingError::Wrong)
    }
}

/// What the QR code carries: where the computer is, its certificate and the secret.
#[derive(Clone, PartialEq, Eq)]
pub struct PairingLink {
    pub addresses: Vec<String>,
    pub port: u16,
    pub fingerprint: String,
    pub secret: String,
}

impl fmt::Debug for PairingLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PairingLink")
            .field("port", &self.port)
            .finish_non_exhaustive()
    }
}

impl PairingLink {
    /// `educator://pair?v=1&h=192.168.1.10,10.0.0.2&p=47615&f=<sha-256>&s=<secret>`.
    pub fn to_uri(&self) -> String {
        format!(
            "{LINK_SCHEME}?v={}&h={}&p={}&f={}&s={}",
            crate::wire::PROTOCOL_VERSION,
            self.addresses.join(","),
            self.port,
            self.fingerprint,
            self.secret
        )
    }

    /// The link of a QR code; `None` when it is not one of ours or of another version.
    pub fn parse(uri: &str) -> Option<Self> {
        let query = uri.trim().strip_prefix(LINK_SCHEME)?.strip_prefix('?')?;
        let mut link = PairingLink {
            addresses: Vec::new(),
            port: 0,
            fingerprint: String::new(),
            secret: String::new(),
        };
        let mut version = None;
        for pair in query.split('&') {
            let (key, value) = pair.split_once('=')?;
            match key {
                "v" => version = value.parse::<u32>().ok(),
                "h" => {
                    link.addresses = value
                        .split(',')
                        .filter(|a| !a.is_empty())
                        .map(str::to_owned)
                        .collect()
                }
                "p" => link.port = value.parse().ok()?,
                "f" => link.fingerprint = value.to_ascii_uppercase(),
                "s" => link.secret = value.to_ascii_uppercase(),
                _ => {}
            }
        }
        let hex = |text: &str, len: usize| {
            text.len() == len && text.bytes().all(|b| b.is_ascii_hexdigit())
        };
        let valid = version == Some(crate::wire::PROTOCOL_VERSION)
            && !link.addresses.is_empty()
            // Only the local network: a forged QR code cannot send the data elsewhere.
            && link.addresses.iter().all(|a| {
                a.parse::<std::net::IpAddr>()
                    .is_ok_and(crate::http::is_local_address)
            })
            && link.port != 0
            && hex(&link.fingerprint, 64)
            && hex(&link.secret, 32);
        valid.then_some(link)
    }
}

/// The QR code of a text, as an SVG (black on white, with its quiet zone).
pub fn qr_svg(text: &str) -> Option<String> {
    let code = qrcode::QrCode::with_error_correction_level(text, qrcode::EcLevel::M).ok()?;
    Some(
        code.render::<qrcode::render::svg::Color<'_>>()
            .min_dimensions(240, 240)
            .dark_color(qrcode::render::svg::Color("#000000"))
            .light_color(qrcode::render::svg::Color("#ffffff"))
            .build(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_offer_serves_once() {
        let mut pairing = Pairing::default();
        let offer = pairing.open(1_000);

        assert_eq!(Ok(()), pairing.consume(&offer.secret, 2_000));
        assert_eq!(
            Err(PairingError::Expired),
            pairing.consume(&offer.secret, 2_001)
        );
    }

    #[test]
    fn an_offer_expires() {
        let mut pairing = Pairing::default();
        let offer = pairing.open(0);

        assert_eq!(
            Err(PairingError::Expired),
            pairing.consume(&offer.secret, PAIRING_SECONDS * 1000)
        );
    }

    #[test]
    fn the_short_code_works_typed_loosely_and_wrong_codes_withdraw_the_offer() {
        let mut pairing = Pairing::default();
        let offer = pairing.open(0);
        let typed = offer.display_code().to_lowercase().replace('0', "o");
        assert_eq!(Ok(()), pairing.consume(&typed, 1));

        let offer = pairing.open(0);
        for _ in 1..MAX_CODE_ATTEMPTS {
            assert_eq!(Err(PairingError::Wrong), pairing.consume("ZZZZ-ZZZZ-Z", 1));
        }
        assert_eq!(Err(PairingError::TooMany), pairing.consume("nope", 1));
        assert_eq!(
            Err(PairingError::Expired),
            pairing.consume(&offer.secret, 1)
        );
    }

    #[test]
    fn a_link_round_trips_and_a_foreign_one_is_refused() {
        let link = PairingLink {
            addresses: vec!["192.168.1.10".to_owned(), "10.0.0.2".to_owned()],
            port: 47615,
            fingerprint: "AB".repeat(32),
            secret: "CD".repeat(16),
        };
        assert_eq!(Some(link.clone()), PairingLink::parse(&link.to_uri()));
        assert!(!format!("{link:?}").contains("CDCD"));
        for uri in [
            "https://example.com",
            "educator://pair?v=2&h=1.2.3.4&p=1&f=AB&s=CD",
            &link.to_uri().replace("&p=47615", ""),
            &link.to_uri().replace("192.168.1.10", "evil.example.com"),
            &link.to_uri().replace("192.168.1.10", "203.0.113.7"),
        ] {
            assert_eq!(None, PairingLink::parse(uri), "{uri}");
        }
        assert!(qr_svg(&link.to_uri()).unwrap().starts_with("<?xml"));
    }
}
