use eventkit::prelude::*;

#[test]
fn calendar_draft_roundtrip_preserves_title() {
    let store = EKEventStore::new().expect("store");
    let draft = EKCalendarDraft::new(EKEntityType::Event, "Example Calendar");
    let roundtrip = draft.roundtrip(&store).expect("roundtrip");
    assert_eq!(roundtrip.title, "Example Calendar");
}
