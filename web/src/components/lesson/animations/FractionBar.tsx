import { motion } from "motion/react";
import { useState } from "react";
import type { ComparisonSign, FractionStrip } from "../../../api/client";
import { fr } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";

type FractionBarProps = {
  bars: FractionStrip[];
  comparison: ComparisonSign | null;
  unequal: boolean;
  notation: boolean;
  interactive: boolean;
  onComplete?(): void;
};

const WIDTH = 600;
const HEIGHT = 64;

/** Which color a part has: the first numerator in racing red, the second in blue. */
const colorOf = (strip: FractionStrip, part: number) => {
  const [first = 0, second = 0] = strip.numerators;
  return part < first ? "fraction-part fraction-part-red" : part < first + second ? "fraction-part fraction-part-blue" : "fraction-part";
};

/** A fraction written in two floors (numerator over denominator). */
function Written({ label, notation }: { label: string; notation: boolean }) {
  const [numerator, denominator] = label.split("/");
  return (
    <span className="fraction-written">
      <span className="fraction-floor">
        <span className="fraction-numerator">{numerator}</span>
        {notation && <span className="fraction-note">{fr.lessons.numerator}</span>}
      </span>
      <span className="fraction-rule" aria-hidden="true" />
      <span className="fraction-floor">
        <span className="fraction-denominator">{denominator}</span>
        {notation && <span className="fraction-note">{fr.lessons.denominator}</span>}
      </span>
    </span>
  );
}

function Strip({ strip, unequal = false, colored, onPart }: { strip: FractionStrip; unequal?: boolean; colored?: boolean[]; onPart?(i: number): void }) {
  // Unequal parts: a first part much larger than the others.
  const widths = Array.from({ length: strip.denominator }, (_, i) =>
    unequal ? (i === 0 ? WIDTH * 0.62 : (WIDTH * 0.38) / (strip.denominator - 1)) : WIDTH / strip.denominator,
  );
  let x = 0;
  return (
    <svg className="fraction-svg" viewBox={`-4 -4 ${WIDTH + 8} ${HEIGHT + 8}`} width={WIDTH} role="img" aria-label={fr.lessons.fractionAria(`${strip.colored}/${strip.denominator}`, strip.words)}>
      {widths.map((w, i) => {
        const left = x;
        x += w;
        const cls = colored ? (colored[i] ? "fraction-part fraction-part-red" : "fraction-part") : colorOf(strip, i);
        return onPart ? (
          <rect
            key={i}
            x={left}
            y={0}
            width={w}
            height={HEIGHT}
            className={`${cls} fraction-part-button`}
            role="button"
            tabIndex={0}
            aria-pressed={colored?.[i] ?? false}
            aria-label={fr.lessons.fractionPart(i + 1)}
            onClick={() => onPart(i)}
            onKeyDown={(e) => {
              if (e.key === "Enter" || e.key === " ") {
                e.preventDefault();
                onPart(i);
              }
            }}
          />
        ) : (
          <rect key={i} x={left} y={0} width={w} height={HEIGHT} className={cls} />
        );
      })}
      {unequal && (
        <g className="fraction-cross">
          <line x1={0} y1={0} x2={WIDTH} y2={HEIGHT} />
          <line x1={0} y1={HEIGHT} x2={WIDTH} y2={0} />
        </g>
      )}
    </svg>
  );
}

/**
 * `fractionBar`: strips of track shared in equal parts, the colored parts in racing red (and blue for a
 * sum), the fraction written next to each strip; two strips are compared. `unequal`: parts that are
 * not equal, crossed out. Interactive: the child colors the parts of the strip.
 */
export function FractionBar({ bars, comparison, unequal, notation, interactive, onComplete }: FractionBarProps) {
  const move = useMove();
  const shown = useSteps(interactive ? 0 : bars.length + (comparison || bars[0].numerators.length > 1 || unequal ? 1 : 0), bars);
  const strip = bars[0];
  const [colored, setColored] = useState<boolean[]>(() => Array.from({ length: strip.denominator }, () => false));
  const [done, setDone] = useState(false);
  const count = colored.filter(Boolean).length;

  if (interactive) {
    const toggle = (i: number) => {
      if (done) {
        return;
      }

      const next = colored.map((c, k) => (k === i ? !c : c));
      setColored(next);
      if (next.filter(Boolean).length === strip.colored) {
        setDone(true);
        onComplete?.();
      }
    };
    return (
      <div className="lesson-fractions">
        <div className="fraction-row">
          <Strip strip={strip} colored={colored} onPart={done ? undefined : toggle} />
          {done && <Written label={strip.label} notation={notation} />}
        </div>
        <p className={done ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
          {done ? fr.lessons.fractionDone(strip.words) : count > strip.colored ? fr.lessons.fractionHint : fr.lessons.fractionAsk(strip.colored)}
        </p>
      </div>
    );
  }

  const visible = Math.max(Math.min(shown, bars.length), 1);
  const final = shown > bars.length;

  return (
    <div className="lesson-fractions">
      {bars.slice(0, visible).map((b, i) => (
        <motion.div key={i} className="fraction-row" initial={{ opacity: 0, x: -30 }} animate={{ opacity: 1, x: 0 }} transition={move}>
          <Strip strip={b} unequal={unequal} />
          {!unequal && b.colored > 0 && <Written label={b.label} notation={notation} />}
        </motion.div>
      ))}
      {final && unequal && <p className="objects-caption">{fr.lessons.unequal(strip.denominator)}</p>}
      {final && comparison && bars.length === 2 && <p className="objects-caption">{`${bars[0].label} ${comparison} ${bars[1].label}`}</p>}
      {final && !comparison && strip.numerators.length > 1 && (
        <p className="objects-caption">{fr.lessons.fractionSum(strip.numerators.map((n) => `${n}/${strip.denominator}`), strip.label)}</p>
      )}
    </div>
  );
}
