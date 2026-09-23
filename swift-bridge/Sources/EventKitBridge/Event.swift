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

func ekrSameInstant(_ lhs: Date?, _ rhs: Date) -> Bool {
    guard let lhs else { return false }
    return abs(lhs.timeIntervalSince(rhs)) < 0.001
}

func ekrResolveEvent(
    store: EKEventStore,
    identifier: String,
    occurrenceDate occurrenceDateString: String?,
    near anchors: [Date]
) throws -> EKEvent? {
    guard let first = store.event(withIdentifier: identifier) else { return nil }
    guard let occurrenceDateString, first.hasRecurrenceRules || first.isDetached else { return first }
    let occurrenceDate = try ekrDate(from: occurrenceDateString)
    if ekrSameInstant(first.occurrenceDate, occurrenceDate) {
        return first
    }
    let window: TimeInterval = 2 * 86_400
    for anchor in [occurrenceDate] + anchors {
        let predicate = store.predicateForEvents(
            withStart: anchor.addingTimeInterval(-window),
            end: anchor.addingTimeInterval(window),
            calendars: first.calendar.map { [$0] }
        )
        let occurrence = store.events(matching: predicate).first {
            $0.eventIdentifier == identifier && ekrSameInstant($0.occurrenceDate, occurrenceDate)
        }
        if let occurrence {
            return occurrence
        }
    }
    throw ekrInvalidArgument("event \(identifier) has no occurrence on \(occurrenceDateString)")
}

func ekrPrepareEvent(
    store: EKEventStore,
    payload: EKREventPayload,
    requireCalendar: Bool
) throws -> EKEvent {
    let startDate = try ekrDate(from: payload.startDate)
    let endDate = try ekrDate(from: payload.endDate)
    let event: EKEvent
    if let identifier = payload.identifier,
       let existing = try ekrResolveEvent(store: store, identifier: identifier, occurrenceDate: payload.occurrenceDate, near: [startDate]) {
        event = existing
    } else {
        event = EKEvent(eventStore: store)
    }

    if event.title != payload.title {
        event.title = payload.title
    }
    if !ekrSameInstant(event.startDate, startDate) {
        event.startDate = startDate
    }
    if !ekrSameInstant(event.endDate, endDate) {
        event.endDate = endDate
    }
    if event.isAllDay != payload.allDay {
        event.isAllDay = payload.allDay
    }
    if event.notes != payload.notes {
        event.notes = payload.notes
    }
    if event.location != payload.location {
        event.location = payload.location
    }
    if event.url?.absoluteString != payload.url {
        event.url = payload.url.flatMap(URL.init(string:))
    }
    if event.timeZone?.identifier != payload.timeZoneIdentifier {
        event.timeZone = payload.timeZoneIdentifier.flatMap(TimeZone.init(identifier:))
    }
    if ekrEncodeStructuredLocation(event.structuredLocation) != payload.structuredLocation {
        event.structuredLocation = ekrDecodeStructuredLocation(payload.structuredLocation)
    }
    let availability = ekrEventAvailability(from: payload.availability)
    if event.availability != availability {
        event.availability = availability
    }

    let calendarIdentifier = payload.calendarIdentifier ?? payload.calendar?.identifier
    if let calendarIdentifier {
        guard let calendar = store.calendar(withIdentifier: calendarIdentifier) else {
            throw NSError(
                domain: "eventkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "unknown event calendar identifier: \(calendarIdentifier)"]
            )
        }
        if event.calendar?.calendarIdentifier != calendar.calendarIdentifier {
            event.calendar = calendar
        }
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

    if (event.alarms ?? []).map(ekrEncodeAlarm) != payload.alarms {
        event.alarms = try payload.alarms.map(ekrDecodeAlarm)
    }
    if (event.recurrenceRules ?? []).map(ekrEncodeRecurrenceRule) != payload.recurrenceRules {
        event.recurrenceRules = try payload.recurrenceRules.map(ekrDecodeRecurrenceRule)
    }
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
    _ occurrenceDate: UnsafePointer<CChar>?,
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
    do {
        let event = try ekrResolveEvent(
            store: eventStore,
            identifier: String(cString: identifier),
            occurrenceDate: occurrenceDate.map { String(cString: $0) },
            near: []
        )
        guard let event, event.refresh() else {
            return nil
        }
        return ekrCString(try ekrEncodeJSON(ekrEncodeEvent(event)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}
