use std::path::PathBuf;
use std::fs;
use reqwest::Client;
use tauri::{AppHandle, Emitter};
use futures_util::StreamExt;
use crate::core::downloader::ProgressPayload;

#[tauri::command]
pub async fn fetch_forge_versions() -> Result<String, String> {
    let client = Client::new();
    let res = client.get("https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json")
        .send().await.map_err(|e| e.to_string())?;
    let text = res.text().await.map_err(|e| e.to_string())?;
    Ok(text)
}

pub async fn download_forge(mc_version: &str, forge_version: &str, mc_dir: &PathBuf, java_path: &PathBuf, app: &AppHandle) -> Result<(), String> {
    let client = Client::new();
    let installer_url = format!(
        "https://maven.minecraftforge.net/net/minecraftforge/forge/{}-{}/forge-{}-{}-installer.jar",
        mc_version, forge_version, mc_version, forge_version
    );
    let temp_dir = std::env::temp_dir();
    let temp_installer = temp_dir.join(format!("forge-installer-{}.jar", forge_version));

    let _ = app.emit("download_progress", ProgressPayload { task: "Downloading Forge Installer".to_string(), progress: 0 });

    let res = client.get(&installer_url).send().await.map_err(|e| e.to_string())?;
    let download_res = if res.status().is_success() { res } else {
        let alt_url = format!(
            "https://maven.minecraftforge.net/net/minecraftforge/forge/{}-{}-{}/forge-{}-{}-{}-installer.jar",
            mc_version, forge_version, mc_version, mc_version, forge_version, mc_version
        );
        let res_alt = client.get(&alt_url).send().await.map_err(|e| e.to_string())?;
        if !res_alt.status().is_success() {
            return Err(format!("Failed to find Forge installer for {} {}", mc_version, forge_version));
        }
        res_alt
    };

    let total_size = download_res.content_length().unwrap_or(0);
    let mut file = fs::File::create(&temp_installer).map_err(|e| e.to_string())?;
    let mut stream = download_res.bytes_stream();
    let mut downloaded: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        use std::io::Write;
        file.write_all(&chunk).map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;
        if total_size > 0 {
            let progress = ((downloaded as f64 / total_size as f64) * 100.0) as u8;
            let _ = app.emit("download_progress", ProgressPayload { task: "Downloading Forge Installer".to_string(), progress });
        }
    }

    let _ = app.emit("download_progress", ProgressPayload {
        task: "Running Forge Installer (this may take a while)".to_string(),
        progress: 100,
    });

    let profiles_path = mc_dir.join("launcher_profiles.json");
    if !profiles_path.exists() {
        let _ = std::fs::write(&profiles_path, "{\"profiles\":{}}");
    }

    let mut java_exe = PathBuf::from(java_path);
    if java_exe.file_name().unwrap_or_default() == "javaw.exe" {
        java_exe.set_file_name("java.exe");
    }

    let mut cmd = std::process::Command::new(&java_exe);
    cmd.current_dir(&temp_dir);
    cmd.arg("-jar").arg(&temp_installer).arg("--installClient").arg(mc_dir);

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }

    let output = cmd.output().map_err(|e| format!("Failed to run installer: {}", e))?;
    let _ = fs::remove_file(&temp_installer);
    let _ = fs::remove_file(temp_dir.join(format!("forge-{}-installer.jar.log", forge_version)));
    let _ = fs::remove_file(temp_dir.join("forge-installer.log"));
    let _ = fs::remove_file(temp_dir.join("forge-client.log"));

    if !output.status.success() {
        return Err(format!("Forge installation failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}
