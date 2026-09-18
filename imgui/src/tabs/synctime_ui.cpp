#include "synctime_ui.h"

#include <imgui.h>

#include <chrono>
#include <cstdio>
#include <ctime>

#include "synctime.h"

namespace clockapp::ui {

void draw_synctime_tab() {
    using clockapp::synctime::angle_for_fraction;
    using clockapp::synctime::fraction_of_period;
    using clockapp::synctime::point_on_circle;
    using clockapp::synctime::Point;

    const auto now_tp = std::chrono::system_clock::now();
    const double seconds_since_epoch =
        std::chrono::duration<double>(now_tp.time_since_epoch()).count();
    const std::time_t now_t = std::chrono::system_clock::to_time_t(now_tp);
    std::tm local{};
    localtime_r(&now_t, &local);

    const float scale = ImGui::GetIO().FontGlobalScale;
    const float outer_radius = 110.0f * scale;
    const float inner_radius = 75.0f * scale;
    const float padding = 20.0f * scale;

    ImVec2 origin = ImGui::GetCursorScreenPos();
    ImVec2 center(origin.x + outer_radius + padding, origin.y + outer_radius + padding);
    ImDrawList* draw_list = ImGui::GetWindowDrawList();

    draw_list->AddCircle(center, outer_radius, IM_COL32(130, 130, 130, 90), 64, 1.5f);
    draw_list->AddCircle(center, inner_radius, IM_COL32(130, 130, 130, 90), 64, 1.5f);

    // Quarter tick marks on the outer ring, placed with the pure
    // point_on_circle() geometry helper (unit-tested separately).
    for (int i = 0; i < 4; ++i) {
        const float angle = angle_for_fraction(i / 4.0);
        const Point a = point_on_circle({center.x, center.y}, outer_radius + 6.0f * scale, angle);
        const Point b = point_on_circle({center.x, center.y}, outer_radius - 6.0f * scale, angle);
        draw_list->AddLine(ImVec2(a.x, a.y), ImVec2(b.x, b.y), IM_COL32(200, 200, 200, 180),
                            2.0f * scale);
    }

    constexpr float kPi = 3.14159265358979323846f;
    const float start_angle = -kPi * 0.5f;  // 12 o'clock
    const float angle_sec = angle_for_fraction(fraction_of_period(seconds_since_epoch, 1.0));
    const float angle_min = angle_for_fraction(fraction_of_period(seconds_since_epoch, 60.0));

    draw_list->PathArcTo(center, outer_radius, start_angle, start_angle + angle_sec, 64);
    draw_list->PathStroke(IM_COL32(70, 160, 255, 255), 0, 4.0f * scale);

    draw_list->PathArcTo(center, inner_radius, start_angle, start_angle + angle_min, 64);
    draw_list->PathStroke(IM_COL32(255, 170, 60, 255), 0, 4.0f * scale);

    char buf[16];
    std::snprintf(buf, sizeof buf, "%02d:%02d:%02d", local.tm_hour, local.tm_min, local.tm_sec);
    const ImVec2 text_size = ImGui::CalcTextSize(buf);
    draw_list->AddText(ImVec2(center.x - text_size.x / 2.0f, center.y - text_size.y / 2.0f),
                        IM_COL32(230, 230, 230, 255), buf);

    ImGui::Dummy(ImVec2((outer_radius + padding) * 2.0f, (outer_radius + padding) * 2.0f));
    ImGui::TextWrapped(
        "Outer arc: one full rotation per second. Inner arc: one full rotation per minute. Run "
        "this on two machines to visually compare their clocks.");
}

}  // namespace clockapp::ui
