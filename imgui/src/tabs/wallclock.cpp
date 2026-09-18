#include "wallclock.h"

#include <array>
#include <cstdio>
#include <cstdlib>

namespace clockapp::wallclock {

std::tm time_in_zone(const std::string& tz, std::time_t utc_now) {
    // setenv/tzset is the classic POSIX way to ask localtime_r() to convert
    // into an arbitrary IANA zone rather than the process's own local zone.
    // Not thread-safe (global TZ state) and POSIX-only (no MSVC support),
    // both fine for this single-threaded demo -- see IMPLEMENTATION.md.
    setenv("TZ", tz.c_str(), 1);
    tzset();
    std::tm result{};
    localtime_r(&utc_now, &result);
    return result;
}

std::string format_clock(const std::tm& local_time, bool colon_visible) {
    const char sep = colon_visible ? ':' : ' ';
    std::array<char, 16> buf{};
    std::snprintf(buf.data(), buf.size(), "%02d%c%02d%c%02d", local_time.tm_hour, sep,
                  local_time.tm_min, sep, local_time.tm_sec);
    return std::string(buf.data());
}

std::string format_date(const std::tm& local_time) {
    std::array<char, 64> buf{};
    std::strftime(buf.data(), buf.size(), "%Y-%m-%d (%A)", &local_time);
    return std::string(buf.data());
}

}  // namespace clockapp::wallclock
