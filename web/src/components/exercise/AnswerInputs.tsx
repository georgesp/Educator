import { Reorder } from "motion/react";
import { useState } from "react";
import type { ComparisonSign } from "../../api/client";
import { fr } from "../../i18n/fr";

const SIGNS: ComparisonSign[] = ["<", "=", ">"];

type SignButtonsProps = {
  onPick(sign: ComparisonSign): void;
  disabled: boolean;
  /** After the answer. */
  picked: ComparisonSign | null;
  expected: ComparisonSign | null;
};

/** Three big buttons `<`, `=`, `>` (SUB000118). */
export function SignButtons({ onPick, disabled, picked, expected }: SignButtonsProps) {
  return (
    <div className="sign-buttons" role="group" aria-label={fr.series.comparison}>
      {SIGNS.map((sign) => {
        const state = expected === null ? "" : sign === expected ? "choice-right" : sign === picked ? "choice-wrong" : "choice-faded";
        return (
          <button key={sign} type="button" className={`choice-tile sign-tile ${state}`} disabled={disabled} aria-pressed={picked === sign} aria-label={fr.series.signs[sign]} onClick={() => onPick(sign)}>
            {sign}
          </button>
        );
      })}
    </div>
  );
}

type YesNoButtonsProps = {
  onPick(value: boolean): void;
  disabled: boolean;
  picked: boolean | null;
  expected: boolean | null;
};

/** Two big buttons Oui / Non (SUB000122). */
export function YesNoButtons({ onPick, disabled, picked, expected }: YesNoButtonsProps) {
  return (
    <div className="yes-no" role="group" aria-label={fr.series.yesNo}>
      {[true, false].map((value) => {
        const state = expected === null ? "" : value === expected ? "choice-right" : value === picked ? "choice-wrong" : "choice-faded";
        return (
          <button key={String(value)} type="button" className={`choice-tile yes-no-tile ${state}`} disabled={disabled} aria-pressed={picked === value} onClick={() => onPick(value)}>
            {value ? fr.series.yes : fr.series.no}
          </button>
        );
      })}
    </div>
  );
}

type OrderInputProps = {
  items: readonly number[];
  ascending: boolean;
  onSubmit(order: number[]): void;
  disabled: boolean;
  /** After the answer: whether the order was right. */
  correct: boolean | null;
};

/** Numbers to put in order: drag and drop, or the small arrows (SUB000118, SUB001606). */
export function OrderInput({ items, ascending, onSubmit, disabled, correct }: OrderInputProps) {
  const [order, setOrder] = useState<number[]>(() => [...items]);
  const move = (index: number, delta: number) => {
    const target = index + delta;
    if (target < 0 || target >= order.length) {
      return;
    }
    const next = [...order];
    [next[index], next[target]] = [next[target], next[index]];
    setOrder(next);
  };

  return (
    <div className="order-input">
      <p className="order-hint">
        {fr.series.order} · {ascending ? fr.series.orderAscending : fr.series.orderDescending}
      </p>
      <Reorder.Group axis="x" values={order} onReorder={disabled ? () => undefined : setOrder} className={`order-list ${correct === null ? "" : correct ? "order-right" : "order-wrong"}`} aria-label={fr.series.order}>
        {order.map((n, index) => (
          <Reorder.Item key={n} value={n} className="order-item" dragListener={!disabled}>
            <span className="order-number">{n}</span>
            <span className="order-arrows">
              <button type="button" className="order-arrow" disabled={disabled || index === 0} aria-label={fr.series.moveLeft(n)} onClick={() => move(index, -1)}>
                ‹
              </button>
              <button type="button" className="order-arrow" disabled={disabled || index === order.length - 1} aria-label={fr.series.moveRight(n)} onClick={() => move(index, 1)}>
                ›
              </button>
            </span>
          </Reorder.Item>
        ))}
      </Reorder.Group>
      {!disabled && (
        <button type="button" className="button button-primary" onClick={() => onSubmit(order)}>
          {fr.series.check}
        </button>
      )}
    </div>
  );
}

type MultiChoiceInputProps = {
  items: readonly number[];
  onSubmit(picked: number[]): void;
  disabled: boolean;
  /** After the answer: the right numbers. */
  expected: readonly number[] | null;
};

/** Every right number to pick, then « Valider » (SUB000122). */
export function MultiChoiceInput({ items, onSubmit, disabled, expected }: MultiChoiceInputProps) {
  const [picked, setPicked] = useState<number[]>([]);
  const toggle = (n: number) => setPicked((current) => (current.includes(n) ? current.filter((x) => x !== n) : [...current, n]));
  return (
    <div className="multi-choice">
      <div className="choice-grid choice-grid-many" role="group" aria-label={fr.series.multiChoice}>
        {items.map((n) => {
          const chosen = picked.includes(n);
          const state =
            expected === null ? (chosen ? "choice-chosen" : "") : expected.includes(n) ? (chosen ? "choice-right" : "choice-missed") : chosen ? "choice-wrong" : "choice-faded";
          return (
            <button key={n} type="button" className={`choice-tile ${state}`} disabled={disabled} aria-pressed={chosen} onClick={() => toggle(n)}>
              {n}
            </button>
          );
        })}
      </div>
      {!disabled && (
        <button type="button" className="button button-primary" disabled={picked.length === 0} onClick={() => onSubmit(picked)}>
          {fr.series.check}
        </button>
      )}
    </div>
  );
}
