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

struct EKRRecurrenceDayOfWeekPayload: Codable, Equatable {
    var dayOfTheWeek: EKRWeekday
    var weekNumber: Int
}

struct EKRRecurrenceRulePayload: Codable, Equatable {
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

private func ekrValidateRecurrenceValues(_ values: [Int], name: String, limit: UInt, signed: Bool) throws {
    for value in values {
        let valid = signed ? value != 0 && value.magnitude <= limit : value >= 1 && value.magnitude <= limit
        guard valid else {
            let range = signed ? "between -\(limit) and \(limit) and not 0" : "between 1 and \(limit)"
            throw ekrInvalidArgument("recurrence rule \(name) must be \(range), got \(value)")
        }
    }
}

func ekrValidateRecurrenceRule(_ payload: EKRRecurrenceRulePayload) throws {
    guard payload.interval > 0 else {
        throw ekrInvalidArgument("recurrence rule interval must be greater than 0, got \(payload.interval)")
    }
    if payload.endDate == nil, let occurrenceCount = payload.occurrenceCount, occurrenceCount <= 0 {
        throw ekrInvalidArgument("recurrence rule occurrence count must be greater than 0, got \(occurrenceCount)")
    }
    for day in payload.daysOfTheWeek {
        guard day.weekNumber.magnitude <= 53 else {
            throw ekrInvalidArgument("recurrence day-of-week week number must be between -53 and 53, got \(day.weekNumber)")
        }
        if payload.frequency == .weekly, day.weekNumber != 0 {
            throw ekrInvalidArgument("weekly recurrence rules need a day-of-week week number of 0, got \(day.weekNumber)")
        }
    }
    try ekrValidateRecurrenceValues(payload.daysOfTheMonth, name: "days of the month", limit: 31, signed: true)
    try ekrValidateRecurrenceValues(payload.monthsOfTheYear, name: "months of the year", limit: 12, signed: false)
    try ekrValidateRecurrenceValues(payload.weeksOfTheYear, name: "weeks of the year", limit: 53, signed: true)
    try ekrValidateRecurrenceValues(payload.daysOfTheYear, name: "days of the year", limit: 366, signed: true)
    try ekrValidateRecurrenceValues(payload.setPositions, name: "set positions", limit: 366, signed: true)
}

func ekrDecodeRecurrenceRule(_ payload: EKRRecurrenceRulePayload) throws -> EKRecurrenceRule {
    try ekrValidateRecurrenceRule(payload)
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
