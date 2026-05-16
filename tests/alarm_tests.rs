use eventkit::prelude::*;

#[test]
fn alarm_roundtrip_preserves_absolute_date() {
    let alarm = EKAlarm::absolute("2026-01-01T09:45:00Z");
    let roundtrip = alarm.roundtrip().expect("roundtrip");
    assert_eq!(
        roundtrip.absolute_date.as_deref(),
        Some("2026-01-01T09:45:00Z")
    );
}
