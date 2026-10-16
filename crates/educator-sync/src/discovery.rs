//! The Bonjour (mDNS) announce of the computer while its server listens (SUB027610): a neutral
//! name (« Educator-XXXX »), no child's name. Desktop only. The iPad finds the computer by the
//! addresses of the QR code and the last address known: browsing Bonjour on iPadOS needs the
//! native framework (follow-up, ADR 0007).

use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::fmt;
use uuid::Uuid;

/// The service type announced.
pub const SERVICE_TYPE: &str = "_educator-sync._tcp.local.";

/// The announce, withdrawn when dropped.
pub struct Announce {
    daemon: ServiceDaemon,
    fullname: String,
}

impl fmt::Debug for Announce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Announce").finish_non_exhaustive()
    }
}

/// Neutral instance name: « Educator-XXXX », 4 characters of the device identifier.
pub fn instance_name(device_id: Uuid) -> String {
    let id = device_id.simple().to_string().to_uppercase();
    format!("Educator-{}", &id[id.len() - 4..])
}

impl Announce {
    pub fn start(device_id: Uuid, port: u16, addresses: &[String]) -> Result<Self, String> {
        let daemon = ServiceDaemon::new().map_err(|e| e.to_string())?;
        let instance = instance_name(device_id);
        let host = format!("{}.local.", instance.to_lowercase());
        let properties = [("v", crate::wire::PROTOCOL_VERSION.to_string())];
        let info = ServiceInfo::new(
            SERVICE_TYPE,
            &instance,
            &host,
            addresses.join(","),
            port,
            &properties[..],
        )
        .map_err(|e| e.to_string())?;
        let fullname = info.get_fullname().to_owned();
        daemon.register(info).map_err(|e| e.to_string())?;
        Ok(Self { daemon, fullname })
    }
}

impl Drop for Announce {
    fn drop(&mut self) {
        let _ = self.daemon.unregister(&self.fullname);
        let _ = self.daemon.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_name_is_neutral() {
        let id = Uuid::parse_str("0199d1f2-0000-7000-8000-00000000ab12").unwrap();
        assert_eq!("Educator-AB12", instance_name(id));
    }
}
