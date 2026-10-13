import { motion } from "motion/react";
import { useState } from "react";
import type { ComparisonSign, RankDigits, RankName } from "../../../api/client";
import { fr } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";

type CompareNumbersProps = {
  a: number;
  b: number;
  sign: ComparisonSign;
  /** From the highest rank (on the left) to the units. */
  ranks: RankDigits[];
  decisiveRank: RankName | null;
  /** The child picks the sign. */
  interactive: boolean;
  onComplete?(): void;
};

const SIGNS: ComparisonSign[] = ["<", ">", "="];

/**
 * `compareNumbers`: two numbers face to face, compared rank after rank from the left; the rank that
 * decides goes racing red, then the sign appears. Interactive: the child picks the sign.
 */
export function CompareNumbers({ a, b, sign, ranks, decisiveRank, interactive, onComplete }: CompareNumbersProps) {
  const move = useMove();
  const decisive = decisiveRank === null ? ranks.length - 1 : ranks.findIndex((r) => r.rank === decisiveRank);
  // Animation: the ranks are checked one by one up to the decisive one, then the sign appears.
  const shown = useSteps(interactive ? 0 : decisive + 2, `${a}-${b}`);
  const [found, setFound] = useState(false);
  const [misses, setMisses] = useState(0);

  const checked = interactive ? (found ? decisive + 1 : 0) : Math.min(shown, decisive + 1);
  const signShown = interactive ? found : shown > decisive + 1;
  const text = fr.lessons.compare(a, sign, b);

  const pick = (picked: ComparisonSign) => {
    if (found) {
      return;
    }

    if (picked === sign) {
      setFound(true);
      onComplete?.();
    } else {
      setMisses((m) => m + 1);
    }
  };

  const digits = (which: "a" | "b") =>
    ranks.map((r, i) => {
      const digit = r[which];
      const cls = i < checked ? (i === decisive && decisiveRank !== null ? "compare-digit compare-digit-key" : "compare-digit compare-digit-checked") : "compare-digit";
      return (
        <span key={r.rank} className={cls}>
          {digit ?? ""}
        </span>
      );
    });

  return (
    <div className="lesson-compare">
      <div className="compare-line" role="img" aria-label={signShown ? text : `${a} ? ${b}`}>
        <div className="compare-number">
          <div className="compare-letters" aria-hidden="true">
            {ranks.map((r) => (
              <span key={r.rank}>{fr.lessons.rankLetters[r.rank]}</span>
            ))}
          </div>
          <div className="compare-digits">{digits("a")}</div>
        </div>
        <span className={signShown ? "compare-sign compare-sign-shown" : "compare-sign"}>
          {signShown ? (
            <motion.span initial={{ opacity: 0, scale: 0.5 }} animate={{ opacity: 1, scale: 1 }} transition={move}>
              {sign}
            </motion.span>
          ) : (
            "?"
          )}
        </span>
        <div className="compare-number">
          <div className="compare-letters" aria-hidden="true">
            {ranks.map((r) => (
              <span key={r.rank}>{fr.lessons.rankLetters[r.rank]}</span>
            ))}
          </div>
          <div className="compare-digits">{digits("b")}</div>
        </div>
      </div>

      {signShown && !interactive && (
        <p className="lesson-blocks-caption">
          {decisiveRank === null ? fr.lessons.compareSame : fr.lessons.compareRank(fr.lessons.rankNames[decisiveRank])} — {a} {fr.series.signs[sign]} {b}
        </p>
      )}

      {interactive && (
        <>
          <div className="lesson-choices" role="group" aria-label={fr.lessons.compareAsk}>
            {SIGNS.map((s) => (
              <button key={s} type="button" className={`lesson-choice${found && s === sign ? " lesson-choice-right" : ""}`} disabled={found} onClick={() => pick(s)} aria-label={fr.series.signs[s]}>
                {s}
              </button>
            ))}
          </div>
          <p className={found ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
            {found
              ? fr.lessons.compareDone(text)
              : misses > 0
                ? fr.lessons.compareHint(fr.lessons.rankNames[ranks[0].rank])
                : fr.lessons.compareAsk}
          </p>
        </>
      )}
    </div>
  );
}
