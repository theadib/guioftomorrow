// imgui clock demonstrator -- the "GUI of Tomorrow" imgui/C++ contester.
// Boilerplate (window/GL context setup, main loop) follows Dear ImGui's own
// examples/example_glfw_opengl3/main.cpp; everything under tabs/ is this
// project's demonstrator code.

#include <GLFW/glfw3.h>
#include <imgui.h>
#include <imgui_impl_glfw.h>
#include <imgui_impl_opengl3.h>

#include <cstdio>

#include "settings.h"
#include "tabs/settings_tab.h"
#include "tabs/stopwatch_ui.h"
#include "tabs/synctime_ui.h"
#include "tabs/wallclock_ui.h"

namespace {
void glfw_error_callback(int error, const char* description) {
    std::fprintf(stderr, "GLFW error %d: %s\n", error, description);
}
}  // namespace

int main() {
    glfwSetErrorCallback(glfw_error_callback);
    if (!glfwInit()) return 1;

    const char* glsl_version = "#version 130";
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 0);

    GLFWwindow* window =
        glfwCreateWindow(900, 640, "GUI of Tomorrow - imgui clock demonstrator", nullptr, nullptr);
    if (!window) {
        glfwTerminate();
        return 1;
    }
    glfwMakeContextCurrent(window);
    glfwSwapInterval(1);  // vsync

    IMGUI_CHECKVERSION();
    ImGui::CreateContext();
    ImGuiIO& io = ImGui::GetIO();
    io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;

    // Capture the pristine, unscaled style once, before any theme/scale is
    // applied -- apply_style() rebuilds from this each time rather than
    // compounding ScaleAllSizes() calls on top of an already-scaled style.
    ImGui::StyleColorsDark();
    const ImGuiStyle base_style = ImGui::GetStyle();

    ImGui_ImplGlfw_InitForOpenGL(window, true);
    ImGui_ImplOpenGL3_Init(glsl_version);

    clockapp::AppSettings settings = clockapp::load_settings();
    bool settings_dirty = false;
    bool system_is_dark = clockapp::ui::poll_system_dark_mode();
    clockapp::ui::apply_style(base_style, settings, system_is_dark);

    double last_system_theme_poll = glfwGetTime();

    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();

        // Re-poll the OS dark-mode setting every 2s (only matters while
        // "System" theme is selected) -- see settings_tab.h's caveat about
        // this being poll-based, not push-based.
        const double now = glfwGetTime();
        if (settings.theme == clockapp::Theme::System && now - last_system_theme_poll > 2.0) {
            const bool new_system_is_dark = clockapp::ui::poll_system_dark_mode();
            if (new_system_is_dark != system_is_dark) {
                system_is_dark = new_system_is_dark;
                settings_dirty = true;  // reuse the dirty flag to trigger a re-apply below
            }
            last_system_theme_poll = now;
        }

        ImGui_ImplOpenGL3_NewFrame();
        ImGui_ImplGlfw_NewFrame();
        ImGui::NewFrame();

        const ImGuiViewport* viewport = ImGui::GetMainViewport();
        ImGui::SetNextWindowPos(viewport->WorkPos);
        ImGui::SetNextWindowSize(viewport->WorkSize);
        ImGui::Begin("GUI of Tomorrow - imgui clock demonstrator", nullptr,
                     ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoMove |
                         ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_NoTitleBar);

        if (ImGui::BeginTabBar("tabs")) {
            if (ImGui::BeginTabItem("Wallclock")) {
                clockapp::ui::draw_wallclock_tab(settings, settings_dirty);
                ImGui::EndTabItem();
            }
            if (ImGui::BeginTabItem("Stopwatch")) {
                clockapp::ui::draw_stopwatch_tab();
                ImGui::EndTabItem();
            }
            if (ImGui::BeginTabItem("Synctime")) {
                clockapp::ui::draw_synctime_tab();
                ImGui::EndTabItem();
            }
            if (ImGui::BeginTabItem("Settings")) {
                clockapp::ui::draw_settings_tab(settings, settings_dirty);
                ImGui::EndTabItem();
            }
            ImGui::EndTabBar();
        }
        ImGui::End();

        if (settings_dirty) {
            clockapp::ui::apply_style(base_style, settings, system_is_dark);
            clockapp::save_settings(settings);
            settings_dirty = false;
        }

        ImGui::Render();
        int display_w, display_h;
        glfwGetFramebufferSize(window, &display_w, &display_h);
        glViewport(0, 0, display_w, display_h);
        glClearColor(0.1f, 0.1f, 0.12f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);
        ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
        glfwSwapBuffers(window);
    }

    ImGui_ImplOpenGL3_Shutdown();
    ImGui_ImplGlfw_Shutdown();
    ImGui::DestroyContext();

    glfwDestroyWindow(window);
    glfwTerminate();
    return 0;
}
