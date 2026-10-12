import { motion } from "motion/react";
import { fr } from "../../../i18n/fr";
import { FlagIcon } from "../../icons";
import { useMove } from "../useSteps";

/** `titleCard`: checkered strip, the lesson title, and the car crossing the line. */
export function TitleCard({ title, subtitle }: { title: string; subtitle: string | null }) {
  const move = useMove();
  return (
    <div className="lesson-title-card">
      <div className="lesson-checker" aria-hidden="true" />
      <motion.h2 className="lesson-title-card-title" initial={{ opacity: 0, x: -40 }} animate={{ opacity: 1, x: 0 }} transition={move}>
        {title}
      </motion.h2>
      {subtitle && <p className="lesson-title-card-subtitle">{subtitle}</p>}
      <motion.svg
        className="lesson-title-car"
        width="120"
        height="60"
        viewBox="0 0 48 24"
        aria-hidden="true"
        initial={{ x: -160, opacity: 0 }}
        animate={{ x: 0, opacity: 1 }}
        transition={{ ...move, delay: 0.15 }}
      >
        <CarShape />
      </motion.svg>
      <div className="lesson-checker" aria-hidden="true" />
    </div>
  );
}

/** `recapCard`: « Je retiens », the rule in a racing-red frame, with an example. */
export function RecapCard({ text, example }: { text: string; example: string | null }) {
  const move = useMove();
  return (
    <motion.div className="lesson-recap" initial={{ opacity: 0, scale: 0.94 }} animate={{ opacity: 1, scale: 1 }} transition={move}>
      <div className="lesson-recap-label">
        <FlagIcon size={28} />
        <span>{fr.lessons.recap}</span>
      </div>
      <p className="lesson-recap-text">{text}</p>
      {example && <p className="lesson-recap-example">{example}</p>}
    </motion.div>
  );
}

/** The car of the race track (RaceTrack), drawn in a 48 × 24 box. */
export function CarShape() {
  return (
    <>
      <path d="M2 17l4-6h14l7-5h10l9 7v4z" fill="var(--primary)" />
      <path d="M28 7h8l4 4H24z" fill="#BFE3F5" />
      <circle cx="12" cy="18" r="4.5" fill="var(--text)" />
      <circle cx="37" cy="18" r="4.5" fill="var(--text)" />
    </>
  );
}
