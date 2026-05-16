use eventkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = EKEventStore::new()?;
    println!("notification: {EK_EVENT_STORE_CHANGED_NOTIFICATION}");
    println!(
        "event authorization: {:?}",
        EKEventStore::authorization_status(EKEntityType::Event)
    );
    println!(
        "reminder authorization: {:?}",
        EKEventStore::authorization_status(EKEntityType::Reminder)
    );
    println!("store identifier: {}", store.event_store_identifier()?);
    println!("sources: {}", store.sources()?.len());
    println!(
        "event calendars: {}",
        store.calendars_for_entity_type(EKEntityType::Event)?.len()
    );
    println!(
        "reminder calendars: {}",
        store
            .calendars_for_entity_type(EKEntityType::Reminder)?
            .len()
    );
    println!("✅ event store OK");
    Ok(())
}
