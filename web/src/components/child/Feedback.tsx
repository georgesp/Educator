import { motion } from "motion/react";
import { useEffect } from "react";
import { playSound } from "../../audio/sounds";

type FeedbackProps = {
  kind: "correct" | "incorrect";
  message: string;
  /** Correction shown after a mistake, e.g. « 7 + 5 = 12 ». */
  detail?: string;
};

/** Reaction after an answer: short animation and sound, never punitive. */
export function Feedback({ kind, message, detail }: FeedbackProps) {
  useEffect(() => playSound(kind), [kind]);

  return (
    <motion.div
      role="status"
      className={`feedback feedback-${kind}`}
      initial={{ opacity: 0, scale: 0.8, y: 8 }}
      animate={{ opacity: 1, scale: 1, y: 0 }}
      transition={{ duration: 0.25, ease: "easeOut" }}
    >
      <span className="feedback-message">{message}</span>
      {detail && <span className={detail.length > 24 ? "feedback-detail feedback-detail-long" : "feedback-detail"}>{detail}</span>}
    </motion.div>
  );
}
