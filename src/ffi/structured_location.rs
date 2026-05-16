#![allow(missing_docs)]

use core::ffi::c_char;

extern "C" {
    pub fn ek_structured_location_roundtrip_json(
        location_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
}
