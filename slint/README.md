# Slint clock demonstrator

The Slint contester for the [GUI of Tomorrow](../README.md) comparison: a
tabbed clock app (wallclock, stopwatch, synctime, settings) built with
[Slint](https://slint.dev)/Rust, a declarative `.slint`-markup UI toolkit
that renders itself (no webview, no native toolkit) and also ships official
C++ and Python bindings. See [../OVERVIEW.md](../OVERVIEW.md#slint-rust) for
the framework write-up and [IMPLEMENTATION.md](IMPLEMENTATION.md) for the
completeness checklist.

## Installing the development environment

1. Install Rust (this project was built and tested against **Rust/Cargo
   1.94.1**, but any toolchain satisfying Slint 1.18's own
   `rust-version = "1.92"` works): via [rustup](https://rustup.rs) or your
   OS package manager.
2. Install the platform headers Slint's `winit`+`femtovg` backend (this
   project's default; see the note on styles below) links against:
   - **Linux desktop**: `fontconfig`/`freetype` development headers are
     linked at build time (this dev environment already had them); X11,
     Wayland and OpenGL/EGL client libraries are `dlopen`'d at *runtime* by
     `winit`, not linked — `ldd` on the release build shows only
     `libfontconfig`/`libfreetype`/`libc`/`libm`/`libgcc_s` plus
     `libfreetype`'s own font-format dependencies (`libpng`, `libz`,
     `libbz2`, `libbrotli*`, `libexpat`). On Debian/Ubuntu:
     ```sh
     sudo apt install build-essential pkg-config libfontconfig1-dev \
       libx11-dev libxrandr-dev libxi-dev libxcursor-dev libxkbcommon-dev \
       libwayland-dev
     ```
   - **Windows desktop**: no extra system packages — Visual Studio's
     "Desktop development with C++" workload provides the linker.
   - **macOS**: Xcode command line tools (`xcode-select --install`).
3. From this directory, fetch dependencies and build:
   ```sh
   cargo build
   ```

No Qt install is required or used: `slint`'s `backend-qt` feature (which
would give the app native Qt widgets) is not part of its `default` feature
set, so a plain `cargo build`/`cargo add slint` never links or requires Qt
— matching this repo's decision to leave the Qt-on-Windows Slint backend
out of the contester list (see the [root README](../README.md#contester)).

## Building, running and debugging

```sh
cargo run              # debug build, opens the app window
cargo run --release    # optimized build
```

For interactive debugging (breakpoints, variable inspection), open this
folder in VS Code (rust-analyzer + CodeLLDB extensions, plus the official
"Slint" extension for live `.slint` editing/preview) or any IDE with Rust
support, and run/debug the `slint_clock` binary target; or drive it
directly with `rust-gdb`/`rust-lldb ./target/debug/slint_clock`. Slint also
ships a standalone live-preview tool for iterating on `.slint` files
without a full Rust rebuild: `cargo install slint-viewer`, then
`slint-viewer ui/wallclock.slint` (previews a single component in
isolation; the tab-switching/timers only run inside the real app).

Static analysis and the test suite:

```sh
cargo clippy --all-targets
cargo test
cargo fmt --check
```

## Creating the deliverable application package

```sh
cargo build --release   # -> target/release/slint_clock (~30 MB on Linux)
```

This produces a single self-contained executable — Slint's `femtovg`
renderer draws its own widgets rather than embedding a webview or a native
toolkit, and the platform windowing libraries are `dlopen`'d at runtime —
so the binary can be copied and distributed directly (see the `ldd` output
above: only `fontconfig`/`freetype` and their own dependencies are actually
linked). There is no official bundler equivalent to `dx bundle`/
`flutter build`; wrapping it in a double-clickable `.app`/`.deb`/`.msi`
would be a manual packaging step, not exercised here.

## What it demonstrates

- **Wallclock** — current date/time in a user-selectable IANA timezone
  ([src/tabs/wallclock.rs](src/tabs/wallclock.rs),
  [ui/wallclock.slint](ui/wallclock.slint)), computed with `chrono`/
  `chrono-tz` and persisted across restarts; the `:` separators blink once
  a second by swapping the text's color between the theme's foreground and
  a dimmed "muted" tone on each 33ms tick.
- **Stopwatch** — start/stop/reset/lap using `std::time::Instant`
  (monotonic, unaffected by system clock adjustments), with recorded laps
  exportable as a plain-text report written straight to the OS data
  directory ([src/tabs/stopwatch.rs](src/tabs/stopwatch.rs),
  [ui/stopwatch.slint](ui/stopwatch.slint)).
- **Synctime** — a `Path` element per arc, built from declarative
  `MoveTo`/`ArcTo` sub-elements whose endpoint coordinates are recomputed
  from pure trigonometry in Rust every 33ms tick and pushed into a
  `Synctime` global — one full rotation per second, one per minute, with a
  live `HH:MM:SS` readout layered on top
  ([src/tabs/synctime.rs](src/tabs/synctime.rs),
  [ui/synctime.slint](ui/synctime.slint)).
- **Settings** — GUI theme (system/light/dark), contrast (normal/high),
  and a 50%–200% UI scale, all persisted to a JSON file in the OS config
  directory ([src/settings.rs](src/settings.rs)) and applied live, without
  restarting the app. See [IMPLEMENTATION.md](IMPLEMENTATION.md) for how
  theme and scale are actually wired into Slint (`Palette.color-scheme`
  override plus an app-defined `AppTheme.scale` multiplier, respectively —
  neither is a single built-in "set the whole app's scale" knob).

## GUI testing

Slint's own testing story is still maturing (see
[../OVERVIEW.md](../OVERVIEW.md#slint-rust)) — there is no official
widget-level or end-to-end UI testing framework comparable to Flutter's.
Instead, every piece of non-trivial logic is factored out into small, pure,
standalone functions in plain Rust (no Slint types involved) and unit
tested directly with `#[cfg(test)]`, colocated in each module:

- clock formatting and timezone conversion
  ([src/tabs/wallclock.rs](src/tabs/wallclock.rs))
- elapsed-time formatting, lap-report export text, and the stopwatch
  `State` machine ([src/tabs/stopwatch.rs](src/tabs/stopwatch.rs))
- arc angle math and the clock-face-to-path-coordinate conversion
  ([src/tabs/synctime.rs](src/tabs/synctime.rs))
- settings (de)serialization and the curated timezone list
  ([src/settings.rs](src/settings.rs))

Run them with `cargo test`. See [IMPLEMENTATION.md](IMPLEMENTATION.md) for
what was, and wasn't, verified by actually launching the compiled GUI
binary.

## Using it from Python

Not exercised by this demonstrator (it's a Rust/`.slint` project), but
Slint is the one Rust-ecosystem contester in this repo with an official
Python story: `pip install slint` lets the same `.slint` UI files be loaded
and driven from Python instead of Rust — see
[../OVERVIEW.md](../OVERVIEW.md#slint-rust).
