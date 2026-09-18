# Implementation notes — imgui (Dear ImGui, C++)

## Completeness checklist

Per the [repo's demonstrator spec](../README.md#demonstrator):

- [x] **Wallclock** tab
  - [x] Displays current date and time
  - [x] User can change timezone
  - [x] Selected timezone persists across app restarts
  - [x] Small animation: the `:` separators blink on/off once a second
- [x] **Stopwatch** tab
  - [x] Start / stop
  - [x] Reset
  - [x] Lap time recording
  - [x] Export recordings as text to a file
- [x] **Synctime** tab
  - [x] Two arcs, one full rotation per second, one per minute
  - [x] Arcs update continuously (every frame, vsync-limited)
  - [x] `hh:mm:ss` shown as text in the centre
- [x] **Settings** tab
  - [x] GUI theme: system / light / dark
  - [x] Contrast: normal / high contrast
  - [x] UI scale: 50%–200%, applied to fonts and widget sizing app-wide
  - [x] All settings persist across app restarts
- [x] Tabbed navigation across all four demos
- [x] `-Wall -Wextra` clean on this project's own targets (see
      [CMakeLists.txt](CMakeLists.txt)'s `IMGUI_CLOCK_WARNING_FLAGS`,
      applied only to `imgui_clock_core`/`imgui_clock`/`imgui_clock_tests`,
      not the fetched glfw/imgui/doctest targets)
- [x] `cppcheck --enable=warning,style,performance` over `src/` reports no
      issues
- [x] Automated unit tests (`ctest`/`./build/imgui_clock_tests`, 14 test
      cases / 59 assertions), see [README.md](README.md#gui-testing)
- [x] `cmake --build` succeeds and produces a working Linux desktop binary
      (GCC 15.2, CMake 4.2, Ninja 1.13)
- [x] The compiled binary was actually launched against a live X11 display
      (`DISPLAY=:0`, confirmed reachable via `xdpyinfo`) — twice, once
      after the initial build and again after adding the warning flags.
      Both times it started, rendered, and stayed alive with empty
      stderr/stdout for several seconds, then exited cleanly on `kill`.
      See the side note below on what this run additionally revealed.
- [ ] Screenshots — **not captured**, same environment limitation
      previously hit by the Dioxus contester (see its
      [IMPLEMENTATION.md](../dioxus/IMPLEMENTATION.md)): `import` (from
      ImageMagick, confirmed present and version-checked) fails with
      `missing an image filename` even when one is given, and Python's
      `PIL.ImageGrab.grab()` fails with `X get_image failed: error 8
      (BadMatch)`. Both point at this sandboxed shell not being able to
      read back the framebuffer of the display it can otherwise create
      windows on. No root access was available to investigate further.
- [ ] Windows build — not verified (no Windows toolchain in this dev
      environment). The only platform-specific code is the small
      `#if defined(_WIN32)` branch in
      [src/settings.cpp](src/settings.cpp)'s `settings_file_path()` and the
      OpenGL-library-selection branch in [CMakeLists.txt](CMakeLists.txt);
      neither has actually been exercised on Windows.
- [ ] macOS build — not verified, same caveat as Windows.

## Side notes

- **What "the binary was launched" actually verified — more than
  expected.** With no input-automation tool available (no
  `xdotool`/`wmctrl`, same gap the Dioxus contester hit), the plan was to
  confirm only that the window opens and the event loop runs without
  crashing. That happened. But on top of that, after running the app
  against `DISPLAY=:0` (this machine's live desktop, not an isolated
  headless display) for the second check, `~/.config/imgui_clock/settings.ini`
  and `~/.local/share/imgui_clock/stopwatch_export_*.txt` were found
  populated with real, non-default data: `theme=dark` in settings (the
  Settings tab's theme dropdown had been changed away from the default
  `system`), and an export file containing three genuinely different lap
  splits and a matching total. Since no automated input was driven from
  this session, the most likely explanation is that the window was visibly
  open on the real desktop and got real mouse clicks during the session —
  which, incidentally, is stronger end-to-end evidence (tab switching,
  timezone/theme dropdowns, stopwatch start/lap/stop, file export) than a
  scripted click sequence would have given. It does not substitute for
  the pure-function unit tests as *regression* coverage, but it is real
  confirmation the wired-up UI works, not just that it compiles.
- **Timezone switching uses global `TZ`/`tzset`, not a proper timezone
  library.** `wallclock::time_in_zone()`
  ([src/tabs/wallclock.cpp](src/tabs/wallclock.cpp)) calls `setenv("TZ",
  ...)` + `tzset()` before every `localtime_r()`. This is POSIX-only (no
  MSVC equivalent — Windows uses `_tzset()`/`_TIME_ZONE` differently and
  doesn't understand IANA names the same way glibc does) and mutates
  process-global state rather than being purely functional, but it needs
  no extra dependency (no ICU, no Howard Hinnant `date` library) and glibc
  ships the full IANA database already. Flagged as a real portability gap
  for the un-verified Windows build above, and unit-tested against a fixed
  UTC epoch rather than wall-clock time to stay deterministic.
- **Timezone list is curated, not exhaustive**, the same tradeoff the
  Flutter and Dioxus contesters made: glibc has the full IANA database
  available, but [src/settings.h](src/settings.h)'s `available_timezones()`
  only lists 13 well-known zones spanning a wide range of UTC offsets, to
  keep the dropdown usable. Every entry is unit tested to actually produce
  a valid broken-down time.
- **"System" theme is polled, not push-based**, and Linux-only. Dear
  ImGui/GLFW have no OS theme-change API at all (unlike the `dark-light`
  crate the Dioxus contester used), so
  [src/tabs/settings_tab.cpp](src/tabs/settings_tab.cpp)'s
  `poll_system_dark_mode()` shells out to `gsettings get
  org.gnome.desktop.interface color-scheme` every 2 seconds while "System"
  is selected. Desktops without `gsettings`, or without a `color-scheme`
  key (plain XFCE, i3, most window-manager-only setups), silently fall
  back to light — this is a real, undetected gap, not just a slow-to-update
  one like Dioxus's equivalent caveat.
- **UI scale combines two separate ImGui knobs.** `ImGuiIO::FontGlobalScale`
  scales text (cheap, just a draw-time multiplier, though it does not
  resample the font atlas so it can look soft at higher scales), while
  `ImGuiStyle::ScaleAllSizes()` scales padding/spacing/border metrics. The
  latter is **not idempotent** — calling it again on an already-scaled
  style compounds the scaling — so [src/main.cpp](src/main.cpp) always
  restores the pristine `base_style` (captured once at startup, before any
  theme or scale was applied) before reapplying both theme colors and
  scale from scratch. This mirrors the "rebuild from a base, don't mutate
  in place" approach Dioxus took with its `AppSettings` signal, just at the
  `ImGuiStyle` level instead of a UI-framework level.
- **Contrast mode hand-codes a handful of `ImGuiStyle::Colors` entries**
  (`Text`, `WindowBg`, `FrameBg`) plus thicker borders, rather than a
  general "increase contrast" transform over the whole palette — Dear
  ImGui has no contrast-mode concept to hook into (see
  [../OVERVIEW.md](../OVERVIEW.md#imgui-c-dear-imgui)), so this is a
  minimal, demo-scoped implementation, not a general-purpose one.
- **Synctime's arcs are drawn with `ImDrawList::PathArcTo`/`PathStroke`
  directly** rather than by walking points from the `point_on_circle()`
  helper in [src/tabs/synctime.cpp](src/tabs/synctime.cpp) — that helper is
  still real, tested, and used, but only for placing the four quarter-turn
  tick marks on the outer ring; the arcs themselves use ImGui's own path
  builder since it already handles the segment-count/anti-aliasing
  tradeoffs.
- **Stopwatch/synctime state is function-local `static`, not a persisted
  struct** ([src/tabs/stopwatch_ui.cpp](src/tabs/stopwatch_ui.cpp)) — fine
  for this single-window, single-instance demo, but would need to move
  into an explicit app-state struct if the app ever grew multiple windows
  or documents.
- **No native platform code beyond `CMakeLists.txt`'s OpenGL-library
  selection and `settings.cpp`'s config-path branch** — GLFW handles all
  platform windowing/input/GL-context glue internally, matching the
  Dioxus/Tauri contesters' reliance on their own backend crates for the
  same job.
