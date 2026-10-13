import { motion } from "motion/react";
import { useState } from "react";
import type { LessonOperator, WrittenColumn, WrittenOperation } from "../../../api/client";
import { fr } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";

type Operator = LessonOperator;
type Layout = WrittenOperation;
type Column = WrittenColumn;

type ColumnOperationProps = {
  operator: Operator;
  operands: number[];
  result: number;
  layouts: Layout[];
  /** Columns computed by the animation, from the units. */
  computedColumns: number;
  /** The child writes the digit of each column of the result. */
  interactive: boolean;
  onComplete?(): void;
};

/**
 * `columnOperation`: a written addition or subtraction, computed column after column from the units.
 * Carries, broken tens (cassage) and added tens (compensation) are in racing red. Everything shown
 * (digits, carries, values of the columns) is computed by the core.
 */
export function ColumnOperation({ operator, operands, result, layouts, computedColumns, interactive, onComplete }: ColumnOperationProps) {
  const shown = useSteps(interactive ? 0 : computedColumns, layouts);
  const [found, setFound] = useState(0);
  const [misses, setMisses] = useState(0);
  const sign = fr.lessons.sign[operator];
  const operation = fr.lessons.operation(operands, sign, result);

  const columns = layouts[0].columns;
  const asked = columns.filter((c) => c.written).length;
  const finished = interactive && found === asked;
  const current = interactive && !finished ? columns[found] : null;

  const pick = (digit: number) => {
    if (!current) {
      return;
    }

    if (digit === current.digit) {
      const next = found + 1;
      setFound(next);
      setMisses(0);
      if (next === asked) {
        onComplete?.();
      }
    } else {
      setMisses((m) => m + 1);
    }
  };

  // The column of the caption: the one being computed (interactive), or the last computed one.
  const last = interactive ? null : shown > 0 ? columns[shown - 1] : null;
  const calc = (column: Column) => fr.lessons.columnCalc(column.terms, sign, column.value);
  const carryOut = (index: number) => (operator === "Add" && index < columns.length - 1 ? Math.floor(columns[index].value / 10) : 0);

  return (
    <div className={layouts.length > 1 ? "lesson-written written-compact" : "lesson-written"}>
      <div className="lesson-written-layouts">
        {layouts.map((layout, i) => (
          <figure key={i} className="lesson-written-figure">
            {layouts.length > 1 && layout.method && <figcaption className="lesson-written-method">{fr.lessons.methods[layout.method]}</figcaption>}
            <WrittenGrid
              operator={operator}
              layout={layout}
              done={interactive ? found : shown}
              interactive={interactive}
              label={fr.lessons.written(operation)}
            />
          </figure>
        ))}
      </div>

      {interactive ? (
        <>
          <div className="lesson-digits" role="group" aria-label={fr.lessons.digits}>
            {Array.from({ length: 10 }, (_, d) => (
              <button
                key={d}
                type="button"
                className="lesson-digit"
                aria-label={fr.lessons.digit(d)}
                disabled={finished}
                onClick={() => pick(d)}
              >
                {d}
              </button>
            ))}
          </div>
          <p className={finished ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
            {finished
              ? fr.lessons.columnDone(operation)
              : current && misses > 0
                ? layouts.length > 1
                  ? // Two techniques: each one computes the column its own way, same digit.
                    fr.lessons.columnHintMethods(
                      layouts.map((l) => `${l.method ? fr.lessons.withMethod[l.method] : ""}, ${calc(l.columns[found])}`),
                      current.digit,
                    )
                  : fr.lessons.columnHint(calc(current), current.digit, carryOut(found))
                : fr.lessons.columnAsk(fr.lessons.columnNames[found])}
          </p>
        </>
      ) : (
        <p className="lesson-hint" aria-live="polite">
          {/* Two techniques side by side do not compute the same columns: no caption then. */}
          {last && layouts.length === 1 ? calc(last) : null}
        </p>
      )}
    </div>
  );
}

type WrittenGridProps = {
  operator: Operator;
  layout: Layout;
  /** Columns computed so far (interactive: answered so far). */
  done: number;
  interactive: boolean;
  label: string;
};

/** The operation in a grid: letters (c d u), marks, the numbers, the rule, the result. */
function WrittenGrid({ operator, layout, done, interactive, label }: WrittenGridProps) {
  const move = useMove();
  const columns = layout.columns;
  const width = columns.length;
  const rows = columns[0].digits.length;
  // From the left: the highest column first.
  const order = Array.from({ length: width }, (_, i) => width - 1 - i);
  // A mark written while computing column k: shown once that column is reached (interactive, as a
  // guide) or computed (animation).
  const markShown = (k: number) => (interactive ? done >= k : done > k);
  const active = interactive && done < width ? done : null;

  const cells = [];
  // Letters of the columns.
  cells.push(<span key="letters" className="written-cell" />);
  for (const c of order) {
    cells.push(
      <span key={`l${c}`} className="written-cell written-letter">
        {fr.lessons.columnLetters[c]}
      </span>,
    );
  }

  // Marks above the top number: carries of an addition, rewritten digits of a cassage.
  cells.push(<span key="marks" className="written-cell" />);
  for (const c of order) {
    const column = columns[c];
    const rewrite = [...column.rewrites].reverse().find((r) => markShown(r.atColumn));
    const carry = operator === "Add" && column.carry > 0 && done >= c;
    cells.push(
      <span key={`m${c}`} className="written-cell written-mark">
        {carry && (
          <motion.span initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={move}>
            {column.carry}
          </motion.span>
        )}
        {rewrite && (
          <motion.span key={rewrite.value} initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={move}>
            {rewrite.value}
          </motion.span>
        )}
      </span>,
    );
  }

  // The numbers.
  for (let row = 0; row < rows; row++) {
    cells.push(
      <span key={`s${row}`} className="written-cell written-sign">
        {row > 0 ? fr.lessons.sign[operator] : ""}
      </span>,
    );
    for (const c of order) {
      const column = columns[c];
      const digit = column.digits[row];
      const crossed = row === 0 && column.rewrites.some((r) => markShown(r.atColumn));
      const plusTen = row === 0 && column.plusTen && markShown(c);
      // Compensation: the ten added below comes from the column on the right.
      const addedTen = row === 1 && layout.method === "Compensation" && column.carry > 0 && markShown(c - 1);
      cells.push(
        <span
          key={`d${row}-${c}`}
          className={`written-cell written-digit${c === active ? " written-active" : ""}${crossed ? " written-crossed" : ""}`}
        >
          {plusTen && <span className="written-small">1</span>}
          {addedTen && <span className="written-small written-small-below">+1</span>}
          <span className="written-value">{digit ?? ""}</span>
        </span>,
      );
    }
  }

  // The rule, then the result.
  cells.push(<span key="rule" className="written-rule" style={{ gridColumn: `1 / span ${width + 1}` }} />);
  cells.push(<span key="r" className="written-cell" />);
  for (const c of order) {
    const column = columns[c];
    cells.push(
      <span key={`r${c}`} className={`written-cell written-result${c === active ? " written-active" : ""}`}>
        {c < done && column.written && (
          <motion.span initial={{ opacity: 0, scale: 0.6 }} animate={{ opacity: 1, scale: 1 }} transition={move}>
            {column.digit}
          </motion.span>
        )}
      </span>,
    );
  }

  return (
    <div className="written-grid" role="img" aria-label={label} style={{ gridTemplateColumns: `repeat(${width + 1}, auto)` }}>
      {cells}
    </div>
  );
}
