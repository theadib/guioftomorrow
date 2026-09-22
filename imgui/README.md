# imgui (Dear ImGui / C++) clock demonstrator

The Dear ImGui contester for the [GUI of Tomorrow](../README.md) comparison:
a tabbed clock app (wallclock, stopwatch, synctime, settings) built with
[Dear ImGui](https://github.com/ocornut/imgui) in C++, rendered via
[GLFW](https://www.glfw.org) + OpenGL3. See
[../OVERVIEW.md](../OVERVIEW.md#imgui-c-dear-imgui) for the framework
write-up and [IMPLEMENTATION.md](IMPLEMENTATION.md) for the completeness
checklist.

Unlike Flutter/Dioxus/Tauri, Dear ImGui has no built-in application
framework: this project owns the window, GL context, and main loop itself
(see [src/main.cpp](src/main.cpp)), which is exactly the "you bring your own
everything" tradeoff described in the OVERVIEW comparison.

## Installing the development environment

1. A C++17 compiler, [CMake](https://cmake.org) 3.20+, and
   [Ninja](https://ninja-build.org) (or any CMake-supported generator) —
   this project was built and tested against **GCC 15.2 / CMake 4.2 /
   Ninja 1.13** on Linux, but any reasonably recent toolchain works.
2. Platform windowing/OpenGL headers that [GLFW](https://www.glfw.org)
   (built from source by this project, see below) needs to compile:
   - **Linux desktop (X11 + Wayland)**:
     ```sh
     sudo apt install libx11-dev libxrandr-dev libxinerama-dev \
       libxcursor-dev libxi-dev libgl1-mesa-dev pkg-config
     ```
     (Wayland support is picked up automatically if `libwayland-dev` and
     `libxkbcommon-dev` are present; X11-only also works fine.)
   - **Windows desktop**: no extra system packages — Visual Studio's
     "Desktop development with C++" workload (linker + Windows SDK, which
     ships `opengl32`) is enough.
   - **macOS**: Xcode command line tools (`xcode-select --install`); OpenGL
     and Cocoa are provided by the system SDK.
3. From this directory, configure and build:
   ```sh
   cmake -S . -B build -G Ninja
   cmake --build build -j"$(nproc)"
   ```

> **No submodules, no package manager.** [CMakeLists.txt](CMakeLists.txt)
> uses CMake's `FetchContent` to pull pinned releases of GLFW (3.5.1), Dear
> ImGui (v1.92.9b, built directly from its sources — it has no CMake
> support upstream, so this project compiles the handful of `.cpp` files
> itself) and doctest (v2.5.3, tests only) straight from their GitHub repos
> at configure time. This needs network access on a clean build; there is
> no vendored fallback.

## Building, running and debugging

```sh
cmake --build build --target imgui_clock && ./build/imgui_clock
```

For a release-optimized build, configure once with
`-DCMAKE_BUILD_TYPE=Release` (or `RelWithDebInfo`, the default here) into a
separate build directory.

For interactive debugging (breakpoints, variable inspection), open this
folder in VS Code (C/C++ or CodeLLDB extension, pointed at
`build/compile_commands.json`) or any CMake-aware IDE (CLion, Qt Creator),
or drive it directly with `gdb ./build/imgui_clock`.

Run the unit test suite:

```sh
cmake --build build --target imgui_clock_tests
./build/imgui_clock_tests
# or, via CTest:
ctest --test-dir build --output-on-failure
```

## Creating the deliverable application package

```sh
cmake -S . -B build-release -G Ninja -DCMAKE_BUILD_TYPE=Release
cmake --build build-release --target imgui_clock -j"$(nproc)"
```

This produces a single executable (`build-release/imgui_clock` — everything
Dear ImGui/GLFW need is statically linked in except the system's own
OpenGL/X11/Wayland libraries, which are assumed present on any Linux
desktop) that can be copied and distributed directly — no installer step
for this demonstrator. Cross-compiling for Windows/macOS from Linux wasn't
exercised here; see [IMPLEMENTATION.md](IMPLEMENTATION.md).

## What it demonstrates

- **Wallclock** — current date/time in a user-selectable timezone
  ([src/tabs/wallclock.cpp](src/tabs/wallclock.cpp),
  [src/tabs/wallclock_ui.cpp](src/tabs/wallclock_ui.cpp)), computed via the
  C library's platform-specific timezone APIs and persisted across restarts; the
  `:` separators blink once a second as a small clock-face animation.
- **Stopwatch** — start/stop/reset/lap using `std::chrono::steady_clock`
  (monotonic, unaffected by system clock adjustments), with recorded laps
  exportable as a plain-text report written to the OS data directory
  ([src/tabs/stopwatch.cpp](src/tabs/stopwatch.cpp),
  [src/tabs/stopwatch_ui.cpp](src/tabs/stopwatch_ui.cpp)).
- **Synctime** — two `ImDrawList` arcs (one rotation/second, one
  rotation/minute) drawn around a live `HH:MM:SS` readout, redrawn every
  frame (vsync-limited, so ~60fps on most displays) via `ImGui`'s immediate
  mode — there's no separate "redraw timer" to manage, the whole UI simply
  re-renders every frame ([src/tabs/synctime.cpp](src/tabs/synctime.cpp),
  [src/tabs/synctime_ui.cpp](src/tabs/synctime_ui.cpp)).
- **Settings** — GUI theme (system/light/dark), contrast (normal/high, via
  `ImGuiStyle` color overrides), and a 50%–200% UI scale (`FontGlobalScale`
  + `ImGuiStyle::ScaleAllSizes`) — all persisted to an INI-style file in the
  OS config directory ([src/settings.cpp](src/settings.cpp)) and applied
  live, without restarting the app
  ([src/tabs/settings_tab.cpp](src/tabs/settings_tab.cpp)).

## GUI testing

Dear ImGui has no built-in testing framework (see
[../OVERVIEW.md](../OVERVIEW.md#imgui-c-dear-imgui) — the community
`imgui_test_engine` project exists but wasn't pulled in here). Following
the same strategy as the Dioxus contester, every piece of non-trivial logic
is factored out into small, pure, standalone functions with no ImGui/GLFW
dependency, and unit tested directly with
[doctest](https://github.com/doctest/doctest) in
[tests/test_main.cpp](tests/test_main.cpp):

- clock formatting and timezone conversion
  ([src/tabs/wallclock.cpp](src/tabs/wallclock.cpp))
- elapsed-time formatting and lap-report export text
  ([src/tabs/stopwatch.cpp](src/tabs/stopwatch.cpp))
- arc angle/point geometry ([src/tabs/synctime.cpp](src/tabs/synctime.cpp))
- settings (de)serialization and the curated timezone list
  ([src/settings.cpp](src/settings.cpp))

These pure functions live in the `imgui_clock_core` static library (see
[CMakeLists.txt](CMakeLists.txt)), kept deliberately free of any
`#include <imgui.h>` so the test binary doesn't need a GL context or a
display to run. The actual widget-drawing code
(`src/tabs/*_ui.cpp`, `src/main.cpp`) calls into these functions but is
itself only exercised by manually running the compiled app — see
[IMPLEMENTATION.md](IMPLEMENTATION.md) for what that check did and didn't
cover.

## Using it from Python

This demonstrator itself is plain C++; it doesn't use Python. The Dear
ImGui *framework* does have Python bindings (`pyimgui`, `imgui-bundle`) —
see the comparison notes in
[../OVERVIEW.md](../OVERVIEW.md#imgui-c-dear-imgui) — but building the same
demo through them wasn't attempted here.
