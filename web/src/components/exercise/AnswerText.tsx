/** A fraction written « a/b » drawn with the numerator above the denominator; anything else as is. */
export function AnswerText({ text }: { text: string }) {
  const fraction = /^(\d+)\/(\d+)$/.exec(text);
  if (fraction) {
    return <FractionText numerator={Number(fraction[1])} denominator={Number(fraction[2])} />;
  }

  return <>{text}</>;
}

/** Numerator above the denominator, with the fraction bar. */
export function FractionText({ numerator, denominator }: { numerator: number; denominator: number }) {
  return (
    <span className="fraction" aria-label={`${numerator}/${denominator}`}>
      <span className="fraction-top" aria-hidden="true">
        {numerator}
      </span>
      <span className="fraction-bottom" aria-hidden="true">
        {denominator}
      </span>
    </span>
  );
}
