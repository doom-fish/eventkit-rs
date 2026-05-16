# EventKit.framework coverage audit

This document audits `eventkit` v0.2.0 against the public EventKit headers shipped in the macOS 26.2 SDK:

- `EKAlarm.h`
- `EKCalendar.h`
- `EKCalendarItem.h`
- `EKEvent.h`
- `EKEventStore.h`
- `EKParticipant.h`
- `EKRecurrenceRule.h`
- `EKReminder.h`
- `EKSource.h`
- `EKStructuredLocation.h`
- `EKTypes.h`
- `EKVirtualConferenceDescriptor.h`
- `EKVirtualConferenceProvider.h`

Legend:

- ✅ implemented in the safe Rust surface
- 🟡 implemented with an intentionally adapted safe representation
- ⏭️ intentionally skipped

## EventStore

| Header API | Status | Rust surface / notes |
| --- | --- | --- |
| `+[EKEventStore authorizationStatusForEntityType:]` | ✅ | `EKEventStore::authorization_status` |
| `-init` | ✅ | `EKEventStore::new` |
| `-initWithSources:` | ✅ | `EKEventStore::with_source_identifiers` |
| `-initWithAccessToEntityTypes:` | ⏭️ | Deprecated initializer; the crate targets the modern full-access/write-only access APIs instead. |
| `-requestFullAccessToEventsWithCompletion:` | ✅ | `EKEventStore::request_full_access_to_events` |
| `-requestWriteOnlyAccessToEventsWithCompletion:` | ✅ | `EKEventStore::request_write_only_access_to_events` |
| `-requestFullAccessToRemindersWithCompletion:` | ✅ | `EKEventStore::request_full_access_to_reminders` |
| `-requestAccessToEntityType:completion:` | ✅ | `request_access_to_events` / `request_access_to_reminders` compatibility helpers call the modern APIs on macOS 14+ and the deprecated API on older releases. |
| `eventStoreIdentifier` | ✅ | `EKEventStore::event_store_identifier` |
| `sources` / `delegateSources` / `sourceWithIdentifier:` | ✅ | `sources`, `delegate_sources`, `source_with_identifier` |
| `calendars` | ⏭️ | Deprecated in the headers; use entity-specific calendar lookup. |
| `calendarsForEntityType:` | ✅ | `calendars_for_entity_type` |
| `defaultCalendarForNewEvents` / `defaultCalendarForNewReminders` | ✅ | `default_calendar_for_new_events`, `default_calendar_for_new_reminders` |
| `calendarWithIdentifier:` | ✅ | `calendar_with_identifier` |
| `saveCalendar:commit:error:` / `removeCalendar:commit:error:` | ✅ | `save_calendar`, `remove_calendar`, `remove_calendar_by_identifier` |
| `calendarItemWithIdentifier:` / `calendarItemsWithExternalIdentifier:` | ✅ | `calendar_item_with_identifier`, `calendar_items_with_external_identifier` |
| `saveEvent:span:commit:error:` / `removeEvent:span:commit:error:` | ✅ | `save_event`, `remove_event` |
| `eventWithIdentifier:` / `eventsMatchingPredicate:` | ✅ | `event_with_identifier`, `events_matching` |
| `enumerateEventsMatchingPredicate:usingBlock:` | ✅ | `enumerate_events_matching` wraps `events_matching` with a Rust callback. |
| `predicateForEventsWithStartDate:endDate:calendars:` | ✅ | `predicate_for_events` |
| `saveReminder:commit:error:` / `removeReminder:commit:error:` | ✅ | `save_reminder`, `remove_reminder` |
| `fetchRemindersMatchingPredicate:completion:` | ✅ | `fetch_reminders_matching` exposes a synchronous safe wrapper. |
| `cancelFetchRequest:` | ✅ | Used internally by the synchronous wrapper when its timeout/cancel path is triggered. |
| `predicateForRemindersInCalendars:` / `predicateForIncompleteRemindersWithDueDateStarting:ending:calendars:` / `predicateForCompletedRemindersWithCompletionDateStarting:ending:calendars:` | ✅ | `predicate_for_reminders`, `predicate_for_incomplete_reminders`, `predicate_for_completed_reminders` |
| `commit:` / `reset` / `refreshSourcesIfNecessary` | ✅ | `commit`, `reset`, `refresh_sources_if_necessary` |
| `EKEventStoreChangedNotification` | ✅ | `EK_EVENT_STORE_CHANGED_NOTIFICATION` |

## Calendar and Source

| Header API | Status | Rust surface / notes |
| --- | --- | --- |
| `+[EKCalendar calendarForEntityType:eventStore:]` | ✅ | `EKCalendarDraft` and store save flows cover calendar creation. |
| `+[EKCalendar calendarWithEventStore:]` | ⏭️ | Deprecated convenience constructor. |
| `calendarIdentifier` / `title` / `type` / `source` | ✅ | Snapshot fields on `EKCalendar`; drafts use `source_identifier`. |
| `allowsContentModifications` / `isSubscribed` / `isImmutable` | ✅ | Snapshot fields on `EKCalendar` |
| `CGColor` / `color` | 🟡 | Surfaced as an RGBA hex string for stable Rust serialization. |
| `supportedEventAvailabilities` / `allowedEntityTypes` | 🟡 | Surfaced as `Vec<EKCalendarEventAvailability>` and `Vec<EKEntityType>` instead of raw bitmasks. |
| `EKSource.sourceIdentifier` / `sourceType` / `title` / `isDelegate` | ✅ | Snapshot fields on `EKSource` |
| `EKSource.calendarsForEntityType:` | ✅ | `EKSource::calendars_for_entity_type` |
| `EKSource.calendars` | ⏭️ | Deprecated in the headers; use the entity-specific variant. |

## Event, calendar items, and participants

| Header API | Status | Rust surface / notes |
| --- | --- | --- |
| `EKCalendarItem` common properties (`calendar`, identifiers, title, location, notes, URL, dates, time zone)` | ✅ | Shared snapshot fields on `EKEvent`, `EKReminder`, and `EKCalendarItem`. |
| `EKCalendarItem` alarms / attendees / recurrence rules / `has*` flags | ✅ | Safe snapshot fields plus round-trip helpers. |
| `EKCalendarItem` mutation helpers (`addAlarm:`, `removeAlarm:`, `addRecurrenceRule:`, `removeRecurrenceRule:`) | ✅ | Expressed through safe Rust snapshots that are saved back through `EKEventStore`. |
| `EKCalendarItem.UUID` | ⏭️ | Deprecated in the headers. |
| `+[EKEvent eventWithEventStore:]` | ✅ | Covered by `EKEvent::roundtrip_in` and `EKEventStore::save_event`. |
| `eventIdentifier` / `allDay` / `startDate` / `endDate` | ✅ | Snapshot fields on `EKEvent` |
| `organizer` / `structuredLocation` / `availability` / `status` / `isDetached` / `occurrenceDate` | ✅ | Snapshot fields on `EKEvent` |
| `birthdayContactIdentifier` / `birthdayPersonUniqueID` | ✅ | Snapshot fields on `EKEvent` |
| `birthdayPersonID` | 🟡 | Bridged via KVC because modern Swift marks it unavailable, but the Objective-C property still exists. |
| `-refresh` / `-compareStartDateWithEvent:` | ✅ | `refresh_in`, `compare_start_date` |
| `EKParticipant` URL, name, status, role, type, current-user flag | ✅ | Snapshot fields on `EKParticipant` |
| `EKParticipant.contactPredicate` | 🟡 | Surfaced as an optional predicate-format string instead of an opaque `NSPredicate`. |
| `EKParticipant` legacy AddressBook accessors | ⏭️ | Deprecated/legacy AddressBook APIs are intentionally not exposed. |

## Reminder and date components

| Header API | Status | Rust surface / notes |
| --- | --- | --- |
| `+[EKReminder reminderWithEventStore:]` | ✅ | Covered by `EKReminder::roundtrip_in` and `EKEventStore::save_reminder`. |
| `startDateComponents` / `dueDateComponents` | ✅ | `NSDateComponents` snapshot type with calendar/time-zone support. |
| `completed` / `completionDate` / `priority` | ✅ | Snapshot fields on `EKReminder`; `priority` uses `EKReminderPriority`. |

## Recurrence rules

| Header API | Status | Rust surface / notes |
| --- | --- | --- |
| `EKRecurrenceFrequency` / `EKWeekday` enums | ✅ | `EKRecurrenceFrequency`, `EKWeekday` |
| `EKRecurrenceDayOfWeek` init and weekday/week-number accessors | ✅ | `EKRecurrenceDayOfWeek::new` and snapshot fields |
| `EKRecurrenceEnd` end-date / occurrence-count factories | ✅ | `EKRecurrenceEnd::from_end_date`, `from_occurrence_count` |
| `EKRecurrenceRule` designated/simple initializers | ✅ | `EKRecurrenceRule::new`, `with_components`, plus round-trip helpers |
| `calendarIdentifier` / `recurrenceEnd` / `frequency` / `interval` / `firstDayOfTheWeek` | ✅ | Snapshot fields on `EKRecurrenceRule` |
| `daysOfTheWeek` / `daysOfTheMonth` / `daysOfTheYear` / `weeksOfTheYear` / `monthsOfTheYear` / `setPositions` | ✅ | Snapshot fields on `EKRecurrenceRule` |

## Alarm and structured location

| Header API | Status | Rust surface / notes |
| --- | --- | --- |
| `+[EKAlarm alarmWithAbsoluteDate:]` / `+[EKAlarm alarmWithRelativeOffset:]` | ✅ | `EKAlarm` round-trip helpers support both forms. |
| `relativeOffset` / `absoluteDate` / `structuredLocation` / `proximity` / `type` / `emailAddress` / `soundName` | ✅ | Snapshot fields on `EKAlarm` |
| `url` | 🟡 | Bridged through KVC because modern Swift marks the procedure-alarm property unavailable even though the Obj-C API still exists. |
| `+[EKStructuredLocation locationWithTitle:]` | ✅ | `EKStructuredLocation` round-trip helpers |
| `title` / `geoLocation` / `radius` | ✅ | Snapshot fields on `EKStructuredLocation` and `EKGeoLocation` |
| `+[EKStructuredLocation locationWithMapItem:]` | ⏭️ | Intentionally skipped to avoid forcing a `MapKit` dependency into the crate. |

## Virtual conference descriptors

| Header API | Status | Rust surface / notes |
| --- | --- | --- |
| `EKVirtualConferenceRoomTypeDescriptor` init/title/identifier | ✅ | `EKVirtualConferenceRoomTypeDescriptor` |
| `EKVirtualConferenceURLDescriptor` init/title/url | ✅ | `EKVirtualConferenceURLDescriptor` |
| `EKVirtualConferenceDescriptor` init/title/URL descriptors/conference details | ✅ | `EKVirtualConferenceDescriptor` |
| `EKVirtualConferenceProvider` fetch hooks | ⏭️ | These are app-extension subclass hooks, not normal runtime APIs. The crate exposes `EK_VIRTUAL_CONFERENCE_PROVIDER_IS_EXTENSION_ONLY` and the descriptor types used by provider implementations. |

## Notes

- The public Rust surface intentionally favors stable, serializable snapshot types over direct exposure of Objective-C reference semantics.
- Deprecated APIs that only duplicate a modern equivalent are skipped unless they materially improve compatibility.
- The integration tests and examples validate one headless-safe path per logical area; mutating EventKit objects still depends on the caller's entitlements and privacy permissions.
