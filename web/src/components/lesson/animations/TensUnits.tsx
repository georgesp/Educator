import { motion } from "motion/react";
import { fr } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";

type TensUnitsProps = { hundreds: number; tens: number; units: number; missing: number };

const CUBE = 26;
const PLATE = CUBE * 10;
const GAP = 14;

/**
 * `tensUnits`: hundred plates, ten bars and unit cubes; `missing` cubes in dotted racing red show
 * what lacks to the next ten (19 is « presque 20 »).
 */
export function TensUnits({ hundreds, tens, units, missing }: TensUnitsProps) {
  const shown = useSteps(missing > 0 ? 1 : 0, missing);
  const move = useMove();

  const plates = Array.from({ length: hundreds }, (_, i) => i * (PLATE + GAP));
  const barsStart = hundreds * (PLATE + GAP);
  const bars = Array.from({ length: tens }, (_, i) => barsStart + i * (CUBE + GAP));
  const unitsStart = barsStart + tens * (CUBE + GAP) + GAP;
  // Units stacked in a column of 10, like a ten bar being built.
  const cube = (i: number) => ({ x: unitsStart, y: PLATE - (i + 1) * CUBE });
  const width = unitsStart + CUBE + 4;

  return (
    <figure className="lesson-blocks">
      <svg viewBox={`-4 -4 ${width + 8} ${PLATE + 8}`} width={Math.min((width + 8) * 1.4, 900)} role="img" aria-label={fr.lessons.blocks(hundreds, tens, units)}>
        {plates.map((px) => (
          <g key={`p${px}`}>
            <rect x={px} y={0} width={PLATE} height={PLATE} className="block block-hundred" />
            {Array.from({ length: 9 }, (_, k) => (
              <g key={k}>
                <line x1={px + (k + 1) * CUBE} y1={0} x2={px + (k + 1) * CUBE} y2={PLATE} className="block-line" />
                <line x1={px} y1={(k + 1) * CUBE} x2={px + PLATE} y2={(k + 1) * CUBE} className="block-line" />
              </g>
            ))}
          </g>
        ))}
        {bars.map((bx) => (
          <g key={`b${bx}`}>
            <rect x={bx} y={0} width={CUBE} height={PLATE} className="block block-ten" />
            {Array.from({ length: 9 }, (_, k) => (
              <line key={k} x1={bx} y1={(k + 1) * CUBE} x2={bx + CUBE} y2={(k + 1) * CUBE} className="block-line" />
            ))}
          </g>
        ))}
        {Array.from({ length: units }, (_, i) => (
          <rect key={`u${i}`} {...cube(i)} width={CUBE} height={CUBE} className="block block-unit" />
        ))}
        {shown > 0 &&
          Array.from({ length: missing }, (_, k) => (
            <motion.rect
              key={`m${k}`}
              {...cube(units + k)}
              width={CUBE}
              height={CUBE}
              className="block-missing"
              initial={{ opacity: 0, y: cube(units + k).y - 30 }}
              animate={{ opacity: 1, y: cube(units + k).y }}
              transition={move}
            />
          ))}
      </svg>
      {missing > 0 && shown > 0 && <figcaption className="lesson-blocks-caption">{fr.lessons.missing(missing)}</figcaption>}
    </figure>
  );
}
