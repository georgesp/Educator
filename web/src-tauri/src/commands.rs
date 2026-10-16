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
    run(state, move |s| {
        s.start_placement(&profile_id, subject, None)
    })
    .await
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

/// WAV of the word of a spelling exercise in progress (`example`: in its short sentence; `slow`:
/// « Plus lentement », SUB026702).
#[tauri::command]
pub async fn speak_series_word(
    state: State<'_, Arc<AppState>>,
    series_id: String,
    index: u32,
    example: Option<bool>,
    slow: Option<bool>,
) -> CommandResult<Response> {
    run(state, move |s| {
        s.speak_series_word(
            &series_id,
            index,
            example.unwrap_or(false),
            slow.unwrap_or(false),
        )
        .map(Response::new)
    })
    .await
}

/// WAV of a word of the bank (`example`: in its short sentence; `slow`: read very slowly).
#[tauri::command]
pub async fn speak_word(
    state: State<'_, Arc<AppState>>,
    word: String,
    example: Option<bool>,
    slow: Option<bool>,
    voice_speed: Option<VoiceSpeedName>,
) -> CommandResult<Response> {
    run(state, move |s| {
        s.speak_word(
            &word,
            example.unwrap_or(false),
            slow.unwrap_or(false),
            voice_speed.map(Into::into).unwrap_or_default(),
        )
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
    profile_id: Option<String>,
) -> CommandResult<LessonsResponse> {
    run(state, move |s| {
        s.lessons(
            grade,
            subject.unwrap_or(Subject::Math),
            profile_id.as_deref(),
        )
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
    voice_speed: Option<VoiceSpeedName>,
) -> CommandResult<Response> {
    run(state, move |s| {
        s.speak(
            &text,
            speed,
            voice_speed.map(Into::into).unwrap_or_default(),
        )
        .map(Response::new)
    })
    .await
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

// ── Lot 3: motivation, settings, backup ─────────────────────────────────────────────────────

/// Level, goal of the day, badges and reviews due of a profile (the menu).
#[tauri::command]
pub async fn get_profile_overview(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
) -> CommandResult<ProfileOverviewResponse> {
    run(state, move |s| s.profile_overview(&profile_id)).await
}

/// Every badge, earned or not (SUB001503).
#[tauri::command]
pub async fn get_badges(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
) -> CommandResult<Vec<BadgeResponse>> {
    run(state, move |s| s.badges(&profile_id)).await
}

/// Every accessory, unlocked or not (SUB001505).
#[tauri::command]
pub async fn get_accessories(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
) -> CommandResult<Vec<AccessoryResponse>> {
    run(state, move |s| s.accessories(&profile_id)).await
}

/// Dresses the avatar of a profile (saved at once).
#[tauri::command]
pub async fn update_avatar(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
    request: UpdateAvatarRequest,
) -> CommandResult<ProfileResponse> {
    run(state, move |s| s.update_avatar(&profile_id, request)).await
}

/// An avatar as one SVG.
#[tauri::command]
pub async fn get_avatar_svg(
    state: State<'_, Arc<AppState>>,
    character_id: String,
    accessories: Vec<WornAccessoryResponse>,
) -> CommandResult<String> {
    run(state, move |s| s.avatar_svg(&character_id, &accessories)).await
}

/// « Je te conseille… » for a subject (SUB001508).
#[tauri::command]
pub async fn get_advice(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
    subject: Subject,
) -> CommandResult<Option<AdviceResponse>> {
    run(state, move |s| s.advice(&profile_id, subject)).await
}

/// « Mes révisions » of a subject (SUB001509).
#[tauri::command]
pub async fn start_review(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
    subject: Subject,
) -> CommandResult<SeriesResponse> {
    run(state, move |s| s.start_review(&profile_id, subject, None)).await
}

/// The settings of a child (SUB001512).
#[tauri::command]
pub async fn get_profile_settings(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
) -> CommandResult<ProfileSettingsResponse> {
    run(state, move |s| s.profile_settings(&profile_id)).await
}

#[tauri::command]
pub async fn update_profile_settings(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
    changes: UpdateProfileSettingsRequest,
) -> CommandResult<ProfileSettingsResponse> {
    run(state, move |s| {
        s.update_profile_settings(&profile_id, changes)
    })
    .await
}

/// Themes, goals and durations offered by the settings.
#[tauri::command]
pub async fn get_settings_options(
    state: State<'_, Arc<AppState>>,
) -> CommandResult<SettingsOptionsResponse> {
    run(state, |s| Ok(s.settings_options())).await
}

/// Writes every profile to the backup file chosen by the parents.
#[tauri::command]
pub async fn export_backup(
    state: State<'_, Arc<AppState>>,
    path: String,
) -> CommandResult<BackupResponse> {
    run(state, move |s| s.export_backup(&path)).await
}

/// Replaces every profile by those of a backup file.
#[tauri::command]
pub async fn import_backup(
    state: State<'_, Arc<AppState>>,
    path: String,
) -> CommandResult<BackupResponse> {
    run(state, move |s| s.import_backup(&path)).await
}

// ── Lot tablette 2: synchronisation with the other device (ADR 0007) ─────────────────────────

/// The state of the screen « Synchronisation » of the parents' space.
#[tauri::command]
pub async fn get_sync_status(state: State<'_, Arc<AppState>>) -> CommandResult<SyncStatusResponse> {
    run(state, |s| s.synchronisation().status()).await
}

/// The screen « Synchronisation » is shown: the computer listens while it is open.
#[tauri::command]
pub async fn open_sync_screen(
    state: State<'_, Arc<AppState>>,
) -> CommandResult<SyncStatusResponse> {
    run(state, |s| s.synchronisation().open_screen()).await
}

#[tauri::command]
pub async fn close_sync_screen(
    state: State<'_, Arc<AppState>>,
) -> CommandResult<SyncStatusResponse> {
    run(state, |s| s.synchronisation().close_screen()).await
}

/// Computer: « Recevoir les synchronisations » even with the screen closed.
#[tauri::command]
pub async fn set_sync_receive(
    state: State<'_, Arc<AppState>>,
    enabled: bool,
) -> CommandResult<SyncStatusResponse> {
    run(state, move |s| s.synchronisation().set_receive(enabled)).await
}

/// Computer: « Associer une tablette » (QR code and short code, 5 minutes).
#[tauri::command]
pub async fn start_sync_pairing(
    state: State<'_, Arc<AppState>>,
) -> CommandResult<SyncPairingResponse> {
    run(state, |s| s.synchronisation().start_pairing()).await
}

#[tauri::command]
pub async fn cancel_sync_pairing(
    state: State<'_, Arc<AppState>>,
) -> CommandResult<SyncStatusResponse> {
    run(state, |s| s.synchronisation().cancel_pairing()).await
}

#[tauri::command]
pub async fn rename_sync_device(
    state: State<'_, Arc<AppState>>,
    id: String,
    name: String,
) -> CommandResult<SyncStatusResponse> {
    run(state, move |s| {
        s.synchronisation().rename_device(&id, &name)
    })
    .await
}

/// Computer: « Retirer cette tablette »; iPad: « Dissocier ».
#[tauri::command]
pub async fn remove_sync_device(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CommandResult<SyncStatusResponse> {
    run(state, move |s| s.synchronisation().remove_device(&id)).await
}

/// iPad: pairs with the computer (QR code, or address and short code).
#[tauri::command]
pub async fn pair_with_computer(
    state: State<'_, Arc<AppState>>,
    request: PairWithComputerRequest,
) -> CommandResult<SyncResultResponse> {
    run(state, move |s| s.synchronisation().pair(&request)).await
}

/// iPad: « Synchroniser maintenant », or the automatic attempt at the profile choice.
#[tauri::command]
pub async fn sync_now(
    state: State<'_, Arc<AppState>>,
    automatic: bool,
) -> CommandResult<SyncResultResponse> {
    run(state, move |s| s.sync_now(automatic)).await
}

/// iPad: the parent's answer to « Est-ce le même enfant ? ».
#[tauri::command]
pub async fn answer_sync_question(
    state: State<'_, Arc<AppState>>,
    local_profile_id: String,
    same_child: bool,
) -> CommandResult<SyncStatusResponse> {
    run(state, move |s| {
        s.synchronisation().answer(&local_profile_id, same_child)
    })
    .await
}

/// The application goes to the background: an exchange in progress stops after its batch.
#[tauri::command]
pub async fn interrupt_sync(state: State<'_, Arc<AppState>>) -> CommandResult<()> {
    run(state, |s| {
        s.synchronisation().interrupt();
        Ok(())
    })
    .await
}
