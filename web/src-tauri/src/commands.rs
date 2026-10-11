//! Tauri commands called by the front (`web/src/api/ui.ts`), one per operation of the former local
//! HTTP API. Each runs on a worker thread (SQLite and Piper are blocking) and returns a
//! [`CommandError`] shaped like the former problem details.

use crate::contracts::*;
use crate::services::Services;
use crate::state::AppState;
use std::sync::Arc;
use tauri::ipc::Response;
use tauri::State;

type CommandResult<T> = Result<T, CommandError>;

async fn run<T: Send + 'static>(
    state: State<'_, Arc<AppState>>,
    operation: impl FnOnce(&Services) -> CommandResult<T> + Send + 'static,
) -> CommandResult<T> {
    let services = state.services()?;
    tauri::async_runtime::spawn_blocking(move || operation(&services))
        .await
        .map_err(|e| CommandError::new(500, "Unexpected error", e.to_string()))?
}

/// Start-up state, polled by the front until ready or failed.
#[tauri::command]
pub fn get_startup(state: State<'_, Arc<AppState>>) -> StartupStatus {
    state.status()
}

#[tauri::command]
pub async fn get_status(state: State<'_, Arc<AppState>>) -> CommandResult<StatusResponse> {
    run(state, |s| Ok(s.status())).await
}

#[tauri::command]
pub async fn get_skills(
    state: State<'_, Arc<AppState>>,
    grade: Grade,
) -> CommandResult<SkillsResponse> {
    run(state, move |s| Ok(s.skills(grade))).await
}

#[tauri::command]
pub async fn get_characters(
    state: State<'_, Arc<AppState>>,
) -> CommandResult<Vec<CharacterResponse>> {
    run(state, |s| Ok(s.characters())).await
}

#[tauri::command]
pub async fn draw_character(
    state: State<'_, Arc<AppState>>,
    except: Option<String>,
) -> CommandResult<CharacterResponse> {
    run(state, move |s| s.draw_character(except.as_deref())).await
}

#[tauri::command]
pub async fn get_character_svg(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CommandResult<String> {
    run(state, move |s| s.character_svg(&id)).await
}

#[tauri::command]
pub async fn list_profiles(state: State<'_, Arc<AppState>>) -> CommandResult<Vec<ProfileResponse>> {
    run(state, |s| s.list_profiles()).await
}

#[tauri::command]
pub async fn create_profile(
    state: State<'_, Arc<AppState>>,
    request: CreateProfileRequest,
) -> CommandResult<ProfileResponse> {
    run(state, move |s| s.create_profile(request)).await
}

#[tauri::command]
pub async fn update_profile(
    state: State<'_, Arc<AppState>>,
    id: String,
    changes: UpdateProfileRequest,
) -> CommandResult<ProfileResponse> {
    run(state, move |s| s.update_profile(&id, changes)).await
}

#[tauri::command]
pub async fn select_profile(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CommandResult<ProfileResponse> {
    run(state, move |s| s.select_profile(&id)).await
}

#[tauri::command]
pub async fn reroll_character(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CommandResult<ProfileResponse> {
    run(state, move |s| s.reroll_character(&id)).await
}

#[tauri::command]
pub async fn delete_profile(state: State<'_, Arc<AppState>>, id: String) -> CommandResult<()> {
    run(state, move |s| s.delete_profile(&id)).await
}

#[tauri::command]
pub async fn start_series(
    state: State<'_, Arc<AppState>>,
    request: StartSeriesRequest,
) -> CommandResult<SeriesResponse> {
    run(state, move |s| s.start_series(request)).await
}

#[tauri::command]
pub async fn answer_series(
    state: State<'_, Arc<AppState>>,
    series_id: String,
    index: u32,
    answer: i32,
) -> CommandResult<SeriesAnswerResponse> {
    run(state, move |s| s.answer_series(&series_id, index, answer)).await
}

#[tauri::command]
pub async fn abandon_series(
    state: State<'_, Arc<AppState>>,
    series_id: String,
) -> CommandResult<SeriesSummaryResponse> {
    run(state, move |s| s.abandon_series(&series_id)).await
}

/// WAV bytes (`audio/wav`), received as an `ArrayBuffer` by the front.
#[tauri::command]
pub async fn speak(
    state: State<'_, Arc<AppState>>,
    text: String,
    speed: Option<f64>,
) -> CommandResult<Response> {
    run(state, move |s| s.speak(&text, speed).map(Response::new)).await
}
