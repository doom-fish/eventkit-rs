use eventkit::prelude::*;

#[test]
fn source_listing_works_without_panicking() {
    let store = EKEventStore::new().expect("store");
    let sources = store.sources().expect("sources");
    if let Some(source) = sources.first() {
        let _ = source
            .calendars_for_entity_type(&store, EKEntityType::Event)
            .expect("source calendars");
    }
}
