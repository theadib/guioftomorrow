use chrono::{DateTime, Utc};
use chrono_tz::Tz;

/// Formats `now` (already converted to the target zone) as `HH`, `MM`, `SS`
/// plus a full date line, and whether the `:` separators should currently
/// be shown (they blink on/off once a second). Pulled out as a pure
/// function so it's unit testable without driving a timer or the UI.
pub fn format_clock(now: &DateTime<Tz>) -> (String, String, String, String, bool) {
    use chrono::Timelike;
    (
        now.format("%H").to_string(),
        now.format("%M").to_string(),
        now.format("%S").to_string(),
        now.format("%A, %-d %B %Y").to_string(),
        now.second().is_multiple_of(2),
    )
}

/// Converts a UTC instant into the named IANA timezone, falling back to UTC
/// for an unrecognized/invalid name.
pub fn in_timezone(now_utc: DateTime<Utc>, timezone: &str) -> DateTime<Tz> {
    let tz: Tz = timezone.parse().unwrap_or(chrono_tz::UTC);
    now_utc.with_timezone(&tz)
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
        let (hh, mm, ss, date_line, _blink) = format_clock(&dt);
        assert_eq!(hh, "07");
        assert_eq!(mm, "08");
        assert_eq!(ss, "09");
        assert_eq!(date_line, "Thursday, 5 March 2026");
    }

    #[test]
    fn blink_toggles_every_second() {
        let even = chrono_tz::UTC
            .with_ymd_and_hms(2026, 3, 5, 7, 8, 8)
            .unwrap();
        let odd = chrono_tz::UTC
            .with_ymd_and_hms(2026, 3, 5, 7, 8, 9)
            .unwrap();
        assert!(format_clock(&even).4);
        assert!(!format_clock(&odd).4);
    }

    #[test]
    fn unknown_timezone_falls_back_to_utc() {
        let now = Utc::now();
        let zoned = in_timezone(now, "Not/AZone");
        assert_eq!(zoned.timezone(), chrono_tz::UTC);
    }
}
