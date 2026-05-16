use eventkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_status = EKEventStore::authorization_status(EKEntityType::Event);
    let reminder_status = EKEventStore::authorization_status(EKEntityType::Reminder);
    println!("event authorization: {event_status:?}");
    println!("reminder authorization: {reminder_status:?}");

    let store = EKEventStore::new()?;
    let calendars = store.calendars_for_entity_type(EKEntityType::Event)?;
    println!("event calendars: {}", calendars.len());
    for calendar in &calendars {
        println!("- {}", calendar.title);
    }

    println!("✅ eventkit OK");
    Ok(())
}
