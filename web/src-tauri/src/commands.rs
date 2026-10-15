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

/// The skills of a subject (mathematics when absent) offered to a grade; with the mastery of the
/// profile when `profile_id` is given.
#[tauri::command]
pub async fn get_skills(
    state: State<'_, Arc<AppState>>,
    grade: Grade,
    subject: Option<Subject>,
    profile_id: Option<String>,
) -> CommandResult<SkillsResponse> {
    run(state, move |s| {
        s.skills(
            grade,
            subject.unwrap_or(Subject::Math),
            profile_id.as_deref(),
        )
    })
    .await
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
    answer: AnswerPayload,
    duration_ms: Option<u32>,
) -> CommandResult<SeriesAnswerResponse> {
    run(state, move |s| {
        s.answer_series(&series_id, index, answer, duration_ms)
    })
    .await
}

/// An exam of a subject (SUB001414).
#[tauri::command]
pub async fn start_exam(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
    subject: Subject,
) -> CommandResult<SeriesResponse> {
    run(state, move |s| s.start_exam(&profile_id, subject, None)).await
}

/// The subjects whose level assessment a profile can still take.
#[tauri::command]
pub async fn get_placement_status(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
) -> CommandResult<PlacementStatusResponse> {
    run(state, move |s| s.placement_status(&profile_id)).await
}

/// The level assessment of a subject, taken once after the creation of a profile.
#[tauri::command]
pub async fn start_placement(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
    subject: Subject,
) -> CommandResult<SeriesResponse> {
    run(state, move |s| s.start_placement(&profile_id, subject, None)).await
}

/// What the timed challenge offers to a profile (SUB001415).
#[tauri::command]
pub async fn get_timed_challenge(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
    subject: Subject,
) -> CommandResult<TimedChallengeResponse> {
    run(state, move |s| s.timed_challenge(&profile_id, subject)).await
}

/// A timed challenge of `seconds` (60, 120 or 180).
#[tauri::command]
pub async fn start_timed(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
    subject: Subject,
    seconds: u32,
) -> CommandResult<SeriesResponse> {
    run(state, move |s| {
        s.start_timed(&profile_id, subject, seconds, None)
    })
    .await
}

/// The end of a timed challenge (the time is over).
#[tauri::command]
pub async fn finish_series(
    state: State<'_, Arc<AppState>>,
    series_id: String,
) -> CommandResult<SeriesSummaryResponse> {
    run(state, move |s| s.finish_series(&series_id)).await
}

/// The mascot (SUB001417): name and pictures; null without a valid mascot.
#[tauri::command]
pub async fn get_mascot(state: State<'_, Arc<AppState>>) -> CommandResult<Option<MascotResponse>> {
    run(state, |s| Ok(s.mascot())).await
}

/// A phrase of the mascot, never twice in a row the same.
#[tauri::command]
pub async fn get_mascot_phrase(
    state: State<'_, Arc<AppState>>,
    situation: MascotSituation,
    title: Option<String>,
) -> CommandResult<Option<MascotPhraseResponse>> {
    run(state, move |s| {
        Ok(s.mascot_phrase(situation, title.as_deref()))
    })
    .await
}

/// WAV of the word of a spelling exercise in progress (`example`: in its short sentence).
#[tauri::command]
pub async fn speak_series_word(
    state: State<'_, Arc<AppState>>,
    series_id: String,
    index: u32,
    example: Option<bool>,
) -> CommandResult<Response> {
    run(state, move |s| {
        s.speak_series_word(&series_id, index, example.unwrap_or(false))
            .map(Response::new)
    })
    .await
}

/// WAV of a word of the bank (`example`: in its short sentence).
#[tauri::command]
pub async fn speak_word(
    state: State<'_, Arc<AppState>>,
    word: String,
    example: Option<bool>,
) -> CommandResult<Response> {
    run(state, move |s| {
        s.speak_word(&word, example.unwrap_or(false))
            .map(Response::new)
    })
    .await
}

#[tauri::command]
pub async fn abandon_series(
    state: State<'_, Arc<AppState>>,
    series_id: String,
) -> CommandResult<SeriesSummaryResponse> {
    run(state, move |s| s.abandon_series(&series_id)).await
}

#[tauri::command]
pub async fn get_multiplication_table(
    state: State<'_, Arc<AppState>>,
    number: i32,
) -> CommandResult<MultiplicationTableResponse> {
    run(state, move |s| s.multiplication_table(number)).await
}

/// The lessons of a grade (CE1 ones when the grade has none yet).
#[tauri::command]
pub async fn list_lessons(
    state: State<'_, Arc<AppState>>,
    grade: Grade,
    subject: Option<Subject>,
) -> CommandResult<LessonsResponse> {
    run(state, move |s| {
        Ok(s.lessons(grade, subject.unwrap_or(Subject::Math)))
    })
    .await
}

/// A lesson, with the slide where the profile left it when `profile_id` is given.
#[tauri::command]
pub async fn get_lesson(
    state: State<'_, Arc<AppState>>,
    id: String,
    profile_id: Option<String>,
) -> CommandResult<LessonResponse> {
    run(state, move |s| s.lesson(&id, profile_id.as_deref())).await
}

/// Keeps the slide where the profile is in a lesson (`slide` null: the lesson is finished).
#[tauri::command]
pub async fn save_lesson_progress(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
    lesson_id: String,
    slide: Option<u32>,
) -> CommandResult<()> {
    run(state, move |s| {
        s.save_lesson_progress(&profile_id, &lesson_id, slide)
    })
    .await
}

/// The end-of-lesson quiz, answered with `answer_series`.
#[tauri::command]
pub async fn start_lesson_quiz(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
    lesson_id: String,
) -> CommandResult<SeriesResponse> {
    run(state, move |s| s.start_lesson_quiz(&profile_id, &lesson_id)).await
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

/// The « adult » gate of the parents' space (SUB001510), drawn again at every call.
#[tauri::command]
pub async fn get_parent_gate(state: State<'_, Arc<AppState>>) -> CommandResult<ParentGateResponse> {
    run(state, |s| Ok(s.parent_gate())).await
}

/// Whether `answer` opens the gate `id`; the gate is used up either way (draw a new one after a
/// wrong answer).
#[tauri::command]
pub async fn check_parent_gate(
    state: State<'_, Arc<AppState>>,
    id: String,
    answer: u32,
) -> CommandResult<bool> {
    run(state, move |s| Ok(s.check_parent_gate(&id, answer))).await
}

/// Statistics of a profile for the parents' space (SUB001511).
#[tauri::command]
pub async fn get_profile_statistics(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
) -> CommandResult<ProfileStatisticsResponse> {
    run(state, move |s| s.profile_statistics(&profile_id)).await
}

/// The parents' card of the profile choice, created with default values the first time.
#[tauri::command]
pub async fn get_parent_profile(
    state: State<'_, Arc<AppState>>,
) -> CommandResult<ParentProfileResponse> {
    run(state, |s| s.parent_profile()).await
}

/// Changes the name or the character of the parents' card (400 with the field when invalid).
#[tauri::command]
pub async fn update_parent_profile(
    state: State<'_, Arc<AppState>>,
    changes: UpdateParentProfileRequest,
) -> CommandResult<ParentProfileResponse> {
    run(state, move |s| s.update_parent_profile(changes)).await
}
