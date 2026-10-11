import type { ReadyBackend } from "./backend";
import { createApiClient, unwrap, type AdditionExercise, type AnswerResponse, type StatusResponse } from "./client";

/** Operations of the UI API used by the screens. Injected through BackendContext (mocked in tests). */
export type UiApi = {
  getStatus(): Promise<StatusResponse>;
  createAddition(): Promise<AdditionExercise>;
  answerAddition(id: string, answer: number): Promise<AnswerResponse>;
  /** WAV of the spoken statement. */
  readAddition(id: string, speed?: number): Promise<Blob>;
};

export function createUiApi(backend: ReadyBackend): UiApi {
  const client = createApiClient(backend);
  return {
    getStatus: async () => unwrap(await client.GET("/api/ui/status")),
    createAddition: async () => unwrap(await client.POST("/api/ui/exercises/additions")),
    answerAddition: async (id, answer) =>
      unwrap(
        await client.POST("/api/ui/exercises/additions/{id}/answer", {
          params: { path: { id } },
          body: { answer },
        }),
      ),
    readAddition: async (id, speed) =>
      unwrap(
        await client.GET("/api/ui/exercises/additions/{id}/speech", {
          params: { path: { id }, query: speed === undefined ? undefined : { speed } },
          parseAs: "blob",
        }),
      ) as Blob,
  };
}
