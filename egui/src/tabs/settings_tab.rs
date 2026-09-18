use egui::Ui;

use crate::settings::{AppSettings, Contrast, ThemeMode};

pub fn ui(ui: &mut Ui, settings: &mut AppSettings) {
    ui.set_max_width(360.0);
    let mut changed = false;

    ui.label("GUI theme");
    ui.horizontal(|ui| {
        changed |= ui
            .radio_value(&mut settings.theme_mode, ThemeMode::System, "System")
            .changed();
        changed |= ui
            .radio_value(&mut settings.theme_mode, ThemeMode::Light, "Light")
            .changed();
        changed |= ui
            .radio_value(&mut settings.theme_mode, ThemeMode::Dark, "Dark")
            .changed();
    });

    ui.add_space(8.0);
    ui.label("Contrast");
    ui.horizontal(|ui| {
        changed |= ui
            .radio_value(&mut settings.contrast, Contrast::Normal, "Normal")
            .changed();
        changed |= ui
            .radio_value(&mut settings.contrast, Contrast::High, "High contrast")
            .changed();
    });

    ui.add_space(8.0);
    ui.label(format!("UI scale — {}%", settings.ui_scale));
    changed |= ui
        .add(egui::Slider::new(&mut settings.ui_scale, 50..=200).step_by(10.0))
        .changed();

    if changed {
        let _ = settings.save();
    }
}
