use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Emitter};

use crate::core::paths::{get_dot_visualclient_dir, get_instances_file};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub loader: String,
    pub version: String,
    pub java_path: String,
    pub icon_path: Option<String>,
}

#[derive(Serialize, Clone)]
struct ProgressPayload {
    task: String,
    progress: u8,
}

pub fn get_instances() -> Result<Vec<Instance>, String> {
    let file_path = get_instances_file();
    let content = crate::core::crypto::read_encrypted_file(&file_path)?;
    if content.is_empty() {
        return Ok(Vec::new());
    }
    let instances: Vec<Instance> = serde_json::from_str(&content).unwrap_or_default();
    Ok(instances)
}

#[tauri::command]
pub fn get_instances_cmd() -> Result<Vec<Instance>, String> {
    get_instances()
}

#[tauri::command]
pub async fn create_instance(
    app: AppHandle,
    name: String,
    loader: String,
    version: String,
    java_version: u32,
    icon_path: Option<String>,
    folder_name: Option<String>,
) -> Result<(), String> {
    let vc_dir = get_dot_visualclient_dir();
    let base_id = folder_name.unwrap_or_else(|| name.clone());
    let mut id = base_id.clone();
    let mut profiles_dir = vc_dir.join("profiles").join(&id);

    let mut i = 1;
    while profiles_dir.exists() {
        id = format!("{} ({})", base_id, i);
        profiles_dir = vc_dir.join("profiles").join(&id);
        i += 1;
    }

    fs::create_dir_all(&profiles_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(vc_dir.join("minecraft")).map_err(|e| e.to_string())?;

    let mut final_icon_path = None;
    if let Some(path) = icon_path {
        let ext = Path::new(&path).extension().unwrap_or_default().to_string_lossy().to_string();
        let target_icon = profiles_dir.join(format!("icon.{}", ext));
        if fs::copy(&path, &target_icon).is_ok() {
            final_icon_path = Some(target_icon.to_string_lossy().to_string());
        }
    }

    let java_dir = vc_dir.join("java");
    fs::create_dir_all(&java_dir).map_err(|e| e.to_string())?;

    let java_path = match crate::core::java::download_java(java_version, &java_dir, &app).await {
        Ok(path) => path,
        Err(e) => {
            let _ = app.emit("download_progress", ProgressPayload { task: format!("Error: {}", e), progress: 0 });
            return Err(e);
        }
    };

    let mc_dir_clone = vc_dir.join("minecraft");
    let app_clone = app.clone();
    let version_clone = version.clone();
    let loader_clone = loader.clone();
    let java_path_clone = java_path.clone();

    let mc_v = version_clone.clone();
    let mut forge_v = None;
    let mut final_loader = loader_clone.clone();

    if loader_clone == "forge" {
        let client = reqwest::Client::new();
        let promos_url = "https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json";
        if let Ok(res) = client.get(promos_url).send().await {
            if let Ok(json) = res.json::<serde_json::Value>().await {
                if let Some(promos) = json.get("promos") {
                    let recommended_key = format!("{}-recommended", mc_v);
                    let latest_key = format!("{}-latest", mc_v);
                    if let Some(ver) = promos.get(&recommended_key).or_else(|| promos.get(&latest_key)) {
                        if let Some(ver_str) = ver.as_str() {
                            forge_v = Some(ver_str.to_string());
                            final_loader = format!("forge-{}", ver_str);
                        }
                    }
                }
            }
        }
        if forge_v.is_none() {
            return Err(format!("Could not find a valid Forge version for Minecraft {}", mc_v));
        }
    }

    tokio::spawn(async move {
        crate::core::downloader::download_minecraft(&mc_v, &mc_dir_clone, &app_clone).await?;
        if loader_clone == "fabric" {
            crate::core::fabric::download_fabric(&mc_v, &mc_dir_clone, &app_clone).await?;
        } else if loader_clone == "forge" {
            if let Some(fv) = forge_v {
                crate::core::forge::download_forge(&mc_v, &fv, &mc_dir_clone, &java_path_clone, &app_clone).await?;
            }
        }
        Ok::<(), String>(())
    }).await.map_err(|e| e.to_string())??;

    fs::write(profiles_dir.join("options.txt"), "").map_err(|e| e.to_string())?;

    let new_instance = Instance {
        id,
        name,
        loader: final_loader,
        version: version.clone(),
        java_path: java_path.to_string_lossy().to_string(),
        icon_path: final_icon_path,
    };

    let mut instances = get_instances().unwrap_or_default();
    instances.push(new_instance);

    crate::core::crypto::write_encrypted_file(
        &get_instances_file(),
        &serde_json::to_string_pretty(&instances).map_err(|e| e.to_string())?
    )?;

    let _ = app.emit("download_progress", ProgressPayload {
        task: "Done".to_string(),
        progress: 100,
    });

    Ok(())
}

#[tauri::command]
pub fn rename_instance(id: String, new_name: String) -> Result<(), String> {
    let mut instances = get_instances().unwrap_or_default();
    if let Some(inst) = instances.iter_mut().find(|i| i.id == id) {
        inst.name = new_name;
        crate::core::crypto::write_encrypted_file(
            &get_instances_file(),
            &serde_json::to_string_pretty(&instances).map_err(|e| e.to_string())?
        )?;
    }
    Ok(())
}

#[tauri::command]
pub fn delete_instance(id: String) -> Result<(), String> {
    let mut instances = get_instances().unwrap_or_default();
    instances.retain(|i| i.id != id);
    crate::core::crypto::write_encrypted_file(
        &get_instances_file(),
        &serde_json::to_string_pretty(&instances).map_err(|e| e.to_string())?
    )?;

    let profiles_dir = get_dot_visualclient_dir().join("profiles").join(&id);
    if profiles_dir.exists() {
        let _ = fs::remove_dir_all(&profiles_dir);
    }
    Ok(())
}

#[tauri::command]
pub fn open_instance_folder(id: String) -> Result<(), String> {
    let profiles_dir = get_dot_visualclient_dir().join("profiles").join(&id);
    if !profiles_dir.exists() {
        fs::create_dir_all(&profiles_dir).map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "windows")]
    { std::process::Command::new("explorer").arg(&profiles_dir).spawn().map_err(|e| e.to_string())?; }
    #[cfg(target_os = "macos")]
    { std::process::Command::new("open").arg(&profiles_dir).spawn().map_err(|e| e.to_string())?; }
    #[cfg(target_os = "linux")]
    { std::process::Command::new("xdg-open").arg(&profiles_dir).spawn().map_err(|e| e.to_string())?; }

    Ok(())
}
