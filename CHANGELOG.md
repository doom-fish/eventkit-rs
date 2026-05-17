# Changelog

## 0.3.0 - 2026-05-16

### Added — Async API (Tier 1, `feature = "async"`)

New module `async_api` gated behind the `async` Cargo feature, providing
`Future`-based wrappers for EventKit's completion-handler and synchronous APIs.

#### Completion-handler APIs (new `@_cdecl` Swift thunks + Rust Future newtypes)

| Rust type | Wraps |
|-----------|-------|
| `RequestAccessFuture` | `EKEventStore.requestFullAccessToEvents(completion:)` |
| `RequestAccessFuture` | `EKEventStore.requestFullAccessToReminders(completion:)` |
| `RequestAccessFuture` | `EKEventStore.requestWriteOnlyAccessToEvents(completion:)` |
| `FetchRemindersFuture` | `EKEventStore.fetchReminders(matching:completion:)` |

#### Synchronous save/remove — thin `Future` wrappers

`AsyncEventStore::save_event`, `remove_event`, `save_reminder`, `remove_reminder`
delegate to the existing blocking implementations via `std::future::ready(…)`,
making them composable in async code.

#### Supporting additions

- `AsyncEventStore` facade wraps `EKEventStore` and exposes all async methods.
- `doom-fish-utils` optional dependency (pulled in by `features = ["async"]`).
- `pollster` dev-dependency for running async examples/tests synchronously.
- Swift bridge: `swift-bridge/Sources/EventKitBridge/Async.swift` with four
  `@_cdecl` thunks.
- Example: `examples/12_async_access.rs`.
- Tests: `tests/async_api_tests.rs` (8 headless-safe tests).

## 0.2.1 - 2026-05-16

- Added a live `EKObject` wrapper for `has_changes`, `is_new`, `reset`, `rollback`, and `refresh`, plus `as_object_in` helpers on `EKEvent`, `EKReminder`, and `EKCalendarDraft`.
- Added `EKParticipantScheduleStatus` to the safe Rust surface and expanded the participant smoke coverage.
- Closed the remaining `COVERAGE_AUDIT.md` gaps and brought the symbol-level audit to 100%.

## 0.2.0 - 2026-05-16

- Expanded the crate to cover ten logical EventKit areas: EventStore, Event, Reminder, Calendar, RecurrenceRule, Alarm, Participant, Source, StructuredLocation, and virtual conference descriptors.
- Split the Swift bridge into per-area files and reorganized the Rust surface into per-area modules plus compatibility re-exports.
- Added source-aware store APIs, richer calendar/event/reminder snapshots, structured locations, participant/source snapshots, and virtual conference descriptor round-trips.
- Added one example and one integration test per logical area, all designed to stay headless-safe on macOS.
- Added `COVERAGE.md`, auditing the macOS 26.2 `EventKit.framework` headers and documenting skipped deprecated or extension-only APIs.

## 0.1.0 - 2026-05-16

- Initial release.
- Added safe Rust bindings for `EKEventStore`, `EKEvent`, `EKReminder`, `EKCalendar`, `EKRecurrenceRule`, and `EKAlarm`.
- Added synchronous Rust wrappers for EventKit's predicate-based event and reminder fetch APIs.
- Added a non-interactive smoke example that reports authorization status and lists visible calendars.
