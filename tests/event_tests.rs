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
