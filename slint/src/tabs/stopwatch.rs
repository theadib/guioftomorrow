use std::time::{Duration, Instant};

use crate::settings::export_dir;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RunState {
    Stopped,
    Running,
}

#[derive(Debug)]
pub struct State {
    run_state: RunState,
    accumulated: Duration,
    started_at: Option<Instant>,
    laps: Vec<Duration>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            run_state: RunState::Stopped,
            accumulated: Duration::ZERO,
            started_at: None,
            laps: Vec::new(),
        }
    }
}

impl State {
    /// Elapsed time so far, live while running. Uses `Instant`
    /// (monotonic, unaffected by system clock adjustments), not wall-clock
    /// deltas.
    pub fn elapsed(&self) -> Duration {
        match self.started_at {
            Some(start) => self.accumulated + start.elapsed(),
            None => self.accumulated,
        }
    }

    pub fn is_running(&self) -> bool {
        self.run_state == RunState::Running
    }

    pub fn laps(&self) -> &[Duration] {
        &self.laps
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

    pub fn lap(&mut self) {
        if self.is_running() {
            self.laps.push(self.elapsed());
        }
    }

    pub fn reset(&mut self) {
        if !self.is_running() {
            self.run_state = RunState::Stopped;
            self.started_at = None;
            self.accumulated = Duration::ZERO;
            self.laps.clear();
        }
    }

    /// Writes the lap report to the OS data directory and returns a
    /// message describing what happened, suitable for direct display.
    pub fn export(&self) -> String {
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
        match result {
            Ok(path) => format!("Exported to {}", path.display()),
            Err(err) => format!("Export failed: {err}"),
        }
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

/// Builds the plain-text lap report exported by the "Export" button; pulled
/// out as a pure function so it can be unit tested without touching the
/// filesystem.
fn export_text(laps: &[Duration]) -> String {
    let mut out = String::from("GUI of Tomorrow — Slint stopwatch export\n\n");
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
            "GUI of Tomorrow — Slint stopwatch export\n\n"
        );
    }

    #[test]
    fn toggle_then_reset_clears_everything() {
        let mut state = State::default();
        state.toggle();
        assert!(state.is_running());
        state.lap();
        assert_eq!(state.laps().len(), 1);
        // Reset is a no-op while running, matching the UI (the Reset
        // button is disabled while the stopwatch is running).
        state.reset();
        assert!(state.is_running());
        state.toggle();
        state.reset();
        assert!(!state.is_running());
        assert!(state.laps().is_empty());
        assert_eq!(state.elapsed(), Duration::ZERO);
    }

    #[test]
    fn lap_is_ignored_while_stopped() {
        let mut state = State::default();
        state.lap();
        assert!(state.laps().is_empty());
    }
}
