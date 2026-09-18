#pragma once

#include <ctime>
#include <string>

namespace clockapp::wallclock {

// Breaks down a UTC timestamp into local wall-clock fields for the given
// IANA zone name (e.g. "Europe/Berlin"). Uses the C library's TZ database
// via TZ/tzset/localtime_r, so it depends on process-global state (the TZ
// environment variable) rather than being purely functional, but the
// timestamp -> broken-down-time mapping it computes is what's tested.
std::tm time_in_zone(const std::string& tz, std::time_t utc_now);

// Formats hh:mm:ss, replacing the ':' separators with a space when
// `colon_visible` is false -- this is toggled once a second by the caller
// to produce the blinking-separator animation.
std::string format_clock(const std::tm& local_time, bool colon_visible);

// Formats the date portion, e.g. "2026-09-18 (Friday)".
std::string format_date(const std::tm& local_time);

}  // namespace clockapp::wallclock
