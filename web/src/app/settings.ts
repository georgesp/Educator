import { useSyncExternalStore } from "react";

/**
 * The mascot hidden or shown (SUB001417). Since lot 3 it is a setting of each child (SUB001512), applied
 * by `ProfileSettingsEffect`; localStorage only keeps the last value for the screens without a child.
 * Read safely: without storage, the default applies.
 */
const MASCOT_HIDDEN_KEY = "educator.mascot.hidden";

const listeners = new Set<() => void>();

function read(key: string): string | null {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}

function write(key: string, value: string): void {
  try {
    localStorage.setItem(key, value);
  } catch {
    // Storage unavailable: the setting only lasts for this session.
  }
}

let mascotHidden = read(MASCOT_HIDDEN_KEY) === "1";

export function isMascotHidden(): boolean {
  return mascotHidden;
}

export function setMascotHidden(hidden: boolean): void {
  mascotHidden = hidden;
  write(MASCOT_HIDDEN_KEY, hidden ? "1" : "0");
  listeners.forEach((l) => l());
}

/** Whether the mascot is hidden, updated when the option changes. */
export function useMascotHidden(): boolean {
  return useSyncExternalStore(
    (listener) => {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
    isMascotHidden,
    isMascotHidden,
  );
}
