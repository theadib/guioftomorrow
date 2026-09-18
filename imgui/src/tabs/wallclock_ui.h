#pragma once

#include "../settings.h"

namespace clockapp::ui {

// Draws the wallclock tab. Sets `settings_dirty` when the user picks a new
// timezone so the caller knows to persist settings to disk.
void draw_wallclock_tab(clockapp::AppSettings& settings, bool& settings_dirty);

}  // namespace clockapp::ui
