use eventkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rule = EKRecurrenceRule::new(EKRecurrenceFrequency::Monthly)
        .with_interval(1)
        .with_days_of_the_week([EKRecurrenceDayOfWeek::new(EKWeekday::Monday).with_week_number(2)])
        .with_set_positions([1]);
    let roundtrip = rule.roundtrip()?;
    println!("rule frequency: {:?}", roundtrip.frequency);
    println!("rule days-of-week: {}", roundtrip.days_of_the_week.len());
    println!("rule set-positions: {:?}", roundtrip.set_positions);
    println!("✅ recurrence rule OK");
    Ok(())
}
