# eventkit

Safe Rust bindings for Apple's [EventKit](https://developer.apple.com/documentation/eventkit) framework on macOS.

> **Status:** v0.2.1 completes the symbol-level audit at 100% coverage by adding `EKObject` state wrappers and `EKParticipantScheduleStatus` to the existing EventKit surfaces. Extension-only `EKVirtualConferenceProvider` request hooks remain documented in [`COVERAGE.md`](COVERAGE.md).

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
eventkit = { version = "0.3", features = ["async"] }
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



`COVERAGE.md` tracks the v0.2.1 audit against the macOS 26.2 `EventKit.framework` headers and calls out the intentionally skipped APIs:

- deprecated legacy initializers / AddressBook integrations,
- cross-framework convenience APIs that would force a `MapKit` dependency,
- extension-only `EKVirtualConferenceProvider` subclass hooks.

## Authorization

`EventKit.framework` access is gated by macOS privacy settings. The shipped examples and tests are intentionally headless-safe: they favor non-mutating lookups and JSON round-trips, and they tolerate zero visible calendars/sources.

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
