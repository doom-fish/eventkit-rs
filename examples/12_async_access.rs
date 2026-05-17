//! Example 12 — Async access-request and reminder fetch
//!
//! Demonstrates [`AsyncEventStore`] by:
//! 1. Creating an event store.
//! 2. Requesting full calendar-events access asynchronously.
//! 3. Requesting full reminders access asynchronously.
//! 4. Fetching all reminders asynchronously and printing a count.
//!
//! The example is **headless-safe**: it never blocks on a UI dialog and exits
//! gracefully when permission has not been granted.  Run it from a terminal on a
//! Mac where `EventKit` access has already been approved for the terminal process.

use eventkit::async_api::AsyncEventStore;
use eventkit::event_store::{EKEventStore, EKReminderPredicate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(async_main())
}

#[allow(clippy::future_not_send)]
async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    let store = EKEventStore::new()?;
    let async_store = AsyncEventStore::new(store);

    // ── Request full access to calendar events ─────────────────────────────
    let events_granted = async_store.request_full_access_to_events().await?;
    println!("request_full_access_to_events: granted={events_granted}");
    if !events_granted {
        println!("Calendar events access not granted — skipping event operations.");
    }

    // ── Request full access to reminders ──────────────────────────────────
    let reminders_granted = async_store.request_full_access_to_reminders().await?;
    println!("request_full_access_to_reminders: granted={reminders_granted}");
    if !reminders_granted {
        println!("Reminders access not granted — skipping reminder fetch.");
        return Ok(());
    }

    // ── Fetch all reminders ────────────────────────────────────────────────
    let predicate = EKReminderPredicate::new();
    let reminders = async_store.fetch_reminders(&predicate)?.await?;
    println!("fetch_reminders: count={}", reminders.len());
    for (i, r) in reminders.iter().take(5).enumerate() {
        println!("  [{i}] title={:?}", r.title);
    }
    if reminders.len() > 5 {
        println!("  … and {} more", reminders.len() - 5);
    }

    // ── Request write-only access to calendar events ───────────────────────
    let write_granted = async_store.request_write_only_access_to_events().await?;
    println!("request_write_only_access_to_events: granted={write_granted}");

    println!("async_api example completed successfully.");
    Ok(())
}
