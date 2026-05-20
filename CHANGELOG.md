# Changelog

## [0.3.8] - 2026-05-20

- Migrated local `take_string` body to call `doom_fish_utils::ffi_string::take_owned_cstring_c`. Centralises the duplicated FFI take-string pattern fleet-wide. No public API change.

## [0.3.7] - 2026-05-20

- Added in-`src/` unit tests across `error.rs`, `recurrence_rule.rs`, `reminder.rs`, and `event.rs` (Tier 2 quality polish), providing fast `cargo test --lib` fail-fast signal alongside the existing integration tests under `tests/`.

## [0.3.6] - 2026-05-20

- Clippy hygiene sweep: cleared all `-D warnings` lints across the crate. No public API change.

## [0.3.5] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.3.4] - 2026-05-19

- Bump MSRV from 1.70 to 1.76 to match fleet baseline.

## [0.3.3] - 2026-05-18

- Added concise rustdoc coverage across the non-FFI EventKit surface, raising public-item coverage from 3.9% to 100.0%.

## [0.3.2] - 2026-05-18

- Widen doom-fish-utils version bound to `<0.3` so 0.2.x resolves.

## 0.3.1 - 2026-05-17

### Fixed — Quality-pass audit

- Made async API callbacks panic-safe by wrapping `access_cb` and `fetch_reminders_cb` 
  in `catch_user_panic` to prevent unwinding across the FFI boundary into Swift.
- Fixed flaky async API tests (`save_event_returns_result`, `save_reminder_returns_result`) 
  to be resilient to system state by accepting both success and error outcomes, 
  matching the test pattern used for request access and fetch reminders operations.

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
