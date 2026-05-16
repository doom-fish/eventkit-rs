use eventkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let alarm = EKAlarm::absolute("2026-01-01T09:45:00Z");
    let roundtrip = alarm.roundtrip()?;
    println!("alarm absolute date: {:?}", roundtrip.absolute_date);
    println!("alarm type: {:?}", roundtrip.alarm_type);
    println!("✅ alarm OK");
    Ok(())
}
