import { invoke } from "@tauri-apps/api/core";

/**
 * State of the .NET backend (educator-host), as reported by the Tauri shell
 * (`get_backend` command, see src-tauri/src/backend.rs).
 */
export type BackendStatus =
  | { state: "starting" }
  | { state: "ready"; baseUrl: string; token: string }
  | { state: "failed"; message: string };

export type ReadyBackend = Extract<BackendStatus, { state: "ready" }>;

const POLL_INTERVAL_MS = 300;

/** True inside the Tauri WebView; false in a plain browser (`npm run dev`, tests). */
export const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

/**
 * Current backend status. Outside Tauri, the backend is the one given by VITE_EDUCATOR_BASE_URL /
 * VITE_EDUCATOR_TOKEN (e.g. a host started with `dotnet run`, see web/README.md).
 */
export async function getBackendStatus(): Promise<BackendStatus> {
  if (isTauri) {
    return invoke<BackendStatus>("get_backend");
  }

  const baseUrl = import.meta.env.VITE_EDUCATOR_BASE_URL;
  const token = import.meta.env.VITE_EDUCATOR_TOKEN;
  return baseUrl && token
    ? { state: "ready", baseUrl, token }
    : {
        state: "failed",
        message:
          "Outside Tauri, set VITE_EDUCATOR_BASE_URL and VITE_EDUCATOR_TOKEN (web/.env.local) to reach a running educator-host.",
      };
}

/** Polls until the backend is ready or has failed (`dotnet run` builds the host first: this can take a while). */
export async function waitForBackend(signal: AbortSignal): Promise<BackendStatus> {
  for (;;) {
    const status = await getBackendStatus();
    if (status.state !== "starting" || signal.aborted) {
      return status;
    }

    await new Promise((resolve) => setTimeout(resolve, POLL_INTERVAL_MS));
  }
}
