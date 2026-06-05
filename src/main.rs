use anyhow::{anyhow, Context, Result};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
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
    stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    stream: Option<bool>,
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

fn get_credentials_path() -> std::path::PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(".mmx")
        .join("credentials.json")
}

fn load_credentials() -> Option<Credentials> {
    let path = get_credentials_path();
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).ok(),
            Err(_) => None,
        }
    } else {
        None
    }
}

fn save_credentials(creds: &Credentials) -> Result<()> {
    let path = get_credentials_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("Failed to create .mmx directory")?;
    }
    let content = serde_json::to_string_pretty(creds).context("Failed to serialize credentials")?;
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
    let resp = client
        .post(format!("{}/oauth2/device/code", oauth_host(&region)))
        .form(&[
            ("client_id", CLIENT_ID),
            ("scope", "openid profile coding_plan"),
            ("code_challenge", &code_challenge),
            ("code_challenge_method", "S256"),
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
        #[serde(rename = "verification_uri")]
        verification_uri: Option<String>,
        interval: Option<u64>,
        #[serde(rename = "expired_in")]
        expired_in: Option<u64>,
    }

    let raw: RawDeviceCodeResponse = resp.json().await.context("Failed to parse device code response")?;

    Ok(DeviceCodeResponse {
        user_code: raw.user_code.unwrap_or_default(),
        verification_uri: raw.verification_uri.unwrap_or_default(),
        interval: raw.interval.unwrap_or(5),
        expires_in: raw.expired_in.unwrap_or(1800),
        code_verifier,
    })
}

fn generate_pkce_verifier() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    let mut bytes = [0u8; 32];
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = ((now >> i) & 0xFF) as u8 ^ ((now >> (i + 8)) & 0xFF) as u8;
    }
    bytes[0] ^= 0xAB;
    bytes[31] ^= 0xCD;
    
    URL_SAFE_NO_PAD.encode(&bytes)
}

fn generate_pkce_challenge(verifier: &str) -> String {
    // Proper SHA256 hash of the verifier
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let result = hasher.finalize();
    URL_SAFE_NO_PAD.encode(&result)
}

async fn poll_for_token(region: &str, user_code: &str, code_verifier: &str) -> Result<TokenResponse> {
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

    let token: TokenResponse = serde_json::from_str(&text).context("Failed to parse token response")?;
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

    let token: TokenResponse = resp.json().await.context("Failed to parse refresh response")?;
    Ok(token)
}

fn get_access_token(state: &AppState) -> Result<String> {
    let creds = state.credentials.lock().unwrap();
    
    let creds = match creds.as_ref() {
        Some(c) => c,
        None => return Err(anyhow!("Not authenticated. Use POST /auth/login to start OAuth flow.")),
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

    let anthropic_req = AnthropicRequest {
        model: model.clone(),
        messages: request.messages,
        max_tokens: request.max_tokens.or(Some(4096)),
        temperature: request.temperature,
        stream: request.stream,
    };

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/anthropic/v1/messages", api_base))
        .header("x-api-key", &access_token)
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
        return Err(bad_request(&format!("Request failed: {}", error_text), None));
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
            finish_reason: anthropic_resp.stop_reason.unwrap_or_else(|| "stop".to_string()),
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

    let expires_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + token.expired_in.unwrap_or(3600);

    let credentials = Credentials {
        access_token: token.access_token.unwrap(),
        refresh_token: token.refresh_token.unwrap_or_default(),
        expires_at,
        region: region.clone(),
        resource_url: token.resource_url,
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
    let (region, refresh_token_str) = {
        let creds = state.credentials.lock().unwrap();
        match creds.as_ref() {
            Some(c) => (c.region.clone(), c.refresh_token.clone()),
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

    let expires_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + token.expired_in.unwrap_or(3600);

    let credentials = Credentials {
        access_token: token.access_token.unwrap(),
        refresh_token: token.refresh_token.unwrap_or(refresh_token_str),
        expires_at,
        region: region.clone(),
        resource_url: token.resource_url,
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
        return Err(anyhow!("Authorization failed: status={}", token_resp.status));
    }
    
    let expires_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + token_resp.expired_in.unwrap_or(3600);
    
    let credentials = Credentials {
        access_token: token_resp.access_token.unwrap(),
        refresh_token: token_resp.refresh_token.unwrap_or_default(),
        expires_at,
        region: region.to_string(),
        resource_url: token_resp.resource_url,
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