use eventkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = EKEventStore::new()?;
    let mut event = EKEvent::new("Planning", "2026-01-01T10:00:00Z", "2026-01-01T11:00:00Z")
        .with_all_day(false)
        .with_structured_location(
            EKStructuredLocation::new("HQ")
                .with_geo_location(EKGeoLocation::new(59.3346, 18.0632))
                .with_radius(50.0),
        );
    event.notes = Some("Quarterly planning sync".into());
    event.alarms.push(EKAlarm::relative(-900.0));
    event.recurrence_rules.push(
        EKRecurrenceRule::new(EKRecurrenceFrequency::Weekly)
            .with_interval(2)
            .with_days_of_the_week([EKRecurrenceDayOfWeek::new(EKWeekday::Thursday)]),
    );

    let roundtrip = event.roundtrip_in(&store)?;
    println!("event title: {}", roundtrip.title);
    println!("event all-day: {}", roundtrip.all_day);
    println!("event alarms: {}", roundtrip.alarms.len());
    println!("event recurrences: {}", roundtrip.recurrence_rules.len());
    println!("✅ event OK");
    Ok(())
}
