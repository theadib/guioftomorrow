# Tauri clock demonstrator

The Tauri contester for the [GUI of Tomorrow](../README.md) comparison: a
tabbed clock app (wallclock, stopwatch, synctime, settings) with a plain
HTML/CSS/JS front end running in the OS's native webview, and a small Rust
back end for settings persistence and file export. See
[../OVERVIEW.md](../OVERVIEW.md#tauri-rust) for the framework write-up and
[IMPLEMENTATION.md](IMPLEMENTATION.md) for the completeness checklist.

## Installing the development environment

1. Install Rust (this project was built and tested against **Rust/Cargo
   1.94.1**, but any reasonably recent stable toolchain works): via
   [rustup](https://rustup.rs) or your OS package manager.
2. Install the platform webview/toolkit headers Tauri links against:
   - **Linux desktop**: GTK 3, WebKitGTK and libsoup development headers —
     on Debian/Ubuntu:
     ```sh
     sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libsoup-3.0-dev \
       libjavascriptcoregtk-4.1-dev libssl-dev pkg-config build-essential
     ```
     (Older Debian/Ubuntu releases only ship `libwebkit2gtk-4.0-dev`/
     `libsoup2.4-dev` — Tauri 2 supports either webkit2gtk generation.)
   - **Windows desktop**: no extra system packages — the OS-provided
     WebView2 runtime (preinstalled on current Windows) is used. Visual
     Studio's "Desktop development with C++" workload provides the linker.
   - **macOS**: Xcode command line tools (`xcode-select --install`); the
     system WKWebView is used, no extra packages.
3. From this directory's [src-tauri/](src-tauri/) folder, fetch
   dependencies and build:
   ```sh
   cd src-tauri
   cargo build
   ```

**No Node.js/npm is required.** The front end
([src/index.html](src/index.html), [src/styles.css](src/styles.css),
[src/app.js](src/app.js), [src/clock-logic.js](src/clock-logic.js)) is
plain, unbundled HTML/CSS/JS served directly from [src/](src/) — there's no
build step, framework, or `package.json`. `tauri.conf.json`'s
`build.frontendDist` just points at that folder as-is.

The `cargo tauri` CLI (`cargo install tauri-cli`) is **not required either**
for this project: since the front end has no build step, plain
`cargo build`/`cargo run` from [src-tauri/](src-tauri/) work exactly like
any other Rust binary. The CLI is only useful here as a convenience for
`cargo tauri dev` (auto-reload on file changes) or `cargo tauri build`
(produces `.deb`/`.AppImage`/`.msi`/`.dmg` installers instead of a bare
binary) — see the packaging section below.

## Building, running and debugging

From [src-tauri/](src-tauri/):

```sh
cargo run              # debug build, opens the app window
cargo run --release    # optimized build
```

For interactive debugging (breakpoints, variable inspection) of the Rust
side, open this folder in VS Code (rust-analyzer + CodeLLDB extensions) or
any IDE with Rust support, and run/debug the `gui_of_tomorrow_tauri` binary
target; or drive it directly with
`rust-gdb`/`rust-lldb ./target/debug/gui_of_tomorrow_tauri`. For the
front-end side, right-click the running window and choose "Inspect Element"
to open WebKit's own developer tools (console, DOM inspector, debugger).

Static analysis and the test suites:

```sh
cd src-tauri
cargo clippy --all-targets
cargo test
cargo fmt --check
```

```sh
# from this directory (not src-tauri/) — see "GUI testing" below
gjs test/clock-logic.test.js
```

## Creating the deliverable application package

Without the `tauri-cli`:

```sh
cd src-tauri
cargo build --release   # -> target/release/gui_of_tomorrow_tauri
```

This produces a single self-contained executable (it links the OS's system
webview rather than bundling one) that can be copied and distributed
directly, with [src/](src/) alongside it — no separate installer step for
this demonstrator.

For a real double-clickable package (`.deb`, `.rpm`, `.AppImage` on Linux;
`.msi`/`.exe` on Windows; `.dmg`/`.app` on macOS), install the CLI once
(`cargo install tauri-cli --locked`) and run:

```sh
cargo tauri build
```

from this directory. That step wasn't exercised here (no network access to
install the CLI's own large dependency tree was budgeted for this
demonstrator beyond what building the app itself needed) — see
[IMPLEMENTATION.md](IMPLEMENTATION.md) for what was and wasn't verified.

## What it demonstrates

- **Wallclock** — current date/time in a user-selectable IANA timezone
  ([src/app.js](src/app.js), [src/clock-logic.js](src/clock-logic.js)),
  computed with the browser's own `Intl.DateTimeFormat` (no date/timezone
  library needed on either side) and persisted across restarts via a Rust
  `save_settings` command; the `:` separators blink once a second as a
  small clock-face animation.
- **Stopwatch** — start/stop/reset/lap using `performance.now()` (monotonic,
  unaffected by system clock adjustments), with recorded laps exportable as
  a plain-text report: the frontend formats the report and hands it to the
  Rust `export_stopwatch` command, which sanitizes the filename and writes
  it straight to the OS data directory
  ([src/app.js](src/app.js), [src-tauri/src/commands.rs](src-tauri/src/commands.rs)).
- **Synctime** — inline SVG `<path>` arcs (one rotation/second, one
  rotation/minute) around a live `HH:MM:SS` readout, redrawn ~30 times a
  second via `setInterval` ([src/app.js](src/app.js)).
- **Settings** — GUI theme (system/light/dark), contrast (normal/high, via
  CSS custom properties), and a 50%–200% UI scale (applied as the root
  element's `font-size`, cascading to every `em`-based measurement) — all
  persisted to a JSON file in the OS config directory via Rust commands
  ([src-tauri/src/settings.rs](src-tauri/src/settings.rs)) and applied
  live, without restarting the app. **System theme changes are pushed, not
  polled**: Tauri's own `getCurrentWindow().theme()`/`onThemeChanged` JS
  API (used directly via `window.__TAURI__`, no `@tauri-apps/api` package
  needed thanks to `app.withGlobalTauri` in
  [src-tauri/tauri.conf.json](src-tauri/tauri.conf.json)) delivers OS theme
  changes as an event the instant they happen, unlike the Dioxus/Slint
  contesters' 2-second `dark-light`-crate poll.

## GUI testing

Tauri ships `tauri-driver`, a WebDriver server that can script the actual
desktop window end-to-end (see the framework notes in
[../OVERVIEW.md](../OVERVIEW.md#tauri-rust)); wiring that up — plus a
WebDriver-capable browser driver — was more automation infrastructure than
this demonstrator's environment had available, so it wasn't exercised here.

Instead, every piece of non-trivial *logic* is factored out into small,
pure, DOM-free functions in [src/clock-logic.js](src/clock-logic.js)
(elapsed-time formatting, lap-report export text, arc angle/path geometry,
timezone-aware wallclock formatting) and unit tested in
[test/clock-logic.test.js](test/clock-logic.test.js) — run with:

```sh
gjs test/clock-logic.test.js
```

[gjs](https://gjs.guide/) (GNOME JavaScript) is used instead of a
Node/Jest/Vitest setup so the test suite needs **no npm dependency
install at all**: it's the same SpiderMonkey-based interpreter that ships
with any GNOME/GTK desktop (the same stack this app's webview already
depends on), with full `Intl`/timezone support. All 18 assertions pass; see
[IMPLEMENTATION.md](IMPLEMENTATION.md) for the corresponding Rust-side unit
tests (settings persistence, filename sanitization) run via `cargo test`.

## Using it from Python

None for building the Tauri app itself — see the comparison notes in
[../OVERVIEW.md](../OVERVIEW.md#tauri-rust). A Python process could only be
wired in as a separate sidecar/backend the Rust side shells out to or talks
to over IPC, which this demonstrator doesn't do.
