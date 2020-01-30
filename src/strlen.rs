//! Call c basic function

use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

pub fn c_string_length(s: &[u8]) -> usize {
    let null_terminated = CString::new(s).unwrap();
    unsafe { strlen(null_terminated.as_ptr()) }
}
