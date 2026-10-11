//! Educator desktop application (Tauri 2, ADR 0003). Everything runs in this process: the commands
//! (`commands.rs`) call the services assembled at start-up (`state.rs`, `services.rs`) from the
//! `educator-core`, `educator-content` and `educator-data` crates. No server, no port, no child
//! process except the Piper voice.

mod commands;
mod contracts;
mod services;
mod state;

#[cfg(test)]
mod bindings;

use state::{AppState, Folders};
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let data_directory = educator_data::app_data_directory();
    let logs = tauri_plugin_log::Builder::new()
        .level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .level_for("tao", log::LevelFilter::Info)
        .level_for("wry", log::LevelFilter::Info)
        .targets([
            tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
            tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Folder {
                path: data_directory.join(educator_data::LOGS_FOLDER_NAME),
                file_name: Some("educator".to_owned()),
            }),
        ])
        .build();

    let builder = tauri::Builder::default();

    // Must be registered first: a second launch focuses the existing window instead of opening
    // the same database twice.
    #[cfg(desktop)]
    let builder = builder
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_window_state::Builder::default().build());

    let state = Arc::new(AppState::default());
    builder
        .plugin(logs)
        .manage(Arc::clone(&state))
        .setup(move |app| {
            let executable = std::env::current_exe()
                .ok()
                .and_then(|exe| exe.parent().map(std::path::Path::to_path_buf))
                .unwrap_or_default();
            state::start(
                state,
                Folders {
                    resources: app.path().resource_dir().ok(),
                    executable,
                    data: data_directory,
                },
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_startup,
            commands::get_status,
            commands::get_skills,
            commands::get_characters,
            commands::draw_character,
            commands::get_character_svg,
            commands::list_profiles,
            commands::create_profile,
            commands::update_profile,
            commands::select_profile,
            commands::reroll_character,
            commands::delete_profile,
            commands::start_series,
            commands::answer_series,
            commands::abandon_series,
            commands::speak,
        ])
        .run(tauri::generate_context!())
        .expect("error while running the application");
}
