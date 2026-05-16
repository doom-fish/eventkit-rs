# eventkit

Safe Rust bindings for Apple's [EventKit](https://developer.apple.com/documentation/eventkit) framework on macOS.

> **Status:** v0.2.0 covers one logical area each for `EKEventStore`, `EKEvent`, `EKReminder`, `EKCalendar`, `EKRecurrenceRule`, `EKAlarm`, `EKParticipant`, `EKSource`, `EKStructuredLocation`, and virtual conference descriptor APIs. Extension-only `EKVirtualConferenceProvider` request hooks are documented in [`COVERAGE.md`](COVERAGE.md).

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
- Rich `EKEvent` + `EKReminder` snapshots with alarms, recurrence rules, participants, organizers, structured locations, and date components.
- `EKCalendar` + `EKSource` snapshots, plus unsaved `EKCalendarDraft` round-trips for safe headless testing.
- `EKRecurrenceRule`, `EKAlarm`, `EKStructuredLocation`, and virtual conference descriptor round-trips.
- One example and one integration test per logical area.

## Coverage audit

`COVERAGE.md` tracks the v0.2.0 audit against the macOS 26.2 `EventKit.framework` headers and calls out the intentionally skipped APIs:

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
