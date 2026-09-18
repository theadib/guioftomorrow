# egui clock demonstrator

The egui contester for the [GUI of Tomorrow](../README.md) comparison: a
tabbed clock app (wallclock, stopwatch, synctime, settings) built with
[egui](https://www.egui.rs)/[eframe](https://docs.rs/eframe)/Rust, an
immediate-mode GUI library that renders itself (via `wgpu`) instead of
relying on a webview or native toolkit. See
[../OVERVIEW.md](../OVERVIEW.md#egui-rust) for the framework write-up and
[IMPLEMENTATION.md](IMPLEMENTATION.md) for the completeness checklist.

## Installing the development environment

1. Install Rust (this project was built and tested against **Rust/Cargo
   1.94.1**, but any toolchain satisfying egui/eframe 0.35's own
   `rust-version = "1.92"` works): via [rustup](https://rustup.rs) or your
   OS package manager.
2. Install the platform windowing/graphics headers eframe's default
   features (`wgpu` + `x11` + `wayland` + `accesskit`) link against — the
   same set the [Iced contester](../iced/README.md) needs, since both render
   via `wgpu`:
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
     back to a software rasterizer if no GPU/Vulkan is found.
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
support, and run/debug the `egui_clock` binary target; or drive it directly
with `rust-gdb`/`rust-lldb ./target/debug/egui_clock`.

Static analysis and the test suite:

```sh
cargo clippy --all-targets
cargo test
cargo fmt --check
```

## Creating the deliverable application package

```sh
cargo build --release   # -> target/release/egui_clock (~28 MB on Linux)
```

This produces a single self-contained executable — egui/eframe render
themselves (`wgpu`, with a CPU fallback) rather than embedding a webview or
linking a native widget toolkit, and the platform windowing libraries are
`dlopen`'d at runtime — so the binary can be copied and distributed
directly. There is no official bundler equivalent to `dx bundle`/`flutter
build`; wrapping it in a double-clickable `.app`/`.deb`/`.msi` would be a
manual packaging step, not exercised here.

## What it demonstrates

- **Wallclock** — current date/time in a user-selectable IANA timezone
  ([src/tabs/wallclock.rs](src/tabs/wallclock.rs)), computed with
  `chrono`/`chrono-tz` and persisted across restarts; the `:` separators
  blink once a second by toggling the label's `Color32` alpha on each
  ~33ms repaint.
- **Stopwatch** — start/stop/reset/lap using `std::time::Instant` (monotonic,
  unaffected by system clock adjustments), with recorded laps exportable as
  a plain-text report written straight to the OS data directory
  ([src/tabs/stopwatch.rs](src/tabs/stopwatch.rs)).
- **Synctime** — an `egui::Painter` strokes two open arcs (one
  rotation/second, one rotation/minute) sampled as polylines over a
  `Painter::line`, centred under a live `HH:MM:SS` text readout drawn with
  `Painter::text`, redrawn ~30 times a second via
  `Context::request_repaint_after` ([src/tabs/synctime.rs](src/tabs/synctime.rs)).
- **Settings** — GUI theme (system/light/dark — system polls the OS via the
  `dark-light` crate every 2s, only while "System" is selected), contrast
  (normal/high, via a custom `egui::Visuals` override that pushes
  background/text to pure black/white and saturates the selection color),
  and a 50%–200% UI scale (applied through egui's native
  `Context::set_pixels_per_point`, so it scales every widget the same way an
  OS DPI scale factor would) — all persisted to a JSON file in the OS config
  directory ([src/settings.rs](src/settings.rs)) and applied live, without
  restarting the app.

## GUI testing

Unlike the [Iced contester](../iced/README.md#gui-testing), egui ships an
official testing crate, `egui_kittest` (built on `kittest` + `AccessKit`),
so this contester uses two layers of tests:

- **Pure-function unit tests**, the same idiomatic-Rust approach as the
  other Rust contesters (`#[cfg(test)]`, colocated in each module): clock
  formatting ([src/tabs/wallclock.rs](src/tabs/wallclock.rs)), elapsed-time
  formatting and lap-report export text
  ([src/tabs/stopwatch.rs](src/tabs/stopwatch.rs)), arc-point angle math
  ([src/tabs/synctime.rs](src/tabs/synctime.rs)), and settings
  (de)serialization plus the curated timezone list
  ([src/settings.rs](src/settings.rs)).
- **A real widget-level interaction test**
  ([src/tabs/stopwatch.rs](src/tabs/stopwatch.rs), `clicking_start_lap_...`)
  that builds an `egui_kittest::Harness` around the stopwatch tab's actual
  `ui` function, looks up the "Start"/"Lap"/"Stop"/"Reset" buttons by their
  accessible label (via `AccessKit`, the same tree screen readers use, not
  by pixel position) and clicks them, then asserts on the resulting state —
  the part that would silently break if a button were ever mislabeled or
  wired to the wrong action, which a pure unit test calling `State::toggle`
  directly cannot catch. It runs headless (no GPU, no window, no
  `--features wgpu`/`snapshot`), so it's exercised by plain `cargo test`.

Run them with `cargo test`. See [IMPLEMENTATION.md](IMPLEMENTATION.md) for
what was, and wasn't, verified by actually launching the compiled GUI
binary.

## Using it from Python

None — egui/eframe have no official Python bindings; only unofficial/
experimental PyO3 wrappers exist (see the comparison notes in
[../OVERVIEW.md](../OVERVIEW.md#egui-rust)).
