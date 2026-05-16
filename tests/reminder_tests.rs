use eventkit::prelude::*;

#[test]
fn reminder_roundtrip_preserves_priority() {
    let store = EKEventStore::new().expect("store");
    let reminder = EKReminder::new("Ship 0.2.1").with_priority_kind(EKReminderPriority::High);
    let roundtrip = reminder.roundtrip_in(&store).expect("roundtrip");
    assert_eq!(roundtrip.priority_kind(), EKReminderPriority::High);
}

#[test]
fn reminder_date_components_helper_sets_values() {
    let components = NSDateComponents::date(2026, 1, 15).with_time(9, 30, 0);
    assert_eq!(components.year, Some(2026));
    assert_eq!(components.hour, Some(9));
}
