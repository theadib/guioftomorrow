# Implementation notes — Dioxus

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
  - [x] Arcs update continuously (~30 fps)
  - [x] `hh:mm:ss` shown as text in the centre
- [x] **Settings** tab
  - [x] GUI theme: system / light / dark
  - [x] Contrast: normal / high contrast
  - [x] UI scale: 50%–200%, applied to fonts app-wide
  - [x] All three persist across app restarts
- [x] Tabbed navigation across all four demos
- [x] `cargo clippy --all-targets` passes with no warnings
- [x] `cargo fmt --check` passes with no diffs
- [x] Automated unit tests (`cargo test`, 12 tests), see
      [README.md](README.md#gui-testing)
- [x] `cargo build` / `cargo build --release` succeed and produce a working
      Linux desktop binary
- [x] The compiled binary was actually launched against a live X11 display
      (not just compiled): it opened, ran for several seconds with no error
      output, and exited cleanly on request. See side notes below for what
      this check did and didn't cover.
- [ ] Screenshots — **not captured**. Unlike the Flutter contester (built
      under Xvfb + xdotool), this dev environment has no window-manager
      automation tooling (`xdotool`/`wmctrl`) and no working screen-capture
      path: `import`/`magick import` (ImageMagick) fails because the
      system's `policy.xml` denies the screen-capture ("X") coder, and
      Pillow's `ImageGrab.grab()` fails with an X `BadMatch` against this
      particular display. Fixing either requires root (to edit
      `/etc/ImageMagick-7/policy.xml`), which wasn't available. The app was
      confirmed running (see above) but not visually captured.
- [ ] Windows build — not verified (no Windows toolchain in this dev
      environment); the code has no platform-specific branches beyond what
      `dark-light`/`dirs`/`tao` already handle per-OS, but `cargo build
      --target x86_64-pc-windows-msvc` has not actually been run.
- [ ] macOS build — not verified, same caveat as Windows.

## Side notes

- **Why `openssl-sys` is vendored.** `dioxus-desktop` unconditionally
  depends on `tungstenite` with the `native-tls` feature on non-Android
  targets (for its own internal dev-asset websocket), which needs OpenSSL.
  This dev environment has `libssl3` but not the `-dev`/headers package, and
  there was no way to install it (no passwordless `sudo`). Pinning
  `openssl-sys = { features = ["vendored"] }` in
  [Cargo.toml](Cargo.toml) makes the build compile its own OpenSSL from
  source instead (needs a C compiler + Perl, both commonly available) —
  documented in [README.md](README.md#installing-the-development-environment)
  since a real developer hitting this would otherwise be stuck on a
  confusing `pkg-config` error.
- **`devtools` feature is disabled.** The `dioxus` crate's `default`
  features pull in `devtools` (hot-reload/dev-panel support), which is what
  actually drags in the websocket/TLS dependency chain above. Since this
  project doesn't use `dx serve`, `Cargo.toml` opts out of default features
  and enables only `desktop`, `launch` and `lib` — smaller dependency tree,
  though `openssl-sys` still gets linked in via `dioxus-desktop` itself
  regardless (see above).
- **What "the binary was launched" actually verified.** With `DISPLAY=:0`
  set and a reachable X server (confirmed via `xdpyinfo`), running
  `./target/debug/dioxus_clock` in the background stayed alive with an
  empty stderr/stdout log for several seconds and exited cleanly on
  `kill`/close — i.e. it got through window/webview creation and into the
  event loop without panicking or erroring, which is the part most likely
  to break (wrong webkit2gtk version, missing GTK init, etc.). It does
  *not* confirm pixel-level rendering correctness or that stopwatch export
  writes a well-formed file end-to-end under real user interaction — those
  are covered instead by the unit tests around the pure formatting/export
  logic (`format_elapsed`, `export_text`, `arc_path`, …), not by driving the
  actual UI, since no input-automation tool was available.
- **Stopwatch timing uses `std::time::Instant`, not wall-clock deltas** —
  monotonic and unaffected by system clock adjustments, mirroring the
  Flutter contester's rationale for using `dart:core`'s `Stopwatch` instead
  of `DateTime.now()` for the same tab.
- **Timezone list is curated, not exhaustive**, same tradeoff as the
  Flutter contester: `chrono-tz` ships the full IANA database, but
  [src/settings.rs](src/settings.rs)'s `AVAILABLE_TIMEZONES` only lists
  ~15 well-known zones to keep the dropdown usable; every entry is unit
  tested to actually parse as a valid `chrono_tz::Tz`.
- **"System" theme polling, not push-based updates.** The `dark-light`
  crate exposes a blocking `detect()` plus an OS-level `subscribe()`/
  `stream()` watcher, but wiring a live cross-platform watcher into a
  Dioxus signal was more machinery than this demo warranted; instead a
  `tokio::task::spawn_blocking(dark_light::detect)` call runs every 2
  seconds (only while "System" is actually selected) and updates a signal.
  This means system-theme changes are picked up within ~2s rather than
  instantly — noted here since it's a real (small) gap versus Flutter's
  `ThemeMode.system`, which Flutter/the OS handle for you.
- **UI scale only scales `font-size` on the app root**, relying on the CSS
  (`assets/main.css`) using `em`/relative units throughout so buttons,
  labels and layout gaps grow with it. This is the same design choice the
  Flutter contester made with `MediaQuery.textScaler` (scale text, not raw
  layout transforms) — verified at the CSS-authoring level, not
  screenshot-verified here (see the screenshots caveat above).
- **Settings are applied live, without an app restart.** `AppSettings` is a
  single `Signal<AppSettings>` created once in the root `App` component and
  shared via `use_context_provider`/`use_context`; every tab that reads or
  writes it triggers a re-render of the whole app (theme class, contrast
  class, font-size) on the next event loop tick.
- **No native platform code was added or modified** — this is a single
  `cargo` binary crate; `tao`/`wry` (via `dioxus-desktop`) handle all
  platform windowing/webview glue internally.
