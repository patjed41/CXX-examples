#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn print_rust(message: &str);
    }

    unsafe extern "C++" {
        include!("example-str/include/print_cpp.h");
        fn print_cpp(message: &str);
    }
}

fn print_rust(message: &str) {
    println!("{}", message);
}

fn main() {
    ffi::print_cpp("hello from Rust");
}
