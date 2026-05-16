use eventkit::prelude::*;

#[test]
fn participant_serializes_and_deserializes() {
    let participant = EKParticipant {
        name: Some("Alex Example".into()),
        participant_status: EKParticipantStatus::Accepted,
        participant_role: EKParticipantRole::Required,
        participant_type: EKParticipantType::Person,
        ..EKParticipant::default()
    };

    let json = serde_json::to_string(&participant).expect("json");
    let decoded: EKParticipant = serde_json::from_str(&json).expect("decode");
    assert_eq!(decoded.name.as_deref(), Some("Alex Example"));
    assert_eq!(decoded.participant_status, EKParticipantStatus::Accepted);

    let status_json = serde_json::to_string(&EKParticipantScheduleStatus::Delivered).expect("json");
    let schedule_status: EKParticipantScheduleStatus =
        serde_json::from_str(&status_json).expect("decode");
    assert_eq!(schedule_status, EKParticipantScheduleStatus::Delivered);
}
