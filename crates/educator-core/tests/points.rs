use educator_core::adaptive::AdaptiveRules;
use educator_core::points::{all_mastered, apply_percent, pace, Pace, PointsRules, USUAL_PERCENT};
use educator_core::sessions::SkillMastery;
use uuid::Uuid;

const RULES: PointsRules = PointsRules::new(10, 20, 20);

#[test]
fn completed_series() {
    for (correct, points) in [(10, 140), (9, 110), (0, 20)] {
        assert_eq!(
            points,
            RULES.for_completed_series(correct, 10),
            "{correct} correct"
        );
    }
}

#[test]
fn abandoned_series_keeps_correct_answers_without_bonus() {
    assert_eq!(0, RULES.for_abandoned_series(0));
    assert_eq!(40, RULES.for_abandoned_series(4));
}

// Percent of the points by period.

#[test]
fn the_percents_of_the_periods_are_50_and_150_by_default() {
    assert_eq!(USUAL_PERCENT, RULES.percent(Pace::Current));
    assert_eq!(50, RULES.percent(Pace::Revision));
    assert_eq!(150, RULES.percent(Pace::Ahead));
    let other = RULES.with_period_percents(80, 120);
    assert_eq!(
        (80, 120),
        (other.percent(Pace::Revision), other.percent(Pace::Ahead))
    );
}

#[test]
fn a_percent_is_rounded_to_the_nearest_point() {
    assert_eq!(70, apply_percent(140, 50));
    assert_eq!(210, apply_percent(140, 150));
    assert_eq!(140, apply_percent(140, USUAL_PERCENT));
    assert_eq!(8, apply_percent(15, 50));
    assert_eq!(0, apply_percent(0, 150));
}

#[test]
fn ahead_of_the_year_earns_more_and_a_past_period_mastered_less() {
    assert_eq!(Pace::Ahead, pace(Some(3), 2, false));
    assert_eq!(Pace::Ahead, pace(Some(3), 2, true));
    assert_eq!(Pace::Current, pace(Some(2), 2, true));
    assert_eq!(Pace::Revision, pace(Some(1), 2, true));
    // A past period not mastered yet: the usual points.
    assert_eq!(Pace::Current, pace(Some(1), 2, false));
    // Not one of the grade's own skills (a revision of the grade before, another grade).
    assert_eq!(Pace::Current, pace(None, 5, true));
}

fn mastery(skill_id: &str, score: u8, attempt_count: u32) -> SkillMastery {
    SkillMastery {
        profile_id: Uuid::nil(),
        skill_id: skill_id.to_owned(),
        score,
        attempt_count,
        last_practiced_at: 0,
    }
}

#[test]
fn mastered_needs_enough_answers_and_the_score_of_the_advice() {
    let rules = AdaptiveRules::DEFAULT;
    let ids = |ids: &[&str]| ids.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
    let masteries = [
        mastery("a", 90, 12),
        mastery("b", 84, 30),
        mastery("c", 100, 9),
        mastery("d", 85, 10),
    ];
    assert!(all_mastered(&ids(&["a"]), &masteries, &rules));
    assert!(all_mastered(&ids(&["a", "d"]), &masteries, &rules));
    assert!(!all_mastered(&ids(&["b"]), &masteries, &rules));
    assert!(!all_mastered(&ids(&["c"]), &masteries, &rules));
    assert!(!all_mastered(&ids(&["a", "b"]), &masteries, &rules));
    assert!(!all_mastered(
        &ids(&["never-practised"]),
        &masteries,
        &rules
    ));
    assert!(!all_mastered(&[], &masteries, &rules));
}
