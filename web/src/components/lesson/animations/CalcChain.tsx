import { AnimatePresence, motion } from "motion/react";
import { Fragment, useMemo, useState } from "react";
import type { Chain, ChainMode } from "../../../api/client";
import { fr, signed } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";

type CalcChainProps = {
  chains: Chain[];
  mode: ChainMode;
  /** Called once every calculation is linked to its chain (mode Match), or the right chain is picked (mode Pick). */
  onComplete?(): void;
};

/** `calcChain`: mental calculation as chains of arrows (46 → + 20 → 66 → − 1 → 65). */
export function CalcChain({ chains, mode, onComplete }: CalcChainProps) {
  switch (mode) {
    case "Match":
      return <ChainMatch chains={chains} onComplete={onComplete} />;
    case "Pick":
      return <ChainPick chains={chains} onComplete={onComplete} />;
    case "Show":
      return <ChainShow chains={chains} />;
  }
}

const stepsText = (chain: Chain) => chain.steps.map((s) => signed(s.delta)).join(", ");

/** The chains appear one step after the other, chain after chain. */
function ChainShow({ chains }: { chains: Chain[] }) {
  const total = chains.reduce((n, c) => n + c.steps.length, 0);
  const shown = useSteps(total, chains);
  const move = useMove();
  let before = 0;

  return (
    <div className="lesson-chains">
      {chains.map((chain, row) => {
        const offset = before;
        before += chain.steps.length;
        return (
          <div
            key={row}
            className={chain.crossed ? "lesson-chain lesson-chain-crossed" : "lesson-chain"}
            role="img"
            aria-label={fr.lessons.chain(chain.start, stepsText(chain), chain.result)}
          >
            <span className="chain-value">{chain.start}</span>
            {chain.steps.map((step, i) => {
              const visible = offset + i < shown;
              const last = i === chain.steps.length - 1;
              return (
                <Fragment key={i}>
                  <AnimatePresence>
                    {visible && (
                      <motion.span className="chain-arrow" initial={{ opacity: 0, x: -12 }} animate={{ opacity: 1, x: 0 }} transition={move}>
                        <span className={last ? "chain-step chain-step-key" : "chain-step"}>{signed(step.delta)}</span>
                        <span className="chain-arrow-line" aria-hidden="true" />
                      </motion.span>
                    )}
                  </AnimatePresence>
                  <AnimatePresence>
                    {visible && (
                      <motion.span
                        className={last ? "chain-value chain-value-result" : "chain-value"}
                        initial={{ opacity: 0, scale: 0.6 }}
                        animate={{ opacity: 1, scale: 1 }}
                        transition={{ ...move, delay: move.duration / 2 }}
                      >
                        {step.value}
                      </motion.span>
                    )}
                  </AnimatePresence>
                </Fragment>
              );
            })}
            {chain.crossed && offset + chain.steps.length <= shown && <span className="chain-wrong">{fr.lessons.wrongWay}</span>}
          </div>
        );
      })}
    </div>
  );
}

/**
 * The child links each calculation (`start + total`) to its chain of steps: a click on a calculation,
 * then a click on a chain. The chains are listed in another order than the calculations.
 */
function ChainMatch({ chains, onComplete }: { chains: Chain[]; onComplete?(): void }) {
  const [selected, setSelected] = useState<number | null>(null);
  const [linked, setLinked] = useState<number[]>([]);
  const [hint, setHint] = useState(false);
  // Chain i belongs to calculation i: shown rotated by one, so that no line is already right.
  const order = useMemo(() => chains.map((_, i) => (i + 1) % chains.length), [chains]);
  const done = linked.length === chains.length;

  const pickChain = (index: number) => {
    if (selected === null || linked.includes(index)) {
      return;
    }

    if (index === selected) {
      const next = [...linked, index];
      setLinked(next);
      setSelected(null);
      setHint(false);
      if (next.length === chains.length) {
        onComplete?.();
      }
    } else {
      // Gentle hint, the calculation stays selected.
      setHint(true);
    }
  };

  return (
    <div className="lesson-match">
      <div className="lesson-match-columns">
        <div className="lesson-match-column" role="group" aria-label={fr.lessons.matchCalculations}>
          <span className="lesson-match-heading">{fr.lessons.matchCalculations}</span>
          {chains.map((chain, i) => (
            <button
              key={i}
              type="button"
              className={`lesson-match-item ${linked.includes(i) ? "match-linked" : selected === i ? "match-selected" : ""}`}
              aria-pressed={selected === i}
              disabled={linked.includes(i)}
              onClick={() => {
                setSelected(i);
                setHint(false);
              }}
            >
              {/* The number added, in racing red: it is what the chain must add in all. */}
              {chain.start} <span className="match-added">+ {chain.total}</span>
              {linked.includes(i) ? ` = ${chain.result}` : null}
            </button>
          ))}
        </div>
        <div className="lesson-match-column" role="group" aria-label={fr.lessons.matchChains}>
          <span className="lesson-match-heading">{fr.lessons.matchChains}</span>
          {order.map((i) => (
            <button
              key={i}
              type="button"
              className={`lesson-match-item lesson-match-chain ${linked.includes(i) ? "match-linked" : ""}`}
              disabled={linked.includes(i)}
              onClick={() => pickChain(i)}
            >
              {chains[i].steps.map((s) => signed(s.delta)).join(` ${fr.lessons.then} `)}
            </button>
          ))}
        </div>
      </div>
      <p className={done ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
        {done
          ? fr.lessons.matchDone
          : hint
            ? fr.lessons.matchHint
            : selected !== null
              ? fr.lessons.matchWhich(chains[selected].total)
              : fr.lessons.matchPick}
      </p>
    </div>
  );
}

/** The values of a chain, from its start: 42 → + 10 → 52 → − 1 → 51. */
function ChainValues({ chain }: { chain: Chain }) {
  return (
    <>
      <span className="chain-value">{chain.start}</span>
      {chain.steps.map((step, i) => (
        <Fragment key={i}>
          <span className="chain-arrow">
            <span className="chain-step">{signed(step.delta)}</span>
            <span className="chain-arrow-line" aria-hidden="true" />
          </span>
          <span className="chain-value">{step.value}</span>
        </Fragment>
      ))}
    </>
  );
}

/**
 * The child clicks the only right chain; the others are classic mistakes (crossed). A wrong chain is
 * crossed out gently, with a hint; the right one turns to the « result » color.
 */
function ChainPick({ chains, onComplete }: { chains: Chain[]; onComplete?(): void }) {
  const [wrong, setWrong] = useState<number[]>([]);
  const [found, setFound] = useState<number | null>(null);
  const right = chains.findIndex((c) => !c.crossed);

  const pick = (index: number) => {
    if (found !== null || wrong.includes(index)) {
      return;
    }

    if (index === right) {
      setFound(index);
      onComplete?.();
    } else {
      setWrong([...wrong, index]);
    }
  };

  const chain = chains[right];
  return (
    <div className="lesson-chains" role="group" aria-label={fr.lessons.pickAsk}>
      {chains.map((c, i) => {
        const crossed = wrong.includes(i) || (found !== null && i !== found);
        return (
          <button
            key={i}
            type="button"
            className={`lesson-chain lesson-chain-pick${crossed ? " lesson-chain-crossed" : ""}${found === i ? " lesson-chain-right" : ""}`}
            disabled={found !== null || wrong.includes(i)}
            aria-label={fr.lessons.pickChain(c.start, stepsText(c))}
            onClick={() => pick(i)}
          >
            <ChainValues chain={c} />
            {crossed && <span className="chain-wrong">{fr.lessons.wrongWay}</span>}
          </button>
        );
      })}
      <p className={found !== null ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
        {found !== null ? fr.lessons.pickDone(chain.start, chain.total, chain.result) : wrong.length > 0 ? fr.lessons.pickHint : fr.lessons.pickAsk}
      </p>
    </div>
  );
}
