use educator_core::skills::{Date, PeriodStart, SchoolCalendar};

#[test]
fn default_calendar() {
    let cases = [
        (2026, 9, 1, 1),
        (2026, 10, 11, 1),
        (2026, 11, 3, 1),
        (2026, 11, 4, 2),
        (2026, 12, 31, 2),
        (2027, 1, 4, 2),
        (2027, 1, 5, 3),
        (2027, 3, 8, 3),
        (2027, 3, 9, 4),
        (2028, 2, 29, 3),
        (2027, 5, 4, 5),
        (2027, 7, 14, 5),
        (2027, 8, 31, 5),
    ];
    for (year, month, day, period) in cases {
        assert_eq!(
            period,
            SchoolCalendar::default().period_of(Date::new(year, month, day)),
            "{year}-{month}-{day}"
        );
    }
}

#[test]
fn periods_out_of_order_are_refused() {
    let starts = vec![
        PeriodStart::new(1, 9, 1),
        PeriodStart::new(2, 1, 5),
        PeriodStart::new(3, 11, 4),
        PeriodStart::new(4, 3, 9),
        PeriodStart::new(5, 5, 4),
    ];
    assert!(SchoolCalendar::new(starts).is_err());
}
