/// <reference types="vite/client" />

interface ImportMetaEnv {
  /** Backend URL when the front runs in a plain browser (outside Tauri), e.g. http://127.0.0.1:5250. */
  readonly VITE_EDUCATOR_BASE_URL?: string;
  /** UI token of that backend (same value as its EDUCATOR_UI_TOKEN). */
  readonly VITE_EDUCATOR_TOKEN?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
