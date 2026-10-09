use std::path::PathBuf;
use std::fs;
use reqwest::Client;
use tauri::{AppHandle, Emitter};
use crate::core::downloader::{download_file_silent, ProgressPayload};

pub async fn download_fabric(version: &str, mc_dir: &PathBuf, app: &AppHandle) -> Result<(), String> {
    let client = Client::new();
    let loaders_url = format!("https://meta.fabricmc.net/v2/versions/loader/{}", version);
    let loaders_res = client.get(&loaders_url).send().await.map_err(|e| e.to_string())?;
    let loaders: serde_json::Value = loaders_res.json().await.map_err(|e| e.to_string())?;

    if loaders.as_array().map_or(true, |arr| arr.is_empty()) {
        return Err("No fabric loader found for this version".to_string());
    }
    let loader_version = loaders[0]["loader"]["version"].as_str().ok_or("No fabric loader version found")?;

    let profile_url = format!("https://meta.fabricmc.net/v2/versions/loader/{}/{}/profile/json", version, loader_version);
    let profile_str = client.get(&profile_url).send().await.map_err(|e| e.to_string())?.text().await.map_err(|e| e.to_string())?;

    let fabric_version_id = format!("fabric-{}", version);
    let version_dir = mc_dir.join("versions").join(&fabric_version_id);
    fs::create_dir_all(&version_dir).map_err(|e| e.to_string())?;
    fs::write(version_dir.join(format!("{}.json", fabric_version_id)), profile_str.clone()).map_err(|e| e.to_string())?;

    let profile: serde_json::Value = serde_json::from_str(&profile_str).map_err(|e| e.to_string())?;
    if let Some(libraries) = profile["libraries"].as_array() {
        for (i, lib) in libraries.iter().enumerate() {
            let name = lib["name"].as_str().unwrap_or("");
            let url = lib["url"].as_str().unwrap_or("");
            if name.is_empty() { continue; }

            let parts: Vec<&str> = name.split(':').collect();
            if parts.len() >= 3 {
                let group = parts[0].replace('.', "/");
                let artifact = parts[1];
                let lib_version = parts[2];
                let jar_name = format!("{}-{}.jar", artifact, lib_version);
                let path = mc_dir.join("libraries").join(&group).join(artifact).join(lib_version).join(&jar_name);
                if !path.exists() {
                    let dl_url = format!("{}{}/{}/{}/{}", url, group, artifact, lib_version, jar_name);
                    let progress = ((i as f64 / libraries.len() as f64) * 100.0) as u8;
                    let _ = app.emit("download_progress", ProgressPayload {
                        task: format!("Downloading Fabric libraries ({}/{})", i + 1, libraries.len()),
                        progress,
                    });
                    download_file_silent(&client, &dl_url, &path).await?;
                }
            }
        }
    }
    Ok(())
}
