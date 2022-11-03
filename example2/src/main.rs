#[cxx::bridge]
mod ffi {
    struct Point {
        x: f64,
        y: f64,
    }

    unsafe extern "C++" {
        include!("example2/include/line.h");

        type Line;

        fn contains_point(&self, p: &Point) -> bool;
        fn new_line(a: f64, b: f64) -> UniquePtr<Line>;
    }
}

fn main() {
    let line = ffi::new_line(2., -3.);
    let point = ffi::Point { x: 1., y: -1. };
    println!("Is point on line? {}", line.contains_point(&point));
}
