#pragma once

#include <cstdlib>
#include <optional>
#include <string>
#include <vector>

namespace clockapp {

enum class Theme { System, Light, Dark };
enum class Contrast { Normal, High };

struct AppSettings {
    std::string timezone = "UTC";
    Theme theme = Theme::System;
    Contrast contrast = Contrast::Normal;
    float scale = 1.0f;  // 0.5 .. 2.0
};

// Curated list of IANA zone names offered in the wallclock tab's dropdown,
// same tradeoff as the other contesters: not exhaustive, just well-known
// zones spanning a wide range of UTC offsets.
const std::vector<std::string>& available_timezones();

std::string theme_to_string(Theme theme);
std::optional<Theme> theme_from_string(const std::string& text);

std::string contrast_to_string(Contrast contrast);
std::optional<Contrast> contrast_from_string(const std::string& text);

// Pure (de)serialization to a simple "key=value" line format, independent
// of any actual filesystem access so it can be unit tested directly.
std::string serialize_settings(const AppSettings& settings);
AppSettings parse_settings(const std::string& text);

// Clamp a requested UI scale into the supported 50%-200% range.
float clamp_scale(float scale);

// Full path to the settings file inside the OS config directory
// ($XDG_CONFIG_HOME or ~/.config on Linux/macOS, %APPDATA% on Windows).
std::string settings_file_path();

// Load settings from disk, falling back to defaults if the file is
// missing, unreadable, or contains invalid values.
AppSettings load_settings();

// Write settings to disk (creating the parent directory if needed).
// Returns false if the file could not be written.
bool save_settings(const AppSettings& settings);

}  // namespace clockapp
