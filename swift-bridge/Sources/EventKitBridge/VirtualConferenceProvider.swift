import EventKit
import Foundation

struct EKRVirtualConferenceRoomTypeDescriptorPayload: Codable {
    var title: String
    var identifier: String
}

struct EKRVirtualConferenceURLDescriptorPayload: Codable {
    var title: String?
    var url: String
}

struct EKRVirtualConferenceDescriptorPayload: Codable {
    var title: String?
    var urlDescriptors: [EKRVirtualConferenceURLDescriptorPayload]
    var conferenceDetails: String?
}

func ekrEncodeVirtualConferenceRoomTypeDescriptor(
    _ descriptor: EKVirtualConferenceRoomTypeDescriptor
) -> EKRVirtualConferenceRoomTypeDescriptorPayload {
    EKRVirtualConferenceRoomTypeDescriptorPayload(
        title: descriptor.title,
        identifier: descriptor.identifier
    )
}

func ekrDecodeVirtualConferenceRoomTypeDescriptor(
    _ payload: EKRVirtualConferenceRoomTypeDescriptorPayload
) -> EKVirtualConferenceRoomTypeDescriptor {
    EKVirtualConferenceRoomTypeDescriptor(title: payload.title, identifier: payload.identifier)
}

func ekrEncodeVirtualConferenceURLDescriptor(
    _ descriptor: EKVirtualConferenceURLDescriptor
) -> EKRVirtualConferenceURLDescriptorPayload {
    EKRVirtualConferenceURLDescriptorPayload(
        title: descriptor.title,
        url: descriptor.url.absoluteString
    )
}

func ekrDecodeVirtualConferenceURLDescriptor(
    _ payload: EKRVirtualConferenceURLDescriptorPayload
) throws -> EKVirtualConferenceURLDescriptor {
    guard let url = URL(string: payload.url) else {
        throw NSError(
            domain: "eventkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "invalid virtual conference URL: \(payload.url)"]
        )
    }
    return EKVirtualConferenceURLDescriptor(title: payload.title, url: url)
}

func ekrEncodeVirtualConferenceDescriptor(
    _ descriptor: EKVirtualConferenceDescriptor
) -> EKRVirtualConferenceDescriptorPayload {
    EKRVirtualConferenceDescriptorPayload(
        title: descriptor.title,
        urlDescriptors: descriptor.urlDescriptors.map(ekrEncodeVirtualConferenceURLDescriptor),
        conferenceDetails: descriptor.conferenceDetails
    )
}

func ekrDecodeVirtualConferenceDescriptor(
    _ payload: EKRVirtualConferenceDescriptorPayload
) throws -> EKVirtualConferenceDescriptor {
    let urlDescriptors = try payload.urlDescriptors.map(ekrDecodeVirtualConferenceURLDescriptor)
    return EKVirtualConferenceDescriptor(
        title: payload.title,
        urlDescriptors: urlDescriptors,
        conferenceDetails: payload.conferenceDetails
    )
}

@_cdecl("ek_virtual_conference_room_type_roundtrip_json")
public func ek_virtual_conference_room_type_roundtrip_json(
    _ descriptorJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let payload = try ekrDecodeJSON(descriptorJSON, as: EKRVirtualConferenceRoomTypeDescriptorPayload.self)
        let descriptor = ekrDecodeVirtualConferenceRoomTypeDescriptor(payload)
        return ekrCString(try ekrEncodeJSON(ekrEncodeVirtualConferenceRoomTypeDescriptor(descriptor)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_virtual_conference_url_roundtrip_json")
public func ek_virtual_conference_url_roundtrip_json(
    _ descriptorJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let payload = try ekrDecodeJSON(descriptorJSON, as: EKRVirtualConferenceURLDescriptorPayload.self)
        let descriptor = try ekrDecodeVirtualConferenceURLDescriptor(payload)
        return ekrCString(try ekrEncodeJSON(ekrEncodeVirtualConferenceURLDescriptor(descriptor)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_virtual_conference_descriptor_roundtrip_json")
public func ek_virtual_conference_descriptor_roundtrip_json(
    _ descriptorJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let payload = try ekrDecodeJSON(descriptorJSON, as: EKRVirtualConferenceDescriptorPayload.self)
        let descriptor = try ekrDecodeVirtualConferenceDescriptor(payload)
        return ekrCString(try ekrEncodeJSON(ekrEncodeVirtualConferenceDescriptor(descriptor)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}
