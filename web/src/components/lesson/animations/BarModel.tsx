import { motion } from "motion/react";
import { useState, type ReactNode } from "react";
import type { BarKind, BarValue, Equality, EquationTerm } from "../../../api/client";
import { fr } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";

type BarModelProps = {
  kind: BarKind;
  values: BarValue[];
  groups: number;
  labels: string[];
  answer: number;
  solution: Equality;
  solutionMissing: EquationTerm | null;
  solve: boolean;
  interactive: boolean;
  onComplete?(): void;
};

/** Width of the whole, in pixels. */
const FULL = 620;

/**
 * `barModel`: the bar diagram of a problem (« schéma en barres »): a whole and its parts, two
 * quantities compared, or equal groups. The « ? » is what the problem asks; `solve` shows the
 * calculation that finds it. Interactive: the child places the numbers of the story on the diagram.
 */
export function BarModel({ kind, values, groups, labels, answer, solution, solutionMissing, solve, interactive, onComplete }: BarModelProps) {
  const move = useMove();
  const shown = useSteps(interactive ? 0 : solve ? 1 : 0, values);
  const [placed, setPlaced] = useState<number[]>([]);
  const [used, setUsed] = useState<number[]>([]);
  const [selected, setSelected] = useState<number | null>(null);
  const [miss, setMiss] = useState(false);

  const known = values.map((v, i) => ({ ...v, i })).filter((v) => !v.unknown);
  const filled = !interactive || placed.length === known.length;
  const solved = solve && (interactive ? filled : shown >= 1);
  const whole = values[0];
  const parts = values.slice(1);

  const place = (slot: number) => {
    if (selected === null || placed.includes(slot)) {
      return;
    }

    if (values[selected].value === values[slot].value) {
      const next = [...placed, slot];
      setPlaced(next);
      setUsed([...used, selected]);
      setSelected(null);
      setMiss(false);
      if (next.length === known.length) {
        onComplete?.();
      }
    } else {
      setMiss(true);
    }
  };

  /** Name of the empty slot of value `i` (the whole, a part, the smaller quantity, the difference). */
  const slotName = (i: number) =>
    i === 0 ? fr.lessons.barSlot("whole", 0) : kind === "Compare" ? fr.lessons.barSlot(i === 1 ? "small" : "difference", i) : fr.lessons.barSlot("part", i);

  /** The number shown at value `i`: the number, « ? », or an empty slot to fill. */
  const label = (i: number): ReactNode => {
    const v = values[i];
    if (v.unknown) {
      return solved ? (
        <motion.span className="bar-answer" initial={{ scale: 0.5 }} animate={{ scale: 1 }} transition={move}>
          {answer}
        </motion.span>
      ) : (
        <span className="bar-question">{fr.lessons.barUnknown}</span>
      );
    }
    if (interactive && !placed.includes(i)) {
      return (
        <button type="button" className="bar-slot" aria-label={slotName(i)} onClick={() => place(i)}>
          {" "}
        </button>
      );
    }
    return <span className="bar-number">{v.value}</span>;
  };

  const scale = FULL / Math.max(whole.value, 1);
  const segment = (value: number, cls: string, content: ReactNode, key: string | number) => (
    <div key={key} className={`bar-segment ${cls}`} style={{ width: Math.max(value * scale, 70) }}>
      {content}
    </div>
  );

  let diagram: ReactNode;
  if (kind === "Parts") {
    diagram = (
      <div className="bar-diagram">
        <div className="bar-row">{parts.map((p, k) => segment(p.value, k % 2 === 0 ? "bar-segment-a" : "bar-segment-b", label(k + 1), k))}</div>
        <div className="bar-brace" style={{ width: Math.max(...[FULL, parts.length * 70]) }}>
          <span className="bar-brace-label">
            {label(0)} <span className="bar-caption">{fr.lessons.barWhole}</span>
          </span>
        </div>
      </div>
    );
  } else if (kind === "Compare") {
    const [small, difference] = parts;
    diagram = (
      <div className="bar-diagram">
        <div className="bar-row">
          <span className="bar-name">{labels[0] ?? ""}</span>
          {segment(small.value, "bar-segment-a", label(1), "small")}
        </div>
        <div className="bar-row">
          <span className="bar-name">{labels[1] ?? ""}</span>
          {/* The larger quantity (« ? » until solved) in the middle of its bar, like the smaller one above. */}
          {segment(small.value, "bar-segment-a", label(0), "same")}
          {segment(difference.value, "bar-segment-diff", <>{label(2)} <span className="bar-caption">{fr.lessons.barDifference}</span></>, "diff")}
        </div>
      </div>
    );
  } else {
    diagram = (
      <div className="bar-diagram">
        <div className="bar-row">
          {Array.from({ length: groups }, (_, k) => segment(parts[0].value, k % 2 === 0 ? "bar-segment-a" : "bar-segment-b", k === 0 ? label(1) : <span className="bar-number bar-number-copy">{parts[0].unknown && !solved ? "?" : parts[0].value}</span>, k))}
        </div>
        <div className="bar-brace" style={{ width: Math.max(FULL, groups * 70) }}>
          <span className="bar-brace-label">
            {label(0)} <span className="bar-caption">{fr.lessons.barWhole}</span>
          </span>
        </div>
      </div>
    );
  }

  const sign = fr.lessons.sign[solution.operator];
  const term = (which: EquationTerm, value: number) => (solutionMissing === which ? "?" : value);
  const calculation = `${term("Left", solution.left)} ${sign} ${term("Right", solution.right)} = ${term("Result", solution.result)}`;

  return (
    <div className="lesson-bar-model" role="group" aria-label={fr.lessons.barAria}>
      {diagram}
      {solved && (
        <motion.p className="bar-solution" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} transition={move}>
          {calculation}
          {solutionMissing !== null && <span className="bar-solution-answer">{` → ${answer}`}</span>}
        </motion.p>
      )}
      {interactive && (
        <>
          {!filled && (
            <div className="lesson-choices" role="group" aria-label={fr.lessons.barNumbers}>
              {known
                .filter((v) => !used.includes(v.i))
                .map((v) => (
                  <button key={v.i} type="button" className={`lesson-choice${selected === v.i ? " lesson-choice-selected" : ""}`} aria-pressed={selected === v.i} onClick={() => setSelected(v.i)}>
                    {v.value}
                  </button>
                ))}
            </div>
          )}
          <p className={filled ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
            {filled ? fr.lessons.barDone : miss ? fr.lessons.barHint : fr.lessons.barAsk}
          </p>
        </>
      )}
    </div>
  );
}
