import type { Picture } from "../../api/client";
import { fr } from "../../i18n/fr";

/**
 * Pictures of the exercises and of the corrections (first pieces of the animation library,
 * SUB000117): base-ten blocks, fraction bar or disc, graduated line, array of dots, bar model, groups.
 * Every number comes from the core; the front only draws.
 */
type ExercisePictureProps = {
  picture: Picture;
  /** Number line: clicking a graduation answers with its number (`Input.numberLine`). */
  onPickTick?: (value: number) => void;
  /** Number line, after the answer: the clicked number and the expected one. */
  picked?: number | null;
  expected?: number | null;
  disabled?: boolean;
};

export function ExercisePicture({ picture, onPickTick, picked = null, expected = null, disabled = false }: ExercisePictureProps) {
  switch (picture.kind) {
    case "baseTen":
      return <BaseTen hundreds={picture.hundreds} tens={picture.tens} units={picture.units} />;
    case "fractionShape":
      return picture.shape === "Bar" ? (
        <FractionBar parts={picture.parts} colored={picture.colored} />
      ) : (
        <FractionDisc parts={picture.parts} colored={picture.colored} />
      );
    case "numberLine":
      return (
        <NumberLine
          start={picture.start}
          step={picture.step}
          ticks={picture.ticks}
          labeled={picture.labeled}
          arrow={picture.arrow}
          onPickTick={onPickTick}
          picked={picked}
          expected={expected}
          disabled={disabled}
        />
      );
    case "arrayGrid":
      return <ArrayGrid rows={picture.rows} columns={picture.columns} split={picture.split} />;
    case "barModel":
      return <BarModel bars={picture.bars} total={picture.total} />;
    case "groups":
      return <Groups groups={picture.groups} perGroup={picture.perGroup} />;
  }
}

const PLATE = 64;
const CUBE = PLATE / 10;

/** Hundred plates (10 × 10), ten bars (10 cubes) and unit cubes, side by side. */
function BaseTen({ hundreds, tens, units }: { hundreds: number; tens: number; units: number }) {
  const gap = 10;
  const plates = Array.from({ length: hundreds }, (_, i) => i * (PLATE + gap));
  const barsStart = hundreds * (PLATE + gap) + (hundreds > 0 ? gap : 0);
  const bars = Array.from({ length: tens }, (_, i) => barsStart + i * (CUBE + 8));
  const unitsStart = barsStart + tens * (CUBE + 8) + (tens > 0 ? gap * 2 : 0);
  const cubes = Array.from({ length: units }, (_, i) => ({ x: unitsStart + (i % 3) * (CUBE + 6), y: PLATE - CUBE - Math.floor(i / 3) * (CUBE + 6) }));
  const width = Math.max(unitsStart + 3 * (CUBE + 6), 60);
  return (
    <svg className="picture picture-base-ten" viewBox={`-2 -2 ${width + 4} ${PLATE + 4}`} width={(width + 4) * 1.6} role="img" aria-label={fr.pictures.baseTen(hundreds, tens, units)}>
      {plates.map((x) => (
        <g key={`p${x}`}>
          <rect x={x} y={0} width={PLATE} height={PLATE} className="block block-hundred" />
          {Array.from({ length: 9 }, (_, k) => (
            <g key={k}>
              <line x1={x + (k + 1) * CUBE} y1={0} x2={x + (k + 1) * CUBE} y2={PLATE} className="block-line" />
              <line x1={x} y1={(k + 1) * CUBE} x2={x + PLATE} y2={(k + 1) * CUBE} className="block-line" />
            </g>
          ))}
        </g>
      ))}
      {bars.map((x) => (
        <g key={`b${x}`}>
          <rect x={x} y={0} width={CUBE} height={PLATE} className="block block-ten" />
          {Array.from({ length: 9 }, (_, k) => (
            <line key={k} x1={x} y1={(k + 1) * CUBE} x2={x + CUBE} y2={(k + 1) * CUBE} className="block-line" />
          ))}
        </g>
      ))}
      {cubes.map(({ x, y }) => (
        <rect key={`u${x}-${y}`} x={x} y={y} width={CUBE} height={CUBE} className="block block-unit" />
      ))}
    </svg>
  );
}

function FractionBar({ parts, colored }: { parts: number; colored: number }) {
  const width = 400;
  const part = width / parts;
  return (
    <svg className="picture" viewBox={`-4 -4 ${width + 8} 72`} width={width + 8} role="img" aria-label={fr.pictures.fraction("Bar", parts, colored)}>
      {Array.from({ length: parts }, (_, i) => (
        <rect key={i} x={i * part} y={0} width={part} height={64} className={i < colored ? "part part-colored" : "part"} />
      ))}
    </svg>
  );
}

function FractionDisc({ parts, colored }: { parts: number; colored: number }) {
  const r = 80;
  const slice = (i: number) => {
    const a0 = (2 * Math.PI * i) / parts - Math.PI / 2;
    const a1 = (2 * Math.PI * (i + 1)) / parts - Math.PI / 2;
    const large = a1 - a0 > Math.PI ? 1 : 0;
    return `M 0 0 L ${r * Math.cos(a0)} ${r * Math.sin(a0)} A ${r} ${r} 0 ${large} 1 ${r * Math.cos(a1)} ${r * Math.sin(a1)} Z`;
  };
  return (
    <svg className="picture" viewBox={`${-r - 4} ${-r - 4} ${2 * r + 8} ${2 * r + 8}`} width={2 * r + 8} role="img" aria-label={fr.pictures.fraction("Disc", parts, colored)}>
      {parts === 1 ? (
        <circle r={r} className={colored > 0 ? "part part-colored" : "part"} />
      ) : (
        Array.from({ length: parts }, (_, i) => <path key={i} d={slice(i)} className={i < colored ? "part part-colored" : "part"} />)
      )}
    </svg>
  );
}

type NumberLineProps = {
  start: number;
  step: number;
  ticks: number;
  labeled: readonly number[];
  arrow: number | null;
  onPickTick?: (value: number) => void;
  picked: number | null;
  expected: number | null;
  disabled: boolean;
};

/** A graduated half-line; with `onPickTick`, every graduation is a button. */
function NumberLine({ start, step, ticks, labeled, arrow, onPickTick, picked, expected, disabled }: NumberLineProps) {
  const spacing = 58;
  const left = 30;
  const width = left * 2 + spacing * (ticks - 1) + 30;
  const y = 70;
  const value = (i: number) => start + i * step;
  return (
    <svg className="picture picture-line" viewBox={`0 0 ${width} 130`} width={width} role="group" aria-label={fr.pictures.numberLine(start, step)}>
      <line x1={left - 10} y1={y} x2={width - 12} y2={y} className="line-axis" />
      <path d={`M ${width - 12} ${y - 9} L ${width} ${y} L ${width - 12} ${y + 9} Z`} className="line-end" />
      {Array.from({ length: ticks }, (_, i) => {
        const x = left + i * spacing;
        const state = value(i) === expected ? "tick-right" : value(i) === picked ? "tick-wrong" : "";
        return (
          <g key={i} className={`tick ${state}`}>
            <line x1={x} y1={y - 14} x2={x} y2={y + 14} className="tick-line" />
            {(labeled.includes(i) || state !== "") && (
              <text x={x} y={y + 44} className="tick-label">
                {value(i)}
              </text>
            )}
            {onPickTick && (
              <rect
                x={x - spacing / 2 + 2}
                y={y - 30}
                width={spacing - 4}
                height={60}
                className="tick-target"
                role="button"
                tabIndex={disabled ? -1 : 0}
                aria-label={fr.series.tick(i)}
                aria-disabled={disabled}
                onClick={() => !disabled && onPickTick(value(i))}
                onKeyDown={(e) => {
                  if (!disabled && (e.key === "Enter" || e.key === " ")) {
                    e.preventDefault();
                    onPickTick(value(i));
                  }
                }}
              />
            )}
          </g>
        );
      })}
      {arrow !== null && (
        <path
          d={`M ${left + arrow * spacing} ${y - 18} l -12 -22 l 24 0 Z`}
          className="line-arrow"
          role="img"
          aria-label={fr.pictures.arrow}
        />
      )}
    </svg>
  );
}

/** Rows of dots; a dashed line after column `split` (13 × 7 = 10 × 7 + 3 × 7). */
function ArrayGrid({ rows, columns, split }: { rows: number; columns: number; split: number | null }) {
  const gap = 22;
  const extra = split !== null ? 14 : 0;
  const width = columns * gap + extra + 8;
  const height = rows * gap + 8;
  const x = (c: number) => 15 + c * gap + (split !== null && c >= split ? extra : 0);
  return (
    <svg className="picture" viewBox={`0 0 ${width + 8} ${height + 8}`} width={Math.min((width + 8) * 1.4, 620)} role="img" aria-label={fr.pictures.array(rows, columns)}>
      {Array.from({ length: rows }, (_, r) =>
        Array.from({ length: columns }, (_, c) => <circle key={`${r}-${c}`} cx={x(c)} cy={15 + r * gap} r={7} className={split !== null && c >= split ? "dot dot-other" : "dot"} />),
      )}
      {split !== null && <line x1={x(split) - gap / 2 - extra / 2 + 4} y1={2} x2={x(split) - gap / 2 - extra / 2 + 4} y2={height + 4} className="split-line" />}
    </svg>
  );
}

/** Bars of segments, widths proportional to their values (an unknown takes an average width). */
function BarModel({ bars, total }: { bars: readonly (readonly (number | null)[])[]; total: number | null }) {
  const known = bars.flat().filter((v): v is number => v !== null);
  const fallback = known.length > 0 ? known.reduce((a, b) => a + b, 0) / known.length : 1;
  const sums = bars.map((bar) => bar.reduce<number>((s, v) => s + (v ?? fallback), 0));
  const scale = 480 / Math.max(...sums, total ?? 0, 1);
  const height = 50;
  const top = total !== null ? 40 : 6;
  return (
    <svg className="picture" viewBox={`0 0 500 ${top + bars.length * (height + 14)}`} width={500} role="img" aria-label={fr.pictures.barModel}>
      {total !== null && (
        <g>
          <path d={`M 10 ${top - 6} v -14 h ${sums[0] * scale} v 14`} className="bar-total" />
          <text x={10 + (sums[0] * scale) / 2} y={top - 26} className="bar-label">
            {total}
          </text>
        </g>
      )}
      {bars.map((bar, b) => {
        let x = 10;
        return bar.map((v, s) => {
          const w = (v ?? fallback) * scale;
          const segment = (
            <g key={`${b}-${s}`}>
              <rect x={x} y={top + b * (height + 14)} width={w} height={height} className={`bar bar-${s % 3}`} />
              <text x={x + w / 2} y={top + b * (height + 14) + height / 2 + 9} className="bar-label">
                {v ?? "?"}
              </text>
            </g>
          );
          x += w;
          return segment;
        });
      })}
    </svg>
  );
}

/** `groups` boxes of `perGroup` dots (4 boxes of 6). */
function Groups({ groups, perGroup }: { groups: number; perGroup: number }) {
  // Rows of 3 up to 9 objects (6 = 3 + 3), of 5 beyond.
  const perRow = perGroup <= 4 ? perGroup : perGroup <= 9 ? 3 : 5;
  const rowsInGroup = Math.ceil(perGroup / perRow);
  const w = perRow * 18 + 14;
  const h = rowsInGroup * 18 + 14;
  return (
    <div className="picture picture-groups" role="img" aria-label={fr.pictures.groups(groups, perGroup)}>
      {Array.from({ length: groups }, (_, g) => (
        <svg key={g} viewBox={`0 0 ${w} ${h}`} width={w * 1.3} aria-hidden="true">
          <rect x={2} y={2} width={w - 4} height={h - 4} rx={8} className="group-box" />
          {Array.from({ length: perGroup }, (_, i) => (
            <circle key={i} cx={16 + (i % perRow) * 18} cy={16 + Math.floor(i / perRow) * 18} r={6} className="dot" />
          ))}
        </svg>
      ))}
    </div>
  );
}
