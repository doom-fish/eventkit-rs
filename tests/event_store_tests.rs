use eventkit::prelude::*;

#[test]
fn event_store_can_be_created_and_dropped() {
    let identifier = {
        let store = EKEventStore::new().expect("store");
        store.event_store_identifier().expect("identifier")
    };
    assert!(!identifier.is_empty());
}

#[test]
fn event_store_supports_non_mutating_maintenance_calls() {
    let store = EKEventStore::new().expect("store");
    let identifier = store.event_store_identifier().expect("identifier");
    store.reset();
    store.refresh_sources_if_necessary();
    assert_eq!(store.event_store_identifier().expect("identifier"), identifier);
}

#[test]
fn unknown_calendar_identifiers_are_rejected() {
    let store = EKEventStore::new().expect("store");
    let missing = "doom-fish.eventkit-tests.missing-calendar";
    let events = store.events_matching(
        &EKEventPredicate::new("2026-01-01T00:00:00Z", "2026-01-02T00:00:00Z")
            .with_calendar_identifiers([missing.to_owned()]),
    );
    assert!(matches!(events, Err(EventKitError::InvalidArgument(message)) if message.contains(missing)));
    let reminders = store
        .fetch_reminders_matching(
        &EKReminderPredicate::new().with_calendar_identifiers([missing.to_owned()]),
    );
    assert!(matches!(reminders, Err(EventKitError::InvalidArgument(_))));
}

#[test]
fn an_empty_calendar_filter_matches_nothing() {
    let store = EKEventStore::new().expect("store");
    let events = store
        .events_matching(
            &EKEventPredicate::new("2000-01-01T00:00:00Z", "2030-01-01T00:00:00Z")
                .with_calendar_identifiers(Vec::<String>::new()),
        )
        .expect("events");
    assert!(events.is_empty());
    let reminders = store
        .fetch_reminders_matching(
            &EKReminderPredicate::new().with_calendar_identifiers(Vec::<String>::new()),
        )
        .expect("reminders");
    assert!(reminders.is_empty());
}

#[test]
fn events_matching_covers_ranges_longer_than_four_years() {
    let store = EKEventStore::new().expect("store");
    let recent = store
        .events_matching(&EKEventPredicate::new(
            "2026-06-01T00:00:00Z",
            "2026-12-31T00:00:00Z",
        ))
        .expect("recent events");
    let long = store
        .events_matching(&EKEventPredicate::new(
            "2020-01-01T00:00:00Z",
            "2027-01-01T00:00:00Z",
        ))
        .expect("seven years of events");
    for event in &recent {
        assert!(
            long.iter().any(|candidate| {
                candidate.calendar_item_identifier == event.calendar_item_identifier
                    && candidate.occurrence_date == event.occurrence_date
            }),
            "an event after the first four years is missing from the long range"
        );
    }
    let mut keys = long
        .iter()
        .map(|event| (event.calendar_item_identifier.clone(), event.occurrence_date.clone()))
        .collect::<Vec<_>>();
    let total = keys.len();
    keys.sort();
    keys.dedup();
    assert_eq!(keys.len(), total, "events spanning a chunk boundary are returned twice");
}
