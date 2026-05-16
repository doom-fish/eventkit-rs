#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn ek_reminder_roundtrip_json(
        store: *mut c_void,
        reminder_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
}
