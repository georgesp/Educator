import { motion } from "motion/react";
import { useId, useState, type KeyboardEvent } from "react";
import type { Jump } from "../../../api/client";
import { isTouchScreen } from "../../../app/device";
import { fr, signed } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";
import { CarShape } from "./Cards";

type NumberLineProps = {
  from: number;
  to: number;
  step: number;
  start: number;
  jumps: Jump[];
  /** The child clicks where the car lands after each jump. */
  interactive: boolean;
  /** Interactive line: the value of the graduation under the pointer shows (off when finding it is the lesson). */
  hoverValues: boolean;
  /** Graduations marked with a checkered flag (bounds of a framing, ten to reach). */
  flags?: number[];
  onComplete?(): void;
};

const WIDTH = 1000;
const HEIGHT = 270;
const MARGIN = 50;
const BASE = 160;
const CAR_WIDTH = 56;
/** Two labels closer than this (in the units of the drawing) go on two rows. */
const LABEL_ROOM = 52;
const ROW_HEIGHT = 36;

/** A gap of 10 between labelled graduations, or every graduation when they are far apart. */
const labelled = (value: number, step: number) => step >= 10 || value % 10 === 0;

/** `numberLine`: a graduated track; the car jumps from `start` (+ 20, then − 1). */
export function NumberLine({ from, to, step, start, jumps, interactive, hoverValues, flags = [], onComplete }: NumberLineProps) {
  const shownSteps = useSteps(interactive ? 0 : jumps.length, jumps);
  const [found, setFound] = useState(0);
  const [misses, setMisses] = useState(0);
  // Graduation under the pointer (or focused with the keyboard).
  const [hovered, setHovered] = useState<number | null>(null);
  const move = useMove();
  // Unique per line: two number lines on the same page do not share their checker pattern.
  const checker = `line-checker-${useId().replace(/:/g, "")}`;

  const done = interactive ? found : shownSteps;
  const at = done === 0 ? start : jumps[done - 1].value;
  const asked = interactive && found < jumps.length ? jumps[found] : null;
  const finished = interactive && found === jumps.length;

  const x = (value: number) => MARGIN + ((value - from) / (to - from)) * (WIDTH - 2 * MARGIN);
  const values = Array.from({ length: Math.floor((to - from) / step) + 1 }, (_, i) => from + i * step);
  const gap = (WIDTH - 2 * MARGIN) / (values.length - 1);
  const reached = new Set([start, ...jumps.slice(0, done).map((j) => j.value)]);
  const final = done > 0 ? jumps[done - 1].value : null;

  // Labels from left to right, a label too close to the previous one of its row goes a row lower.
  const rows = new Map<number, number>();
  const lastOfRow: number[] = [];
  for (const v of values.filter((v) => labelled(v, step) || reached.has(v) || flags.includes(v))) {
    let row = 0;
    while (lastOfRow[row] !== undefined && x(v) - lastOfRow[row] < LABEL_ROOM) {
      row += 1;
    }
    lastOfRow[row] = x(v);
    rows.set(v, row);
  }

  const pick = (value: number) => {
    if (!asked) {
      return;
    }

    if (value === asked.value) {
      const next = found + 1;
      setFound(next);
      setMisses(0);
      if (next === jumps.length) {
        onComplete?.();
      }
    } else {
      setMisses((m) => m + 1);
    }
  };

  const onKey = (event: KeyboardEvent, value: number) => {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      pick(value);
    }
  };

  const showHover = interactive && hoverValues && !finished;
  const total = jumps.slice(0, done).reduce((sum, j) => sum + j.delta, 0);
  const departure = done === 0 ? start : found > 0 ? jumps[found - 1].value : start;

  return (
    <div className="lesson-line">
      <svg className="lesson-line-svg" viewBox={`0 0 ${WIDTH} ${HEIGHT}`} role="img" aria-label={fr.lessons.line(from, to)}>
        <rect x={MARGIN - 20} y={BASE - 14} width={WIDTH - 2 * MARGIN + 40} height={28} rx={6} className="line-road" />
        <defs>
          <pattern id={checker} width="14" height="14" patternUnits="userSpaceOnUse">
            <rect width="14" height="14" className="line-checker-light" />
            <rect width="7" height="7" className="line-checker-dark" />
            <rect x="7" y="7" width="7" height="7" className="line-checker-dark" />
          </pattern>
        </defs>
        <rect x={x(to) + 20} y={BASE - 14} width={28} height={28} fill={`url(#${checker})`} />

        {/* Flags on a tall pole, above the car that may stand on the same graduation. */}
        {flags.map((f) => (
          <g key={`flag-${f}`} className="line-flag" role="img" aria-label={fr.lessons.flag(f)}>
            <line x1={x(f)} y1={BASE - 14} x2={x(f)} y2={BASE - 96} className="line-flag-pole" />
            <rect x={x(f)} y={BASE - 96} width={36} height={24} fill={`url(#${checker})`} className="line-flag-cloth" />
          </g>
        ))}

        {values.map((v) => (
          <g key={v}>
            <line x1={x(v)} y1={BASE - (labelled(v, step) ? 12 : 7)} x2={x(v)} y2={BASE + (labelled(v, step) ? 12 : 7)} className="line-tick" />
            {rows.has(v) && (
              <text
                x={x(v)}
                y={BASE + 46 + (rows.get(v) ?? 0) * ROW_HEIGHT}
                textAnchor="middle"
                className={
                  v === start
                    ? "line-label line-label-start"
                    : v === final
                      ? "line-label line-label-key"
                      : reached.has(v)
                        ? "line-label line-label-reached"
                        : "line-label"
                }
              >
                {v}
              </text>
            )}
          </g>
        ))}

        {jumps.slice(0, done).map((jump, i) => {
          const a = i === 0 ? start : jumps[i - 1].value;
          const b = jump.value;
          const lift = 40 + Math.min(Math.abs(x(b) - x(a)) / 4, 70);
          const mid = (x(a) + x(b)) / 2;
          return (
            <g key={i}>
              <motion.path
                d={`M ${x(a)} ${BASE - 18} Q ${mid} ${BASE - 18 - 2 * lift} ${x(b)} ${BASE - 18}`}
                className={i === jumps.length - 1 ? "line-jump line-jump-key" : "line-jump"}
                initial={{ pathLength: 0 }}
                animate={{ pathLength: 1 }}
                transition={move}
              />
              <text x={mid} y={BASE - 26 - lift} textAnchor="middle" className="line-jump-label">
                {signed(jump.delta)}
              </text>
            </g>
          );
        })}

        {/* The starting point, in its own color: the child sees at once where the car leaves from. */}
        <circle cx={x(start)} cy={BASE} r={9} className="line-start" />

        {asked && misses >= 2 && <circle cx={x(asked.value)} cy={BASE} r={16} className="line-target" />}

        <motion.g initial={false} animate={{ x: x(at) - CAR_WIDTH / 2 }} transition={move}>
          <svg x={0} y={BASE - 46} width={CAR_WIDTH} height={28} viewBox="0 0 48 24" aria-hidden="true">
            <CarShape />
          </svg>
        </motion.g>

        {interactive &&
          values.map((v) => (
            <rect
              key={`hit-${v}`}
              x={x(v) - gap / 2}
              y={BASE - 60}
              width={gap}
              height={120}
              className="line-hit"
              role="button"
              tabIndex={finished ? -1 : 0}
              aria-label={fr.series.tick(v)}
              aria-disabled={finished || undefined}
              // A click does not leave a focus frame; the keyboard still moves between the graduations.
              onMouseDown={(e) => e.preventDefault()}
              onClick={() => pick(v)}
              onKeyDown={(e) => onKey(e, v)}
              // The preview follows a mouse only: after a tap it would stay stuck on a touch screen (SUB023809).
              onMouseEnter={() => !isTouchScreen() && setHovered(v)}
              onMouseLeave={() => setHovered((h) => (h === v ? null : h))}
              onFocus={() => setHovered(v)}
              onBlur={() => setHovered((h) => (h === v ? null : h))}
            />
          ))}
        {showHover && hovered !== null && (
          <g className="line-hover" aria-hidden="true">
            <rect x={x(hovered) - 34} y={BASE + 18} width={68} height={42} rx={10} className="line-hover-box" />
            <text x={x(hovered)} y={BASE + 48} textAnchor="middle" className="line-hover-text">
              {hovered}
            </text>
          </g>
        )}
      </svg>

      {interactive && (
        <p className={finished ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
          {finished
            ? fr.lessons.lineDone(start, total, at)
            : misses > 0 && asked
              ? fr.lessons.lineHint(departure, asked.delta)
              : asked
                ? fr.lessons.lineAsk(asked.delta)
                : null}
        </p>
      )}
    </div>
  );
}
