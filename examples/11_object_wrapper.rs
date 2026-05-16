use eventkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = EKEventStore::new()?;

    let event_object = EKEvent::new(
        "Object wrapper demo",
        "2026-05-16T10:00:00Z",
        "2026-05-16T11:00:00Z",
    )
    .as_object_in(&store)?;
    println!(
        "event object: is_new={}, has_changes={}, refresh={}",
        event_object.is_new(),
        event_object.has_changes(),
        event_object.refresh()
    );
    event_object.reset();
    println!(
        "event object after reset: has_changes={}",
        event_object.has_changes()
    );

    let reminder_object = EKReminder::new("Object wrapper reminder").as_object_in(&store)?;
    println!(
        "reminder object: is_new={}, has_changes={}",
        reminder_object.is_new(),
        reminder_object.has_changes()
    );

    let calendar_object = EKCalendarDraft::new(EKEntityType::Event, "Object wrapper calendar")
        .as_object_in(&store)?;
    println!(
        "calendar object: is_new={}, has_changes={}",
        calendar_object.is_new(),
        calendar_object.has_changes()
    );

    println!("✅ object wrapper OK");
    Ok(())
}
