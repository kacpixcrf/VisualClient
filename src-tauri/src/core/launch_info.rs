use std::path::PathBuf;
use std::fs;
use crate::core::version_manifest::{VersionJson, is_library_allowed};

pub struct LaunchInfo {
    pub classpath: String,
    pub asset_index: String,
    pub main_class: String,
    pub jvm_args: Vec<String>,
    pub game_args: Vec<String>,
}

pub fn get_launch_info(version: &str, loader: &str, mc_dir: &PathBuf) -> Result<LaunchInfo, String> {
    let version_json_path = mc_dir.join("versions").join(version).join(format!("{}.json", version));
    let version_json_str = fs::read_to_string(&version_json_path)
        .map_err(|e| format!("Could not read version JSON: {}", e))?;
    let version_json: VersionJson = serde_json::from_str(&version_json_str).map_err(|e| e.to_string())?;

    let os_name = if cfg!(target_os = "windows") { "windows" }
                  else if cfg!(target_os = "macos") { "osx" }
                  else { "linux" };

    let natives_classifier = match os_name {
        "windows" => "natives-windows",
        "osx" => "natives-macos",
        _ => "natives-linux",
    };

    let mut cp_entries = Vec::new();
    let natives_dir = mc_dir.join("versions").join(version).join("natives");
    let _ = fs::create_dir_all(&natives_dir);

    for lib in version_json.libraries {
        if !is_library_allowed(&lib.rules, os_name) { continue; }

        if let Some(artifact) = lib.downloads.artifact {
            let path = mc_dir.join("libraries").join(&artifact.path);
            cp_entries.push(path.to_string_lossy().into_owned());
        }

        if let Some(classifiers) = lib.downloads.classifiers {
            if let Some(artifact) = classifiers.get(natives_classifier) {
                let path = mc_dir.join("libraries").join(&artifact.path);
                if path.exists() {
                    if let Ok(file) = std::fs::File::open(&path) {
                        if let Ok(mut archive) = zip::ZipArchive::new(file) {
                            for i in 0..archive.len() {
                                if let Ok(mut file) = archive.by_index(i) {
                                    if let Ok(name_str) = file.name() {
                                        if !file.is_dir() && !name_str.starts_with("META-INF") {
                                            if let Some(name) = std::path::Path::new(name_str.as_ref()).file_name() {
                                                let outpath = natives_dir.join(name);
                                                if let Ok(mut outfile) = std::fs::File::create(&outpath) {
                                                    let _ = std::io::copy(&mut file, &mut outfile);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let jar_path = mc_dir.join("versions").join(version).join(format!("{}.jar", version));
    cp_entries.push(jar_path.to_string_lossy().into_owned());

    let mut main_class = "net.minecraft.client.main.Main".to_string();
    let mut jvm_args = Vec::new();
    let mut game_args = Vec::new();

    if loader == "fabric" {
        let fabric_version_id = format!("fabric-{}", version);
        let fabric_json_path = mc_dir.join("versions").join(&fabric_version_id).join(format!("{}.json", fabric_version_id));
        if let Ok(fabric_json_str) = fs::read_to_string(&fabric_json_path) {
            if let Ok(fabric_json) = serde_json::from_str::<serde_json::Value>(&fabric_json_str) {
                if let Some(mc) = fabric_json["mainClass"].as_str() {
                    main_class = mc.to_string();
                }
                if let Some(libraries) = fabric_json["libraries"].as_array() {
                    for lib in libraries {
                        let name = lib["name"].as_str().unwrap_or("");
                        let parts: Vec<&str> = name.split(':').collect();
                        if parts.len() >= 3 {
                            let group = parts[0].replace('.', "/");
                            let artifact = parts[1];
                            let lib_version = parts[2];
                            let jar_name = format!("{}-{}.jar", artifact, lib_version);
                            let path = mc_dir.join("libraries").join(&group).join(artifact).join(lib_version).join(&jar_name);
                            cp_entries.push(path.to_string_lossy().into_owned());
                        }
                    }
                }
            }
        }
    } else if loader.starts_with("forge-") {
        let forge_ver = loader.strip_prefix("forge-").unwrap();
        let mut forge_id = format!("{}-forge-{}", version, forge_ver);
        let mut forge_json_path = mc_dir.join("versions").join(&forge_id).join(format!("{}.json", forge_id));
        if !forge_json_path.exists() {
            forge_id = format!("{}-forge-{}-{}", version, version, forge_ver);
            forge_json_path = mc_dir.join("versions").join(&forge_id).join(format!("{}.json", forge_id));
        }
        if let Ok(forge_json_str) = fs::read_to_string(&forge_json_path) {
            if let Ok(forge_json) = serde_json::from_str::<serde_json::Value>(&forge_json_str) {
                if let Some(mc) = forge_json["mainClass"].as_str() {
                    main_class = mc.to_string();
                }
                if let Some(libraries) = forge_json["libraries"].as_array() {
                    for lib in libraries {
                        let name = lib["name"].as_str().unwrap_or("");
                        let parts: Vec<&str> = name.split(':').collect();
                        if parts.len() >= 3 {
                            let group = parts[0].replace('.', "/");
                            let artifact = parts[1];
                            let lib_version = parts[2];
                            let mut jar_name = format!("{}-{}.jar", artifact, lib_version);
                            if parts.len() >= 4 { jar_name = format!("{}-{}-{}.jar", artifact, lib_version, parts[3]); }
                            let path = mc_dir.join("libraries").join(&group).join(artifact).join(lib_version).join(&jar_name);
                            cp_entries.push(path.to_string_lossy().into_owned());
                        }
                    }
                }
                if let Some(args) = forge_json.get("arguments") {
                    if let Some(jvm) = args.get("jvm").and_then(|v| v.as_array()) {
                        for arg in jvm { if let Some(s) = arg.as_str() { jvm_args.push(s.to_string()); } }
                    }
                    if let Some(game) = args.get("game").and_then(|v| v.as_array()) {
                        for arg in game { if let Some(s) = arg.as_str() { game_args.push(s.to_string()); } }
                    }
                }
            }
        }
    }

    let separator = if cfg!(target_os = "windows") { ";" } else { ":" };
    Ok(LaunchInfo {
        classpath: cp_entries.join(separator),
        asset_index: version_json.asset_index.id,
        main_class,
        jvm_args,
        game_args,
    })
}
