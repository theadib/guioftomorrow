# Dioxus clock demonstrator

The Dioxus contester for the [GUI of Tomorrow](../README.md) comparison: a
tabbed clock app (wallclock, stopwatch, synctime, settings) built with
[Dioxus](https://dioxuslabs.com)/Rust, running as a desktop app in the OS's
native webview. See [../OVERVIEW.md](../OVERVIEW.md#dioxus-rust) for the
framework write-up and [IMPLEMENTATION.md](IMPLEMENTATION.md) for the
completeness checklist.

## Installing the development environment

1. Install Rust (this project was built and tested against **Rust/Cargo
   1.94.1**, but any reasonably recent stable toolchain works): via
   [rustup](https://rustup.rs) or your OS package manager.
2. Install the platform webview/toolkit headers Dioxus desktop links against:
   - **Linux desktop**: GTK 3 and WebKitGTK development headers — on
     Debian/Ubuntu:
     ```sh
     sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libssl-dev pkg-config
     ```
     (`libssl-dev` is only needed if you'd rather link the system OpenSSL;
     see the note below — without it, the build compiles its own vendored
     copy automatically.)
   - **Windows desktop**: no extra system packages — the OS-provided
     WebView2 runtime (preinstalled on current Windows) is used. Visual
     Studio's "Desktop development with C++" workload provides the linker.
   - **macOS**: Xcode command line tools (`xcode-select --install`); the
     system WKWebView is used, no extra packages.
3. From this directory, fetch dependencies and build:
   ```sh
   cargo build
   ```

> **Note on OpenSSL:** `dioxus-desktop` links `tungstenite`/`native-tls` on
> Linux for its own internal dev-asset websocket, which needs OpenSSL. This
> project's [Cargo.toml](Cargo.toml) pins `openssl-sys` with the `vendored`
> feature, so `cargo build` compiles OpenSSL from source and does **not**
> require `libssl-dev` to be installed — useful in minimal/CI containers.
> If you'd rather link the system OpenSSL (faster incremental builds), drop
> that line from `Cargo.toml` and install `libssl-dev` instead.

This project uses plain `cargo build`/`cargo run` rather than the `dx` CLI —
no asset pipeline or hot-reload is used, so the Dioxus CLI is optional. If
you'd like hot-reload during development, install it with
`cargo install dioxus-cli` and use `dx serve` instead of `cargo run`.

## Building, running and debugging

```sh
cargo run              # debug build, opens the app window
cargo run --release    # optimized build
```

For interactive debugging (breakpoints, variable inspection), open this
folder in VS Code (rust-analyzer + CodeLLDB extensions) or any IDE with Rust
support, and run/debug the `dioxus_clock` binary target; or drive it
directly with `rust-gdb`/`rust-lldb ./target/debug/dioxus_clock`.

Static analysis and the test suite:

```sh
cargo clippy --all-targets
cargo test
cargo fmt --check
```

## Creating the deliverable application package

```sh
cargo build --release   # -> target/release/dioxus_clock
```

This produces a single self-contained executable (it links the OS's system
webview rather than bundling one) that can be copied and distributed
directly — no separate installer step for this demonstrator. For a
double-clickable `.app`/`.deb`/`.msi` package, `dx bundle` (from
`dioxus-cli`) can wrap this same binary; that step wasn't exercised here.

## What it demonstrates

- **Wallclock** — current date/time in a user-selectable IANA timezone
  ([src/tabs/wallclock.rs](src/tabs/wallclock.rs)), computed with
  `chrono`/`chrono-tz` and persisted across restarts; the `:` separators
  blink once a second as a small clock-face animation (a CSS `opacity`
  toggle driven by the same once-a-second re-render).
- **Stopwatch** — start/stop/reset/lap using `std::time::Instant` (monotonic,
  unaffected by system clock adjustments), with recorded laps exportable as
  a plain-text report written straight to the OS data directory
  ([src/tabs/stopwatch.rs](src/tabs/stopwatch.rs)).
- **Synctime** — inline SVG `<path>` arcs (one rotation/second, one
  rotation/minute) around a live `HH:MM:SS` readout, redrawn ~30 times a
  second via a `tokio::time::sleep` loop
  ([src/tabs/synctime.rs](src/tabs/synctime.rs)).
- **Settings** — GUI theme (system/light/dark — system polls the OS via the
  `dark-light` crate every 2s), contrast (normal/high, via CSS custom
  properties), and a 50%–200% UI scale (applied as the root element's
  `font-size`, cascading to every `em`/`rem`-based measurement) — all
  persisted to a JSON file in the OS config directory
  ([src/settings.rs](src/settings.rs)) and applied live, without restarting
  the app.

## GUI testing

Dioxus's own component-testing story is still young, and this app doesn't
have a native widget-level test harness comparable to Flutter's
`flutter_test`. Instead, every piece of non-trivial logic is factored out
into small, pure, standalone functions and unit tested directly with
`#[cfg(test)]` (the idiomatic Rust convention, colocated in each module
rather than a separate `test/` directory):

- clock formatting ([src/tabs/wallclock.rs](src/tabs/wallclock.rs))
- elapsed-time formatting and lap-report export text
  ([src/tabs/stopwatch.rs](src/tabs/stopwatch.rs))
- arc angle/path geometry ([src/tabs/synctime.rs](src/tabs/synctime.rs))
- settings (de)serialization and the curated timezone list
  ([src/settings.rs](src/settings.rs))

Run them with `cargo test`. See [IMPLEMENTATION.md](IMPLEMENTATION.md) for
what was, and wasn't, verified by actually launching the compiled GUI
binary.

## Using it from Python

None — Dioxus is a Rust framework with no official Python bindings for
building the UI itself (see the comparison notes in
[../OVERVIEW.md](../OVERVIEW.md#dioxus-rust)).
