import { useReducedMotion } from "motion/react";
import { useEffect, useState } from "react";

/** Pause between two steps of an animation shown by itself (each move lasts less than 600 ms). */
export const LESSON_STEP_MS = 900;

/**
 * Number of steps revealed so far, one more every LESSON_STEP_MS until `count`. With « reduce motion »
 * asked by the system, everything is shown at once. Starts again when `key` changes.
 */
export function useSteps(count: number, key: unknown = null): number {
  const reduce = useReducedMotion() ?? false;
  const [shown, setShown] = useState(reduce ? count : 0);

  useEffect(() => {
    setShown(reduce ? count : 0);
  }, [count, key, reduce]);

  useEffect(() => {
    if (shown >= count) {
      return;
    }

    const timer = window.setTimeout(() => setShown((s) => Math.min(s + 1, count)), LESSON_STEP_MS);
    return () => window.clearTimeout(timer);
  }, [shown, count]);

  return shown;
}

/** Short moves (< 600 ms), replaced by a plain fade when the system asks to reduce motion. */
export function useMove(): { duration: number } {
  const reduce = useReducedMotion() ?? false;
  return { duration: reduce ? 0.2 : 0.45 };
}
