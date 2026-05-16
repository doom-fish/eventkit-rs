#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn ek_store_calendars_json(
        store: *mut c_void,
        entity_type: i32,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ek_store_default_event_calendar_json(
        store: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ek_store_default_reminder_calendar_json(
        store: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ek_store_calendar_json(
        store: *mut c_void,
        identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ek_store_save_calendar_json(
        store: *mut c_void,
        calendar_json: *const c_char,
        commit: bool,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ek_store_remove_calendar(
        store: *mut c_void,
        identifier: *const c_char,
        commit: bool,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn ek_calendar_roundtrip_json(
        store: *mut c_void,
        calendar_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
}
