import EventKit
import Foundation

enum EKREventAvailability: String, Codable {
    case notSupported
    case busy
    case free
    case tentative
    case unavailable
}

enum EKREventStatus: String, Codable {
    case none
    case confirmed
    case tentative
    case canceled
}

struct EKREventPayload: Codable {
    var identifier: String?
    var title: String
    var startDate: String
    var endDate: String
    var calendarIdentifier: String?
    var calendar: EKRCalendarPayload?
    var notes: String?
    var location: String?
    var alarms: [EKRAlarmPayload]
    var recurrenceRules: [EKRRecurrenceRulePayload]
    var calendarItemIdentifier: String?
    var calendarItemExternalIdentifier: String?
    var url: String?
    var lastModifiedDate: String?
    var creationDate: String?
    var timeZoneIdentifier: String?
    var hasAlarms: Bool
    var hasRecurrenceRules: Bool
    var hasAttendees: Bool
    var hasNotes: Bool
    var attendees: [EKRParticipantPayload]
    var allDay: Bool
    var structuredLocation: EKRStructuredLocationPayload?
    var organizer: EKRParticipantPayload?
    var availability: EKREventAvailability
    var status: EKREventStatus
    var isDetached: Bool
    var occurrenceDate: String?
    var birthdayContactIdentifier: String?
    var birthdayPersonID: Int?
    var birthdayPersonUniqueID: String?
}

func ekrEventAvailability(from availability: EKREventAvailability) -> EKEventAvailability {
    switch availability {
    case .notSupported:
        return .notSupported
    case .busy:
        return .busy
    case .free:
        return .free
    case .tentative:
        return .tentative
    case .unavailable:
        return .unavailable
    }
}

func ekrEventAvailabilityPayload(from availability: EKEventAvailability) -> EKREventAvailability {
    switch availability {
    case .notSupported:
        return .notSupported
    case .busy:
        return .busy
    case .free:
        return .free
    case .tentative:
        return .tentative
    case .unavailable:
        return .unavailable
    @unknown default:
        return .notSupported
    }
}

func ekrEventStatusPayload(from status: EKEventStatus) -> EKREventStatus {
    switch status {
    case .none:
        return .none
    case .confirmed:
        return .confirmed
    case .tentative:
        return .tentative
    case .canceled:
        return .canceled
    @unknown default:
        return .none
    }
}

func ekrPrepareEvent(
    store: EKEventStore,
    payload: EKREventPayload,
    requireCalendar: Bool
) throws -> EKEvent {
    let event: EKEvent
    if let identifier = payload.identifier, let existing = store.event(withIdentifier: identifier) {
        event = existing
    } else {
        event = EKEvent(eventStore: store)
    }

    event.title = payload.title
    event.startDate = try ekrDate(from: payload.startDate)
    event.endDate = try ekrDate(from: payload.endDate)
    event.isAllDay = payload.allDay
    event.notes = payload.notes
    event.location = payload.location
    event.url = payload.url.flatMap(URL.init(string:))
    event.timeZone = payload.timeZoneIdentifier.flatMap(TimeZone.init(identifier:))
    event.structuredLocation = ekrDecodeStructuredLocation(payload.structuredLocation)
    event.availability = ekrEventAvailability(from: payload.availability)

    let calendarIdentifier = payload.calendarIdentifier ?? payload.calendar?.identifier
    if let calendarIdentifier {
        guard let calendar = store.calendar(withIdentifier: calendarIdentifier) else {
            throw NSError(
                domain: "eventkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "unknown event calendar identifier: \(calendarIdentifier)"]
            )
        }
        event.calendar = calendar
    } else if requireCalendar, event.calendar == nil {
        guard let calendar = store.defaultCalendarForNewEvents else {
            throw NSError(
                domain: "eventkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "no default calendar for new events"]
            )
        }
        event.calendar = calendar
    }

    event.alarms = payload.alarms.compactMap { try? ekrDecodeAlarm($0) }
    event.recurrenceRules = payload.recurrenceRules.compactMap { try? ekrDecodeRecurrenceRule($0) }
    return event
}

func ekrEncodeEvent(_ event: EKEvent) -> EKREventPayload {
    EKREventPayload(
        identifier: event.eventIdentifier,
        title: event.title,
        startDate: ekrDateString(event.startDate) ?? "",
        endDate: ekrDateString(event.endDate) ?? "",
        calendarIdentifier: event.calendar?.calendarIdentifier,
        calendar: event.calendar.map(ekrEncodeCalendar),
        notes: event.notes,
        location: event.location,
        alarms: (event.alarms ?? []).map(ekrEncodeAlarm),
        recurrenceRules: (event.recurrenceRules ?? []).map(ekrEncodeRecurrenceRule),
        calendarItemIdentifier: event.calendarItemIdentifier,
        calendarItemExternalIdentifier: event.calendarItemExternalIdentifier,
        url: event.url?.absoluteString,
        lastModifiedDate: ekrDateString(event.lastModifiedDate),
        creationDate: ekrDateString(event.creationDate),
        timeZoneIdentifier: event.timeZone?.identifier,
        hasAlarms: event.hasAlarms,
        hasRecurrenceRules: event.hasRecurrenceRules,
        hasAttendees: event.hasAttendees,
        hasNotes: event.hasNotes,
        attendees: (event.attendees ?? []).map(ekrEncodeParticipant),
        allDay: event.isAllDay,
        structuredLocation: ekrEncodeStructuredLocation(event.structuredLocation),
        organizer: event.organizer.map(ekrEncodeParticipant),
        availability: ekrEventAvailabilityPayload(from: event.availability),
        status: ekrEventStatusPayload(from: event.status),
        isDetached: event.isDetached,
        occurrenceDate: ekrDateString(event.occurrenceDate),
        birthdayContactIdentifier: event.birthdayContactIdentifier,
        birthdayPersonID: (event.value(forKey: "birthdayPersonID") as? NSNumber)?.intValue,
        birthdayPersonUniqueID: event.birthdayPersonUniqueID
    )
}

@_cdecl("ek_event_compare_start_date_json")
public func ek_event_compare_start_date_json(
    _ lhsJSON: UnsafePointer<CChar>?,
    _ rhsJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let lhsPayload = try ekrDecodeJSON(lhsJSON, as: EKREventPayload.self)
        let rhsPayload = try ekrDecodeJSON(rhsJSON, as: EKREventPayload.self)
        let store = EKEventStore()
        let lhsEvent = try ekrPrepareEvent(store: store, payload: lhsPayload, requireCalendar: false)
        let rhsEvent = try ekrPrepareEvent(store: store, payload: rhsPayload, requireCalendar: false)
        return Int32(lhsEvent.compareStartDate(with: rhsEvent).rawValue)
    } catch {
        ekrSetError(outError, error)
        return 0
    }
}

@_cdecl("ek_event_roundtrip_json")
public func ek_event_roundtrip_json(
    _ store: UnsafeMutableRawPointer?,
    _ eventJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }

    do {
        let payload = try ekrDecodeJSON(eventJSON, as: EKREventPayload.self)
        let event = try ekrPrepareEvent(store: ekrBorrow(store, as: EKEventStore.self), payload: payload, requireCalendar: false)
        return ekrCString(try ekrEncodeJSON(ekrEncodeEvent(event)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_store_refresh_event_json")
public func ek_store_refresh_event_json(
    _ store: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }
    guard let identifier else {
        ekrSetMessageError(outError, message: "missing EKEvent identifier")
        return nil
    }

    let eventStore = ekrBorrow(store, as: EKEventStore.self)
    guard let event = eventStore.event(withIdentifier: String(cString: identifier)) else {
        return nil
    }

    guard event.refresh() else {
        return nil
    }

    do {
        return ekrCString(try ekrEncodeJSON(ekrEncodeEvent(event)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}
