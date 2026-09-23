use std::cmp::Ordering;

use eventkit::prelude::*;

#[test]
fn events_compare_by_start_date() {
    let earlier = EKEvent::new("Earlier", "2026-01-01T09:00:00Z", "2026-01-01T10:00:00Z");
    let later = EKEvent::new("Later", "2026-01-01T11:00:00Z", "2026-01-01T12:00:00Z");
    assert_eq!(
        earlier.compare_start_date(&later).expect("compare"),
        Ordering::Less
    );
}

#[test]
fn event_roundtrip_preserves_core_fields() {
    let store = EKEventStore::new().expect("store");
    let event = EKEvent::new("Demo", "2026-01-01T10:00:00Z", "2026-01-01T11:00:00Z")
        .with_structured_location(EKStructuredLocation::new("HQ"));
    let roundtrip = event.roundtrip_in(&store).expect("roundtrip");
    assert_eq!(roundtrip.title, "Demo");
    assert_eq!(
        roundtrip
            .structured_location
            .and_then(|location| location.title),
        Some("HQ".into())
    );
}

#[test]
fn event_roundtrip_keeps_alarms_and_rejects_undecodable_ones() {
    let store = EKEventStore::new().expect("store");
    let mut event = EKEvent::new("Demo", "2026-01-01T10:00:00Z", "2026-01-01T11:00:00Z");
    event.alarms = vec![
        EKAlarm::relative(-900.0),
        EKAlarm::absolute("2026-01-01T09:30:00Z"),
    ];
    event.recurrence_rules = vec![EKRecurrenceRule::new(EKRecurrenceFrequency::Weekly)];
    let roundtrip = event.roundtrip_in(&store).expect("roundtrip");
    assert_eq!(roundtrip.alarms.len(), 2);
    assert!(roundtrip
        .alarms
        .iter()
        .any(|alarm| alarm.absolute_date.is_none() && alarm.relative_offset == Some(-900.0)));
    assert!(roundtrip
        .alarms
        .iter()
        .any(|alarm| alarm.absolute_date.as_deref() == Some("2026-01-01T09:30:00.000Z")));
    assert_eq!(roundtrip.recurrence_rules.len(), 1);

    event.alarms.push(EKAlarm::absolute("not a date"));
    assert!(event.roundtrip_in(&store).is_err());
}
