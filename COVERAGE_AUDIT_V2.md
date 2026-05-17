# eventkit-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 46
VERIFIED: 37
GAPS: 0
EXEMPT: 9
COVERAGE_PCT: 100.0

Re-verified against MacOSX26.2.sdk headers by exhaustive enumeration of EventKit.framework public interfaces, enum/options typedefs, extern constants, and deprecated enum aliases. No changes to symbol count or coverage since v1 audit.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `EKAlarm` | interface | `EKAlarm.h` | `EKAlarm` (`src/alarm.rs`), bridged by `ek_alarm_roundtrip_json` (`swift-bridge/Sources/EventKitBridge/Alarm.swift`) |
| `EKAlarmProximity` | typedef enum | `EKTypes.h` | `EKAlarmProximity` (`src/alarm.rs`) |
| `EKAlarmType` | typedef enum | `EKTypes.h` | `EKAlarmType` (`src/alarm.rs`) |
| `EKAuthorizationStatus` | typedef enum | `EKTypes.h` | `EKAuthorizationStatus` + `EKEventStore::authorization_status` (`src/error.rs`, `src/event_store.rs`) |
| `EKCalendar` | interface | `EKCalendar.h` | `EKCalendar`, `EKCalendarDraft`, and `EKEventStore` calendar lookup/save APIs (`src/calendar.rs`, `src/event_store.rs`) |
| `EKCalendarEventAvailabilityMask` | typedef options | `EKTypes.h` | Safe adapter `Vec<EKCalendarEventAvailability>` on `EKCalendar::supported_event_availabilities` (`src/calendar.rs`) |
| `EKCalendarItem` | interface | `EKCalendarItem.h` | `EKCalendarItem` + `EKEventStore::{calendar_item_with_identifier,calendar_items_with_external_identifier}` (`src/event_store.rs`) |
| `EKCalendarType` | typedef enum | `EKTypes.h` | `EKCalendarType` (`src/calendar.rs`) |
| `EKEntityMask` | typedef options | `EKTypes.h` | Safe adapter `Vec<EKEntityType>` on `EKCalendar::allowed_entity_types` (`src/calendar.rs`) |
| `EKEntityType` | typedef enum | `EKTypes.h` | `EKEntityType` and store/calendar/source APIs (`src/event_store.rs`, `src/calendar.rs`, `src/source.rs`) |
| `EKErrorCode` | typedef enum | `EKError.h` | Adapted via `EventKitError::Framework(NSErrorInfo { code, .. })` (`src/error.rs`, `swift-bridge/Sources/EventKitBridge/Core.swift`) |
| `EKErrorDomain` | extern const | `EKError.h` | Adapted via `EventKitError::Framework(NSErrorInfo { domain, .. })` (`src/error.rs`, `swift-bridge/Sources/EventKitBridge/Core.swift`) |
| `EKEvent` | interface | `EKEvent.h` | `EKEvent` + `EKEventStore::{event_with_identifier,events_matching,save_event,remove_event}` (`src/event.rs`, `src/event_store.rs`) |
| `EKEventAvailability` | typedef enum | `EKEvent.h` | `EKEventAvailability` (`src/event.rs`) |
| `EKEventStatus` | typedef enum | `EKEvent.h` | `EKEventStatus` (`src/event.rs`) |
| `EKEventStore` | interface | `EKEventStore.h` | `EKEventStore` (`src/event_store.rs`), bridged by `EventStore.swift` thunks |
| `EKEventStoreChangedNotification` | extern const | `EKEventStore.h` | `EK_EVENT_STORE_CHANGED_NOTIFICATION` (`src/event_store.rs`) |
| `EKObject` | interface | `EKObject.h` | `EKObject` live wrapper with `has_changes`, `is_new`, `reset`, `rollback`, and `refresh`, plus `as_object_in` helpers on `EKEvent`, `EKReminder`, and `EKCalendarDraft` (`src/object.rs`, `src/event.rs`, `src/reminder.rs`, `src/calendar.rs`), bridged by `ek_object_*` thunks (`swift-bridge/Sources/EventKitBridge/Object.swift`) |
| `EKParticipant` | interface | `EKParticipant.h` | `EKParticipant` snapshot embedded in events/reminders (`src/participant.rs`, `src/event.rs`, `src/reminder.rs`) |
| `EKParticipantRole` | typedef enum | `EKTypes.h` | `EKParticipantRole` (`src/participant.rs`) |
| `EKParticipantScheduleStatus` | typedef enum | `EKTypes.h` | `EKParticipantScheduleStatus` (`src/participant.rs`) |
| `EKParticipantStatus` | typedef enum | `EKTypes.h` | `EKParticipantStatus` (`src/participant.rs`) |
| `EKParticipantType` | typedef enum | `EKTypes.h` | `EKParticipantType` (`src/participant.rs`) |
| `EKRecurrenceDayOfWeek` | interface | `EKRecurrenceDayOfWeek.h` | `EKRecurrenceDayOfWeek` (`src/recurrence_rule.rs`) |
| `EKRecurrenceEnd` | interface | `EKRecurrenceEnd.h` | `EKRecurrenceEnd` (`src/recurrence_rule.rs`) |
| `EKRecurrenceFrequency` | typedef enum | `EKTypes.h` | `EKRecurrenceFrequency` (`src/recurrence_rule.rs`) |
| `EKRecurrenceRule` | interface | `EKRecurrenceRule.h` | `EKRecurrenceRule` (`src/recurrence_rule.rs`), bridged by `ek_recurrence_rule_roundtrip_json` |
| `EKReminder` | interface | `EKReminder.h` | `EKReminder` + `EKEventStore::{fetch_reminders_matching,save_reminder,remove_reminder}` (`src/reminder.rs`, `src/event_store.rs`) |
| `EKReminderPriority` | typedef enum | `EKTypes.h` | `EKReminderPriority` (`src/reminder.rs`) |
| `EKSource` | interface | `EKSource.h` | `EKSource` + `EKEventStore::{sources,delegate_sources,source_with_identifier}` (`src/source.rs`, `src/event_store.rs`) |
| `EKSourceType` | typedef enum | `EKTypes.h` | `EKSourceType` (`src/source.rs`) |
| `EKSpan` | typedef enum | `EKEventStore.h` | `EKSpan` (`src/event_store.rs`) |
| `EKStructuredLocation` | interface | `EKStructuredLocation.h` | `EKStructuredLocation` (`src/structured_location.rs`), bridged by `ek_structured_location_roundtrip_json` |
| `EKVirtualConferenceDescriptor` | interface | `EKVirtualConferenceDescriptor.h` | `EKVirtualConferenceDescriptor` (`src/virtual_conference_provider.rs`) |
| `EKVirtualConferenceRoomTypeDescriptor` | interface | `EKVirtualConferenceDescriptor.h` | `EKVirtualConferenceRoomTypeDescriptor` (`src/virtual_conference_provider.rs`) |
| `EKVirtualConferenceURLDescriptor` | interface | `EKVirtualConferenceDescriptor.h` | `EKVirtualConferenceURLDescriptor` (`src/virtual_conference_provider.rs`) |
| `EKWeekday` | typedef enum | `EKTypes.h` | `EKWeekday` (`src/recurrence_rule.rs`) |

## 🔴 GAPS
None.

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `EKAuthorizationStatusAuthorized` | enum constant | `EKTypes.h` | Deprecated alias of `EKAuthorizationStatusFullAccess`; audit instructions exempt deprecated symbols. | `NS_ENUM_DEPRECATED(10_0, 14_0, 6_0, 17_0, "Check for full access or write only access")` |
| `EKMonday` | enum constant | `EKTypes.h` | Deprecated alias of `EKWeekdayMonday`. | `NS_ENUM_DEPRECATED(10_8, 10_11, 4_0, 9_0, "Use EKWeekdayMonday instead")` |
| `EKSaturday` | enum constant | `EKTypes.h` | Deprecated alias of `EKWeekdaySaturday`. | `NS_ENUM_DEPRECATED(10_8, 10_11, 4_0, 9_0, "Use EKWeekdaySaturday instead")` |
| `EKSunday` | enum constant | `EKTypes.h` | Deprecated alias of `EKWeekdaySunday`. | `NS_ENUM_DEPRECATED(10_8, 10_11, 4_0, 9_0, "Use EKWeekdaySunday instead")` |
| `EKThursday` | enum constant | `EKTypes.h` | Deprecated alias of `EKWeekdayThursday`. | `NS_ENUM_DEPRECATED(10_8, 10_11, 4_0, 9_0, "Use EKWeekdayThursday instead")` |
| `EKTuesday` | enum constant | `EKTypes.h` | Deprecated alias of `EKWeekdayTuesday`. | `NS_ENUM_DEPRECATED(10_8, 10_11, 4_0, 9_0, "Use EKWeekdayTuesday instead")` |
| `EKVirtualConferenceProvider` | interface | `EKVirtualConferenceProvider.h` | Extension-only subclass hook; this crate intentionally exposes the descriptor value types but does not host EventKit app extensions. | `API_AVAILABLE(macos(12.0), ios(15.0))` |
| `EKWednesday` | enum constant | `EKTypes.h` | Deprecated alias of `EKWeekdayWednesday`. | `NS_ENUM_DEPRECATED(10_8, 10_11, 4_0, 9_0, "Use EKWeekdayWednesday instead")` |
| `EKFriday` | enum constant | `EKTypes.h` | Deprecated alias of `EKWeekdayFriday`. | `NS_ENUM_DEPRECATED(10_8, 10_11, 4_0, 9_0, "Use EKWeekdayFriday instead")` |
