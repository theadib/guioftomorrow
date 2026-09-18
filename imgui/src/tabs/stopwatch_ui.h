#pragma once

namespace clockapp::ui {

// Draws the stopwatch tab. Owns its running/laps state as function-local
// statics -- fine for this single-window, single-instance demo app.
void draw_stopwatch_tab();

}  // namespace clockapp::ui
