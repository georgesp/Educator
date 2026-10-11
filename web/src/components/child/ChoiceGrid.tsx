type ChoiceGridProps = {
  choices: readonly number[];
  onPick(value: number): void;
  /** After the answer: the picked value and the right one, to color the tiles. */
  picked?: number | null;
  expected?: number | null;
  disabled?: boolean;
  label: string;
};

/** Two to four answers in large tiles. */
export function ChoiceGrid({ choices, onPick, picked = null, expected = null, disabled = false, label }: ChoiceGridProps) {
  return (
    <div className="choice-grid" role="group" aria-label={label}>
      {choices.map((choice) => {
        const state = expected === null ? "" : choice === expected ? "choice-right" : choice === picked ? "choice-wrong" : "choice-faded";
        return (
          <button
            key={choice}
            type="button"
            className={`choice-tile ${state}`}
            disabled={disabled}
            aria-pressed={picked === choice}
            onClick={() => onPick(choice)}
          >
            {choice}
          </button>
        );
      })}
    </div>
  );
}
