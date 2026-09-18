use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    System,
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Contrast {
    Normal,
    High,
}

/// Curated list of well-known IANA timezones, mirroring the Flutter and
/// Dioxus contesters' approach of offering a manageable subset rather than
/// the full IANA database.
pub const AVAILABLE_TIMEZONES: &[&str] = &[
    "UTC",
    "Europe/Berlin",
    "Europe/London",
    "Europe/Moscow",
    "America/New_York",
    "America/Chicago",
    "America/Los_Angeles",
    "America/Sao_Paulo",
    "Asia/Tokyo",
    "Asia/Shanghai",
    "Asia/Kolkata",
    "Asia/Dubai",
    "Australia/Sydney",
    "Pacific/Auckland",
    "Africa/Johannesburg",
];

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme_mode: ThemeMode,
    pub contrast: Contrast,
    /// UI scale as a percentage (50..=200), applied as the window's scale
    /// factor.
    pub ui_scale: u32,
    pub timezone: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::System,
            contrast: Contrast::Normal,
            ui_scale: 100,
            timezone: "UTC".to_string(),
        }
    }
}

fn app_dir(base: Option<PathBuf>) -> PathBuf {
    base.unwrap_or_else(std::env::temp_dir)
        .join("gui-of-tomorrow-iced")
}

fn config_file() -> PathBuf {
    app_dir(dirs::config_dir()).join("settings.json")
}

/// Directory the stopwatch tab exports lap reports into.
pub fn export_dir() -> PathBuf {
    app_dir(dirs::data_dir()).join("exports")
}

impl AppSettings {
    pub fn load() -> Self {
        let path = config_file();
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|contents| serde_json::from_str(&contents).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = config_file();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self).expect("AppSettings is always serializable");
        std::fs::write(path, json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_are_sane() {
        let settings = AppSettings::default();
        assert_eq!(settings.theme_mode, ThemeMode::System);
        assert_eq!(settings.contrast, Contrast::Normal);
        assert_eq!(settings.ui_scale, 100);
        assert_eq!(settings.timezone, "UTC");
    }

    #[test]
    fn round_trips_through_json() {
        let settings = AppSettings {
            theme_mode: ThemeMode::Dark,
            contrast: Contrast::High,
            ui_scale: 150,
            timezone: "Asia/Tokyo".to_string(),
        };
        let json = serde_json::to_string(&settings).unwrap();
        let back: AppSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, back);
    }

    #[test]
    fn all_curated_timezones_parse() {
        for name in AVAILABLE_TIMEZONES {
            assert!(
                name.parse::<chrono_tz::Tz>().is_ok(),
                "{name} should be a valid IANA timezone name"
            );
        }
    }
}
