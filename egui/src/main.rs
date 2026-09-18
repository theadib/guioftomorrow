mod settings;
mod tabs;

use std::time::{Duration, Instant};

use chrono::Utc;

use settings::{AppSettings, Contrast, ThemeMode};

const SLOW_POLL_INTERVAL: Duration = Duration::from_secs(2);
const TICK_INTERVAL: Duration = Duration::from_millis(33);

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 560.0]),
        ..Default::default()
    };
    eframe::run_native(
        "GUI of Tomorrow — egui",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Wallclock,
    Stopwatch,
    Synctime,
    Settings,
}

const TABS: [(Tab, &str); 4] = [
    (Tab::Wallclock, "Wallclock"),
    (Tab::Stopwatch, "Stopwatch"),
    (Tab::Synctime, "Synctime"),
    (Tab::Settings, "Settings"),
];

struct App {
    settings: AppSettings,
    active_tab: Tab,
    stopwatch: tabs::stopwatch::State,
    os_dark: bool,
    last_os_poll: Instant,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let settings = AppSettings::load();
        let os_dark = matches!(dark_light::detect(), Ok(dark_light::Mode::Dark));
        Self {
            settings,
            active_tab: Tab::Wallclock,
            stopwatch: tabs::stopwatch::State::default(),
            os_dark,
            last_os_poll: Instant::now(),
        }
    }

    /// The system theme is only worth polling while "System" is actually
    /// selected; `dark_light::detect()` is a cheap synchronous OS query
    /// (reads a registry key / gsettings / `NSApp` appearance), so a plain
    /// 2-second poll on the UI thread is simple and fast enough for a demo.
    fn poll_os_theme(&mut self) {
        if self.settings.theme_mode != ThemeMode::System {
            return;
        }
        if self.last_os_poll.elapsed() < SLOW_POLL_INTERVAL {
            return;
        }
        self.last_os_poll = Instant::now();
        self.os_dark = matches!(dark_light::detect(), Ok(dark_light::Mode::Dark));
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        let dark = match self.settings.theme_mode {
            ThemeMode::Dark => true,
            ThemeMode::Light => false,
            ThemeMode::System => self.os_dark,
        };
        let mut visuals = if dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };
        if self.settings.contrast == Contrast::High {
            apply_high_contrast(&mut visuals, dark);
        }
        ctx.set_visuals(visuals);
        ctx.set_pixels_per_point(self.settings.ui_scale as f32 / 100.0);
    }
}

/// Pushes background/text to pure black/white and saturates the selection
/// color, the same "custom high-contrast palette" approach the Iced
/// contester uses, translated to egui's `Visuals` fields.
fn apply_high_contrast(visuals: &mut egui::Visuals, dark: bool) {
    let (bg, fg) = if dark {
        (egui::Color32::BLACK, egui::Color32::WHITE)
    } else {
        (egui::Color32::WHITE, egui::Color32::BLACK)
    };
    visuals.override_text_color = Some(fg);
    visuals.panel_fill = bg;
    visuals.window_fill = bg;
    visuals.extreme_bg_color = bg;
    visuals.faint_bg_color = bg;
    visuals.widgets.noninteractive.bg_fill = bg;
    visuals.widgets.inactive.bg_fill = bg;
    visuals.selection.bg_fill = egui::Color32::from_rgb(0x00, 0x66, 0xFF);
    visuals.selection.stroke = egui::Stroke::new(2.0, fg);
    visuals.hyperlink_color = egui::Color32::from_rgb(0x40, 0x80, 0xFF);
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.poll_os_theme();
        self.apply_theme(ui.ctx());

        let now_utc = Utc::now();

        egui::Panel::top("tabs").show(ui, |ui| {
            ui.add_space(4.0);
            ui.heading("GUI of Tomorrow — egui");
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                for (tab, label) in TABS {
                    if ui.selectable_label(self.active_tab == tab, label).clicked() {
                        self.active_tab = tab;
                    }
                }
            });
            ui.add_space(4.0);
        });

        egui::CentralPanel::default().show(ui, |ui| match self.active_tab {
            Tab::Wallclock => tabs::wallclock::ui(ui, now_utc, &mut self.settings),
            Tab::Stopwatch => tabs::stopwatch::ui(ui, &mut self.stopwatch),
            Tab::Synctime => tabs::synctime::ui(ui, now_utc.with_timezone(&chrono::Local)),
            Tab::Settings => tabs::settings_tab::ui(ui, &mut self.settings),
        });

        // Immediate-mode apps must ask to be redrawn; a steady ~30fps tick
        // drives the wallclock blink, the live stopwatch display and the
        // synctime arcs, mirroring the Iced contester's single app-wide
        // subscription tick.
        ui.ctx().request_repaint_after(TICK_INTERVAL);
    }
}
