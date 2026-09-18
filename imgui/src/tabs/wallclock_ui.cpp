#include "wallclock_ui.h"

#include <imgui.h>

#include <chrono>
#include <ctime>

#include "wallclock.h"

namespace clockapp::ui {

void draw_wallclock_tab(clockapp::AppSettings& settings, bool& settings_dirty) {
    const std::time_t now = std::chrono::system_clock::to_time_t(std::chrono::system_clock::now());
    const std::tm local = clockapp::wallclock::time_in_zone(settings.timezone, now);
    // Blink the ':' separators on/off once a second (500ms visible, 500ms hidden).
    const bool colon_visible = (now % 2) == 0;

    ImGui::Spacing();
    ImGui::SetWindowFontScale(2.5f);
    ImGui::Text("%s", clockapp::wallclock::format_clock(local, colon_visible).c_str());
    ImGui::SetWindowFontScale(1.0f);
    ImGui::Text("%s", clockapp::wallclock::format_date(local).c_str());

    ImGui::Spacing();
    ImGui::Separator();
    ImGui::Spacing();
    ImGui::Text("Timezone");

    const auto& zones = clockapp::available_timezones();
    if (ImGui::BeginCombo("##timezone", settings.timezone.c_str())) {
        for (const auto& zone : zones) {
            const bool selected = (zone == settings.timezone);
            if (ImGui::Selectable(zone.c_str(), selected)) {
                if (settings.timezone != zone) {
                    settings.timezone = zone;
                    settings_dirty = true;
                }
            }
            if (selected) ImGui::SetItemDefaultFocus();
        }
        ImGui::EndCombo();
    }
}

}  // namespace clockapp::ui
