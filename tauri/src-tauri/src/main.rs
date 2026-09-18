// Suppress the extra Windows console window on release builds; a no-op on
// Linux/macOS.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod settings;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::available_timezones,
            commands::load_settings,
            commands::save_settings,
            commands::export_stopwatch,
        ])
        .run(tauri::generate_context!())
        .expect("error while running the Tauri application");
}
