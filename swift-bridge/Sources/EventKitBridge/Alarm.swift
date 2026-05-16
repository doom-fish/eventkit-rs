import EventKit
import Foundation

enum EKRAlarmProximity: String, Codable {
    case none
    case enter
    case leave
}

enum EKRAlarmType: String, Codable {
    case display
    case audio
    case procedure
    case email
}

struct EKRAlarmPayload: Codable {
    var absoluteDate: String?
    var relativeOffset: Double?
    var structuredLocation: EKRStructuredLocationPayload?
    var proximity: EKRAlarmProximity?
    var alarmType: EKRAlarmType?
    var emailAddress: String?
    var soundName: String?
    var url: String?
}

func ekrAlarmProximity(from proximity: EKRAlarmProximity) -> EKAlarmProximity {
    switch proximity {
    case .none:
        return .none
    case .enter:
        return .enter
    case .leave:
        return .leave
    }
}

func ekrAlarmProximityPayload(from proximity: EKAlarmProximity) -> EKRAlarmProximity {
    switch proximity {
    case .none:
        return .none
    case .enter:
        return .enter
    case .leave:
        return .leave
    @unknown default:
        return .none
    }
}

func ekrAlarmTypePayload(from type: EKAlarmType) -> EKRAlarmType {
    switch type {
    case .display:
        return .display
    case .audio:
        return .audio
    case .procedure:
        return .procedure
    case .email:
        return .email
    @unknown default:
        return .display
    }
}

func ekrProcedureAlarmURL(_ alarm: EKAlarm) -> String? {
    (alarm.value(forKey: "url") as? URL)?.absoluteString
}

func ekrSetProcedureAlarmURL(_ alarm: EKAlarm, url: String?) {
    alarm.setValue(url.flatMap(URL.init(string:)), forKey: "url")
}

func ekrEncodeAlarm(_ alarm: EKAlarm) -> EKRAlarmPayload {
    EKRAlarmPayload(
        absoluteDate: ekrDateString(alarm.absoluteDate),
        relativeOffset: alarm.relativeOffset,
        structuredLocation: ekrEncodeStructuredLocation(alarm.structuredLocation),
        proximity: ekrAlarmProximityPayload(from: alarm.proximity),
        alarmType: ekrAlarmTypePayload(from: alarm.type),
        emailAddress: alarm.emailAddress,
        soundName: alarm.soundName,
        url: ekrProcedureAlarmURL(alarm)
    )
}

func ekrDecodeAlarm(_ payload: EKRAlarmPayload) throws -> EKAlarm {
    let alarm: EKAlarm
    if let absoluteDate = payload.absoluteDate {
        alarm = EKAlarm(absoluteDate: try ekrDate(from: absoluteDate))
    } else if let relativeOffset = payload.relativeOffset {
        alarm = EKAlarm(relativeOffset: relativeOffset)
    } else {
        alarm = EKAlarm(relativeOffset: 0)
    }

    alarm.structuredLocation = ekrDecodeStructuredLocation(payload.structuredLocation)
    if let proximity = payload.proximity {
        alarm.proximity = ekrAlarmProximity(from: proximity)
    }
    if let emailAddress = payload.emailAddress {
        alarm.emailAddress = emailAddress
    }
    if let soundName = payload.soundName {
        alarm.soundName = soundName
    }
    if let url = payload.url {
        ekrSetProcedureAlarmURL(alarm, url: url)
    }
    return alarm
}

@_cdecl("ek_alarm_roundtrip_json")
public func ek_alarm_roundtrip_json(
    _ alarmJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let payload = try ekrDecodeJSON(alarmJSON, as: EKRAlarmPayload.self)
        _ = try ekrDecodeAlarm(payload)

        var validated = payload
        if validated.alarmType == nil {
            if validated.emailAddress != nil {
                validated.alarmType = .email
            } else if validated.soundName != nil {
                validated.alarmType = .audio
            } else if validated.url != nil {
                validated.alarmType = .procedure
            } else {
                validated.alarmType = .display
            }
        }

        return ekrCString(try ekrEncodeJSON(validated))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}
