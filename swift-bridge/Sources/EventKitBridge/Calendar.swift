import CoreGraphics
import EventKit
import Foundation

enum EKRCalendarType: String, Codable {
    case local
    case calDav
    case exchange
    case subscription
    case birthday
}

enum EKRCalendarEventAvailability: String, Codable {
    case busy
    case free
    case tentative
    case unavailable
}

struct EKRCalendarPayload: Codable {
    var identifier: String
    var title: String
    var calendarType: EKRCalendarType
    var allowedEntityTypes: [EKREntityType]
    var color: String?
    var source: EKRSourcePayload?
    var allowsContentModifications: Bool
    var isSubscribed: Bool
    var isImmutable: Bool
    var supportedEventAvailabilities: [EKRCalendarEventAvailability]
}

struct EKRCalendarDraftPayload: Codable {
    var identifier: String?
    var entityType: EKREntityType
    var sourceIdentifier: String?
    var title: String
    var color: String?
}

func ekrCalendarTypePayload(from calendarType: EKCalendarType) -> EKRCalendarType {
    switch calendarType {
    case .local:
        return .local
    case .calDAV:
        return .calDav
    case .exchange:
        return .exchange
    case .subscription:
        return .subscription
    case .birthday:
        return .birthday
    @unknown default:
        return .local
    }
}

func ekrCalendarEventAvailabilities(_ mask: EKCalendarEventAvailabilityMask) -> [EKRCalendarEventAvailability] {
    var result: [EKRCalendarEventAvailability] = []
    if mask.rawValue & EKCalendarEventAvailabilityMask.busy.rawValue != 0 {
        result.append(.busy)
    }
    if mask.rawValue & EKCalendarEventAvailabilityMask.free.rawValue != 0 {
        result.append(.free)
    }
    if mask.rawValue & EKCalendarEventAvailabilityMask.tentative.rawValue != 0 {
        result.append(.tentative)
    }
    if mask.rawValue & EKCalendarEventAvailabilityMask.unavailable.rawValue != 0 {
        result.append(.unavailable)
    }
    return result
}

func ekrColorString(_ color: CGColor?) -> String? {
    guard let color else { return nil }
    let sRGB = CGColorSpace(name: CGColorSpace.sRGB)
    let converted = sRGB.flatMap { color.converted(to: $0, intent: .defaultIntent, options: nil) } ?? color
    guard let components = converted.components else { return nil }

    let rgba: (CGFloat, CGFloat, CGFloat, CGFloat)
    switch converted.numberOfComponents {
    case 2:
        rgba = (components[0], components[0], components[0], components[1])
    case 4:
        rgba = (components[0], components[1], components[2], components[3])
    default:
        return nil
    }

    return String(
        format: "#%02X%02X%02X%02X",
        Int((rgba.0 * 255).rounded()),
        Int((rgba.1 * 255).rounded()),
        Int((rgba.2 * 255).rounded()),
        Int((rgba.3 * 255).rounded())
    )
}

func ekrCGColor(from hex: String) -> CGColor? {
    let trimmed = hex.trimmingCharacters(in: .whitespacesAndNewlines)
    guard trimmed.hasPrefix("#") else { return nil }
    let value = String(trimmed.dropFirst())
    var number: UInt64 = 0
    guard Scanner(string: value).scanHexInt64(&number) else { return nil }

    let rgba: (CGFloat, CGFloat, CGFloat, CGFloat)
    switch value.count {
    case 6:
        rgba = (
            CGFloat((number & 0xFF0000) >> 16) / 255,
            CGFloat((number & 0x00FF00) >> 8) / 255,
            CGFloat(number & 0x0000FF) / 255,
            1
        )
    case 8:
        rgba = (
            CGFloat((number & 0xFF000000) >> 24) / 255,
            CGFloat((number & 0x00FF0000) >> 16) / 255,
            CGFloat((number & 0x0000FF00) >> 8) / 255,
            CGFloat(number & 0x000000FF) / 255
        )
    default:
        return nil
    }

    return CGColor(
        red: rgba.0,
        green: rgba.1,
        blue: rgba.2,
        alpha: rgba.3
    )
}

func ekrEncodeCalendar(_ calendar: EKCalendar) -> EKRCalendarPayload {
    EKRCalendarPayload(
        identifier: calendar.calendarIdentifier,
        title: calendar.title,
        calendarType: ekrCalendarTypePayload(from: calendar.type),
        allowedEntityTypes: ekrAllowedEntityTypes(calendar.allowedEntityTypes),
        color: ekrColorString(calendar.cgColor),
        source: calendar.source.map(ekrEncodeSource),
        allowsContentModifications: calendar.allowsContentModifications,
        isSubscribed: calendar.isSubscribed,
        isImmutable: calendar.isImmutable,
        supportedEventAvailabilities: ekrCalendarEventAvailabilities(calendar.supportedEventAvailabilities)
    )
}

func ekrPrepareCalendar(
    store: EKEventStore,
    payload: EKRCalendarDraftPayload,
    requireSource: Bool
) throws -> EKCalendar {
    let calendar: EKCalendar
    if let identifier = payload.identifier {
        guard let existing = store.calendar(withIdentifier: identifier) else {
            throw NSError(
                domain: "eventkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "calendar not found: \(identifier)"]
            )
        }
        calendar = existing
        if let sourceIdentifier = payload.sourceIdentifier,
           let existingSource = calendar.source,
           sourceIdentifier != existingSource.sourceIdentifier {
            throw NSError(
                domain: "eventkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "cannot move an existing calendar to a different source"]
            )
        }
    } else {
        calendar = EKCalendar(for: ekrEntityType(from: payload.entityType), eventStore: store)
        if let sourceIdentifier = payload.sourceIdentifier {
            guard let source = store.source(withIdentifier: sourceIdentifier) else {
                throw NSError(
                    domain: "eventkit-rs",
                    code: -1,
                    userInfo: [NSLocalizedDescriptionKey: "unknown calendar source identifier: \(sourceIdentifier)"]
                )
            }
            calendar.source = source
        } else if requireSource {
            throw NSError(
                domain: "eventkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "new calendars require sourceIdentifier"]
            )
        }
    }

    calendar.title = payload.title
    if let color = payload.color {
        guard let cgColor = ekrCGColor(from: color) else {
            throw NSError(
                domain: "eventkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "invalid calendar color: \(color)"]
            )
        }
        calendar.cgColor = cgColor
    }
    return calendar
}

@_cdecl("ek_store_calendars_json")
public func ek_store_calendars_json(
    _ store: UnsafeMutableRawPointer?,
    _ entityType: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }

    do {
        let entityType = try ekrEntityType(from: entityType)
        let calendars = ekrBorrow(store, as: EKEventStore.self)
            .calendars(for: entityType)
            .map(ekrEncodeCalendar)
        return ekrCString(try ekrEncodeJSON(calendars))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_store_default_event_calendar_json")
public func ek_store_default_event_calendar_json(
    _ store: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }

    do {
        let calendar = ekrBorrow(store, as: EKEventStore.self).defaultCalendarForNewEvents
        guard let calendar else { return nil }
        return ekrCString(try ekrEncodeJSON(ekrEncodeCalendar(calendar)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_store_default_reminder_calendar_json")
public func ek_store_default_reminder_calendar_json(
    _ store: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }

    do {
        let calendar = ekrBorrow(store, as: EKEventStore.self).defaultCalendarForNewReminders()
        guard let calendar else { return nil }
        return ekrCString(try ekrEncodeJSON(ekrEncodeCalendar(calendar)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_store_calendar_json")
public func ek_store_calendar_json(
    _ store: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }
    guard let identifier else {
        ekrSetMessageError(outError, message: "missing EKCalendar identifier")
        return nil
    }

    let calendar = ekrBorrow(store, as: EKEventStore.self).calendar(withIdentifier: String(cString: identifier))
    guard let calendar else { return nil }

    do {
        return ekrCString(try ekrEncodeJSON(ekrEncodeCalendar(calendar)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_store_save_calendar_json")
public func ek_store_save_calendar_json(
    _ store: UnsafeMutableRawPointer?,
    _ calendarJSON: UnsafePointer<CChar>?,
    _ commit: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }

    do {
        let payload = try ekrDecodeJSON(calendarJSON, as: EKRCalendarDraftPayload.self)
        let eventStore = ekrBorrow(store, as: EKEventStore.self)
        let calendar = try ekrPrepareCalendar(store: eventStore, payload: payload, requireSource: true)
        try eventStore.saveCalendar(calendar, commit: commit)
        return ekrCString(try ekrEncodeJSON(ekrEncodeCalendar(calendar)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_store_remove_calendar")
public func ek_store_remove_calendar(
    _ store: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ commit: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return EKR_ERROR
    }
    guard let identifier else {
        ekrSetMessageError(outError, message: "missing EKCalendar identifier")
        return EKR_ERROR
    }

    do {
        let eventStore = ekrBorrow(store, as: EKEventStore.self)
        let calendarIdentifier = String(cString: identifier)
        guard let calendar = eventStore.calendar(withIdentifier: calendarIdentifier) else {
            throw NSError(
                domain: "eventkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "calendar not found: \(calendarIdentifier)"]
            )
        }
        try eventStore.removeCalendar(calendar, commit: commit)
        return EKR_OK
    } catch {
        ekrSetError(outError, error)
        return EKR_ERROR
    }
}

@_cdecl("ek_calendar_roundtrip_json")
public func ek_calendar_roundtrip_json(
    _ store: UnsafeMutableRawPointer?,
    _ calendarJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }

    do {
        let payload = try ekrDecodeJSON(calendarJSON, as: EKRCalendarDraftPayload.self)
        let calendar = try ekrPrepareCalendar(store: ekrBorrow(store, as: EKEventStore.self), payload: payload, requireSource: false)
        return ekrCString(try ekrEncodeJSON(ekrEncodeCalendar(calendar)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}
