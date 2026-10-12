import type { ReactNode } from "react";
import type { Token } from "../../api/client";
import { FractionText } from "./AnswerText";

type StatementProps = {
  tokens: readonly Token[];
  /** What to draw at each answer slot (an input, the given answer…). */
  renderSlot(slot: number): ReactNode;
  /** Written form with `?`, read by screen readers. */
  label: string;
};

/** Size of the statement: the longer it is, the smaller the characters. */
function sizeClass(tokens: readonly Token[]): string {
  const length = tokens.reduce((total, t) => {
    switch (t.kind) {
      case "number":
        return total + String(t.value).length + 1;
      case "text":
        return total + t.value.length;
      case "fraction":
        return total + 2;
      default:
        return total + 2;
    }
  }, 0);
  return length > 26 ? "equation equation-small" : length > 15 ? "equation equation-medium" : "equation";
}

/** The written statement: numbers, signs, words, fractions and the answer slots. */
export function Statement({ tokens, renderSlot, label }: StatementProps) {
  if (tokens.length === 0) {
    return null;
  }

  return (
    <div className={sizeClass(tokens)} aria-label={label}>
      {tokens.map((token, i) => {
        switch (token.kind) {
          case "number":
            return <span key={i}>{token.value}</span>;
          case "sign":
            return (
              <span key={i} className="equation-sign">
                {token.value}
              </span>
            );
          case "text":
            return (
              <span key={i} className="equation-text">
                {token.value}
              </span>
            );
          case "fraction":
            return <FractionText key={i} numerator={token.numerator} denominator={token.denominator} />;
          case "slot":
            return <span key={i}>{renderSlot(token.slot)}</span>;
        }
      })}
    </div>
  );
}
