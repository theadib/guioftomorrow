#pragma once

namespace clockapp::ui {

// Draws the synctime tab: two continuously-updating arcs (one full
// rotation/second, one full rotation/minute) around a live hh:mm:ss
// readout, so the same app running on two machines lets you visually
// compare their clocks.
void draw_synctime_tab();

}  // namespace clockapp::ui
