/**
 * Access to the parents' space (SUB001510): opened by the « adult » gate, closed again when the
 * parent goes back to the child's menu or comes back to the gate. Kept in memory only: a new start
 * of the application asks again.
 */
let unlocked = false;

export function unlockParentSpace(): void {
  unlocked = true;
}

export function lockParentSpace(): void {
  unlocked = false;
}

export function isParentSpaceUnlocked(): boolean {
  return unlocked;
}

/**
 * The screen the parents' space was opened from (router state `{ from }`): the profile choice
 * (the parents' card) or the options, by default.
 */
export function parentOrigin(state: unknown): "/" | "/options" {
  return (state as { from?: string } | null)?.from === "/" ? "/" : "/options";
}
