//! Dependency rule (ADR 0001/0003): educator-core depends on no infrastructure crate. Only the
//! crates listed here may appear among its normal (non-dev) dependencies.

use std::process::Command;

const ALLOWED: [&str; 4] = ["rand", "rand_chacha", "uuid", "thiserror"];

#[test]
fn core_depends_on_no_infrastructure_crate() {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let output = Command::new(cargo)
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--manifest-path",
        ])
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .output()
        .expect("cargo metadata runs");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let packages = metadata["packages"].as_array().unwrap();
    let core = packages
        .iter()
        .find(|p| p["name"] == "educator-core")
        .expect("educator-core in the workspace");
    let normal: Vec<&str> = core["dependencies"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["kind"].is_null())
        .map(|d| d["name"].as_str().unwrap())
        .collect();

    for dependency in &normal {
        assert!(
            ALLOWED.contains(dependency),
            "educator-core must not depend on {dependency} (allowed: {ALLOWED:?})"
        );
    }

    for package in packages.iter().filter(|p| {
        p["name"]
            .as_str()
            .is_some_and(|n| n.starts_with("educator-"))
    }) {
        let name = package["name"].as_str().unwrap();
        let uses_core = package["dependencies"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["name"] == "educator-core");
        if name == "educator-core" {
            continue;
        }

        // Data and content depend on core, never the other way round, and never on each other.
        if name == "educator-data" || name == "educator-content" {
            assert!(uses_core, "{name} builds on educator-core");
            let other = if name == "educator-data" {
                "educator-content"
            } else {
                "educator-data"
            };
            assert!(
                !package["dependencies"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|d| d["name"] == other),
                "{name} must not depend on {other}"
            );
        }
    }
}
