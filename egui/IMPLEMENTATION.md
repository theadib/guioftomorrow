# Implementation notes — egui

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
- [x] Automated tests (`cargo test`, 18 tests: 17 pure-function unit tests
      plus one real `egui_kittest` widget-interaction test), see
      [README.md](README.md#gui-testing)
- [x] `cargo build` / `cargo build --release` succeed and produce a working
      Linux desktop binary
- [x] The compiled binary was actually launched against a live display (not
      just compiled): it opened, ran for ~9 seconds with no error output,
      and exited cleanly on `SIGTERM`. See side notes below for what this
      check did and didn't cover.
- [ ] Screenshots — **not captured**. This dev environment is a live GNOME/
      Wayland session (confirmed via `$WAYLAND_DISPLAY`/`$XDG_SESSION_TYPE`);
      the GNOME Shell screenshot D-Bus method returns `AccessDenied:
      Screenshot is not allowed`, and while ImageMagick's `import` *is*
      installed here (unlike the Iced contester's environment), it only
      captures X11 windows — `xwininfo -root -tree` finds no X11 window for
      the running process, confirming winit created a native Wayland
      surface (no XWayland fallback), which `import` cannot see either. The
      app was confirmed running (see above) but not visually captured.
- [ ] Windows build — not verified (no Windows toolchain in this dev
      environment); the code has no platform-specific branches beyond what
      `dark-light`/`dirs`/`winit`/`wgpu` already handle per-OS, but `cargo
      build --target x86_64-pc-windows-msvc` has not actually been run.
- [ ] macOS build — not verified, same caveat as Windows.

## Side notes

- **`eframe::App`'s required method is `ui(&mut self, ui: &mut egui::Ui,
  frame: &mut Frame)`, not the classic `update(&mut self, ctx: &Context,
  frame: &mut Frame)`** familiar from older egui tutorials/examples found
  online. In the `eframe 0.35.0`/`egui 0.35.0` pair resolved by this
  project's `Cargo.lock`, panels also take `&mut Ui` instead of `&Context`
  (`egui::Panel::top(id).show(ui, |ui| ...)`,
  `egui::CentralPanel::default().show(ui, |ui| ...)`; the old
  `TopBottomPanel`/`SidePanel` names are gone in favour of a single
  `Panel::top`/`left`/`right`/`bottom` constructor). [src/main.rs](src/main.rs)
  gets the `Context` back out via `ui.ctx()` for `set_visuals`/
  `set_pixels_per_point`/`request_repaint_after`. Worth flagging explicitly
  since it means egui code samples predating this version won't compile
  as-is against it.
- **`egui`/`eframe`/`egui_kittest` are pinned to `0.35.0`, not the newest
  `0.36.2` available on crates.io**, because `0.36.2` declares
  `rust-version = "1.95"` while this dev environment's toolchain is
  `1.94.1`; `0.35.0` only requires `1.92` and was the newest version this
  toolchain could actually build.
- **A direct `egui = "0.35.0"` dependency was required alongside `eframe`**,
  even though `eframe` re-exports egui as `eframe::egui`. Every tab module
  does `use egui::{...}` directly (so tab code doesn't need to route every
  type through the `eframe::` path), which only resolves if `egui` is also
  a declared dependency of this crate, not just a transitive one accessed
  through `eframe::egui`.
- **The GUI test story is a genuine egui advantage over the Iced
  contester**: `egui_kittest` (an official crate, built on `kittest` +
  `AccessKit`) lets a test build a real `Harness` around a tab's actual
  `ui(&mut Ui, &mut State)` function, query widgets by their accessible
  label the same way a screen reader would, click them, and assert on the
  resulting state — see
  `clicking_start_lap_and_reset_drives_the_real_buttons` in
  [src/tabs/stopwatch.rs](src/tabs/stopwatch.rs). This runs with zero extra
  Cargo features (no GPU, no window, no `--features wgpu`) because
  `Harness::new_ui_state` only needs to lay out and query the accessibility
  tree, not rasterize pixels — pixel-snapshot testing (`--features
  snapshot,wgpu`) exists in the crate too but wasn't exercised here, since
  the point was to verify real interaction/wiring, not pixel-perfect
  rendering.
- **Only the stopwatch tab got a widget-interaction test, not settings.**
  [src/tabs/settings_tab.rs](src/tabs/settings_tab.rs)'s `ui` function calls
  `AppSettings::save()` (writing to the real OS config directory) on every
  changed widget; a kittest test clicking its radio buttons would therefore
  write to the developer's actual `~/.config/gui-of-tomorrow-egui/` during
  `cargo test`, which felt like the wrong tradeoff for a demo. The
  stopwatch's `export()` has the same real-filesystem-write property, which
  is why the interaction test drives Start/Lap/Stop/Reset but deliberately
  never clicks "Export".
- **UI scale uses egui's native `Context::set_pixels_per_point`**, not a
  per-widget font-size trick, mirroring the Iced contester's use of
  `Application::scale_factor` for the same reason: it multiplies the
  effective rendering scale the same way an OS DPI setting would, scaling
  every widget (text, padding, control sizes) uniformly for free.
- **Contrast is implemented as a custom `egui::Visuals` override**
  ([src/main.rs](src/main.rs), `apply_high_contrast`), not a CSS class
  toggle: it pushes `panel_fill`/`window_fill`/`extreme_bg_color`/
  `faint_bg_color` and `override_text_color` to pure black/white and
  saturates `selection.bg_fill`, applied on top of whichever of
  `Visuals::light()`/`dark()` is active. This only reached "looks plausible
  in code" — not visually verified, per the screenshot caveat above.
- **"System" theme polling, not push-based updates**, the same tradeoff the
  Iced and Dioxus contesters made: `dark_light::detect()` is a synchronous
  OS query, so [src/main.rs](src/main.rs)'s `poll_os_theme` just calls it
  directly from the UI thread, gated to once every 2 seconds and only while
  `ThemeMode::System` is selected — no async task machinery needed the way
  Iced's `Subscription`/`Task` model required, since egui's immediate-mode
  `App::ui` is already called on a plain synchronous loop. System-theme
  changes are therefore picked up within ~2s rather than instantly.
- **A single `request_repaint_after(33ms)` call drives everything**
  (wallclock digits, the blink, the stopwatch display while running, and
  the synctime arcs) rather than per-tab timers, the same deliberate
  simplification the Iced contester documents: egui doesn't tear down
  inactive tabs' state, so there's no cheap "unmounted" signal to gate a
  timer on, and the CPU cost of one ~30fps repaint is negligible for a demo
  app.
- **Stopwatch timing uses `std::time::Instant`, not wall-clock deltas** —
  monotonic and unaffected by system clock adjustments, same rationale as
  every other contester.
- **Timezone list is curated, not exhaustive**, same tradeoff as the other
  Rust/Dart contesters: `chrono-tz` ships the full IANA database, but
  [src/settings.rs](src/settings.rs)'s `AVAILABLE_TIMEZONES` only lists ~15
  well-known zones to keep the timezone `ComboBox` usable; every entry is
  unit tested to actually parse as a valid `chrono_tz::Tz`.
- **Settings are applied live, without an app restart.** `AppSettings`
  lives directly in the root `App` struct; every settings change writes
  through `settings.save()` immediately (see the note above about why this
  keeps the settings tab out of the kittest suite), and `apply_theme`
  re-reads `self.settings` from scratch on every single `App::ui` call, so
  the very next frame reflects the change.
- **The synctime arc math uses a "clock face" convention directly** (`0°` =
  12 o'clock = straight up, clockwise-positive), converted to egui's
  y-down screen space with `x = cx + r·sin(θ), y = cy − r·cos(θ)` in
  `clock_point` ([src/tabs/synctime.rs](src/tabs/synctime.rs)); an arc is
  then just a sampled polyline (`arc_points`, 32 segments) fed to
  `Painter::line`, since egui — unlike Iced's `canvas::Arc` — has no
  built-in stroked-arc primitive. Unit tested at the two easy-to-get-wrong
  reference points (12 o'clock and 3 o'clock) plus the polyline's sampled
  endpoints.
- **No native platform code was added or modified** — this is a single
  `cargo` binary crate; `winit`/`wgpu` (via `egui-winit`/`egui-wgpu`) handle
  all platform windowing/rendering glue internally, identical in spirit to
  the Iced contester's setup.
