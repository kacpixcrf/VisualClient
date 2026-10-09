use super::models::Settings;
use super::manager;

#[tauri::command]
pub fn get_settings(app: tauri::AppHandle) -> Settings {
    manager::load_settings(&app)
}

#[tauri::command]
pub fn save_settings(app: tauri::AppHandle, settings: Settings) -> Result<(), String> {
    manager::save_settings(&app, &settings)
}
