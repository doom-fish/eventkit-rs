use eventkit::prelude::*;

#[test]
fn recurrence_rule_roundtrip_preserves_weekday_rules() {
    let rule = EKRecurrenceRule::new(EKRecurrenceFrequency::Weekly)
        .with_days_of_the_week([EKRecurrenceDayOfWeek::new(EKWeekday::Monday)])
        .with_set_positions([1]);
    let roundtrip = rule.roundtrip().expect("roundtrip");
    assert_eq!(roundtrip.frequency, EKRecurrenceFrequency::Weekly);
    assert_eq!(roundtrip.days_of_the_week.len(), 1);
    assert_eq!(roundtrip.set_positions, vec![1]);
}
