# Overview

This document collects, for each GUI framework demonstrated in this repo, a
brief comparison: pros, cons, implementation complexity, support for MVC or
similar architectural patterns, GUI testing support, theming support (dark,
light, system), support for dockable (sub)windows, Python usability, lines
of code for the implementation, canvas drawing, licensing, maintenance,
restyling, 3D scene support, and guidance on when to choose it and when not
to. Each project's own `README.md` covers environment setup,
build/run/debug, and packaging; each project's own `IMPLEMENTATION.md`
tracks its completeness checklist and side notes. This file is the
cross-project summary, updated as each contester's demonstrator is
implemented.

See [README.md](README.md) for the demonstrator app (a tabbed clock app with
wallclock, stopwatch, and synctime tabs) and the list of contesters. Qt,
Slint's C# binding (which uses Qt on Windows) and plain C# were considered
and deliberately left out of the contester list; Qt is nonetheless
implemented in [qt/](qt/) as a **reference, not a contester** — a baseline
to measure the actual contesters against — and is included in the
comparison below for that reason.

## Status

All demonstrators — the 7 hand-selected contesters plus the Qt reference —
have now been implemented.

| Project        | Language | Role       | Demonstrator |
|----------------|----------|------------|--------------|
| Flutter        | Dart     | Contester  | [Done](flutter/) |
| Tauri          | Rust     | Contester  | [Done](tauri/) |
| Slint          | Rust     | Contester  | [Done](slint/) |
| Dioxus         | Rust     | Contester  | [Done](dioxus/) |
| Iced           | Rust     | Contester  | [Done](iced/) |
| egui           | Rust     | Contester  | [Done](egui/) |
| imgui          | C++      | Contester  | [Done](imgui/) |
| Qt Widgets     | C++      | Reference  | [Done](qt/) |

## Comparison at a glance

A condensed view of the "ability" metrics and lines of code discussed in
each framework's own section below. **LOC** counts non-blank,
non-comment-only lines in each project's own implementation code (UI +
app logic; excludes tests, generated/build output and lockfiles) — see the
per-framework sections for the file/language breakdown.

| Project    | Complexity  | MVC / pattern           | GUI tests         | Theming (D/L/S)        | Dockable windows        | Python              | LOC   |
|------------|-------------|--------------------------|-------------------|-------------------------|--------------------------|----------------------|------:|
| Flutter    | Medium      | Via packages (MVVM/BLoC) | Strong, official  | Built-in, incl. system  | No (3rd-party package)  | Yes (Flet)           |   682 |
| Tauri      | Medium-high | Depends on frontend      | Good (web e2e + `tauri-driver`) | DIY (CSS + APIs)       | No (needs JS lib)        | No (sidecar only)    |   779 |
| Slint      | Medium      | Yes (MVVM-like)          | Limited, maturing | Built-in, incl. system  | No (multi-window only)  | Yes (official)       |  1064 |
| Dioxus     | Medium-high | Component/hooks (MVVM-ish)| Basic            | DIY (CSS)               | No (needs JS lib)        | No                   |   852 |
| Iced       | High        | Yes (Elm/MVU)            | Limited (unit only)| Built-in, no auto-system| No (multi-window only)  | No                   |   775 |
| egui       | Low         | No (immediate mode)      | Emerging (`egui_kittest`) | Built-in, limited system| Yes (`egui_dock` crate) | No (unofficial only) |   667 |
| imgui      | Medium      | No (immediate mode)      | None built-in (community `imgui_test_engine`) | DIY (manual style colors) | Yes, built-in (docking branch) | Yes (`pyimgui`) |   657 |
| Qt Widgets *(reference)* | Medium-high | Supported (Model/View classes) | Strong, official (`QTest`) | Built-in, incl. push-based system | Yes, built-in (`QDockWidget`) | Yes (`PyQt6`/`PySide6`) | 785 |

The additional comparison points below describe the practical route for each
framework rather than claiming that every capability is built in:

| Project | Canvas drawing | License(s) to evaluate | Maintenance | Restyling | 3D scenes |
|---------|----------------|-------------------------|-------------|-----------|-----------|
| Flutter | `CustomPainter`, plus packages such as `flutter_gl` | BSD-3-Clause | Active, backed by Google | `ThemeData`, widget themes, or per-widget properties | Package/plugin or embedded native/web rendering; no core 3D scene graph |
| Tauri | HTML `<canvas>`, WebGL, or a frontend canvas library | MIT/Apache-2.0 for Tauri; frontend licenses vary | Active, backed by the Tauri Foundation/community | CSS, frontend components, and platform APIs | WebGL/WebGPU libraries such as Three.js or Babylon.js |
| Slint | `Path`/`Image` primitives and custom widgets | LGPLv3 or commercial license | Active, maintained by Slint | Palette, globals, and component properties | No built-in scene graph; integrate a native renderer or separate surface |
| Dioxus | HTML `<canvas>`/WebGL through the webview renderer | MIT | Active, community-led | CSS and component properties | WebGL/WebGPU libraries such as Three.js |
| Iced | `Canvas` widget and custom `Program` drawing | MIT | Active, community-led | Themes, styles, and custom widget styling | Integrate with `wgpu`; no built-in 3D scene graph |
| egui | `Painter`, custom widgets, and plots | MIT/Apache-2.0 | Active, community-led | `Visuals`, style fields, and per-widget overrides | Integrate with the renderer/backend; no built-in 3D scene graph |
| imgui | `ImDrawList` and renderer-specific primitives | MIT | Active, maintained by the Dear ImGui project | `ImGuiStyle` and per-item style stacks | Use the host engine/renderer; Dear ImGui is not a 3D engine |
| Qt Widgets *(reference)* | `QPainter`, `QGraphicsView`, or custom widgets | LGPLv3/GPLv3 or commercial license | Active, maintained by Qt Group | Stylesheets, `QStyle`, palettes, and widget properties | Qt Quick 3D/Qt 3D integration, usually alongside Widgets |

## Flutter (Dart)

- **Pros:** One codebase targets Windows, Linux, macOS, Android, iOS and web;
  mature, widget-rich framework with excellent tooling, hot reload and
  documentation; consistent custom rendering (Impeller/Skia) gives
  pixel-identical UI across platforms; rich animation support.
- **Cons:** Dart is essentially only used for Flutter, so it's an extra
  language for the team; larger binary/runtime footprint; UI is custom-drawn
  rather than native controls, so it never looks 100% platform-native;
  desktop (especially Linux) packaging is less polished than mobile.
- **When to use:** You want a single codebase across mobile and desktop,
  Android is a primary target, and you're fine investing in Dart for a
  large, actively maintained widget/animation ecosystem.
- **When not to use:** You need genuine native look-and-feel or minimal
  binary size, want to avoid a new language, or are targeting desktop only
  with a very small utility app.
- **Complexity:** Medium. Dart is a new language for non-Flutter teams and
  the widget-tree mental model takes some adjustment, but hot reload,
  mature docs and a huge example base flatten the curve quickly.
- **MVC/architecture:** No single pattern is enforced; the widget tree is
  the view, and state-management packages (Provider, Riverpod, BLoC) layer
  MVVM/MVI-style separation of state and business logic on top.
- **GUI tests:** Strong first-class support — `flutter_test` for widget
  tests and the `integration_test` package for full end-to-end/driver-based
  UI tests, both officially maintained.
- **Theming:** Built-in `ThemeData`/`ThemeMode` with first-class light,
  dark and `system` (follows the OS setting automatically) modes.
- **Dockable windows:** No built-in docking manager; a drag-to-dock
  panel layout needs a third-party package (e.g. `docking`), so it's
  possible but not a first-class feature.
- **Python:** yes. using Flet package
- **Lines of code:** ~682 (Dart), across 7 files.

## Tauri (Rust)

- **Pros:** UI is plain HTML/CSS/JS (any web framework) with a Rust backend;
  uses the OS's native webview instead of bundling a browser, so binaries
  are much smaller than Electron; strong security-focused architecture
  (capability-based permissions); Tauri 2 adds mobile (Android/iOS) targets.
- **Cons:** Rendering depends on the system webview (WebView2 on Windows,
  WebKitGTK on Linux), so behavior/appearance can vary across OS versions;
  UI is web tech, not native controls; requires comfort with both a
  frontend stack and Rust for the backend.
- **When to use:** You want small, secure desktop apps built with familiar
  web UI tech plus a Rust backend, and can accept relying on the OS webview.
- **When not to use:** You need guaranteed, version-independent rendering,
  want to avoid web tech entirely, or must support environments with very
  outdated/missing system webviews.
- **Complexity:** Medium-high. Two stacks have to be bridged — a web
  frontend and a Rust backend — plus the IPC `#[command]` boundary between
  them, so there's real surface area even for a small app.
- **MVC/architecture:** Not prescribed by Tauri itself — whatever pattern
  the chosen frontend framework supports (component-based, MVVM, Flux/Redux,
  etc.) applies, with the Rust side acting as a backend/service layer.
- **GUI tests:** Good — the front end is testable with standard web
  end-to-end tools, and Tauri ships `tauri-driver`, a WebDriver server for
  scripting the actual desktop window.
- **Theming:** Not built in beyond what the frontend stack provides — a
  standard `prefers-color-scheme` CSS media query plus Tauri's theme APIs
  (reading and listening for OS theme changes) let you implement light,
  dark and system-following themes yourself.
- **Dockable windows:** No built-in docking widget; the frontend can use a
  JS docking library (e.g. Golden Layout, Dockview) inside a single Tauri
  window, or Tauri's own multi-window API for separate OS windows, but
  neither is provided out of the box.
- **Python:** No official bindings for building the Tauri app itself; a
  Python process can only be used as a separate sidecar/backend the Rust
  side talks to, not as the framework's UI layer.
- **Lines of code:** ~779 (188 Rust backend + 591 HTML/CSS/JS frontend),
  across 7 files.

## Slint (Rust)

- **Pros:** Declarative `.slint` UI markup with a live-preview designer tool;
  small, fast runtime designed for embedded and resource-constrained
  targets as well as desktop; officially supports Rust, C++ **and Python**
  from the same markup; clean separation of UI design from application
  logic.
- **Cons:** Smaller community and third-party widget ecosystem than
  Flutter/Qt; dual-licensed (GPLv3 or a commercial license), which needs
  evaluating for closed-source commercial use; younger project with fewer
  real-world examples.
- **When to use:** You want a lightweight declarative toolkit that scales
  down to embedded devices, and/or you want to prototype or ship the same
  UI from Rust, C++ or Python.
- **When not to use:** You need a huge widget/plugin ecosystem, the widest
  possible community support, or want to avoid licensing considerations for
  closed-source commercial products.
- **Complexity:** Medium. The `.slint` markup language and its
  property/callback binding model are new vocabulary to learn, but the
  live-preview designer and concise syntax keep individual screens small.
- **MVC/architecture:** Naturally MVVM-like — `.slint` markup is the view,
  bound via properties/callbacks to a model/business-logic layer written in
  Rust, C++ or Python, keeping UI and logic cleanly separated.
- **GUI tests:** Still maturing — testing is mostly done by driving the
  Rust/C++/Python application logic directly; there isn't yet an official,
  mature widget/e2e testing framework comparable to Flutter's.
- **Theming:** Built-in light/dark color scheme support (`Palette`/
  `ColorScheme`) that can follow the OS setting or be switched manually.
- **Dockable windows:** No built-in docking manager; `.slint` supports
  multiple top-level `Window` components, but arranging them into a
  drag-to-dock layout would have to be built by hand.
- **Python:** Yes — official Python bindings (`pip install slint`) let you
  build and run the same `.slint` UIs directly from Python.
- **Lines of code:** ~1064 (638 Rust + 426 `.slint` markup), across 14
  files.

## Dioxus (Rust)

- **Pros:** React-like component/hooks model in Rust; one codebase can target
  web (WASM), desktop and (experimentally) mobile; hot-reload during
  development; appealing to Rust developers who like React's mental model.
- **Cons:** Younger and still evolving, especially on desktop/mobile
  polish; smaller ecosystem than Flutter; desktop rendering typically goes
  through a webview (similar caveats to Tauri) unless using the newer,
  still-early native `Blitz` renderer; Rust-only, no Python story.
- **When to use:** A Rust team wants React-style ergonomics and a single
  codebase spanning web and desktop, and is comfortable with a fast-moving,
  still-maturing framework.
- **When not to use:** You need a production-hardened, long-established
  framework today, need Python, or need fully native, non-webview desktop
  rendering out of the box.
- **Complexity:** Medium-high. The React-like hooks model is familiar to
  web developers, but a younger ecosystem means more friction elsewhere —
  e.g. this project had to vendor `openssl-sys` to work around a
  `dioxus-desktop` build dependency (see `dioxus/IMPLEMENTATION.md`).
- **MVC/architecture:** React-like component/hooks model, so it's
  component-based rather than classic MVC; hooks and shared state serve the
  ViewModel role, similar to React/MVVM patterns.
- **GUI tests:** Basic component/unit testing utilities exist; desktop
  end-to-end UI testing is not yet as mature as Flutter's, though the web
  target can reuse standard web e2e tooling.
- **Theming:** No built-in theme system; light/dark/system theming is
  implemented via CSS (including `prefers-color-scheme`) since desktop
  rendering typically goes through a webview.
- **Dockable windows:** No built-in docking manager; since desktop
  rendering goes through a webview, a JS docking library would need to be
  layered on top, same as Tauri.
- **Python:** None. Dioxus is Rust-only with no official Python bindings.
- **Lines of code:** ~852 (631 Rust + 221 CSS), across 8 files.

## Iced (Rust)

- **Pros:** Elm-inspired Model-Update-View architecture; renders itself
  (via `wgpu`/`tiny-skia`) instead of relying on a webview or native
  toolkit, giving consistent, predictable rendering across platforms;
  functional, type-safe style makes state changes easy to reason about;
  good performance.
- **Cons:** Smaller built-in widget set than Flutter/Qt; native look,
  accessibility and platform integration are still maturing; Rust-only;
  the Elm-style architecture has a learning curve for teams unfamiliar
  with it.
- **When to use:** You want a pure-Rust desktop app with no webview
  dependency, predictable custom rendering, and you're comfortable with (or
  want) a functional, message-passing UI architecture.
- **When not to use:** You need a large library of ready-made widgets, a
  native platform look, Python support, or a gentler learning curve.
- **Complexity:** High. The Elm-style Model-Update-View split is a real
  paradigm shift if the team hasn't used it before, and every interaction
  needs an explicit `Message` variant and `update` arm, which adds
  boilerplate as the app grows.
- **MVC/architecture:** Follows The Elm Architecture (Model-Update-View), a
  distinct but related pattern to MVC — a single immutable model, pure
  `update` functions for state transitions, and a `view` function that
  renders from the model.
- **GUI tests:** Limited — the pure `update` functions are easy to unit
  test, but there is no official widget-level or end-to-end UI testing
  framework yet; screenshot testing is possible only via community effort.
- **Theming:** Built-in `Theme` enum with `Light`/`Dark`/custom themes;
  following the OS setting automatically is not built in and needs a
  community crate or manual detection.
- **Dockable windows:** No built-in docking widget; multi-window support
  exists (`iced::multi_window`) for separate OS windows, but a drag-to-dock
  panel layout would need to be built as a custom widget.
- **Python:** None. Iced is Rust-only.
- **Lines of code:** ~775 (Rust), across 7 files.

## egui (Rust)

- **Pros:** Immediate-mode GUI with a very small, simple API; extremely
  fast to prototype; pure-Rust rendering that compiles to native apps and
  the web (WASM) with the same code; well suited to tools, debug overlays
  and data-heavy UIs (e.g. `egui_plot`).
- **Cons:** Immediate-mode style is less suited to deeply nested, highly
  branded/stylable "consumer app" UIs; theming/styling is more limited than
  retained-mode frameworks; not aimed at pixel-perfect native look;
  managing complex persistent layout state can get verbose.
- **When to use:** Rapid prototyping, internal tools, debug/dev overlays,
  or data-visualization-heavy apps where minimal boilerplate and a
  Rust-only stack matter more than a highly custom look.
- **When not to use:** You need a richly branded, highly custom consumer
  UI, strong accessibility support, or Python.
- **Complexity:** Low. The immediate-mode API is tiny and self-explanatory
  — a widget is a function call — so a working tabbed UI comes together
  quickly, though this project did have to pin an older `egui`/`eframe`
  version to match the available Rust toolchain (see `egui/IMPLEMENTATION.md`).
- **MVC/architecture:** Immediate-mode by design, so UI and logic are not
  separated — the whole UI is rebuilt from application state every frame in
  a single function, which is the opposite of a classic MVC split.
- **GUI tests:** An official `egui_kittest` crate provides snapshot and
  interaction testing, but the story is still relatively new compared to
  established frameworks.
- **Theming:** Built-in light/dark `Visuals` presets switchable at runtime;
  `eframe` can pick up the OS light/dark setting on startup, but live
  system-theme-change following is limited.
- **Dockable windows:** Yes, via the popular `egui_dock` ecosystem crate —
  not part of egui core, but a widely-used, drag-to-dock tab/panel layout
  is readily available.
- **Python:** No official bindings; only unofficial/experimental PyO3
  wrappers exist, not a first-class option.
- **Lines of code:** ~667 (Rust), across 7 files.

## imgui (C++, Dear ImGui)

- **Pros:** Battle-tested immediate-mode GUI widely used in game
  development and tooling; extremely fast and lightweight; huge number of
  rendering/windowing backends (DirectX, OpenGL, Vulkan, SDL, GLFW, …);
  large existing ecosystem of extensions; ideal for debug UIs and
  real-time overlays.
- **Cons:** Not designed for polished, branded "end-user application" UIs —
  no built-in theming system, accessibility, or native look; C++
  integration is manual (you must pick and wire up your own
  renderer/windowing backend); no high-level app framework (window
  creation, event loop, etc. are your responsibility).
- **When to use:** Building developer tools, in-game debug UI, or
  real-time visualization overlays, especially when you already have a
  C++/game-engine style render loop to hook into.
- **When not to use:** Building a polished, accessible, native-looking
  consumer desktop app, or wanting easy packaging/distribution without
  hand-rolled build setup.
- **Complexity:** Medium. Per-widget calls are simple, but there's no
  application framework at all — window creation, the render loop and the
  rendering backend (OpenGL/Vulkan/DirectX + GLFW/SDL) all have to be
  wired up by hand before any UI code runs.
- **MVC/architecture:** Immediate-mode like egui — no enforced separation
  between view and logic; the app's render-loop function both holds and
  displays state each frame, so any MVC-style layering is left to the
  integrator.
- **GUI tests:** No built-in automated testing framework; testing is
  largely manual/visual, though the community `imgui_test_engine` project
  (by Dear ImGui's author) adds scripted UI test capability.
- **Theming:** No built-in dark/light/system switching; theming is done
  manually by setting `ImGuiStyle` colors yourself (the built-in
  `StyleColorsDark`/`StyleColorsLight` presets are a starting point, not an
  OS-aware system).
- **Dockable windows:** Yes, first-class and built in — Dear ImGui's
  docking branch (merged into upstream `master` since 2023) lets users
  drag windows together into tabbed/split layouts, one of its signature
  features.
- **Python:** Yes — bindings such as `pyimgui`/`imgui-bundle` let you build
  Dear ImGui UIs directly from Python.
- **Lines of code:** ~657 (575 C++ + 82 headers), across 17 files.

## Qt Widgets (C++) — reference, not a contester

Qt is deliberately **not** one of the contesters (see the root
[README.md](README.md#contester)) — it's included here purely as a
baseline, since it's the "old guard" native-widget toolkit the demonstrator
app is meant to move on from. The [qt/](qt/) project implements the exact
same demonstrator so it can be measured against the actual contesters below.

- **Pros:** Extremely mature (30+ years) and comprehensive; genuinely
  native-feeling widgets on every desktop platform; huge ecosystem, tooling
  (Qt Creator, Qt Designer) and commercial support; best-in-class docking
  (`QDockWidget`) and Model/View framework.
- **Cons:** C++ build tooling (CMake + `moc`) adds friction; verbose
  signal/slot boilerplate compared to modern declarative frameworks;
  dual-licensed (LGPL/commercial) with commercial-use nuances; no official
  mobile/web story as strong as Flutter's.
- **When to use:** You want the most mature, native-feeling desktop toolkit
  available, need best-in-class docking or a mature Model/View data
  framework, and C++ (or Python via PyQt6/PySide6) is an acceptable
  language.
- **When not to use:** You're deliberately avoiding Qt (as this repo is —
  see the root README), need first-class mobile targets, or want a
  lighter-weight/declarative modern stack.
- **Complexity:** Medium-high. Qt's APIs are mature and well documented,
  but verbose C++ signal/slot wiring and the `moc` build step add ceremony
  a newer framework would avoid.
- **MVC/architecture:** Not enforced, but well supported — Qt's
  Model/View classes (`QAbstractItemModel` and friends) are a genuine,
  mature MV(C) implementation, on top of which widgets are usually wired
  together via signals/slots rather than a formal controller layer.
- **GUI tests:** Strong — `QTest`, a mature, official unit/widget testing
  module, ships with Qt itself.
- **Theming:** Built-in via `QPalette`, plus (Qt 6.5+) a push-based
  `QGuiApplication::styleHints()->colorScheme()` API with a
  `colorSchemeChanged` signal for following OS light/dark changes live,
  with no polling loop needed (see `qt/IMPLEMENTATION.md`).
- **Dockable windows:** Yes, first-class and built in — `QMainWindow` +
  `QDockWidget` is one of Qt's classic, best-known features, used by
  countless IDEs and professional tools.
- **Python:** Yes — mature, official-adjacent bindings (`PyQt6`/`PySide6`),
  among the most established Python GUI options available.
- **Lines of code:** ~785 (595 C++ + 190 headers), across 21 files.
