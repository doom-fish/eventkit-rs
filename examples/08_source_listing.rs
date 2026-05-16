use eventkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = EKEventStore::new()?;
    let sources = store.sources()?;
    println!("sources: {}", sources.len());
    if let Some(source) = sources.first() {
        let calendars = source.calendars_for_entity_type(&store, EKEntityType::Event)?;
        println!("first source: {}", source.title);
        println!("event calendars in first source: {}", calendars.len());
    } else {
        println!("no EventKit sources available");
    }
    println!("✅ source OK");
    Ok(())
}
