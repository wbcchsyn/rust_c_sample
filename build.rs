extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++17")
        .file("src/sqrt2.cpp")
        .compile("my_m")
}
