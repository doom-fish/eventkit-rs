use eventkit::prelude::*;
use eventkit::{
    EKCalendarDraft, EKGeoLocation, EKSource, EKStructuredLocation, EKVirtualConferenceDescriptor,
    EKVirtualConferenceURLDescriptor,
};

const SECRET: &str = "Jane Appleseed <jane@example.com>";

#[test]
fn debug_output_redacts_personal_data() {
    let participant = EKParticipant {
        url: Some(format!("mailto:{SECRET}")),
        name: Some(SECRET.to_owned()),
        contact_predicate: Some(SECRET.to_owned()),
        ..EKParticipant::default()
    };
    let location = EKStructuredLocation::new(SECRET)
        .with_geo_location(EKGeoLocation::new(59.329_323, 18.068_581));
    let mut alarm = EKAlarm::relative(-600.0);
    alarm.email_address = Some(SECRET.to_owned());
    alarm.url = Some(SECRET.to_owned());

    let mut event = EKEvent::new(SECRET, "2026-01-01T10:00:00Z", "2026-01-01T11:00:00Z")
        .with_structured_location(location.clone());
    event.notes = Some(SECRET.to_owned());
    event.location = Some(SECRET.to_owned());
    event.url = Some(SECRET.to_owned());
    event.attendees = vec![participant.clone()];
    event.organizer = Some(participant.clone());
    event.alarms = vec![alarm.clone()];
    event.birthday_contact_identifier = Some(SECRET.to_owned());
    event.birthday_person_unique_id = Some(SECRET.to_owned());

    let mut reminder = EKReminder::new(SECRET);
    reminder.notes = Some(SECRET.to_owned());
    reminder.location = Some(SECRET.to_owned());
    reminder.url = Some(SECRET.to_owned());
    reminder.attendees = vec![participant.clone()];

    let source = EKSource {
        title: SECRET.to_owned(),
        ..EKSource::default()
    };
    let calendar = EKCalendar {
        title: SECRET.to_owned(),
        source: Some(source.clone()),
        ..EKCalendar::default()
    };
    let draft = EKCalendarDraft::new(EKEntityType::Event, SECRET);
    let mut conference =
        EKVirtualConferenceDescriptor::new(vec![EKVirtualConferenceURLDescriptor::new(format!(
            "https://example.com/{SECRET}"
        ))
        .with_title("Join")]);
    conference.conference_details = Some(SECRET.to_owned());

    let outputs = [
        format!("{participant:?}"),
        format!("{location:?}"),
        format!("{alarm:?}"),
        format!("{event:?}"),
        format!("{reminder:?}"),
        format!("{source:?}"),
        format!("{calendar:?}"),
        format!("{draft:?}"),
        format!("{conference:?}"),
    ];
    for output in &outputs {
        assert!(!output.contains("Appleseed"), "{output}");
        assert!(!output.contains("example.com"), "{output}");
        assert!(!output.contains("59.329"), "{output}");
        assert!(!output.contains("18.068"), "{output}");
        assert!(output.contains("<redacted>"), "{output}");
    }
    assert!(outputs[3].contains("2026-01-01T10:00:00Z"));
    assert!(outputs[3].contains("participant_status"));
    assert!(outputs[2].contains("-600.0"));
}
