import { animate, motion, useMotionValue, useTransform } from "motion/react";
import { useEffect } from "react";
import { fr } from "../../i18n/fr";

/** Points of the profile; the number counts up when it increases. */
export function PointsBadge({ points, label = fr.menu.points }: { points: number; label?: string }) {
  const value = useMotionValue(points);
  const rounded = useTransform(value, (v) => Math.round(v).toString());

  useEffect(() => {
    const controls = animate(value, points, { duration: 0.5, ease: "easeOut" });
    return () => controls.stop();
  }, [points, value]);

  return (
    <span className="points-badge" aria-label={`${points} ${label.toLowerCase()}`}>
      <svg width="22" height="22" viewBox="0 0 24 24" aria-hidden="true">
        <path d="M12 2l3 6.5 7 .8-5.2 4.8 1.4 7L12 17.6 5.8 21.1l1.4-7L2 9.3l7-.8z" fill="currentColor" />
      </svg>
      <motion.span aria-hidden="true">{rounded}</motion.span>
    </span>
  );
}
