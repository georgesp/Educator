// Generated from web/src-tauri/src/contracts.rs by `npm run api:generate` (cargo test). Do not edit.

export type Grade = "CP" | "CE1" | "CE2" | "CM1" | "CM2";

export type AnswerMode = "Keyboard" | "Choice";

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
revisionFrom: Grade | null, answerModes: Array<AnswerMode>, family: SkillFamily, };

export type SkillFamily = "Calculation" | "Multiplication" | "Numeration" | "DoubleHalf" | "Fraction" | "Problem";

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

export type ExerciseResponse = { index: number, 
/**
 * Sentence shown above the statement (« Quel est le double de 35 ? », a problem).
 */
question: string | null, 
/**
 * The written statement, e.g. `7`, `+`, slot, `=`, `12`.
 */
statement: Array<TokenResponse>, picture: PictureResponse | null, input: InputResponse, 
/**
 * Proposals of a pick (QCM), otherwise null.
 */
choices: Array<ChoiceResponse> | null, 
/**
 * Written form with `?` for the answer, e.g. `7 + ? = 12` (accessibility).
 */
prompt: string, 
/**
 * French text to read aloud (send it to `speak`).
 */
spokenPrompt: string, };

export type TokenResponse = { "kind": "number", value: number, } | { "kind": "sign", value: string, } | { "kind": "text", value: string, } | { "kind": "fraction", numerator: number, denominator: number, } | { "kind": "slot", slot: number, };

export type FractionShape = "Bar" | "Disc";

export type PictureResponse = { "kind": "baseTen", hundreds: number, tens: number, units: number, } | { "kind": "fractionShape", shape: FractionShape, parts: number, colored: number, } | { "kind": "numberLine", start: number, step: number, ticks: number, labeled: Array<number>, arrow: number | null, } | { "kind": "arrayGrid", rows: number, columns: number, split: number | null, } | { "kind": "barModel", bars: Array<Array<number | null>>, total: number | null, } | { "kind": "groups", groups: number, perGroup: number, };

export type InputResponse = { "kind": "number", slots: number, } | { "kind": "fraction" } | { "kind": "comparison" } | { "kind": "yesNo" } | { "kind": "order", items: Array<number>, ascending: boolean, } | { "kind": "multiChoice", items: Array<number>, } | { "kind": "pick" } | { "kind": "numberLine" };

export type ChoiceResponse = { label: string, answer: AnswerPayload, };

export type ComparisonSign = "<" | ">" | "=";

export type AnswerPayload = { "kind": "number", value: number, } | { "kind": "fraction", numerator: number, denominator: number, } | { "kind": "comparison", sign: ComparisonSign, } | { "kind": "yesNo", value: boolean, } | { "kind": "sequence", values: Array<number>, } | { "kind": "set", values: Array<number>, } | { "kind": "text", value: string, };

export type SeriesAnswerResponse = { correct: boolean, expected: AnswerPayload, 
/**
 * The expected answer as written in the correction (« 3/4 », « < », « 300 et 400 »).
 */
expectedLabel: string, 
/**
 * Correction shown and read after a mistake, e.g. `7 + 5 = 12`.
 */
explanation: string, 
/**
 * Picture shown with the correction (decomposition, bar model).
 */
help: PictureResponse | null, 
/**
 * Set when this answer ended the series: points awarded.
 */
summary: SeriesSummaryResponse | null, };

export type SeriesSummaryResponse = { correctCount: number, exerciseCount: number, pointsEarned: number, 
/**
 * New total of the profile.
 */
profilePoints: number, };

export type MultiplicationTableResponse = { number: number, lines: Array<TableLineResponse>, };

export type TableLineResponse = { left: number, right: number, result: number, 
/**
 * « 3 fois 7, 21. »
 */
spoken: string, };

export type LessonsResponse = { 
/**
 * Grade asked for.
 */
grade: Grade, 
/**
 * Grade of the returned lessons: the asked one, or CE1 when the asked grade has no lesson yet.
 */
lessonsGrade: Grade, 
/**
 * School period of today (1 to 5).
 */
currentPeriod: number, lessons: Array<LessonSummaryResponse>, };

export type LessonSummaryResponse = { id: string, title: string, period: number, 
/**
 * True for the lessons of the current period.
 */
isCurrentPeriod: boolean, durationMinutes: number, };

export type LessonResponse = { id: string, title: string, grade: Grade, 
/**
 * Skills practised after the lesson (« Je m'entraîne »), the first one being the main one.
 */
skillIds: Array<string>, slides: Array<SlideResponse>, quiz: Array<QuizQuestionResponse>, };

export type SlideResponse = { title: string, 
/**
 * Shown.
 */
text: string, 
/**
 * Read aloud (send it to `speak`).
 */
speech: string, 
/**
 * The child has to do something before going on.
 */
interactive: boolean, animation: AnimationResponse, };

export type AnimationResponse = { "type": "titleCard", title: string, subtitle: string | null, } | { "type": "recapCard", text: string, example: string | null, } | { "type": "calcChain", chains: Array<ChainResponse>, mode: ChainMode, } | { "type": "numberLine", from: number, to: number, step: number, start: number, jumps: Array<JumpResponse>, 
/**
 * The child clicks each landing graduation, in order.
 */
interactive: boolean, 
/**
 * The value of a graduation shows under the pointer (interactive line).
 */
hoverValues: boolean, } | { "type": "tensUnits", value: number, hundreds: number, tens: number, units: number, missing: number, };

export type ChainMode = "Show" | "Match";

export type ChainResponse = { start: number, steps: Array<JumpResponse>, 
/**
 * What the chain adds in all (+ 20 − 1 → 19): the calculation is `start + total`.
 */
total: number, result: number, 
/**
 * A classic mistake, shown crossed out.
 */
crossed: boolean, };

export type JumpResponse = { delta: number, value: number, };

export type QuizQuestionResponse = { index: number, question: string, 
/**
 * Read aloud (send it to `speak`).
 */
spoken: string, choices: Array<ChoiceResponse> | null, };

export type QuizAnswerResponse = { correct: boolean, expected: AnswerPayload, expectedLabel: string, 
/**
 * Shown and read after the answer, right or wrong.
 */
explanation: string, };

export type CommandError = { status: number, title: string, detail: string | null, 
/**
 * Request field in error (`name`, `grade`, `characterId`), for the profile errors.
 */
field: string | null, };
