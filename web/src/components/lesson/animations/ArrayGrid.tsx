import type { GridPart, GridTask } from "../../../api/client";
import { motion } from "motion/react";
import { useState } from "react";
import { fr } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";
import { ColoredCar } from "./RaceObjects";

type ArrayGridProps = {
  rows: number;
  cols: number;
  total: number;
  split: number | null;
  parts: GridPart[];
  repeated: number[];
  rotate: boolean;
  removed: number;
  remaining: number;
  task: GridTask | null;
  onComplete?(): void;
};

const CELL_W = 58;
const CELL_H = 34;
const PAD = 8;

type CarsProps = {
  rows: number;
  cols: number;
  /** Rows drawn after this one are in the second color (the second part of a cut grid). */
  splitAt?: number | null;
  /** The last cars, drawn as leaving. */
  leaving?: number;
  /** Interactive cut: the gaps between rows are buttons. */
  onCut?(after: number): void;
  cutAt?: number | null;
  label: string;
};

/** The starting grid: `rows` rows of `cols` cars. */
function Cars({ rows, cols, splitAt = null, leaving = 0, onCut, cutAt = null, label }: CarsProps) {
  const width = cols * CELL_W + 2 * PAD;
  const gap = (row: number) => (splitAt !== null && row >= splitAt ? 16 : 0);
  const height = rows * CELL_H + 2 * PAD + (splitAt !== null ? 16 : 0);
  const total = rows * cols;
  const cut = cutAt ?? splitAt;

  return (
    <svg className="grid-svg" viewBox={`0 0 ${width + (onCut ? 40 : 0)} ${height}`} width={Math.min((width + (onCut ? 40 : 0)) * 1.15, 760)} role="img" aria-label={label}>
      <rect x={0} y={0} width={width} height={height} rx={10} className="grid-track" />
      {Array.from({ length: rows }, (_, r) =>
        Array.from({ length: cols }, (_, c) => {
          const index = r * cols + c;
          const gone = index >= total - leaving;
          return (
            <svg key={`${r}-${c}`} x={PAD + c * CELL_W + 4} y={PAD + r * CELL_H + gap(r) + 4} width={CELL_W - 8} height={CELL_H - 8} viewBox="0 0 48 24" className={gone ? "race-object-ghost" : undefined}>
              <ColoredCar color={splitAt !== null && r >= splitAt ? "blue" : "red"} />
            </svg>
          );
        }),
      )}
      {cut !== null && (
        <line x1={0} x2={width} y1={PAD + cut * CELL_H + 8} y2={PAD + cut * CELL_H + 8} className="grid-cut" />
      )}
      {onCut &&
        Array.from({ length: rows - 1 }, (_, r) => (
          <rect
            key={`cut-${r}`}
            x={0}
            y={PAD + (r + 1) * CELL_H - 8}
            width={width + 40}
            height={16}
            className="grid-cut-hit"
            role="button"
            tabIndex={0}
            aria-label={fr.lessons.gridCut(r + 1)}
            onClick={() => onCut(r + 1)}
            onKeyDown={(e) => {
              if (e.key === "Enter" || e.key === " ") {
                e.preventDefault();
                onCut(r + 1);
              }
            }}
          />
        ))}
    </svg>
  );
}

/** The products of the parts, then their sum (13 × 7 = 10 × 7 + 3 × 7 = 70 + 21 = 91). */
function SplitSum({ parts, total }: { parts: GridPart[]; total: number }) {
  return (
    <div className="grid-products">
      {parts.map((p, i) => (
        <span key={i} className={i === 0 ? "grid-product" : "grid-product grid-product-second"}>
          {fr.lessons.gridProduct(p.rows, p.cols, p.product)}
        </span>
      ))}
      <span className="grid-product grid-product-total">{fr.lessons.gridSum(parts.map((p) => p.product), total)}</span>
    </div>
  );
}

/**
 * `arrayGrid`: cars in rows and columns. The rows add up (5 + 5 + 5 = 3 × 5), the grid turns
 * (3 × 5 = 5 × 3), is cut in two (13 × 7 = 10 × 7 + 3 × 7) or cars leave. Interactive: the child
 * places the rows, or clicks where to cut.
 */
export function ArrayGrid(props: ArrayGridProps) {
  if (props.task === "Build") {
    return <GridBuild {...props} />;
  }
  if (props.task === "Split") {
    return <GridSplit {...props} />;
  }
  return <GridShow {...props} />;
}

function GridShow({ rows, cols, total, split, parts, repeated, rotate, removed, remaining }: ArrayGridProps) {
  const steps = repeated.length > 0 ? rows : split !== null ? 2 : rotate || removed > 0 ? 1 : 0;
  const shown = useSteps(steps, `${rows}-${cols}`);
  const move = useMove();

  if (repeated.length > 0) {
    const visible = Math.max(shown, 1);
    const sum = repeated.slice(0, visible);
    return (
      <div className="lesson-grid">
        <Cars rows={visible} cols={cols} label={fr.lessons.grid(visible, cols)} />
        <p className="grid-sum">{fr.lessons.gridSum(sum, sum.reduce((a, b) => a + b, 0))}</p>
        {visible === rows && (
          <motion.p className="grid-product grid-product-total" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} transition={move}>
            {fr.lessons.gridProduct(rows, cols, total)}
          </motion.p>
        )}
      </div>
    );
  }

  if (rotate) {
    const turned = shown >= 1;
    return (
      <div className="lesson-grid">
        <motion.div key={turned ? "turned" : "first"} initial={{ opacity: 0, rotate: turned ? -90 : 0 }} animate={{ opacity: 1, rotate: 0 }} transition={move}>
          <Cars rows={turned ? cols : rows} cols={turned ? rows : cols} label={fr.lessons.grid(turned ? cols : rows, turned ? rows : cols)} />
        </motion.div>
        <div className="grid-products">
          <span className="grid-product">{fr.lessons.gridProduct(rows, cols, total)}</span>
          {turned && <span className="grid-product grid-product-second">{fr.lessons.gridProduct(cols, rows, total)}</span>}
        </div>
      </div>
    );
  }

  return (
    <div className="lesson-grid">
      <Cars rows={rows} cols={cols} splitAt={split !== null && shown >= 1 ? split : null} leaving={removed > 0 && shown >= 1 ? removed : 0} label={fr.lessons.grid(rows, cols)} />
      {split !== null ? (
        shown >= 2 && <SplitSum parts={parts} total={total} />
      ) : removed > 0 ? (
        shown >= 1 && <p className="grid-sum">{fr.lessons.gridLeaving(removed, remaining)}</p>
      ) : (
        <p className="grid-product">{fr.lessons.gridProduct(rows, cols, total)}</p>
      )}
    </div>
  );
}

/** The child places the rows one by one, up to `rows`. */
function GridBuild({ rows, cols, total, onComplete }: ArrayGridProps) {
  const [placed, setPlaced] = useState(0);
  const done = placed === rows;

  const change = (delta: number) => {
    if (done) {
      return;
    }

    const next = Math.max(0, Math.min(placed + delta, 10));
    setPlaced(next);
    if (next === rows) {
      onComplete?.();
    }
  };

  return (
    <div className="lesson-grid">
      {placed > 0 ? <Cars rows={placed} cols={cols} label={fr.lessons.grid(placed, cols)} /> : <div className="grid-empty" />}
      <div className="lesson-block-buttons">
        <button type="button" className="lesson-choice lesson-choice-wide" disabled={done || placed >= 10} onClick={() => change(1)}>
          {fr.lessons.gridAddRow}
        </button>
        <button type="button" className="lesson-choice lesson-choice-wide" disabled={done || placed === 0} onClick={() => change(-1)}>
          {fr.lessons.gridRemoveRow}
        </button>
      </div>
      <p className={done ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
        {done ? fr.lessons.gridBuildDone(rows, cols, total) : `${fr.lessons.gridBuildAsk(rows, cols)} ${fr.lessons.gridRows(placed)}.`}
      </p>
    </div>
  );
}

/** The child clicks between two rows to cut the grid after `split` rows. */
function GridSplit({ rows, cols, total, split, parts, onComplete }: ArrayGridProps) {
  const [cut, setCut] = useState(false);
  const [miss, setMiss] = useState(false);
  const target = split ?? 1;

  const onCut = (after: number) => {
    if (cut) {
      return;
    }

    if (after === target) {
      setCut(true);
      setMiss(false);
      onComplete?.();
    } else {
      setMiss(true);
    }
  };

  return (
    <div className="lesson-grid">
      <Cars rows={rows} cols={cols} splitAt={cut ? target : null} onCut={cut ? undefined : onCut} label={fr.lessons.grid(rows, cols)} />
      {cut && <SplitSum parts={parts} total={total} />}
      <p className={cut ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
        {cut
          ? fr.lessons.gridSplitDone(fr.lessons.gridSum(parts.map((p) => p.product), total))
          : miss
            ? fr.lessons.gridSplitHint(target)
            : fr.lessons.gridSplitAsk(target)}
      </p>
    </div>
  );
}
