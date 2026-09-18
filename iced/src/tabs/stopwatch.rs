use std::time::{Duration, Instant};

use iced::widget::{button, column, row, scrollable, text};
use iced::{Center, Element, Fill};

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
}

#[derive(Debug, Clone)]
pub enum Message {
    Toggle,
    Lap,
    Reset,
    Export,
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
    let mut out = String::from("GUI of Tomorrow — Iced stopwatch export\n\n");
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

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::Toggle => match state.run_state {
            RunState::Running => {
                state.accumulated = state.elapsed();
                state.started_at = None;
                state.run_state = RunState::Stopped;
            }
            RunState::Stopped => {
                state.started_at = Some(Instant::now());
                state.run_state = RunState::Running;
            }
        },
        Message::Reset => {
            state.run_state = RunState::Stopped;
            state.started_at = None;
            state.accumulated = Duration::ZERO;
            state.laps.clear();
            state.export_message = None;
        }
        Message::Lap => {
            state.laps.push(state.elapsed());
        }
        Message::Export => {
            let text = export_text(&state.laps);
            let dir = export_dir();
            let result = std::fs::create_dir_all(&dir).and_then(|_| {
                let path = dir.join(format!(
                    "stopwatch-{}.txt",
                    chrono::Local::now().format("%Y%m%d-%H%M%S")
                ));
                std::fs::write(&path, text)?;
                Ok(path)
            });
            state.export_message = Some(match result {
                Ok(path) => format!("Exported to {}", path.display()),
                Err(err) => format!("Export failed: {err}"),
            });
        }
    }
}

pub fn view(state: &State) -> Element<'_, Message> {
    let running = state.run_state == RunState::Running;

    let buttons = row![
        button(if running { "Stop" } else { "Start" }).on_press(Message::Toggle),
        button("Lap").on_press_maybe(running.then_some(Message::Lap)),
        button("Reset").on_press_maybe((!running).then_some(Message::Reset)),
        button("Export").on_press_maybe((!state.laps.is_empty()).then_some(Message::Export)),
    ]
    .spacing(8);

    let mut content = column![text(format_elapsed(state.elapsed())).size(48), buttons,]
        .spacing(16)
        .align_x(Center);

    if let Some(message) = &state.export_message {
        content = content.push(text(message.clone()));
    }

    let laps = state
        .laps
        .iter()
        .enumerate()
        .rev()
        .fold(column![].spacing(4), |col, (index, lap)| {
            col.push(text(format!("Lap {}: {}", index + 1, format_elapsed(*lap))))
        });

    content.push(scrollable(laps).height(Fill)).into()
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
            "GUI of Tomorrow — Iced stopwatch export\n\n"
        );
    }

    #[test]
    fn toggle_then_reset_clears_everything() {
        let mut state = State::default();
        update(&mut state, Message::Toggle);
        assert_eq!(state.run_state, RunState::Running);
        update(&mut state, Message::Lap);
        assert_eq!(state.laps.len(), 1);
        update(&mut state, Message::Reset);
        assert_eq!(state.run_state, RunState::Stopped);
        assert!(state.laps.is_empty());
        assert_eq!(state.accumulated, Duration::ZERO);
    }
}
