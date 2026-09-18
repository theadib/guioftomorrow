#include "settings_tab.h"

#include <array>
#include <cstdio>
#include <string>

namespace clockapp::ui {

namespace {

bool command_output_contains(const char* command, const char* needle) {
#if defined(__linux__)
    FILE* pipe = popen(command, "r");
    if (!pipe) return false;
    std::array<char, 256> buf{};
    std::string output;
    while (fgets(buf.data(), buf.size(), pipe)) output += buf.data();
    pclose(pipe);
    return output.find(needle) != std::string::npos;
#else
    (void)command;
    (void)needle;
    return false;
#endif
}

}  // namespace

bool poll_system_dark_mode() {
    return command_output_contains(
        "gsettings get org.gnome.desktop.interface color-scheme 2>/dev/null", "dark");
}

void apply_style(const ImGuiStyle& base_style, const clockapp::AppSettings& settings,
                  bool system_is_dark) {
    ImGuiStyle& style = ImGui::GetStyle();
    style = base_style;

    const bool use_dark = settings.theme == clockapp::Theme::Dark ||
                           (settings.theme == clockapp::Theme::System && system_is_dark);
    if (use_dark) {
        ImGui::StyleColorsDark(&style);
    } else {
        ImGui::StyleColorsLight(&style);
    }

    if (settings.contrast == clockapp::Contrast::High) {
        ImVec4* colors = style.Colors;
        if (use_dark) {
            colors[ImGuiCol_Text] = ImVec4(1.0f, 1.0f, 1.0f, 1.0f);
            colors[ImGuiCol_WindowBg] = ImVec4(0.0f, 0.0f, 0.0f, 1.0f);
            colors[ImGuiCol_FrameBg] = ImVec4(0.05f, 0.05f, 0.05f, 1.0f);
        } else {
            colors[ImGuiCol_Text] = ImVec4(0.0f, 0.0f, 0.0f, 1.0f);
            colors[ImGuiCol_WindowBg] = ImVec4(1.0f, 1.0f, 1.0f, 1.0f);
            colors[ImGuiCol_FrameBg] = ImVec4(0.9f, 0.9f, 0.9f, 1.0f);
        }
        style.FrameBorderSize = 2.0f;
        style.WindowBorderSize = 2.0f;
    }

    const float scale = clockapp::clamp_scale(settings.scale);
    style.ScaleAllSizes(scale);
    ImGui::GetIO().FontGlobalScale = scale;
}

void draw_settings_tab(clockapp::AppSettings& settings, bool& settings_dirty) {
    ImGui::Text("GUI theme");
    int theme_idx = static_cast<int>(settings.theme);
    const char* theme_labels[] = {"System", "Light", "Dark"};
    if (ImGui::Combo("##theme", &theme_idx, theme_labels, IM_ARRAYSIZE(theme_labels))) {
        settings.theme = static_cast<clockapp::Theme>(theme_idx);
        settings_dirty = true;
    }

    ImGui::Spacing();
    ImGui::Text("Contrast");
    int contrast_idx = static_cast<int>(settings.contrast);
    const char* contrast_labels[] = {"Normal", "High contrast"};
    if (ImGui::Combo("##contrast", &contrast_idx, contrast_labels, IM_ARRAYSIZE(contrast_labels))) {
        settings.contrast = static_cast<clockapp::Contrast>(contrast_idx);
        settings_dirty = true;
    }

    ImGui::Spacing();
    ImGui::Text("UI scale");
    float scale_percent = settings.scale * 100.0f;
    if (ImGui::SliderFloat("##scale", &scale_percent, 50.0f, 200.0f, "%.0f%%")) {
        settings.scale = clockapp::clamp_scale(scale_percent / 100.0f);
        settings_dirty = true;
    }

    ImGui::Spacing();
    ImGui::Separator();
    ImGui::TextDisabled("Settings are saved automatically and applied live.");
}

}  // namespace clockapp::ui
