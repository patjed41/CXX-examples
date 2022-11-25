#include <iostream>

#include "line.h"
#include "rust/src/lib.rs.h"
#include "rust/cxx.h"

int main() {
    Line line(2, -3);
    rust::Box<Point> point = new_point(1, -1);
    std::cout << "Is point (1, -1) on line y = 2x - 3? "
              << (line.contains_point(*point) ? "Yes.\n" : "No.\n");
    
    std::cout << "Is point (5, -2) a cross point of lines y = -3x + 13 and y = 2x - 12? "
              << (is_cross_point(-3, 13, 2, -12, 5, -2) ? "Yes.\n" : "No.\n");
}
