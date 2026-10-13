import { AnimatePresence, motion } from "motion/react";
import { useState } from "react";
import type { Equality, EquationTerm, LessonOperator } from "../../../api/client";
import { fr } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";

type EquationProps = {
  left: number;
  operator: LessonOperator;
  right: number;
  result: number;
  missing: EquationTerm | null;
  missingValue: number | null;
  /** The equality first, then the rest of its family. */
  equalities: Equality[];
  /** The child picks the missing term among `choices`. */
  interactive: boolean;
  choices: number[];
  onComplete?(): void;
};

const text = (e: Equality) => `${e.left} ${fr.lessons.sign[e.operator]} ${e.right} = ${e.result}`;

/**
 * `equation`: an equality that builds up, maybe with a box to find (« 7 + … = 15 »), then the other
 * equalities of its family (8 + 7 = 15, 15 − 7 = 8, 15 − 8 = 7).
 */
export function Equation({ left, operator, right, result, missing, missingValue, equalities, interactive, choices, onComplete }: EquationProps) {
  const [found, setFound] = useState(false);
  const [misses, setMisses] = useState(0);
  const move = useMove();

  // Animation: the box opens, then the family appears line after line. Interactive: the family
  // appears once the box is found.
  const boxSteps = missing && !interactive ? 1 : 0;
  const family = equalities.length - 1;
  const ready = !interactive || found;
  const shown = useSteps(ready ? boxSteps + family : 0, ready);
  const boxOpen = interactive ? found : missing === null || shown >= 1;
  const familyShown = Math.max(0, shown - boxSteps);

  const pick = (value: number) => {
    if (found) {
      return;
    }

    if (value === missingValue) {
      setFound(true);
      setMisses(0);
      onComplete?.();
    } else {
      setMisses((m) => m + 1);
    }
  };

  const term = (which: EquationTerm, value: number) =>
    missing === which ? (
      <span className={boxOpen ? "equation-box equation-box-open" : "equation-box"} aria-label={boxOpen ? String(value) : fr.lessons.box}>
        {boxOpen ? (
          <motion.span initial={{ opacity: 0, scale: 0.6 }} animate={{ opacity: 1, scale: 1 }} transition={move}>
            {value}
          </motion.span>
        ) : (
          "?"
        )}
      </span>
    ) : (
      <span className="equation-term">{value}</span>
    );

  return (
    <div className="lesson-equation">
      <p className="equation-line" aria-live="polite">
        {term("Left", left)}
        <span className="equation-sign">{fr.lessons.sign[operator]}</span>
        {term("Right", right)}
        <span className="equation-sign">=</span>
        {term("Result", result)}
      </p>

      {interactive && (
        <>
          <div className="lesson-choices" role="group" aria-label={fr.lessons.equationAsk}>
            {choices.map((choice) => (
              <button
                key={choice}
                type="button"
                className={`lesson-choice${found && choice === missingValue ? " lesson-choice-right" : ""}`}
                disabled={found}
                onClick={() => pick(choice)}
              >
                {choice}
              </button>
            ))}
          </div>
          <p className={found ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
            {found
              ? fr.lessons.equationDone(text(equalities[0]))
              : misses > 0 && missing
                ? fr.lessons.equationHint(operator, missing, left, right, result)
                : fr.lessons.equationAsk}
          </p>
        </>
      )}

      {family > 0 && (
        <div className="equation-family" aria-label={fr.lessons.family}>
          <AnimatePresence>
            {equalities.slice(1, 1 + familyShown).map((e) => (
              <motion.span key={text(e)} className="equation-family-line" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} transition={move}>
                {text(e)}
              </motion.span>
            ))}
          </AnimatePresence>
        </div>
      )}
    </div>
  );
}
