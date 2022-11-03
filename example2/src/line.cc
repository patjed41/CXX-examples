#include "example2/include/line.h"
#include "example2/src/main.rs.h"
#include <cmath>

const double EPSILON = 1e-5;

Line::Line(double a, double b) : a(a), b(b) {}

bool Line::contains_point(const Point &p) const {
    return fabs(p.y - (a * p.x + b)) < EPSILON;
}

std::unique_ptr<Line> new_line(double a, double b) {
    return std::make_unique<Line>(a, b);
}
