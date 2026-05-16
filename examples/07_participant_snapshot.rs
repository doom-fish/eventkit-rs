use eventkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let participant = EKParticipant {
        name: Some("Alex Example".into()),
        participant_status: EKParticipantStatus::Accepted,
        participant_role: EKParticipantRole::Required,
        participant_type: EKParticipantType::Person,
        ..EKParticipant::default()
    };

    println!(
        "participant: {}",
        serde_json::to_string_pretty(&participant)?
    );
    println!(
        "sample schedule status: {:?}",
        EKParticipantScheduleStatus::Delivered
    );
    println!("✅ participant OK");
    Ok(())
}
