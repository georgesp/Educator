//! Levels of a profile (SUB001501): computed from the points earned, with a progressive scale read
//! from `content/rewards/levels.json`. Points are never taken away, so a level is never lost; the
//! level is never stored: an existing profile gets the right level from its points.

/// Points needed for each level, from level 1 (0 points), then a fixed step after the last one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelScale {
    /// `thresholds[n]`: points from which level `n + 1` is reached (`thresholds[0]` is 0).
    thresholds: Vec<i32>,
    /// Points between two levels after the last threshold (the scale never ends).
    step_after: i32,
}

/// Where a profile stands on the scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelProgress {
    /// From 1.
    pub level: u32,
    pub points: i32,
    /// Points from which the current level was reached.
    pub level_start: i32,
    /// Points from which the next level is reached.
    pub next_level_at: i32,
}

impl LevelProgress {
    /// Share of the way to the next level, from 0 to 100.
    pub fn percent(&self) -> u8 {
        let span = i64::from(self.next_level_at - self.level_start).max(1);
        let done = i64::from(self.points - self.level_start).clamp(0, span);
        (done * 100 / span) as u8
    }
}

impl LevelScale {
    /// `thresholds` start at 0 and grow strictly; `step_after` is positive.
    pub fn new(thresholds: Vec<i32>, step_after: i32) -> Result<Self, String> {
        if thresholds.first() != Some(&0) {
            return Err("The first level starts at 0 points.".to_owned());
        }
        if thresholds.windows(2).any(|w| w[0] >= w[1]) {
            return Err("The points of the levels must grow.".to_owned());
        }
        if step_after <= 0 {
            return Err("The step after the last level must be positive.".to_owned());
        }
        Ok(Self {
            thresholds,
            step_after,
        })
    }

    /// The scale of the content file, used when the file is invalid.
    pub fn default_scale() -> Self {
        Self {
            thresholds: vec![
                0, 100, 250, 450, 700, 1000, 1400, 1900, 2500, 3200, 4000, 5000, 6200, 7600, 9200,
                11000, 13000, 15500, 18000, 21000,
            ],
            step_after: 3000,
        }
    }

    /// Points from which `level` (from 1) is reached.
    pub fn points_for(&self, level: u32) -> i32 {
        let index = level.max(1) as usize - 1;
        match self.thresholds.get(index) {
            Some(points) => *points,
            None => {
                let last = *self.thresholds.last().unwrap_or(&0);
                let beyond = i32::try_from(index + 1 - self.thresholds.len()).unwrap_or(i32::MAX);
                last.saturating_add(beyond.saturating_mul(self.step_after))
            }
        }
    }

    /// The level reached with `points`.
    pub fn level_of(&self, points: i32) -> u32 {
        let points = points.max(0);
        let listed = self.thresholds.iter().filter(|t| **t <= points).count();
        if listed < self.thresholds.len() {
            return u32::try_from(listed).unwrap_or(u32::MAX);
        }
        let last = *self.thresholds.last().unwrap_or(&0);
        let beyond = (points - last) / self.step_after;
        u32::try_from(listed).unwrap_or(u32::MAX) + u32::try_from(beyond).unwrap_or(0)
    }

    pub fn progress(&self, points: i32) -> LevelProgress {
        let level = self.level_of(points);
        LevelProgress {
            level,
            points: points.max(0),
            level_start: self.points_for(level),
            next_level_at: self.points_for(level + 1),
        }
    }
}

impl Default for LevelScale {
    fn default() -> Self {
        Self::default_scale()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_follow_the_thresholds_then_the_step() {
        let scale = LevelScale::new(vec![0, 100, 250], 200).unwrap();
        assert_eq!(1, scale.level_of(0));
        assert_eq!(1, scale.level_of(99));
        assert_eq!(2, scale.level_of(100));
        assert_eq!(3, scale.level_of(250));
        assert_eq!(3, scale.level_of(449));
        assert_eq!(4, scale.level_of(450));
        assert_eq!(5, scale.level_of(650));
        assert_eq!(650, scale.points_for(5));
        assert_eq!(
            LevelProgress {
                level: 2,
                points: 175,
                level_start: 100,
                next_level_at: 250
            },
            scale.progress(175)
        );
        assert_eq!(50, scale.progress(175).percent());
    }

    #[test]
    fn a_bad_scale_is_refused() {
        assert!(LevelScale::new(vec![10, 100], 100).is_err());
        assert!(LevelScale::new(vec![0, 100, 100], 100).is_err());
        assert!(LevelScale::new(vec![0, 100], 0).is_err());
    }
}
