#![allow(missing_docs)]

use core::ffi::c_char;

extern "C" {
    pub fn ek_string_free(string: *mut c_char);
}
