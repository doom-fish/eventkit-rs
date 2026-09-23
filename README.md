# eventkit

Safe Rust bindings for Apple's [EventKit](https://developer.apple.com/documentation/eventkit) framework on macOS.

See [`COVERAGE.md`](COVERAGE.md) for the API matrix, the adapted representations and the intentionally skipped APIs.

## Requirements

- macOS 13 or later; the Swift bridge's deployment target is macOS 13.
- The full-access and write-only access requests need macOS 14. On macOS 13 they fall back to `requestAccessToEntityType:completion:`.

## Installation

```toml
[dependencies]
eventkit = "0.4"
```

## Quick start

```rust,no_run
use eventkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = EKEventStore::new()?;
    println!("store id: {}", store.event_store_identifier()?);
    println!("event access: {:?}", EKEventStore::authorization_status(EKEntityType::Event));

    let sources = store.sources()?;
    let calendars = store.calendars_for_entity_type(EKEntityType::Event)?;
    println!("sources: {}", sources.len());
    println!("event calendars: {}", calendars.len());
    Ok(())
}
```

## Highlights

- `EKEventStore` wrappers for authorization, source-scoped stores, source/calendar lookup, event/reminder predicates, save/remove flows, commit/reset, and source refresh.
- Live `EKObject` wrappers for `has_changes`, `is_new`, `reset`, `rollback`, and `refresh`, plus `as_object_in` helpers on `EKEvent`, `EKReminder`, and `EKCalendarDraft`.
- Rich `EKEvent` + `EKReminder` snapshots with alarms, recurrence rules, participants, organizers, structured locations, date components, and `EKParticipantScheduleStatus`.
- `EKCalendar` + `EKSource` snapshots, plus unsaved `EKCalendarDraft` round-trips for safe headless testing.
- `EKRecurrenceRule`, `EKAlarm`, `EKStructuredLocation`, and virtual conference descriptor round-trips.
- One example and one integration test per logical area.

## Async API

Enable the `async` Cargo feature for `Future`-based wrappers around EventKit's
completion-handler APIs:

```toml
[dependencies]
eventkit = { version = "0.4", features = ["async"] }
```

```rust,no_run
use eventkit::async_api::AsyncEventStore;
use eventkit::event_store::{EKEventStore, EKReminderPredicate};

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    let store = AsyncEventStore::new(EKEventStore::new()?);

    let granted = store.request_full_access_to_reminders().await?;
    if granted {
        let reminders = store.fetch_reminders(&EKReminderPredicate::new())?.await?;
        println!("found {} reminder(s)", reminders.len());
    }
    Ok(())
}
```

The async API is **executor-agnostic** — it works with tokio, async-std, smol,
`pollster`, or any other runtime. See [`async_api`] in the crate docs and
`examples/12_async_access.rs` for a runnable example.

> **Tier-2 note:** `EKEventStore` change notifications (multi-fire stream) are
> not yet wrapped; they will appear in a future `Stream`-based Tier-2 release.



`COVERAGE.md` was written against the macOS 26.2 `EventKit.framework` headers, re-checked against the 26.5 and 27.0 SDKs, and calls out the intentionally skipped APIs:

- deprecated legacy initializers / AddressBook integrations,
- cross-framework convenience APIs that would force a `MapKit` dependency,
- extension-only `EKVirtualConferenceProvider` subclass hooks.

## Authorization

`EventKit.framework` access is gated by macOS privacy settings (TCC).

- Add usage strings to the app's `Info.plist`: `NSCalendarsFullAccessUsageDescription` and `NSRemindersFullAccessUsageDescription` for full access, `NSCalendarsWriteOnlyAccessUsageDescription` for write-only event access, and on macOS 13 `NSCalendarsUsageDescription` and `NSRemindersUsageDescription`. Without them the request is denied.
- Sandboxed and hardened-runtime apps need the `com.apple.security.personal-information.calendars` entitlement.
- Command-line tools get the permission of their responsible process, such as the terminal app.
- The synchronous `request_*_access_*` methods wait up to 30 seconds for an answer and then return `EventKitError::TimedOut` while the prompt may still be showing. `AsyncEventStore` waits for the answer without a timeout.

## Behavior notes

- For a recurring event, `save_event` and `remove_event` act on the occurrence named by the snapshot's `occurrence_date`, with the `EKSpan` you pass, and return an error when that occurrence can't be found. `event_with_identifier` returns the first occurrence, as EventKit does.
- Saving an event or reminder writes only the fields that differ from the stored item. Unchanged alarms and recurrence rules are left as they are.
- Recurrence rules are checked before they reach EventKit: the interval and occurrence count must be positive, week numbers must be between -53 and 53 (0 for weekly rules), and the other values must be in their documented ranges. An invalid rule returns `EventKitError::InvalidArgument` instead of raising an Objective-C exception.
- `events_matching` handles ranges longer than EventKit's four-year limit by querying them in chunks. Unknown calendar identifiers return `EventKitError::InvalidArgument`, and an empty calendar list matches nothing.
- `enumerate_events_matching` fetches all matching events before it calls the closure.
- The snapshot types' `Debug` output redacts personal data such as titles, notes, locations, names and email addresses. Serialize them with serde when you need the values.

## Tests

The examples and tests use the real EventKit and tolerate zero visible calendars and sources. When calendar access is granted they read your calendars and reminders, but they never save or remove items. The access-request tests skip when the authorization status is not determined, so they never show a prompt.

## Examples

Run the store smoke example with:

```bash
cargo run --example 01_event_store_smoke
```

Run the full example suite with:

```bash
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
