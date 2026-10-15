import { AnswerText } from "../exercise/AnswerText";

type ChoiceGridProps = {
  /** Text of each tile: a number, a fraction (« 3/4 »), words. */
  labels: readonly string[];
  onPick(index: number): void;
  /** After the answer: the picked tile and the right one, to color the tiles. */
  picked?: number | null;
  expected?: number | null;
  disabled?: boolean;
  label: string;
};

/** Two to four answers in large tiles. */
export function ChoiceGrid({ labels, onPick, picked = null, expected = null, disabled = false, label }: ChoiceGridProps) {
  // Words take the word layout as soon as they are longer than « oui » / « non »: in the large number
  // tiles, a 7-letter word (« souvent ») no longer fits.
  const long = labels.some((l) => (/\p{L}/u.test(l) && l.length > 3) || l.length > 8);
  return (
    <div className={long ? "choice-grid choice-grid-words" : "choice-grid"} role="group" aria-label={label}>
      {labels.map((text, index) => {
        const state = expected === null ? "" : index === expected ? "choice-right" : index === picked ? "choice-wrong" : "choice-faded";
        return (
          <button
            key={`${index}-${text}`}
            type="button"
            className={`choice-tile ${state}`}
            disabled={disabled}
            aria-pressed={picked === index}
            aria-label={text}
            onClick={() => onPick(index)}
          >
            <AnswerText text={text} />
          </button>
        );
      })}
    </div>
  );
}
