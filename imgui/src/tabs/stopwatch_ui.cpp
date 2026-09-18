#include "stopwatch_ui.h"

#include <imgui.h>

#include <chrono>
#include <ctime>
#include <vector>

#include "stopwatch.h"

namespace clockapp::ui {

void draw_stopwatch_tab() {
    using Clock = std::chrono::steady_clock;

    static bool running = false;
    static Clock::time_point start_time{};
    static std::chrono::milliseconds accumulated{0};
    static std::vector<clockapp::stopwatch::Lap> laps;
    static std::chrono::milliseconds last_lap_total{0};
    static std::string status_message;

    std::chrono::milliseconds elapsed = accumulated;
    if (running) {
        elapsed += std::chrono::duration_cast<std::chrono::milliseconds>(Clock::now() - start_time);
    }

    ImGui::SetWindowFontScale(2.0f);
    ImGui::Text("%s", clockapp::stopwatch::format_elapsed(elapsed).c_str());
    ImGui::SetWindowFontScale(1.0f);
    ImGui::Spacing();

    if (!running) {
        if (ImGui::Button("Start")) {
            running = true;
            start_time = Clock::now();
        }
    } else {
        if (ImGui::Button("Stop")) {
            running = false;
            accumulated = elapsed;
        }
    }

    ImGui::SameLine();
    ImGui::BeginDisabled(!running);
    if (ImGui::Button("Lap")) {
        const int index = static_cast<int>(laps.size()) + 1;
        const auto split = elapsed - last_lap_total;
        laps.push_back(clockapp::stopwatch::Lap{index, split, elapsed});
        last_lap_total = elapsed;
    }
    ImGui::EndDisabled();

    ImGui::SameLine();
    if (ImGui::Button("Reset")) {
        running = false;
        accumulated = std::chrono::milliseconds{0};
        laps.clear();
        last_lap_total = std::chrono::milliseconds{0};
        status_message.clear();
    }

    ImGui::SameLine();
    if (ImGui::Button("Export laps to file")) {
        const std::string text = clockapp::stopwatch::build_export_text(laps, elapsed);
        const std::string path = clockapp::stopwatch::export_file_path(std::time(nullptr));
        const bool ok = clockapp::stopwatch::write_export_file(path, text);
        status_message = ok ? ("Exported to " + path) : ("Failed to write " + path);
    }

    if (!status_message.empty()) {
        ImGui::TextWrapped("%s", status_message.c_str());
    }

    ImGui::Separator();
    ImGui::Text("Laps");
    ImGui::BeginChild("laps", ImVec2(0, 160), true);
    for (auto it = laps.rbegin(); it != laps.rend(); ++it) {
        ImGui::Text("Lap %d   split %s   total %s", it->index,
                     clockapp::stopwatch::format_elapsed(it->split).c_str(),
                     clockapp::stopwatch::format_elapsed(it->total_at_lap).c_str());
    }
    ImGui::EndChild();
}

}  // namespace clockapp::ui
