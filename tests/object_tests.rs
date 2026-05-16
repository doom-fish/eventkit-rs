use eventkit::prelude::*;

#[test]
fn ek_object_wraps_live_eventkit_objects() {
    let store = EKEventStore::new().expect("store");

    let event_object = EKEvent::new(
        "Object wrapper demo",
        "2026-05-16T10:00:00Z",
        "2026-05-16T11:00:00Z",
    )
    .as_object_in(&store)
    .expect("event object");
    assert!(event_object.is_new());
    assert!(event_object.has_changes());
    assert!(!event_object.refresh());
    event_object.rollback();
    assert!(event_object.has_changes());
    event_object.reset();
    assert!(!event_object.has_changes());

    let reminder_object = EKReminder::new("Object wrapper reminder")
        .as_object_in(&store)
        .expect("reminder object");
    assert!(reminder_object.is_new());
    assert!(reminder_object.has_changes());

    let calendar_object = EKCalendarDraft::new(EKEntityType::Event, "Object wrapper calendar")
        .as_object_in(&store)
        .expect("calendar object");
    assert!(calendar_object.is_new());
    assert!(calendar_object.has_changes());
}
