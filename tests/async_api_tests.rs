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
    use eventkit::error::EKAuthorizationStatus;
    use eventkit::event_store::{EKEntityType, EKEventStore, EKReminderPredicate};

    // ── Helper ──────────────────────────────────────────────────────────────

    fn make_async_store() -> AsyncEventStore {
        AsyncEventStore::new(EKEventStore::new().expect("EKEventStore::new"))
    }

    fn assert_granted_when_authorized(
        entity_type: EKEntityType,
        result: &Result<bool, eventkit::error::EventKitError>,
    ) {
        if EKEventStore::authorization_status(entity_type) == EKAuthorizationStatus::FullAccess {
            assert_eq!(result, &Ok(true));
        }
    }

    fn access_prompt_possible(entity_type: EKEntityType) -> bool {
        let prompt = EKEventStore::authorization_status(entity_type)
            == EKAuthorizationStatus::NotDetermined;
        if prompt {
            eprintln!("skipped: requesting {entity_type:?} access would show a permission prompt");
        }
        prompt
    }

    // ── RequestAccessFuture — happy path (resolves without panic) ───────────

    #[test]
    fn request_full_access_events_resolves() {
        if access_prompt_possible(EKEntityType::Event) {
            return;
        }
        let store = make_async_store();
        let result = pollster::block_on(store.request_full_access_to_events());
        assert_granted_when_authorized(EKEntityType::Event, &result);
        match result {
            Ok(granted) => println!("events access granted={granted}"),
            Err(e) => println!("events access error (expected in headless CI): {e}"),
        }
    }

    #[test]
    fn request_full_access_reminders_resolves() {
        if access_prompt_possible(EKEntityType::Reminder) {
            return;
        }
        let store = make_async_store();
        let result = pollster::block_on(store.request_full_access_to_reminders());
        assert_granted_when_authorized(EKEntityType::Reminder, &result);
        match result {
            Ok(granted) => println!("reminders access granted={granted}"),
            Err(e) => println!("reminders access error (expected in headless CI): {e}"),
        }
    }

    #[test]
    fn request_write_only_access_events_resolves() {
        if access_prompt_possible(EKEntityType::Event) {
            return;
        }
        let store = make_async_store();
        let result = pollster::block_on(store.request_write_only_access_to_events());
        assert_granted_when_authorized(EKEntityType::Event, &result);
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
    fn fetch_reminders_rejects_unknown_calendars() {
        let store = make_async_store();
        let predicate = EKReminderPredicate::new()
            .with_calendar_identifiers(["doom-fish.eventkit-tests.missing-calendar".to_owned()]);
        let result = pollster::block_on(store.fetch_reminders(&predicate).expect("predicate"));
        assert!(
            matches!(result, Err(eventkit::error::EventKitError::InvalidArgument(_))),
            "{result:?}"
        );
    }

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
        if access_prompt_possible(EKEntityType::Event)
            || access_prompt_possible(EKEntityType::Reminder)
        {
            return;
        }
        pollster::block_on(async {
            let store = make_async_store();
            let r1 = store.request_full_access_to_events().await;
            let r2 = store.request_full_access_to_reminders().await;
            let r3 = store.request_write_only_access_to_events().await;
            assert_granted_when_authorized(EKEntityType::Event, &r1);
            assert_granted_when_authorized(EKEntityType::Reminder, &r2);
            assert_granted_when_authorized(EKEntityType::Event, &r3);
        });
    }

    // ── AsyncEventStore::save_event / remove_event (sync wrappers) ─────────

    #[test]
    fn save_event_returns_result() {
        use eventkit::event::EKEvent;
        use eventkit::event_store::EKSpan;

        pollster::block_on(async {
            let store = make_async_store();
            let event = EKEvent::new("test", "2025-01-01T00:00:00Z", "2025-01-01T01:00:00Z")
                .with_calendar_identifier("doom-fish.eventkit-tests.missing-calendar");
            let result = store.save_event(&event, EKSpan::ThisEvent, true).await;
            assert!(result.is_err(), "an unknown calendar must be rejected before saving");
        });
    }

    #[test]
    fn save_reminder_returns_result() {
        use eventkit::reminder::EKReminder;

        pollster::block_on(async {
            let store = make_async_store();
            let mut reminder = EKReminder::new("test reminder");
            reminder.calendar_identifier =
                Some("doom-fish.eventkit-tests.missing-calendar".to_owned());
            let result = store.save_reminder(&reminder, true).await;
            assert!(result.is_err(), "an unknown calendar must be rejected before saving");
        });
    }
}
