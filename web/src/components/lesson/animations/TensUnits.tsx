import { motion } from "motion/react";
import { useState } from "react";
import type { Blocks, BlockStep, BlockTask } from "../../../api/client";
import { fr } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";

type TensUnitsProps = {
  value?: number;
  hundreds: number;
  tens: number;
  units: number;
  /** Dotted cubes up to the next ten (19 is « presque 20 »). */
  missing: number;
  /** The states shown one after the other (computed by the core); the first one is the start. */
  steps: BlockStep[];
  /** The number added or removed, or the number of parts of a share. */
  change: number | null;
  result: number;
  parts: number;
  interactive?: boolean;
  task: BlockTask | null;
  interactiveStart: Blocks;
  target: Blocks;
  onComplete?(): void;
};

type Kind = "hundreds" | "tens" | "units";

const CUBE = 26;
const PLATE = CUBE * 10;
const GAP = 14;

const valueOf = (b: { hundreds: number; tens: number; units: number }) => b.hundreds * 100 + b.tens * 10 + b.units;
const none = { hundreds: 0, tens: 0, units: 0 };

type PictureProps = {
  blocks: { hundreds: number; tens: number; units: number };
  /** The last blocks of each kind drawn in racing red (what just came, the new bar, the 10 new cubes). */
  fresh?: { hundreds: number; tens: number; units: number };
  /** Blocks drawn in dotted lines after the others (what was taken away). */
  ghosts?: { hundreds: number; tens: number; units: number };
  /** Dotted cubes up to the next ten. */
  missing?: number;
  /** Scale of the drawing (1: the blocks of one number on a slide). */
  scale?: number;
  label: string;
};

/** Hundred plates, ten bars and unit cubes (cubes in columns of 10, like a bar being built). */
function BlocksPicture({ blocks, fresh = none, ghosts = none, missing = 0, scale = 1.4, label }: PictureProps) {
  const move = useMove();
  const hundreds = blocks.hundreds + ghosts.hundreds;
  const tens = blocks.tens + ghosts.tens;
  const units = blocks.units + ghosts.units;
  const plates = Array.from({ length: hundreds }, (_, i) => i * (PLATE + GAP));
  const barsStart = hundreds * (PLATE + GAP);
  const bars = Array.from({ length: tens }, (_, i) => barsStart + i * (CUBE + GAP));
  const unitsStart = barsStart + tens * (CUBE + GAP) + (tens > 0 || hundreds > 0 ? GAP : 0);
  const columns = Math.max(1, Math.ceil((units + missing) / 10));
  const cube = (i: number) => ({ x: unitsStart + Math.floor(i / 10) * (CUBE + 6), y: PLATE - ((i % 10) + 1) * CUBE });
  const width = Math.max(unitsStart + columns * (CUBE + 6), CUBE * 2);

  const cls = (base: string, index: number, count: number, fresh: number, ghost: number) =>
    index >= count - ghost ? `${base} block-ghost` : index >= count - ghost - fresh ? `${base} block-new` : base;

  return (
    <svg viewBox={`-4 -4 ${width + 8} ${PLATE + 8}`} width={Math.min((width + 8) * scale, 900)} role="img" aria-label={label}>
      {plates.map((px, i) => (
        <g key={`p${i}`} className={cls("block-group", i, hundreds, fresh.hundreds, ghosts.hundreds)}>
          <rect x={px} y={0} width={PLATE} height={PLATE} className="block block-hundred" />
          {Array.from({ length: 9 }, (_, k) => (
            <g key={k}>
              <line x1={px + (k + 1) * CUBE} y1={0} x2={px + (k + 1) * CUBE} y2={PLATE} className="block-line" />
              <line x1={px} y1={(k + 1) * CUBE} x2={px + PLATE} y2={(k + 1) * CUBE} className="block-line" />
            </g>
          ))}
        </g>
      ))}
      {bars.map((bx, i) => (
        <g key={`b${i}`} className={cls("block-group", i, tens, fresh.tens, ghosts.tens)}>
          <rect x={bx} y={0} width={CUBE} height={PLATE} className="block block-ten" />
          {Array.from({ length: 9 }, (_, k) => (
            <line key={k} x1={bx} y1={(k + 1) * CUBE} x2={bx + CUBE} y2={(k + 1) * CUBE} className="block-line" />
          ))}
        </g>
      ))}
      {Array.from({ length: units }, (_, i) => (
        <g key={`u${i}`} className={cls("block-group", i, units, fresh.units, ghosts.units)}>
          <rect {...cube(i)} width={CUBE} height={CUBE} className="block block-unit" />
        </g>
      ))}
      {Array.from({ length: missing }, (_, k) => (
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
  );
}

const describe = (b: { hundreds: number; tens: number; units: number }) => fr.lessons.blocks(b.hundreds, b.tens, b.units);

/**
 * `tensUnits`: hundred plates, ten bars and unit cubes. Shown by itself: the cubes missing to the next
 * ten, or the steps computed by the core (a number joins and 10 cubes become a bar, a bar is broken to
 * take away, the blocks are shared). Interactive: the child builds the number, or adds, removes or
 * shares blocks until the target.
 */
export function TensUnits(props: TensUnitsProps) {
  if (props.task === "Share") {
    return <SharePlay {...props} />;
  }
  if (props.task !== null) {
    return <BlocksPlay {...props} task={props.task} />;
  }
  return <BlocksShow {...props} />;
}

function BlocksShow({ hundreds, tens, units, missing, steps, change, result, parts }: TensUnitsProps) {
  const start = { hundreds, tens, units };
  const value = valueOf(start);
  // The dotted cubes, or one step after the other.
  const count = steps.length > 1 ? steps.length - 1 : missing > 0 ? 1 : 0;
  const shown = useSteps(count, `${value}-${change ?? 0}-${parts}`);

  if (steps.length <= 1) {
    return (
      <figure className="lesson-blocks">
        <BlocksPicture blocks={start} missing={shown > 0 ? missing : 0} label={describe(start)} />
        {missing > 0 && shown > 0 && <figcaption className="lesson-blocks-caption">{fr.lessons.missing(missing)}</figcaption>}
      </figure>
    );
  }

  const step = steps[Math.min(shown, steps.length - 1)];
  const last = shown >= steps.length - 1;
  const caption = (() => {
    switch (step.kind) {
      case "Start":
        return describe(start);
      case "Join":
        return `${value} ${fr.lessons.sign.Add} ${change ?? 0}`;
      case "TakeAway":
        return `${value} ${fr.lessons.sign.Subtract} ${change ?? 0} = ${result}`;
      case "Share":
        return fr.lessons.share(value, parts, result);
      default:
        return fr.lessons.blockStep[step.kind];
    }
  })();
  const total = step.kind === "Join" || step.kind === "ExchangeUnits" || step.kind === "ExchangeTens" ? (last ? `${value} + ${change ?? 0} = ${result}` : null) : null;

  if (step.kind === "Share") {
    return (
      <figure className="lesson-blocks">
        <div className="blocks-parts">
          {Array.from({ length: parts }, (_, i) => (
            <motion.div key={i} className="blocks-part" initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}>
              <BlocksPicture blocks={step.blocks} scale={0.9} label={`${fr.lessons.sharePart(i + 1)} : ${describe(step.blocks)}`} />
            </motion.div>
          ))}
        </div>
        <figcaption className="lesson-blocks-caption">{caption}</figcaption>
      </figure>
    );
  }

  return (
    <figure className="lesson-blocks">
      <BlocksPicture
        blocks={step.blocks}
        fresh={step.kind === "TakeAway" ? none : step.highlight}
        ghosts={step.kind === "TakeAway" ? step.highlight : none}
        label={describe(step.blocks)}
      />
      <figcaption className="lesson-blocks-caption">{total ?? caption}</figcaption>
    </figure>
  );
}

/** Build the target from nothing, or add or remove blocks from the start until the target. */
function BlocksPlay({ hundreds, tens, units, change, interactiveStart, target, task, onComplete }: TensUnitsProps & { task: Exclude<BlockTask, "Share"> }) {
  const [blocks, setBlocks] = useState({ hundreds: interactiveStart.hundreds, tens: interactiveStart.tens, units: interactiveStart.units });
  const [hint, setHint] = useState<string | null>(null);
  const [done, setDone] = useState(false);
  const start = valueOf({ hundreds, tens, units });
  const goal = target.value;
  // Plates only for numbers of 100 or more (and for a plate the child made anyway, to take it back).
  const withPlates = target.hundreds > 0 || interactiveStart.hundreds > 0;
  const kinds: Kind[] = withPlates || blocks.hundreds > 0 ? ["hundreds", "tens", "units"] : ["tens", "units"];

  const update = (next: typeof blocks) => {
    setBlocks(next);
    setHint(null);
    if (next.hundreds === target.hundreds && next.tens === target.tens && next.units === target.units) {
      setDone(true);
      onComplete?.();
    }
  };

  const add = (kind: Kind, delta: number) => {
    if (done) {
      return;
    }
    if (delta < 0 && blocks[kind] === 0) {
      setHint(kind === "units" ? fr.lessons.noCube : kind === "tens" ? fr.lessons.noBar : null);
      return;
    }
    const next = { ...blocks, [kind]: blocks[kind] + delta };
    if (task === "Remove" && valueOf(next) < goal) {
      // Taking that block away would leave less than the result: refused, with a hint.
      setHint(fr.lessons.blocksTooFew(goal));
      return;
    }
    update(next);
  };

  const exchange = (kind: "units" | "tens") => {
    if (done) {
      return;
    }
    update(kind === "units" ? { ...blocks, units: blocks.units - 10, tens: blocks.tens + 1 } : { ...blocks, tens: blocks.tens - 10, hundreds: blocks.hundreds + 1 });
  };

  const breakOne = (kind: "tens" | "hundreds") => {
    if (done) {
      return;
    }
    update(kind === "tens" ? { ...blocks, tens: blocks.tens - 1, units: blocks.units + 10 } : { ...blocks, hundreds: blocks.hundreds - 1, tens: blocks.tens + 10 });
  };

  const value = valueOf(blocks);
  const ask =
    task === "Build" ? fr.lessons.blocksBuildAsk(goal) : task === "Add" ? fr.lessons.blocksAddAsk(start, change ?? 0) : fr.lessons.blocksRemoveAsk(start, change ?? 0);
  const status = done
    ? task === "Remove"
      ? fr.lessons.blocksLeft(goal)
      : fr.lessons.blocksDone(goal)
    : (hint ??
      (blocks.units >= 10
        ? fr.lessons.exchangeUnitsHint
        : blocks.tens >= 10 && withPlates
          ? fr.lessons.exchangeTensHint
          : task !== "Remove" && value > goal
            ? fr.lessons.blocksTooMuch(goal)
            : `${ask} ${fr.lessons.blocksValue(value)}`));

  return (
    <figure className="lesson-blocks">
      {value > 0 ? <BlocksPicture blocks={blocks} label={describe(blocks)} /> : <div className="blocks-empty" />}
      <div className="lesson-block-buttons">
        {task !== "Remove" &&
          kinds.map((kind) => (
            <button key={`+${kind}`} type="button" className="lesson-choice lesson-choice-wide" disabled={done} onClick={() => add(kind, 1)}>
              {fr.lessons.blocksPlus[kind]}
            </button>
          ))}
        {kinds.map((kind) => (
          <button key={`-${kind}`} type="button" className="lesson-choice lesson-choice-wide" disabled={done || (task !== "Remove" && blocks[kind] === 0)} onClick={() => add(kind, -1)}>
            {fr.lessons.blocksMinus[kind]}
          </button>
        ))}
        {blocks.units >= 10 && (
          <button type="button" className="lesson-choice lesson-choice-wide lesson-choice-key" disabled={done} onClick={() => exchange("units")}>
            {fr.lessons.exchangeUnits}
          </button>
        )}
        {blocks.tens >= 10 && withPlates && (
          <button type="button" className="lesson-choice lesson-choice-wide lesson-choice-key" disabled={done} onClick={() => exchange("tens")}>
            {fr.lessons.exchangeTens}
          </button>
        )}
        {task === "Remove" && blocks.tens > 0 && (
          <button type="button" className="lesson-choice lesson-choice-wide" disabled={done} onClick={() => breakOne("tens")}>
            {fr.lessons.breakTen}
          </button>
        )}
        {task === "Remove" && withPlates && blocks.hundreds > 0 && (
          <button type="button" className="lesson-choice lesson-choice-wide" disabled={done} onClick={() => breakOne("hundreds")}>
            {fr.lessons.breakHundred}
          </button>
        )}
      </div>
      <p className={done ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
        {status}
      </p>
    </figure>
  );
}

/**
 * The child gives the blocks one by one to the parts, in turn. A plate or a bar that cannot be shared
 * any more is broken into 10 smaller blocks.
 */
function SharePlay({ interactiveStart, parts, target, result, onComplete }: TensUnitsProps) {
  const [pile, setPile] = useState({ hundreds: interactiveStart.hundreds, tens: interactiveStart.tens, units: interactiveStart.units });
  const [given, setGiven] = useState(() => Array.from({ length: parts }, () => ({ ...none })));
  const [done, setDone] = useState(false);

  // The largest kind left is stuck when what remains of it cannot go once more to every part: it is
  // broken into 10 smaller blocks. Cubes are never broken (the core checks that the share is exact).
  const largest = (["hundreds", "tens"] as Kind[]).find((k) => pile[k] > 0);
  const stuck = (kind: Kind) =>
    kind !== "units" && kind === largest && pile[kind] < parts && given.every((g) => g[kind] === given[0][kind]);
  // While a block must be broken, the smaller ones wait.
  const waiting = (kind: Kind) => largest !== undefined && stuck(largest) && kind !== largest;

  const give = (kind: Kind) => {
    if (done) {
      return;
    }
    if (waiting(kind)) {
      return;
    }
    if (stuck(kind)) {
      // Break the block: a plate becomes 10 bars, a bar 10 cubes.
      setPile(kind === "hundreds" ? { ...pile, hundreds: pile.hundreds - 1, tens: pile.tens + 10 } : { ...pile, tens: pile.tens - 1, units: pile.units + 10 });
      return;
    }
    // To the part that has the fewest blocks of this kind (the first one on a tie).
    const fewest = given.reduce((best, g, i) => (g[kind] < given[best][kind] ? i : best), 0);
    const nextGiven = given.map((g, i) => (i === fewest ? { ...g, [kind]: g[kind] + 1 } : g));
    const nextPile = { ...pile, [kind]: pile[kind] - 1 };
    setGiven(nextGiven);
    setPile(nextPile);
    if (valueOf(nextPile) === 0 && nextGiven.every((g) => valueOf(g) === target.value)) {
      setDone(true);
      onComplete?.();
    }
  };

  const kinds: Kind[] = (["hundreds", "tens", "units"] as Kind[]).filter((k) => pile[k] > 0);
  const stuckKind = kinds.find((k) => stuck(k));

  return (
    <figure className="lesson-blocks">
      {valueOf(pile) > 0 && <BlocksPicture blocks={pile} scale={0.8} label={`${fr.lessons.shareLeft} : ${describe(pile)}`} />}
      <div className="blocks-parts">
        {given.map((g, i) => (
          <div key={i} className="blocks-part" role="group" aria-label={`${fr.lessons.sharePart(i + 1)} : ${describe(g)}`}>
            {valueOf(g) > 0 ? <BlocksPicture blocks={g} scale={0.7} label={describe(g)} /> : <div className="blocks-empty blocks-empty-small" />}
          </div>
        ))}
      </div>
      <div className="lesson-block-buttons">
        {kinds.map((kind) => (
          <button key={kind} type="button" className={`lesson-choice lesson-choice-wide${stuck(kind) ? " lesson-choice-key" : ""}`} disabled={done || waiting(kind)} onClick={() => give(kind)}>
            {stuck(kind) ? (kind === "hundreds" ? fr.lessons.shareBreakHundred : fr.lessons.shareBreakTen) : fr.lessons.shareGive[kind]}
          </button>
        ))}
      </div>
      <p className={done ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
        {done ? fr.lessons.shareDone(result) : stuckKind === "hundreds" ? fr.lessons.shareHundred : stuckKind === "tens" ? fr.lessons.shareTen : fr.lessons.shareAsk(parts)}
      </p>
    </figure>
  );
}
