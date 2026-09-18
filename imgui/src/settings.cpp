#include "settings.h"

#include <algorithm>
#include <filesystem>
#include <fstream>
#include <sstream>

namespace clockapp {

const std::vector<std::string>& available_timezones() {
    static const std::vector<std::string> zones = {
        "UTC",
        "Europe/Berlin",
        "Europe/London",
        "Europe/Moscow",
        "America/New_York",
        "America/Los_Angeles",
        "America/Sao_Paulo",
        "Asia/Tokyo",
        "Asia/Shanghai",
        "Asia/Kolkata",
        "Asia/Dubai",
        "Australia/Sydney",
        "Pacific/Auckland",
    };
    return zones;
}

std::string theme_to_string(Theme theme) {
    switch (theme) {
        case Theme::Light: return "light";
        case Theme::Dark: return "dark";
        case Theme::System: default: return "system";
    }
}

std::optional<Theme> theme_from_string(const std::string& text) {
    if (text == "light") return Theme::Light;
    if (text == "dark") return Theme::Dark;
    if (text == "system") return Theme::System;
    return std::nullopt;
}

std::string contrast_to_string(Contrast contrast) {
    return contrast == Contrast::High ? "high" : "normal";
}

std::optional<Contrast> contrast_from_string(const std::string& text) {
    if (text == "high") return Contrast::High;
    if (text == "normal") return Contrast::Normal;
    return std::nullopt;
}

float clamp_scale(float scale) {
    return std::clamp(scale, 0.5f, 2.0f);
}

std::string serialize_settings(const AppSettings& settings) {
    std::ostringstream out;
    out << "timezone=" << settings.timezone << "\n";
    out << "theme=" << theme_to_string(settings.theme) << "\n";
    out << "contrast=" << contrast_to_string(settings.contrast) << "\n";
    out << "scale=" << clamp_scale(settings.scale) << "\n";
    return out.str();
}

AppSettings parse_settings(const std::string& text) {
    AppSettings settings;  // start from defaults; unrecognized/missing keys keep them
    std::istringstream in(text);
    std::string line;
    while (std::getline(in, line)) {
        const auto eq = line.find('=');
        if (eq == std::string::npos) continue;
        const std::string key = line.substr(0, eq);
        const std::string value = line.substr(eq + 1);
        if (key == "timezone" && !value.empty()) {
            settings.timezone = value;
        } else if (key == "theme") {
            if (auto t = theme_from_string(value)) settings.theme = *t;
        } else if (key == "contrast") {
            if (auto c = contrast_from_string(value)) settings.contrast = *c;
        } else if (key == "scale") {
            try {
                settings.scale = clamp_scale(std::stof(value));
            } catch (...) {
                // keep default on malformed number
            }
        }
    }
    return settings;
}

std::string settings_file_path() {
    namespace fs = std::filesystem;
#if defined(_WIN32)
    const char* appdata = std::getenv("APPDATA");
    fs::path base = appdata ? fs::path(appdata) : fs::path(".");
#else
    const char* xdg = std::getenv("XDG_CONFIG_HOME");
    fs::path base;
    if (xdg && *xdg) {
        base = fs::path(xdg);
    } else {
        const char* home = std::getenv("HOME");
        base = fs::path(home ? home : ".") / ".config";
    }
#endif
    return (base / "imgui_clock" / "settings.ini").string();
}

AppSettings load_settings() {
    std::ifstream file(settings_file_path());
    if (!file) return AppSettings{};
    std::ostringstream buffer;
    buffer << file.rdbuf();
    return parse_settings(buffer.str());
}

bool save_settings(const AppSettings& settings) {
    namespace fs = std::filesystem;
    const fs::path path = settings_file_path();
    std::error_code ec;
    fs::create_directories(path.parent_path(), ec);
    std::ofstream file(path, std::ios::trunc);
    if (!file) return false;
    file << serialize_settings(settings);
    return static_cast<bool>(file);
}

}  // namespace clockapp
