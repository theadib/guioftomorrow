mod settings;
mod tabs;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use chrono::{Local, Utc};
use slint::language::ColorScheme;
use slint::{ComponentHandle, ModelRc, SharedString, Timer, TimerMode, VecModel};

use settings::{AppSettings, Contrast, ThemeMode, AVAILABLE_TIMEZONES};

slint::include_modules!();

fn theme_mode_to_str(mode: ThemeMode) -> &'static str {
    match mode {
        ThemeMode::System => "system",
        ThemeMode::Light => "light",
        ThemeMode::Dark => "dark",
    }
}

fn theme_mode_from_str(s: &str) -> ThemeMode {
    match s {
        "light" => ThemeMode::Light,
        "dark" => ThemeMode::Dark,
        _ => ThemeMode::System,
    }
}

/// Applies `settings`'s theme mode/contrast/scale to the live UI: forces
/// `Palette.color-scheme` for an explicit Light/Dark choice (leaving it at
/// `Unknown` for "System", so Slint keeps following the OS setting on its
/// own), and updates `AppTheme`'s contrast/scale multiplier.
fn apply_theme(app: &AppWindow, settings: &AppSettings) {
    let palette = app.global::<Palette>();
    palette.set_color_scheme(match settings.theme_mode {
        ThemeMode::System => ColorScheme::Unknown,
        ThemeMode::Light => ColorScheme::Light,
        ThemeMode::Dark => ColorScheme::Dark,
    });

    let theme = app.global::<AppTheme>();
    theme.set_high_contrast(settings.contrast == Contrast::High);
    theme.set_scale(settings.ui_scale as f32 / 100.0);
}

/// Refreshes the stopwatch display fields that change on every tick as
/// well as after a user action: the elapsed-time readout and the
/// start/stop-dependent button states.
fn sync_stopwatch_running(app: &AppWindow, state: &tabs::stopwatch::State) {
    let sw = app.global::<Stopwatch>();
    sw.set_elapsed_text(tabs::stopwatch::format_elapsed(state.elapsed()).into());
    sw.set_running(state.is_running());
    sw.set_can_lap(state.is_running());
    sw.set_can_reset(!state.is_running());
}

/// Rebuilds the lap list and the "Export" button's enabled state; only
/// needed after `lap()`/`reset()`, not on every 33ms tick.
fn sync_stopwatch_laps(app: &AppWindow, state: &tabs::stopwatch::State) {
    let sw = app.global::<Stopwatch>();
    let laps: Vec<SharedString> = state
        .laps()
        .iter()
        .enumerate()
        .rev()
        .map(|(index, lap)| {
            format!(
                "Lap {}: {}",
                index + 1,
                tabs::stopwatch::format_elapsed(*lap)
            )
            .into()
        })
        .collect();
    sw.set_laps(ModelRc::new(VecModel::from(laps)));
    sw.set_can_export(!state.laps().is_empty());
}

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    let settings = Rc::new(RefCell::new(AppSettings::load()));
    let stopwatch = Rc::new(RefCell::new(tabs::stopwatch::State::default()));

    apply_theme(&app, &settings.borrow());

    // --- Wallclock ---------------------------------------------------
    {
        let wallclock = app.global::<WallclockModel>();
        let zones: Vec<SharedString> = AVAILABLE_TIMEZONES
            .iter()
            .map(|zone| SharedString::from(*zone))
            .collect();
        wallclock.set_timezones(ModelRc::new(VecModel::from(zones)));
        wallclock.set_selected_timezone(settings.borrow().timezone.clone().into());

        let settings = settings.clone();
        wallclock.on_timezone_selected(move |zone| {
            settings.borrow_mut().timezone = zone.to_string();
            let _ = settings.borrow().save();
        });
    }

    // --- Settings ------------------------------------------------------
    {
        let s = settings.borrow();
        let sm = app.global::<SettingsModel>();
        sm.set_theme_mode(theme_mode_to_str(s.theme_mode).into());
        sm.set_high_contrast(s.contrast == Contrast::High);
        sm.set_ui_scale(s.ui_scale as f32);
        sm.set_ui_scale_label(format!("{}%", s.ui_scale).into());
    }
    {
        let settings = settings.clone();
        let app_weak = app.as_weak();
        app.global::<SettingsModel>()
            .on_theme_mode_selected(move |mode| {
                let Some(app) = app_weak.upgrade() else {
                    return;
                };
                {
                    let mut s = settings.borrow_mut();
                    s.theme_mode = theme_mode_from_str(&mode);
                    let _ = s.save();
                }
                app.global::<SettingsModel>().set_theme_mode(mode);
                apply_theme(&app, &settings.borrow());
            });
    }
    {
        let settings = settings.clone();
        let app_weak = app.as_weak();
        app.global::<SettingsModel>()
            .on_contrast_selected(move |high_contrast| {
                let Some(app) = app_weak.upgrade() else {
                    return;
                };
                {
                    let mut s = settings.borrow_mut();
                    s.contrast = if high_contrast {
                        Contrast::High
                    } else {
                        Contrast::Normal
                    };
                    let _ = s.save();
                }
                app.global::<SettingsModel>()
                    .set_high_contrast(high_contrast);
                apply_theme(&app, &settings.borrow());
            });
    }
    {
        let settings = settings.clone();
        let app_weak = app.as_weak();
        app.global::<SettingsModel>()
            .on_scale_changed(move |value| {
                let Some(app) = app_weak.upgrade() else {
                    return;
                };
                let percent = ((value / 10.0).round() * 10.0).clamp(50.0, 200.0);
                {
                    let mut s = settings.borrow_mut();
                    s.ui_scale = percent as i32;
                    let _ = s.save();
                }
                let sm = app.global::<SettingsModel>();
                sm.set_ui_scale(percent);
                sm.set_ui_scale_label(format!("{}%", percent as i32).into());
                apply_theme(&app, &settings.borrow());
            });
    }

    // --- Stopwatch -------------------------------------------------------
    sync_stopwatch_running(&app, &stopwatch.borrow());
    sync_stopwatch_laps(&app, &stopwatch.borrow());
    {
        let stopwatch = stopwatch.clone();
        let app_weak = app.as_weak();
        app.global::<Stopwatch>().on_toggle(move || {
            let Some(app) = app_weak.upgrade() else {
                return;
            };
            stopwatch.borrow_mut().toggle();
            sync_stopwatch_running(&app, &stopwatch.borrow());
        });
    }
    {
        let stopwatch = stopwatch.clone();
        let app_weak = app.as_weak();
        app.global::<Stopwatch>().on_lap(move || {
            let Some(app) = app_weak.upgrade() else {
                return;
            };
            stopwatch.borrow_mut().lap();
            sync_stopwatch_laps(&app, &stopwatch.borrow());
        });
    }
    {
        let stopwatch = stopwatch.clone();
        let app_weak = app.as_weak();
        app.global::<Stopwatch>().on_reset(move || {
            let Some(app) = app_weak.upgrade() else {
                return;
            };
            stopwatch.borrow_mut().reset();
            sync_stopwatch_running(&app, &stopwatch.borrow());
            sync_stopwatch_laps(&app, &stopwatch.borrow());
            app.global::<Stopwatch>().set_export_message("".into());
        });
    }
    {
        let stopwatch = stopwatch.clone();
        let app_weak = app.as_weak();
        app.global::<Stopwatch>().on_export(move || {
            let Some(app) = app_weak.upgrade() else {
                return;
            };
            let message = stopwatch.borrow().export();
            app.global::<Stopwatch>().set_export_message(message.into());
        });
    }

    // --- One 33ms tick drives the wallclock digits, the synctime arcs and
    // the live stopwatch readout, mirroring the Iced contester's approach
    // of a single always-on subscription rather than a per-tab timer.
    let timer = Timer::default();
    {
        let app_weak = app.as_weak();
        let settings = settings.clone();
        let stopwatch = stopwatch.clone();
        timer.start(TimerMode::Repeated, Duration::from_millis(33), move || {
            let Some(app) = app_weak.upgrade() else {
                return;
            };

            let now_utc = Utc::now();
            let timezone = settings.borrow().timezone.clone();
            let zoned = tabs::wallclock::in_timezone(now_utc, &timezone);
            let (hh, mm, ss, date_line, blink) = tabs::wallclock::format_clock(&zoned);
            let wc = app.global::<WallclockModel>();
            wc.set_hh(hh.into());
            wc.set_mm(mm.into());
            wc.set_ss(ss.into());
            wc.set_date_line(date_line.into());
            wc.set_blink(blink);

            let local_now = Local::now();
            let second_deg = tabs::synctime::second_angle(&local_now);
            let minute_deg = tabs::synctime::minute_angle(&local_now);
            let (sx1, sy1) = tabs::synctime::point_on_circle(
                tabs::synctime::SECOND_RADIUS,
                second_deg - tabs::synctime::SWEEP_DEG,
            );
            let (sx2, sy2) =
                tabs::synctime::point_on_circle(tabs::synctime::SECOND_RADIUS, second_deg);
            let (mx1, my1) = tabs::synctime::point_on_circle(
                tabs::synctime::MINUTE_RADIUS,
                minute_deg - tabs::synctime::SWEEP_DEG,
            );
            let (mx2, my2) =
                tabs::synctime::point_on_circle(tabs::synctime::MINUTE_RADIUS, minute_deg);
            let st = app.global::<Synctime>();
            st.set_second_start_x(sx1);
            st.set_second_start_y(sy1);
            st.set_second_end_x(sx2);
            st.set_second_end_y(sy2);
            st.set_minute_start_x(mx1);
            st.set_minute_start_y(my1);
            st.set_minute_end_x(mx2);
            st.set_minute_end_y(my2);
            st.set_time_text(local_now.format("%H:%M:%S").to_string().into());

            sync_stopwatch_running(&app, &stopwatch.borrow());
        });
    }

    app.run()
}
