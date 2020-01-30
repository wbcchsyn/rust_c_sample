mod sqrt;
mod sqrt2;
mod strlen;

fn main() {
    let rust_str = "I'll be back";
    assert_eq!(strlen::c_string_length(rust_str.as_bytes()), 12);

    let val = 3.14;
    println!("Square root of 3.14 is {}", sqrt::c_sqrt(val));

    let val = 2.0;
    println!("Square root of 2.0 is {}", sqrt2::c_sqrt2(val));
}
