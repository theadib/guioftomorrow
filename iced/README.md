# Iced clock demonstrator

The Iced contester for the [GUI of Tomorrow](../README.md) comparison: a
tabbed clock app (wallclock, stopwatch, synctime, settings) built with
[Iced](https://iced.rs)/Rust, a self-rendering (no webview, no native
toolkit) GUI library following The Elm Architecture. See
[../OVERVIEW.md](../OVERVIEW.md#iced-rust) for the framework write-up and
[IMPLEMENTATION.md](IMPLEMENTATION.md) for the completeness checklist.

## Installing the development environment

1. Install Rust (this project was built and tested against **Rust/Cargo
   1.94.1**, but any toolchain satisfying Iced's own `rust-version = "1.88"`
   works): via [rustup](https://rustup.rs) or your OS package manager.
2. Install the platform windowing/graphics headers `iced`'s default features
   (`wgpu` + `x11` + `wayland`) link against:
   - **Linux desktop**: X11 and Wayland client headers, plus a Vulkan loader
     for GPU-accelerated rendering — on Debian/Ubuntu:
     ```sh
     sudo apt install build-essential pkg-config libx11-dev libxrandr-dev \
       libxi-dev libxcursor-dev libxkbcommon-dev libwayland-dev \
       libvulkan1 mesa-vulkan-drivers
     ```
     (this dev environment already had all of the above installed). None of
     these end up dynamically linked into the built binary though — `ldd` on
     the release build shows only `libc`/`libm`/`libgcc_s`; `winit`/`wgpu`
     `dlopen` the X11/Wayland/Vulkan client libraries at *runtime*, falling
     back to Iced's CPU renderer (`tiny-skia`) if no GPU/Vulkan is found.
   - **Windows desktop**: no extra system packages — Visual Studio's
     "Desktop development with C++" workload provides the linker; rendering
     uses DirectX/Vulkan via `wgpu` with an automatic software fallback.
   - **macOS**: Xcode command line tools (`xcode-select --install`);
     rendering uses Metal via `wgpu`.
3. From this directory, fetch dependencies and build:
   ```sh
   cargo build
   ```

## Building, running and debugging

```sh
cargo run              # debug build, opens the app window
cargo run --release    # optimized build
```

For interactive debugging (breakpoints, variable inspection), open this
folder in VS Code (rust-analyzer + CodeLLDB extensions) or any IDE with Rust
support, and run/debug the `iced_clock` binary target; or drive it directly
with `rust-gdb`/`rust-lldb ./target/debug/iced_clock`.

Static analysis and the test suite:

```sh
cargo clippy --all-targets
cargo test
cargo fmt --check
```

## Creating the deliverable application package

```sh
cargo build --release   # -> target/release/iced_clock (~25 MB on Linux)
```

This produces a single self-contained executable — Iced renders itself
(`wgpu`/`tiny-skia`) rather than embedding a webview or linking a native
widget toolkit, and the platform windowing libraries are `dlopen`'d at
runtime — so the binary can be copied and distributed directly. There is no
official bundler equivalent to `dx bundle`/`flutter build`; wrapping it in a
double-clickable `.app`/`.deb`/`.msi` would be a manual packaging step, not
exercised here.

## What it demonstrates

- **Wallclock** — current date/time in a user-selectable IANA timezone
  ([src/tabs/wallclock.rs](src/tabs/wallclock.rs)), computed with
  `chrono`/`chrono-tz` and persisted across restarts; the `:` separators
  blink once a second by swapping the text's `Color` between the theme's
  default and a dimmed one on each 33ms tick.
- **Stopwatch** — start/stop/reset/lap using `std::time::Instant` (monotonic,
  unaffected by system clock adjustments), with recorded laps exportable as
  a plain-text report written straight to the OS data directory
  ([src/tabs/stopwatch.rs](src/tabs/stopwatch.rs)).
- **Synctime** — a `canvas::Program` draws two stroked arcs (one
  rotation/second, one rotation/minute) with `canvas::Path`/`Arc`, stacked
  (`widget::stack!`) under a live `HH:MM:SS` text readout, redrawn ~30 times
  a second via the app's `Subscription` ([src/tabs/synctime.rs](src/tabs/synctime.rs)).
- **Settings** — GUI theme (system/light/dark — system polls the OS via the
  `dark-light` crate every 2s, only while "System" is selected), contrast
  (normal/high, via a custom `iced::Theme::custom` high-contrast `Palette`),
  and a 50%–200% UI scale (applied through Iced's native
  `Application::scale_factor`, so it scales every widget the same way the OS
  DPI scale factor would) — all persisted to a JSON file in the OS config
  directory ([src/settings.rs](src/settings.rs)) and applied live, without
  restarting the app.

## GUI testing

Iced has no official widget-level or end-to-end UI testing framework yet
(see [../OVERVIEW.md](../OVERVIEW.md#iced-rust)). Instead, every piece of
non-trivial logic is factored out into small, pure, standalone functions and
unit tested directly with `#[cfg(test)]` (the idiomatic Rust convention,
colocated in each module rather than a separate `test/` directory):

- clock formatting ([src/tabs/wallclock.rs](src/tabs/wallclock.rs))
- elapsed-time formatting, lap-report export text, and the `update` state
  machine ([src/tabs/stopwatch.rs](src/tabs/stopwatch.rs))
- arc angle math and the clock-face-to-canvas angle convention
  ([src/tabs/synctime.rs](src/tabs/synctime.rs))
- settings (de)serialization and the curated timezone list
  ([src/settings.rs](src/settings.rs))

Run them with `cargo test`. See [IMPLEMENTATION.md](IMPLEMENTATION.md) for
what was, and wasn't, verified by actually launching the compiled GUI
binary.

## Using it from Python

None — Iced is a Rust-only framework with no official Python bindings (see
the comparison notes in [../OVERVIEW.md](../OVERVIEW.md#iced-rust)).
