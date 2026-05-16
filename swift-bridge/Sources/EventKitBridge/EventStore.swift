import EventKit
import Foundation

enum EKREntityType: String, Codable {
    case event
    case reminder
}

enum EKRReminderPredicateKind: String, Codable {
    case all
    case incomplete
    case completed
}

enum EKRCalendarItemKind: String, Codable {
    case event
    case reminder
}

struct EKREventPredicatePayload: Codable {
    var startDate: String
    var endDate: String
    var calendarIdentifiers: [String]?
}

struct EKRReminderPredicatePayload: Codable {
    var calendarIdentifiers: [String]?
    var kind: EKRReminderPredicateKind
    var startDate: String?
    var endDate: String?
}

struct EKRCalendarItemPayload: Codable {
    var kind: EKRCalendarItemKind
    var event: EKREventPayload?
    var reminder: EKRReminderPayload?
}

func ekrEntityType(from rawValue: Int32) throws -> EKEntityType {
    switch rawValue {
    case 0:
        return .event
    case 1:
        return .reminder
    default:
        throw NSError(
            domain: "eventkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "invalid EKEntityType raw value: \(rawValue)"]
        )
    }
}

func ekrEntityType(from payload: EKREntityType) -> EKEntityType {
    switch payload {
    case .event:
        return .event
    case .reminder:
        return .reminder
    }
}

func ekrAllowedEntityTypes(_ mask: EKEntityMask) -> [EKREntityType] {
    var allowed: [EKREntityType] = []
    if mask.rawValue & EKEntityMask.event.rawValue != 0 {
        allowed.append(.event)
    }
    if mask.rawValue & EKEntityMask.reminder.rawValue != 0 {
        allowed.append(.reminder)
    }
    return allowed
}

func ekrSpan(from rawValue: Int32) throws -> EKSpan {
    switch rawValue {
    case 0:
        return .thisEvent
    case 1:
        return .futureEvents
    default:
        throw NSError(
            domain: "eventkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "invalid EKSpan raw value: \(rawValue)"]
        )
    }
}

func ekrResolveCalendars(store: EKEventStore, identifiers: [String]?) -> [EKCalendar]? {
    identifiers.map { identifiers in
        identifiers.compactMap { store.calendar(withIdentifier: $0) }
    }
}

func ekrResolveSources(store: EKEventStore, identifiers: [String]) throws -> [EKSource] {
    let sources = identifiers.compactMap { store.source(withIdentifier: $0) }
    guard sources.count == identifiers.count else {
        let resolved = Set(sources.map { $0.sourceIdentifier })
        let missing = Set(identifiers).subtracting(resolved)
        throw NSError(
            domain: "eventkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "unknown source identifiers: \(Array(missing).sorted().joined(separator: ", "))"]
        )
    }
    return sources
}

func ekrEncodeCalendarItem(_ item: EKCalendarItem) throws -> EKRCalendarItemPayload {
    if let event = item as? EKEvent {
        return EKRCalendarItemPayload(kind: .event, event: ekrEncodeEvent(event), reminder: nil)
    }
    if let reminder = item as? EKReminder {
        return EKRCalendarItemPayload(kind: .reminder, event: nil, reminder: ekrEncodeReminder(reminder))
    }
    throw NSError(
        domain: "eventkit-rs",
        code: -1,
        userInfo: [NSLocalizedDescriptionKey: "unsupported EKCalendarItem subclass: \(type(of: item))"]
    )
}

func ekrRunAccessRequest(
    work: (@escaping (Bool, Error?) -> Void) -> Void,
    outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    let semaphore = DispatchSemaphore(value: 0)
    var granted = false
    var capturedError: Error?

    work { didGrant, error in
        granted = didGrant
        capturedError = error
        semaphore.signal()
    }

    _ = semaphore.wait(timeout: .now() + .seconds(30))
    if let capturedError {
        ekrSetError(outError, capturedError)
    }
    return granted
}

@_cdecl("ek_authorization_status")
public func ek_authorization_status(_ entityType: Int32) -> Int32 {
    do {
        return Int32(EKEventStore.authorizationStatus(for: try ekrEntityType(from: entityType)).rawValue)
    } catch {
        return -1
    }
}

@_cdecl("ek_store_new")
public func ek_store_new() -> UnsafeMutableRawPointer {
    ekrRetain(EKEventStore())
}

@_cdecl("ek_store_new_with_sources_json")
public func ek_store_new_with_sources_json(
    _ sourceIdentifiersJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let identifiers = try ekrDecodeJSON(sourceIdentifiersJSON, as: [String].self)
        let bootstrapStore = EKEventStore()
        let sources = try ekrResolveSources(store: bootstrapStore, identifiers: identifiers)
        return ekrRetain(EKEventStore(sources: sources))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_store_release")
public func ek_store_release(_ store: UnsafeMutableRawPointer?) {
    guard let store else { return }
    ekrRelease(store)
}

@_cdecl("ek_store_request_access_events")
public func ek_store_request_access_events(
    _ store: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return false
    }

    let eventStore = ekrBorrow(store, as: EKEventStore.self)
    if #available(macOS 14.0, *) {
        return ekrRunAccessRequest(work: { completion in
            eventStore.requestFullAccessToEvents(completion: completion)
        }, outError: outError)
    }

    return ekrRunAccessRequest(work: { completion in
        eventStore.requestAccess(to: .event, completion: completion)
    }, outError: outError)
}

@_cdecl("ek_store_request_full_access_events")
public func ek_store_request_full_access_events(
    _ store: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return false
    }

    let eventStore = ekrBorrow(store, as: EKEventStore.self)
    if #available(macOS 14.0, *) {
        return ekrRunAccessRequest(work: { completion in
            eventStore.requestFullAccessToEvents(completion: completion)
        }, outError: outError)
    }

    return ekrRunAccessRequest(work: { completion in
        eventStore.requestAccess(to: .event, completion: completion)
    }, outError: outError)
}

@_cdecl("ek_store_request_write_only_access_events")
public func ek_store_request_write_only_access_events(
    _ store: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return false
    }

    let eventStore = ekrBorrow(store, as: EKEventStore.self)
    if #available(macOS 14.0, *) {
        return ekrRunAccessRequest(work: { completion in
            eventStore.requestWriteOnlyAccessToEvents(completion: completion)
        }, outError: outError)
    }

    return ekrRunAccessRequest(work: { completion in
        eventStore.requestAccess(to: .event, completion: completion)
    }, outError: outError)
}

@_cdecl("ek_store_request_access_reminders")
public func ek_store_request_access_reminders(
    _ store: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return false
    }

    let eventStore = ekrBorrow(store, as: EKEventStore.self)
    if #available(macOS 14.0, *) {
        return ekrRunAccessRequest(work: { completion in
            eventStore.requestFullAccessToReminders(completion: completion)
        }, outError: outError)
    }

    return ekrRunAccessRequest(work: { completion in
        eventStore.requestAccess(to: .reminder, completion: completion)
    }, outError: outError)
}

@_cdecl("ek_store_request_full_access_reminders")
public func ek_store_request_full_access_reminders(
    _ store: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return false
    }

    let eventStore = ekrBorrow(store, as: EKEventStore.self)
    if #available(macOS 14.0, *) {
        return ekrRunAccessRequest(work: { completion in
            eventStore.requestFullAccessToReminders(completion: completion)
        }, outError: outError)
    }

    return ekrRunAccessRequest(work: { completion in
        eventStore.requestAccess(to: .reminder, completion: completion)
    }, outError: outError)
}

@_cdecl("ek_store_identifier")
public func ek_store_identifier(_ store: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let store else { return nil }
    return ekrCString(ekrBorrow(store, as: EKEventStore.self).eventStoreIdentifier)
}

@_cdecl("ek_store_calendar_item_json")
public func ek_store_calendar_item_json(
    _ store: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }
    guard let identifier else {
        ekrSetMessageError(outError, message: "missing EKCalendarItem identifier")
        return nil
    }

    do {
        let item = ekrBorrow(store, as: EKEventStore.self).calendarItem(withIdentifier: String(cString: identifier))
        guard let item else { return nil }
        return ekrCString(try ekrEncodeJSON(try ekrEncodeCalendarItem(item)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_store_calendar_items_external_json")
public func ek_store_calendar_items_external_json(
    _ store: UnsafeMutableRawPointer?,
    _ externalIdentifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }
    guard let externalIdentifier else {
        ekrSetMessageError(outError, message: "missing EKCalendarItem external identifier")
        return nil
    }

    do {
        let items = ekrBorrow(store, as: EKEventStore.self)
            .calendarItems(withExternalIdentifier: String(cString: externalIdentifier))
        let payload = try items.map(ekrEncodeCalendarItem)
        return ekrCString(try ekrEncodeJSON(payload))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_store_event_json")
public func ek_store_event_json(
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

    do {
        let event = ekrBorrow(store, as: EKEventStore.self).event(withIdentifier: String(cString: identifier))
        guard let event else { return nil }
        return ekrCString(try ekrEncodeJSON(ekrEncodeEvent(event)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_store_events_matching_json")
public func ek_store_events_matching_json(
    _ store: UnsafeMutableRawPointer?,
    _ predicateJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }

    do {
        let payload = try ekrDecodeJSON(predicateJSON, as: EKREventPredicatePayload.self)
        let eventStore = ekrBorrow(store, as: EKEventStore.self)
        let predicate = eventStore.predicateForEvents(
            withStart: try ekrDate(from: payload.startDate),
            end: try ekrDate(from: payload.endDate),
            calendars: ekrResolveCalendars(store: eventStore, identifiers: payload.calendarIdentifiers)
        )
        let events = eventStore.events(matching: predicate).map(ekrEncodeEvent)
        return ekrCString(try ekrEncodeJSON(events))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_store_fetch_reminders_json")
public func ek_store_fetch_reminders_json(
    _ store: UnsafeMutableRawPointer?,
    _ predicateJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }

    do {
        let payload = try ekrDecodeJSON(predicateJSON, as: EKRReminderPredicatePayload.self)
        let eventStore = ekrBorrow(store, as: EKEventStore.self)
        let calendars = ekrResolveCalendars(store: eventStore, identifiers: payload.calendarIdentifiers)
        let predicate: NSPredicate
        switch payload.kind {
        case .all:
            predicate = eventStore.predicateForReminders(in: calendars)
        case .incomplete:
            predicate = eventStore.predicateForIncompleteReminders(
                withDueDateStarting: try payload.startDate.map(ekrDate(from:)),
                ending: try payload.endDate.map(ekrDate(from:)),
                calendars: calendars
            )
        case .completed:
            predicate = eventStore.predicateForCompletedReminders(
                withCompletionDateStarting: try payload.startDate.map(ekrDate(from:)),
                ending: try payload.endDate.map(ekrDate(from:)),
                calendars: calendars
            )
        }

        let semaphore = DispatchSemaphore(value: 0)
        var reminders: [EKReminder] = []
        var completed = false
        let token = eventStore.fetchReminders(matching: predicate) { fetched in
            reminders = fetched ?? []
            completed = true
            semaphore.signal()
        }

        if semaphore.wait(timeout: .now() + .seconds(30)) == .timedOut {
            eventStore.cancelFetchRequest(token)
            throw NSError(
                domain: "eventkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "fetchReminders timed out"]
            )
        }
        guard completed else {
            throw NSError(
                domain: "eventkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "fetchReminders did not complete"]
            )
        }

        return ekrCString(try ekrEncodeJSON(reminders.map(ekrEncodeReminder)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}

@_cdecl("ek_store_save_event")
public func ek_store_save_event(
    _ store: UnsafeMutableRawPointer?,
    _ eventJSON: UnsafePointer<CChar>?,
    _ spanRaw: Int32,
    _ commit: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return EKR_ERROR
    }

    do {
        let payload = try ekrDecodeJSON(eventJSON, as: EKREventPayload.self)
        let eventStore = ekrBorrow(store, as: EKEventStore.self)
        let event = try ekrPrepareEvent(store: eventStore, payload: payload, requireCalendar: true)
        try eventStore.save(event, span: try ekrSpan(from: spanRaw), commit: commit)
        return EKR_OK
    } catch {
        ekrSetError(outError, error)
        return EKR_ERROR
    }
}

@_cdecl("ek_store_remove_event")
public func ek_store_remove_event(
    _ store: UnsafeMutableRawPointer?,
    _ eventJSON: UnsafePointer<CChar>?,
    _ spanRaw: Int32,
    _ commit: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return EKR_ERROR
    }

    do {
        let payload = try ekrDecodeJSON(eventJSON, as: EKREventPayload.self)
        guard let identifier = payload.identifier else {
            throw NSError(domain: "eventkit-rs", code: -1, userInfo: [NSLocalizedDescriptionKey: "removeEvent requires identifier"])
        }
        let eventStore = ekrBorrow(store, as: EKEventStore.self)
        guard let event = eventStore.event(withIdentifier: identifier) else {
            throw NSError(domain: "eventkit-rs", code: -1, userInfo: [NSLocalizedDescriptionKey: "event not found: \(identifier)"])
        }
        try eventStore.remove(event, span: try ekrSpan(from: spanRaw), commit: commit)
        return EKR_OK
    } catch {
        ekrSetError(outError, error)
        return EKR_ERROR
    }
}

@_cdecl("ek_store_save_reminder")
public func ek_store_save_reminder(
    _ store: UnsafeMutableRawPointer?,
    _ reminderJSON: UnsafePointer<CChar>?,
    _ commit: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return EKR_ERROR
    }

    do {
        let payload = try ekrDecodeJSON(reminderJSON, as: EKRReminderPayload.self)
        let eventStore = ekrBorrow(store, as: EKEventStore.self)
        let reminder = try ekrPrepareReminder(store: eventStore, payload: payload, requireCalendar: true)
        try eventStore.save(reminder, commit: commit)
        return EKR_OK
    } catch {
        ekrSetError(outError, error)
        return EKR_ERROR
    }
}

@_cdecl("ek_store_remove_reminder")
public func ek_store_remove_reminder(
    _ store: UnsafeMutableRawPointer?,
    _ reminderJSON: UnsafePointer<CChar>?,
    _ commit: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return EKR_ERROR
    }

    do {
        let payload = try ekrDecodeJSON(reminderJSON, as: EKRReminderPayload.self)
        guard let identifier = payload.identifier else {
            throw NSError(domain: "eventkit-rs", code: -1, userInfo: [NSLocalizedDescriptionKey: "removeReminder requires identifier"])
        }
        let eventStore = ekrBorrow(store, as: EKEventStore.self)
        guard let reminder = eventStore.calendarItem(withIdentifier: identifier) as? EKReminder else {
            throw NSError(domain: "eventkit-rs", code: -1, userInfo: [NSLocalizedDescriptionKey: "reminder not found: \(identifier)"])
        }
        try eventStore.remove(reminder, commit: commit)
        return EKR_OK
    } catch {
        ekrSetError(outError, error)
        return EKR_ERROR
    }
}

@_cdecl("ek_store_commit")
public func ek_store_commit(
    _ store: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return EKR_ERROR
    }

    do {
        try ekrBorrow(store, as: EKEventStore.self).commit()
        return EKR_OK
    } catch {
        ekrSetError(outError, error)
        return EKR_ERROR
    }
}

@_cdecl("ek_store_reset")
public func ek_store_reset(_ store: UnsafeMutableRawPointer?) {
    guard let store else { return }
    ekrBorrow(store, as: EKEventStore.self).reset()
}

@_cdecl("ek_store_refresh_sources_if_necessary")
public func ek_store_refresh_sources_if_necessary(_ store: UnsafeMutableRawPointer?) {
    guard let store else { return }
    ekrBorrow(store, as: EKEventStore.self).refreshSourcesIfNecessary()
}
