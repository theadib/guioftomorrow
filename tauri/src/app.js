// DOM/Tauri glue layer. All non-trivial logic (formatting, geometry, export
// text) lives in clock-logic.js so it can be unit tested standalone with
// `gjs` (see test/clock-logic.test.js) without a browser or Node.

const { invoke } = window.__TAURI__.core;
const { getCurrentWindow } = window.__TAURI__.window;

const root = document.getElementById("app-root");

// ---------------------------------------------------------------------------
// Tabs
// ---------------------------------------------------------------------------
document.querySelectorAll(".tab-button").forEach((button) => {
  button.addEventListener("click", () => {
    const target = button.dataset.tab;
    document
      .querySelectorAll(".tab-button")
      .forEach((b) => b.classList.toggle("active", b === button));
    document
      .querySelectorAll("[data-panel]")
      .forEach((panel) => (panel.hidden = panel.dataset.panel !== target));
  });
});

// ---------------------------------------------------------------------------
// Settings: load once, persist on every change, apply live (no restart).
// ---------------------------------------------------------------------------
let settings = {
  theme_mode: "system",
  contrast: "normal",
  ui_scale: 100,
  timezone: "UTC",
};
let osIsDark = false;

function applyAppearance() {
  const dark = settings.theme_mode === "dark" || (settings.theme_mode === "system" && osIsDark);
  root.classList.toggle("theme-dark", dark);
  root.classList.toggle("contrast-high", settings.contrast === "high");
  root.style.fontSize = `${settings.ui_scale}%`;
}

function applySettingsToControls() {
  document.querySelectorAll('input[name="theme-mode"]').forEach((input) => {
    input.checked = input.value === settings.theme_mode;
  });
  document.querySelectorAll('input[name="contrast"]').forEach((input) => {
    input.checked = input.value === settings.contrast;
  });
  const scaleInput = document.getElementById("ui-scale");
  scaleInput.value = String(settings.ui_scale);
  document.getElementById("ui-scale-legend").textContent = `UI scale — ${settings.ui_scale}%`;
  const tzSelect = document.getElementById("wc-tz");
  if (tzSelect.value !== settings.timezone) tzSelect.value = settings.timezone;
}

async function saveSettings() {
  applyAppearance();
  try {
    await invoke("save_settings", { settings });
  } catch (err) {
    console.error("Failed to save settings:", err);
  }
}

function wireSettingsControls() {
  document.querySelectorAll('input[name="theme-mode"]').forEach((input) => {
    input.addEventListener("change", () => {
      if (input.checked) {
        settings.theme_mode = input.value;
        saveSettings();
      }
    });
  });
  document.querySelectorAll('input[name="contrast"]').forEach((input) => {
    input.addEventListener("change", () => {
      if (input.checked) {
        settings.contrast = input.value;
        saveSettings();
      }
    });
  });
  document.getElementById("ui-scale").addEventListener("input", (evt) => {
    settings.ui_scale = Number(evt.target.value);
    document.getElementById("ui-scale-legend").textContent = `UI scale — ${settings.ui_scale}%`;
    saveSettings();
  });
  document.getElementById("wc-tz").addEventListener("change", (evt) => {
    settings.timezone = evt.target.value;
    saveSettings();
  });
}

async function initSettings() {
  const [loaded, timezones] = await Promise.all([
    invoke("load_settings"),
    invoke("available_timezones"),
  ]);
  settings = loaded;

  const tzSelect = document.getElementById("wc-tz");
  tzSelect.innerHTML = "";
  timezones.forEach((zone) => {
    const option = document.createElement("option");
    option.value = zone;
    option.textContent = zone;
    tzSelect.appendChild(option);
  });

  applySettingsToControls();
  applyAppearance();
  wireSettingsControls();
}

// Tauri exposes the OS theme both as a one-shot getter and as a live,
// push-based change event — no polling loop needed (unlike the dark-light
// crate's poll-every-2s approach the Dioxus/Slint contesters use).
async function initSystemTheme() {
  const win = getCurrentWindow();
  osIsDark = (await win.theme()) === "dark";
  applyAppearance();
  await win.onThemeChanged(({ payload: theme }) => {
    osIsDark = theme === "dark";
    if (settings.theme_mode === "system") applyAppearance();
  });
}

// ---------------------------------------------------------------------------
// Wallclock
// ---------------------------------------------------------------------------
function tickWallclock() {
  const { hh, mm, ss, dateLine } = formatWallclock(new Date(), settings.timezone);
  document.getElementById("wc-hh").textContent = hh;
  document.getElementById("wc-mm").textContent = mm;
  document.getElementById("wc-ss").textContent = ss;
  document.getElementById("wc-date").textContent = dateLine;
  const blink = Number(ss) % 2 === 0;
  const opacity = blink ? "1" : "0.25";
  document.getElementById("wc-colon1").style.opacity = opacity;
  document.getElementById("wc-colon2").style.opacity = opacity;
}

// ---------------------------------------------------------------------------
// Stopwatch (performance.now() is monotonic, unaffected by system clock
// adjustments — the same rationale the other Rust contesters use
// std::time::Instant for).
// ---------------------------------------------------------------------------
const stopwatch = {
  running: false,
  accumulatedMs: 0,
  startedAt: null,
  laps: [],
};

function stopwatchElapsedMs() {
  return stopwatch.accumulatedMs + (stopwatch.running ? performance.now() - stopwatch.startedAt : 0);
}

function renderStopwatch() {
  document.getElementById("sw-elapsed").textContent = formatElapsed(stopwatchElapsedMs());
  document.getElementById("sw-toggle").textContent = stopwatch.running ? "Stop" : "Start";
  document.getElementById("sw-lap").disabled = !stopwatch.running;
  document.getElementById("sw-reset").disabled = stopwatch.running;
  document.getElementById("sw-export").disabled = stopwatch.laps.length === 0;

  const list = document.getElementById("sw-laps");
  list.innerHTML = "";
  for (let i = stopwatch.laps.length - 1; i >= 0; i -= 1) {
    const li = document.createElement("li");
    li.textContent = `Lap ${i + 1}: ${formatElapsed(stopwatch.laps[i])}`;
    list.appendChild(li);
  }
}

function wireStopwatch() {
  document.getElementById("sw-toggle").addEventListener("click", () => {
    if (stopwatch.running) {
      stopwatch.accumulatedMs = stopwatchElapsedMs();
      stopwatch.startedAt = null;
      stopwatch.running = false;
    } else {
      stopwatch.startedAt = performance.now();
      stopwatch.running = true;
    }
    renderStopwatch();
  });

  document.getElementById("sw-lap").addEventListener("click", () => {
    stopwatch.laps.push(stopwatchElapsedMs());
    renderStopwatch();
  });

  document.getElementById("sw-reset").addEventListener("click", () => {
    stopwatch.running = false;
    stopwatch.startedAt = null;
    stopwatch.accumulatedMs = 0;
    stopwatch.laps = [];
    document.getElementById("sw-export-message").textContent = "";
    renderStopwatch();
  });

  document.getElementById("sw-export").addEventListener("click", async () => {
    const text = buildExportText(stopwatch.laps);
    const stamp = new Date()
      .toISOString()
      .replace(/[:.]/g, "-");
    const messageEl = document.getElementById("sw-export-message");
    try {
      const path = await invoke("export_stopwatch", {
        filename: `stopwatch-${stamp}.txt`,
        contents: text,
      });
      messageEl.textContent = `Exported to ${path}`;
    } catch (err) {
      messageEl.textContent = `Export failed: ${err}`;
    }
  });

  setInterval(() => {
    if (stopwatch.running) renderStopwatch();
  }, 30);
}

// ---------------------------------------------------------------------------
// Synctime
// ---------------------------------------------------------------------------
const SYNC_CENTER = 120;
const SYNC_SECOND_RADIUS = 108;
const SYNC_MINUTE_RADIUS = 82;
const SYNC_SWEEP_DEG = 46;

function tickSynctime() {
  const now = new Date();
  document
    .getElementById("st-second-arc")
    .setAttribute(
      "d",
      arcPath(SYNC_CENTER, SYNC_CENTER, SYNC_SECOND_RADIUS, secondAngle(now), SYNC_SWEEP_DEG),
    );
  document
    .getElementById("st-minute-arc")
    .setAttribute(
      "d",
      arcPath(SYNC_CENTER, SYNC_CENTER, SYNC_MINUTE_RADIUS, minuteAngle(now), SYNC_SWEEP_DEG),
    );
  const pad = (n) => String(n).padStart(2, "0");
  document.getElementById("st-readout").textContent =
    `${pad(now.getHours())}:${pad(now.getMinutes())}:${pad(now.getSeconds())}`;
}

// ---------------------------------------------------------------------------
// Boot
// ---------------------------------------------------------------------------
async function main() {
  wireStopwatch();
  renderStopwatch();
  await Promise.all([initSettings(), initSystemTheme()]);
  tickWallclock();
  tickSynctime();
  setInterval(tickWallclock, 500);
  setInterval(tickSynctime, 33);
}

main();
