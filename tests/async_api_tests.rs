//! Integration tests for the `async_api` module.
//!
//! Tests run headless (no UI dialogs) and are designed to exercise the full
//! async plumbing without requiring `EventKit` permissions to be granted in the
//! test environment.  Each test verifies that the Future types resolve — the
//! actual `granted` value depends on the system's TCC state and is not
//! asserted.

#[cfg(feature = "async")]
mod async_tests {
    use eventkit::async_api::AsyncEventStore;
    use eventkit::event_store::{EKEventStore, EKReminderPredicate};

    // ── Helper ──────────────────────────────────────────────────────────────

    fn make_async_store() -> AsyncEventStore {
        AsyncEventStore::new(EKEventStore::new().expect("EKEventStore::new"))
    }

    // ── RequestAccessFuture — happy path (resolves without panic) ───────────

    #[test]
    fn request_full_access_events_resolves() {
        let store = make_async_store();
        let result = pollster::block_on(store.request_full_access_to_events());
        // We cannot assert the bool value in a headless CI context, but the
        // Future must resolve to Ok(_) or Err(_) without hanging or panicking.
        match result {
            Ok(granted) => println!("events access granted={granted}"),
            Err(e) => println!("events access error (expected in headless CI): {e}"),
        }
    }

    #[test]
    fn request_full_access_reminders_resolves() {
        let store = make_async_store();
        let result = pollster::block_on(store.request_full_access_to_reminders());
        match result {
            Ok(granted) => println!("reminders access granted={granted}"),
            Err(e) => println!("reminders access error (expected in headless CI): {e}"),
        }
    }

    #[test]
    fn request_write_only_access_events_resolves() {
        let store = make_async_store();
        let result = pollster::block_on(store.request_write_only_access_to_events());
        match result {
            Ok(granted) => println!("write-only events access granted={granted}"),
            Err(e) => println!("write-only events access error (expected in headless CI): {e}"),
        }
    }

    // ── FetchRemindersFuture — happy path ──────────────────────────────────

    #[test]
    fn fetch_reminders_resolves() {
        let store = make_async_store();
        let predicate = EKReminderPredicate::new();
        // fetch_reminders can fail if reminders access is not granted; that is OK.
        let future = store
            .fetch_reminders(&predicate)
            .expect("predicate JSON encoding should not fail");
        let result = pollster::block_on(future);
        match result {
            Ok(reminders) => println!("fetched {} reminder(s)", reminders.len()),
            Err(e) => println!("fetch reminders error (expected in headless CI): {e}"),
        }
    }

    // ── FetchRemindersFuture — error path (invalid predicate JSON) ─────────
    //
    // We cannot easily inject a bad predicate through the public API because
    // `json_cstring` only fails on NUL bytes, which serde_json never emits.
    // Instead, verify that an empty (all-reminders) predicate round-trips.

    #[test]
    fn fetch_reminders_predicate_encodes_cleanly() {
        let store = make_async_store();
        // `all` predicate with no calendar filter and no date range.
        let predicate = EKReminderPredicate::new();
        let future_result = store.fetch_reminders(&predicate);
        assert!(future_result.is_ok(), "predicate encoding must not fail");
    }

    // ── RequestAccessFuture — multiple sequential awaits ───────────────────

    #[test]
    fn multiple_access_requests_are_independent() {
        pollster::block_on(async {
            let store = make_async_store();
            let r1 = store.request_full_access_to_events().await;
            let r2 = store.request_full_access_to_reminders().await;
            let r3 = store.request_write_only_access_to_events().await;
            // All three must resolve (not hang) without shared state issues.
            println!("r1={r1:?} r2={r2:?} r3={r3:?}");
        });
    }

    // ── AsyncEventStore::save_event / remove_event (sync wrappers) ─────────
    //
    // We cannot create or save real events without permission, but we can
    // verify the `async fn` wrappers compile and return the right result
    // by attempting with a minimal event (which will likely fail due to missing
    // calendar/access, but may succeed depending on system state).

    #[test]
    fn save_event_returns_result() {
        use eventkit::event::EKEvent;
        use eventkit::event_store::EKSpan;

        pollster::block_on(async {
            let store = make_async_store();
            // A minimal EKEvent with no calendar will likely fail to save.
            let event = EKEvent::new("test", "2025-01-01T00:00:00Z", "2025-01-01T01:00:00Z");
            let result = store.save_event(&event, EKSpan::ThisEvent, true).await;
            // The result depends on system state and permissions; just verify it returns.
            match result {
                Ok(()) => println!("event saved (unexpected but OK)"),
                Err(e) => println!("event save failed (expected in headless CI): {e}"),
            }
        });
    }

    #[test]
    fn save_reminder_returns_result() {
        use eventkit::reminder::EKReminder;

        pollster::block_on(async {
            let store = make_async_store();
            let reminder = EKReminder::new("test reminder");
            let result = store.save_reminder(&reminder, true).await;
            // The result depends on system state and permissions; just verify it returns.
            match result {
                Ok(()) => println!("reminder saved (unexpected but OK)"),
                Err(e) => println!("reminder save failed (expected in headless CI): {e}"),
            }
        });
    }
}
