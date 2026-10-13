import { motion, useReducedMotion } from "motion/react";
import { useEffect, useRef } from "react";
import { fr } from "../../i18n/fr";

type TimerBarProps = {
  seconds: number;
  /** Right answers so far, shown next to the bar. */
  rightCount: number;
  /** The time is over. */
  onOver(): void;
};

/**
 * The time of a timed challenge (SUB001415): a bar that empties, discreet, without sound or
 * countdown read aloud — the challenge must stay a game, not a pressure.
 */
export function TimerBar({ seconds, rightCount, onOver }: TimerBarProps) {
  const reduce = useReducedMotion() ?? false;
  const over = useRef(onOver);
  over.current = onOver;

  useEffect(() => {
    const timer = window.setTimeout(() => over.current(), seconds * 1000);
    return () => window.clearTimeout(timer);
  }, [seconds]);

  return (
    <div className="timer">
      <div className="timer-bar" role="timer" aria-label={fr.exam.timerLabel(seconds)}>
        <motion.div
          className="timer-fill"
          initial={{ scaleX: 1 }}
          animate={{ scaleX: 0 }}
          transition={{ duration: seconds, ease: "linear" }}
          style={reduce ? { opacity: 0.6 } : undefined}
        />
      </div>
      <span className="timer-score" aria-live="polite">
        {fr.exam.rightSoFar(rightCount)}
      </span>
    </div>
  );
}
