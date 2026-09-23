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

fn assert_invalid(rule: &EKRecurrenceRule, expected: &str) {
    match rule.roundtrip() {
        Err(EventKitError::InvalidArgument(message)) => {
            assert!(message.contains(expected), "{message}");
        }
        other => panic!("expected an invalid-argument error mentioning {expected:?}, got {other:?}"),
    }
}

#[test]
fn recurrence_rule_rejects_values_that_raise_in_eventkit() {
    let daily = EKRecurrenceRule::new(EKRecurrenceFrequency::Daily);
    assert_invalid(&daily.clone().with_interval(0), "interval");
    assert_invalid(&daily.clone().with_interval(-3), "interval");
    assert_invalid(&daily.with_occurrence_count(0), "occurrence count");

    let weekly = EKRecurrenceRule::new(EKRecurrenceFrequency::Weekly).with_days_of_the_week([
        EKRecurrenceDayOfWeek::new(EKWeekday::Monday).with_week_number(1),
    ]);
    assert_invalid(&weekly, "week number of 0");

    let monthly = EKRecurrenceRule::new(EKRecurrenceFrequency::Monthly);
    for week_number in [54, -54, i64::MIN] {
        assert_invalid(
            &monthly.clone().with_days_of_the_week([
                EKRecurrenceDayOfWeek::new(EKWeekday::Friday).with_week_number(week_number),
            ]),
            "week number",
        );
    }
    assert_invalid(&monthly.clone().with_days_of_the_month([32]), "days of the month");
    assert_invalid(&monthly.with_days_of_the_month([0]), "days of the month");

    let yearly = EKRecurrenceRule::new(EKRecurrenceFrequency::Yearly);
    assert_invalid(&yearly.clone().with_months_of_the_year([13]), "months of the year");
    assert_invalid(&yearly.clone().with_months_of_the_year([-1]), "months of the year");
    assert_invalid(&yearly.clone().with_weeks_of_the_year([54]), "weeks of the year");
    assert_invalid(&yearly.clone().with_days_of_the_year([-367]), "days of the year");
    assert_invalid(&yearly.with_set_positions([0]), "set positions");
}

#[test]
fn recurrence_rule_accepts_boundary_values() {
    let rule = EKRecurrenceRule::new(EKRecurrenceFrequency::Yearly)
        .with_interval(2)
        .with_occurrence_count(1)
        .with_days_of_the_week([
            EKRecurrenceDayOfWeek::new(EKWeekday::Sunday).with_week_number(53),
            EKRecurrenceDayOfWeek::new(EKWeekday::Monday).with_week_number(-53),
        ])
        .with_days_of_the_month([31, -31])
        .with_months_of_the_year([1, 12])
        .with_weeks_of_the_year([53, -53])
        .with_days_of_the_year([366, -366])
        .with_set_positions([1, -366]);
    let roundtrip = rule.roundtrip().expect("roundtrip");
    assert_eq!(roundtrip.interval, 2);
    assert_eq!(roundtrip.occurrence_count, Some(1));
    assert_eq!(roundtrip.months_of_the_year, vec![1, 12]);
    assert_eq!(roundtrip.days_of_the_year, vec![366, -366]);
}

#[test]
fn events_with_invalid_recurrence_rules_are_rejected() {
    let store = EKEventStore::new().expect("store");
    let mut event = EKEvent::new("Demo", "2026-01-01T10:00:00Z", "2026-01-01T11:00:00Z");
    event.recurrence_rules = vec![EKRecurrenceRule::new(EKRecurrenceFrequency::Daily).with_interval(0)];
    assert!(matches!(
        event.roundtrip_in(&store),
        Err(EventKitError::InvalidArgument(_))
    ));

    let mut reminder = EKReminder::new("Demo");
    reminder.recurrence_rules =
        vec![EKRecurrenceRule::new(EKRecurrenceFrequency::Weekly).with_occurrence_count(0)];
    assert!(matches!(
        reminder.roundtrip_in(&store),
        Err(EventKitError::InvalidArgument(_))
    ));
}
