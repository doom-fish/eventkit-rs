#![allow(missing_docs)]

use core::ffi::c_char;

extern "C" {
    pub fn ek_virtual_conference_room_type_roundtrip_json(
        descriptor_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ek_virtual_conference_url_roundtrip_json(
        descriptor_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ek_virtual_conference_descriptor_roundtrip_json(
        descriptor_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
}
