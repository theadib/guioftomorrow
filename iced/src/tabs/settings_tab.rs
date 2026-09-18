use iced::widget::{column, radio, row, slider, text};
use iced::{Element, Fill};

use crate::settings::{AppSettings, Contrast, ThemeMode};

#[derive(Debug, Clone)]
pub enum Message {
    ThemeModeSelected(ThemeMode),
    ContrastSelected(Contrast),
    ScaleChanged(u32),
}

pub fn update(settings: &mut AppSettings, message: Message) {
    match message {
        Message::ThemeModeSelected(mode) => settings.theme_mode = mode,
        Message::ContrastSelected(contrast) => settings.contrast = contrast,
        Message::ScaleChanged(scale) => settings.ui_scale = scale,
    }
    let _ = settings.save();
}

pub fn view(settings: &AppSettings) -> Element<'_, Message> {
    let theme_row = row![
        radio(
            "System",
            ThemeMode::System,
            Some(settings.theme_mode),
            Message::ThemeModeSelected,
        ),
        radio(
            "Light",
            ThemeMode::Light,
            Some(settings.theme_mode),
            Message::ThemeModeSelected,
        ),
        radio(
            "Dark",
            ThemeMode::Dark,
            Some(settings.theme_mode),
            Message::ThemeModeSelected,
        ),
    ]
    .spacing(16);

    let contrast_row = row![
        radio(
            "Normal",
            Contrast::Normal,
            Some(settings.contrast),
            Message::ContrastSelected,
        ),
        radio(
            "High contrast",
            Contrast::High,
            Some(settings.contrast),
            Message::ContrastSelected,
        ),
    ]
    .spacing(16);

    column![
        text("GUI theme").size(16),
        theme_row,
        text("Contrast").size(16),
        contrast_row,
        text(format!("UI scale — {}%", settings.ui_scale)).size(16),
        slider(50..=200u32, settings.ui_scale, Message::ScaleChanged).step(10u32),
    ]
    .spacing(12)
    .width(Fill)
    .max_width(360)
    .into()
}
