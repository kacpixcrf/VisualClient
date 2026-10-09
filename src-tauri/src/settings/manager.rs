use std::fs;
use std::path::PathBuf;
use tauri::Manager;
use super::models::Settings;

pub fn get_settings_path(app: &tauri::AppHandle) -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        app.path().config_dir().unwrap().join(".visualclient").join("launcher").join("settings.json")
    }
    #[cfg(target_os = "macos")]
    {
        app.path().config_dir().unwrap().join("visualclient").join("launcher").join("settings.json")
    }
    #[cfg(target_os = "linux")]
    {
        app.path().home_dir().unwrap().join(".visualclient").join("launcher").join("settings.json")
    }
}

pub fn load_settings(app: &tauri::AppHandle) -> Settings {
    let path = get_settings_path(app);
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(settings) = serde_json::from_str(&content) {
            return settings;
        }
    }
    let default_settings = Settings::default();
    let _ = save_settings(app, &default_settings);
    default_settings
}

pub fn save_settings(app: &tauri::AppHandle, settings: &Settings) -> Result<(), String> {
    let path = get_settings_path(app);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let content = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())?;
    Ok(())
}
