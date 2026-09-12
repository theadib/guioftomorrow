# Overview

This document collects, for each GUI framework demonstrated in this repo, a
brief comparison: pros, cons, GUI testing support, Python usability, and
guidance on when to choose it and when not to. Each project's own
`README.md` covers environment setup, build/run/debug, and packaging; each
project's own `IMPLEMENTATION.md` tracks its completeness checklist and side
notes. This file is the cross-project summary, updated as each contester's
demonstrator is implemented.

See [README.md](README.md) for the demonstrator app (a tabbed clock app with
wallclock, stopwatch, and synctime tabs) and the list of contesters.

## Status

Framework comparison notes below are filled in ahead of implementation, based
on each framework's general capabilities. The table tracks whether the
demonstrator app itself has actually been built for each contester.

| Contester      | Language | Demonstrator |
|----------------|----------|--------------|
| Flutter        | Dart     | Pending      |
| Tauri          | Rust     | Pending      |
| Slint          | Rust     | Pending      |
| Dioxus         | Rust     | Pending      |
| Iced           | Rust     | Pending      |
| egui           | Rust     | Pending      |
| imgui          | C++      | Pending      |

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
- **GUI tests:** Strong first-class support — `flutter_test` for widget
  tests and the `integration_test` package for full end-to-end/driver-based
  UI tests, both officially maintained.
- **Python:** yes. using Flet package

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
- **GUI tests:** Good — the front end is testable with standard web
  end-to-end tools, and Tauri ships `tauri-driver`, a WebDriver server for
  scripting the actual desktop window.
- **Python:** No official bindings for building the Tauri app itself; a
  Python process can only be used as a separate sidecar/backend the Rust
  side talks to, not as the framework's UI layer.

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
- **GUI tests:** Still maturing — testing is mostly done by driving the
  Rust/C++/Python application logic directly; there isn't yet an official,
  mature widget/e2e testing framework comparable to Flutter's.
- **Python:** Yes — official Python bindings (`pip install slint`) let you
  build and run the same `.slint` UIs directly from Python.

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
- **GUI tests:** Basic component/unit testing utilities exist; desktop
  end-to-end UI testing is not yet as mature as Flutter's, though the web
  target can reuse standard web e2e tooling.
- **Python:** None. Dioxus is Rust-only with no official Python bindings.

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
- **GUI tests:** Limited — the pure `update` functions are easy to unit
  test, but there is no official widget-level or end-to-end UI testing
  framework yet; screenshot testing is possible only via community effort.
- **Python:** None. Iced is Rust-only.

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
- **GUI tests:** An official `egui_kittest` crate provides snapshot and
  interaction testing, but the story is still relatively new compared to
  established frameworks.
- **Python:** No official bindings; only unofficial/experimental PyO3
  wrappers exist, not a first-class option.

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
- **GUI tests:** No built-in automated testing framework; testing is
  largely manual/visual, though the community `imgui_test_engine` project
  (by Dear ImGui's author) adds scripted UI test capability.
- **Python:** Yes — bindings such as `pyimgui`/`imgui-bundle` let you build
  Dear ImGui UIs directly from Python.
