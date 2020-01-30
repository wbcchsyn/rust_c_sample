//! Call c function requiring library

use std::os::raw::c_double;

#[link(name = "m")]
extern "C" {
    fn sqrt(v: c_double) -> c_double;
}

pub fn c_sqrt(v: f64) -> f64 {
    unsafe {
        let ret = sqrt(v as c_double);
        ret as f64
    }
}
