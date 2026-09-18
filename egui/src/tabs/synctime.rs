use chrono::{DateTime, Local, Timelike};
use egui::{Color32, Pos2, Sense, Stroke, Ui, Vec2};

const CANVAS_SIZE: f32 = 240.0;
const SECOND_RADIUS: f32 = 104.0;
const MINUTE_RADIUS: f32 = 78.0;
const SWEEP_DEG: f64 = 46.0;
const ARC_SEGMENTS: usize = 32;

/// Point on a circle of the given `radius` around `center`, for `degrees`
/// clockwise from 12 o'clock (the convention used throughout this module,
/// matching a clock face) — screen space has y growing downward, so 12
/// o'clock (0°) is straight up and 3 o'clock (90°) is to the right.
fn clock_point(center: Pos2, radius: f32, degrees_from_twelve: f64) -> Pos2 {
    let rad = degrees_from_twelve.to_radians();
    Pos2::new(
        center.x + radius * rad.sin() as f32,
        center.y - radius * rad.cos() as f32,
    )
}

/// Samples an open arc from `start_deg` to `end_deg` (clockwise-from-12
/// degrees) into a polyline suitable for `Shape::line`.
fn arc_points(center: Pos2, radius: f32, start_deg: f64, end_deg: f64) -> Vec<Pos2> {
    (0..=ARC_SEGMENTS)
        .map(|i| {
            let t = i as f64 / ARC_SEGMENTS as f64;
            clock_point(center, radius, start_deg + (end_deg - start_deg) * t)
        })
        .collect()
}

/// Degrees clockwise from 12 o'clock for the once-per-second arc: a full
/// rotation for every second that ticks over (driven only by the
/// sub-second fraction, so it resets to 0 at the top of every second).
fn second_angle(now: &DateTime<Local>) -> f64 {
    now.timestamp_subsec_millis() as f64 / 1000.0 * 360.0
}

/// Degrees clockwise from 12 o'clock for the once-per-minute arc: a full
/// rotation for every minute that ticks over (driven by seconds-within-the-
/// current-minute, so it resets to 0 at the top of every minute).
fn minute_angle(now: &DateTime<Local>) -> f64 {
    (now.second() as f64 + now.timestamp_subsec_millis() as f64 / 1000.0) / 60.0 * 360.0
}

pub fn ui(ui: &mut Ui, now: DateTime<Local>) {
    ui.vertical_centered(|ui| {
        let (rect, _response) = ui.allocate_exact_size(Vec2::splat(CANVAS_SIZE), Sense::hover());
        let painter = ui.painter_at(rect);
        let center = rect.center();
        let visuals = ui.visuals();

        for radius in [SECOND_RADIUS, MINUTE_RADIUS] {
            painter.circle_stroke(center, radius, Stroke::new(1.0, visuals.weak_text_color()));
        }

        let second_end = second_angle(&now);
        painter.line(
            arc_points(center, SECOND_RADIUS, second_end - SWEEP_DEG, second_end),
            Stroke::new(6.0, visuals.selection.bg_fill),
        );

        let minute_end = minute_angle(&now);
        painter.line(
            arc_points(center, MINUTE_RADIUS, minute_end - SWEEP_DEG, minute_end),
            Stroke::new(6.0, Color32::from_rgb(0xC8, 0x6E, 0x00)),
        );

        painter.text(
            center,
            egui::Align2::CENTER_CENTER,
            now.format("%H:%M:%S").to_string(),
            egui::FontId::monospace(24.0),
            visuals.text_color(),
        );

        ui.add_space(16.0);
        ui.label(
            "The outer arc completes one rotation per second, the inner arc \
             one rotation per minute — run this next to another instance (or \
             another contester's demo) to compare clocks visually.",
        );
    });
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
    fn twelve_oclock_is_straight_up_from_center() {
        let center = Pos2::new(100.0, 100.0);
        let p = clock_point(center, 50.0, 0.0);
        assert!((p.x - 100.0).abs() < 1e-4);
        assert!((p.y - 50.0).abs() < 1e-4);
    }

    #[test]
    fn three_oclock_is_right_of_center() {
        let center = Pos2::new(100.0, 100.0);
        let p = clock_point(center, 50.0, 90.0);
        assert!((p.x - 150.0).abs() < 1e-4);
        assert!((p.y - 100.0).abs() < 1e-4);
    }

    #[test]
    fn arc_points_starts_and_ends_at_the_requested_angles() {
        let center = Pos2::new(0.0, 0.0);
        let points = arc_points(center, 10.0, 0.0, 90.0);
        assert_eq!(points.len(), ARC_SEGMENTS + 1);
        let first = points.first().unwrap();
        let last = points.last().unwrap();
        assert!((first.x - 0.0).abs() < 1e-4 && (first.y - (-10.0)).abs() < 1e-4);
        assert!((last.x - 10.0).abs() < 1e-4 && (last.y - 0.0).abs() < 1e-4);
    }
}
