import { invoke } from "@tauri-apps/api/core";
import { currentVoiceSpeed } from "../app/voiceSpeed";
import {
  toApiError,
  type Accessory,
  type Advice,
  type Backup,
  type Badge,
  type ProfileOverview,
  type ProfileSettings,
  type SettingsOptions,
  type UpdateAvatar,
  type UpdateProfileSettings,
  type WornAccessory,
  type AnswerMode,
  type AnswerPayload,
  type Character,
  type Grade,
  type Lesson,
  type Lessons,
  type Mascot,
  type PlacementStatus,
  type ParentGate,
  type ParentProfile,
  type ProfileStatistics,
  type MascotPhrase,
  type MascotSituation,
  type MultiplicationTable,
  type Profile,
  type Series,
  type SeriesAnswer,
  type SeriesSummary,
  type Skills,
  type SpellingFormat,
  type StatusResponse,
  type Subject,
  type TimedChallenge,
  type UpdateParentProfile,
  type UpdateProfile,
} from "./client";

/** Operations of the application core used by the screens. Injected through BackendContext (mocked in tests). */
export type UiApi = {
  getStatus(): Promise<StatusResponse>;
  /** Skills of a subject (mathematics by default) offered to a grade, with the profile's mastery when given. */
  getSkills(grade: Grade, subject?: Subject, profileId?: string): Promise<Skills>;
  getCharacters(): Promise<Character[]>;
  /** A character drawn at random, different from `except`. */
  drawCharacter(except?: string): Promise<Character>;
  /** SVG source of a character. */
  getCharacterSvg(id: string): Promise<string>;
  listProfiles(): Promise<Profile[]>;
  createProfile(name: string, grade: Grade, characterId?: string): Promise<Profile>;
  updateProfile(id: string, changes: UpdateProfile): Promise<Profile>;
  selectProfile(id: string): Promise<Profile>;
  /** Deletes a profile with its sessions, answers and masteries. */
  deleteProfile(id: string): Promise<void>;
  /** `format`: the kind of spelling exercise chosen (every kind the answer mode allows when absent). */
  startSeries(profileId: string, skillId: string, answerMode: AnswerMode, format?: SpellingFormat): Promise<Series>;
  /** `durationMs`: time taken to answer (statistics, SUB001401). */
  answerSeries(seriesId: string, index: number, answer: AnswerPayload, durationMs?: number): Promise<SeriesAnswer>;
  abandonSeries(seriesId: string): Promise<SeriesSummary>;
  /** The end of a timed challenge (the time is over): the questions answered count. */
  finishSeries(seriesId: string): Promise<SeriesSummary>;
  /** An exam of a subject: 15 questions up to the current period (SUB001414). */
  startExam(profileId: string, subject: Subject): Promise<Series>;
  /** The subjects whose level assessment can still be taken (not done, with skills for the grade). */
  getPlacementStatus(profileId: string): Promise<PlacementStatus>;
  /** The level assessment of a subject (SUB001418): once per subject, valid once finished. */
  startPlacement(profileId: string, subject: Subject): Promise<Series>;
  /** What the timed challenge offers: the mastered skills and the best score of each duration. */
  getTimedChallenge(profileId: string, subject: Subject): Promise<TimedChallenge>;
  /** A timed challenge of 60, 120 or 180 seconds (SUB001415). */
  startTimed(profileId: string, subject: Subject, seconds: number): Promise<Series>;
  /** « Mes tables »: the multiplication table of a number from 0 to 10. */
  getMultiplicationTable(number: number): Promise<MultiplicationTable>;
  /** Lessons of a subject for a grade (those of CE1 when the grade has none yet), with the profile's stars when given. */
  listLessons(grade: Grade, subject?: Subject, profileId?: string): Promise<Lessons>;
  /** A lesson, with the slide where the profile left it (`resumeSlide`) when `profileId` is given. */
  getLesson(id: string, profileId?: string): Promise<Lesson>;
  /** Keeps the slide where the profile is in a lesson (`null`: the lesson is finished). */
  saveLessonProgress(profileId: string, lessonId: string, slide: number | null): Promise<void>;
  /** The end-of-lesson quiz, answered with `answerSeries` and recorded (SUB001411). */
  startLessonQuiz(profileId: string, lessonId: string): Promise<Series>;
  /** The mascot and her pictures (null without a valid mascot in the content). */
  getMascot(): Promise<Mascot | null>;
  /** A phrase of the mascot for a situation, never twice in a row the same. */
  getMascotPhrase(situation: MascotSituation, title?: string): Promise<MascotPhrase | null>;
  /** WAV of a French text (numbers and signs are read as words by the core). */
  speak(text: string, speed?: number): Promise<Blob>;
  /**
   * WAV of the word of a spelling exercise in progress, alone or in its short sentence (`example`),
   * or read very slowly (`slow`, « Plus lentement »).
   */
  speakSeriesWord(seriesId: string, index: number, example?: boolean, slow?: boolean): Promise<Blob>;
  /** WAV of a word of the bank, alone or in its short sentence, or read very slowly. */
  speakWord(word: string, example?: boolean, slow?: boolean): Promise<Blob>;
  /** The « adult » gate of the parents' space, drawn again at every call (SUB001510). */
  getParentGate(): Promise<ParentGate>;
  /** Whether `answer` opens the gate; the gate is used up either way. */
  checkParentGate(id: string, answer: number): Promise<boolean>;
  /** Statistics of a profile for the parents' space (SUB001511). */
  getProfileStatistics(profileId: string): Promise<ProfileStatistics>;
  /** The parents' card of the profile choice, created with default values the first time. */
  getParentProfile(): Promise<ParentProfile>;
  /** Changes the name or the character of the parents' card. */
  updateParentProfile(changes: UpdateParentProfile): Promise<ParentProfile>;
  /** Level, goal of the day, badges and reviews due of a profile (lot 3). */
  getProfileOverview(profileId: string): Promise<ProfileOverview>;
  /** Every badge, earned or not (SUB001503). */
  getBadges(profileId: string): Promise<Badge[]>;
  /** Every accessory, unlocked or not (SUB001505). */
  getAccessories(profileId: string): Promise<Accessory[]>;
  /** Dresses the avatar (saved at once); a locked accessory is refused. */
  updateAvatar(profileId: string, request: UpdateAvatar): Promise<Profile>;
  /** An avatar (character and accessories) as one SVG. */
  getAvatarSvg(characterId: string, accessories: WornAccessory[]): Promise<string>;
  /** « Je te conseille… » for a subject (SUB001508); null when there is nothing to advise. */
  getAdvice(profileId: string, subject: Subject): Promise<Advice | null>;
  /** « Mes révisions » of a subject (SUB001509). */
  startReview(profileId: string, subject: Subject): Promise<Series>;
  /** The settings of a child (SUB001512). */
  getProfileSettings(profileId: string): Promise<ProfileSettings>;
  updateProfileSettings(profileId: string, changes: UpdateProfileSettings): Promise<ProfileSettings>;
  /** Themes, goals and durations offered by the settings. */
  getSettingsOptions(): Promise<SettingsOptions>;
  /** Asks where to save the backup, then writes it; null when the parents cancel. */
  exportBackup(): Promise<Backup | null>;
  /** Asks for a backup file, then restores it (replaces every profile); null when the parents cancel. */
  importBackup(): Promise<Backup | null>;
};

/** Calls a Tauri command (src-tauri/src/commands.rs); a rejection becomes an ApiError. */
async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    throw toApiError(error);
  }
}

/** A command answering WAV bytes: an ArrayBuffer in Tauri (a list of numbers from a simulated core). */
async function wavOf(command: string, args: Record<string, unknown>): Promise<Blob> {
  const wav = await call<ArrayBuffer | number[]>(command, args);
  return new Blob([new Uint8Array(wav)], { type: "audio/wav" });
}

export function createUiApi(): UiApi {
  return {
    getStatus: () => call("get_status"),
    getSkills: (grade, subject, profileId) => call("get_skills", { grade, subject: subject ?? null, profileId: profileId ?? null }),
    getCharacters: () => call("get_characters"),
    drawCharacter: (except) => call("draw_character", { except: except ?? null }),
    getCharacterSvg: (id) => call("get_character_svg", { id }),
    listProfiles: () => call("list_profiles"),
    createProfile: (name, grade, characterId) => call("create_profile", { request: { name, grade, characterId } }),
    updateProfile: (id, changes) => call("update_profile", { id, changes }),
    selectProfile: (id) => call("select_profile", { id }),
    deleteProfile: (id) => call("delete_profile", { id }),
    startSeries: (profileId, skillId, answerMode, format) => call("start_series", { request: { profileId, skillId, answerMode, format } }),
    answerSeries: (seriesId, index, answer, durationMs) => call("answer_series", { seriesId, index, answer, durationMs: durationMs ?? null }),
    abandonSeries: (seriesId) => call("abandon_series", { seriesId }),
    finishSeries: (seriesId) => call("finish_series", { seriesId }),
    startExam: (profileId, subject) => call("start_exam", { profileId, subject }),
    getPlacementStatus: (profileId) => call("get_placement_status", { profileId }),
    startPlacement: (profileId, subject) => call("start_placement", { profileId, subject }),
    getTimedChallenge: (profileId, subject) => call("get_timed_challenge", { profileId, subject }),
    startTimed: (profileId, subject, seconds) => call("start_timed", { profileId, subject, seconds }),
    getMultiplicationTable: (number) => call("get_multiplication_table", { number }),
    listLessons: (grade, subject, profileId) => call("list_lessons", { grade, subject: subject ?? null, profileId: profileId ?? null }),
    getLesson: (id, profileId) => call("get_lesson", { id, profileId: profileId ?? null }),
    saveLessonProgress: (profileId, lessonId, slide) => call("save_lesson_progress", { profileId, lessonId, slide }),
    startLessonQuiz: (profileId, lessonId) => call("start_lesson_quiz", { profileId, lessonId }),
    getMascot: () => call("get_mascot"),
    getMascotPhrase: (situation, title) => call("get_mascot_phrase", { situation, title: title ?? null }),
    speak: (text, speed) => wavOf("speak", { text, speed: speed ?? null, voiceSpeed: currentVoiceSpeed() }),
    speakSeriesWord: (seriesId, index, example, slow) =>
      wavOf("speak_series_word", { seriesId, index, example: example ?? false, slow: slow ?? false }),
    speakWord: (word, example, slow) =>
      wavOf("speak_word", { word, example: example ?? false, slow: slow ?? false, voiceSpeed: currentVoiceSpeed() }),
    getParentGate: () => call("get_parent_gate"),
    checkParentGate: (id, answer) => call("check_parent_gate", { id, answer }),
    getProfileStatistics: (profileId) => call("get_profile_statistics", { profileId }),
    getParentProfile: () => call("get_parent_profile"),
    updateParentProfile: (changes) => call("update_parent_profile", { changes }),
    getProfileOverview: (profileId) => call("get_profile_overview", { profileId }),
    getBadges: (profileId) => call("get_badges", { profileId }),
    getAccessories: (profileId) => call("get_accessories", { profileId }),
    updateAvatar: (profileId, request) => call("update_avatar", { profileId, request }),
    getAvatarSvg: (characterId, accessories) => call("get_avatar_svg", { characterId, accessories }),
    getAdvice: (profileId, subject) => call("get_advice", { profileId, subject }),
    startReview: (profileId, subject) => call("start_review", { profileId, subject }),
    getProfileSettings: (profileId) => call("get_profile_settings", { profileId }),
    updateProfileSettings: (profileId, changes) => call("update_profile_settings", { profileId, changes }),
    getSettingsOptions: () => call("get_settings_options"),
    exportBackup: async () => {
      const { save } = await import("@tauri-apps/plugin-dialog");
      const path = await save({ defaultPath: backupFileName(), filters: [{ name: "Sauvegarde Educator", extensions: ["json"] }] });
      return path ? call<Backup>("export_backup", { path }) : null;
    },
    importBackup: async () => {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const path = await open({ multiple: false, directory: false, filters: [{ name: "Sauvegarde Educator", extensions: ["json"] }] });
      return typeof path === "string" ? call<Backup>("import_backup", { path }) : null;
    },
  };
}

/** `2026-10-15_educator-sauvegarde.json`: the date first, as every file of the family. */
function backupFileName(): string {
  const now = new Date();
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${now.getFullYear()}-${pad(now.getMonth() + 1)}-${pad(now.getDate())}_educator-sauvegarde.json`;
}
