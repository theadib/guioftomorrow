use chrono::{DateTime, Local, Timelike};
use iced::widget::canvas::path::Arc;
use iced::widget::canvas::{self, Frame, LineCap, Path, Stroke};
use iced::widget::{column, container, stack, text};
use iced::{mouse, Center, Element, Fill, Radians, Rectangle, Renderer, Theme};

const CANVAS_SIZE: f32 = 240.0;
const SECOND_RADIUS: f32 = 104.0;
const MINUTE_RADIUS: f32 = 78.0;
const SWEEP_DEG: f64 = 46.0;

#[derive(Debug, Clone)]
pub enum Message {}

/// Converts an angle expressed as degrees clockwise from 12 o'clock (the
/// convention used throughout this module, matching a clock face) into the
/// `Radians` clockwise-from-positive-x-axis convention `canvas::Arc` uses.
fn clock_degrees_to_radians(deg: f64) -> Radians {
    Radians(((deg - 90.0).to_radians()) as f32)
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

struct Arcs {
    now: DateTime<Local>,
}

impl<Message> canvas::Program<Message> for Arcs {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let center = frame.center();
        let palette = theme.extended_palette();

        for radius in [SECOND_RADIUS, MINUTE_RADIUS] {
            frame.stroke(
                &Path::circle(center, radius),
                Stroke::default()
                    .with_width(1.0)
                    .with_color(palette.background.strong.color),
            );
        }

        let second_end = second_angle(&self.now);
        let second_arc = Path::new(|builder| {
            builder.arc(Arc {
                center,
                radius: SECOND_RADIUS,
                start_angle: clock_degrees_to_radians(second_end - SWEEP_DEG),
                end_angle: clock_degrees_to_radians(second_end),
            });
        });
        frame.stroke(
            &second_arc,
            Stroke::default()
                .with_width(6.0)
                .with_color(palette.primary.base.color)
                .with_line_cap(LineCap::Round),
        );

        let minute_end = minute_angle(&self.now);
        let minute_arc = Path::new(|builder| {
            builder.arc(Arc {
                center,
                radius: MINUTE_RADIUS,
                start_angle: clock_degrees_to_radians(minute_end - SWEEP_DEG),
                end_angle: clock_degrees_to_radians(minute_end),
            });
        });
        frame.stroke(
            &minute_arc,
            Stroke::default()
                .with_width(6.0)
                .with_color(palette.secondary.base.color)
                .with_line_cap(LineCap::Round),
        );

        vec![frame.into_geometry()]
    }
}

pub fn view<'a>(now: DateTime<Local>) -> Element<'a, Message> {
    let time_text = now.format("%H:%M:%S").to_string();

    let dial = stack![
        iced::widget::canvas(Arcs { now })
            .width(CANVAS_SIZE)
            .height(CANVAS_SIZE),
        container(text(time_text).size(24)).center(CANVAS_SIZE),
    ]
    .width(CANVAS_SIZE)
    .height(CANVAS_SIZE);

    column![
        dial,
        text(
            "The outer arc completes one rotation per second, the inner arc \
             one rotation per minute — run this next to another instance (or \
             another contester's demo) to compare clocks visually."
        )
        .size(14),
    ]
    .spacing(16)
    .align_x(Center)
    .width(Fill)
    .into()
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
    fn twelve_oclock_maps_to_the_top_of_the_circle() {
        // 12 o'clock (0 degrees clockwise from 12) is "straight up", which
        // in canvas::Arc's clockwise-from-positive-x-axis convention is
        // -90 degrees (-PI/2 radians).
        let radians = clock_degrees_to_radians(0.0);
        assert!((radians.0 - (-std::f32::consts::FRAC_PI_2)).abs() < 1e-6);
    }

    #[test]
    fn three_oclock_maps_to_the_positive_x_axis() {
        let radians = clock_degrees_to_radians(90.0);
        assert!(radians.0.abs() < 1e-6);
    }
}
