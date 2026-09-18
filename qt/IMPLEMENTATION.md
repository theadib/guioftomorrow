# Implementation notes — Qt Widgets (reference implementation)

**Not a contester** — see [README.md](README.md)'s opening note. This
checklist follows the same [repo demonstrator spec](../README.md#demonstrator)
as the actual contesters purely so the two are easy to compare side by side.

## Completeness checklist

- [x] **Wallclock** tab
  - [x] Displays current date and time
  - [x] User can change timezone
  - [x] Selected timezone persists across app restarts
  - [x] Small animation: the `:` separators blink on/off once a second
- [x] **Stopwatch** tab
  - [x] Start / stop
  - [x] Reset
  - [x] Lap time recording
  - [x] Export recordings as text to a file (via a native save dialog)
- [x] **Synctime** tab
  - [x] Two arcs, one full rotation per second, one per minute
  - [x] Arcs update continuously (60fps timer)
  - [x] `hh:mm:ss` shown as text in the centre
- [x] **Settings** tab
  - [x] GUI theme: system / light / dark
  - [x] Contrast: normal / high contrast
  - [x] UI scale: 50%–200%, applied to fonts and widget padding app-wide
  - [x] All settings persist across app restarts
- [x] Tabbed navigation across all four demos
- [x] `-Wall -Wextra` clean on this project's own targets (see
      [CMakeLists.txt](CMakeLists.txt)'s `QT_CLOCK_WARNING_FLAGS`, applied
      only to `qt_clock_core`/`qt_clock`/the `tst_*` test binaries)
- [x] `cppcheck --enable=warning,style,performance` over `src/` reports no
      real findings — only expected `unknownMacro` configuration notices
      for Qt's `slots`/`QT_END_NAMESPACE` macros, which cppcheck doesn't
      know about without a Qt-aware config
- [x] Automated unit tests (`ctest`, 4 suites / 19 test functions across
      `tst_settings`, `tst_wallclock`, `tst_stopwatch`, `tst_synctime`), see
      [README.md](README.md#gui-testing)
- [x] `cmake --build` succeeds and produces a working Linux desktop binary
      (GCC 15.2, CMake 4.2, Ninja 1.13, Qt 6.10.2)
- [x] The compiled binary was actually launched against a live display
      (`DISPLAY=:0`) and **screenshotted** — see the side note below. This
      is a step further than the imgui/Dioxus contesters managed in this
      same class of dev environment: their `IMPLEMENTATION.md`s record
      `import`/`PIL.ImageGrab` failing outright. Here, `import -window ""`
      (ImageMagick) worked, intermittently — most attempts succeeded on the
      first try, one needed a retry after a "missing an image filename"
      failure, matching the flakiness those contesters also described, just
      not the total failure they hit.
  - [x] Wallclock tab screenshotted: clock, date, timezone dropdown render
        correctly.
  - [x] Stopwatch tab screenshotted: Start/Lap/Reset/Export buttons, lap
        list, all present and correctly enabled/disabled at rest (Lap
        disabled until Start is pressed).
  - [x] Synctime tab screenshotted mid-rotation: outer (blue, per-second)
        and inner (orange, per-minute) arcs, quarter tick marks, and the
        centered `HH:mm:ss` readout all rendered as designed.
  - [x] Settings tab screenshotted at defaults (System / Normal / 100%).
  - [x] Live theming verified end-to-end: hand-edited
        `~/.config/GuiOfTomorrow/QtClock.ini` to
        `theme=dark, contrast=high, scale=1.5, timezone=Asia/Tokyo` *before*
        launch, relaunched, and confirmed via screenshot that `load_settings()`
        picked all four up correctly on startup — dark palette applied,
        thicker high-contrast borders visible on the combo box, ~1.5x
        larger text, and the clock correctly showing Tokyo local time (a
        9-hour offset from the UTC value shown in the earlier screenshot).
        This exercises the real `QSettings` I/O path end to end, not just
        the pure `read_settings()`/`write_settings()` functions the unit
        tests cover.
  - [x] No input-automation tool was available (no `xdotool`/`wmctrl`, the
        same gap the imgui/Dioxus contesters hit) to script actual clicks
        (Start/Lap/theme-dropdown/etc.) against the live process, so that
        interaction path itself relies on the unit tests plus manual code
        review of the `connect()` wiring in each `*_tab.cpp`, not a live
        end-to-end click-through.
- [ ] Windows build — not verified (no Windows toolchain in this dev
      environment). No platform-specific code exists in this project's own
      sources beyond CMake's `WIN32_EXECUTABLE`/`MACOSX_BUNDLE` target
      properties — Qt itself abstracts the rest — so this is a low-risk gap,
      but genuinely untested.
- [ ] macOS build — not verified, same caveat as Windows.
- [ ] `windeployqt`/`macdeployqt`/`linuxdeployqt` packaging — not
      exercised; see [README.md](README.md#creating-the-deliverable-application-package).

## Side notes

- **Timezone conversion uses `QTimeZone`/`QDateTime::toTimeZone()`, not an
  env-var hack.** This is a genuine, structural difference from the
  imgui/C contester, which has to `setenv("TZ", ...)` + `tzset()` before
  every `localtime_r()` call — POSIX-only, not thread-safe, and mutates
  process-global state. `wallclock::time_in_zone()`
  ([src/tabs/wallclock.cpp](src/tabs/wallclock.cpp)) is a pure function
  with no such side effect and works identically on Windows, since Qt ships
  its own IANA timezone backend (via the system's ICU/tzdata where
  available, with a bundled fallback) rather than relying on the C
  library's timezone support.
- **"System" theme is push-based, not polled — a real advantage over the
  imgui contester.** Dear ImGui/GLFW have no OS theme-change API at all, so
  that contester shells out to `gsettings` every 2 seconds. Qt 6.5 added
  `QGuiApplication::styleHints()->colorScheme()` plus a `colorSchemeChanged`
  signal; [src/mainwindow.cpp](src/mainwindow.cpp) connects that signal
  once and calls `apply_style()` immediately when it fires (only while
  "System" is selected), so a live OS theme switch is picked up instantly
  with no polling loop and no per-desktop-environment guesswork. The
  tradeoff: this is a fairly new Qt API (6.5, released 2023), so it isn't
  available on older LTS Qt deployments (e.g. Qt 6.2).
- **Theming is Fusion-style + hand-rolled `QPalette`, not the native
  platform style.** [src/main.cpp](src/main.cpp) forces
  `QStyleFactory::create("Fusion")` at startup specifically so the
  dark/light/high-contrast palettes in [src/theming.cpp](src/theming.cpp)
  actually take effect everywhere — some native styles (particularly
  Windows' default) partially ignore an app-set `QPalette`. The cost: the
  app no longer looks 100% native on any platform, trading "native
  look-and-feel" (one of Qt Widgets' classic selling points, see
  [../OVERVIEW.md](../OVERVIEW.md)) for reliable cross-platform theming —
  the same kind of tradeoff Flutter/Iced/Dioxus make by design by not using
  native widgets at all.
- **UI scale combines two separate, independent mechanisms.**
  `QApplication::setFont()` rescales text everywhere (cheap, and Qt
  re-flows layouts automatically since widget size hints are
  font-metric-based), while a small app-wide stylesheet
  (`QPushButton, QComboBox { padding: ...px ...px; }` in
  `apply_style()`) scales button/combo-box padding, since font size alone
  doesn't grow a button's click target. `apply_style()` always rebuilds
  both from the pristine `base_font` captured once at startup in
  `MainWindow`'s constructor (mirroring the imgui contester's
  "always rebuild from a captured base, don't compound scale factors"
  approach with `ImGuiStyle::ScaleAllSizes()`), rather than scaling
  whatever the current font happens to be.
- **Contrast mode hand-codes a handful of `QPalette` roles**
  (`WindowText`, `Text`, `ButtonText`, `Window`, `Base`, `Button`) to pure
  black/white plus a 2px border stylesheet, rather than a general
  "increase contrast" transform over the whole palette — Qt has no
  built-in contrast-mode concept to hook into any more than Dear ImGui
  does (see [../OVERVIEW.md](../OVERVIEW.md)), so this is the same
  minimal, demo-scoped tradeoff the imgui contester made, not a
  general-purpose one.
- **Export uses a real native save dialog
  (`QFileDialog::getSaveFileName()`), unlike the imgui contester.** Dear
  ImGui has no file dialog at all, so that contester always writes to a
  fixed, auto-generated path under the OS data directory. Here, the
  suggested default path is still auto-generated
  (`stopwatch::default_export_file_name()` under
  `QStandardPaths::DocumentsLocation`), but the user can redirect it
  anywhere via the OS's native picker — a straightforward win from having
  a real application framework underneath, rather than a gap in this
  implementation.
- **Synctime's arcs use `QPainter::drawArc()` directly**
  ([src/tabs/synctime_tab.cpp](src/tabs/synctime_tab.cpp)) rather than
  walking points from the shared `point_on_circle()` helper — that helper
  is still real, unit tested, and used, but only for placing the four
  quarter-turn tick marks; the arcs themselves use Qt's own arc primitive
  since it already handles anti-aliased curve rendering. Note
  `QPainter::drawArc()`'s angle convention (0 = 3 o'clock, positive =
  counter-clockwise, units of 1/16 degree) is the opposite winding
  direction from this project's `angle_for_fraction()` (clockwise from 12
  o'clock, radians) — the conversion (`90 * 16` start, negated span) is
  handled directly in the widget's `paintEvent()`, not in the shared/tested
  geometry module, so it's exercised only by the manual screenshot check
  above, not a unit test.
- **Stopwatch/synctime state lives on the widget instance, not in a shared
  app-state struct** ([src/tabs/stopwatch_tab.h](src/tabs/stopwatch_tab.h)) —
  fine for this single-window, single-instance demo (and arguably more
  idiomatic Qt than the imgui contester's function-local `static` variables,
  since each `StopwatchTab` instance now owns its own state cleanly), but
  would need to move into an explicit shared model if the app ever grew
  multiple windows showing the same stopwatch.
- **No native platform code at all**, beyond CMake's
  `WIN32_EXECUTABLE`/`MACOSX_BUNDLE` target properties in
  [CMakeLists.txt](CMakeLists.txt) — Qt itself abstracts window creation,
  the event loop, timers, and the timezone database, so unlike the imgui
  contester (which owns GLFW/OpenGL setup directly, see its own
  [IMPLEMENTATION.md](../imgui/IMPLEMENTATION.md)), there is no
  "bring your own windowing" code here at all. This is the central
  tradeoff this reference implementation is meant to illustrate: Qt Widgets
  gives you a complete, batteries-included application framework where the
  Rust/C++ contesters each have to assemble more of it themselves (or
  accept a webview) — at the cost of a heavier, non-vendorable system
  dependency and a UI that, once Fusion-styled for theming (see above),
  isn't fully native either.
