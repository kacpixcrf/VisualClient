use std::fs;
use serde::{Deserialize, Serialize};
use fastnbt::from_bytes;
use craftping::tokio::ping;

#[derive(Serialize)]
pub struct ServerItem {
    pub name: String,
    pub ip: String,
    pub accept_textures: Option<u8>,
    pub icon_base64: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
struct ServersDat {
    servers: Option<Vec<ServerEntry>>,
}

#[derive(Deserialize, Serialize, Clone)]
struct ServerEntry {
    name: Option<String>,
    ip: Option<String>,
    #[serde(rename = "acceptTextures", skip_serializing_if = "Option::is_none")]
    accept_textures: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
}

#[derive(Serialize)]
pub struct PingResponse {
    pub motd: String,
    pub online: bool,
    pub players_online: usize,
    pub players_max: usize,
}

fn servers_dat_path(id: &str) -> std::path::PathBuf {
    crate::core::paths::get_dot_visualclient_dir()
        .join("profiles").join(id).join("servers.dat")
}

fn load_servers_dat(path: &std::path::PathBuf) -> ServersDat {
    if path.exists() {
        if let Ok(bytes) = fs::read(path) {
            return fastnbt::from_bytes::<ServersDat>(&bytes)
                .unwrap_or(ServersDat { servers: Some(Vec::new()) });
        }
    }
    ServersDat { servers: Some(Vec::new()) }
}

fn save_servers_dat(path: &std::path::PathBuf, data: &ServersDat) -> Result<(), String> {
    let out_bytes = fastnbt::to_bytes(data).map_err(|e| e.to_string())?;
    fs::write(path, out_bytes).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_instance_servers(id: String) -> Result<Vec<ServerItem>, String> {
    let path = servers_dat_path(&id);
    if !path.exists() { return Ok(Vec::new()); }

    let bytes = fs::read(&path).map_err(|e| e.to_string())?;
    let data: ServersDat = match from_bytes(&bytes) {
        Ok(d) => d,
        Err(_) => return Ok(Vec::new()),
    };

    let mut result = Vec::new();
    if let Some(servers) = data.servers {
        for s in servers {
            if let (Some(name), Some(ip)) = (s.name, s.ip) {
                result.push(ServerItem { name, ip, accept_textures: s.accept_textures, icon_base64: s.icon });
            }
        }
    }
    Ok(result)
}

#[tauri::command]
pub fn add_instance_server(id: String, name: String, ip: String, accept_textures: Option<u8>) -> Result<(), String> {
    let path = servers_dat_path(&id);
    let mut data = load_servers_dat(&path);
    let new_server = ServerEntry { name: Some(name), ip: Some(ip), accept_textures, icon: None };
    if let Some(ref mut servers) = data.servers {
        servers.push(new_server);
    } else {
        data.servers = Some(vec![new_server]);
    }
    save_servers_dat(&path, &data)
}

#[tauri::command]
pub fn update_instance_server(id: String, original_ip: String, new_name: String, new_ip: String, accept_textures: Option<u8>) -> Result<(), String> {
    let path = servers_dat_path(&id);
    let mut data = load_servers_dat(&path);
    if let Some(ref mut servers) = data.servers {
        for s in servers.iter_mut() {
            if s.ip.as_deref() == Some(&original_ip) {
                s.name = Some(new_name);
                s.ip = Some(new_ip);
                s.accept_textures = accept_textures;
                break;
            }
        }
    }
    save_servers_dat(&path, &data)
}

#[tauri::command]
pub fn remove_instance_server(id: String, ip_to_remove: String) -> Result<(), String> {
    let path = servers_dat_path(&id);
    if !path.exists() { return Ok(()); }
    let bytes = fs::read(&path).map_err(|e| e.to_string())?;
    let mut data: ServersDat = fastnbt::from_bytes(&bytes).map_err(|e| e.to_string())?;
    if let Some(ref mut servers) = data.servers {
        servers.retain(|s| s.ip.as_deref() != Some(&ip_to_remove));
    }
    save_servers_dat(&path, &data)
}

#[tauri::command]
pub fn update_server_icon(id: String, ip_to_match: String, icon_base64: String) -> Result<(), String> {
    let path = servers_dat_path(&id);
    if !path.exists() { return Ok(()); }
    let bytes = fs::read(&path).map_err(|e| e.to_string())?;
    let mut data: ServersDat = fastnbt::from_bytes(&bytes).map_err(|e| e.to_string())?;
    if let Some(ref mut servers) = data.servers {
        for s in servers {
            if s.ip.as_deref() == Some(&ip_to_match) {
                let clean = if icon_base64.starts_with("data:image/") {
                    icon_base64.split(',').nth(1).unwrap_or(&icon_base64).to_string()
                } else { icon_base64.clone() };
                s.icon = Some(clean);
                break;
            }
        }
    }
    save_servers_dat(&path, &data)
}

fn extract_motd(desc: &Option<serde_json::Value>) -> String {
    match desc {
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(serde_json::Value::Object(obj)) => {
            let mut result = String::new();
            if let Some(serde_json::Value::String(text)) = obj.get("text") { result.push_str(text); }
            if let Some(serde_json::Value::Array(extra)) = obj.get("extra") {
                for item in extra {
                    if let Some(serde_json::Value::String(text)) = item.get("text") { result.push_str(text); }
                }
            }
            if result.is_empty() { result = serde_json::to_string(obj).unwrap_or_default(); }
            result
        },
        Some(val) => serde_json::to_string(val).unwrap_or_default(),
        None => "".to_string(),
    }
}

fn html_escape(text: &str) -> String {
    text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
        .replace("\"", "&quot;").replace("'", "&#39;")
}

#[tauri::command]
pub async fn ping_server(ip: String) -> Result<PingResponse, String> {
    let mut host = ip.clone();
    let mut port = 25565u16;
    if let Some(idx) = ip.find(':') {
        let (h, p) = ip.split_at(idx);
        host = h.to_string();
        if let Ok(p_num) = p[1..].parse::<u16>() { port = p_num; }
    }

    let stream_result = tokio::time::timeout(
        std::time::Duration::from_secs(3),
        tokio::net::TcpStream::connect(format!("{}:{}", host, port))
    ).await;

    let mut stream = match stream_result {
        Ok(Ok(s)) => s,
        _ => return Ok(PingResponse { motd: "".to_string(), online: false, players_online: 0, players_max: 0 }),
    };

    match ping(&mut stream, &host, port, craftping::PROTOCOL_VERSION_NOT_SET).await {
        Ok(res) => Ok(PingResponse {
            motd: html_escape(&extract_motd(&res.description)),
            online: true,
            players_online: res.online_players,
            players_max: res.max_players,
        }),
        Err(_) => Ok(PingResponse { motd: "".to_string(), online: false, players_online: 0, players_max: 0 }),
    }
}
