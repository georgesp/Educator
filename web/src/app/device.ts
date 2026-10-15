/**
 * Touch screens (the iPad, SUB023810): the answer fields use the keyboards of the application
 * (`NumberPad`, `LetterKeyboard`) and the system keyboard never opens over the exercise, so it can
 * neither hide the field nor correct the child's spelling. With a mouse (the computer), nothing changes.
 */
export function isTouchScreen(): boolean {
  return typeof window !== "undefined" && typeof window.matchMedia === "function" && window.matchMedia("(pointer: coarse)").matches;
}

/**
 * The iPad (SUB023814): what it cannot do yet, such as saving the profiles to a file of its choice,
 * is hidden there. Since iPadOS 13, Safari introduces itself as a Mac: the touch points tell them apart.
 */
export function isIpad(): boolean {
  if (typeof navigator === "undefined") {
    return false;
  }
  return /iPad|iPhone/.test(navigator.userAgent) || (/Macintosh/.test(navigator.userAgent) && navigator.maxTouchPoints > 1);
}

/** `inputMode` of an answer field: `none` on a touch screen, where the application's keyboard is shown. */
export function answerInputMode(onComputer: "numeric" | "text"): "numeric" | "text" | "none" {
  return isTouchScreen() ? "none" : onComputer;
}
