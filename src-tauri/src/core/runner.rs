use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tauri::{AppHandle, Emitter, Manager, State};

use crate::core::launch_args::{
    resolve_launch_args, build_base_jvm_args, apply_game_args, apply_fallback_game_args,
    check_quick_play, get_uuid_and_token,
};

const EVENT_DOWNLOAD_PROGRESS: &str = "download_progress";
const EVENT_INSTANCE_STOPPED: &str = "instance_stopped";
const TASK_DONE: &str = "Done";
const ACCOUNT_TYPE_MICROSOFT: &str = "Microsoft";
const ERR_REFRESH_FAILED: &str = "Failed to refresh Microsoft token. Please log in again.";
const ERR_NO_REFRESH_TOKEN: &str = "No refresh token available. Please log in again.";

pub struct LauncherState(pub Arc<Mutex<HashMap<String, tokio::sync::oneshot::Sender<()>>>>);

#[derive(serde::Serialize, Clone)]
struct ProgressPayload {
    task: String,
    progress: u8,
}

#[tauri::command]
pub async fn launch_instance(
    app: AppHandle,
    id: String,
    username: String,
    launching_text: String,
    server_ip: Option<String>,
    world_folder: Option<String>,
) -> Result<(), String> {
    let _ = app.emit(EVENT_DOWNLOAD_PROGRESS, ProgressPayload { task: launching_text, progress: 100 });

    let mut args = resolve_launch_args(&id, &username)?;

    if args.account.account_type.as_deref() == Some(ACCOUNT_TYPE_MICROSOFT) {
        if args.account.refresh_token.is_some() {
            if let Ok(profile) = crate::commands::auth::refresh_account_token(username.clone()).await {
                args.account.mc_token = Some(profile.mc_token);
            } else {
                return Err(ERR_REFRESH_FAILED.to_string());
            }
        } else {
            return Err(ERR_NO_REFRESH_TOKEN.to_string());
        }
    }

    let launch_info = crate::core::launch_info::get_launch_info(&args.instance.version, &args.instance.loader, &args.mc_dir)?;
    let supports_quick_play = check_quick_play(&args.instance.version);
    let (uuid_str, token_str, user_type_str) = get_uuid_and_token(&args.account);

    let mut cmd = tokio::process::Command::new(&args.java_path);
    cmd.current_dir(&args.profiles_dir);

    build_base_jvm_args(&mut cmd, &launch_info, &args);

    let has_username = apply_game_args(&mut cmd, &launch_info, &args, &uuid_str, &token_str, user_type_str, supports_quick_play);

    if !has_username {
        apply_fallback_game_args(&mut cmd, &launch_info, &args, &uuid_str, &token_str, user_type_str, supports_quick_play);
    }

    if let Some(ip) = server_ip {
        if supports_quick_play {
            cmd.arg("--quickPlayMultiplayer").arg(&ip);
        } else if ip.contains(':') {
            let parts: Vec<&str> = ip.split(':').collect();
            if parts.len() == 2 {
                cmd.arg("--server").arg(parts[0]);
                cmd.arg("--port").arg(parts[1]);
            } else {
                cmd.arg("--server").arg(&ip);
            }
        } else {
            cmd.arg("--server").arg(&ip);
        }
    } else if let Some(world) = world_folder {
        if supports_quick_play {
            cmd.arg("--quickPlaySingleplayer").arg(&world);
        }
    }

    #[cfg(target_os = "windows")]
    { cmd.creation_flags(0x08000000); }

    let mut child = cmd.spawn().map_err(|e| format!("Failed to start game: {}", e))?;

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    {
        let state: State<LauncherState> = app.state();
        state.0.lock().unwrap().insert(id.clone(), tx);
    }

    let app_clone = app.clone();
    let id_clone = id.clone();
    tauri::async_runtime::spawn(async move {
        tokio::select! {
            _ = child.wait() => {}
            _ = rx => { let _ = child.kill().await; }
        }
        let state: State<LauncherState> = app_clone.state();
        state.0.lock().unwrap().remove(&id_clone);
        let _ = app_clone.emit(EVENT_INSTANCE_STOPPED, id_clone);
    });

    let _ = app.emit(EVENT_DOWNLOAD_PROGRESS, ProgressPayload { task: TASK_DONE.to_string(), progress: 100 });
    Ok(())
}

#[tauri::command]
pub fn kill_instance(app: AppHandle, id: String) -> Result<(), String> {
    let state: State<LauncherState> = app.state();
    if let Some(tx) = state.0.lock().unwrap().remove(&id) {
        let _ = tx.send(());
    }
    Ok(())
}

#[tauri::command]
pub fn get_running_instances(app: AppHandle) -> Result<Vec<String>, String> {
    let state: State<LauncherState> = app.state();
    let keys: Vec<String> = state.0.lock().unwrap().keys().cloned().collect();
    Ok(keys)
}
