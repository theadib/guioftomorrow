#include "synctime.h"

#include <cmath>

namespace clockapp::synctime {

namespace {
constexpr double kPi = 3.14159265358979323846;
}

double fraction_of_period(double seconds_since_epoch, double period_seconds) {
    double remainder = std::fmod(seconds_since_epoch, period_seconds);
    if (remainder < 0.0) remainder += period_seconds;  // fmod keeps input's sign
    return remainder / period_seconds;
}

double angle_for_fraction(double fraction) {
    return fraction * 2.0 * kPi;
}

Point point_on_circle(Point center, double radius, double angle_radians) {
    // Angle 0 points up (12 o'clock); increasing angle sweeps clockwise,
    // which on-screen (y grows downward) means +sin(x), -cos(y).
    return Point{center.x + radius * std::sin(angle_radians),
                 center.y - radius * std::cos(angle_radians)};
}

}  // namespace clockapp::synctime
