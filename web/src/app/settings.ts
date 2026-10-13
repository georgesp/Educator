import { useSyncExternalStore } from "react";

/**
 * Settings kept on this computer until the Options of lot 3 (SUB001512): the mascot can be hidden
 * (SUB001417). Stored in localStorage, read safely: without storage, the default applies.
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
