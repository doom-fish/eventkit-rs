use eventkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = EKEventStore::new()?;
    let draft =
        EKCalendarDraft::new(EKEntityType::Event, "Rust EventKit Demo").with_color("#3366FFAA");
    let calendar = draft.roundtrip(&store)?;
    println!("calendar title: {}", calendar.title);
    println!("calendar type: {:?}", calendar.calendar_type);
    println!(
        "calendar allows modifications: {}",
        calendar.allows_content_modifications
    );
    println!("✅ calendar OK");
    Ok(())
}
