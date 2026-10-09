use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub username: String,
    pub active: bool,
    #[serde(rename = "type")]
    pub account_type: Option<String>,
    pub uuid: Option<String>,
    pub mc_token: Option<String>,
    pub refresh_token: Option<String>,
}

fn get_dot_visualclient_dir() -> PathBuf {
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

fn get_accounts_file() -> PathBuf {
    get_dot_visualclient_dir().join("launcher").join("accounts.json")
}

#[tauri::command]
pub fn get_accounts() -> Result<Vec<Account>, String> {
    let file = get_accounts_file();
    let content = crate::core::crypto::read_encrypted_file(&file)?;
    if content.is_empty() {
        return Ok(Vec::new());
    }
    let accounts: Vec<Account> = serde_json::from_str(&content).unwrap_or_default();
    Ok(accounts)
}

#[tauri::command]
pub fn add_account(username: String, account_type: String) -> Result<(), String> {
    let mut accounts = get_accounts().unwrap_or_default();
    if !accounts.iter().any(|a| a.username == username) {
        accounts.push(Account { username, active: false, account_type: Some(account_type), uuid: None, mc_token: None, refresh_token: None });
        let file = get_accounts_file();
        crate::core::crypto::write_encrypted_file(&file, &serde_json::to_string_pretty(&accounts).unwrap())?;
    }
    Ok(())
}

#[tauri::command]
pub fn add_microsoft_account(username: String, uuid: String, mctoken: String, refreshtoken: String) -> Result<(), String> {
    let mut accounts = get_accounts().unwrap_or_default();
    
    // Find existing or add new
    if let Some(existing) = accounts.iter_mut().find(|a| a.username == username) {
        existing.account_type = Some("Microsoft".to_string());
        existing.uuid = Some(uuid);
        existing.mc_token = Some(mctoken);
        existing.refresh_token = Some(refreshtoken);
    } else {
        accounts.push(Account { 
            username, 
            active: false, 
            account_type: Some("Microsoft".to_string()),
            uuid: Some(uuid),
            mc_token: Some(mctoken),
            refresh_token: Some(refreshtoken)
        });
    }

    let file = get_accounts_file();
    crate::core::crypto::write_encrypted_file(&file, &serde_json::to_string_pretty(&accounts).unwrap())?;
    
    Ok(())
}

#[tauri::command]
pub fn select_account(username: String) -> Result<(), String> {
    let mut accounts = get_accounts().unwrap_or_default();
    for acc in &mut accounts {
        acc.active = acc.username == username;
    }
    crate::core::crypto::write_encrypted_file(&get_accounts_file(), &serde_json::to_string_pretty(&accounts).unwrap())?;
    Ok(())
}

#[tauri::command]
pub fn delete_account(username: String) -> Result<(), String> {
    let mut accounts = get_accounts().unwrap_or_default();
    accounts.retain(|a| a.username != username);
    crate::core::crypto::write_encrypted_file(&get_accounts_file(), &serde_json::to_string_pretty(&accounts).unwrap())?;
    Ok(())
}

pub fn update_account_tokens(username: &str, mc_token: String, refresh_token: String) -> Result<(), String> {
    let mut accounts = get_accounts().unwrap_or_default();
    if let Some(existing) = accounts.iter_mut().find(|a| a.username == username) {
        existing.mc_token = Some(mc_token);
        existing.refresh_token = Some(refresh_token);
        let file = get_accounts_file();
        crate::core::crypto::write_encrypted_file(&file, &serde_json::to_string_pretty(&accounts).unwrap())?;
    }
    Ok(())
}
