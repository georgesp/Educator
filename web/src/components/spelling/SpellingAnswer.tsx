import { useEffect, useRef, useState, type FormEvent } from "react";
import { answerInputMode } from "../../app/device";
import { fr } from "../../i18n/fr";
import { LetterKeyboard } from "./LetterKeyboard";

type SpellingAnswerProps = {
  /** Missing letters: the letters around the blank; a whole word otherwise. */
  before: string | null;
  after: string | null;
  /** Missing letters: how many, shown as dashes (« bat___ »). */
  missing?: number | null;
  /** A noun: its article, shown before the box and not written (« un [______] »). */
  article?: string | null;
  maxLength: number;
  disabled: boolean;
  onSubmit(text: string): void;
  /** After the answer: what was written, shown in place of the field. */
  given?: string | null;
  correct?: boolean | null;
};

/**
 * Writing a word or the missing letters (SUB001406, SUB001407): the physical keyboard or the
 * on-screen keyboard with the accented letters. Enter or « Valider » sends the text.
 */
export function SpellingAnswer({
  before,
  after,
  missing = null,
  article = null,
  maxLength,
  disabled,
  onSubmit,
  given = null,
  correct = null,
}: SpellingAnswerProps) {
  const [value, setValue] = useState("");
  const input = useRef<HTMLInputElement | null>(null);
  const blank = before !== null || after !== null;
  const answered = given !== null;

  useEffect(() => {
    if (!answered) {
      input.current?.focus();
    }
  }, [answered]);

  const submit = () => {
    if (value.trim() !== "" && !disabled) {
      onSubmit(value);
    }
  };
  const onForm = (event: FormEvent) => {
    event.preventDefault();
    submit();
  };

  const field = answered ? (
    <span className={`spelling-given ${correct ? "slot-right" : "slot-wrong"}`}>{given === "" ? " " : given}</span>
  ) : (
    <input
      ref={input}
      className={blank ? "spelling-input spelling-input-blank" : "spelling-input"}
      aria-label={blank ? (missing !== null ? fr.spelling.missingCount(missing) : fr.spelling.missingLabel) : fr.spelling.wordLabel}
      placeholder={blank && missing !== null ? "_".repeat(missing) : undefined}
      autoComplete="off"
      autoCapitalize="off"
      autoCorrect="off"
      spellCheck={false}
      inputMode={answerInputMode("text")}
      enterKeyHint="done"
      lang="fr"
      maxLength={maxLength}
      value={value}
      size={blank ? Math.max(missing ?? 3, 2) + 1 : 12}
      onChange={(e) => setValue(e.target.value.slice(0, maxLength))}
      disabled={disabled}
    />
  );

  return (
    <form className="spelling-answer" onSubmit={onForm}>
      <p className={blank ? "spelling-word spelling-word-blank" : "spelling-word"}>
        {article && (
          <span className="spelling-article" title={fr.spelling.articleLabel(article)}>
            {article}
          </span>
        )}
        {before && <span className="spelling-around">{before}</span>}
        {field}
        {after && <span className="spelling-around">{after}</span>}
      </p>
      <button type="submit" hidden tabIndex={-1} aria-hidden="true" />
      {!answered && (
        <LetterKeyboard value={value} onChange={setValue} onSubmit={submit} maxLength={maxLength} disabled={disabled} withSpace={!blank} />
      )}
    </form>
  );
}
