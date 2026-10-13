import { fr } from "../../i18n/fr";
import { EraseIcon } from "../icons";

/** Rows of the on-screen keyboard: the letters in the order of a French keyboard (AZERTY). */
export const LETTER_ROWS: readonly (readonly string[])[] = [
  ["a", "z", "e", "r", "t", "y", "u", "i", "o", "p"],
  ["q", "s", "d", "f", "g", "h", "j", "k", "l", "m"],
  ["w", "x", "c", "v", "b", "n"],
];

/** The accented letters, in a row of their own so that they are easy to find (SUB001406). */
export const ACCENTED_LETTERS: readonly string[] = ["é", "è", "ê", "à", "â", "ù", "û", "ç", "ë", "ï", "î", "ô", "œ"];

type LetterKeyboardProps = {
  value: string;
  onChange(value: string): void;
  onSubmit(): void;
  maxLength: number;
  disabled?: boolean;
  /** A space and an apostrophe, for the groups of words (« des chats »). */
  withSpace?: boolean;
};

/**
 * On-screen keyboard for the spelling exercises, usable with the mouse alone: the letters, the
 * accented letters well apart, Erase and Validate. Keys are in DOM order (tab order follows the rows).
 */
export function LetterKeyboard({ value, onChange, onSubmit, maxLength, disabled = false, withSpace = false }: LetterKeyboardProps) {
  const type = (letter: string) => {
    if ([...value].length < maxLength) {
      onChange(value + letter);
    }
  };
  const key = (letter: string, extra = "") => (
    <button key={letter} type="button" className={`letter-key ${extra}`.trim()} disabled={disabled} onClick={() => type(letter)} aria-label={fr.spelling.letter(letter)}>
      {letter}
    </button>
  );

  return (
    <div className="letter-keyboard" role="group" aria-label={fr.spelling.keyboard}>
      {LETTER_ROWS.map((row, i) => (
        <div key={i} className="letter-row">
          {row.map((letter) => key(letter))}
        </div>
      ))}
      <div className="letter-row letter-row-accents" role="group" aria-label={fr.spelling.accents}>
        {ACCENTED_LETTERS.map((letter) => key(letter, "letter-key-accent"))}
      </div>
      <div className="letter-row letter-row-actions">
        {withSpace && (
          <>
            <button type="button" className="letter-key letter-key-wide" disabled={disabled} onClick={() => type(" ")}>
              {fr.spelling.space}
            </button>
            {key("'")}
            {key("-")}
          </>
        )}
        <button type="button" className="letter-key letter-key-erase" disabled={disabled || value === ""} onClick={() => onChange([...value].slice(0, -1).join(""))} aria-label={fr.pad.erase}>
          <EraseIcon size={26} />
          <span aria-hidden="true">{fr.pad.erase}</span>
        </button>
        <button type="button" className="letter-key letter-key-submit" disabled={disabled || value.trim() === ""} onClick={onSubmit}>
          {fr.series.check}
        </button>
      </div>
    </div>
  );
}
