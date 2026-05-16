import EventKit
import Foundation

enum EKRRecurrenceFrequency: String, Codable {
    case daily
    case weekly
    case monthly
    case yearly
}

enum EKRWeekday: String, Codable {
    case sunday
    case monday
    case tuesday
    case wednesday
    case thursday
    case friday
    case saturday
}

struct EKRRecurrenceDayOfWeekPayload: Codable {
    var dayOfTheWeek: EKRWeekday
    var weekNumber: Int
}

struct EKRRecurrenceRulePayload: Codable {
    var frequency: EKRRecurrenceFrequency
    var interval: Int
    var endDate: String?
    var occurrenceCount: Int?
    var calendarIdentifier: String?
    var firstDayOfTheWeek: EKRWeekday?
    var daysOfTheWeek: [EKRRecurrenceDayOfWeekPayload]
    var daysOfTheMonth: [Int]
    var monthsOfTheYear: [Int]
    var weeksOfTheYear: [Int]
    var daysOfTheYear: [Int]
    var setPositions: [Int]
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

func ekrWeekday(from weekday: EKRWeekday) -> EKWeekday {
    switch weekday {
    case .sunday:
        return .sunday
    case .monday:
        return .monday
    case .tuesday:
        return .tuesday
    case .wednesday:
        return .wednesday
    case .thursday:
        return .thursday
    case .friday:
        return .friday
    case .saturday:
        return .saturday
    }
}

func ekrWeekdayPayload(from weekday: EKWeekday) -> EKRWeekday {
    switch weekday {
    case .sunday:
        return .sunday
    case .monday:
        return .monday
    case .tuesday:
        return .tuesday
    case .wednesday:
        return .wednesday
    case .thursday:
        return .thursday
    case .friday:
        return .friday
    case .saturday:
        return .saturday
    @unknown default:
        return .sunday
    }
}

func ekrWeekdayPayload(from rawValue: Int) -> EKRWeekday? {
    guard let weekday = EKWeekday(rawValue: rawValue) else { return nil }
    return ekrWeekdayPayload(from: weekday)
}

func ekrEncodeRecurrenceDayOfWeek(_ day: EKRecurrenceDayOfWeek) -> EKRRecurrenceDayOfWeekPayload {
    EKRRecurrenceDayOfWeekPayload(
        dayOfTheWeek: ekrWeekdayPayload(from: day.dayOfTheWeek),
        weekNumber: day.weekNumber
    )
}

func ekrDecodeRecurrenceDayOfWeek(_ payload: EKRRecurrenceDayOfWeekPayload) -> EKRecurrenceDayOfWeek {
    EKRecurrenceDayOfWeek(dayOfTheWeek: ekrWeekday(from: payload.dayOfTheWeek), weekNumber: payload.weekNumber)
}

func ekrOptionalArray<T>(_ values: [T]) -> [T]? {
    values.isEmpty ? nil : values
}

func ekrEncodeRecurrenceRule(_ rule: EKRecurrenceRule) -> EKRRecurrenceRulePayload {
    let recurrenceEnd = rule.recurrenceEnd
    return EKRRecurrenceRulePayload(
        frequency: ekrRecurrenceFrequencyPayload(from: rule.frequency),
        interval: rule.interval,
        endDate: ekrDateString(recurrenceEnd?.endDate),
        occurrenceCount: recurrenceEnd?.occurrenceCount == 0 ? nil : recurrenceEnd?.occurrenceCount,
        calendarIdentifier: rule.calendarIdentifier,
        firstDayOfTheWeek: ekrWeekdayPayload(from: rule.firstDayOfTheWeek),
        daysOfTheWeek: (rule.daysOfTheWeek ?? []).map(ekrEncodeRecurrenceDayOfWeek),
        daysOfTheMonth: (rule.daysOfTheMonth ?? []).map { $0.intValue },
        monthsOfTheYear: (rule.monthsOfTheYear ?? []).map { $0.intValue },
        weeksOfTheYear: (rule.weeksOfTheYear ?? []).map { $0.intValue },
        daysOfTheYear: (rule.daysOfTheYear ?? []).map { $0.intValue },
        setPositions: (rule.setPositions ?? []).map { $0.intValue }
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
        daysOfTheWeek: ekrOptionalArray(payload.daysOfTheWeek.map(ekrDecodeRecurrenceDayOfWeek)),
        daysOfTheMonth: ekrOptionalArray(payload.daysOfTheMonth.map(NSNumber.init(value:))),
        monthsOfTheYear: ekrOptionalArray(payload.monthsOfTheYear.map(NSNumber.init(value:))),
        weeksOfTheYear: ekrOptionalArray(payload.weeksOfTheYear.map(NSNumber.init(value:))),
        daysOfTheYear: ekrOptionalArray(payload.daysOfTheYear.map(NSNumber.init(value:))),
        setPositions: ekrOptionalArray(payload.setPositions.map(NSNumber.init(value:))),
        end: recurrenceEnd
    )
}

@_cdecl("ek_recurrence_rule_roundtrip_json")
public func ek_recurrence_rule_roundtrip_json(
    _ ruleJSON: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let payload = try ekrDecodeJSON(ruleJSON, as: EKRRecurrenceRulePayload.self)
        let rule = try ekrDecodeRecurrenceRule(payload)
        return ekrCString(try ekrEncodeJSON(ekrEncodeRecurrenceRule(rule)))
    } catch {
        ekrSetError(outError, error)
        return nil
    }
}
