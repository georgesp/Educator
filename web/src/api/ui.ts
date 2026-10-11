import type { ReadyBackend } from "./backend";
import {
  createApiClient,
  unwrap,
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

/** Operations of the UI API used by the screens. Injected through BackendContext (mocked in tests). */
export type UiApi = {
  getStatus(): Promise<StatusResponse>;
  getSkills(grade: Grade): Promise<Skills>;
  getCharacters(): Promise<Character[]>;
  /** A character drawn at random by the host, different from `except`. */
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
  /** WAV of a French text (numbers and signs are read as words by the host). */
  speak(text: string, speed?: number): Promise<Blob>;
};

export function createUiApi(backend: ReadyBackend): UiApi {
  const client = createApiClient(backend);
  return {
    getStatus: async () => unwrap(await client.GET("/api/ui/status")),
    getSkills: async (grade) => unwrap(await client.GET("/api/ui/skills", { params: { query: { grade } } })),
    getCharacters: async () => unwrap(await client.GET("/api/ui/characters")),
    drawCharacter: async (except) =>
      unwrap(await client.GET("/api/ui/characters/draw", { params: { query: except === undefined ? {} : { except } } })),
    getCharacterSvg: async (id) =>
      unwrap(await client.GET("/api/ui/characters/{id}.svg", { params: { path: { id } }, parseAs: "text" })) as string,
    listProfiles: async () => unwrap(await client.GET("/api/ui/profiles")),
    createProfile: async (name, grade, characterId) =>
      unwrap(await client.POST("/api/ui/profiles", { body: { name, grade, characterId } })),
    updateProfile: async (id, changes) =>
      unwrap(await client.PATCH("/api/ui/profiles/{id}", { params: { path: { id } }, body: changes })),
    selectProfile: async (id) => unwrap(await client.POST("/api/ui/profiles/{id}/select", { params: { path: { id } } })),
    startSeries: async (profileId, skillId, answerMode) =>
      unwrap(await client.POST("/api/ui/series", { body: { profileId, skillId, answerMode } })),
    answerSeries: async (seriesId, index, answer) =>
      unwrap(await client.POST("/api/ui/series/{id}/answers", { params: { path: { id: seriesId } }, body: { index, answer } })),
    abandonSeries: async (seriesId) =>
      unwrap(await client.POST("/api/ui/series/{id}/abandon", { params: { path: { id: seriesId } } })),
    speak: async (text, speed) =>
      unwrap(await client.POST("/api/ui/speech", { body: { text, speed }, parseAs: "blob" })) as Blob,
  };
}
