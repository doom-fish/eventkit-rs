import EventKit
import Foundation

struct EKRDateComponentsPayload: Codable {
    var era: Int?
    var year: Int?
    var month: Int?
    var day: Int?
    var hour: Int?
    var minute: Int?
    var second: Int?
    var nanosecond: Int?
    var weekday: Int?
    var weekdayOrdinal: Int?
    var quarter: Int?
    var weekOfMonth: Int?
    var weekOfYear: Int?
    var yearForWeekOfYear: Int?
    var isLeapMonth: Bool?
    var timeZoneIdentifier: String?
    var calendarIdentifier: String?
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
    var startDateComponents: EKRDateComponentsPayload?
    var completionDate: String?
    var location: String?
    var url: String?
    var lastModifiedDate: String?
    var creationDate: String?
    var timeZoneIdentifier: String?
    var hasAlarms: Bool
    var hasRecurrenceRules: Bool
    var hasAttendees: Bool
    var hasNotes: Bool
    var attendees: [EKRParticipantPayload]
}

func ekrCalendarIdentifierString(_ calendar: Calendar?) -> String? {
    guard let identifier = calendar?.identifier else { return nil }
    switch identifier {
    case .gregorian:
        return "gregorian"
    case .buddhist:
        return "buddhist"
    case .chinese:
        return "chinese"
    case .coptic:
        return "coptic"
    case .ethiopicAmeteMihret:
        return "ethiopic-amete-mihret"
    case .ethiopicAmeteAlem:
        return "ethiopic-amete-alem"
    case .hebrew:
        return "hebrew"
    case .iso8601:
        return "iso8601"
    case .indian:
        return "indian"
    case .islamic:
        return "islamic"
    case .islamicCivil:
        return "islamic-civil"
    case .japanese:
        return "japanese"
    case .persian:
        return "persian"
    case .republicOfChina:
        return "roc"
    @unknown default:
        return nil
    }
}

func ekrCalendarIdentifier(from value: String) -> Calendar.Identifier? {
    switch value {
    case "gregorian":
        return .gregorian
    case "buddhist":
        return .buddhist
    case "chinese":
        return .chinese
    case "coptic":
        return .coptic
    case "ethiopic-amete-mihret":
        return .ethiopicAmeteMihret
    case "ethiopic-amete-alem":
        return .ethiopicAmeteAlem
    case "hebrew":
        return .hebrew
    case "iso8601":
        return .iso8601
    case "indian":
        return .indian
    case "islamic":
        return .islamic
    case "islamic-civil":
        return .islamicCivil
    case "japanese":
        return .japanese
    case "persian":
        return .persian
    case "roc":
        return .republicOfChina
    default:
        return nil
    }
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
        nanosecond: components.nanosecond,
        weekday: components.weekday,
        weekdayOrdinal: components.weekdayOrdinal,
        quarter: components.quarter,
        weekOfMonth: components.weekOfMonth,
        weekOfYear: components.weekOfYear,
        yearForWeekOfYear: components.yearForWeekOfYear,
        isLeapMonth: components.isLeapMonth,
        timeZoneIdentifier: components.timeZone?.identifier,
        calendarIdentifier: ekrCalendarIdentifierString(components.calendar)
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
    components.nanosecond = payload.nanosecond
    components.weekday = payload.weekday
    components.weekdayOrdinal = payload.weekdayOrdinal
    components.quarter = payload.quarter
    components.weekOfMonth = payload.weekOfMonth
    components.weekOfYear = payload.weekOfYear
    components.yearForWeekOfYear = payload.yearForWeekOfYear
    components.isLeapMonth = payload.isLeapMonth
    components.timeZone = payload.timeZoneIdentifier.flatMap(TimeZone.init(identifier:))
    if let identifier = payload.calendarIdentifier,
       let calendarIdentifier = ekrCalendarIdentifier(from: identifier) {
        components.calendar = Calendar(identifier: calendarIdentifier)
    }
    return components
}

func ekrPrepareReminder(
    store: EKEventStore,
    payload: EKRReminderPayload,
    requireCalendar: Bool
) throws -> EKReminder {
    let reminder: EKReminder
    if let identifier = payload.identifier,
       let existing = store.calendarItem(withIdentifier: identifier) as? EKReminder {
        reminder = existing
    } else {
        reminder = EKReminder(eventStore: store)
    }

    reminder.title = payload.title
    reminder.notes = payload.notes
    reminder.location = payload.location
    reminder.url = payload.url.flatMap(URL.init(string:))
    reminder.timeZone = payload.timeZoneIdentifier.flatMap(TimeZone.init(identifier:))
    reminder.startDateComponents = ekrDecodeDateComponents(payload.startDateComponents)
    reminder.dueDateComponents = ekrDecodeDateComponents(payload.dueDateComponents)
    reminder.priority = payload.priority
    if let completionDate = payload.completionDate {
        reminder.completionDate = try ekrDate(from: completionDate)
    } else {
        reminder.isCompleted = payload.isCompleted
    }

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
    } else if requireCalendar, reminder.calendar == nil {
        guard let calendar = store.defaultCalendarForNewReminders() else {
            throw NSError(
                domain: "eventkit-rs",
                code: -1,
                userInfo: [NSLocalizedDescriptionKey: "no default calendar for new reminders"]
            )
        }
        reminder.calendar = calendar
    }

    reminder.alarms = payload.alarms.compactMap { try? ekrDecodeAlarm($0) }
    reminder.recurrenceRules = payload.recurrenceRules.compactMap { try? ekrDecodeRecurrenceRule($0) }
    return reminder
}

func ekrEncodeReminder(_ reminder: EKReminder) -> EKRReminderPayload {
    EKRReminderPayload(
        identifier: reminder.calendarItemIdentifier,
        title: reminder.title,
        calendarIdentifier: reminder.calendar?.calendarIdentifier,
        calendar: reminder.calendar.map(ekrEncodeCalendar),
        dueDateComponents: ekrEncodeDateComponents(reminder.dueDateComponents),
        isCompleted: reminder.isCompleted,
        priority: reminder.priority,
        notes: reminder.notes,
        alarms: (reminder.alarms ?? []).map(ekrEncodeAlarm),
        recurrenceRules: (reminder.recurrenceRules ?? []).map(ekrEncodeRecurrenceRule),
        startDateComponents: ekrEncodeDateComponents(reminder.startDateComponents),
        completionDate: ekrDateString(reminder.completionDate),
        location: reminder.location,
        url: reminder.url?.absoluteString,
        lastModifiedDate: ekrDateString(reminder.lastModifiedDate),
        creationDate: ekrDateString(reminder.creationDate),
        timeZoneIdentifier: reminder.timeZone?.identifier,
        hasAlarms: reminder.hasAlarms,
        hasRecurrenceRules: reminder.hasRecurrenceRules,
        hasAttendees: reminder.hasAttendees,
        hasNotes: reminder.hasNotes,
        attendees: (reminder.attendees ?? []).map(ekrEncodeParticipant)
    )
}

@_cdecl("ek_reminder_roundtrip_json")
public func ek_reminder_roundtrip_json(
    _ store: UnsafeMutableRawPointer?,
    _ reminderJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let store else {
        ekrSetMessageError(outError, message: "missing EKEventStore")
        return nil
    }

    do {
        let payload = try ekrDecodeJSON(reminderJSON, as: EKRReminderPayload.self)
        let reminder = try ekrPrepareReminder(store: ekrBorrow(store, as: EKEventStore.self), payload: payload, requireCalendar: false)
        return ekrCString(try ekrEncodeJSON(ekrEncodeReminder(reminder)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}
