use eventkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let location = EKStructuredLocation::new("Conference Room")
        .with_geo_location(EKGeoLocation::new(37.3349, -122.0090))
        .with_radius(15.0);
    let roundtrip = location.roundtrip()?;
    println!("location title: {:?}", roundtrip.title);
    println!("location radius: {}", roundtrip.radius);
    println!("✅ structured location OK");
    Ok(())
}
