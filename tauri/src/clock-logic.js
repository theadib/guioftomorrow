// Pure, DOM-free helpers shared by app.js and the gjs test suite
// (test/clock-logic.test.js). Plain global function declarations on
// purpose — no bundler/module system is used anywhere in this project, so
// a plain <script> include and a `gjs`-evaluated include need to see the
// exact same functions in the exact same way.

/**
 * Splits a `Date` into the wallclock fields for `timeZone`, using the
 * platform's own `Intl` timezone database rather than a bundled one.
 * Returns 24-hour `hh`/`mm`/`ss` and a full weekday/date line.
 */
function formatWallclock(date, timeZone) {
  const time = new Intl.DateTimeFormat("en-GB", {
    timeZone,
    hourCycle: "h23",
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  })
    .formatToParts(date)
    .reduce((acc, part) => {
      acc[part.type] = part.value;
      return acc;
    }, {});

  const dateLine = new Intl.DateTimeFormat("en-US", {
    timeZone,
    weekday: "long",
    day: "numeric",
    month: "long",
    year: "numeric",
  }).format(date);

  return { hh: time.hour, mm: time.minute, ss: time.second, dateLine };
}

/** Formats an elapsed duration (in milliseconds) as `mm:ss.cc`, or `h:mm:ss.cc` past an hour. */
function formatElapsed(totalMs) {
  const ms = Math.max(0, Math.round(totalMs));
  const hours = Math.floor(ms / 3_600_000);
  const minutes = Math.floor(ms / 60_000) % 60;
  const seconds = Math.floor(ms / 1000) % 60;
  const centis = Math.floor(ms / 10) % 100;
  const pad = (n) => String(n).padStart(2, "0");
  return hours > 0
    ? `${hours}:${pad(minutes)}:${pad(seconds)}.${pad(centis)}`
    : `${pad(minutes)}:${pad(seconds)}.${pad(centis)}`;
}

/** Builds the plain-text lap report exported by the "Export" button, from cumulative lap times in milliseconds. */
function buildExportText(lapsMs) {
  let out = "GUI of Tomorrow — Tauri stopwatch export\n\n";
  let previous = 0;
  lapsMs.forEach((lap, index) => {
    const split = lap - previous;
    out += `Lap ${String(index + 1).padStart(2, " ")}: split ${formatElapsed(split)}  total ${formatElapsed(lap)}\n`;
    previous = lap;
  });
  return out;
}

/** A point on a circle of radius `r` centred at `(cx, cy)`, `deg` clockwise from 12 o'clock (SVG coordinates, y grows downward). */
function pointOnCircle(cx, cy, r, deg) {
  const theta = (deg * Math.PI) / 180;
  return { x: cx + r * Math.sin(theta), y: cy - r * Math.cos(theta) };
}

/** SVG path `d` for a short comet-trail arc of `sweepDeg` degrees ending at `endDeg` (clockwise from 12 o'clock). */
function arcPath(cx, cy, r, endDeg, sweepDeg) {
  const end = ((endDeg % 360) + 360) % 360;
  const start = end - sweepDeg;
  const s = pointOnCircle(cx, cy, r, start);
  const e = pointOnCircle(cx, cy, r, end);
  const largeArc = sweepDeg > 180 ? 1 : 0;
  return `M ${s.x.toFixed(2)} ${s.y.toFixed(2)} A ${r.toFixed(2)} ${r.toFixed(2)} 0 ${largeArc} 1 ${e.x.toFixed(2)} ${e.y.toFixed(2)}`;
}

/** Degrees clockwise from 12 o'clock for the once-per-second arc: resets to 0 at the top of every second. */
function secondAngle(date) {
  return (date.getMilliseconds() / 1000) * 360;
}

/** Degrees clockwise from 12 o'clock for the once-per-minute arc: resets to 0 at the top of every minute. */
function minuteAngle(date) {
  return ((date.getSeconds() + date.getMilliseconds() / 1000) / 60) * 360;
}
