use reqwest::Client;
use serde_json::json;
use serde::{Deserialize, Serialize};

const CLIENT_ID: &str = "00000000402b5328";
const MS_TOKEN_URL: &str = "https://login.live.com/oauth20_token.srf";
const MS_REDIRECT_URI: &str = "https://login.live.com/oauth20_desktop.srf";
const XBL_AUTH_URL: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XSTS_AUTH_URL: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const MC_LOGIN_URL: &str = "https://api.minecraftservices.com/authentication/login_with_xbox";
const MC_PROFILE_URL: &str = "https://api.minecraftservices.com/minecraft/profile";
const XBL_RELYING_PARTY: &str = "http://auth.xboxlive.com";
const XSTS_RELYING_PARTY: &str = "rp://api.minecraftservices.com/";
const GRANT_TYPE_AUTH_CODE: &str = "authorization_code";
const GRANT_TYPE_REFRESH: &str = "refresh_token";

#[derive(Debug, Serialize, Deserialize)]
pub struct MinecraftProfile {
    pub id: String,
    pub name: String,
    pub mc_token: String,
    pub refresh_token: String,
}

pub async fn login_with_code(code: &str) -> Result<MinecraftProfile, String> {
    let client = Client::new();
    let res = client.post(MS_TOKEN_URL)
        .form(&[
            ("client_id", CLIENT_ID),
            ("code", code),
            ("grant_type", GRANT_TYPE_AUTH_CODE),
            ("redirect_uri", MS_REDIRECT_URI),
        ])
        .send().await.map_err(|e| e.to_string())?;
    let ms_auth: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let access_token = ms_auth["access_token"].as_str()
        .ok_or_else(|| format!("No access token. MS Response: {}", ms_auth))?;
    let refresh_token = ms_auth["refresh_token"].as_str().unwrap_or("").to_string();
    login_with_ms_token(access_token, refresh_token).await
}

pub async fn login_with_refresh_token(refresh_token: &str) -> Result<MinecraftProfile, String> {
    let client = Client::new();
    let res = client.post(MS_TOKEN_URL)
        .form(&[
            ("client_id", CLIENT_ID),
            ("refresh_token", refresh_token),
            ("grant_type", GRANT_TYPE_REFRESH),
            ("redirect_uri", MS_REDIRECT_URI),
        ])
        .send().await.map_err(|e| e.to_string())?;
    let ms_auth: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let access_token = ms_auth["access_token"].as_str()
        .ok_or_else(|| format!("No access token. MS Response: {}", ms_auth))?;
    let new_refresh_token = ms_auth["refresh_token"].as_str().unwrap_or(refresh_token).to_string();
    login_with_ms_token(access_token, new_refresh_token).await
}

async fn login_with_ms_token(access_token: &str, refresh_token: String) -> Result<MinecraftProfile, String> {
    let client = Client::new();

    let xbl_payload = json!({
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": format!("d={}", access_token)
        },
        "RelyingParty": XBL_RELYING_PARTY,
        "TokenType": "JWT"
    });
    let xbl_auth: serde_json::Value = client.post(XBL_AUTH_URL)
        .json(&xbl_payload)
        .send().await.map_err(|e| e.to_string())?
        .json().await.map_err(|e| e.to_string())?;
    let xbl_token = xbl_auth["Token"].as_str().ok_or("No XBL token")?;
    let uhs = xbl_auth["DisplayClaims"]["xui"][0]["uhs"].as_str().ok_or("No uhs")?;

    let xsts_payload = json!({
        "Properties": {
            "SandboxId": "RETAIL",
            "UserTokens": [xbl_token]
        },
        "RelyingParty": XSTS_RELYING_PARTY,
        "TokenType": "JWT"
    });
    let xsts_auth: serde_json::Value = client.post(XSTS_AUTH_URL)
        .json(&xsts_payload)
        .send().await.map_err(|e| e.to_string())?
        .json().await.map_err(|e| e.to_string())?;
    if xsts_auth.get("XErr").is_some() {
        return Err("XSTS Error - Account might not have Minecraft or requires child approval".into());
    }
    let xsts_token = xsts_auth["Token"].as_str().ok_or("No XSTS token")?;

    let mc_payload = json!({
        "identityToken": format!("XBL3.0 x={};{}", uhs, xsts_token)
    });
    let mc_auth: serde_json::Value = client.post(MC_LOGIN_URL)
        .json(&mc_payload)
        .send().await.map_err(|e| e.to_string())?
        .json().await.map_err(|e| e.to_string())?;
    let mc_token = mc_auth["access_token"].as_str().ok_or("No Minecraft token")?;

    let profile: serde_json::Value = client.get(MC_PROFILE_URL)
        .header("Authorization", format!("Bearer {}", mc_token))
        .send().await.map_err(|e| e.to_string())?
        .json().await.map_err(|e| e.to_string())?;
    let id = profile["id"].as_str().ok_or("No Minecraft ID (You probably don't own the game)")?;
    let name = profile["name"].as_str().ok_or("No Minecraft Name")?;

    Ok(MinecraftProfile {
        id: id.to_string(),
        name: name.to_string(),
        mc_token: mc_token.to_string(),
        refresh_token,
    })
}
