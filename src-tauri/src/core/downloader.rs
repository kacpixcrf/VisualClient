use std::path::PathBuf;
use std::fs;
use futures_util::{Stream, StreamExt};
use reqwest::Client;
use tauri::{AppHandle, Emitter};

use crate::core::version_manifest::{
    VersionManifest, VersionJson, AssetIndexJson,
    is_library_allowed,
};

const MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
const ASSETS_BASE_URL: &str = "https://resources.download.minecraft.net";
const EVENT_DOWNLOAD_PROGRESS: &str = "download_progress";
const PARALLEL_ASSET_DOWNLOADS: usize = 20;

#[derive(serde::Serialize, Clone)]
pub struct ProgressPayload {
    pub task: String,
    pub progress: u8,
}

pub async fn download_file(client: &Client, url: &str, path: &PathBuf, app: &AppHandle, task_name: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let res = client.get(url).send().await.map_err(|e| e.to_string())?;
    let total_size = res.content_length().unwrap_or(0);
    let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
    let mut stream = res.bytes_stream();
    let mut downloaded: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        use std::io::Write;
        file.write_all(&chunk).map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;
        if total_size > 0 {
            let progress = ((downloaded as f64 / total_size as f64) * 100.0) as u8;
            let _ = app.emit(EVENT_DOWNLOAD_PROGRESS, ProgressPayload { task: task_name.to_string(), progress });
        }
    }
    Ok(())
}

pub async fn download_file_silent(client: &Client, url: &str, path: &PathBuf) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let res = client.get(url).send().await.map_err(|e| e.to_string())?;
    let bytes = res.bytes().await.map_err(|e| e.to_string())?;
    fs::write(path, bytes).map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn download_minecraft(version: &str, mc_dir: &PathBuf, app: &AppHandle) -> Result<(), String> {
    let client = Client::new();
    let manifest: VersionManifest = client.get(MANIFEST_URL)
        .send().await.map_err(|e| e.to_string())?
        .json().await.map_err(|e| e.to_string())?;

    let version_entry = manifest.versions.iter().find(|v| v.id == version)
        .ok_or_else(|| format!("Version {} not found", version))?;

    let version_json_str = client.get(&version_entry.url)
        .send().await.map_err(|e| e.to_string())?
        .text().await.map_err(|e| e.to_string())?;

    let version_json: VersionJson = serde_json::from_str(&version_json_str).map_err(|e| e.to_string())?;

    let version_dir = mc_dir.join("versions").join(version);
    fs::create_dir_all(&version_dir).map_err(|e| e.to_string())?;
    fs::write(version_dir.join(format!("{}.json", version)), &version_json_str).map_err(|e| e.to_string())?;

    let jar_path = version_dir.join(format!("{}.jar", version));
    if !jar_path.exists() {
        download_file(&client, &version_json.downloads.client.url, &jar_path, app, &format!("Downloading {} jar", version)).await?;
    }

    let os_name = if cfg!(target_os = "windows") { "windows" }
                  else if cfg!(target_os = "macos") { "osx" }
                  else { "linux" };

    let natives_classifier = match os_name {
        "windows" => "natives-windows",
        "osx" => "natives-macos",
        _ => "natives-linux",
    };

    let mut library_urls = Vec::new();
    for lib in version_json.libraries {
        if !is_library_allowed(&lib.rules, os_name) { continue; }
        if let Some(artifact) = lib.downloads.artifact {
            library_urls.push((artifact.url, mc_dir.join("libraries").join(artifact.path)));
        }
        if let Some(classifiers) = lib.downloads.classifiers {
            if let Some(artifact) = classifiers.get(natives_classifier) {
                library_urls.push((artifact.url.clone(), mc_dir.join("libraries").join(&artifact.path)));
            }
        }
    }

    let total_libs = library_urls.len();
    for (i, (url, path)) in library_urls.into_iter().enumerate() {
        if !path.exists() && !url.is_empty() {
            let progress = ((i as f64 / total_libs as f64) * 100.0) as u8;
            let _ = app.emit(EVENT_DOWNLOAD_PROGRESS, ProgressPayload {
                task: format!("Downloading libraries ({}/{})", i + 1, total_libs),
                progress,
            });
            download_file_silent(&client, &url, &path).await?;
        }
    }

    let asset_index_path = mc_dir.join("assets").join("indexes").join(format!("{}.json", version_json.asset_index.id));
    if let Some(parent) = asset_index_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let asset_index_str = client.get(&version_json.asset_index.url)
        .send().await.map_err(|e| e.to_string())?
        .text().await.map_err(|e| e.to_string())?;
    fs::write(&asset_index_path, &asset_index_str).map_err(|e| e.to_string())?;

    let asset_index: AssetIndexJson = serde_json::from_str(&asset_index_str).map_err(|e| e.to_string())?;

    let mut asset_urls = Vec::new();
    for (_key, object) in asset_index.objects {
        let hash = object.hash;
        let prefix = &hash[0..2];
        let url = format!("{}/{}/{}", ASSETS_BASE_URL, prefix, hash);
        let path = mc_dir.join("assets").join("objects").join(prefix).join(&hash);
        asset_urls.push((url, path));
    }

    let mut tasks = Vec::new();
    for (i, (url, path)) in asset_urls.into_iter().enumerate() {
        if !path.exists() {
            let client = client.clone();
            tasks.push(async move {
                let _ = download_file_silent(&client, &url, &path).await;
                i
            });
        }
    }

    let mut stream = futures_util::stream::iter(tasks).buffer_unordered(PARALLEL_ASSET_DOWNLOADS);
    let mut completed = 0;
    let total_tasks = stream.size_hint().1.unwrap_or(0);

    if total_tasks > 0 {
        while let Some(_res) = stream.next().await {
            completed += 1;
            let progress = ((completed as f64 / total_tasks as f64) * 100.0) as u8;
            if completed % 50 == 0 || completed == total_tasks {
                let _ = app.emit(EVENT_DOWNLOAD_PROGRESS, ProgressPayload {
                    task: format!("Downloading assets ({}/{})", completed, total_tasks),
                    progress,
                });
            }
        }
    }

    Ok(())
}
