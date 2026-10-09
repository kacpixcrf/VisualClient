use std::path::{Path, PathBuf};
use std::fs;
use tauri::{AppHandle, Emitter};
use futures_util::StreamExt;

#[derive(serde::Serialize, Clone)]
struct ProgressPayload {
    task: String,
    progress: u8,
}

pub async fn download_java(version: u32, target_dir: &PathBuf, app: &AppHandle) -> Result<PathBuf, String> {
    let extract_dir = target_dir.join(format!("jdk{}", version));
    if extract_dir.exists() {
        if let Ok(exe_path) = find_java_executable(&extract_dir) {
            let mut cmd = std::process::Command::new(&exe_path);
            cmd.env_remove("JAVA_HOME").env_remove("PATH").env_remove("Path").arg("-version");
            #[cfg(target_os = "windows")]
            {
                use std::os::windows::process::CommandExt;
                cmd.creation_flags(0x08000000);
            }
            if let Ok(output) = cmd.output() {
                let output_str = String::from_utf8_lossy(&output.stderr);
                if output.status.success() && output_str.contains("version") {
                    let _ = app.emit("download_progress", ProgressPayload {
                        task: format!("Using existing Java {}", version),
                        progress: 100,
                    });
                    return Ok(exe_path);
                }
            }
        }
        let _ = fs::remove_dir_all(&extract_dir);
    }

    let os = if cfg!(target_os = "windows") { "windows" }
             else if cfg!(target_os = "macos") { "mac" }
             else { "linux" };
    let arch = if cfg!(target_arch = "x86_64") { "x64" }
               else if cfg!(target_arch = "aarch64") { "aarch64" }
               else { "x64" };

    let mut url = format!("https://api.adoptium.net/v3/binary/latest/{}/ga/{}/{}/jre/hotspot/normal/eclipse", version, os, arch);
    let mut res = reqwest::get(&url).await.map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        url = format!("https://api.adoptium.net/v3/binary/latest/{}/ga/{}/{}/jdk/hotspot/normal/eclipse", version, os, arch);
        res = reqwest::get(&url).await.map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            url = format!("https://api.adoptium.net/v3/binary/latest/21/ga/{}/{}/jre/hotspot/normal/eclipse", os, arch);
            res = reqwest::get(&url).await.map_err(|e| e.to_string())?;
            if !res.status().is_success() {
                return Err(format!("Failed to find Java version {} on Adoptium", version));
            }
        }
    }

    let total_size = res.content_length().unwrap_or(0);
    let is_zip = os == "windows" || res.url().path().ends_with(".zip");
    let tmp_file = target_dir.join(format!("java_download_{}.archive", version));
    let mut file = std::fs::File::create(&tmp_file).map_err(|e| e.to_string())?;
    let mut stream = res.bytes_stream();
    let mut downloaded: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        use std::io::Write;
        file.write_all(&chunk).map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;
        if total_size > 0 {
            let progress = ((downloaded as f64 / total_size as f64) * 100.0) as u8;
            let _ = app.emit("download_progress", ProgressPayload {
                task: format!("Downloading Java {}", version),
                progress,
            });
        }
    }

    let _ = app.emit("download_progress", ProgressPayload {
        task: format!("Extracting Java {}", version),
        progress: 100,
    });

    let extract_dir = target_dir.join(format!("jdk{}", version));
    if extract_dir.exists() {
        fs::remove_dir_all(&extract_dir).map_err(|e| e.to_string())?;
    }
    fs::create_dir_all(&extract_dir).map_err(|e| e.to_string())?;

    if is_zip {
        let file = std::fs::File::open(&tmp_file).map_err(|e| e.to_string())?;
        let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
            let outpath = match file.enclosed_name() {
                Some(path) => {
                    let mut components = path.components();
                    components.next();
                    let stripped = components.as_path();
                    if stripped.as_os_str().is_empty() { continue; }
                    extract_dir.join(stripped)
                },
                None => continue,
            };
            if file.name().map_or(true, |n| n.ends_with('/')) {
                fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() { fs::create_dir_all(p).map_err(|e| e.to_string())?; }
                }
                let mut outfile = fs::File::create(&outpath).map_err(|e| e.to_string())?;
                std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
            }
        }
    } else {
        let file = std::fs::File::open(&tmp_file).map_err(|e| e.to_string())?;
        let tar = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(tar);
        for file in archive.entries().map_err(|e| e.to_string())? {
            let mut file = file.map_err(|e| e.to_string())?;
            let path = file.path().map_err(|e| e.to_string())?.into_owned();
            let mut components = path.components();
            components.next();
            let stripped = components.as_path();
            if stripped.as_os_str().is_empty() { continue; }
            let outpath = extract_dir.join(stripped);
            if file.header().entry_type().is_dir() {
                fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() { fs::create_dir_all(p).map_err(|e| e.to_string())?; }
                }
                file.unpack(&outpath).map_err(|e| e.to_string())?;
            }
        }
    }

    let _ = fs::remove_file(&tmp_file);
    find_java_executable(&extract_dir)
}

pub fn find_java_executable(dir: &Path) -> Result<PathBuf, String> {
    for entry in walkdir::WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let name = entry.file_name().to_string_lossy();
        if name == "java.exe" || name == "java" {
            if entry.path().parent().map(|p| p.ends_with("bin")).unwrap_or(false) {
                return Ok(entry.path().to_path_buf());
            }
        }
    }
    Err("Could not find java executable in extracted archive".to_string())
}
