#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn ek_event_compare_start_date_json(
        lhs_json: *const c_char,
        rhs_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn ek_event_roundtrip_json(
        store: *mut c_void,
        event_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ek_store_refresh_event_json(
        store: *mut c_void,
        identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
}
