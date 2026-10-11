import { fr } from "../../i18n/fr";
import { EraseIcon } from "../icons";

type NumberPadProps = {
  value: string;
  onChange(value: string): void;
  onSubmit(): void;
  maxLength?: number;
  disabled?: boolean;
  submitLabel?: string;
};

const DIGITS = ["7", "8", "9", "4", "5", "6", "1", "2", "3"];

/** On-screen keypad 0–9 with Erase and Validate, for children who do not use the keyboard yet. */
export function NumberPad({ value, onChange, onSubmit, maxLength = 10, disabled = false, submitLabel = fr.series.check }: NumberPadProps) {
  const type = (digit: string) => {
    if (value.length < maxLength) {
      onChange(value + digit);
    }
  };

  return (
    <div className="number-pad" role="group" aria-label={fr.pad.keypad}>
      {DIGITS.map((digit) => (
        <button key={digit} type="button" className="pad-key" disabled={disabled} onClick={() => type(digit)}>
          {digit}
        </button>
      ))}
      <button type="button" className="pad-key pad-erase" disabled={disabled || value === ""} onClick={() => onChange(value.slice(0, -1))} aria-label={fr.pad.erase}>
        <EraseIcon />
      </button>
      <button type="button" className="pad-key" disabled={disabled} onClick={() => type("0")}>
        0
      </button>
      <button type="button" className="pad-key pad-submit" disabled={disabled || value === ""} onClick={onSubmit}>
        {submitLabel}
      </button>
    </div>
  );
}
