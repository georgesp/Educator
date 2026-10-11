import createClient, { type Middleware } from "openapi-fetch";
import type { ReadyBackend } from "./backend";
import type { components, paths } from "./schema";

export type Schemas = components["schemas"];
export type StatusResponse = Schemas["StatusResponse"];
export type AdditionExercise = Schemas["AdditionExerciseResponse"];
export type AnswerResponse = Schemas["AnswerResponse"];

/** Error returned by the UI API (RFC 7807 problem details). */
export class ApiError extends Error {
  constructor(
    readonly status: number,
    readonly title: string,
    readonly detail?: string,
  ) {
    super(detail ? `${title}: ${detail}` : title);
    this.name = "ApiError";
  }
}

/** Typed client of the UI API, generated from web/openapi/openapi.json — never written by hand. */
export function createApiClient(backend: ReadyBackend) {
  const client = createClient<paths>({ baseUrl: backend.baseUrl });
  const auth: Middleware = {
    onRequest({ request }) {
      request.headers.set("X-Local-Token", backend.token);
      return request;
    },
  };
  client.use(auth);
  return client;
}

type Problem = { title?: string | null; detail?: string | null } | undefined;

/** Unwraps an openapi-fetch result: the data, or an ApiError. */
export function unwrap<T>(result: { data?: T; error?: unknown; response: Response }): T {
  if (result.error !== undefined || result.data === undefined) {
    const problem = result.error as Problem;
    throw new ApiError(result.response.status, problem?.title ?? result.response.statusText, problem?.detail ?? undefined);
  }

  return result.data;
}
