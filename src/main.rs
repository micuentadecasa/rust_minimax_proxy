use anyhow::{anyhow, Context, Result};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::sync::Mutex;
use tracing::{error, info};

const CLIENT_ID: &str = "659cf4c1-615c-45f6-a5f6-4bf15eb476e5";
const DEFAULT_PORT: u16 = 8080;

struct AppState {
    credentials: Mutex<Option<Credentials>>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            credentials: Mutex::new(self.credentials.lock().unwrap().clone()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Credentials {
    access_token: String,
    refresh_token: String,
    expires_at: u64,
    region: String,
    resource_url: Option<String>,
    account: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    messages: Vec<ChatMessage>,
    model: Option<String>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    stream: Option<bool>,
    system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
    index: u32,
    finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicResponse {
    id: String,
    #[serde(rename = "type")]
    msg_type: String,
    role: String,
    content: Vec<AnthropicContent>,
    model: String,
    #[serde(rename = "stop_reason")]
    stop_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
    thinking: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
    code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceCodeResponse {
    #[serde(rename = "user_code")]
    user_code: String,
    #[serde(rename = "verification_uri")]
    verification_uri: String,
    interval: u64,
    expires_in: u64,
    code_verifier: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    status: String,
    #[serde(rename = "access_token")]
    access_token: Option<String>,
    #[serde(rename = "refresh_token")]
    refresh_token: Option<String>,
    #[serde(rename = "expired_in")]
    expired_in: Option<u64>,
    #[serde(rename = "resource_url")]
    resource_url: Option<String>,
}

fn auth_error(message: &str, code: Option<&str>) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse {
            error: ErrorDetail {
                message: message.to_string(),
                error_type: "authentication_required".to_string(),
                code: code.map(|s| s.to_string()),
            },
        }),
    )
}

fn server_error(message: &str) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            error: ErrorDetail {
                message: message.to_string(),
                error_type: "internal_error".to_string(),
                code: None,
            },
        }),
    )
}

fn bad_request(message: &str, code: Option<&str>) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            error: ErrorDetail {
                message: message.to_string(),
                error_type: "bad_request".to_string(),
                code: code.map(|s| s.to_string()),
            },
        }),
    )
}

fn get_config_path() -> std::path::PathBuf {
    if let Ok(dir) = env::var("MMX_CONFIG_DIR") {
        return std::path::PathBuf::from(dir).join("config.json");
    }
    dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(".mmx")
        .join("config.json")
}

fn get_legacy_credentials_path() -> std::path::PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(".mmx")
        .join("credentials.json")
}

fn parse_expires_at_iso(value: &str) -> Option<u64> {
    chrono::DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|dt| dt.timestamp().max(0) as u64)
}

fn expires_to_iso(expires_at: u64) -> String {
    chrono::DateTime::<chrono::Utc>::from_timestamp(expires_at as i64, 0)
        .unwrap_or_else(chrono::Utc::now)
        .to_rfc3339()
}

fn token_expiry_to_epoch_seconds(expired_in: Option<u64>) -> u64 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    match expired_in {
        // MiniMax OAuth returns absolute epoch milliseconds.
        Some(v) if v > 10_000_000_000 => v / 1000,
        // Be tolerant of absolute epoch seconds.
        Some(v) if v > 1_000_000_000 => v,
        // Also tolerate duration seconds.
        Some(v) => now + v,
        None => now + 3600,
    }
}

fn load_credentials() -> Option<Credentials> {
    let config_path = get_config_path();
    if let Ok(content) = fs::read_to_string(&config_path) {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(oauth) = value.get("oauth") {
                let access_token = oauth.get("access_token")?.as_str()?.to_string();
                let refresh_token = oauth
                    .get("refresh_token")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let expires_at = oauth
                    .get("expires_at")
                    .and_then(|v| v.as_str())
                    .and_then(parse_expires_at_iso)?;
                let region = oauth
                    .get("region")
                    .and_then(|v| v.as_str())
                    .unwrap_or("global")
                    .to_string();
                let resource_url = oauth
                    .get("resource_url")
                    .and_then(|v| v.as_str())
                    .map(str::to_string);
                let account = oauth
                    .get("account")
                    .and_then(|v| v.as_str())
                    .map(str::to_string);
                return Some(Credentials {
                    access_token,
                    refresh_token,
                    expires_at,
                    region,
                    resource_url,
                    account,
                });
            }
        }
    }

    // Backward compatibility with the old guide's ~/.mmx/credentials.json format.
    let legacy_path = get_legacy_credentials_path();
    fs::read_to_string(&legacy_path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
}

fn save_credentials(creds: &Credentials) -> Result<()> {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("Failed to create .mmx directory")?;
    }

    let mut config = fs::read_to_string(&path)
        .ok()
        .and_then(|content| serde_json::from_str::<serde_json::Value>(&content).ok())
        .unwrap_or_else(|| serde_json::json!({}));

    config["oauth"] = serde_json::json!({
        "access_token": creds.access_token,
        "refresh_token": creds.refresh_token,
        "expires_at": expires_to_iso(creds.expires_at),
        "region": creds.region,
        "resource_url": creds.resource_url,
        "account": creds.account,
    });
    config["region"] = serde_json::json!(creds.region);

    let content =
        serde_json::to_string_pretty(&config).context("Failed to serialize credentials")?;
    fs::write(&path, content).context("Failed to write credentials")?;
    Ok(())
}

fn oauth_host(region: &str) -> String {
    match region {
        "cn" => "https://account.minimaxi.com".to_string(),
        _ => "https://account.minimax.io".to_string(),
    }
}

fn api_host(region: &str, resource_url: Option<&str>) -> String {
    if let Some(url) = resource_url {
        url.to_string()
    } else {
        match region {
            "cn" => "https://api.minimaxi.com".to_string(),
            _ => "https://api.minimax.io".to_string(),
        }
    }
}

async fn start_device_code(region: String) -> Result<DeviceCodeResponse> {
    let code_verifier = generate_pkce_verifier();
    let code_challenge = generate_pkce_challenge(&code_verifier);

    let client = reqwest::Client::new();
    let state = generate_pkce_verifier();
    let resp = client
        .post(format!("{}/oauth2/device/code", oauth_host(&region)))
        .form(&[
            ("client_id", CLIENT_ID),
            ("scope", "openid profile coding_plan"),
            ("code_challenge", &code_challenge),
            ("code_challenge_method", "S256"),
            ("state", &state),
        ])
        .send()
        .await
        .context("Failed to request device code")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(anyhow!("Device code request failed ({}): {}", status, text));
    }

    #[derive(Deserialize)]
    struct RawDeviceCodeResponse {
        #[serde(rename = "user_code")]
        user_code: Option<String>,
        #[serde(rename = "verification_uri_complete")]
        verification_uri_complete: Option<String>,
        #[serde(rename = "verification_uri")]
        verification_uri: Option<String>,
        interval: Option<u64>,
        #[serde(rename = "expired_in")]
        expired_in: Option<u64>,
        state: Option<String>,
        #[serde(rename = "base_resp")]
        base_resp: Option<BaseResp>,
    }

    #[derive(Deserialize)]
    struct BaseResp {
        #[serde(rename = "status_code")]
        status_code: Option<i32>,
    }

    let raw: RawDeviceCodeResponse = resp
        .json()
        .await
        .context("Failed to parse device code response")?;

    if let Some(br) = raw.base_resp {
        if br.status_code != Some(0) {
            return Err(anyhow!(
                "Device code request failed: status_code={}",
                br.status_code.unwrap_or(-1)
            ));
        }
    }

    if raw.state.as_deref() != Some(&state) {
        return Err(anyhow!("OAuth state mismatch"));
    }

    Ok(DeviceCodeResponse {
        user_code: raw.user_code.unwrap_or_default(),
        verification_uri: raw
            .verification_uri_complete
            .or(raw.verification_uri)
            .unwrap_or_default(),
        interval: raw.interval.unwrap_or(3000),
        expires_in: raw.expired_in.unwrap_or(0),
        code_verifier,
        state,
    })
}

fn generate_pkce_verifier() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(&bytes)
}

fn generate_pkce_challenge(verifier: &str) -> String {
    // Proper SHA256 hash of the verifier
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let result = hasher.finalize();
    URL_SAFE_NO_PAD.encode(&result)
}

async fn poll_for_token(
    region: &str,
    user_code: &str,
    code_verifier: &str,
) -> Result<TokenResponse> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/oauth2/token", oauth_host(region)))
        .form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ("client_id", CLIENT_ID),
            ("user_code", user_code),
            ("code_verifier", code_verifier),
        ])
        .send()
        .await
        .context("Failed to poll for token")?;

    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();

    // Debug: print what we got
    info!("Poll response status: {}, body: {}", status, text);

    // Check if it's not JSON
    if !text.starts_with('{') && !text.starts_with('[') {
        // Not JSON - might be error page or something else
        if text.contains("rate") || text.contains("limit") || text.contains("error") {
            return Ok(TokenResponse {
                status: "pending".to_string(),
                access_token: None,
                refresh_token: None,
                expired_in: None,
                resource_url: None,
            });
        }
        return Err(anyhow!("Non-JSON response ({}): {}", status, text));
    }

    let token: TokenResponse =
        serde_json::from_str(&text).context("Failed to parse token response")?;
    Ok(token)
}

async fn refresh_token(region: &str, refresh_token: &str) -> Result<TokenResponse> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/oauth2/token", oauth_host(region)))
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", CLIENT_ID),
        ])
        .send()
        .await
        .context("Failed to refresh token")?;

    let token: TokenResponse = resp
        .json()
        .await
        .context("Failed to parse refresh response")?;
    Ok(token)
}

fn get_access_token(state: &AppState) -> Result<String> {
    let creds = state.credentials.lock().unwrap();

    let creds = match creds.as_ref() {
        Some(c) => c,
        None => {
            return Err(anyhow!(
                "Not authenticated. Use POST /auth/login to start OAuth flow."
            ))
        }
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if creds.expires_at <= now + 300 {
        return Err(anyhow!("Token expired. Use POST /auth/refresh to refresh."));
    }

    Ok(creds.access_token.clone())
}

async fn chat_handler(
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (StatusCode, Json<ErrorResponse>)> {
    info!("Chat request: {:?}", request);

    let access_token = match get_access_token(&state) {
        Ok(token) => token,
        Err(e) => {
            let err_msg = e.to_string();
            if err_msg.contains("Not authenticated") {
                return Err(auth_error(
                    "MiniMax OAuth authentication required. Use POST /auth/login to start OAuth flow.",
                    Some("AUTH_REQUIRED"),
                ));
            }
            if err_msg.contains("expired") {
                return Err(auth_error(
                    "MiniMax OAuth token expired. Use POST /auth/refresh",
                    Some("TOKEN_EXPIRED"),
                ));
            }
            return Err(server_error(&format!("Auth error: {}", e)));
        }
    };

    let creds = {
        let c = state.credentials.lock().unwrap();
        c.clone()
    };

    let creds = match creds {
        Some(c) => c,
        None => return Err(auth_error("No credentials", Some("NO_CREDENTIALS"))),
    };

    let model = request.model.unwrap_or_else(|| "MiniMax-M2.7".to_string());
    let api_base = api_host(&creds.region, creds.resource_url.as_deref());

    let mut system = request.system;
    let mut messages = Vec::new();
    for msg in request.messages {
        if msg.role == "system" {
            if system.is_none() {
                system = Some(msg.content);
            }
        } else {
            messages.push(msg);
        }
    }

    let anthropic_req = AnthropicRequest {
        model: model.clone(),
        messages,
        max_tokens: request.max_tokens.or(Some(4096)),
        temperature: request.temperature,
        top_p: request.top_p,
        // OpenAI streaming translation is not implemented yet; request a normal JSON response.
        stream: Some(false),
        system,
    };

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/anthropic/v1/messages", api_base))
        .header("x-api-key", access_token)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&anthropic_req)
        .send()
        .await
        .map_err(|e| {
            error!("Failed to send request: {}", e);
            server_error(&format!("Request failed: {}", e))
        })?;

    let status = resp.status();

    if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN {
        return Err(auth_error(
            "MiniMax OAuth token invalid or expired. Use POST /auth/refresh",
            Some("TOKEN_INVALID"),
        ));
    }

    if status.as_u16() == 402 || status.as_u16() >= 500 {
        let error_text = resp.text().await.unwrap_or_default();
        error!("API error ({}): {}", status, error_text);
        return Err(bad_request(
            &format!("API error: {}", error_text),
            Some(&status.as_u16().to_string()),
        ));
    }

    if !status.is_success() {
        let error_text = resp.text().await.unwrap_or_default();
        error!("Request failed ({}): {}", status, error_text);
        return Err(bad_request(
            &format!("Request failed: {}", error_text),
            None,
        ));
    }

    let anthropic_resp: AnthropicResponse = resp.json().await.map_err(|e| {
        error!("Failed to parse response: {}", e);
        server_error(&format!("Failed to parse response: {}", e))
    })?;

    let content = anthropic_resp
        .content
        .iter()
        .filter_map(|c| c.text.clone())
        .collect::<Vec<_>>()
        .join("")
        .trim()
        .to_string();

    let response = ChatResponse {
        id: anthropic_resp.id,
        object: "chat.completion".to_string(),
        created: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        model,
        choices: vec![ChatChoice {
            message: ChatMessage {
                role: anthropic_resp.role,
                content,
            },
            index: 0,
            finish_reason: anthropic_resp
                .stop_reason
                .unwrap_or_else(|| "stop".to_string()),
        }],
    };

    info!("Chat response: {:?}", response);
    Ok(Json(response))
}

async fn health_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    let authenticated = {
        let creds = state.credentials.lock().unwrap();
        creds.is_some()
    };

    Json(serde_json::json!({
        "status": if authenticated { "ok" } else { "auth_required" },
        "authenticated": authenticated
    }))
}

async fn auth_status_handler(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let creds = state.credentials.lock().unwrap();

    match creds.as_ref() {
        Some(c) => {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let expires_in = if c.expires_at > now {
                c.expires_at - now
            } else {
                0
            };

            Ok(Json(serde_json::json!({
                "authenticated": true,
                "region": c.region,
                "expires_in_seconds": expires_in,
                "has_refresh_token": !c.refresh_token.is_empty()
            })))
        }
        None => Err(auth_error(
            "Not authenticated. Use POST /auth/login to start OAuth flow.",
            Some("AUTH_REQUIRED"),
        )),
    }
}

#[derive(Deserialize)]
struct LoginRequest {}

async fn auth_login_handler(
    State(_state): State<AppState>,
    Json(_body): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    Err(bad_request(
        "OAuth is automatically started on server startup. No manual login needed.",
        Some("AUTO_AUTH"),
    ))
}

#[derive(Deserialize)]
struct TokenRequest {
    user_code: String,
    code_verifier: String,
    region: Option<String>,
}

async fn auth_token_handler(
    State(state): State<AppState>,
    Json(body): Json<TokenRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let region = body.region.unwrap_or_else(|| "global".to_string());

    info!("Polling for token with user_code... (will retry until authorized)");

    let interval_ms = 3000;

    let token_resp = loop {
        match poll_for_token(&region, &body.user_code, &body.code_verifier).await {
            Ok(resp) => {
                if resp.status != "pending" {
                    break resp;
                }
            }
            Err(e) => {
                error!("Poll failed: {}", e);
                return Err(server_error(&format!("Token poll failed: {}", e)));
            }
        };

        info!("Authorization pending, waiting...");
        tokio::time::sleep(tokio::time::Duration::from_millis(interval_ms)).await;
    };

    if token_resp.status != "success" || token_resp.access_token.is_none() {
        return Err(bad_request(
            &format!("Authorization failed: status={}", token_resp.status),
            Some("AUTH_FAILED"),
        ));
    }

    let token = token_resp;

    let expires_at = token_expiry_to_epoch_seconds(token.expired_in);

    let credentials = Credentials {
        access_token: token.access_token.unwrap(),
        refresh_token: token.refresh_token.unwrap_or_default(),
        expires_at,
        region: region.clone(),
        resource_url: token.resource_url,
        account: None,
    };

    if let Err(e) = save_credentials(&credentials) {
        error!("Failed to save credentials: {}", e);
    }

    {
        let mut creds = state.credentials.lock().unwrap();
        *creds = Some(credentials.clone());
    }

    info!("OAuth authentication successful!");

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Successfully authenticated with MiniMax OAuth",
        "region": credentials.region,
        "expires_in": credentials.expires_at - std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    })))
}

async fn auth_refresh_handler(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let (region, refresh_token_str, old_resource_url, old_account) = {
        let creds = state.credentials.lock().unwrap();
        match creds.as_ref() {
            Some(c) => (
                c.region.clone(),
                c.refresh_token.clone(),
                c.resource_url.clone(),
                c.account.clone(),
            ),
            None => return Err(auth_error("Not authenticated", Some("AUTH_REQUIRED"))),
        }
    };

    if refresh_token_str.is_empty() {
        return Err(bad_request(
            "No refresh token available. Please re-authenticate with POST /auth/login",
            Some("NO_REFRESH_TOKEN"),
        ));
    }

    info!("Refreshing OAuth token...");

    let token_resp = match refresh_token(&region, &refresh_token_str).await {
        Ok(resp) => resp,
        Err(e) => {
            error!("Token refresh failed: {}", e);
            return Err(server_error(&format!("Token refresh failed: {}", e)));
        }
    };

    if token_resp.status != "success" || token_resp.access_token.is_none() {
        return Err(auth_error(
            "Token refresh failed. Please re-authenticate with POST /auth/login",
            Some("REFRESH_FAILED"),
        ));
    }

    let token = token_resp;

    let expires_at = token_expiry_to_epoch_seconds(token.expired_in);

    let credentials = Credentials {
        access_token: token.access_token.unwrap(),
        refresh_token: token.refresh_token.unwrap_or(refresh_token_str),
        expires_at,
        region: region.clone(),
        resource_url: token.resource_url.or(old_resource_url),
        account: old_account,
    };

    if let Err(e) = save_credentials(&credentials) {
        error!("Failed to save credentials: {}", e);
    }

    {
        let mut creds = state.credentials.lock().unwrap();
        *creds = Some(credentials.clone());
    }

    info!("Token refreshed successfully!");

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Token refreshed successfully",
        "expires_in": credentials.expires_at - std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    })))
}

fn open_browser(url: &str) {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(url).spawn().ok();
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open").arg(url).spawn().ok();
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/c", "start", "", url])
            .spawn()
            .ok();
    }
}

async fn auto_auth(state: &AppState) -> Result<()> {
    info!("Starting automatic OAuth authentication...");

    let region = "global";
    let device_resp = start_device_code(region.to_string()).await?;

    println!();
    println!("==============================================");
    println!("  🔐 MiniMax OAuth Authentication Required");
    println!("==============================================");
    println!();
    println!("  User Code: {}", device_resp.user_code);
    println!();
    println!("  Opening browser for authorization...");
    println!();
    println!("  If browser doesn't open, visit:");
    println!("  {}", device_resp.verification_uri);
    println!();
    println!("  Enter the user code: {}", device_resp.user_code);
    println!("  Then click Authorize");
    println!();
    println!("  Waiting for authentication...");
    println!("  (Press Ctrl+C to exit and try again)");
    println!();

    // Open browser
    open_browser(&device_resp.verification_uri);

    // Poll for token - MiniMax's interval appears to be in milliseconds (3000 = 3 seconds)
    let interval_ms = device_resp.interval;

    info!("Polling every {}ms... (interval from server)", interval_ms);

    let token_resp = loop {
        match poll_for_token(region, &device_resp.user_code, &device_resp.code_verifier).await {
            Ok(resp) => {
                if resp.status != "pending" {
                    break resp;
                }
            }
            Err(e) => {
                error!("Poll error: {}", e);
            }
        };

        print!(".");
        std::io::Write::flush(&mut std::io::stdout()).ok();
        tokio::time::sleep(tokio::time::Duration::from_millis(interval_ms)).await;
    };

    println!();

    if token_resp.status != "success" || token_resp.access_token.is_none() {
        return Err(anyhow!(
            "Authorization failed: status={}",
            token_resp.status
        ));
    }

    let expires_at = token_expiry_to_epoch_seconds(token_resp.expired_in);

    let credentials = Credentials {
        access_token: token_resp.access_token.unwrap(),
        refresh_token: token_resp.refresh_token.unwrap_or_default(),
        expires_at,
        region: region.to_string(),
        resource_url: token_resp.resource_url,
        account: None,
    };

    if let Err(e) = save_credentials(&credentials) {
        error!("Failed to save credentials: {}", e);
    }

    {
        let mut creds = state.credentials.lock().unwrap();
        *creds = Some(credentials.clone());
    }

    println!();
    println!("  ✅ Authentication successful!");
    println!();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| DEFAULT_PORT.to_string())
        .parse()
        .unwrap_or(DEFAULT_PORT);

    // Load existing credentials
    let credentials = load_credentials();

    let state = AppState {
        credentials: Mutex::new(credentials),
    };

    // Check if we have valid credentials
    let needs_auth = {
        let creds = state.credentials.lock().unwrap();
        match creds.as_ref() {
            Some(c) => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                c.expires_at <= now + 300 // Expires within 5 minutes
            }
            None => true,
        }
    };

    if needs_auth {
        println!();
        println!("==============================================");
        println!("  MiniMax OAuth Proxy");
        println!("==============================================");
        println!();

        if let Err(e) = auto_auth(&state).await {
            error!("Auto-auth failed: {}", e);
            println!();
            println!("  ✗ Authentication failed: {}", e);
            println!("  Please try running again or check your internet connection.");
            std::process::exit(1);
        }
    } else {
        info!("Using existing OAuth credentials");
    }

    println!();
    println!("==============================================");
    println!("  MiniMax Proxy Server (OAuth)");
    println!("  port: http://0.0.0.0:{}", port);
    println!("==============================================");
    println!();

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/auth/status", get(auth_status_handler))
        .route("/auth/login", post(auth_login_handler))
        .route("/auth/token", post(auth_token_handler))
        .route("/auth/refresh", post(auth_refresh_handler))
        .route("/v1/chat/completions", post(chat_handler))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    info!("Starting server on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
