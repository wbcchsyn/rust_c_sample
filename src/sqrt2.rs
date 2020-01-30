//! Compile cpp program and call the function.

use std::os::raw::c_double;

#[link(name = "my_m")]
extern "C" {
    fn sqrt2(v: c_double) -> c_double;
}

pub fn c_sqrt2(v: f64) -> f64 {
    unsafe {
        let ret = sqrt2(v as c_double);
        ret as f64
    }
}
