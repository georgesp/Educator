import { motion } from "motion/react";

type RaceTrackProps = {
  /** Number of exercises already answered. */
  done: number;
  total: number;
  /** Correctness of each answered exercise, to color its marker. */
  results: readonly (boolean | null)[];
  label: string;
};

/** Progress of a series: a car drives along the track to the checkered flag, one marker per exercise. */
export function RaceTrack({ done, total, results, label }: RaceTrackProps) {
  const progress = total === 0 ? 0 : done / total;
  return (
    <div className="race-track" role="progressbar" aria-label={label} aria-valuemin={0} aria-valuemax={total} aria-valuenow={done}>
      <div className="race-markers" aria-hidden="true" style={{ gridTemplateColumns: `repeat(${Math.max(total, 1)}, minmax(0, 1fr))` }}>
        {Array.from({ length: total }, (_, i) => (
          <span key={i} className={`race-marker ${results[i] === true ? "race-marker-right" : results[i] === false ? "race-marker-wrong" : ""}`} />
        ))}
      </div>
      <div className="race-finish" aria-hidden="true" />
      <motion.svg
        className="race-car"
        width="72"
        height="36"
        viewBox="0 0 48 24"
        aria-hidden="true"
        initial={false}
        animate={{ left: `calc(${progress * 100}% * 0.9)` }}
        transition={{ type: "spring", stiffness: 120, damping: 18 }}
      >
        <path d="M2 17l4-6h14l7-5h10l9 7v4z" fill="var(--primary)" />
        <path d="M28 7h8l4 4H24z" fill="#BFE3F5" />
        <circle cx="12" cy="18" r="4.5" fill="var(--text)" />
        <circle cx="37" cy="18" r="4.5" fill="var(--text)" />
      </motion.svg>
    </div>
  );
}
