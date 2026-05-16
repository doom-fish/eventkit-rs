use eventkit::prelude::*;

#[test]
fn event_store_can_be_created_and_dropped() {
    let _identifier = {
        let store = EKEventStore::new().expect("store");
        store.event_store_identifier().expect("identifier")
    };
}

#[test]
fn event_store_supports_non_mutating_maintenance_calls() {
    let store = EKEventStore::new().expect("store");
    store.reset();
    store.refresh_sources_if_necessary();
}
