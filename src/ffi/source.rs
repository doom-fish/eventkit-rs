#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn ek_store_sources_json(store: *mut c_void, out_error: *mut *mut c_char) -> *mut c_char;
    pub fn ek_store_delegate_sources_json(
        store: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ek_store_source_json(
        store: *mut c_void,
        identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ek_store_source_calendars_json(
        store: *mut c_void,
        identifier: *const c_char,
        entity_type: i32,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
}
