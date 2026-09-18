use chrono::{DateTime, Timelike, Utc};
use chrono_tz::Tz;
use iced::widget::{column, pick_list, row, text};
use iced::{Center, Element};

use crate::settings::AVAILABLE_TIMEZONES;

#[derive(Debug, Clone)]
pub enum Message {
    TimezoneSelected(&'static str),
}

/// Formats `now` (already converted to the target zone) as `HH`, `MM`, `SS`
/// plus a full date line. Pulled out as a pure function so it's unit
/// testable without driving a timer.
pub fn format_clock(now: &DateTime<Tz>) -> (String, String, String, String) {
    (
        now.format("%H").to_string(),
        now.format("%M").to_string(),
        now.format("%S").to_string(),
        now.format("%A, %-d %B %Y").to_string(),
    )
}

pub fn view<'a>(now_utc: DateTime<Utc>, timezone: &str) -> Element<'a, Message> {
    let tz: Tz = timezone.parse().unwrap_or(chrono_tz::UTC);
    let zoned = now_utc.with_timezone(&tz);
    let (hh, mm, ss, date_line) = format_clock(&zoned);

    // The `:` separators blink on/off once a second as a small clock-face
    // animation, driven by the same once-a-second re-render as the digits.
    let blink = zoned.second().is_multiple_of(2);
    let colon = || {
        let label = text(":").size(56);
        if blink {
            label
        } else {
            label.color(iced::Color::from_rgba(0.5, 0.5, 0.5, 0.4))
        }
    };

    let clock = row![
        text(hh).size(56),
        colon(),
        text(mm).size(56),
        colon(),
        text(ss).size(56),
    ]
    .align_y(Center);

    let selected: &'static str = AVAILABLE_TIMEZONES
        .iter()
        .copied()
        .find(|zone| *zone == timezone)
        .unwrap_or("UTC");

    column![
        clock,
        text(date_line).size(18),
        row![
            text("Timezone"),
            pick_list(
                AVAILABLE_TIMEZONES,
                Some(selected),
                Message::TimezoneSelected
            ),
        ]
        .spacing(8)
        .align_y(Center),
    ]
    .spacing(16)
    .align_x(Center)
    .into()
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
