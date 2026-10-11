import { invoke } from "@tauri-apps/api/core";
import {
  toApiError,
  type AnswerMode,
  type Character,
  type Grade,
  type Profile,
  type Series,
  type SeriesAnswer,
  type SeriesSummary,
  type Skills,
  type StatusResponse,
  type UpdateProfile,
} from "./client";

/** Operations of the application core used by the screens. Injected through BackendContext (mocked in tests). */
export type UiApi = {
  getStatus(): Promise<StatusResponse>;
  getSkills(grade: Grade): Promise<Skills>;
  getCharacters(): Promise<Character[]>;
  /** A character drawn at random, different from `except`. */
  drawCharacter(except?: string): Promise<Character>;
  /** SVG source of a character. */
  getCharacterSvg(id: string): Promise<string>;
  listProfiles(): Promise<Profile[]>;
  createProfile(name: string, grade: Grade, characterId?: string): Promise<Profile>;
  updateProfile(id: string, changes: UpdateProfile): Promise<Profile>;
  selectProfile(id: string): Promise<Profile>;
  startSeries(profileId: string, skillId: string, answerMode: AnswerMode): Promise<Series>;
  answerSeries(seriesId: string, index: number, answer: number): Promise<SeriesAnswer>;
  abandonSeries(seriesId: string): Promise<SeriesSummary>;
  /** WAV of a French text (numbers and signs are read as words by the core). */
  speak(text: string, speed?: number): Promise<Blob>;
};

/** Calls a Tauri command (src-tauri/src/commands.rs); a rejection becomes an ApiError. */
async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    throw toApiError(error);
  }
}

export function createUiApi(): UiApi {
  return {
    getStatus: () => call("get_status"),
    getSkills: (grade) => call("get_skills", { grade }),
    getCharacters: () => call("get_characters"),
    drawCharacter: (except) => call("draw_character", { except: except ?? null }),
    getCharacterSvg: (id) => call("get_character_svg", { id }),
    listProfiles: () => call("list_profiles"),
    createProfile: (name, grade, characterId) => call("create_profile", { request: { name, grade, characterId } }),
    updateProfile: (id, changes) => call("update_profile", { id, changes }),
    selectProfile: (id) => call("select_profile", { id }),
    startSeries: (profileId, skillId, answerMode) => call("start_series", { request: { profileId, skillId, answerMode } }),
    answerSeries: (seriesId, index, answer) => call("answer_series", { seriesId, index, answer }),
    abandonSeries: (seriesId) => call("abandon_series", { seriesId }),
    speak: async (text, speed) => {
      // Raw bytes: an ArrayBuffer in Tauri (a list of numbers from a simulated core).
      const wav = await call<ArrayBuffer | number[]>("speak", { text, speed: speed ?? null });
      return new Blob([new Uint8Array(wav)], { type: "audio/wav" });
    },
  };
}
