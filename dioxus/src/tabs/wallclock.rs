use std::time::Duration;

use chrono::{Timelike, Utc};
use chrono_tz::Tz;
use dioxus::prelude::*;

use crate::settings::{AppSettings, AVAILABLE_TIMEZONES};

/// Formats `now` (already converted to the target zone) as `HH`, `MM`, `SS`
/// plus a full date line. Pulled out as a pure function so it's unit
/// testable without driving a timer.
fn format_clock(now: &chrono::DateTime<Tz>) -> (String, String, String, String) {
    (
        now.format("%H").to_string(),
        now.format("%M").to_string(),
        now.format("%S").to_string(),
        now.format("%A, %-d %B %Y").to_string(),
    )
}

#[component]
pub fn WallclockTab() -> Element {
    let mut settings = use_context::<Signal<AppSettings>>();
    let mut now = use_signal(Utc::now);

    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;
            now.set(Utc::now());
        }
    });

    let tz_name = settings.read().timezone.clone();
    let tz: Tz = tz_name.parse().unwrap_or(chrono_tz::UTC);
    let zoned = now.read().with_timezone(&tz);
    let (hh, mm, ss, date_line) = format_clock(&zoned);
    let blink = zoned.second() % 2 == 0;
    let colon_opacity = if blink { 1.0 } else { 0.25 };

    rsx! {
        div { class: "tab wallclock-tab",
            div { class: "clock-display",
                span { class: "clock-digits", "{hh}" }
                span { class: "clock-colon", style: "opacity: {colon_opacity}", ":" }
                span { class: "clock-digits", "{mm}" }
                span { class: "clock-colon", style: "opacity: {colon_opacity}", ":" }
                span { class: "clock-digits", "{ss}" }
            }
            div { class: "date-display", "{date_line}" }
            div { class: "field-row",
                label { r#for: "wallclock-tz", "Timezone" }
                select {
                    id: "wallclock-tz",
                    value: "{tz_name}",
                    onchange: move |evt| {
                        let mut s = settings.write();
                        s.timezone = evt.value();
                        let _ = s.save();
                    },
                    for zone in AVAILABLE_TIMEZONES {
                        option { value: "{zone}", "{zone}" }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn formats_hh_mm_ss_and_date() {
        let dt = chrono_tz::UTC
            .with_ymd_and_hms(2026, 3, 5, 7, 8, 9)
            .unwrap();
        let (hh, mm, ss, date_line) = format_clock(&dt);
        assert_eq!(hh, "07");
        assert_eq!(mm, "08");
        assert_eq!(ss, "09");
        assert_eq!(date_line, "Thursday, 5 March 2026");
    }
}
