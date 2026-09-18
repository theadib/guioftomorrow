use std::time::Duration;

use chrono::{Local, Timelike};
use dioxus::prelude::*;

const SIZE: f64 = 240.0;
const CENTER: f64 = SIZE / 2.0;
const SECOND_RADIUS: f64 = 108.0;
const MINUTE_RADIUS: f64 = 82.0;
const SWEEP_DEG: f64 = 46.0;

/// A point on a circle of radius `r` centred at `(cx, cy)`, measuring `deg`
/// clockwise from 12 o'clock (screen/SVG coordinates, y grows downward).
fn point_on_circle(cx: f64, cy: f64, r: f64, deg: f64) -> (f64, f64) {
    let theta = deg.to_radians();
    (cx + r * theta.sin(), cy - r * theta.cos())
}

/// SVG path `d` for a short comet-trail arc of `sweep_deg` degrees ending at
/// `end_deg` (clockwise from 12 o'clock), so the direction of rotation stays
/// visible at a glance. Pure/unit-testable independent of any timer.
fn arc_path(cx: f64, cy: f64, r: f64, end_deg: f64, sweep_deg: f64) -> String {
    let end_deg = end_deg.rem_euclid(360.0);
    let start_deg = end_deg - sweep_deg;
    let (sx, sy) = point_on_circle(cx, cy, r, start_deg);
    let (ex, ey) = point_on_circle(cx, cy, r, end_deg);
    let large_arc = if sweep_deg > 180.0 { 1 } else { 0 };
    format!("M {sx:.2} {sy:.2} A {r:.2} {r:.2} 0 {large_arc} 1 {ex:.2} {ey:.2}")
}

/// Degrees clockwise from 12 o'clock for the once-per-second arc: a full
/// rotation for every second that ticks over (driven only by the
/// sub-second fraction, so it resets to 0 at the top of every second).
fn second_angle(now: &chrono::DateTime<Local>) -> f64 {
    now.timestamp_subsec_millis() as f64 / 1000.0 * 360.0
}

/// Degrees clockwise from 12 o'clock for the once-per-minute arc: a full
/// rotation for every minute that ticks over (driven by seconds-within-the-
/// current-minute, so it resets to 0 at the top of every minute).
fn minute_angle(now: &chrono::DateTime<Local>) -> f64 {
    (now.second() as f64 + now.timestamp_subsec_millis() as f64 / 1000.0) / 60.0 * 360.0
}

#[component]
pub fn SynctimeTab() -> Element {
    let mut now = use_signal(Local::now);

    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_millis(33)).await;
            now.set(Local::now());
        }
    });

    let zoned = *now.read();
    let second_path = arc_path(
        CENTER,
        CENTER,
        SECOND_RADIUS,
        second_angle(&zoned),
        SWEEP_DEG,
    );
    let minute_path = arc_path(
        CENTER,
        CENTER,
        MINUTE_RADIUS,
        minute_angle(&zoned),
        SWEEP_DEG,
    );
    let time_text = zoned.format("%H:%M:%S").to_string();

    rsx! {
        div { class: "tab synctime-tab",
            div { class: "synctime-canvas",
                svg {
                    view_box: "0 0 {SIZE} {SIZE}",
                    width: "{SIZE}",
                    height: "{SIZE}",
                    circle {
                        cx: "{CENTER}",
                        cy: "{CENTER}",
                        r: "{SECOND_RADIUS}",
                        class: "synctime-ring",
                    }
                    circle {
                        cx: "{CENTER}",
                        cy: "{CENTER}",
                        r: "{MINUTE_RADIUS}",
                        class: "synctime-ring",
                    }
                    path { class: "synctime-arc synctime-arc-second", d: "{second_path}" }
                    path { class: "synctime-arc synctime-arc-minute", d: "{minute_path}" }
                }
                div { class: "synctime-readout", "{time_text}" }
            }
            p { class: "synctime-hint",
                "The outer arc completes one rotation per second, the inner "
                "arc one rotation per minute — run this next to another "
                "instance (or another contester's demo) to compare clocks "
                "visually."
            }
        }
    }
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
    fn arc_path_ends_on_the_requested_point() {
        // At 0 degrees the arc should end exactly at the top of the circle.
        let path = arc_path(100.0, 100.0, 50.0, 0.0, 40.0);
        assert!(path.ends_with("100.00 50.00"));
    }
}
