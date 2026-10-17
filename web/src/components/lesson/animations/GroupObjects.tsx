import { AnimatePresence, motion } from "motion/react";
import { useState } from "react";
import type { GroupMode, Grouping, RaceObjectName } from "../../../api/client";
import { fr } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";
import { RaceObject, type ObjectColor } from "./RaceObjects";

type GroupObjectsProps = {
  mode: GroupMode;
  object: RaceObjectName;
  counts: number[];
  total: number;
  removed: number;
  size: number;
  result: number;
  groupings: Grouping[];
  numbers: number[];
  parities: boolean[];
  commute: boolean;
  interactive: boolean;
  choices: number[];
  onComplete?(): void;
};

/**
 * `groupObjects`: racing objects joined, taken away, grouped, shared, or numbers sorted in the
 * « pair » and « impair » garages.
 */
export function GroupObjects(props: GroupObjectsProps) {
  switch (props.mode) {
    case "Join":
      return props.interactive ? <JoinPlay {...props} /> : <JoinShow {...props} />;
    case "Remove":
      return props.interactive ? <RemovePlay {...props} /> : <RemoveShow {...props} />;
    case "GroupBy":
      return props.interactive ? <GroupByPlay {...props} /> : <GroupByShow {...props} />;
    case "Share":
      return props.interactive ? <SharePlay {...props} /> : <ShareShow {...props} />;
    case "SortParity":
      return <SortParity {...props} />;
  }
}

const names = (object: RaceObjectName) => fr.lessons.objects[object];

function Objects({ object, count, color, ghosts = 0, label }: { object: RaceObjectName; count: number; color: ObjectColor; ghosts?: number; label?: string }) {
  return (
    <span className="objects-row" role={label ? "img" : undefined} aria-label={label}>
      {Array.from({ length: count }, (_, i) => (
        <RaceObject key={i} object={object} color={color} ghost={i >= count - ghosts} />
      ))}
    </span>
  );
}

function JoinShow({ object, counts, result, commute }: GroupObjectsProps) {
  const shown = useSteps(commute ? 2 : 1, counts);
  const move = useMove();
  const [a, b] = counts;
  const swapped = commute && shown >= 2;
  const groups = swapped ? ([b, "blue"] as const) : ([a, "red"] as const);
  const others = swapped ? ([a, "red"] as const) : ([b, "blue"] as const);

  return (
    <div className="lesson-objects">
      <motion.div className={shown >= 1 ? "objects-line objects-line-joined" : "objects-line"} layout transition={move}>
        <Objects object={object} count={groups[0]} color={groups[1]} label={fr.lessons.objectCount(groups[0], names(object))} />
        <Objects object={object} count={others[0]} color={others[1]} label={fr.lessons.objectCount(others[0], names(object))} />
      </motion.div>
      <p className="objects-caption">
        {shown >= 1 ? fr.lessons.frameSum(groups[0], others[0], result) : `${a} ${fr.lessons.sign.Add} ${b}`}
        {commute && shown >= 2 && <span className="objects-caption-second">{fr.lessons.frameSum(a, b, result)}</span>}
      </p>
    </div>
  );
}

/** Pick the total among the choices, after the objects are moved. */
function TotalChoice({ ask, choices, result, onRight }: { ask: string; choices: number[]; result: number; onRight(): void }) {
  const [miss, setMiss] = useState(false);
  const [found, setFound] = useState(false);
  return (
    <>
      <div className="lesson-choices" role="group" aria-label={ask}>
        {choices.map((c) => (
          <button
            key={c}
            type="button"
            className={`lesson-choice${found && c === result ? " lesson-choice-right" : ""}`}
            disabled={found}
            onClick={() => {
              if (c === result) {
                setFound(true);
                onRight();
              } else {
                setMiss(true);
              }
            }}
          >
            {c}
          </button>
        ))}
      </div>
      {!found && (
        <p className="lesson-hint" role="status">
          {miss ? fr.lessons.countHint : ask}
        </p>
      )}
    </>
  );
}

/** The child brings each object onto the starting line, then picks the total. */
function JoinPlay({ object, counts, result, choices, onComplete }: GroupObjectsProps) {
  const [a, b] = counts;
  const [moved, setMoved] = useState<number[]>([]);
  const [done, setDone] = useState(false);
  const all = a + b;
  const allMoved = moved.length === all;
  const n = names(object);

  return (
    <div className="lesson-objects">
      <div className="objects-paddocks">
        {[0, 1].map((group) => (
          <div key={group} className="objects-paddock">
            {Array.from({ length: group === 0 ? a : b }, (_, i) => {
              const id = group === 0 ? i : a + i;
              return moved.includes(id) ? null : (
                <button key={id} type="button" className="object-button" aria-label={n[0]} onClick={() => setMoved([...moved, id])}>
                  <RaceObject object={object} color={group === 0 ? "red" : "blue"} />
                </button>
              );
            })}
          </div>
        ))}
      </div>
      <div className="objects-line objects-line-joined" role="group" aria-label={fr.lessons.objectCount(moved.length, n)}>
        <AnimatePresence>
          {moved.map((id) => (
            <motion.span key={id} initial={{ opacity: 0, y: -30 }} animate={{ opacity: 1, y: 0 }}>
              <RaceObject object={object} color={id < a ? "red" : "blue"} />
            </motion.span>
          ))}
        </AnimatePresence>
      </div>
      {!allMoved && (
        <p className="lesson-hint" role="status">
          {fr.lessons.joinAsk(n)}
        </p>
      )}
      {allMoved && !done && (
        <TotalChoice
          ask={fr.lessons.totalAsk}
          choices={choices}
          result={result}
          onRight={() => {
            setDone(true);
            onComplete?.();
          }}
        />
      )}
      {done && (
        <p className="lesson-hint lesson-hint-done" role="status">
          {fr.lessons.joinDone(a, b, result)}
        </p>
      )}
    </div>
  );
}

function RemoveShow({ object, total, removed, result }: GroupObjectsProps) {
  const shown = useSteps(1, total);
  return (
    <div className="lesson-objects">
      <Objects object={object} count={total} color="red" ghosts={shown >= 1 ? removed : 0} label={fr.lessons.objectCount(shown >= 1 ? result : total, names(object))} />
      <p className="objects-caption">{shown >= 1 ? `${total} ${fr.lessons.sign.Subtract} ${removed} = ${result}` : fr.lessons.objectCount(total, names(object))}</p>
    </div>
  );
}

/** The child makes `removed` objects leave, then picks what remains. */
function RemovePlay({ object, total, removed, result, choices, onComplete }: GroupObjectsProps) {
  const [gone, setGone] = useState<number[]>([]);
  const [done, setDone] = useState(false);
  const allGone = gone.length === removed;
  const n = names(object);

  return (
    <div className="lesson-objects">
      <div className="objects-row">
        {Array.from({ length: total }, (_, i) =>
          gone.includes(i) ? (
            <RaceObject key={i} object={object} ghost />
          ) : (
            <button key={i} type="button" className="object-button" aria-label={n[0]} disabled={allGone} onClick={() => setGone([...gone, i])}>
              <RaceObject object={object} />
            </button>
          ),
        )}
      </div>
      {!allGone && (
        <p className="lesson-hint" role="status">
          {fr.lessons.removeAsk(removed - gone.length, n)}
        </p>
      )}
      {allGone && !done && (
        <TotalChoice
          ask={fr.lessons.restAsk}
          choices={choices}
          result={result}
          onRight={() => {
            setDone(true);
            onComplete?.();
          }}
        />
      )}
      {done && (
        <p className="lesson-hint lesson-hint-done" role="status">
          {fr.lessons.removeDone(total, removed, result)}
        </p>
      )}
    </div>
  );
}

function GroupByShow({ object, size, groupings }: GroupObjectsProps) {
  const shown = useSteps(1, groupings);
  return (
    <div className="lesson-objects">
      {groupings.map((g, k) => (
        <div key={k} className="objects-grouping">
          <div className="objects-groups">
            {shown >= 1 ? (
              <>
                {Array.from({ length: g.groups }, (_, i) => (
                  <span key={i} className="objects-group">
                    {Array.from({ length: size }, (_, j) => (
                      <RaceObject key={j} object={object} color={k === 0 ? "red" : "blue"} size={44} />
                    ))}
                  </span>
                ))}
                {g.rest > 0 && (
                  <span className="objects-group objects-rest">
                    {Array.from({ length: g.rest }, (_, j) => (
                      <RaceObject key={j} object={object} color={k === 0 ? "red" : "blue"} size={44} />
                    ))}
                  </span>
                )}
              </>
            ) : (
              <Objects object={object} count={g.count} color={k === 0 ? "red" : "blue"} />
            )}
          </div>
          {shown >= 1 && <p className="objects-caption">{size === 2 ? fr.lessons.pairs(g.count, g.groups, g.rest) : fr.lessons.groups(g.count, g.groups, size, g.rest)}</p>}
        </div>
      ))}
    </div>
  );
}

/** The child makes the packets of `size` (10) by clicking the loose objects, then picks the number. */
function GroupByPlay({ object, counts, size, result, choices, onComplete }: GroupObjectsProps) {
  const count = counts[0] ?? 0;
  const [packed, setPacked] = useState(0);
  const [done, setDone] = useState(false);
  const groups = Math.floor(packed / size);
  const filling = packed % size;
  const rest = count - Math.floor(count / size) * size;
  const allPacked = packed === count - rest;
  const n = names(object);

  return (
    <div className="lesson-objects">
      <div className="objects-groups" role="group" aria-label={fr.lessons.tensMade(groups)}>
        {Array.from({ length: groups }, (_, i) => (
          <motion.span key={i} className="objects-group objects-ten" initial={{ scale: 0.8 }} animate={{ scale: 1 }}>
            {Array.from({ length: size }, (_, j) => (
              <RaceObject key={j} object={object} color="blue" size={28} />
            ))}
          </motion.span>
        ))}
        {filling > 0 && (
          <span className="objects-group objects-filling" aria-label={fr.lessons.tensFilling(filling, size)}>
            {Array.from({ length: filling }, (_, j) => (
              <motion.span key={j} initial={{ opacity: 0, y: -20 }} animate={{ opacity: 1, y: 0 }}>
                <RaceObject object={object} color="blue" size={28} />
              </motion.span>
            ))}
          </span>
        )}
      </div>
      <div className="objects-row">
        {Array.from({ length: count - packed }, (_, i) => (
          <button
            key={i}
            type="button"
            className="object-button"
            aria-label={n[0]}
            disabled={allPacked}
            onClick={() => setPacked((p) => Math.min(p + 1, count - rest))}
          >
            <RaceObject object={object} color="blue" />
          </button>
        ))}
      </div>
      {!allPacked && (
        <p className="lesson-hint" role="status">
          {fr.lessons.tensAsk(size, filling)}
        </p>
      )}
      {allPacked && !done && (
        <>
          <p className="objects-caption">{fr.lessons.tensGrouped(groups, rest)}</p>
          <TotalChoice
            ask={fr.lessons.totalAsk}
            choices={choices}
            result={result}
            onRight={() => {
              setDone(true);
              onComplete?.();
            }}
          />
        </>
      )}
      {done && (
        <p className="lesson-hint lesson-hint-done" role="status">
          {fr.lessons.tensDone(groups, rest, result)}
        </p>
      )}
    </div>
  );
}

/** A fair share played by the child (SUB012215): each object clicked goes to the next part, in turn. */
function SharePlay({ object, total, size, result, choices, onComplete }: GroupObjectsProps) {
  const [given, setGiven] = useState(0);
  const [done, setDone] = useState(false);
  const parts = Math.max(size, 1);
  const allGiven = given === total;
  const n = names(object);
  const inPart = (part: number) => Math.floor(given / parts) + (part < given % parts ? 1 : 0);

  return (
    <div className="lesson-objects">
      <div className="objects-groups">
        {Array.from({ length: parts }, (_, part) => (
          <span key={part} className="objects-group" role="group" aria-label={`${fr.lessons.sharePart(part + 1)} : ${fr.lessons.objectCount(inPart(part), n)}`}>
            {Array.from({ length: inPart(part) }, (_, j) => (
              <motion.span key={j} initial={{ opacity: 0, y: -20 }} animate={{ opacity: 1, y: 0 }}>
                <RaceObject object={object} color="blue" size={28} />
              </motion.span>
            ))}
          </span>
        ))}
      </div>
      <div className="objects-row" role="group" aria-label={fr.lessons.shareLeft}>
        {Array.from({ length: total - given }, (_, i) => (
          <button key={i} type="button" className="object-button" aria-label={n[0]} onClick={() => setGiven((g) => Math.min(g + 1, total))}>
            <RaceObject object={object} color="red" />
          </button>
        ))}
      </div>
      {!allGiven && (
        <p className="lesson-hint" role="status">
          {fr.lessons.shareObjectsAsk(n)}
        </p>
      )}
      {allGiven && !done && (
        <TotalChoice
          ask={fr.lessons.shareEachAsk}
          choices={choices}
          result={result}
          onRight={() => {
            setDone(true);
            onComplete?.();
          }}
        />
      )}
      {done && (
        <p className="lesson-hint lesson-hint-done" role="status">
          {fr.lessons.shareObjectsDone(total, parts, result)}
        </p>
      )}
    </div>
  );
}

function ShareShow({ object, total, size, result }: GroupObjectsProps) {
  const shown = useSteps(1, total);
  return (
    <div className="lesson-objects">
      {shown >= 1 ? (
        <div className="objects-groups">
          {Array.from({ length: size }, (_, i) => (
            <motion.span key={i} className="objects-group" initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}>
              {Array.from({ length: result }, (_, j) => (
                <RaceObject key={j} object={object} size={40} />
              ))}
            </motion.span>
          ))}
        </div>
      ) : (
        <Objects object={object} count={total} color="red" />
      )}
      <p className="objects-caption">{shown >= 1 ? fr.lessons.share(total, size, result) : fr.lessons.objectCount(total, names(object))}</p>
    </div>
  );
}

/** Numbers parked in the « pair » or « impair » garage; interactive, the child parks each one. */
function SortParity({ numbers, parities, interactive, onComplete }: GroupObjectsProps) {
  const auto = useSteps(interactive ? 0 : numbers.length, numbers);
  const [parked, setParked] = useState<number[]>([]);
  const [selected, setSelected] = useState<number | null>(null);
  const [hint, setHint] = useState<number | null>(null);
  const placed = interactive ? parked : numbers.slice(0, auto).map((_, i) => i);
  const done = interactive && parked.length === numbers.length;

  const park = (even: boolean) => {
    if (selected === null || done) {
      return;
    }

    if (parities[selected] === even) {
      const next = [...parked, selected];
      setParked(next);
      setSelected(null);
      setHint(null);
      if (next.length === numbers.length) {
        onComplete?.();
      }
    } else {
      setHint(numbers[selected]);
    }
  };

  const garage = (even: boolean) => (
    <button type="button" className="parity-garage" disabled={!interactive || selected === null || done} onClick={() => park(even)}>
      <span className="parity-garage-name">{even ? fr.lessons.garageEven : fr.lessons.garageOdd}</span>
      <span className="parity-garage-numbers">
        {placed
          .filter((i) => parities[i] === even)
          .map((i) => (
            <motion.span key={i} className="parity-number parity-parked" initial={{ opacity: 0, scale: 0.6 }} animate={{ opacity: 1, scale: 1 }}>
              {numbers[i]}
            </motion.span>
          ))}
      </span>
    </button>
  );

  return (
    <div className="lesson-parity">
      <div className="parity-waiting" role="group" aria-label={fr.lessons.sortAsk}>
        {numbers.map((n, i) =>
          placed.includes(i) ? null : interactive ? (
            <button key={i} type="button" className={`parity-number${selected === i ? " parity-selected" : ""}`} aria-pressed={selected === i} onClick={() => setSelected(i)}>
              {n}
            </button>
          ) : (
            <span key={i} className="parity-number">
              {n}
            </span>
          ),
        )}
      </div>
      <div className="parity-garages">
        {garage(true)}
        {garage(false)}
      </div>
      {interactive && (
        <p className={done ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
          {done ? fr.lessons.sortDone : hint !== null ? fr.lessons.sortHint(hint) : selected !== null ? fr.lessons.sortPick(numbers[selected]) : fr.lessons.sortAsk}
        </p>
      )}
    </div>
  );
}
