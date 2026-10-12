use educator_core::points::PointsRules;

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
