#pragma once

#include <QString>
#include <QStringList>

#include <optional>

class QSettings;

namespace clockapp {

enum class Theme { System, Light, Dark };
enum class Contrast { Normal, High };

struct AppSettings {
    QString timezone = QStringLiteral("UTC");
    Theme theme = Theme::System;
    Contrast contrast = Contrast::Normal;
    double scale = 1.0;  // 0.5 .. 2.0
};

// Curated list of IANA zone names offered in the wallclock tab's dropdown,
// same tradeoff as the other contesters: not exhaustive, just well-known
// zones spanning a wide range of UTC offsets.
const QStringList& available_timezones();

QString theme_to_string(Theme theme);
std::optional<Theme> theme_from_string(const QString& text);

QString contrast_to_string(Contrast contrast);
std::optional<Contrast> contrast_from_string(const QString& text);

// Clamp a requested UI scale into the supported 50%-200% range.
double clamp_scale(double scale);

// Pure (de)serialization to/from an already-open QSettings store, kept
// separate from load_settings()/save_settings() below so tests can point a
// QSettings at a temporary file instead of the real per-user config file.
void write_settings(QSettings& store, const AppSettings& settings);
AppSettings read_settings(QSettings& store);

// Full path to the settings file QSettings will read/write (an ini file
// under the OS config directory: $XDG_CONFIG_HOME or ~/.config on Linux,
// %APPDATA% on Windows, ~/Library/Preferences on macOS).
QString settings_file_path();

// Load settings from the real per-user config file, falling back to
// defaults for anything missing, unreadable, or invalid.
AppSettings load_settings();

// Write settings to the real per-user config file.
void save_settings(const AppSettings& settings);

}  // namespace clockapp
