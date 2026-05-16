use eventkit::prelude::*;

#[test]
fn structured_location_roundtrip_preserves_radius() {
    let location = EKStructuredLocation::new("Office")
        .with_geo_location(EKGeoLocation::new(59.3293, 18.0686))
        .with_radius(15.0);
    let roundtrip = location.roundtrip().expect("roundtrip");
    assert!((roundtrip.radius - 15.0).abs() < f64::EPSILON);
    assert_eq!(roundtrip.title.as_deref(), Some("Office"));
}
