import CoreGraphics
import EventKit
import Foundation

enum EKREntityType: Int32, Codable {
    case event = 0
    case reminder = 1
}

enum EKRCalendarType: String, Codable {
    case local
    case calDav
    case exchange
    case subscription
    case birthday
}

enum EKRRecurrenceFrequency: String, Codable {
    case daily
    case weekly
    case monthly
    case yearly
}

enum EKRAlarmProximity: String, Codable {
    case none
    case enter
    case leave
}

struct EKRDateComponentsPayload: Codable {
    var era: Int?
    var year: Int?
    var month: Int?
    var day: Int?
    var hour: Int?
    var minute: Int?
    var second: Int?
    var isLeapMonth: Bool?
    var timeZoneIdentifier: String?
}

struct EKRAlarmPayload: Codable {
    var absoluteDate: String?
    var relativeOffset: Double?
    var proximity: EKRAlarmProximity?
    var emailAddress: String?
    var soundName: String?
}

struct EKRRecurrenceRulePayload: Codable {
    var frequency: EKRRecurrenceFrequency
    var interval: Int
    var endDate: String?
    var occurrenceCount: Int?
}

struct EKRCalendarPayload: Codable {
    var identifier: String
    var title: String
    var calendarType: EKRCalendarType
    var allowedEntityTypes: [EKREntityType]
    var color: String?
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
}

struct EKRReminderPayload: Codable {
    var identifier: String?
    var title: String
    var calendarIdentifier: String?
    var calendar: EKRCalendarPayload?
    var dueDateComponents: EKRDateComponentsPayload?
    var isCompleted: Bool
    var priority: Int
    var notes: String?
    var alarms: [EKRAlarmPayload]
    var recurrenceRules: [EKRRecurrenceRulePayload]
}

struct EKREventPredicatePayload: Codable {
    var startDate: String
    var endDate: String
    var calendarIdentifiers: [String]?
}

struct EKRReminderPredicatePayload: Codable {
    var calendarIdentifiers: [String]?
}

func ekrEntityType(from rawValue: Int32) throws -> EKEntityType {
    guard let entityType = EKREntityType(rawValue: rawValue) else {
        throw NSError(
            domain: "eventkit-rs",
            code: -1,
            userInfo: [NSLocalizedDescriptionKey: "invalid EKEntityType raw value: \(rawValue)"]
        )
    }

    switch entityType {
    case .event:
        return .event
    case .reminder:
        return .reminder
    }
}

func ekrEntityTypePayload(from entityType: EKEntityType) -> EKREntityType {
    switch entityType {
    case .event:
        return .event
    case .reminder:
        return .reminder
    @unknown default:
        return .event
    }
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

func ekrRecurrenceFrequency(from frequency: EKRRecurrenceFrequency) -> EKRecurrenceFrequency {
    switch frequency {
    case .daily:
        return .daily
    case .weekly:
        return .weekly
    case .monthly:
        return .monthly
    case .yearly:
        return .yearly
    }
}

func ekrRecurrenceFrequencyPayload(from frequency: EKRecurrenceFrequency) -> EKRRecurrenceFrequency {
    switch frequency {
    case .daily:
        return .daily
    case .weekly:
        return .weekly
    case .monthly:
        return .monthly
    case .yearly:
        return .yearly
    @unknown default:
        return .daily
    }
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

func ekrEncodeDateComponents(_ components: DateComponents?) -> EKRDateComponentsPayload? {
    guard let components else { return nil }
    return EKRDateComponentsPayload(
        era: components.era,
        year: components.year,
        month: components.month,
        day: components.day,
        hour: components.hour,
        minute: components.minute,
        second: components.second,
        isLeapMonth: components.isLeapMonth,
        timeZoneIdentifier: components.timeZone?.identifier
    )
}

func ekrDecodeDateComponents(_ payload: EKRDateComponentsPayload?) -> DateComponents? {
    guard let payload else { return nil }
    var components = DateComponents()
    components.era = payload.era
    components.year = payload.year
    components.month = payload.month
    components.day = payload.day
    components.hour = payload.hour
    components.minute = payload.minute
    components.second = payload.second
    components.isLeapMonth = payload.isLeapMonth
    components.timeZone = payload.timeZoneIdentifier.flatMap(TimeZone.init(identifier:))
    return components
}

func ekrEncodeAlarm(_ alarm: EKAlarm) -> EKRAlarmPayload {
    EKRAlarmPayload(
        absoluteDate: ekrDateString(alarm.absoluteDate),
        relativeOffset: alarm.relativeOffset,
        proximity: ekrAlarmProximityPayload(from: alarm.proximity),
        emailAddress: alarm.emailAddress,
        soundName: alarm.soundName
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

    if let proximity = payload.proximity {
        alarm.proximity = ekrAlarmProximity(from: proximity)
    }
    alarm.emailAddress = payload.emailAddress
    alarm.soundName = payload.soundName
    return alarm
}

func ekrEncodeRecurrenceRule(_ rule: EKRecurrenceRule) -> EKRRecurrenceRulePayload {
    let recurrenceEnd = rule.recurrenceEnd
    return EKRRecurrenceRulePayload(
        frequency: ekrRecurrenceFrequencyPayload(from: rule.frequency),
        interval: rule.interval,
        endDate: ekrDateString(recurrenceEnd?.endDate),
        occurrenceCount: recurrenceEnd?.occurrenceCount == 0 ? nil : recurrenceEnd?.occurrenceCount
    )
}

func ekrDecodeRecurrenceRule(_ payload: EKRRecurrenceRulePayload) throws -> EKRecurrenceRule {
    let recurrenceEnd: EKRecurrenceEnd?
    if let endDate = payload.endDate {
        recurrenceEnd = EKRecurrenceEnd(end: try ekrDate(from: endDate))
    } else if let occurrenceCount = payload.occurrenceCount {
        recurrenceEnd = EKRecurrenceEnd(occurrenceCount: occurrenceCount)
    } else {
        recurrenceEnd = nil
    }

    return EKRecurrenceRule(
        recurrenceWith: ekrRecurrenceFrequency(from: payload.frequency),
        interval: payload.interval,
        end: recurrenceEnd
    )
}

func ekrEncodeCalendar(_ calendar: EKCalendar) -> EKRCalendarPayload {
    EKRCalendarPayload(
        identifier: calendar.calendarIdentifier,
        title: calendar.title,
        calendarType: ekrCalendarTypePayload(from: calendar.type),
        allowedEntityTypes: ekrAllowedEntityTypes(calendar.allowedEntityTypes),
        color: ekrColorString(calendar.cgColor)
    )
}

func ekrResolveCalendars(
    store: EKEventStore,
    identifiers: [String]?
) -> [EKCalendar]? {
    identifiers.map { identifiers in
        identifiers.compactMap { store.calendar(withIdentifier: $0) }
    }
}

func ekrPrepareEvent(store: EKEventStore, payload: EKREventPayload) throws -> EKEvent {
    let event: EKEvent
    if let identifier = payload.identifier, let existing = store.event(withIdentifier: identifier) {
        event = existing
    } else {
        event = EKEvent(eventStore: store)
    }

    event.title = payload.title
    event.startDate = try ekrDate(from: payload.startDate)
    event.endDate = try ekrDate(from: payload.endDate)
    event.notes = payload.notes
    event.location = payload.location

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
    } else if event.calendar == nil {
        guard let calendar = store.defaultCalendarForNewEvents else {
            throw NSError(
                domain: "eventkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "no default calendar for new events"]
            )
        }
        event.calendar = calendar
    }

    event.alarms = payload.alarms.map { try? ekrDecodeAlarm($0) }.compactMap { $0 }
    event.recurrenceRules = payload.recurrenceRules.map { try? ekrDecodeRecurrenceRule($0) }.compactMap { $0 }
    return event
}

func ekrPrepareReminder(store: EKEventStore, payload: EKRReminderPayload) throws -> EKReminder {
    let reminder: EKReminder
    if let identifier = payload.identifier,
       let existing = store.calendarItem(withIdentifier: identifier) as? EKReminder {
        reminder = existing
    } else {
        reminder = EKReminder(eventStore: store)
    }

    reminder.title = payload.title
    reminder.notes = payload.notes
    reminder.dueDateComponents = ekrDecodeDateComponents(payload.dueDateComponents)
    reminder.isCompleted = payload.isCompleted
    reminder.priority = payload.priority

    let calendarIdentifier = payload.calendarIdentifier ?? payload.calendar?.identifier
    if let calendarIdentifier {
        guard let calendar = store.calendar(withIdentifier: calendarIdentifier) else {
            throw NSError(
                domain: "eventkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "unknown reminder calendar identifier: \(calendarIdentifier)"]
            )
        }
        reminder.calendar = calendar
    } else if reminder.calendar == nil {
        guard let calendar = store.defaultCalendarForNewReminders() else {
            throw NSError(
                domain: "eventkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "no default calendar for new reminders"]
            )
        }
        reminder.calendar = calendar
    }

    reminder.alarms = payload.alarms.map { try? ekrDecodeAlarm($0) }.compactMap { $0 }
    reminder.recurrenceRules = payload.recurrenceRules.map { try? ekrDecodeRecurrenceRule($0) }.compactMap { $0 }
    return reminder
}

func ekrEncodeEvent(_ event: EKEvent) -> EKREventPayload {
    EKREventPayload(
        identifier: event.eventIdentifier,
        title: event.title,
        startDate: ekrDateString(event.startDate) ?? "",
        endDate: ekrDateString(event.endDate) ?? "",
        calendarIdentifier: event.calendar.calendarIdentifier,
        calendar: ekrEncodeCalendar(event.calendar),
        notes: event.notes,
        location: event.location,
        alarms: (event.alarms ?? []).map(ekrEncodeAlarm),
        recurrenceRules: (event.recurrenceRules ?? []).map(ekrEncodeRecurrenceRule)
    )
}

func ekrEncodeReminder(_ reminder: EKReminder) -> EKRReminderPayload {
    EKRReminderPayload(
        identifier: reminder.calendarItemIdentifier,
        title: reminder.title,
        calendarIdentifier: reminder.calendar.calendarIdentifier,
        calendar: ekrEncodeCalendar(reminder.calendar),
        dueDateComponents: ekrEncodeDateComponents(reminder.dueDateComponents),
        isCompleted: reminder.isCompleted,
        priority: reminder.priority,
        notes: reminder.notes,
        alarms: (reminder.alarms ?? []).map(ekrEncodeAlarm),
        recurrenceRules: (reminder.recurrenceRules ?? []).map(ekrEncodeRecurrenceRule)
    )
}

func ekrRunAccessRequest(
    store: EKEventStore,
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
        return ekrRunAccessRequest(store: eventStore, work: { completion in
            eventStore.requestFullAccessToEvents(completion: completion)
        }, outError: outError)
    }

    return ekrRunAccessRequest(store: eventStore, work: { completion in
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
        return ekrRunAccessRequest(store: eventStore, work: { completion in
            eventStore.requestFullAccessToReminders(completion: completion)
        }, outError: outError)
    }

    return ekrRunAccessRequest(store: eventStore, work: { completion in
        eventStore.requestAccess(to: .reminder, completion: completion)
    }, outError: outError)
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
        let predicate = eventStore.predicateForReminders(in: ekrResolveCalendars(store: eventStore, identifiers: payload.calendarIdentifiers))
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
        let event = try ekrPrepareEvent(store: eventStore, payload: payload)
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
        let reminder = try ekrPrepareReminder(store: eventStore, payload: payload)
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

    let eventStore = ekrBorrow(store, as: EKEventStore.self)
    do {
        try eventStore.commit()
        return EKR_OK
    } catch {
        ekrSetError(outError, error)
        return EKR_ERROR
    }
}
