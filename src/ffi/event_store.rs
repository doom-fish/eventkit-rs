#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn ek_authorization_status(entity_type: i32) -> i32;

    pub fn ek_store_new() -> *mut c_void;
    pub fn ek_store_new_with_sources_json(
        source_identifiers_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn ek_store_release(store: *mut c_void);

    pub fn ek_store_request_access_events(store: *mut c_void, out_error: *mut *mut c_char) -> bool;
    pub fn ek_store_request_full_access_events(
        store: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> bool;
    pub fn ek_store_request_write_only_access_events(
        store: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> bool;
    pub fn ek_store_request_access_reminders(
        store: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> bool;
    pub fn ek_store_request_full_access_reminders(
        store: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> bool;

    pub fn ek_store_identifier(store: *mut c_void) -> *mut c_char;

    pub fn ek_store_calendar_item_json(
        store: *mut c_void,
        identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ek_store_calendar_items_external_json(
        store: *mut c_void,
        external_identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;

    pub fn ek_store_event_json(
        store: *mut c_void,
        identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ek_store_events_matching_json(
        store: *mut c_void,
        predicate_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ek_store_fetch_reminders_json(
        store: *mut c_void,
        predicate_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ek_store_save_event(
        store: *mut c_void,
        event_json: *const c_char,
        span: i32,
        commit: bool,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn ek_store_remove_event(
        store: *mut c_void,
        event_json: *const c_char,
        span: i32,
        commit: bool,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn ek_store_save_reminder(
        store: *mut c_void,
        reminder_json: *const c_char,
        commit: bool,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn ek_store_remove_reminder(
        store: *mut c_void,
        reminder_json: *const c_char,
        commit: bool,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn ek_store_commit(store: *mut c_void, out_error: *mut *mut c_char) -> i32;
    pub fn ek_store_reset(store: *mut c_void);
    pub fn ek_store_refresh_sources_if_necessary(store: *mut c_void);
}

#[cfg(feature = "async")]
extern "C" {
    /// Async: request full calendar-events access.
    pub fn ek_store_request_full_access_events_async(
        store: *mut c_void,
        cb: extern "C" fn(*const c_void, *const i8, *mut c_void),
        ctx: *mut c_void,
    );

    /// Async: request full reminders access.
    pub fn ek_store_request_full_access_reminders_async(
        store: *mut c_void,
        cb: extern "C" fn(*const c_void, *const i8, *mut c_void),
        ctx: *mut c_void,
    );

    /// Async: request write-only calendar-events access.
    pub fn ek_store_request_write_only_access_events_async(
        store: *mut c_void,
        cb: extern "C" fn(*const c_void, *const i8, *mut c_void),
        ctx: *mut c_void,
    );

    /// Async: fetch reminders matching a JSON-encoded predicate.
    ///
    /// On success the callback receives a `strdup`-allocated JSON C string
    /// as `result`; the Rust side must free it via `ek_string_free`.
    pub fn ek_store_fetch_reminders_async(
        store: *mut c_void,
        predicate_json: *const c_char,
        cb: extern "C" fn(*const c_void, *const i8, *mut c_void),
        ctx: *mut c_void,
    );
}
