use eventkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = EKEventStore::new()?;
    let mut reminder = EKReminder::new("Ship 0.2.1").with_priority_kind(EKReminderPriority::High);
    reminder.start_date_components = Some(NSDateComponents::date(2026, 1, 1));
    reminder.due_date_components = Some(NSDateComponents::date(2026, 1, 15).with_time(17, 0, 0));
    reminder.notes = Some("Remember to push the tag after verification.".into());
    reminder.alarms.push(EKAlarm::relative(-3600.0));

    let roundtrip = reminder.roundtrip_in(&store)?;
    println!("reminder title: {}", roundtrip.title);
    println!("reminder priority: {:?}", roundtrip.priority_kind());
    println!("reminder alarms: {}", roundtrip.alarms.len());
    println!("✅ reminder OK");
    Ok(())
}
