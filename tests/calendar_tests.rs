use eventkit::prelude::*;

#[test]
fn calendar_draft_roundtrip_preserves_title() {
    let store = EKEventStore::new().expect("store");
    let draft = EKCalendarDraft::new(EKEntityType::Event, "Example Calendar");
    let roundtrip = draft.roundtrip(&store).expect("roundtrip");
    assert_eq!(roundtrip.title, "Example Calendar");
}

#[test]
fn calendar_draft_roundtrip_preserves_color() {
    let store = EKEventStore::new().expect("store");
    let mut draft = EKCalendarDraft::new(EKEntityType::Event, "Example Calendar");
    draft.color = Some("#FF8000FF".to_owned());
    let roundtrip = draft.roundtrip(&store).expect("roundtrip");
    assert_eq!(roundtrip.color.as_deref(), Some("#FF8000FF"));
}
