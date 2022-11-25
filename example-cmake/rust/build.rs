fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("../cpp/line.cc")
        .flag_if_supported("-std=c++20")
        .compile("rust_line");
}