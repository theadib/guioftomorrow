# Qt Widgets clock demonstrator (reference implementation)

**Qt is not one of the [GUI of Tomorrow](../README.md) contesters** — the
root README explicitly leaves it out, alongside Slint's C++/Qt binding and
plain C#. This project exists anyway, purely **for reference**: Qt Widgets
is the mature, "old guard" native-widget toolkit the author has 20+ years
of history with (see the root README's opening line), so it makes a useful
baseline to compare the actual contesters against. It is intentionally
*not* listed in [../OVERVIEW.md](../OVERVIEW.md)'s contester table.

It implements the same tabbed clock app (wallclock, stopwatch, synctime,
settings) described in [../README.md](../README.md#demonstrator), built
with [Qt Widgets](https://doc.qt.io/qt-6/qtwidgets-index.html) (Qt 6) in
C++. See [IMPLEMENTATION.md](IMPLEMENTATION.md) for the completeness
checklist and side notes.

## Installing the development environment

1. A C++17 compiler and [CMake](https://cmake.org) 3.20+ (any generator;
   this project was built and tested with **GCC 15.2 / CMake 4.2 / Ninja
   1.13** on Linux).
2. Qt 6.5 or newer, with the **Widgets** and **Test** modules — this
   project was built and tested against **Qt 6.10.2**:
   - **Linux (Debian/Ubuntu)**:
     ```sh
     sudo apt install qt6-base-dev qt6-base-dev-tools
     ```
     (`qt6-base-dev` pulls in `libqt6widgets6`/`libqt6test6`; no separate
     package is needed for QtTest on this distro.)
   - **Windows / macOS**: install Qt via the
     [online installer](https://www.qt.io/download-qt-installer) (select
     the "Desktop" component for your compiler) or a package manager
     (`choco install qt6`, `brew install qt`), then point CMake at it with
     `-DCMAKE_PREFIX_PATH=<path-to-Qt-6.x.y>/<platform>` if it isn't found
     automatically.
3. From this directory, configure and build:
   ```sh
   cmake -S . -B build -G Ninja
   cmake --build build -j"$(nproc)"
   ```

No submodules, no `FetchContent`, no vendoring — Qt is a system dependency
here (installed via the platform's package manager / official installer),
unlike the imgui and Rust contesters, which pull their GUI dependency in at
build time. This is a genuine tradeoff worth naming directly: it makes the
build itself simpler (no network access needed at configure time, no
multi-minute first build compiling a whole GUI toolkit from source) at the
cost of needing a real Qt installation on the build machine.

## Building, running and debugging

```sh
cmake --build build --target qt_clock && ./build/qt_clock
```

For a release-optimized build, configure once with
`-DCMAKE_BUILD_TYPE=Release` (or `RelWithDebInfo`, the default here) into a
separate build directory.

For interactive debugging (breakpoints, variable inspection), open this
folder in **Qt Creator** (native project support, no extra config needed),
VS Code (C/C++ extension pointed at `build/compile_commands.json`), or any
CMake-aware IDE, or drive it directly with `gdb ./build/qt_clock`.

Run the unit test suite:

```sh
cmake --build build --target all
ctest --test-dir build --output-on-failure
# or run an individual suite directly, e.g.:
./build/tst_stopwatch
```

## Creating the deliverable application package

```sh
cmake -S . -B build-release -G Ninja -DCMAKE_BUILD_TYPE=Release
cmake --build build-release --target qt_clock -j"$(nproc)"
```

Unlike the imgui contester's fully static executable, `qt_clock` dynamically
links against the Qt libraries (`libQt6Widgets`, `libQt6Gui`, `libQt6Core`,
plus whichever platform plugin it loads at runtime), so it is **not**
directly redistributable as a single file:

- **Linux**: use [`linuxdeployqt`](https://github.com/probonopd/linuxdeployqt)
  or [`windeployqt`](https://doc.qt.io/qt-6/windows-deployment.html)'s Linux
  counterpart to bundle the Qt `.so`s and platform plugins next to the
  binary (or package it as a `.deb`/AppImage/Flatpak); not exercised here.
- **Windows**: run `windeployqt.exe build-release\qt_clock.exe` to copy the
  required Qt DLLs and plugins alongside the executable.
- **macOS**: run `macdeployqt build-release/qt_clock.app` to produce a
  self-contained `.app` bundle.

None of the three deployment tools above were actually run in this dev
environment (Linux-only, no Windows/macOS toolchain) — see
[IMPLEMENTATION.md](IMPLEMENTATION.md).

## What it demonstrates

- **Wallclock** — current date/time in a user-selectable IANA timezone
  ([src/tabs/wallclock.cpp](src/tabs/wallclock.cpp),
  [src/tabs/wallclock_tab.cpp](src/tabs/wallclock_tab.cpp)), computed via
  `QTimeZone`/`QDateTime::toTimeZone()` and persisted across restarts; the
  `:` separators blink once a second as a small clock-face animation.
  Unlike the imgui/C contester's `setenv("TZ", ...)` + `tzset()` approach,
  this touches no process-global state — see
  [IMPLEMENTATION.md](IMPLEMENTATION.md).
- **Stopwatch** — start/stop/reset/lap using `QElapsedTimer` (monotonic,
  unaffected by system clock adjustments), with recorded laps exportable as
  a plain-text report through a native `QFileDialog::getSaveFileName()` save
  dialog ([src/tabs/stopwatch.cpp](src/tabs/stopwatch.cpp),
  [src/tabs/stopwatch_tab.cpp](src/tabs/stopwatch_tab.cpp)) — a real UX
  advantage over the imgui contester, which has no file dialog and always
  writes to a fixed, auto-generated path.
- **Synctime** — two arcs (one rotation/second, one rotation/minute) drawn
  with `QPainter::drawArc()` around a live `HH:mm:ss` readout, redrawn on a
  60fps `QTimer` ([src/tabs/synctime.cpp](src/tabs/synctime.cpp),
  [src/tabs/synctime_tab.cpp](src/tabs/synctime_tab.cpp)); tick and arc
  colors are read from the active `QPalette`, so they automatically follow
  whatever theme is selected instead of being hardcoded.
- **Settings** — GUI theme (system/light/dark), contrast (normal/high, via
  `QPalette` overrides), and a 50%–200% UI scale (`QApplication::setFont()`
  plus a small padding stylesheet) — all persisted via `QSettings`
  ([src/settings.cpp](src/settings.cpp)) and applied live, without
  restarting the app ([src/theming.cpp](src/theming.cpp),
  [src/tabs/settings_tab.cpp](src/tabs/settings_tab.cpp)). The "System"
  option follows the OS's actual live setting via Qt 6.5's
  `QStyleHints::colorSchemeChanged` signal — see
  [IMPLEMENTATION.md](IMPLEMENTATION.md) for how this compares to the
  imgui contester's 2-second polling loop.

## GUI testing

Qt ships its own first-class unit test framework,
[QtTest](https://doc.qt.io/qt-6/qttest-index.html) (the `Qt6::Test`
module), used here the same way the imgui contester used doctest: every
piece of non-trivial logic is factored out into small, pure, standalone
functions with **no `QWidget`/GUI dependency**, and unit tested directly:

- clock formatting and timezone conversion
  ([src/tabs/wallclock.cpp](src/tabs/wallclock.cpp) /
  [tests/tst_wallclock.cpp](tests/tst_wallclock.cpp))
- elapsed-time formatting, lap-report export text, and the export
  filename/round-trip
  ([src/tabs/stopwatch.cpp](src/tabs/stopwatch.cpp) /
  [tests/tst_stopwatch.cpp](tests/tst_stopwatch.cpp))
- arc angle/point geometry ([src/tabs/synctime.cpp](src/tabs/synctime.cpp) /
  [tests/tst_synctime.cpp](tests/tst_synctime.cpp))
- settings (de)serialization through a real (temp-file-backed) `QSettings`
  instance, and the curated timezone list
  ([src/settings.cpp](src/settings.cpp) /
  [tests/tst_settings.cpp](tests/tst_settings.cpp))

These pure functions live in the `qt_clock_core` static library (see
[CMakeLists.txt](CMakeLists.txt)), which links only against `Qt6::Core` —
kept deliberately free of `Qt6::Widgets`, so the test binaries don't need a
display or event loop to run. The widget-drawing code (`src/tabs/*_tab.cpp`,
`src/mainwindow.cpp`, `src/theming.cpp`) calls into these functions but is
itself only exercised by manually running the compiled app — see
[IMPLEMENTATION.md](IMPLEMENTATION.md) for what that check did and didn't
cover.

Beyond this project's own hand-rolled unit tests, Qt Widgets also has a
mature story for actual **widget-level** GUI testing that this demo didn't
exercise: `QTest::mouseClick()`/`keyClick()` for driving real widgets
in-process, and [Squish](https://www.qt.io/squish) (commercial) for
full black-box UI test automation — see the framework comparison in
[../OVERVIEW.md](../OVERVIEW.md) for how this stacks up against the
contesters.

## Using it from Python

This demonstrator itself is plain C++; it doesn't use Python. Qt Widgets
*does* have mature, official-adjacent Python bindings —
[PySide6](https://doc.qt.io/qtforpython-6/) (Qt Company-maintained) and
[PyQt6](https://riverbankcomputing.com/software/pyqt/) (Riverbank
Computing) — either of which could build the same UI from Python, but
building the same demo through them wasn't attempted here.
