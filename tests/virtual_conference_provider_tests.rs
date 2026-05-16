use eventkit::prelude::*;

#[test]
fn virtual_conference_descriptors_roundtrip() {
    let room_type = EKVirtualConferenceRoomTypeDescriptor::new("Room", "room");
    let roundtrip_room_type = room_type.roundtrip().expect("room type");
    assert_eq!(roundtrip_room_type.title, "Room");

    let descriptor =
        EKVirtualConferenceDescriptor::new(vec![EKVirtualConferenceURLDescriptor::new(
            "https://example.invalid/join",
        )
        .with_title("Join link")])
        .with_title("Standup")
        .with_conference_details("Daily sync");
    let roundtrip = descriptor.roundtrip().expect("descriptor");
    assert_eq!(roundtrip.title.as_deref(), Some("Standup"));
    let extension_only = EK_VIRTUAL_CONFERENCE_PROVIDER_IS_EXTENSION_ONLY;
    assert!(extension_only);
}
