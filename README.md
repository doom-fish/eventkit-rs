# eventkit

Safe Rust bindings for Apple's [EventKit](https://developer.apple.com/documentation/eventkit) framework on macOS.

> **Status:** v0.1.0 covers the practical calendar + reminders surface for `EKEventStore`, `EKEvent`, `EKReminder`, `EKCalendar`, `EKRecurrenceRule`, `EKAlarm`, predicate helpers, save/remove flows, and batched commits.

## Quick start

```rust,no_run
use eventkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = EKEventStore::new()?;
    let status = EKEventStore::authorization_status(EKEntityType::Event);
    println!("event access: {status:?}");

    let calendars = store.calendars_for_entity_type(EKEntityType::Event)?;
    println!("event calendars: {}", calendars.len());

    let predicate = store.predicate_for_events(
        "2026-01-01T00:00:00Z",
        "2026-01-31T23:59:59Z",
        Some(&calendars),
    );
    let events = store.events_matching(&predicate)?;
    println!("january events: {}", events.len());
    Ok(())
}
```

## Highlights

- `EKEventStore::authorization_status`, `request_access_to_events`, `request_access_to_reminders`
- Calendar listing with `EKCalendar` title, type, allowed entity types, and color snapshots
- Event queries via `predicate_for_events` + `events_matching`
- Reminder queries via `predicate_for_reminders` + synchronous `fetch_reminders_matching`
- `EKEvent` and `EKReminder` save / remove helpers with batched `commit`
- `EKAlarm` and `EKRecurrenceRule` snapshots that round-trip through save flows

## Authorization

`EventKit.framework` access is gated by macOS privacy settings. The smoke example never prompts; it only reports current authorization and lists already-visible calendars.

## Smoke example

Run the framework smoke test with:

```bash
cargo run --all-features --example 01_eventkit_smoke
```

Expected success footer:

```text
✅ eventkit OK
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
