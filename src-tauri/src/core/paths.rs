use std::path::PathBuf;

pub fn get_dot_visualclient_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        dirs::config_dir().unwrap_or_else(|| dirs::home_dir().unwrap().join("AppData").join("Roaming")).join(".visualclient")
    }
    #[cfg(target_os = "macos")]
    {
        dirs::config_dir().unwrap_or_else(|| dirs::home_dir().unwrap().join("Library").join("Application Support")).join("visualclient")
    }
    #[cfg(target_os = "linux")]
    {
        dirs::home_dir().unwrap().join(".visualclient")
    }
}

pub fn get_instances_file() -> PathBuf {
    get_dot_visualclient_dir().join("launcher").join("instances.json")
}
