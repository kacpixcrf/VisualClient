use std::fs;
use serde::{Deserialize, Serialize};
use fastnbt::from_bytes;
use flate2::read::GzDecoder;
use std::io::Read;

#[derive(Serialize)]
pub struct WorldItem {
    pub folder_name: String,
    pub name: String,
    pub last_played: i64,
    pub icon_base64: Option<String>,
}

#[derive(Deserialize)]
struct LevelDat {
    #[serde(rename = "Data")]
    data: LevelData,
}

#[derive(Deserialize)]
struct LevelData {
    #[serde(rename = "LevelName")]
    level_name: Option<String>,
    #[serde(rename = "LastPlayed")]
    last_played: Option<i64>,
}

#[tauri::command]
pub fn get_instance_worlds(id: String) -> Result<Vec<WorldItem>, String> {
    let saves_dir = crate::core::paths::get_dot_visualclient_dir()
        .join("profiles").join(&id).join("saves");

    if !saves_dir.exists() { return Ok(Vec::new()); }

    let mut worlds = Vec::new();

    if let Ok(entries) = fs::read_dir(saves_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if !path.is_dir() { continue; }
            let level_dat = path.join("level.dat");
            if !level_dat.exists() { continue; }

            let folder_name = entry.file_name().to_string_lossy().to_string();
            let bytes = fs::read(&level_dat).unwrap_or_default();

            let icon_base64 = {
                let icon_path = path.join("icon.png");
                if icon_path.exists() {
                    if let Ok(b) = fs::read(&icon_path) {
                        use base64::{Engine as _, engine::general_purpose::STANDARD};
                        Some(STANDARD.encode(&b))
                    } else { None }
                } else { None }
            };

            let parse_level = |data: &[u8]| -> Option<WorldItem> {
                from_bytes::<LevelDat>(data).ok().map(|dat| WorldItem {
                    folder_name: folder_name.clone(),
                    name: dat.data.level_name.unwrap_or(folder_name.clone()),
                    last_played: dat.data.last_played.unwrap_or(0),
                    icon_base64: icon_base64.clone(),
                })
            };

            let mut decoder = GzDecoder::new(&bytes[..]);
            let mut uncompressed = Vec::new();
            let world = if decoder.read_to_end(&mut uncompressed).is_ok() {
                parse_level(&uncompressed)
            } else {
                parse_level(&bytes)
            };

            if let Some(w) = world { worlds.push(w); }
        }
    }

    worlds.sort_by(|a, b| b.last_played.cmp(&a.last_played));
    Ok(worlds)
}
