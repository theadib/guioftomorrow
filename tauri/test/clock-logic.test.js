#!/usr/bin/env gjs
// Standalone test runner for src/clock-logic.js, run with `gjs
// test/clock-logic.test.js` (see README.md#gui-testing). No Node/npm is
// required — gjs (GNOME JavaScript, ships with any GNOME/GTK desktop,
// including the webkit2gtk stack this app itself links against) provides
// a plain SpiderMonkey engine with ICU/Intl support, which is all these
// pure functions need.

// Run from the `tauri/` project root (`gjs test/clock-logic.test.js`), so
// the file to load is simply src/clock-logic.js relative to the cwd.
const GLib = imports.gi.GLib;
const [, bytes] = GLib.file_get_contents("src/clock-logic.js");
eval(imports.byteArray.toString(bytes));

let failures = 0;
let count = 0;

function assertEqual(actual, expected, label) {
  count += 1;
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) {
    failures += 1;
    printerr(`not ok - ${label}: expected ${e}, got ${a}`);
  } else {
    print(`ok - ${label}`);
  }
}

function assertClose(actual, expected, label, epsilon = 1e-9) {
  count += 1;
  if (Math.abs(actual - expected) > epsilon) {
    failures += 1;
    printerr(`not ok - ${label}: expected ~${expected}, got ${actual}`);
  } else {
    print(`ok - ${label}`);
  }
}

// --- formatWallclock -------------------------------------------------------
{
  const d = new Date(Date.UTC(2026, 2, 5, 7, 8, 9)); // Thu 5 Mar 2026, 07:08:09 UTC
  const { hh, mm, ss, dateLine } = formatWallclock(d, "UTC");
  assertEqual(hh, "07", "formatWallclock: hh");
  assertEqual(mm, "08", "formatWallclock: mm");
  assertEqual(ss, "09", "formatWallclock: ss");
  assertEqual(dateLine, "Thursday, March 5, 2026", "formatWallclock: date line");
}
{
  // Asia/Tokyo is UTC+9 with no DST, so midnight UTC New Year's Day is 09:00 local.
  const d = new Date(Date.UTC(2026, 0, 1, 0, 0, 0));
  const { hh, mm, ss } = formatWallclock(d, "Asia/Tokyo");
  assertEqual(`${hh}:${mm}:${ss}`, "09:00:00", "formatWallclock: honours the timezone offset");
}

// --- formatElapsed -----------------------------------------------------------
assertEqual(formatElapsed(9_345), "00:09.34", "formatElapsed: sub-minute");
assertEqual(formatElapsed(3 * 3_600_000 + 61_000), "3:01:01.00", "formatElapsed: past an hour");
assertEqual(formatElapsed(0), "00:00.00", "formatElapsed: zero");

// --- buildExportText ---------------------------------------------------------
{
  const text = buildExportText([1_000, 2_500, 2_600]);
  assertEqual(
    text.includes("Lap  1: split 00:01.00  total 00:01.00"),
    true,
    "buildExportText: lap 1 split/total",
  );
  assertEqual(
    text.includes("Lap  2: split 00:01.50  total 00:02.50"),
    true,
    "buildExportText: lap 2 split/total",
  );
  assertEqual(
    text.includes("Lap  3: split 00:00.10  total 00:02.60"),
    true,
    "buildExportText: lap 3 split/total",
  );
}
assertEqual(
  buildExportText([]),
  "GUI of Tomorrow — Tauri stopwatch export\n\n",
  "buildExportText: no laps is just the header",
);

// --- second/minute angle + arcPath -------------------------------------------
assertClose(secondAngle(new Date(2026, 0, 1, 12, 0, 0, 0)), 0, "secondAngle: zero at top of second");
assertClose(
  secondAngle(new Date(2026, 0, 1, 12, 0, 0, 500)),
  180,
  "secondAngle: half rotation mid-second",
);
assertClose(
  secondAngle(new Date(2026, 0, 1, 12, 0, 42, 500)),
  180,
  "secondAngle: resets every second regardless of the minute",
);
assertClose(minuteAngle(new Date(2026, 0, 1, 12, 0, 30, 0)), 180, "minuteAngle: half rotation at 30s");
assertClose(
  minuteAngle(new Date(2026, 0, 1, 12, 45, 30, 0)),
  180,
  "minuteAngle: resets every minute regardless of the hour",
);
{
  const path = arcPath(100, 100, 50, 0, 40);
  assertEqual(path.endsWith("100.00 50.00"), true, "arcPath: ends exactly at the requested point");
}

print(`\n${count - failures}/${count} assertions passed`);
if (failures > 0) {
  imports.system.exit(1);
}
