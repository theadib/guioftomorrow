#include "stopwatch.h"

#include <array>
#include <cstdio>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <sstream>

#include "platform_time.h"

namespace clockapp::stopwatch {

std::string format_elapsed(std::chrono::milliseconds elapsed) {
    using namespace std::chrono;
    const auto total_ms = elapsed.count();
    const auto hours_part = total_ms / 3'600'000;
    const auto minutes_part = (total_ms / 60'000) % 60;
    const auto seconds_part = (total_ms / 1000) % 60;
    const auto millis_part = total_ms % 1000;

    std::array<char, 32> buf{};
    if (hours_part > 0) {
        std::snprintf(buf.data(), buf.size(), "%lld:%02lld:%02lld.%03lld",
                      static_cast<long long>(hours_part), static_cast<long long>(minutes_part),
                      static_cast<long long>(seconds_part), static_cast<long long>(millis_part));
    } else {
        std::snprintf(buf.data(), buf.size(), "%02lld:%02lld.%03lld",
                      static_cast<long long>(minutes_part), static_cast<long long>(seconds_part),
                      static_cast<long long>(millis_part));
    }
    return std::string(buf.data());
}

std::string build_export_text(const std::vector<Lap>& laps, std::chrono::milliseconds total) {
    std::ostringstream out;
    out << "Stopwatch export (imgui contester)\n";
    out << "==================================\n";
    if (laps.empty()) {
        out << "(no laps recorded)\n";
    } else {
        for (const auto& lap : laps) {
            out << "Lap " << lap.index << ": split " << format_elapsed(lap.split) << ", total "
                << format_elapsed(lap.total_at_lap) << "\n";
        }
    }
    out << "----------------------------------\n";
    out << "Total elapsed: " << format_elapsed(total) << "\n";
    return out.str();
}

std::string export_file_path(std::time_t now) {
    namespace fs = std::filesystem;
    const char* xdg = std::getenv("XDG_DATA_HOME");
    fs::path base;
    if (xdg && *xdg) {
        base = fs::path(xdg);
    } else {
        const char* home = std::getenv("HOME");
        base = fs::path(home ? home : ".") / ".local" / "share";
    }
    const std::tm tm = platform::local_time(now);
    std::array<char, 32> stamp{};
    std::strftime(stamp.data(), stamp.size(), "%Y%m%d_%H%M%S", &tm);
    const std::string filename = std::string("stopwatch_export_") + stamp.data() + ".txt";
    return (base / "imgui_clock" / filename).string();
}

bool write_export_file(const std::string& path, const std::string& text) {
    namespace fs = std::filesystem;
    std::error_code ec;
    fs::create_directories(fs::path(path).parent_path(), ec);
    std::ofstream file(path, std::ios::trunc);
    if (!file) return false;
    file << text;
    return static_cast<bool>(file);
}

}  // namespace clockapp::stopwatch
