#pragma once

#include <imgui.h>

#include "../settings.h"

namespace clockapp::ui {

// Best-effort OS dark-mode detection for the "System" theme option. Dear
// ImGui has no built-in OS theme API (see OVERVIEW.md's imgui theming
// notes), so this shells out to `gsettings` on Linux; desktops without it
// (or without a "color-scheme" key) fall back to light. See
// IMPLEMENTATION.md for the caveat.
bool poll_system_dark_mode();

// Rebuilds ImGui::GetStyle() from `base_style` (the pristine style captured
// once at startup) applying the theme/contrast/scale in `settings`. Called
// whenever settings change or the polled system theme flips, rather than
// every frame, since ScaleAllSizes() is not idempotent to call repeatedly
// on an already-scaled style.
void apply_style(const ImGuiStyle& base_style, const clockapp::AppSettings& settings,
                  bool system_is_dark);

// Draws the settings tab. Sets `settings_dirty` on any change so the
// caller persists settings to disk and re-applies the style.
void draw_settings_tab(clockapp::AppSettings& settings, bool& settings_dirty);

}  // namespace clockapp::ui
