use std::time::{Duration, Instant};

use egui::{RichText, ScrollArea, Ui};

use crate::settings::export_dir;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RunState {
    Stopped,
    Running,
}

pub struct State {
    run_state: RunState,
    accumulated: Duration,
    started_at: Option<Instant>,
    laps: Vec<Duration>,
    export_message: Option<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            run_state: RunState::Stopped,
            accumulated: Duration::ZERO,
            started_at: None,
            laps: Vec::new(),
            export_message: None,
        }
    }
}

impl State {
    /// Elapsed time so far, live while running.
    pub fn elapsed(&self) -> Duration {
        match self.started_at {
            Some(start) => self.accumulated + start.elapsed(),
            None => self.accumulated,
        }
    }

    pub fn is_running(&self) -> bool {
        self.run_state == RunState::Running
    }

    pub fn toggle(&mut self) {
        match self.run_state {
            RunState::Running => {
                self.accumulated = self.elapsed();
                self.started_at = None;
                self.run_state = RunState::Stopped;
            }
            RunState::Stopped => {
                self.started_at = Some(Instant::now());
                self.run_state = RunState::Running;
            }
        }
    }

    pub fn reset(&mut self) {
        self.run_state = RunState::Stopped;
        self.started_at = None;
        self.accumulated = Duration::ZERO;
        self.laps.clear();
        self.export_message = None;
    }

    pub fn lap(&mut self) {
        self.laps.push(self.elapsed());
    }

    pub fn export(&mut self) {
        let text = export_text(&self.laps);
        let dir = export_dir();
        let result = std::fs::create_dir_all(&dir).and_then(|_| {
            let path = dir.join(format!(
                "stopwatch-{}.txt",
                chrono::Local::now().format("%Y%m%d-%H%M%S")
            ));
            std::fs::write(&path, text)?;
            Ok(path)
        });
        self.export_message = Some(match result {
            Ok(path) => format!("Exported to {}", path.display()),
            Err(err) => format!("Export failed: {err}"),
        });
    }
}

/// Formats an elapsed duration as `mm:ss.cc`, or `h:mm:ss.cc` past an hour.
pub fn format_elapsed(d: Duration) -> String {
    let total_ms = d.as_millis();
    let hours = total_ms / 3_600_000;
    let minutes = (total_ms / 60_000) % 60;
    let seconds = (total_ms / 1000) % 60;
    let centis = (total_ms / 10) % 100;
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}.{centis:02}")
    } else {
        format!("{minutes:02}:{seconds:02}.{centis:02}")
    }
}

/// Builds the plain-text lap report the "Export" button writes to disk;
/// pulled out as a pure function so it can be unit tested without touching
/// the filesystem.
fn export_text(laps: &[Duration]) -> String {
    let mut out = String::from("GUI of Tomorrow — egui stopwatch export\n\n");
    let mut previous = Duration::ZERO;
    for (index, lap) in laps.iter().enumerate() {
        let split = *lap - previous;
        out.push_str(&format!(
            "Lap {:>2}: split {}  total {}\n",
            index + 1,
            format_elapsed(split),
            format_elapsed(*lap)
        ));
        previous = *lap;
    }
    out
}

pub fn ui(ui: &mut Ui, state: &mut State) {
    ui.vertical_centered(|ui| {
        ui.label(
            RichText::new(format_elapsed(state.elapsed()))
                .size(48.0)
                .monospace(),
        );

        ui.horizontal(|ui| {
            let running = state.is_running();
            if ui.button(if running { "Stop" } else { "Start" }).clicked() {
                state.toggle();
            }
            if ui.add_enabled(running, egui::Button::new("Lap")).clicked() {
                state.lap();
            }
            if ui
                .add_enabled(!running, egui::Button::new("Reset"))
                .clicked()
            {
                state.reset();
            }
            if ui
                .add_enabled(!state.laps.is_empty(), egui::Button::new("Export"))
                .clicked()
            {
                state.export();
            }
        });

        if let Some(message) = &state.export_message {
            ui.label(message);
        }

        ui.add_space(8.0);
        ScrollArea::vertical().show(ui, |ui| {
            for (index, lap) in state.laps.iter().enumerate().rev() {
                ui.label(format!("Lap {}: {}", index + 1, format_elapsed(*lap)));
            }
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_sub_minute_elapsed() {
        assert_eq!(format_elapsed(Duration::from_millis(9_345)), "00:09.34");
    }

    #[test]
    fn formats_elapsed_past_an_hour() {
        assert_eq!(
            format_elapsed(Duration::from_secs(3 * 3600 + 61)),
            "3:01:01.00"
        );
    }

    #[test]
    fn export_text_reports_splits_and_totals() {
        let laps = vec![
            Duration::from_millis(1_000),
            Duration::from_millis(2_500),
            Duration::from_millis(2_600),
        ];
        let text = export_text(&laps);
        assert!(text.contains("Lap  1: split 00:01.00  total 00:01.00"));
        assert!(text.contains("Lap  2: split 00:01.50  total 00:02.50"));
        assert!(text.contains("Lap  3: split 00:00.10  total 00:02.60"));
    }

    #[test]
    fn export_text_of_no_laps_is_just_the_header() {
        assert_eq!(
            export_text(&[]),
            "GUI of Tomorrow — egui stopwatch export\n\n"
        );
    }

    #[test]
    fn toggle_then_reset_clears_everything() {
        let mut state = State::default();
        state.toggle();
        assert!(state.is_running());
        state.lap();
        assert_eq!(state.laps.len(), 1);
        state.reset();
        assert!(!state.is_running());
        assert!(state.laps.is_empty());
        assert_eq!(state.accumulated, Duration::ZERO);
    }

    /// A widget-level test (via `egui_kittest`) driving the real `ui`
    /// function through actual button clicks, rather than calling
    /// `State::toggle`/`lap`/`reset` directly — this is the part that
    /// would break if the buttons were ever mislabeled or wired to the
    /// wrong action.
    #[test]
    fn clicking_start_lap_and_reset_drives_the_real_buttons() {
        use egui_kittest::kittest::Queryable as _;
        use egui_kittest::Harness;

        let mut harness = Harness::new_ui_state(super::ui, State::default());

        harness.get_by_label("Start").click();
        harness.run();
        assert!(harness.state().is_running());

        harness.get_by_label("Lap").click();
        harness.run();
        assert_eq!(harness.state().laps.len(), 1);

        // Reset is disabled while running; stop first.
        harness.get_by_label("Stop").click();
        harness.run();
        harness.get_by_label("Reset").click();
        harness.run();
        assert!(!harness.state().is_running());
        assert!(harness.state().laps.is_empty());
    }
}
