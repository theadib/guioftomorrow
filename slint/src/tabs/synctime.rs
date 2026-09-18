use chrono::{DateTime, Local, Timelike};

/// Radii below are in the same viewBox units as the `viewbox-width`/
/// `viewbox-height: 240` set on the `Path` elements in `ui/synctime.slint`
/// (a 240-unit square centered on the origin).
pub const SECOND_RADIUS: f32 = 104.0;
pub const MINUTE_RADIUS: f32 = 78.0;
pub const SWEEP_DEG: f64 = 46.0;

/// Degrees clockwise from 12 o'clock for the once-per-second arc: a full
/// rotation for every second that ticks over (driven only by the
/// sub-second fraction, so it resets to 0 at the top of every second).
pub fn second_angle(now: &DateTime<Local>) -> f64 {
    now.timestamp_subsec_millis() as f64 / 1000.0 * 360.0
}

/// Degrees clockwise from 12 o'clock for the once-per-minute arc: a full
/// rotation for every minute that ticks over (driven by seconds-within-the-
/// current-minute, so it resets to 0 at the top of every minute).
pub fn minute_angle(now: &DateTime<Local>) -> f64 {
    (now.second() as f64 + now.timestamp_subsec_millis() as f64 / 1000.0) / 60.0 * 360.0
}

/// Converts an angle expressed as degrees clockwise from 12 o'clock (the
/// convention used by [`second_angle`]/[`minute_angle`], matching a clock
/// face) into an (x, y) point on a circle of the given `radius` centered on
/// the viewBox origin, in the y-axis-points-down path-coordinate space
/// `ui/synctime.slint`'s `Path` elements use.
pub fn point_on_circle(radius: f32, clock_deg: f64) -> (f32, f32) {
    let theta = (clock_deg - 90.0).to_radians();
    (radius * theta.cos() as f32, radius * theta.sin() as f32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn second_angle_is_zero_at_top_of_second() {
        let dt = Local.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        assert_eq!(second_angle(&dt), 0.0);
    }

    #[test]
    fn second_angle_is_half_rotation_mid_second() {
        let dt = Local
            .with_ymd_and_hms(2026, 1, 1, 12, 0, 0)
            .unwrap()
            .with_nanosecond(500_000_000)
            .unwrap();
        assert!((second_angle(&dt) - 180.0).abs() < 1e-9);
    }

    #[test]
    fn second_angle_resets_each_second_regardless_of_the_minute() {
        let dt = Local
            .with_ymd_and_hms(2026, 1, 1, 12, 0, 42)
            .unwrap()
            .with_nanosecond(500_000_000)
            .unwrap();
        assert!((second_angle(&dt) - 180.0).abs() < 1e-9);
    }

    #[test]
    fn minute_angle_is_half_rotation_at_30s() {
        let dt = Local.with_ymd_and_hms(2026, 1, 1, 12, 0, 30).unwrap();
        assert!((minute_angle(&dt) - 180.0).abs() < 1e-9);
    }

    #[test]
    fn minute_angle_resets_each_minute_regardless_of_the_hour() {
        let dt = Local.with_ymd_and_hms(2026, 1, 1, 12, 45, 30).unwrap();
        assert!((minute_angle(&dt) - 180.0).abs() < 1e-9);
    }

    #[test]
    fn twelve_oclock_is_straight_up() {
        // 12 o'clock (0 degrees clockwise from 12) is "straight up", which
        // in a y-down coordinate space is negative y, zero x.
        let (x, y) = point_on_circle(100.0, 0.0);
        assert!(x.abs() < 1e-4);
        assert!((y - (-100.0)).abs() < 1e-4);
    }

    #[test]
    fn three_oclock_is_straight_right() {
        let (x, y) = point_on_circle(100.0, 90.0);
        assert!((x - 100.0).abs() < 1e-4);
        assert!(y.abs() < 1e-4);
    }

    #[test]
    fn six_oclock_is_straight_down() {
        let (x, y) = point_on_circle(100.0, 180.0);
        assert!(x.abs() < 1e-4);
        assert!((y - 100.0).abs() < 1e-4);
    }
}
