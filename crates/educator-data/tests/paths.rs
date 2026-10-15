//! Location of the data files on every platform, computed without Tauri (SUB023803, ADR 0006).

use educator_data::{data_directory_for, Platform, APP_FOLDER_NAME, DATA_DIR_VARIABLE};
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

const ALL_PLATFORMS: [Platform; 4] = [
    Platform::MacOs,
    Platform::Ios,
    Platform::Windows,
    Platform::Other,
];

/// Environment made of the given variables only.
fn environment(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<OsString> {
    let variables: HashMap<String, OsString> = pairs
        .iter()
        .map(|(name, value)| ((*name).to_owned(), OsString::from(value)))
        .collect();
    move |name| variables.get(name).cloned()
}

fn application_support(home: &str) -> PathBuf {
    Path::new(home)
        .join("Library")
        .join("Application Support")
        .join(APP_FOLDER_NAME)
}

#[test]
fn ipad_data_live_in_the_library_of_the_app_container() {
    // iOS gives each app its own container as HOME; only Documents, Library and tmp are writable.
    let container = "/private/var/mobile/Containers/Data/Application/1F2E3D4C";
    let directory = data_directory_for(Platform::Ios, environment(&[("HOME", container)]));

    assert_eq!(directory, application_support(container));
    assert!(!directory.ends_with(Path::new(".config").join(APP_FOLDER_NAME)));
}

#[test]
fn mac_data_live_in_application_support() {
    let directory = data_directory_for(Platform::MacOs, environment(&[("HOME", "/Users/georges")]));

    assert_eq!(directory, application_support("/Users/georges"));
}

#[test]
fn windows_data_live_in_appdata() {
    let appdata = r"C:\Users\georges\AppData\Roaming";
    let with_appdata = data_directory_for(
        Platform::Windows,
        environment(&[("USERPROFILE", r"C:\Users\georges"), ("APPDATA", appdata)]),
    );
    let without_appdata = data_directory_for(
        Platform::Windows,
        environment(&[("USERPROFILE", r"C:\Users\georges")]),
    );

    assert_eq!(with_appdata, Path::new(appdata).join(APP_FOLDER_NAME));
    assert_eq!(
        without_appdata,
        Path::new(r"C:\Users\georges")
            .join("AppData")
            .join("Roaming")
            .join(APP_FOLDER_NAME)
    );
}

#[test]
fn other_systems_use_the_config_folder() {
    let with_xdg = data_directory_for(
        Platform::Other,
        environment(&[("HOME", "/home/ci"), ("XDG_CONFIG_HOME", "/home/ci/cfg")]),
    );
    let without_xdg = data_directory_for(Platform::Other, environment(&[("HOME", "/home/ci")]));

    assert_eq!(with_xdg, Path::new("/home/ci/cfg").join(APP_FOLDER_NAME));
    assert_eq!(
        without_xdg,
        Path::new("/home/ci").join(".config").join(APP_FOLDER_NAME)
    );
}

#[test]
fn the_override_variable_wins_on_every_platform() {
    let overridden = std::path::absolute("/tmp/educator-tests").unwrap();
    for platform in ALL_PLATFORMS {
        let directory = data_directory_for(
            platform,
            environment(&[
                ("HOME", "/Users/georges"),
                (DATA_DIR_VARIABLE, "/tmp/educator-tests"),
            ]),
        );

        assert_eq!(directory, overridden, "{platform:?}");
    }
}

#[test]
fn a_relative_override_becomes_absolute() {
    let directory =
        data_directory_for(Platform::MacOs, environment(&[(DATA_DIR_VARIABLE, "data")]));

    assert!(directory.is_absolute());
    assert!(directory.ends_with("data"));
}

#[test]
fn an_empty_variable_counts_as_missing() {
    let directory = data_directory_for(
        Platform::Ios,
        environment(&[("HOME", "/container"), (DATA_DIR_VARIABLE, "")]),
    );

    assert_eq!(directory, application_support("/container"));
}
