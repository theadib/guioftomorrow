# Implementation notes — Iced

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
  - [x] UI scale: 50%–200%, applied app-wide
  - [x] All three persist across app restarts
- [x] Tabbed navigation across all four demos
- [x] `cargo clippy --all-targets` passes with no warnings
- [x] `cargo fmt --check` passes with no diffs
- [x] Automated unit tests (`cargo test`, 16 tests), see
      [README.md](README.md#gui-testing)
- [x] `cargo build` / `cargo build --release` succeed and produce a working
      Linux desktop binary
- [x] The compiled binary was actually launched against a live display (not
      just compiled): it opened, ran for several seconds with no error
      output, and exited cleanly on request. See side notes below for what
      this check did and didn't cover.
- [ ] Screenshots — **not captured**. This dev environment is a live GNOME/
      Wayland session (confirmed via `$WAYLAND_DISPLAY`/`$XDG_SESSION_TYPE`),
      unlike the Dioxus/imgui contesters' plain-X11 environment, but it is
      still sandboxed against screen capture: `gdbus call --session --dest
      org.gnome.Shell --object-path /org/gnome/Shell/Screenshot --method
      org.gnome.Shell.Screenshot.Screenshot …` returns
      `AccessDenied: Screenshot is not allowed`, and no CLI screenshot tool
      (`grim`, `gnome-screenshot`, `scrot`, `import`/ImageMagick) is
      installed. The app was confirmed running (see above) but not visually
      captured.
- [ ] Windows build — not verified (no Windows toolchain in this dev
      environment); the code has no platform-specific branches beyond what
      `dark-light`/`dirs`/`winit`/`wgpu` already handle per-OS, but `cargo
      build --target x86_64-pc-windows-msvc` has not actually been run.
- [ ] macOS build — not verified, same caveat as Windows.

## Side notes

- **Rendering backend and what "the binary was launched" actually
  verified.** With `WAYLAND_DISPLAY=wayland-0` set (a real GNOME/mutter
  session, confirmed reachable), running `./target/debug/iced_clock` in the
  background stayed alive with an empty stderr/stdout log for 7+ seconds and
  exited cleanly on `pkill` — i.e. it got through `winit` window creation and
  `wgpu`/`tiny-skia` renderer initialization and into the event loop without
  panicking, which is the part most likely to break (no GPU adapter, missing
  Wayland/X11 client libs, etc.). It does *not* confirm pixel-level
  rendering correctness (see the screenshot caveat above) or that the
  stopwatch export writes a well-formed file end-to-end under real user
  interaction — those are covered instead by the unit tests around the pure
  formatting/export/update logic (`format_elapsed`, `export_text`,
  `stopwatch::update`, the angle-conversion functions, …), not by driving the
  actual UI, since no input-automation tool (`xdotool`/`wmctrl`, and no
  Wayland equivalent) was available.
- **No dynamically-linked GUI libraries.** `ldd target/release/iced_clock`
  shows only `libc`/`libm`/`libgcc_s`. `winit` (via `x11-dl`/
  `wayland-client`) and `wgpu-hal` (via `ash`) `dlopen` the platform
  windowing/Vulkan libraries at runtime instead of linking them at build
  time, so the built binary has no hard `.so` dependency on X11, Wayland or
  Vulkan — it degrades to Iced's CPU renderer (`tiny-skia`) if none of them
  are present, rather than failing to start. Documented in
  [README.md](README.md#creating-the-deliverable-application-package) since
  it's a genuinely different distribution story than Dioxus's
  webview-dependent binary.
- **UI scale uses Iced's native `Application::scale_factor` hook**, not a
  CSS-style font-size trick. This multiplies the effective window scale
  factor the same way an OS DPI setting would, so it scales every widget
  (text, padding, control sizes) uniformly for free — arguably a cleaner
  mechanism than the Dioxus contester's `font-size`-on-root-plus-`em`-units
  approach, since it doesn't rely on every stylesheet rule using relative
  units.
- **Contrast is implemented as a custom `iced::Theme::custom` palette**
  ([src/main.rs](src/main.rs)), not a CSS class toggle: `HIGH_CONTRAST_LIGHT`
  /`HIGH_CONTRAST_DARK` push `background`/`text` to pure white/black and
  saturate `primary`/`success`/`warning`/`danger`, and Iced regenerates the
  full `palette::Extended` (button/text/container style variants) from that
  automatically. This only reached "looks plausible in code" — not visually
  verified, per the screenshot caveat above.
- **"System" theme polling, not push-based updates**, mirroring the Dioxus
  contester's approach: the `dark-light` crate exposes a blocking `detect()`
  plus an OS-level watcher, but wiring a live cross-platform watcher into
  Iced's `Subscription` was more machinery than this demo warranted.
  Instead, the app's `subscription` function only includes a 2-second
  `iced::time::every` tick *while* "System" is actually selected
  ([src/main.rs](src/main.rs)), which dispatches a `Task::perform` running
  `tokio::task::spawn_blocking(dark_light::detect)`. System-theme changes
  are therefore picked up within ~2s rather than instantly.
- **A single 33ms app-wide tick drives everything** (wallclock digits, the
  blink, the stopwatch display while running, and the synctime arcs), rather
  than the Dioxus contester's per-tab timers that only ran while that tab's
  component was mounted. Iced's `State` isn't torn down when a tab is
  merely not the active `view`, so there's no equivalent "unmounted" signal
  to gate on cheaply; a single always-on subscription was simpler and the
  CPU cost of one 30fps timer is negligible for a demo app. Noted here as a
  deliberate simplification, not an oversight.
- **Stopwatch timing uses `std::time::Instant`, not wall-clock deltas** —
  monotonic and unaffected by system clock adjustments, same rationale as
  the Flutter and Dioxus contesters.
- **Timezone list is curated, not exhaustive**, same tradeoff as the other
  Rust/Dart contesters: `chrono-tz` ships the full IANA database, but
  [src/settings.rs](src/settings.rs)'s `AVAILABLE_TIMEZONES` only lists ~15
  well-known zones to keep the `pick_list` dropdown usable; every entry is
  unit tested to actually parse as a valid `chrono_tz::Tz`.
- **Settings are applied live, without an app restart.** `AppSettings` lives
  directly in the root `App` state; every settings change goes through
  `update`, which persists to disk and lets Iced's `theme`/`scale_factor`
  functions (which are just `Fn(&State) -> _`, re-evaluated on every
  message) pick up the change on the very next redraw.
- **The synctime canvas angle convention needed an explicit conversion.**
  This app's arc math (and the Dioxus contester's SVG-path math) both use
  "degrees clockwise from 12 o'clock", but `canvas::Arc` expects radians
  clockwise from the positive x-axis (3 o'clock). `clock_degrees_to_radians`
  in [src/tabs/synctime.rs](src/tabs/synctime.rs) does the `deg - 90°`
  conversion and is unit tested at the two easy-to-get-wrong reference
  points (12 o'clock and 3 o'clock).
- **No native platform code was added or modified** — this is a single
  `cargo` binary crate; `winit`/`wgpu` (via `iced_winit`/`iced_renderer`)
  handle all platform windowing/rendering glue internally.
