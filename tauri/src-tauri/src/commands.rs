use tauri::{AppHandle, Manager};

use crate::settings::{AppSettings, AVAILABLE_TIMEZONES};

/// Keeps only characters safe for a single path segment, so a filename
/// coming from the frontend can never escape the export directory (no `/`,
/// `..`, drive letters, etc.).
fn sanitize_filename(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        .collect();
    if cleaned.is_empty() || cleaned == "." || cleaned == ".." {
        "stopwatch-export.txt".to_string()
    } else {
        cleaned
    }
}

#[tauri::command]
pub fn available_timezones() -> Vec<&'static str> {
    AVAILABLE_TIMEZONES.to_vec()
}

#[tauri::command]
pub fn load_settings(app: AppHandle) -> AppSettings {
    match app.path().app_config_dir() {
        Ok(dir) => AppSettings::load(&dir),
        Err(_) => AppSettings::default(),
    }
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: AppSettings) -> Result<(), String> {
    let dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    settings.save(&dir).map_err(|e| e.to_string())
}

/// Writes the stopwatch lap report (already formatted by the frontend) to
/// the OS data directory and returns the full path it was written to, for
/// display in the UI.
#[tauri::command]
pub fn export_stopwatch(
    app: AppHandle,
    filename: String,
    contents: String,
) -> Result<String, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("exports");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join(sanitize_filename(&filename));
    std::fs::write(&path, contents).map_err(|e| e.to_string())?;
    Ok(path.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_strips_path_traversal() {
        assert_eq!(sanitize_filename("../../etc/passwd"), "....etcpasswd");
        assert_eq!(
            sanitize_filename("stopwatch-20260101.txt"),
            "stopwatch-20260101.txt"
        );
    }

    #[test]
    fn sanitize_falls_back_when_nothing_left() {
        assert_eq!(sanitize_filename("///"), "stopwatch-export.txt");
        assert_eq!(sanitize_filename(".."), "stopwatch-export.txt");
    }
}
