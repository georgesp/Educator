import { motion } from "motion/react";
import { useState } from "react";
import type { FrameSlot, FriendPair } from "../../../api/client";
import { fr } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";
import { ColoredCar } from "./RaceObjects";

type TenFrameProps = {
  frames: number;
  slots: FrameSlot[];
  filled: number;
  added: number;
  removed: number;
  total: number;
  friends: FriendPair[];
  interactive: boolean;
  onComplete?(): void;
};

/** What a place shows. */
type Place = "empty" | "parked" | "added" | "gone";

const SIZE = 10;

function Garage({ places, onPick, small = false, label }: { places: Place[]; onPick?(index: number): void; small?: boolean; label: string }) {
  const move = useMove();
  return (
    <div className={small ? "ten-frame ten-frame-small" : "ten-frame"} role="group" aria-label={label}>
      {places.map((place, i) => {
        const car =
          place === "empty" ? null : (
            <motion.svg
              key={place}
              width="100%"
              height="100%"
              viewBox="0 0 48 24"
              className={place === "gone" ? "race-object-ghost" : undefined}
              initial={place === "added" ? { opacity: 0, x: -20 } : false}
              animate={{ opacity: 1, x: 0 }}
              transition={move}
              aria-hidden="true"
            >
              <ColoredCar color={place === "added" ? "blue" : "red"} />
            </motion.svg>
          );
        const clickable = onPick !== undefined && (place === "empty" || place === "parked");
        return clickable ? (
          <button key={i} type="button" className={`ten-place ten-place-button ten-place-${place}`} aria-label={fr.lessons.frameSlot(i + 1)} onClick={() => onPick(i)}>
            {car}
          </button>
        ) : (
          <span key={i} className={`ten-place ten-place-${place}`}>
            {car}
          </span>
        );
      })}
    </div>
  );
}

const placeOf = (slot: FrameSlot, revealed: boolean): Place => {
  switch (slot) {
    case "Empty":
      return "empty";
    case "Filled":
      return "parked";
    case "Added":
      return revealed ? "added" : "empty";
    case "Removed":
      return revealed ? "gone" : "parked";
  }
};

/**
 * `tenFrame`: the « garage de 10 places » (2 rows of 5), one or two garages filled in reading order.
 * The added cars come in a second color, the removed ones leave. Interactive: the child parks the
 * added cars or makes the removed ones leave. `friends`: the friends of 10, one small garage each.
 */
export function TenFrame({ frames, slots, filled, added, removed, total, friends, interactive, onComplete }: TenFrameProps) {
  const shown = useSteps(interactive || friends.length > 0 ? (interactive ? 0 : friends.length) : 1, slots);
  const [places, setPlaces] = useState<Place[]>(() => slots.map((s) => placeOf(s, false)));
  const [count, setCount] = useState(0);

  if (friends.length > 0) {
    return (
      <div className="lesson-frames lesson-friends">
        {friends.slice(0, Math.max(shown, 1)).map((pair) => (
          <figure key={pair.a} className="friend-pair">
            <Garage small places={Array.from({ length: SIZE }, (_, i) => (i < pair.a ? "parked" : "added"))} label={fr.lessons.friendPair(pair.a, pair.b)} />
            <figcaption>{fr.lessons.friendPair(pair.a, pair.b)}</figcaption>
          </figure>
        ))}
      </div>
    );
  }

  const goal = added > 0 ? added : removed;
  const done = interactive && count === goal;

  const pick = (index: number) => {
    if (done) {
      return;
    }

    if (added > 0 && places[index] === "empty") {
      // The car parks in the first empty place: the garage fills in reading order.
      const first = places.indexOf("empty");
      setPlaces(places.map((p, i) => (i === first ? "added" : p)));
    } else if (removed > 0 && places[index] === "parked") {
      setPlaces(places.map((p, i) => (i === index ? "gone" : p)));
    } else {
      return;
    }

    const next = count + 1;
    setCount(next);
    if (next === goal) {
      onComplete?.();
    }
  };

  const current = interactive ? places : slots.map((s) => placeOf(s, shown >= 1));
  const cars = current.filter((p) => p === "parked" || p === "added").length;

  return (
    <div className="lesson-frames">
      <div className="ten-frames">
        {Array.from({ length: frames }, (_, f) => (
          <Garage key={f} places={current.slice(f * SIZE, (f + 1) * SIZE)} onPick={interactive && !done ? (i) => pick(f * SIZE + i) : undefined} label={fr.lessons.frame(cars)} />
        ))}
      </div>
      {interactive ? (
        <p className={done ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
          {done
            ? added > 0
              ? fr.lessons.frameAddDone(filled, added, total)
              : fr.lessons.frameRemoveDone(filled, removed, total)
            : added > 0
              ? fr.lessons.frameAddAsk(added - count)
              : fr.lessons.frameRemoveAsk(removed - count)}
        </p>
      ) : (
        shown >= 1 &&
        (added > 0 || removed > 0) && (
          <p className="objects-caption">{added > 0 ? fr.lessons.frameSum(filled, added, total) : `${filled} ${fr.lessons.sign.Subtract} ${removed} = ${total}`}</p>
        )
      )}
    </div>
  );
}
