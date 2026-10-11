import { createContext, useContext, type ReactNode } from "react";
import type { UiApi } from "../api/ui";

const BackendContext = createContext<UiApi | null>(null);

/** Provides the typed UI API to the screens (a fake one in the component tests). */
export function BackendProvider({ api, children }: { api: UiApi; children: ReactNode }) {
  return <BackendContext.Provider value={api}>{children}</BackendContext.Provider>;
}

/** The UI API. Only usable below `BackendProvider`. */
export function useApi(): UiApi {
  const api = useContext(BackendContext);
  if (!api) {
    throw new Error("useApi must be used below BackendProvider.");
  }

  return api;
}
