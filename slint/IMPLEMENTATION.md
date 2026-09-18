# Implementation notes — Slint

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
  - [x] UI scale: 50%–200%, applied app-wide (to every custom element; see
        the std-widget caveat below)
  - [x] All three persist across app restarts
- [x] Tabbed navigation across all four demos
- [x] `cargo clippy --all-targets` passes with no warnings
- [x] `cargo fmt --check` passes with no diffs
- [x] Automated unit tests (`cargo test`, 20 tests), see
      [README.md](README.md#gui-testing)
- [x] `cargo build` / `cargo build --release` succeed and produce a working
      Linux desktop binary (Rust/Cargo 1.94.1)
- [x] The compiled binary was actually launched against a live display (not
      just compiled) — three times (debug twice, release once), each
      staying alive with no error output and exiting cleanly on request.
      See the side note below on what one of those runs additionally
      revealed.
- [ ] Screenshots — **not captured**, the same sandbox limitation the
      Dioxus/imgui contesters hit: `import` (ImageMagick, confirmed
      present) fails with `missing an image filename` even when one is
      given. No other screenshot tool (`grim`, `gnome-screenshot`,
      `scrot`) was installed.
- [ ] Windows build — not verified (no Windows toolchain in this dev
      environment); the code has no platform-specific branches beyond what
      `dirs`/`winit`/Slint's own backend already handle per-OS, but `cargo
      build --target x86_64-pc-windows-msvc` has not actually been run.
- [ ] macOS build — not verified, same caveat as Windows.

## Side notes

- **What one of the launches actually verified — more than expected.**
  This is a live GNOME/Wayland desktop (`WAYLAND_DISPLAY=wayland-0`,
  `XDG_SESSION_TYPE=wayland`), not an isolated headless display, matching
  the environment the Iced and imgui contesters ran in. With no
  input-automation tool available (no `xdotool`/`wmctrl`, and no Wayland
  equivalent), the plan was to confirm only that the window opens and the
  event loop runs without crashing. On the second debug-binary launch,
  that happened — and more: checking `~/.config/gui-of-tomorrow-slint/`
  and `~/.local/share/gui-of-tomorrow-slint/exports/` a few seconds apart,
  during the same run, showed `settings.json`'s `theme_mode` change from
  `"Dark"` to `"Light"` and `ui_scale` from `100` to `90`, and a
  `stopwatch-20260919-004016.txt` export file appear containing four
  genuinely different, plausible lap splits (`00:02.21`, `00:00.58`,
  `00:00.52`, `00:00.52`). No automated input was driven from this
  session, so the most likely explanation — as with the imgui contester's
  equivalent note — is that the window was visibly open on the real
  desktop and received real mouse clicks during the session: tab
  switching, the theme/contrast/scale controls, and a stopwatch
  start → four laps → export sequence. That is stronger end-to-end
  evidence than a scripted click sequence would have given, though it
  doesn't substitute for the pure-function unit tests as *regression*
  coverage — it confirms the wired-up UI works, not that it keeps working.
- **`Palette.color-scheme` is the theme override mechanism, not a
  custom-built dark/light palette.** Slint's `std-widgets` export a global
  `Palette` with an `in-out property <ColorScheme> color-scheme` that every
  built-in widget already reads to pick its colors, and that defaults to
  `Unknown` (follow the OS setting). [ui/theme.slint](ui/theme.slint)
  re-exports it unchanged; `apply_theme()` in
  [src/main.rs](src/main.rs) sets it to `ColorScheme.Light`/`Dark` for an
  explicit user choice and back to `ColorScheme.Unknown` for "System" —
  Slint then keeps following the OS setting on its own, no polling loop
  needed (unlike the Iced/Dioxus/imgui contesters, which all poll
  `dark-light`/`gsettings` every 1–2s). This only works because the app
  intentionally avoids the `backend-qt` feature (see below): the `qt`
  widget style has no exported `Palette`, since it defers theming entirely
  to the native Qt platform theme.
- **No Qt dependency, by construction, not by extra configuration.**
  `slint`'s `backend-qt` feature is *not* part of its `default` feature set
  (confirmed by reading `slint-1.18.0`'s `Cargo.toml`), so a plain
  `slint = "1.18.0"` dependency never links or requires Qt even on a system
  that has it installed — `i-slint-common`'s native-style detection only
  resolves to `"qt"` when the `backend-qt` feature is compiled in, which it
  isn't here. `Cargo.toml` doesn't disable default features or force a
  style, letting each OS's default (`fluent` on Linux/Windows, `cupertino`
  on macOS) apply — both support the `Palette` override above.
- **UI scale is an app-defined multiplier (`AppTheme.scale`), not a native
  "scale the whole app" API.** Slint's `Window::scale_factor()` is a
  read-only reflection of the OS DPI setting from the windowing backend;
  the one runtime setter, `set_const_scale_factor()`, is explicitly
  one-shot ("constant and cannot be changed anymore" per its own doc
  comment) — unsuitable for a live, user-adjustable 50–200% slider. So
  every font-size, spacing, padding and radius binding this app's own
  `.slint` components define is written as `<literal>px * AppTheme.scale`
  ([ui/theme.slint](ui/theme.slint) holds the `in-out property <float>
  scale`, set from `settings.ui_scale / 100.0` in `apply_theme()`).
  Functionally this is the same trick the Dioxus contester used with CSS
  `em`-on-root, just expressed as a Slint property multiplier instead of a
  CSS unit — and it has the same real limitation: `std-widgets`' own
  internal metrics (`ComboBox`'s padding/arrow, `Slider`'s groove/handle
  thickness) are fixed px values in the shipped widget style, so only the
  *text* inside those two controls scales, not their chrome. This is why
  the tab bar, the theme/contrast selectors and the stopwatch buttons are
  all a hand-rolled [`PillButton`](ui/widgets.slint) instead of `std`
  `Button` — full control over both scale and contrast colors — while only
  the timezone dropdown (`ComboBox`) and the UI-scale control (`Slider`)
  keep the documented native-chrome-doesn't-scale gap, the same kind of
  pragmatic, documented trade-off the imgui contester made for contrast.
- **Contrast is a hand-rolled color table (`AppTheme` in
  [ui/theme.slint](ui/theme.slint)), not a Slint built-in** — Slint has no
  contrast-mode concept (see [../OVERVIEW.md](../OVERVIEW.md#slint-rust)).
  `AppTheme` computes `background`/`surface`/`foreground`/`muted`/
  `accent`/`accent-secondary`/`border` from two booleans (`dark`, read from
  `Palette.color-scheme`, and `high-contrast`, set by `apply_theme()`), and
  every custom-drawn element in this app reads from it. As with the scale
  caveat above, `ComboBox`/`Slider` keep their own fixed
  `FluentPalette`-derived colors regardless of `high-contrast` — a real,
  documented gap, not a general-purpose contrast system.
- **Synctime's arcs are declarative `Path { MoveTo {} ArcTo {} }` elements
  with property-bound endpoints, not an SVG path string.** `Path`'s
  `commands` property (SVG path syntax) turned out to be `@fake` in the
  compiler sources — sugar that's parsed into static geometry at compile
  time, not a runtime-bindable string. So
  [ui/synctime.slint](ui/synctime.slint) instead declares each arc as
  `MoveTo { x; y; }` / `ArcTo { x; y; radius-x; radius-y; sweep; large-arc;
  }` with `x`/`y` bound to `float` properties on a `Synctime` global, which
  [src/tabs/synctime.rs](src/tabs/synctime.rs)'s pure, unit-tested
  `point_on_circle()` recomputes every 33ms tick from the same
  degrees-clockwise-from-12-o'clock convention the Iced contester used
  (`sweep`/`large-arc` are static: the 46° sweep is always the minor,
  clockwise arc). The `Path` itself uses an explicit `viewbox-x/y/width/
  height` (a 240-unit square centered on the origin) so the coordinate math
  matches Iced's `canvas`-based version almost line for line, independent
  of `AppTheme.scale` (the `Path`'s pixel `width`/`height` scale instead,
  and Slint's `fit: contain` maps the fixed viewbox onto them).
- **One 33ms app-wide `slint::Timer`, not per-tab timers**, mirroring the
  Iced contester's "single always-on subscription" choice for the same
  reason: Slint's `if cond : Component {}` conditional elements are torn
  down when their tab isn't active, but reconstructing a per-tab timer on
  every tab switch would be more machinery than this demo warrants, and a
  30fps timer's CPU cost is negligible. The timer callback updates the
  wallclock/synctime globals unconditionally and the stopwatch's
  elapsed-time readout every tick, but only rebuilds the lap list
  (`sync_stopwatch_laps()`) from `on_lap`/`on_reset`, not from the ticking
  timer, since laps only change on those two user actions.
  ([src/main.rs](src/main.rs))
- **The `ComboBox` needed a non-empty default `timezones` list.**
  `WallclockModel.timezones` originally defaulted to `[]` in `.slint`, with
  `main.rs` populating the real curated list right after
  `AppWindow::new()`. But `ComboBoxBase`'s one-time init-time value
  resolution runs *during* component construction, i.e. before that Rust
  code executes, against the still-empty list — which cleared the
  selection and logged `ComboBox: current-value was set to "UTC", which is
  not in model` (a known Slint issue, slint-ui/slint#11970). Seeding the
  `.slint` default as `["UTC"]` instead (matching
  `selected-timezone`'s own default) gives that first resolution a match,
  so the message no longer appears; `main.rs` still immediately replaces
  it with the full 15-zone list from
  [src/settings.rs](src/settings.rs)'s `AVAILABLE_TIMEZONES`.
- **Timezone list is curated, not exhaustive**, the same tradeoff every
  other Rust/Dart/C++ contester in this repo made: `chrono-tz` ships the
  full IANA database, but `AVAILABLE_TIMEZONES` only lists 15 well-known
  zones to keep the dropdown usable; every entry is unit tested to
  actually parse as a valid `chrono_tz::Tz`.
- **Stopwatch timing uses `std::time::Instant`, not wall-clock deltas** —
  monotonic and unaffected by system clock adjustments, same rationale as
  every other contester.
- **No native platform code was added or modified** — this is a single
  `cargo` binary crate plus `.slint` markup; `winit` (via Slint's own
  `i-slint-backend-winit`) handles all platform windowing glue internally,
  and the release binary's `ldd` output (see
  [README.md](README.md#creating-the-deliverable-application-package))
  shows no hard dependency on X11/Wayland/OpenGL client libraries, only on
  `fontconfig`/`freetype` (linked for font discovery/shaping) and libc.
