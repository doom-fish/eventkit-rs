#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn ek_object_from_event_json(
        store: *mut c_void,
        event_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn ek_object_from_reminder_json(
        store: *mut c_void,
        reminder_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn ek_object_from_calendar_draft_json(
        store: *mut c_void,
        calendar_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;

    pub fn ek_object_release(object: *mut c_void);
    pub fn ek_object_has_changes(object: *mut c_void) -> bool;
    pub fn ek_object_is_new(object: *mut c_void) -> bool;
    pub fn ek_object_reset(object: *mut c_void);
    pub fn ek_object_rollback(object: *mut c_void);
    pub fn ek_object_refresh(object: *mut c_void) -> bool;
}
