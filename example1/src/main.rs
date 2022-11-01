#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Point;

        pub fn x(&self) -> f64;
        pub fn y(&self) -> f64;
    }

    unsafe extern "C++" {
        include!("example1/include/line.h");

        type Line;

        fn contains_point(&self, p: &Point) -> bool;
        fn new_line(a: f64, b: f64) -> UniquePtr<Line>;
    }
}

pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

fn main() {
    let line = ffi::new_line(2., -3.);
    let point = Point { x: 1., y: -1. };
    println!("Is point on line? {}", line.contains_point(&point));
}
