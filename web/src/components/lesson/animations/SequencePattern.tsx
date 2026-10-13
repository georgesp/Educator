import { AnimatePresence, motion } from "motion/react";
import { Fragment, useState } from "react";
import type { SequenceMode } from "../../../api/client";
import { fr, signed } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";
import { ColoredCar, type ObjectColor } from "./RaceObjects";

type SequencePatternProps = {
  mode: SequenceMode;
  items: number[];
  ordered: number[];
  hidden: number[];
  step: number;
  descending: boolean;
  interactive: boolean;
  onComplete?(): void;
};

const COLORS: ObjectColor[] = ["red", "blue", "yellow", "green"];

/** A car carrying a number, like on the starting grid. */
function NumberCar({ value, color, hidden = false }: { value: number | null; color: ObjectColor; hidden?: boolean }) {
  return (
    <span className="sequence-car">
      <svg width="72" height="36" viewBox="0 0 48 24" aria-hidden="true">
        <ColoredCar color={color} />
      </svg>
      <span className={hidden ? "sequence-number sequence-hidden" : "sequence-number"}>{value ?? "?"}</span>
    </span>
  );
}

/** `sequencePattern`: a counting sequence with hidden numbers, or cars to put in order on the grid. */
export function SequencePattern(props: SequencePatternProps) {
  return props.mode === "Count" ? <CountSequence {...props} /> : <OrderSequence {...props} />;
}

/** The numbers of the sequence, the hidden ones in a box, revealed one by one. */
function CountSequence({ items, hidden, step }: SequencePatternProps) {
  const revealed = useSteps(hidden.length, items);
  const move = useMove();

  return (
    <div className="lesson-sequence" role="img" aria-label={fr.lessons.sequence(items.join(", "))}>
      {items.map((value, i) => {
        const at = hidden.indexOf(i);
        const isHidden = at >= 0 && at >= revealed;
        return (
          <Fragment key={i}>
            {i > 0 && <span className="sequence-step">{signed(step)}</span>}
            <motion.span initial={false} animate={{ scale: at >= 0 && !isHidden ? [1.2, 1] : 1 }} transition={move}>
              <NumberCar value={isHidden ? null : value} color={at >= 0 ? "red" : "blue"} hidden={isHidden} />
            </motion.span>
          </Fragment>
        );
      })}
    </div>
  );
}

/** The cars go to the starting grid in order; interactive, the child clicks them in that order. */
function OrderSequence({ items, ordered, descending, interactive, onComplete }: SequencePatternProps) {
  const auto = useSteps(interactive ? 0 : ordered.length, items);
  const [placed, setPlaced] = useState(0);
  const [miss, setMiss] = useState(false);
  const move = useMove();
  const count = interactive ? placed : auto;
  const onGrid = ordered.slice(0, count);
  const done = interactive && placed === ordered.length;
  const colorOf = (value: number) => COLORS[items.indexOf(value) % COLORS.length];

  const pick = (value: number) => {
    if (!interactive || done) {
      return;
    }

    if (value === ordered[placed]) {
      const next = placed + 1;
      setPlaced(next);
      setMiss(false);
      if (next === ordered.length) {
        onComplete?.();
      }
    } else {
      setMiss(true);
    }
  };

  return (
    <div className="lesson-order">
      <div className="order-waiting" role="group" aria-label={fr.lessons.orderAsk(descending)}>
        <AnimatePresence>
          {items
            .filter((v) => !onGrid.includes(v))
            .map((value) =>
              interactive ? (
                <motion.button key={value} type="button" className="order-car-button" onClick={() => pick(value)} exit={{ opacity: 0, y: 40 }} transition={move}>
                  <NumberCar value={value} color={colorOf(value)} />
                </motion.button>
              ) : (
                <motion.span key={value} exit={{ opacity: 0, y: 40 }} transition={move}>
                  <NumberCar value={value} color={colorOf(value)} />
                </motion.span>
              ),
            )}
        </AnimatePresence>
      </div>
      <div className="order-grid" aria-label={fr.lessons.orderGrid}>
        {ordered.map((value, i) => (
          <span key={i} className="order-slot">
            <span className="order-rank">{i + 1}</span>
            {i < count && (
              <motion.span initial={{ opacity: 0, y: -40 }} animate={{ opacity: 1, y: 0 }} transition={move}>
                <NumberCar value={value} color={colorOf(value)} />
              </motion.span>
            )}
            {i < ordered.length - 1 && <span className="order-sign">{descending ? ">" : "<"}</span>}
          </span>
        ))}
      </div>
      {interactive && (
        <p className={done ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
          {done ? fr.lessons.orderDone : miss ? fr.lessons.orderHint(descending) : fr.lessons.orderAsk(descending)}
        </p>
      )}
    </div>
  );
}
