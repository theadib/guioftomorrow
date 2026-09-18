#pragma once

#include <chrono>
#include <optional>
#include <string>
#include <vector>

namespace clockapp::stopwatch {

struct Lap {
    int index;
    std::chrono::milliseconds split;       // time since the previous lap
    std::chrono::milliseconds total_at_lap; // time since start when recorded
};

// Formats a duration as "MM:SS.mmm", or "H:MM:SS.mmm" once it reaches an
// hour -- pure formatting, no wall-clock/system-time dependency.
std::string format_elapsed(std::chrono::milliseconds elapsed);

// Builds the plain-text lap report written out by the "Export" button.
std::string build_export_text(const std::vector<Lap>& laps, std::chrono::milliseconds total);

// Full path (inside the OS data directory) the next export would be
// written to, e.g. .../imgui_clock/stopwatch_export_20260918_153000.txt.
// `now` is injected so the naming scheme is independently testable.
std::string export_file_path(std::time_t now);

// Writes `text` to `path`, creating the parent directory if needed.
// Returns false if the file could not be written.
bool write_export_file(const std::string& path, const std::string& text);

}  // namespace clockapp::stopwatch
