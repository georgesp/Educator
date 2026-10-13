import { motion } from "motion/react";
import type { SpellingCorrection } from "../../api/client";
import { fr } from "../../i18n/fr";

/** The expected word, rebuilt from the correction (every expected letter, in order). */
export function expectedWord(correction: SpellingCorrection): string {
  return correction.letters.map((l) => l.expected ?? "").join("");
}

/**
 * The child's word letter by letter (SUB001405): right letters in green, a wrong letter in orange with
 * the right one above, a forgotten letter as a gap showing the missing letter, a letter too many
 * crossed out; then the word well written.
 */
export function WordCorrection({ correction }: { correction: SpellingCorrection }) {
  const word = expectedWord(correction);
  return (
    <div className="word-correction">
      <p className="word-correction-label">{fr.spelling.yourWord}</p>
      <p className="word-letters" aria-label={fr.spelling.yourWordAria(correction.letters.map((l) => l.given ?? "").join(""))}>
        {correction.letters.map((letter, i) => (
          <motion.span
            key={i}
            className={`word-letter word-letter-${letter.kind.toLowerCase()}`}
            initial={{ opacity: 0, y: 6 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: i * 0.04 }}
            title={fr.spelling.letterMark(letter.kind, letter.given, letter.expected)}
          >
            {letter.kind === "Wrong" && <span className="word-letter-fix">{letter.expected === " " ? "␣" : letter.expected}</span>}
            {letter.kind === "Missing" ? letter.expected : letter.given === " " ? " " : letter.given}
          </motion.span>
        ))}
      </p>
      {!correction.correct && (
        <p className="word-right">
          <span className="word-correction-label">{fr.spelling.rightWord}</span>
          <span className="word-right-text">{word}</span>
        </p>
      )}
    </div>
  );
}
