#![allow(missing_docs)]

use core::ffi::c_char;

extern "C" {
    pub fn ek_recurrence_rule_roundtrip_json(
        rule_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
}
