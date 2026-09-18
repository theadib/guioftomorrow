#pragma once

#include <QFont>

#include "settings.h"

namespace clockapp::ui {

// Rebuilds the application-wide QPalette/QFont/stylesheet from `settings`,
// consulting the live OS color scheme (Qt 6.5+'s QStyleHints::colorScheme())
// for the "System" theme option. Unlike the imgui/C contester's 2-second
// polling loop, this only needs to be called on an actual settings change
// or a QStyleHints::colorSchemeChanged signal -- see MainWindow.
//
// `base_font` is the application's pristine, unscaled font captured once at
// startup (before any scale was ever applied) -- apply_style() always
// rebuilds from it rather than compounding scale factors on top of an
// already-scaled font.
void apply_style(const clockapp::AppSettings& settings, const QFont& base_font);

}  // namespace clockapp::ui
