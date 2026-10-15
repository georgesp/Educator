import { useQuery } from "@tanstack/react-query";
import { motion, useReducedMotion, type TargetAndTransition } from "motion/react";
import { useState } from "react";
import type { MascotExpression, MascotSituation } from "../../api/client";
import { useApi } from "../../app/BackendContext";
import { useMascotHidden } from "../../app/settings";
import { SpeakButton } from "./SpeakButton";

/** A small move of each expression (Framer Motion), replaced by a fade when motion is reduced. */
const MOVES: Record<MascotExpression, TargetAndTransition> = {
  Happy: { y: [0, -6, 0], transition: { duration: 1.6, repeat: Infinity, ease: "easeInOut" } },
  Thinking: { rotate: [0, -4, 0, 4, 0], transition: { duration: 3, repeat: Infinity, ease: "easeInOut" } },
  Encouraging: { rotate: [0, 6, 0], transition: { duration: 0.9, repeat: 2, ease: "easeInOut" } },
  Party: { y: [0, -18, 0], scale: [1, 1.05, 1], transition: { duration: 0.6, repeat: 3, ease: "easeOut" } },
};

type MascotProps = {
  situation: MascotSituation;
  /** The lesson presented, at its start. */
  title?: string;
  size?: number;
};

let mount = 0;

/**
 * The mascot (SUB001417): Plume the owl says a phrase of the situation, in a bubble, read on click
 * of its speaker (never by itself). Hidden from the Options; absent without a valid mascot in the
 * content.
 */
export function Mascot({ situation, title, size = 120 }: MascotProps) {
  const api = useApi();
  const hidden = useMascotHidden();
  const reduce = useReducedMotion() ?? false;
  // One phrase per appearance: drawn again when the mascot comes back.
  const [appearance] = useState(() => (mount += 1));
  const mascot = useQuery({ queryKey: ["mascot"], queryFn: () => api.getMascot(), staleTime: Infinity, enabled: !hidden });
  const phrase = useQuery({
    queryKey: ["mascot-phrase", situation, title ?? null, appearance],
    queryFn: () => api.getMascotPhrase(situation, title),
    staleTime: Infinity,
    gcTime: 0,
    enabled: !hidden,
  });
  if (hidden || !mascot.data || !phrase.data) {
    return null;
  }

  const picture = mascot.data.pictures.find((p) => p.expression === phrase.data!.expression) ?? mascot.data.pictures[0];
  return (
    <motion.figure className="mascot" initial={{ opacity: 0, y: 12 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.3 }} data-testid="mascot">
      <motion.img
        className="mascot-picture"
        src={`data:image/svg+xml;charset=utf-8,${encodeURIComponent(picture.svg)}`}
        width={size}
        height={size}
        alt={mascot.data.name}
        animate={reduce ? undefined : MOVES[picture.expression]}
      />
      <figcaption className="mascot-bubble" role="status">
        {phrase.data.text}
        <SpeakButton text={phrase.data.text} compact />
      </figcaption>
    </motion.figure>
  );
}
