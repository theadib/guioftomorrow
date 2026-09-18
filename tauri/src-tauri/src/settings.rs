use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    System,
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Contrast {
    Normal,
    High,
}

/// Curated list of well-known IANA timezones, mirroring the other Rust
/// contesters' approach of offering a manageable subset rather than the
/// full IANA database. Exposed to the frontend via the `available_timezones`
/// command so the dropdown and the persisted value share one source of
/// truth; validity is enforced by the browser's own `Intl.DateTimeFormat`,
/// which is also what renders each zone's wallclock time.
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
    /// UI scale as a percentage (50..=200).
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

fn config_file(config_dir: &Path) -> PathBuf {
    config_dir.join("settings.json")
}

impl AppSettings {
    pub fn load(config_dir: &Path) -> Self {
        std::fs::read_to_string(config_file(config_dir))
            .ok()
            .and_then(|contents| serde_json::from_str(&contents).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, config_dir: &Path) -> std::io::Result<()> {
        std::fs::create_dir_all(config_dir)?;
        let json = serde_json::to_string_pretty(self).expect("AppSettings is always serializable");
        std::fs::write(config_file(config_dir), json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("gui-of-tomorrow-tauri-test-{name}"))
    }

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
    fn missing_settings_file_falls_back_to_default() {
        let dir = temp_dir("missing");
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(AppSettings::load(&dir), AppSettings::default());
    }

    #[test]
    fn save_then_load_round_trips_on_disk() {
        let dir = temp_dir("roundtrip");
        let _ = std::fs::remove_dir_all(&dir);
        let settings = AppSettings {
            theme_mode: ThemeMode::Light,
            contrast: Contrast::High,
            ui_scale: 75,
            timezone: "Europe/Berlin".to_string(),
        };
        settings.save(&dir).unwrap();
        assert_eq!(AppSettings::load(&dir), settings);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
