use eventkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let room_type = EKVirtualConferenceRoomTypeDescriptor::new("Team Meeting", "team-meeting");
    let url = EKVirtualConferenceURLDescriptor::new("https://example.invalid/join/team-meeting")
        .with_title("Join link");
    let descriptor = EKVirtualConferenceDescriptor::new(vec![url])
        .with_title("Remote Standup")
        .with_conference_details("Dial in a few minutes early.");

    println!("room type: {}", room_type.roundtrip()?.title);
    println!("conference title: {:?}", descriptor.roundtrip()?.title);
    println!("provider extension-only: {EK_VIRTUAL_CONFERENCE_PROVIDER_IS_EXTENSION_ONLY}");
    println!("✅ virtual conference OK");
    Ok(())
}
