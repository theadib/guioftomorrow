#pragma once

#include <cstdlib>
#include <ctime>
#include <string>

namespace clockapp::platform {

inline void set_timezone(const std::string& timezone) {
#ifdef _WIN32
    _putenv_s("TZ", timezone.c_str());
    _tzset();
#else
    setenv("TZ", timezone.c_str(), 1);
    tzset();
#endif
}

inline std::tm local_time(std::time_t timestamp) {
    std::tm result{};
#ifdef _WIN32
    localtime_s(&result, &timestamp);
#else
    localtime_r(&timestamp, &result);
#endif
    return result;
}

}  // namespace clockapp::platform