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

#[test]
fn alarm_roundtrip_preserves_procedure_url() {
    let mut alarm = EKAlarm::relative(-60.0);
    alarm.url = Some("file:///Applications/Calendar.app".to_owned());
    let roundtrip = alarm.roundtrip().expect("roundtrip");
    assert_eq!(
        roundtrip.url.as_deref(),
        Some("file:///Applications/Calendar.app")
    );
    assert_eq!(roundtrip.alarm_type, Some(EKAlarmType::Procedure));
}
