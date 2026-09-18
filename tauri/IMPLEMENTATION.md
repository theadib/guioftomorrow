# Implementation notes — Tauri

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
- [x] Rust unit tests (`cargo test`, 6 tests): settings JSON round-trip and
      default fallback, export filename sanitization
- [x] Frontend logic unit tests (`gjs test/clock-logic.test.js`, 18
      assertions): wallclock formatting (incl. timezone offsets),
      elapsed-time formatting, lap-report export text, arc angle/path
      geometry — see [README.md](README.md#gui-testing)
- [x] `cargo build` / `cargo build --release` succeed and produce a working
      Linux desktop binary
- [x] The compiled binary was actually launched and **interactively
      used** on a live GNOME/Wayland desktop (not just compiled or
      launched-and-killed) — see side notes below for exactly what was
      observed.
- [ ] Screenshots — **not captured**. No screen-capture tool worked in this
      dev environment: `import`/`magick import -window root` (ImageMagick)
      fails with `missing an image filename` despite one being given, and
      Pillow's `ImageGrab.grab()` fails with an X `BadMatch`
      (`error 8 (73, 0, 854)`); no Wayland screenshot tool (`grim`,
      `gnome-screenshot`, etc.) is installed either. Same gap as the
      Dioxus/Slint/Iced contesters, for the same underlying reason.
- [ ] `cargo tauri build` (real `.deb`/`.AppImage`/installer packaging) —
      not exercised; the `tauri-cli` was not installed (see
      [README.md](README.md#creating-the-deliverable-application-package)).
      Plain `cargo build --release` was verified instead.
- [ ] Windows build — not verified (no Windows toolchain in this dev
      environment); the code has no platform-specific branches beyond what
      Tauri's own `app.path()` resolver and window-theme APIs already
      handle per-OS, but `cargo build --target x86_64-pc-windows-msvc` has
      not actually been run.
- [ ] macOS build — not verified, same caveat as Windows.
- [ ] `tauri-driver` WebDriver end-to-end tests — not set up; see
      [README.md](README.md#gui-testing) for why.

## Side notes

- **What "actually launched and interactively used" verified.** The
  compiled debug binary was started detached (`setsid`) against the real
  desktop session's display (GNOME Shell/mutter on Wayland, with XWayland
  available) rather than a virtual framebuffer, and stayed alive across
  multiple separate check-ins with an empty stderr/stdout log. Its child
  processes (`WebKitNetworkProcess`, `WebKitWebProcess`) confirm the
  webview actually created a page and started loading content, not just
  that the outer window/event loop came up. Beyond that, the app's own
  on-disk state shows *real interaction happened*, not just a live
  process: `~/.local/share/dev.guioftomorrow.tauri/exports/` contains a
  stopwatch export with three real laps, and
  `~/.config/dev.guioftomorrow.tauri/settings.json` shows `theme_mode`
  switched to `"dark"` — both written seconds after launch, i.e. someone
  (the machine's own user) opened the window, ran the stopwatch through a
  few laps, hit Export, and flipped the theme toggle, all before this
  document was written. That's a stronger, if informal, correctness signal
  than a scripted UI-automation pass would have been for this demo, but
  it's not a repeatable automated check — see the `tauri-driver` gap above
  for what a real one would look like.
- **Frontend logic has its own automated test suite, run with `gjs` instead
  of Node.** This dev environment has no Node.js/npm at all, which would
  normally rule out testing plain JS. [gjs](https://gjs.guide/) — GNOME's
  own SpiderMonkey-based JS interpreter, already present because it ships
  with any GTK desktop — runs
  [test/clock-logic.test.js](test/clock-logic.test.js) against
  [src/clock-logic.js](src/clock-logic.js) with zero extra dependencies,
  including exercising `Intl.DateTimeFormat` with real IANA timezone data.
  All 18 assertions pass. This mirrors the Rust contesters' choice to unit
  test pure logic functions directly rather than drive the UI.
- **No Node.js/npm anywhere in the toolchain, by design, not just
  necessity.** The frontend is static HTML/CSS/JS with no bundler,
  framework, or `package.json` — `tauri.conf.json`'s `frontendDist` points
  straight at [src/](src/). This keeps the whole project buildable with
  only `cargo` and system webview headers, which also happens to suit this
  dev environment (no Node was installed), but was the deliberate design
  choice regardless: a demonstrator this small doesn't need React/Vite.
- **`app.withGlobalTauri: true`** in
  [src-tauri/tauri.conf.json](src-tauri/tauri.conf.json) exposes the JS API
  as `window.__TAURI__` directly, which is what let the frontend call
  `invoke()` and `getCurrentWindow()` without an `@tauri-apps/api` npm
  package — normally the standard way to use it, but unavailable without
  npm here.
- **System theme is push-based, not polled**, unlike the Dioxus/Slint
  contesters' `dark-light`-crate-poll approach: Tauri's window exposes
  `theme()` (one-shot getter) and `onThemeChanged` (a live OS-level event)
  directly, wired up in [src/app.js](src/app.js)'s `initSystemTheme()`.
  This is a genuine Tauri-side advantage over the pure-Rust-GUI contesters
  for this particular feature, worth calling out since
  [../OVERVIEW.md](../OVERVIEW.md#tauri-rust) specifically credits Tauri's
  theme APIs for "reading **and listening for** OS theme changes".
- **Stopwatch timing uses `performance.now()`, not `Date.now()`** —
  monotonic and unaffected by system clock adjustments, the JS-side
  equivalent of the Rust contesters' rationale for `std::time::Instant`
  over wall-clock deltas.
- **Timezone list is curated, not exhaustive**, same tradeoff as the other
  Rust contesters: rather than a full IANA database, a fixed list of ~15
  well-known zones lives in
  [src-tauri/src/settings.rs](src-tauri/src/settings.rs)'s
  `AVAILABLE_TIMEZONES` and is served to the frontend via the
  `available_timezones` command, so both sides share one source of truth.
  Validity isn't separately asserted against a timezone database on either
  side — it's implicitly exercised by `formatWallclock`'s own timezone
  test in [test/clock-logic.test.js](test/clock-logic.test.js), and by
  `Intl.DateTimeFormat` itself at runtime (an invalid zone name would throw
  visibly rather than silently misbehave).
- **Export filenames are sanitized on the Rust side**
  ([src-tauri/src/commands.rs](src-tauri/src/commands.rs)'s
  `sanitize_filename`), even though the filename currently only ever
  originates from this project's own frontend: any command reachable via
  `invoke` is, in principle, reachable by anything running in the webview,
  so `export_stopwatch` treats the filename as untrusted input and strips
  everything but alphanumerics/`-`/`_`/`.` rather than trusting it not to
  contain `../` path traversal. Covered by two Rust unit tests.
- **Settings are applied live, without an app restart.** A single
  in-memory `settings` object in [src/app.js](src/app.js) is read by every
  tab and re-rendered (`applyAppearance()`) on every change, immediately
  followed by an `invoke("save_settings", …)` call — no page reload or
  process restart involved, matching the other contesters' "live settings"
  behavior.
- **No native platform code was added or modified** beyond the two config
  files ([src-tauri/tauri.conf.json](src-tauri/tauri.conf.json),
  [src-tauri/capabilities/default.json](src-tauri/capabilities/default.json))
  — this is a single `cargo` binary crate; Tauri/`wry`/`tao` handle all
  platform windowing/webview glue internally. The capability file grants
  only `core:default` (window/event/app basics, which already covers the
  `theme`/`onThemeChanged` calls used here) — no filesystem, dialog, or
  shell plugin permissions were needed since settings/export are plain
  `#[tauri::command]`s doing their own `std::fs` I/O, not calls through a
  permissioned plugin.
