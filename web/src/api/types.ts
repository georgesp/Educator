// Generated from web/src-tauri/src/contracts.rs by `npm run api:generate` (cargo test). Do not edit.

export type Grade = "CP" | "CE1" | "CE2" | "CM1" | "CM2";

export type AnswerMode = "Keyboard" | "Choice";

export type HiddenPart = "Left" | "Right" | "Result";

export type CharacterKind = "Animal" | "Child";

export type StartupStatus = { "state": "starting" } | { "state": "ready" } | { "state": "failed", message: string, };

export type StatusResponse = { version: string, database: DatabaseStatusResponse, speech: SpeechStatusResponse, };

export type DatabaseStatusResponse = { 
/**
 * True once the database is created and migrated.
 */
isReady: boolean, };

export type SpeechStatusResponse = { isAvailable: boolean, 
/**
 * Voice name, when available (e.g. `fr_FR-siwis-medium`).
 */
voice: string | null, 
/**
 * Why the voice is unavailable (English, diagnostics only).
 */
problem: string | null, };

export type SkillsResponse = { 
/**
 * Grade asked for.
 */
grade: Grade, 
/**
 * Grade of the returned skills: the asked one, or CE1 when the asked grade has no skill yet.
 */
skillsGrade: Grade, 
/**
 * School period of today (1 to 5), from `content/calendar/periods.json`.
 */
currentPeriod: number, skills: Array<SkillResponse>, };

export type SkillResponse = { id: string, title: string, instruction: string, 
/**
 * Period where the skill is offered to this grade.
 */
period: number, 
/**
 * True for the skills of the current period (revisions excepted).
 */
isCurrentPeriod: boolean, 
/**
 * Grade the skill comes from when it is offered as a revision (SUB000113), otherwise `null`.
 */
revisionFrom: Grade | null, answerModes: Array<AnswerMode>, };

export type CharacterResponse = { id: string, name: string, kind: CharacterKind, };

export type ProfileResponse = { id: string, name: string, grade: Grade, characterId: string, points: number, 
/**
 * ISO 8601 instant in UTC, e.g. `2026-10-11T08:00:00.000Z`.
 */
lastUsedAt: string, };

export type CreateProfileRequest = { name: string, grade: Grade, 
/**
 * Character shown on the creation screen (`draw_character`); drawn at random when absent.
 */
characterId?: string, };

export type UpdateProfileRequest = { name?: string, grade?: Grade, characterId?: string, };

export type StartSeriesRequest = { profileId: string, skillId: string, answerMode: AnswerMode, 
/**
 * Fixed seed to replay a series (tests, debugging); random when absent.
 */
seed?: number, };

export type SeriesResponse = { seriesId: string, seed: number, skillId: string, instruction: string, answerMode: AnswerMode, exercises: Array<ExerciseResponse>, };

export type ExerciseResponse = { index: number, left: number | null, 
/**
 * `+` or `−` (U+2212).
 */
operator: string, right: number | null, result: number | null, hidden: HiddenPart, 
/**
 * Written form with `?` for the asked number, e.g. `7 + ? = 12`.
 */
prompt: string, 
/**
 * French question to read aloud (send it to `speak`).
 */
spokenPrompt: string, 
/**
 * Four proposals in QCM mode, otherwise null.
 */
choices: Array<number> | null, };

export type SeriesAnswerResponse = { correct: boolean, expected: number, 
/**
 * Complete equality shown and read after a mistake, e.g. `7 + 5 = 12`.
 */
explanation: string, 
/**
 * Set when this answer ended the series: points awarded.
 */
summary: SeriesSummaryResponse | null, };

export type SeriesSummaryResponse = { correctCount: number, exerciseCount: number, pointsEarned: number, 
/**
 * New total of the profile.
 */
profilePoints: number, };

export type CommandError = { status: number, title: string, detail: string | null, 
/**
 * Request field in error (`name`, `grade`, `characterId`), for the profile errors.
 */
field: string | null, };
