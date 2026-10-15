import { useEffect, useRef, useState } from "react";
import { usePageVisible } from "../../app/visibility";
import { fr } from "../../i18n/fr";

type QuestionTimerProps = {
  /** Time given for the question. */
  seconds: number;
  /** The time runs only while true: paused during a reading aloud or a confirmation. */
  running: boolean;
  /** The time is over. */
  onOver(): void;
};

const TICK_MS = 100;
const LOW_SECONDS = 5;

/**
 * The time of one question (level assessment, exam with a timer; SUB001418): a bar that empties,
 * with the seconds left. It starts over for each question (`key` the question) and can be paused.
 */
export function QuestionTimer({ seconds, running, onOver }: QuestionTimerProps) {
  const left = useRef(seconds * 1000);
  const fired = useRef(false);
  const over = useRef(onOver);
  over.current = onOver;
  const [remaining, setRemaining] = useState(seconds * 1000);
  // Paused while the application is in the background (a call, another app: SUB023811).
  const visible = usePageVisible();
  const counting = running && visible;

  useEffect(() => {
    if (!counting) {
      return;
    }

    let last = Date.now();
    const timer = window.setInterval(() => {
      const now = Date.now();
      left.current = Math.max(0, left.current - (now - last));
      last = now;
      setRemaining(left.current);
      if (left.current === 0 && !fired.current) {
        fired.current = true;
        over.current();
      }
    }, TICK_MS);
    return () => window.clearInterval(timer);
  }, [counting]);

  const secondsLeft = Math.ceil(remaining / 1000);
  const low = secondsLeft <= LOW_SECONDS;
  return (
    <div className="timer question-timer">
      <div className="timer-bar" role="timer" aria-label={fr.placement.timerLabel(seconds)}>
        <div className={low ? "timer-fill question-timer-fill question-timer-low" : "timer-fill question-timer-fill"} style={{ transform: `scaleX(${remaining / (seconds * 1000)})` }} />
      </div>
      <span className="timer-score question-timer-seconds" aria-hidden="true">
        {fr.placement.secondsLeft(secondsLeft)}
      </span>
    </div>
  );
}
