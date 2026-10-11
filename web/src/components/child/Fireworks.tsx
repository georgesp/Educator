import { motion } from "motion/react";
import { useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { StarIcon } from "../icons";

/** Length of the animation; the bursts are spread over its first two thirds. */
export const FIREWORKS_DURATION_MS = 2600;

type FireworksProps = {
  /** Read by screen readers (the screen writes its own message next to the fireworks). */
  label: string;
  /** Called when the animation is over (not in the still version, which stays). */
  onDone?(): void;
};

/** Theme colours of the sparks (CSS variables of styles.css). */
const COLORS = ["var(--primary)", "var(--firework-gold)", "var(--correct)", "var(--firework-blue)"];

const PARTICLES_PER_BURST = 18;

/** Where and when each burst explodes: x and y in % of the screen, delay in seconds. Fixed, for calm and reproducible tests. */
const BURSTS = [
  { x: 22, y: 30, delay: 0 },
  { x: 74, y: 24, delay: 0.35 },
  { x: 50, y: 42, delay: 0.7 },
  { x: 30, y: 58, delay: 1.05 },
  { x: 70, y: 55, delay: 1.35 },
];

/**
 * Fireworks over the screen, for a series (and later an exam, SUB001414) done without any mistake
 * (SUB000114). Never blocks the buttons underneath (`pointer-events: none`). When the system asks for
 * less motion, still stars are shown instead, where the component is placed, next to the message.
 */
export function Fireworks({ label, onDone }: FireworksProps) {
  const reduceMotion = usePrefersReducedMotion();
  const [running, setRunning] = useState(true);
  // Latest callback, read when the time is up: a new onDone at each render must not restart the timer.
  const onDoneRef = useRef(onDone);
  useEffect(() => {
    onDoneRef.current = onDone;
  }, [onDone]);

  useEffect(() => {
    if (reduceMotion) {
      return;
    }

    const timer = window.setTimeout(() => {
      setRunning(false);
      onDoneRef.current?.();
    }, FIREWORKS_DURATION_MS);
    return () => window.clearTimeout(timer);
  }, [reduceMotion]);

  if (reduceMotion) {
    return (
      <div className="fireworks-still" role="img" aria-label={label} data-testid="fireworks">
        <span className="fireworks-stars" aria-hidden="true">
          <StarIcon size={40} />
          <StarIcon size={56} />
          <StarIcon size={40} />
        </span>
      </div>
    );
  }

  if (!running) {
    return null;
  }

  // In a portal: a moving (transformed) parent would otherwise trap the fixed overlay.
  return createPortal(
    <div className="fireworks" role="img" aria-label={label} data-testid="fireworks">
      {BURSTS.map((burst, b) => (
        <span key={b} className="fireworks-burst" style={{ left: `${burst.x}%`, top: `${burst.y}%` }} aria-hidden="true">
          {Array.from({ length: PARTICLES_PER_BURST }, (_, p) => {
            const angle = (2 * Math.PI * p) / PARTICLES_PER_BURST + b;
            const distance = 110 + ((p * 37 + b * 11) % 70);
            return (
              <motion.span
                key={p}
                className="fireworks-spark"
                style={{ backgroundColor: COLORS[(p + b) % COLORS.length] }}
                initial={{ x: 0, y: 0, opacity: 1, scale: 1 }}
                animate={{ x: Math.cos(angle) * distance, y: Math.sin(angle) * distance + 40, opacity: [1, 1, 0], scale: [1, 1, 0.5] }}
                transition={{ delay: burst.delay, duration: 1.2, ease: "easeOut", times: [0, 0.6, 1] }}
              />
            );
          })}
        </span>
      ))}
    </div>,
    document.body,
  );
}

const REDUCED_MOTION_QUERY = "(prefers-reduced-motion: reduce)";

/** The system setting « réduire les animations », followed while the component is shown. */
function usePrefersReducedMotion(): boolean {
  const query = () => (typeof window.matchMedia === "function" ? window.matchMedia(REDUCED_MOTION_QUERY) : null);
  const [reduce, setReduce] = useState(() => query()?.matches ?? false);

  useEffect(() => {
    const media = query();
    if (!media) {
      return;
    }

    const update = () => setReduce(media.matches);
    media.addEventListener("change", update);
    return () => media.removeEventListener("change", update);
  }, []);

  return reduce;
}
