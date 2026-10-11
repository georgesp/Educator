//! Tauri shell of Educator (ADR 0001): no business logic here.
//! It starts the .NET backend (`educator-host`), hands its URL and token to the
//! front through the `get_backend` command, and stops it on exit.

mod backend;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Must be registered first: a second launch focuses the existing window
        // instead of starting a second backend on the same database.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .manage(backend::BackendState::default())
        .setup(|app| {
            backend::start(app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![backend::get_backend])
        .build(tauri::generate_context!())
        .expect("error while building the application")
        .run(|app, event| match event {
            // Last window closed or app quit: stop the host before the process exits.
            tauri::RunEvent::ExitRequested { .. } | tauri::RunEvent::Exit => backend::stop(app),
            _ => {}
        });
}
