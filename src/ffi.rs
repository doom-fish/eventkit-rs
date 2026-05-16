#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn ek_string_free(string: *mut c_char);

    pub fn ek_authorization_status(entity_type: i32) -> i32;

    pub fn ek_store_new() -> *mut c_void;
    pub fn ek_store_release(store: *mut c_void);
    pub fn ek_store_request_access_events(store: *mut c_void, out_error: *mut *mut c_char) -> bool;
    pub fn ek_store_request_access_reminders(
        store: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> bool;
    pub fn ek_store_calendars_json(
        store: *mut c_void,
        entity_type: i32,
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
}

pub mod status {
    pub const OK: i32 = 0;
}
