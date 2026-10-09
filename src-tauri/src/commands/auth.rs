use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};
use crate::commands::ms_token::{MinecraftProfile, login_with_code, login_with_refresh_token};

const MS_LOGIN_URL_BASE: &str = "https://login.live.com/oauth20_authorize.srf";
const MS_DESKTOP_REDIRECT: &str = "https://login.live.com/oauth20_desktop.srf";
const MS_CLIENT_ID: &str = "00000000402b5328";
const POLL_INTERVAL_MS: u64 = 500;
const WINDOW_LABEL: &str = "ms_login";
const WINDOW_TITLE: &str = "Microsoft Login";
const WINDOW_WIDTH: f64 = 500.0;
const WINDOW_HEIGHT: f64 = 600.0;
const ACCOUNT_TYPE_MICROSOFT: &str = "Microsoft";
const QUERY_PARAM_CODE: &str = "code";

#[tauri::command]
pub async fn start_microsoft_login(app: AppHandle) -> Result<MinecraftProfile, String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let tx = std::sync::Arc::new(std::sync::Mutex::new(Some(tx)));

    let login_url = format!(
        "{}?client_id={}&response_type=code&redirect_uri={}&scope=XboxLive.signin%20offline_access&prompt=select_account",
        MS_LOGIN_URL_BASE, MS_CLIENT_ID, MS_DESKTOP_REDIRECT
    );

    let window_tx = tx.clone();
    let tx_for_poll = window_tx.clone();

    let window = WebviewWindowBuilder::new(&app, WINDOW_LABEL, WebviewUrl::External(login_url.parse().unwrap()))
        .title(WINDOW_TITLE)
        .inner_size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .on_navigation(move |url| {
            if url.as_str().starts_with(MS_DESKTOP_REDIRECT) {
                let mut code = String::new();
                for (key, value) in url.query_pairs() {
                    if key == QUERY_PARAM_CODE {
                        code = value.to_string();
                    }
                }
                if !code.is_empty() {
                    if let Ok(mut lock) = window_tx.lock() {
                        if let Some(tx) = lock.take() {
                            let _ = tx.send(code);
                        }
                    }
                    return false;
                }
            }
            true
        })
        .build()
        .map_err(|e| e.to_string())?;

    let window_for_poll = window.clone();

    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(POLL_INTERVAL_MS)).await;
            match window_for_poll.url() {
                Ok(url) => {
                    if url.as_str().starts_with(MS_DESKTOP_REDIRECT) {
                        let mut code = String::new();
                        for (key, value) in url.query_pairs() {
                            if key == QUERY_PARAM_CODE {
                                code = value.to_string();
                            }
                        }
                        if !code.is_empty() {
                            if let Ok(mut lock) = tx_for_poll.lock() {
                                if let Some(tx) = lock.take() {
                                    let _ = tx.send(code);
                                }
                            }
                            break;
                        }
                    }
                }
                Err(_) => { break; }
            }
        }
    });

    let code = rx.await.map_err(|_| "Login window closed".to_string())?;
    let _ = window.close();
    login_with_code(&code).await
}

#[tauri::command]
pub async fn cancel_microsoft_login(app: AppHandle) -> Result<(), String> {
    use tauri::Manager;
    if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
        let _ = window.close();
    }
    Ok(())
}

#[tauri::command]
pub async fn refresh_account_token(username: String) -> Result<MinecraftProfile, String> {
    let accounts = crate::commands::accounts::get_accounts().unwrap_or_default();
    let account = accounts.into_iter().find(|a| a.username == username).ok_or("Account not found")?;

    if account.account_type.as_deref() != Some(ACCOUNT_TYPE_MICROSOFT) {
        return Err("Not a Microsoft account".to_string());
    }

    let refresh_token = account.refresh_token.ok_or("No refresh token available")?;
    let profile = login_with_refresh_token(&refresh_token).await?;
    crate::commands::accounts::update_account_tokens(&username, profile.mc_token.clone(), profile.refresh_token.clone())?;
    Ok(profile)
}
