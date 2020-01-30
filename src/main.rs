mod sqrt;
mod strlen;

fn main() {
    let rust_str = "I'll be back";
    assert_eq!(strlen::c_string_length(rust_str.as_bytes()), 12);

    let val = 3.14;
    println!("Square root of 3.14 is {}", sqrt::c_sqrt(val));
}
