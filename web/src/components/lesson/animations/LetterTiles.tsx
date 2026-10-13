import { motion } from "motion/react";
import { useRef, useState } from "react";
import { fr } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";

type LetterTilesProps = {
  word: string;
  /** The tiles of the word, in order. */
  tiles: string[];
  distractors: string[];
  /** Every tile as shown, mixed up by the core: indexes in `tiles`, then in `distractors`. */
  order: number[];
  interactive: boolean;
  onComplete?(): void;
};

/** A point of the page (`pageX`, `pageY`, as given by a drag) inside a box of the viewport. */
export function isInside(point: { x: number; y: number }, box: DOMRect, scroll = { x: window.scrollX, y: window.scrollY }): boolean {
  const x = point.x - scroll.x;
  const y = point.y - scroll.y;
  return x >= box.left && x <= box.right && y >= box.top && y <= box.bottom;
}

/**
 * `letterTiles` (SUB001410): the letters or graphemes of a word, mixed up with a few intruders. Shown
 * only, the tiles go to their place one after the other. Interactive: the child drags them onto the
 * word, or clicks them, in order; a wrong tile shakes and goes back to its place.
 */
export function LetterTiles({ word, tiles, distractors, order, interactive, onComplete }: LetterTilesProps) {
  const move = useMove();
  const shown = useSteps(interactive ? 0 : tiles.length, word);
  const [placed, setPlaced] = useState<number[]>([]);
  const [wrong, setWrong] = useState<number | null>(null);
  const texts = [...tiles, ...distractors];
  const built = interactive ? placed.length : shown;
  const done = interactive && placed.length === tiles.length;
  const target = useRef<HTMLParagraphElement | null>(null);
  // A drag ends with a click on some browsers: it must not count twice.
  const dragging = useRef(false);

  const pick = (index: number) => {
    if (done || placed.includes(index)) {
      return;
    }

    // Two tiles with the same text are worth the same: the text decides.
    if (texts[index] === tiles[placed.length]) {
      const next = [...placed, index];
      setPlaced(next);
      setWrong(null);
      if (next.length === tiles.length) {
        onComplete?.();
      }
    } else {
      setWrong(index);
    }
  };

  return (
    <div className="letter-tiles">
      <p ref={target} className="letter-tiles-word" aria-label={fr.lessons.tilesWord(built === tiles.length ? word : "")}>
        {tiles.map((tile, i) => (
          <span key={i} className={i < built ? "letter-slot letter-slot-filled" : "letter-slot"}>
            {i < built && (
              <motion.span initial={{ opacity: 0, y: 30 }} animate={{ opacity: 1, y: 0 }} transition={move}>
                {tile}
              </motion.span>
            )}
          </span>
        ))}
      </p>
      {interactive ? (
        <div className="letter-tiles-pool" role="group" aria-label={fr.lessons.tilesPool}>
          {order.map((index) => {
            const used = placed.includes(index);
            return (
              <motion.button
                key={index}
                type="button"
                className={`letter-tile ${used ? "letter-tile-used" : ""} ${wrong === index ? "letter-tile-wrong" : ""}`}
                disabled={used || done}
                aria-label={fr.lessons.tile(texts[index])}
                onClick={() => {
                  if (!dragging.current) {
                    pick(index);
                  }
                }}
                drag={!used && !done}
                dragSnapToOrigin
                dragMomentum={false}
                whileDrag={{ scale: 1.15, zIndex: 2 }}
                onDragStart={() => {
                  dragging.current = true;
                }}
                onDragEnd={(_, info) => {
                  window.setTimeout(() => {
                    dragging.current = false;
                  }, 0);
                  const box = target.current?.getBoundingClientRect();
                  if (box && isInside(info.point, box)) {
                    pick(index);
                  }
                }}
                animate={wrong === index ? { x: [0, -6, 6, -4, 4, 0] } : { x: 0 }}
                transition={{ duration: 0.35 }}
              >
                {texts[index]}
              </motion.button>
            );
          })}
        </div>
      ) : (
        distractors.length > 0 && (
          <div className="letter-tiles-pool" aria-hidden="true">
            {distractors.map((d) => (
              <span key={d} className="letter-tile letter-tile-used">
                {d}
              </span>
            ))}
          </div>
        )
      )}
      {interactive && (
        <p className={done ? "lesson-hint lesson-hint-done" : "lesson-hint"} role="status">
          {done ? fr.lessons.tilesDone(word) : wrong !== null ? fr.lessons.tilesHint : fr.lessons.tilesAsk}
        </p>
      )}
    </div>
  );
}
