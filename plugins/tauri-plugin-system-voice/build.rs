//! Builds and links the Swift package (`ios/`) when compiling for iOS. On other targets the crate
//! has no native side, and nothing is generated.

/// No command is called from the front: the plugin is used from Rust only (`SystemVoice`).
const COMMANDS: &[&str] = &[];

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("ios") {
        tauri_plugin::Builder::new(COMMANDS).ios_path("ios").build();
    }
}
