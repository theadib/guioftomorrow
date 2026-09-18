use std::time::{Duration, Instant};

use dioxus::prelude::*;

use crate::settings::export_dir;

/// Formats an elapsed duration as `mm:ss.cc`, or `h:mm:ss.cc` past an hour.
fn format_elapsed(d: Duration) -> String {
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
    let mut out = String::from("GUI of Tomorrow — Dioxus stopwatch export\n\n");
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

#[derive(Clone, Copy, PartialEq)]
enum RunState {
    Stopped,
    Running,
}

#[component]
pub fn StopwatchTab() -> Element {
    let mut state = use_signal(|| RunState::Stopped);
    let mut accumulated = use_signal(|| Duration::ZERO);
    let mut started_at = use_signal(|| None::<Instant>);
    let mut laps = use_signal(Vec::<Duration>::new);
    let mut export_message = use_signal(|| None::<String>);
    // Bumped every tick just to force a re-render while running; the actual
    // elapsed time is always recomputed from `accumulated`/`started_at`.
    let mut tick = use_signal(|| 0u64);

    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_millis(30)).await;
            if *state.read() == RunState::Running {
                tick += 1;
            }
        }
    });

    let elapsed = {
        let base = *accumulated.read();
        match *started_at.read() {
            Some(start) => base + start.elapsed(),
            None => base,
        }
    };
    let _ = *tick.read(); // subscribe to ticks so `elapsed` above re-renders

    let running = *state.read() == RunState::Running;

    let toggle = move |_| {
        if running {
            let base = *accumulated.read();
            if let Some(start) = *started_at.read() {
                accumulated.set(base + start.elapsed());
            }
            started_at.set(None);
            state.set(RunState::Stopped);
        } else {
            started_at.set(Some(Instant::now()));
            state.set(RunState::Running);
        }
    };

    let reset = move |_| {
        state.set(RunState::Stopped);
        started_at.set(None);
        accumulated.set(Duration::ZERO);
        laps.write().clear();
        export_message.set(None);
    };

    let lap = move |_| {
        laps.write().push(elapsed);
    };

    let export = move |_| {
        let text = export_text(&laps.read());
        let dir = export_dir();
        let result = std::fs::create_dir_all(&dir).and_then(|_| {
            let path = dir.join(format!(
                "stopwatch-{}.txt",
                chrono::Local::now().format("%Y%m%d-%H%M%S")
            ));
            std::fs::write(&path, text)?;
            Ok(path)
        });
        export_message.set(Some(match result {
            Ok(path) => format!("Exported to {}", path.display()),
            Err(err) => format!("Export failed: {err}"),
        }));
    };

    rsx! {
        div { class: "tab stopwatch-tab",
            div { class: "clock-display stopwatch-elapsed", "{format_elapsed(elapsed)}" }
            div { class: "button-row",
                button { onclick: toggle, if running { "Stop" } else { "Start" } }
                button { disabled: !running, onclick: lap, "Lap" }
                button { disabled: running, onclick: reset, "Reset" }
                button { disabled: laps.read().is_empty(), onclick: export, "Export" }
            }
            if let Some(message) = export_message.read().as_ref() {
                div { class: "export-message", "{message}" }
            }
            ol { class: "lap-list",
                for (index , lap) in laps.read().iter().enumerate().rev() {
                    li { key: "{index}", "Lap {index + 1}: {format_elapsed(*lap)}" }
                }
            }
        }
    }
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
            "GUI of Tomorrow — Dioxus stopwatch export\n\n"
        );
    }
}
