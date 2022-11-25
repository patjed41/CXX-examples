#include <cmath>

#include "line.h"
#include "rust/src/lib.rs.h"

const double EPSILON = 1e-5;

Line::Line(double a, double b) : a(a), b(b) {}

bool Line::contains_point(const Point &p) const {
    return fabs(p.y() - (a * p.x() + b)) < EPSILON;
}

std::unique_ptr<Line> new_line(double a, double b) {
    return std::make_unique<Line>(a, b);
}
