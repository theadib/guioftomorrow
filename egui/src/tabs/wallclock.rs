use chrono::{DateTime, Timelike, Utc};
use chrono_tz::Tz;
use egui::{Color32, RichText, Ui};

use crate::settings::{AppSettings, AVAILABLE_TIMEZONES};

/// Formats `now` (already converted to the target zone) as `HH`, `MM`, `SS`
/// plus a full date line. Pulled out as a pure function so it's unit
/// testable without driving a timer or a UI.
pub fn format_clock(now: &DateTime<Tz>) -> (String, String, String, String) {
    (
        now.format("%H").to_string(),
        now.format("%M").to_string(),
        now.format("%S").to_string(),
        now.format("%A, %-d %B %Y").to_string(),
    )
}

pub fn ui(ui: &mut Ui, now_utc: DateTime<Utc>, settings: &mut AppSettings) {
    let tz: Tz = settings.timezone.parse().unwrap_or(chrono_tz::UTC);
    let zoned = now_utc.with_timezone(&tz);
    let (hh, mm, ss, date_line) = format_clock(&zoned);

    // The `:` separators blink on/off once a second as a small clock-face
    // animation, driven by the same continuous repaint that updates the
    // digits.
    let blink = zoned.second().is_multiple_of(2);
    let digit = |s: String| RichText::new(s).size(56.0).monospace();
    let colon = || {
        let label = RichText::new(":").size(56.0).monospace();
        if blink {
            label
        } else {
            label.color(Color32::from_rgba_unmultiplied(128, 128, 128, 100))
        }
    };

    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            ui.label(digit(hh));
            ui.label(colon());
            ui.label(digit(mm));
            ui.label(colon());
            ui.label(digit(ss));
        });
        ui.label(RichText::new(date_line).size(18.0));
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            ui.label("Timezone");
            egui::ComboBox::from_id_salt("wallclock_timezone")
                .selected_text(settings.timezone.clone())
                .show_ui(ui, |ui| {
                    for &zone in AVAILABLE_TIMEZONES {
                        let selected = settings.timezone == zone;
                        if ui.selectable_label(selected, zone).clicked() && !selected {
                            settings.timezone = zone.to_string();
                            let _ = settings.save();
                        }
                    }
                });
        });
    });
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
