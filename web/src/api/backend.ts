import { invoke } from "@tauri-apps/api/core";
import type { StartupStatus } from "./client";

/**
 * Start-up of the application core (content and database), as reported by the `get_startup`
 * command (src-tauri/src/state.rs). Everything runs inside the Tauri application: no server to wait for.
 */
export type { StartupStatus };

const POLL_INTERVAL_MS = 100;

/** True inside the Tauri WebView (or a test page simulating it); false in a plain browser. */
export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function getStartupStatus(): Promise<StartupStatus> {
  if (!isTauri()) {
    return {
      state: "failed",
      message: "Educator runs inside its Tauri window: start it with `npm run tauri dev` (web/README.md).",
    };
  }

  return invoke<StartupStatus>("get_startup");
}

/** Polls until the core is ready or has failed (loading the content takes a fraction of a second). */
export async function waitForStartup(signal: AbortSignal): Promise<StartupStatus> {
  for (;;) {
    const status = await getStartupStatus();
    if (status.state !== "starting" || signal.aborted) {
      return status;
    }

    await new Promise((resolve) => setTimeout(resolve, POLL_INTERVAL_MS));
  }
}
