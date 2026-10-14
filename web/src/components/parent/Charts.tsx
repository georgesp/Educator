/**
 * Simple charts of the parents' space (SUB001511), drawn by hand: no chart library. Every value is
 * also written as text (labels, `aria-label`), never carried by the color alone.
 */

export type Bar = {
  key: string;
  /** Under the bar: « 12/10 ». */
  label: string;
  value: number;
  /** Above the bar: « 15 min »; empty for a zero value. */
  valueLabel: string;
  /** Read by screen readers (hidden text) and shown on hover: « lun. 12 oct. : 15 min ». */
  description: string;
  /** Emphasised (today, the current week). */
  current?: boolean;
};

/** Vertical bars with their value above and their label below. */
export function BarChart({ title, caption, bars, max }: { title: string; caption: string; bars: Bar[]; max?: number }) {
  const top = Math.max(max ?? 0, ...bars.map((b) => b.value), 1);
  return (
    <figure className="parent-chart">
      <figcaption>
        <strong>{title}</strong>
        <span>{caption}</span>
      </figcaption>
      <ul className="bar-chart plain-list" aria-label={title}>
        {bars.map((bar) => (
          <li key={bar.key} className={bar.current ? "bar current" : "bar"} title={bar.description}>
            <span className="visually-hidden">{bar.description}</span>
            <span className="bar-value" aria-hidden="true">
              {bar.valueLabel}
            </span>
            <span className="bar-track" aria-hidden="true">
              <span className="bar-fill" style={{ height: `${(bar.value / top) * 100}%` }} />
            </span>
            <span className="bar-label" aria-hidden="true">
              {bar.label}
            </span>
          </li>
        ))}
      </ul>
    </figure>
  );
}

const SPARK_WIDTH = 132;
const SPARK_HEIGHT = 36;
const SPARK_PAD = 4;

/** Coordinates of `values` in a box, from 0 to the largest value; one value sits in the middle. */
function placePoints(values: number[], width: number, height: number, pad: number): [number, number][] {
  const top = Math.max(...values, 1);
  const step = values.length > 1 ? (width - 2 * pad) / (values.length - 1) : 0;
  return values.map((value, i) => [
    values.length > 1 ? pad + i * step : width / 2,
    height - pad - (Math.max(value, 0) / top) * (height - 2 * pad),
  ]);
}

/** The points of the last series of an exercise, as a small line; the last point is marked. */
export function Sparkline({ values, label }: { values: number[]; label: string }) {
  const points = placePoints(values, SPARK_WIDTH, SPARK_HEIGHT, SPARK_PAD);
  const last = points[points.length - 1];
  return (
    <svg className="sparkline" width={SPARK_WIDTH} height={SPARK_HEIGHT} viewBox={`0 0 ${SPARK_WIDTH} ${SPARK_HEIGHT}`} role="img" aria-label={label}>
      <title>{label}</title>
      <line className="sparkline-base" x1={SPARK_PAD} x2={SPARK_WIDTH - SPARK_PAD} y1={SPARK_HEIGHT - SPARK_PAD} y2={SPARK_HEIGHT - SPARK_PAD} />
      {points.length > 1 && <polyline className="sparkline-line" points={points.map(([x, y]) => `${x},${y}`).join(" ")} />}
      {last && <circle className="sparkline-last" cx={last[0]} cy={last[1]} r={3.5} />}
    </svg>
  );
}

export type ChartPoint = { key: string; value: number; label: string; description: string };

const LINE_WIDTH = 640;
const LINE_HEIGHT = 200;
const LINE_PAD_X = 28;
const LINE_PAD_TOP = 26;
const LINE_PAD_BOTTOM = 30;

/** A larger line chart: every point with its value above and its label below (every few points). */
export function LineChart({ title, points }: { title: string; points: ChartPoint[] }) {
  const top = Math.max(...points.map((p) => p.value), 1);
  const inner = LINE_HEIGHT - LINE_PAD_TOP - LINE_PAD_BOTTOM;
  const step = points.length > 1 ? (LINE_WIDTH - 2 * LINE_PAD_X) / (points.length - 1) : 0;
  const placed = points.map((p, i) => ({
    ...p,
    x: points.length > 1 ? LINE_PAD_X + i * step : LINE_WIDTH / 2,
    y: LINE_PAD_TOP + inner - (Math.max(p.value, 0) / top) * inner,
  }));
  // At most ~8 date labels under the axis.
  const labelEvery = Math.max(1, Math.ceil(points.length / 8));
  return (
    <figure className="parent-chart line-chart">
      <figcaption>
        <strong>{title}</strong>
      </figcaption>
      <svg viewBox={`0 0 ${LINE_WIDTH} ${LINE_HEIGHT}`} role="img" aria-label={`${title} : ${points.map((p) => p.description).join(" ; ")}`}>
        <line className="line-axis" x1={LINE_PAD_X / 2} x2={LINE_WIDTH - LINE_PAD_X / 2} y1={LINE_PAD_TOP + inner} y2={LINE_PAD_TOP + inner} />
        {placed.length > 1 && <polyline className="line-path" points={placed.map((p) => `${p.x},${p.y}`).join(" ")} />}
        {placed.map((p, i) => (
          <g key={p.key}>
            <circle className="line-dot" cx={p.x} cy={p.y} r={5}>
              <title>{p.description}</title>
            </circle>
            <text className="line-value" x={p.x} y={p.y - 10} textAnchor="middle">
              {p.value}
            </text>
            {(i % labelEvery === 0 || i === placed.length - 1) && (
              <text className="line-label" x={p.x} y={LINE_HEIGHT - 8} textAnchor="middle">
                {p.label}
              </text>
            )}
          </g>
        ))}
      </svg>
    </figure>
  );
}
