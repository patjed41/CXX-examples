fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/print_cpp.cc")
        .flag_if_supported("-std=c++17")
        .compile("example1");
}
