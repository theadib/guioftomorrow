# Implementation notes — Flutter

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
- [x] `flutter analyze` passes with no issues
- [x] Automated GUI/unit tests (`flutter test`), see [README.md](README.md#gui-testing)
- [x] Builds a release Linux desktop binary; manually exercised under Xvfb
      (all four tabs, stopwatch start/lap/stop/export end-to-end, and
      theme/contrast/scale settings verified visually via screenshots)
- [ ] Windows build — not verified (no Windows toolchain in the dev
      container this was built in); the `windows/` platform folder was
      scaffolded and the code is platform-agnostic, but `flutter build
      windows` has not actually been run.
- [ ] Android build — not verified (no Android SDK in the dev container);
      same caveat as Windows.
- [x] Screenshots — see [README.md](README.md#screenshots) /
      [screenshots/](screenshots), captured from the Linux release build
      running under Xvfb at its default 1280×720 window size.

## Side notes

- **Timezone list is curated, not exhaustive.** The `timezone` package
  ships the full IANA database, but the dropdown only offers ~13 well-known
  zones (`lib/screens/wallclock_tab.dart`, `kAvailableTimezones`) to keep the
  demo UI simple. Swapping in the full list (`tz.timeZoneDatabase.locations.keys`)
  would be a one-line change if needed.
- **`getApplicationDocumentsDirectory()` is not used for the stopwatch
  export.** It initially was, but on Linux it resolves via `xdg-user-dirs`
  and throws `MissingPlatformDirectoryException` when that isn't configured
  (verified by actually running the export under a minimal `$HOME` with no
  XDG config — this reproduced in the dev container). Switched to
  `getApplicationSupportDirectory()`, which always resolves to a
  per-app directory and doesn't depend on desktop-environment configuration.
- **Stopwatch timing uses `dart:core`'s `Stopwatch`, not `DateTime.now()`
  deltas** — it's monotonic and unaffected by system clock adjustments,
  which matters for a stopwatch specifically (as opposed to the wallclock/
  synctime tabs, which are deliberately showing wall-clock time).
- **Wallclock's blinking colons** (`_buildAnimatedTime` in
  `lib/screens/wallclock_tab.dart`) use `AnimatedOpacity` toggled by
  `zoned.second.isEven`, reusing the existing once-a-second timer/rebuild
  rather than adding a second ticker — kept intentionally small/subtle
  rather than animating the digits themselves.
- **Settings are applied live, without an app restart.** `AppSettingsController`
  (`lib/services/app_settings_controller.dart`) is a `ChangeNotifier` created
  once in `ClockApp`'s state; `MaterialApp` listens to it and rebuilds with
  the new `themeMode`/`ColorScheme.fromSeed(contrastLevel: ...)`/
  `MediaQuery.textScaler` on every change, so toggling a setting updates the
  whole app (including the currently-visible tab) immediately.
- **UI scale only scales text (`MediaQuery.textScaler`), not layout metrics.**
  A literal `Transform.scale` over the whole app was considered and rejected:
  at 200% it would clip/overflow fixed-size layouts (e.g. the synctime
  circle, button rows) instead of reflowing them. Scaling text only mirrors
  how OS-level "text size" accessibility settings behave and is what
  Material widgets are built to respond to; most widgets (buttons, app bar,
  list tiles) grow with it since their intrinsic sizing follows text size.
- **`ColorScheme.fromSeed`'s `contrastLevel`** (0.0 normal, 1.0 for "high
  contrast" here) is Flutter's own dynamic-color contrast dial — verified
  visually (screenshots) that toggling it visibly shifts the palette in both
  light and dark theme.
- **Synctime's "two arcs" are drawn as short comet-trail arcs**
  (`Canvas.drawArc`) rather than full sweeping hands, so the direction and
  speed of rotation stay visible even when the two arcs happen to overlap.
- **No native platform code was added or modified** — `android/`, `linux/`
  and `windows/` are exactly what `flutter create` scaffolded; nothing there
  needed changes.
- **Verification environment**: built and run in a headless Linux
  container (Ubuntu 24.04, Flutter 3.47.4 stable) using `Xvfb` + `xdotool`
  to drive the actual compiled GUI binary and confirm the tabs render and
  the stopwatch/export flow works end-to-end, in addition to `flutter test`.
