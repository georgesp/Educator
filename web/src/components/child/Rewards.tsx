import { motion } from "motion/react";
import type { Badge, DailyGoal, Level, SessionRewards } from "../../api/client";
import { fr } from "../../i18n/fr";
import { StarIcon, TargetIcon } from "../icons";

/** Up to 3 stars (SUB001502): the stars won in color, the others in outline. */
export function Stars({ count, size = 18, className }: { count: number; size?: number; className?: string }) {
  return (
    <span className={`stars ${className ?? ""}`} role="img" aria-label={fr.rewards.starsLabel(count)}>
      {[1, 2, 3].map((n) => (
        <svg key={n} width={size} height={size} viewBox="0 0 24 24" aria-hidden="true" className={n <= count ? "star star-won" : "star"}>
          <path d="M12 2l3 6.5 7 .8-5.2 4.8 1.4 7L12 17.6 5.8 21.1l1.4-7L2 9.3l7-.8z" />
        </svg>
      ))}
    </span>
  );
}

/** The level and the way to the next one (SUB001501). */
export function LevelGauge({ level }: { level: Level }) {
  const toNext = level.nextLevelAt - level.points;
  return (
    <div className="level-gauge">
      <div className="level-gauge-head">
        <span className="level-gauge-level">{fr.rewards.level(level.level)}</span>
        <span className="level-gauge-next">{fr.rewards.toNextLevel(toNext)}</span>
      </div>
      <div
        className="level-gauge-bar"
        role="progressbar"
        aria-label={fr.rewards.levelProgress(level.level + 1)}
        aria-valuemin={0}
        aria-valuemax={100}
        aria-valuenow={level.percent}
      >
        <motion.span className="level-gauge-fill" initial={false} animate={{ width: `${level.percent}%` }} transition={{ duration: 0.6 }} />
      </div>
    </div>
  );
}

/** The goal of the day (SUB001507): a few series, the days in a row; never anything negative. */
export function DailyGoalCard({ goal }: { goal: DailyGoal }) {
  if (goal.target === 0) {
    return null;
  }
  const done = Math.min(goal.done, goal.target);
  return (
    <div className={`daily-goal ${goal.reached ? "daily-goal-reached" : ""}`}>
      <div className="daily-goal-head">
        <TargetIcon size={22} />
        <span className="daily-goal-title">{fr.rewards.goalTitle}</span>
      </div>
      <p className="daily-goal-text">{goal.reached ? fr.rewards.goalReached : fr.rewards.goalProgress(done, goal.target)}</p>
      <div className="daily-goal-steps" aria-hidden="true">
        {Array.from({ length: goal.target }, (_, i) => (
          <span key={i} className={i < done ? "daily-goal-step daily-goal-step-done" : "daily-goal-step"} />
        ))}
      </div>
      {goal.streak > 1 && <p className="daily-goal-streak">{fr.rewards.streak(goal.streak)}</p>}
    </div>
  );
}

/** A badge: in color once earned, in grey with its condition until then. */
export function BadgeIcon({ badge, size }: { badge: Badge; size: number }) {
  return (
    <img
      className={badge.earned ? "badge-icon" : "badge-icon badge-icon-locked"}
      src={`data:image/svg+xml;charset=utf-8,${encodeURIComponent(badge.icon)}`}
      width={size}
      height={size}
      alt=""
    />
  );
}

/**
 * What a session brought (lot 3), at its end: the stars, the bonus of the goal of the day, the new
 * level and what it unlocks, the new badges.
 */
export function RewardsPanel({ rewards }: { rewards: SessionRewards | null }) {
  if (!rewards) {
    return null;
  }
  const levelUp = rewards.level.level > rewards.levelBefore.level;
  const betterStars = rewards.stars !== null && rewards.stars > rewards.previousBestStars && rewards.previousBestStars > 0;
  return (
    <div className="rewards-panel">
      {rewards.stars !== null && rewards.stars > 0 && (
        <motion.div className="rewards-stars" initial={{ scale: 0.4, opacity: 0 }} animate={{ scale: 1, opacity: 1 }} transition={{ delay: 0.35, type: "spring", stiffness: 240, damping: 12 }}>
          <Stars count={rewards.stars} size={44} />
          {betterStars && <p className="rewards-note">{fr.rewards.betterStars}</p>}
        </motion.div>
      )}
      {rewards.goalBonus > 0 && (
        <p className="rewards-goal" role="status">
          <TargetIcon size={22} /> {fr.rewards.goalBonus(rewards.goalBonus)}
        </p>
      )}
      {levelUp && (
        <motion.div
          className="rewards-level-up"
          role="status"
          initial={{ scale: 0.6, rotate: -4, opacity: 0 }}
          animate={{ scale: 1, rotate: 0, opacity: 1 }}
          transition={{ delay: 0.6, type: "spring", stiffness: 220, damping: 10 }}
        >
          <StarIcon size={28} />
          <span>{fr.rewards.levelUp(rewards.level.level)}</span>
        </motion.div>
      )}
      {rewards.newAccessories.length > 0 && (
        <p className="rewards-unlocked" role="status">
          {fr.rewards.unlocked(rewards.newAccessories.map((a) => a.name))}
        </p>
      )}
      {rewards.newBadges.length > 0 && (
        <div className="rewards-badges" role="status">
          <p className="rewards-badges-title">{fr.rewards.newBadges(rewards.newBadges.length)}</p>
          <ul className="plain-list rewards-badge-list">
            {rewards.newBadges.map((badge) => (
              <motion.li key={badge.id} initial={{ y: 12, opacity: 0 }} animate={{ y: 0, opacity: 1 }} transition={{ delay: 0.8 }}>
                <BadgeIcon badge={badge} size={64} />
                <span>{badge.title}</span>
              </motion.li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}
