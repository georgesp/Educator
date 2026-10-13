import { motion } from "motion/react";
import { useState } from "react";
import type { SortedWord as SortedWordResponse } from "../../../api/client";
import { fr } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";

type SoundSortProps = {
  groups: string[];
  words: SortedWordResponse[];
  interactive: boolean;
  onComplete?(): void;
};

/** The word with the letters of its grapheme colored. */
function ColoredWord({ word }: { word: SortedWordResponse }) {
  const letters = Array.from(word.word);
  return (
    <>
      {letters.slice(0, word.start).join("")}
      <span className="sound-grapheme">{letters.slice(word.start, word.start + word.length).join("")}</span>
      {letters.slice(word.start + word.length).join("")}
    </>
  );
}

/**
 * `soundSort` (SUB001410): words to put in the box of the grapheme that writes their sound. Shown
 * only, the words go to their box one after the other. Interactive: the child clicks a word, then
 * its box; the grapheme is colored once the word is in its box.
 */
export function SoundSort({ groups, words, interactive, onComplete }: SoundSortProps) {
  const move = useMove();
  const shown = useSteps(interactive ? 0 : words.length, words);
  const [sorted, setSorted] = useState<number[]>([]);
  const [selected, setSelected] = useState<number | null>(null);
  const [hint, setHint] = useState<string | null>(null);
  const inBox = (i: number) => (interactive ? sorted.includes(i) : i < shown);
  const done = interactive && sorted.length === words.length;

  const drop = (group: number) => {
    if (selected === null || done) {
      return;
    }

    if (words[selected].group === group) {
      const next = [...sorted, selected];
      setSorted(next);
      setSelected(null);
      setHint(null);
      if (next.length === words.length) {
        onComplete?.();
      }
    } else {
      setHint(fr.lessons.sortSoundHint(words[selected].word));
    }
  };

  return (
    <div className="sound-sort">
      {interactive && (
        <div className="sound-sort-words" role="group" aria-label={fr.lessons.sortWords}>
          {words.map((word, i) =>
            sorted.includes(i) ? null : (
              <button
                key={word.word}
                type="button"
                className={selected === i ? "sound-word sound-word-selected" : "sound-word"}
                aria-pressed={selected === i}
                onClick={() => {
                  setSelected(i);
                  setHint(null);
                }}
              >
                {word.word}
              </button>
            ),
          )}
        </div>
      )}
      <div className="sound-sort-boxes">
        {groups.map((group, g) => {
          const content = words.map((w, i) => ({ w, i })).filter(({ w, i }) => w.group === g && inBox(i));
          const box = (
            <>
              <span className="sound-box-label">{group}</span>
              <span className="sound-box-words">
                {content.map(({ w }) => (
                  <motion.span key={w.word} className="sound-word sound-word-sorted" initial={{ opacity: 0, y: -16 }} animate={{ opacity: 1, y: 0 }} transition={move}>
                    <ColoredWord word={w} />
                  </motion.span>
                ))}
              </span>
            </>
          );
          return interactive ? (
            <button key={group} type="button" className="sound-box" aria-label={fr.lessons.sortBox(group)} disabled={selected === null || done} onClick={() => drop(g)}>
              {box}
            </button>
          ) : (
            <div key={group} className="sound-box" role="group" aria-label={fr.lessons.sortBox(group)}>
              {box}
            </div>
          );
        })}
      </div>
      {interactive && (
        <p className={done ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
          {done ? fr.lessons.sortSoundDone : (hint ?? (selected === null ? fr.lessons.sortSoundAsk : fr.lessons.sortSoundWhich(words[selected].word)))}
        </p>
      )}
    </div>
  );
}
