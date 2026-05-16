import CoreLocation
import EventKit
import Foundation

struct EKRGeoLocationPayload: Codable {
    var latitude: Double
    var longitude: Double
    var altitude: Double?
    var horizontalAccuracy: Double?
    var verticalAccuracy: Double?
}

struct EKRStructuredLocationPayload: Codable {
    var title: String?
    var geoLocation: EKRGeoLocationPayload?
    var radius: Double
}

func ekrEncodeGeoLocation(_ location: CLLocation?) -> EKRGeoLocationPayload? {
    guard let location else { return nil }
    return EKRGeoLocationPayload(
        latitude: location.coordinate.latitude,
        longitude: location.coordinate.longitude,
        altitude: location.altitude,
        horizontalAccuracy: location.horizontalAccuracy,
        verticalAccuracy: location.verticalAccuracy
    )
}

func ekrDecodeGeoLocation(_ payload: EKRGeoLocationPayload?) -> CLLocation? {
    guard let payload else { return nil }
    let coordinate = CLLocationCoordinate2D(latitude: payload.latitude, longitude: payload.longitude)
    if let altitude = payload.altitude,
       let horizontalAccuracy = payload.horizontalAccuracy,
       let verticalAccuracy = payload.verticalAccuracy {
        return CLLocation(
            coordinate: coordinate,
            altitude: altitude,
            horizontalAccuracy: horizontalAccuracy,
            verticalAccuracy: verticalAccuracy,
            timestamp: Date()
        )
    }
    return CLLocation(latitude: payload.latitude, longitude: payload.longitude)
}

func ekrEncodeStructuredLocation(_ location: EKStructuredLocation?) -> EKRStructuredLocationPayload? {
    guard let location else { return nil }
    return EKRStructuredLocationPayload(
        title: location.title,
        geoLocation: ekrEncodeGeoLocation(location.geoLocation),
        radius: location.radius
    )
}

func ekrDecodeStructuredLocation(_ payload: EKRStructuredLocationPayload?) -> EKStructuredLocation? {
    guard let payload else { return nil }
    let location = EKStructuredLocation(title: payload.title ?? "")
    location.title = payload.title
    location.geoLocation = ekrDecodeGeoLocation(payload.geoLocation)
    location.radius = payload.radius
    return location
}

@_cdecl("ek_structured_location_roundtrip_json")
public func ek_structured_location_roundtrip_json(
    _ locationJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let payload = try ekrDecodeJSON(locationJSON, as: EKRStructuredLocationPayload.self)
        let location = ekrDecodeStructuredLocation(payload)
        return ekrCString(try ekrEncodeJSON(ekrEncodeStructuredLocation(location)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}
