#pragma once

namespace clockapp::synctime {

struct Point {
    double x;
    double y;
};

// How far into a `period_seconds`-long cycle `seconds_since_epoch` falls,
// as a fraction in [0, 1). E.g. period 1.0 for the once-a-second arc,
// period 60.0 for the once-a-minute arc.
double fraction_of_period(double seconds_since_epoch, double period_seconds);

// Converts a [0, 1) fraction of a full rotation into radians, measured
// clockwise from the top (12 o'clock position).
double angle_for_fraction(double fraction);

// Point on a circle for a given angle, measured clockwise from the top
// (12 o'clock position), matching screen coordinates (y grows downward).
Point point_on_circle(Point center, double radius, double angle_radians);

}  // namespace clockapp::synctime
