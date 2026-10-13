import { motion } from "motion/react";
import { useState } from "react";
import type { RankCount, RankName, Spellings } from "../../../api/client";
import { fr } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";

type PlaceValueTableProps = {
  value: number;
  ranks: RankName[];
  digits: (number | null)[];
  focus: RankName | null;
  countOf: RankCount | null;
  decomposition: number[];
  words: Spellings | null;
  interactive: boolean;
  onComplete?(): void;
};

const DIGITS = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

/**
 * `placeValueTable`: the table of numeration. The digits take their place, the focused rank in racing
 * red, then the number of tens (347: 34 tens), the decomposition (300 + 40 + 7) and the number in
 * words (traditional spelling, then the 1990 one). Interactive: the child writes the number read by
 * the voice, digit by digit from the left.
 */
export function PlaceValueTable({ value, ranks, digits, focus, countOf, decomposition, words, interactive, onComplete }: PlaceValueTableProps) {
  const move = useMove();
  const extras = [countOf !== null, decomposition.length > 0, words !== null].filter(Boolean).length;
  const shown = useSteps(interactive ? 0 : 1 + extras, value);
  // Columns the child fills: those of the number, from the left.
  const first = digits.findIndex((d) => d !== null);
  const [filled, setFilled] = useState(0);
  const [miss, setMiss] = useState(false);
  const toFill = digits.length - first;
  const done = interactive && filled === toFill;

  const pick = (digit: number) => {
    if (done) {
      return;
    }

    if (digit === digits[first + filled]) {
      const next = filled + 1;
      setFilled(next);
      setMiss(false);
      if (next === toFill) {
        onComplete?.();
      }
    } else {
      setMiss(true);
    }
  };

  const digitShown = (i: number) => (interactive ? i >= first && i < first + filled : shown >= 1);
  const active = interactive && !done ? first + filled : -1;
  let step = 1;
  const next = () => {
    step += 1;
    return shown >= step;
  };
  const showCount = countOf !== null && next();
  const showDecomposition = decomposition.length > 0 && next();
  const showWords = words !== null && next();

  return (
    <div className="lesson-place">
      <table className="place-table" aria-label={fr.lessons.placeTable(value)}>
        <thead>
          <tr>
            {ranks.map((rank) => (
              <th key={rank} scope="col" className={rank === focus ? "place-focus" : undefined}>
                {fr.lessons.rankNames[rank]}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          <tr>
            {digits.map((digit, i) => (
              <td key={ranks[i]} className={`place-digit${ranks[i] === focus ? " place-focus" : ""}${i === active ? " place-active" : ""}`}>
                {digit !== null && digitShown(i) ? (
                  <motion.span initial={{ opacity: 0, y: -16 }} animate={{ opacity: 1, y: 0 }} transition={move}>
                    {digit}
                  </motion.span>
                ) : null}
              </td>
            ))}
          </tr>
        </tbody>
      </table>

      {showCount && countOf && <p className="place-line">{fr.lessons.countOf(fr.lessons.rankNames[countOf.rank], countOf.count)}</p>}
      {showDecomposition && <p className="place-line place-big">{`${value} = ${decomposition.join(" + ")}`}</p>}
      {showWords && words && (
        <div className="place-words">
          {words.traditional === words.rectified ? (
            <p>
              <span className="place-words-label">{fr.lessons.spellingSame}</span> {words.traditional}
            </p>
          ) : (
            <>
              <p>
                <span className="place-words-label">{fr.lessons.spellingTraditional}</span> {words.traditional}
              </p>
              <p>
                <span className="place-words-label">{fr.lessons.spellingRectified}</span> {words.rectified}
              </p>
            </>
          )}
        </div>
      )}

      {interactive && (
        <>
          <div className="lesson-digits" role="group" aria-label={done ? fr.lessons.digits : fr.lessons.placeColumnAsk(fr.lessons.rankNames[ranks[active] ?? "Units"])}>
            {DIGITS.map((d) => (
              <button key={d} type="button" className="lesson-digit" disabled={done} onClick={() => pick(d)} aria-label={fr.lessons.digit(d)}>
                {d}
              </button>
            ))}
          </div>
          <p className={done ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
            {done ? fr.lessons.placeDone(value) : miss ? fr.lessons.placeHint : fr.lessons.placeColumnAsk(fr.lessons.rankNames[ranks[active]])}
          </p>
        </>
      )}
    </div>
  );
}
