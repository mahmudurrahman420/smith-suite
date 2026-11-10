//! HTTP client for the launcher API. Field names follow the v1 contract.

use std::io::{Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};
use sha2::Digest;

use crate::endpoint::dev_approve_allowed;
use crate::error::{Error, Result};
use smithcc_catalog::Catalog;

pub const LAUNCHER_VERSION: &str = env!("CARGO_PKG_VERSION");
const MAX_DOWNLOAD: u64 = 2_200_000_000;

#[derive(Debug, Clone)]
pub struct ApiClient {
    http: reqwest::blocking::Client,
    download: reqwest::blocking::Client,
    base: String,
    version: String,
}

impl ApiClient {
    pub fn new(base: &str, version: &str) -> Result<Self> {
        let base = base.trim().trim_end_matches('/').to_string();
        if base.is_empty() {
            return Err(Error::msg("the API base URL is empty"));
        }
        let http = reqwest::blocking::Client::builder()
            .user_agent(format!("SmithCreativeCloud/{version}"))
            .connect_timeout(Duration::from_secs(15))
            .timeout(Duration::from_secs(45))
            .build()
            .map_err(|e| Error::Network(e.to_string()))?;
        let download = reqwest::blocking::Client::builder()
            .user_agent(format!("SmithCreativeCloud/{version}"))
            .connect_timeout(Duration::from_secs(15))
            .timeout(Duration::from_secs(60 * 60))
            .build()
            .map_err(|e| Error::Network(e.to_string()))?;
        Ok(Self { http, download, base, version: version.to_string() })
    }

    pub fn health(&self) -> Result<Health> {
        let raw = self.send(Method::Get, "/health", None::<&()>, None, None)?;
        expect_status(&raw, &[200])?;
        from_json(&raw.body)
    }

    pub fn catalog(&self) -> Result<Catalog> {
        let raw = self.send(Method::Get, "/catalog", None::<&()>, None, None)?;
        if raw.status == 503 {
            return Err(api_error(&raw));
        }
        expect_status(&raw, &[200])?;
        from_json(&raw.body)
    }

    pub fn create_session(&self, body: &SessionRequest) -> Result<SessionCreated> {
        let raw = self.send(Method::Post, "/auth/sessions", Some(body), None, None)?;
        if raw.status == 429 {
            return Err(api_error(&raw));
        }
        expect_status(&raw, &[201])?;
        from_json(&raw.body)
    }

    pub fn poll_session(&self, session_id: &str) -> Result<Poll> {
        let path = format!("/auth/sessions/{}", encode_segment(session_id));
        let raw = self.send(Method::Get, &path, None::<&()>, None, None)?;
        interpret_poll(&raw)
    }

    pub fn refresh(&self, refresh_token: &str, device_id: &str) -> Result<Refreshed> {
        let body = RefreshRequest { refresh_token, device_id };
        let raw = self.send(Method::Post, "/auth/refresh", Some(&body), None, None)?;
        expect_status(&raw, &[200])?;
        from_json(&raw.body)
    }

    pub fn logout(&self, access_token: &str, refresh_token: &str) -> Result<()> {
        let body = LogoutRequest { refresh_token };
        let raw = self.send(Method::Post, "/auth/logout", Some(&body), Some(access_token), None)?;
        expect_status(&raw, &[204])?;
        Ok(())
    }

    pub fn me(&self, access_token: &str) -> Result<User> {
        let raw = self.send(Method::Get, "/me", None::<&()>, Some(access_token), None)?;
        expect_status(&raw, &[200])?;
        from_json(&raw.body)
    }

    pub fn admin_me(&self, access_token: &str) -> Result<AdminMe> {
        let raw = self.send(Method::Get, "/admin/me", None::<&()>, Some(access_token), None)?;
        expect_status(&raw, &[200])?;
        let mut me: AdminMe = from_json(&raw.body)?;
        me.is_admin = true;
        Ok(me)
    }

    pub fn admin_stats(&self, access_token: &str) -> Result<AdminStats> {
        let raw = self.send(Method::Get, "/admin/stats", None::<&()>, Some(access_token), None)?;
        expect_status(&raw, &[200])?;
        from_json(&raw.body)
    }

    pub fn admin_users(&self, access_token: &str, query: &UserQuery) -> Result<UserPage> {
        let path = format!("/admin/users?{}", query.as_query());
        let raw = self.send(Method::Get, &path, None::<&()>, Some(access_token), None)?;
        expect_status(&raw, &[200])?;
        from_json(&raw.body)
    }

    pub fn heartbeat(&self, access_token: &str, body: &Heartbeat) -> Result<HeartbeatAck> {
        let raw = self.send(Method::Post, "/devices/heartbeat", Some(body), Some(access_token), None)?;
        expect_status(&raw, &[200])?;
        from_json(&raw.body)
    }

    pub fn post_event(&self, access_token: &str, idempotency_key: &str, body: &InstallEvent) -> Result<EventAck> {
        if idempotency_key.trim().is_empty() {
            return Err(Error::msg("an install event needs an Idempotency-Key"));
        }
        let raw = self.send(Method::Post, "/installs/events", Some(body), Some(access_token), Some(idempotency_key))?;
        expect_status(&raw, &[202])?;
        from_json(&raw.body)
    }

    /// Staging-only. Refuses every other origin, including production, before opening a socket.
    pub fn approve_dev_session(&self, session_id: &str, email: Option<&str>, name: Option<&str>) -> Result<()> {
        if !dev_approve_allowed(&self.base) {
            return Err(Error::msg("the staging test sign-in is refused for this server"));
        }
        let path = format!("/dev/sessions/{}/approve", encode_segment(session_id));
        let body = DevApprove { email, name };
        let raw = self.send(Method::Post, &path, Some(&body), None, None)?;
        expect_status(&raw, &[200])?;
        Ok(())
    }

    #[cfg(test)]
    pub fn get_text(&self, url: &str) -> Result<String> {
        let response = self.http.get(url).send().map_err(network_error)?;
        let status = response.status().as_u16();
        let text = response.text().map_err(network_error)?;
        if !(200..300).contains(&status) {
            return Err(Error::msg(format!("GET {url} returned {status}")));
        }
        Ok(text)
    }

    pub fn download(&self, request: Download<'_>) -> Result<()> {
        if request.expected_sha256.len() != 64 || !request.expected_sha256.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(Error::msg("the catalog entry has no usable sha256"));
        }
        if request.expected_size > MAX_DOWNLOAD {
            return Err(Error::msg("the package is larger than the 2 GB limit"));
        }
        if let Some(parent) = request.dest.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut response = self.download.get(request.url).send().map_err(network_error)?;
        let status = response.status().as_u16();
        if status == 429 || status >= 400 {
            let retry_after = response.headers().get("retry-after").and_then(|v| v.to_str().ok()).and_then(parse_retry_after);
            let bytes = response.bytes().map_err(network_error)?;
            return Err(api_error(&Raw { status, retry_after, body: bytes.to_vec() }));
        }
        if let Some(len) = response.content_length()
            && (len > MAX_DOWNLOAD || (request.expected_size > 0 && len != request.expected_size))
        {
            return Err(Error::msg("the download size does not match the catalog"));
        }
        let total = response.content_length().unwrap_or(request.expected_size);
        let mut file = std::fs::File::create(request.dest)?;
        let mut hasher = sha2::Sha256::new();
        let mut downloaded = 0u64;
        let mut buf = [0u8; 64 * 1024];
        loop {
            if request.cancel.load(Ordering::Relaxed) {
                drop(file);
                let _ = std::fs::remove_file(request.dest);
                return Err(Error::msg("download cancelled"));
            }
            let n = response.read(&mut buf).map_err(|e| Error::Network(e.to_string()))?;
            if n == 0 {
                break;
            }
            downloaded = downloaded.saturating_add(n as u64);
            if downloaded > MAX_DOWNLOAD {
                drop(file);
                let _ = std::fs::remove_file(request.dest);
                return Err(Error::msg("the package is larger than the 2 GB limit"));
            }
            file.write_all(&buf[..n])?;
            hasher.update(&buf[..n]);
            (request.on_progress)(downloaded, total);
        }
        file.sync_all()?;
        if request.expected_size > 0 && downloaded != request.expected_size {
            let _ = std::fs::remove_file(request.dest);
            return Err(Error::msg("the download size does not match the catalog"));
        }
        let got = hex_encode(&hasher.finalize());
        if !got.eq_ignore_ascii_case(request.expected_sha256.trim()) {
            let _ = std::fs::remove_file(request.dest);
            return Err(Error::msg("the package hash does not match the catalog"));
        }
        Ok(())
    }

    fn send<T: Serialize>(&self, method: Method, path: &str, body: Option<&T>, access: Option<&str>, idempotency: Option<&str>) -> Result<Raw> {
        let url = format!("{}{}", self.base, path);
        let mut builder = match method {
            Method::Get => self.http.get(&url),
            Method::Post => self.http.post(&url),
        };
        builder = builder.header("X-Launcher-Version", &self.version).header("Accept", "application/json");
        if let Some(token) = access {
            builder = builder.header("Authorization", format!("Bearer {token}"));
        }
        if let Some(key) = idempotency {
            builder = builder.header("Idempotency-Key", key);
        }
        if let Some(body) = body {
            builder = builder.json(body);
        }
        let response = builder.send().map_err(network_error)?;
        let status = response.status().as_u16();
        let retry_after = response.headers().get("retry-after").and_then(|v| v.to_str().ok()).and_then(parse_retry_after);
        let body = response.bytes().map_err(network_error)?.to_vec();
        Ok(Raw { status, retry_after, body })
    }
}

pub struct Download<'a> {
    pub url: &'a str,
    pub dest: &'a Path,
    pub expected_sha256: &'a str,
    pub expected_size: u64,
    pub cancel: &'a AtomicBool,
    pub on_progress: &'a mut dyn FnMut(u64, u64),
}

#[derive(Clone, Copy)]
enum Method {
    Get,
    Post,
}

struct Raw {
    status: u16,
    retry_after: Option<u64>,
    body: Vec<u8>,
}

fn network_error(err: reqwest::Error) -> Error {
    if err.is_connect() {
        Error::Network(format!("connect: {err}"))
    } else if err.is_timeout() {
        Error::Network(format!("timeout: {err}"))
    } else {
        Error::Network(err.to_string())
    }
}

fn expect_status(raw: &Raw, ok: &[u16]) -> Result<()> {
    if ok.contains(&raw.status) { Ok(()) } else { Err(api_error(raw)) }
}

fn api_error(raw: &Raw) -> Error {
    let parsed: Option<ErrorBody> = serde_json::from_slice(&raw.body).ok();
    if let Some(body) = parsed {
        Error::Api { status: raw.status, code: body.error.code, message: body.error.message, retry_after: raw.retry_after }
    } else {
        let message = String::from_utf8_lossy(&raw.body).trim().to_string();
        let message = if message.is_empty() { format!("HTTP {}", raw.status) } else { message };
        Error::Api { status: raw.status, code: "http".into(), message, retry_after: raw.retry_after }
    }
}

fn from_json<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    serde_json::from_slice(bytes).map_err(|e| Error::msg(format!("the server returned JSON the launcher does not understand ({e})")))
}

fn interpret_poll(raw: &Raw) -> Result<Poll> {
    if raw.status == 429 {
        return Err(api_error(raw));
    }
    if raw.status == 410 {
        return Ok(Poll::Expired);
    }
    if raw.status != 200 {
        return Err(api_error(raw));
    }
    let value: serde_json::Value = from_json(&raw.body)?;
    match value.get("status").and_then(serde_json::Value::as_str) {
        Some("pending") => Ok(Poll::Pending),
        Some("denied") => Ok(Poll::Denied),
        Some("expired") => Ok(Poll::Expired),
        Some("approved") => {
            let issued: Issued = from_json(&raw.body)?;
            Ok(Poll::Approved(issued))
        }
        _ => Err(Error::msg("the sign-in poll returned an unknown status")),
    }
}

fn parse_retry_after(value: &str) -> Option<u64> {
    value.trim().parse::<u64>().ok()
}

fn encode_segment(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for b in text.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(b as char),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

pub fn percent_encode(text: &str) -> String {
    encode_segment(text)
}

pub fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

fn null_string<'de, D: Deserializer<'de>>(deserializer: D) -> std::result::Result<String, D::Error> {
    Ok(Option::<String>::deserialize(deserializer)?.unwrap_or_default())
}

#[derive(Debug, Deserialize)]
struct ErrorBody {
    error: ErrorInfo,
}

#[derive(Debug, Deserialize)]
struct ErrorInfo {
    code: String,
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Health {
    pub ok: bool,
    #[serde(default)]
    pub env: String,
    #[serde(default)]
    pub oauth_configured: bool,
    #[serde(default)]
    pub server_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    #[serde(default, deserialize_with = "null_string")]
    pub picture: String,
    pub created_at: String,
    #[serde(default)]
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SessionRequest {
    pub device_id: String,
    pub device_name: String,
    pub launcher_version: String,
    pub os: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct SessionCreated {
    pub session_id: String,
    pub login_url: String,
    pub poll_interval: u64,
    pub expires_in: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Poll {
    Pending,
    Approved(Issued),
    Denied,
    Expired,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Issued {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub user: User,
}

#[derive(Debug, Serialize)]
struct RefreshRequest<'a> {
    refresh_token: &'a str,
    device_id: &'a str,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Refreshed {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
struct LogoutRequest<'a> {
    refresh_token: &'a str,
}

#[derive(Debug, Serialize)]
struct DevApprove<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstalledReport {
    pub app_id: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Heartbeat {
    pub device_id: String,
    pub device_name: String,
    pub launcher_version: String,
    pub os: String,
    pub installed: Vec<InstalledReport>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct HeartbeatAck {
    pub ok: bool,
    pub server_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstallEvent {
    pub device_id: String,
    pub app_id: String,
    pub event: String,
    pub version: String,
    pub from_version: Option<String>,
    pub error: Option<String>,
    pub at: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct EventAck {
    pub ok: bool,
    #[serde(default)]
    pub duplicate: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AdminMe {
    #[serde(default)]
    pub is_admin: bool,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AdminStats {
    pub users_total: u64,
    pub users_active_30d: u64,
    pub devices_total: u64,
    #[serde(default)]
    pub installs_by_app: Vec<AppInstallStats>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AppInstallStats {
    pub app_id: String,
    pub installed_now: u64,
    pub installs_total: u64,
    #[serde(default)]
    pub latest_version: String,
    pub on_latest: u64,
}

#[derive(Debug, Clone)]
pub struct UserQuery<'a> {
    pub cursor: Option<&'a str>,
    pub limit: u32,
    pub q: Option<&'a str>,
}

impl UserQuery<'_> {
    fn as_query(&self) -> String {
        let mut parts = vec![format!("limit={}", self.limit.clamp(1, 100))];
        if let Some(cursor) = self.cursor {
            parts.push(format!("cursor={}", percent_encode(cursor)));
        }
        if let Some(q) = self.q {
            parts.push(format!("q={}", percent_encode(q)));
        }
        parts.join("&")
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct UserPage {
    pub items: Vec<AdminUser>,
    #[serde(default)]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AdminUser {
    pub id: String,
    pub email: String,
    pub name: String,
    pub created_at: String,
    #[serde(default, deserialize_with = "null_string")]
    pub last_seen_at: String,
    pub devices: u64,
    #[serde(default)]
    pub apps: Vec<AdminUserApp>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AdminUserApp {
    pub app_id: String,
    pub version: String,
    #[serde(default)]
    pub installed_at: String,
}

pub fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_picture_may_be_null() {
        let user: User =
            serde_json::from_str(r#"{"id":"usr_1","email":"a@gmail.com","name":"A","picture":null,"created_at":"2026-11-10T08:00:00Z","is_admin":false}"#)
                .unwrap();
        assert!(user.picture.is_empty());
    }

    #[test]
    fn poll_statuses_match_the_contract() {
        let pending = interpret_poll(&Raw { status: 200, retry_after: None, body: br#"{"status":"pending"}"#.to_vec() }).unwrap();
        assert_eq!(pending, Poll::Pending);
        let expired = interpret_poll(&Raw { status: 410, retry_after: None, body: br#"{"status":"expired"}"#.to_vec() }).unwrap();
        assert_eq!(expired, Poll::Expired);
        let denied = interpret_poll(&Raw { status: 200, retry_after: None, body: br#"{"status":"denied"}"#.to_vec() }).unwrap();
        assert_eq!(denied, Poll::Denied);
        let limited = interpret_poll(&Raw {
            status: 429,
            retry_after: Some(1),
            body: br#"{"error":{"code":"rate_limited","message":"Polling too fast; respect poll_interval"}}"#.to_vec(),
        })
        .unwrap_err();
        assert!(limited.is_rate_limited());
        assert_eq!(limited.retry_after(), Some(Duration::from_secs(1)));
    }

    #[test]
    fn catalog_unavailable_is_a_typed_error() {
        let err = api_error(&Raw {
            status: 503,
            retry_after: Some(60),
            body: br#"{"error":{"code":"catalog_unavailable","message":"Catalog is not available yet, try again later"}}"#.to_vec(),
        });
        assert!(err.is_catalog_unavailable());
        assert_eq!(crate::error::backoff_secs(&err, 30), 60);
    }

    #[test]
    fn event_ack_treats_duplicate_as_success_shape() {
        let ack: EventAck = serde_json::from_str(r#"{"ok":true,"duplicate":true}"#).unwrap();
        assert!(ack.ok && ack.duplicate);
        let first: EventAck = serde_json::from_str(r#"{"ok":true}"#).unwrap();
        assert!(first.ok && !first.duplicate);
    }

    #[test]
    fn prod_client_refuses_dev_approve_without_a_request() {
        let client = ApiClient::new("https://slmptech.com/api/v1", "1.0.0").unwrap();
        let err = client.approve_dev_session("abc", None, None).unwrap_err();
        assert!(err.to_string().contains("refused"));
    }
}
