//! Local stand-in for the v1 API so the launcher can be tried before the backend is reachable.

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::api::now_rfc3339;
use crate::error::{Error, Result};
use crate::install::zip_bytes;
use smithcc_catalog::{AppRelease, Catalog, Package, PackageFormat};

#[derive(Debug, Clone)]
pub struct MockOptions {
    pub session_limit: u32,
    pub catalog_unavailable: bool,
}

impl Default for MockOptions {
    fn default() -> Self {
        Self { session_limit: 10, catalog_unavailable: false }
    }
}

pub struct MockServer {
    base: String,
    port: u16,
    #[cfg_attr(not(test), allow(dead_code))]
    state: Arc<Mutex<State>>,
    shutdown: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

struct State {
    options: MockOptions,
    sessions: HashMap<String, Session>,
    access: HashMap<String, String>,
    refresh: HashMap<String, Refresh>,
    revoked: HashSet<String>,
    events: Vec<serde_json::Value>,
    seen_keys: HashSet<String>,
    session_times: Vec<(String, Instant)>,
    next_token: u64,
    packages: HashMap<String, Vec<u8>>,
    catalog: Catalog,
}

struct Session {
    device_id: String,
    approved: bool,
    consumed: bool,
}

struct Refresh {
    device_id: String,
    replaced: bool,
}

struct UserRecord {
    id: &'static str,
    email: &'static str,
    name: &'static str,
    is_admin: bool,
}

const USER: UserRecord = UserRecord { id: "usr_mock", email: "ada@gmail.com", name: "Ada Lovelace", is_admin: true };

impl MockServer {
    pub fn start() -> Result<Self> {
        Self::start_with(MockOptions::default())
    }

    pub fn start_with(options: MockOptions) -> Result<Self> {
        let server = tiny_http::Server::http("127.0.0.1:0").map_err(|e| Error::msg(format!("mock server failed to bind ({e})")))?;
        let port = match server.server_addr() {
            tiny_http::ListenAddr::IP(addr) => addr.port(),
            _ => return Err(Error::msg("mock server did not bind a tcp port")),
        };
        let base = format!("http://127.0.0.1:{port}/api/v1");
        let (packages, catalog) = build_catalog(&base);
        let state = Arc::new(Mutex::new(State {
            options,
            sessions: HashMap::new(),
            access: HashMap::new(),
            refresh: HashMap::new(),
            revoked: HashSet::new(),
            events: Vec::new(),
            seen_keys: HashSet::new(),
            session_times: Vec::new(),
            next_token: 1,
            packages,
            catalog,
        }));
        let shutdown = Arc::new(AtomicBool::new(false));
        let state_thread = Arc::clone(&state);
        let stop = Arc::clone(&shutdown);
        let base_thread = base.clone();
        let thread = std::thread::spawn(move || {
            loop {
                if stop.load(Ordering::Relaxed) {
                    break;
                }
                let Ok(Some(mut request)) = server.recv_timeout(Duration::from_millis(200)) else {
                    continue;
                };
                let url = request.url().to_string();
                let method = request.method().as_str().to_string();
                let headers = header_map(&request);
                let mut body = Vec::new();
                let _ = request.as_reader().read_to_end(&mut body);
                let response = route(&base_thread, &method, &url, &headers, &body, &state_thread);
                let _ = request.respond(response);
            }
        });
        Ok(Self { base, port, state, shutdown, thread: Some(thread) })
    }

    pub fn base_url(&self) -> String {
        self.base.clone()
    }

    #[cfg(test)]
    pub fn event_count(&self) -> usize {
        lock(&self.state).events.len()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
        let _ = std::net::TcpStream::connect_timeout(&std::net::SocketAddr::from(([127, 0, 0, 1], self.port)), Duration::from_millis(200));
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn lock(state: &Mutex<State>) -> std::sync::MutexGuard<'_, State> {
    state.lock().unwrap_or_else(|err| err.into_inner())
}

fn header_map(request: &tiny_http::Request) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for header in request.headers() {
        let name = header.field.as_str().as_str().to_ascii_lowercase();
        let value = std::str::from_utf8(header.value.as_bytes()).unwrap_or("").to_string();
        map.insert(name, value);
    }
    map
}

fn route(
    base: &str,
    method: &str,
    url: &str,
    headers: &HashMap<String, String>,
    body: &[u8],
    state: &Mutex<State>,
) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let (path, query) = url.split_once('?').unwrap_or((url, ""));
    let path = path.trim_end_matches('/');
    let api = path.strip_prefix("/api/v1").unwrap_or(path);
    if method == "GET" && api == "/health" {
        return json(200, &serde_json::json!({"ok": true, "env": "mock", "oauth_configured": true, "server_time": now_rfc3339()}));
    }
    if method == "GET" && api == "/catalog" {
        let state = lock(state);
        if state.options.catalog_unavailable {
            return with_retry(json(503, &err("catalog_unavailable", "Catalog is not available yet, try again later")), 60);
        }
        let body = serde_json::to_value(&state.catalog).unwrap_or(serde_json::Value::Null);
        return json(200, &body);
    }
    if method == "GET"
        && let Some(name) = api.strip_prefix("/packages/")
    {
        let state = lock(state);
        if let Some(bytes) = state.packages.get(name) {
            return bytes_response(200, bytes.clone(), "application/zip");
        }
        return json(404, &err("not_found", "package not found"));
    }
    if method == "POST" && api == "/auth/sessions" {
        return create_session(base, body, state);
    }
    if method == "GET" && api.starts_with("/auth/sessions/") {
        let id = api.trim_start_matches("/auth/sessions/");
        return poll_session(id, state);
    }
    if method == "GET" && api == "/auth/google/start" {
        let session = query_value(query, "session");
        return approve_page(&session, state);
    }
    if method == "POST" && api.starts_with("/dev/sessions/") && api.ends_with("/approve") {
        let id = api.trim_start_matches("/dev/sessions/").trim_end_matches("/approve").trim_matches('/');
        return dev_approve(id, state);
    }
    if method == "POST" && api == "/auth/refresh" {
        return refresh(body, state);
    }
    if method == "POST" && api == "/auth/logout" {
        return logout(headers, body, state);
    }
    if method == "GET" && api == "/me" {
        return me(headers, state);
    }
    if method == "GET" && api == "/admin/me" {
        return admin_me(headers, state);
    }
    if method == "GET" && api == "/admin/stats" {
        return admin_stats(headers, state);
    }
    if method == "GET" && api == "/admin/users" {
        return admin_users(headers, state);
    }
    if method == "POST" && api == "/devices/heartbeat" {
        return heartbeat(headers, body, state);
    }
    if method == "POST" && api == "/installs/events" {
        return install_event(headers, body, state);
    }
    json(404, &err("not_found", "unknown route"))
}

fn create_session(base: &str, body: &[u8], state: &Mutex<State>) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let parsed: serde_json::Value = match serde_json::from_slice(body) {
        Ok(value) => value,
        Err(_) => return json(400, &err("bad_request", "invalid JSON")),
    };
    let device_id = parsed.get("device_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
    if device_id.is_empty() {
        return json(400, &err("bad_request", "device_id is required"));
    }
    let mut state = lock(state);
    let now = Instant::now();
    state.session_times.retain(|(_, at)| now.duration_since(*at) < Duration::from_secs(60));
    let recent = state.session_times.iter().filter(|(id, _)| id == &device_id).count();
    if recent >= state.options.session_limit as usize {
        return with_retry(json(429, &err("rate_limited", "Too many sign-in sessions")), 60);
    }
    state.session_times.push((device_id.clone(), now));
    let id = format!("sess{:032x}", state.next_token);
    state.next_token = state.next_token.saturating_add(1);
    state.sessions.insert(id.clone(), Session { device_id, approved: false, consumed: false });
    let login_url = format!("{base}/auth/google/start?session={id}");
    json(
        201,
        &serde_json::json!({
            "session_id": id,
            "login_url": login_url,
            "poll_interval": 1,
            "expires_in": 600
        }),
    )
}

fn approve_page(session: &str, state: &Mutex<State>) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    mark_approved(session, state);
    html(
        "<!doctype html><meta charset=\"utf-8\"><title>Smith Creative Cloud</title><body style=\"font-family:sans-serif;background:#16161a;color:#f4f6fb\"><main><h1>You can return to Smith Creative Cloud</h1><p>This window approved a local developer sign-in.</p></main>",
    )
}

fn dev_approve(session: &str, state: &Mutex<State>) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    if !mark_approved(session, state) {
        return json(404, &err("not_found", "session not found"));
    }
    json(200, &serde_json::json!({"ok": true, "session_id": session, "dev_fake_login": true}))
}

fn mark_approved(session: &str, state: &Mutex<State>) -> bool {
    let mut state = lock(state);
    let device = {
        let Some(found) = state.sessions.get_mut(session) else {
            return false;
        };
        found.approved = true;
        found.device_id.clone()
    };
    state.revoked.remove(&device);
    true
}

fn poll_session(id: &str, state: &Mutex<State>) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let mut state = lock(state);
    let Some(session) = state.sessions.get(id).map(|s| (s.device_id.clone(), s.approved, s.consumed)) else {
        return json(410, &serde_json::json!({"status": "expired"}));
    };
    if session.2 {
        return json(410, &serde_json::json!({"status": "expired"}));
    }
    if !session.1 {
        return json(200, &serde_json::json!({"status": "pending"}));
    }
    let (access, refresh) = issue_tokens(&mut state, &session.0);
    if let Some(found) = state.sessions.get_mut(id) {
        found.consumed = true;
    }
    json(
        200,
        &serde_json::json!({
            "status": "approved",
            "access_token": access,
            "expires_in": 3600,
            "refresh_token": refresh,
            "user": user_json()
        }),
    )
}

fn issue_tokens(state: &mut State, device_id: &str) -> (String, String) {
    let n = state.next_token;
    state.next_token = state.next_token.saturating_add(1);
    let access = format!("mock-access.{device_id}.{n}");
    let refresh = format!("mock-refresh.{device_id}.{n}");
    state.access.insert(access.clone(), device_id.to_string());
    state.refresh.insert(refresh.clone(), Refresh { device_id: device_id.to_string(), replaced: false });
    (access, refresh)
}

fn refresh(body: &[u8], state: &Mutex<State>) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let parsed: serde_json::Value = match serde_json::from_slice(body) {
        Ok(value) => value,
        Err(_) => return json(400, &err("bad_request", "invalid JSON")),
    };
    let token = parsed.get("refresh_token").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let device_id = parsed.get("device_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let mut state = lock(state);
    let Some(found) = state.refresh.get(&token).map(|r| (r.device_id.clone(), r.replaced)) else {
        return json(401, &err("invalid_grant", "Refresh token already used"));
    };
    if found.1 || state.revoked.contains(&found.0) || found.0 != device_id {
        state.revoked.insert(found.0);
        if let Some(row) = state.refresh.get_mut(&token) {
            row.replaced = true;
        }
        return json(401, &err("invalid_grant", "Refresh token already used"));
    }
    if let Some(row) = state.refresh.get_mut(&token) {
        row.replaced = true;
    }
    let (access, refresh_token) = issue_tokens(&mut state, &device_id);
    json(200, &serde_json::json!({"access_token": access, "expires_in": 3600, "refresh_token": refresh_token}))
}

fn logout(headers: &HashMap<String, String>, body: &[u8], state: &Mutex<State>) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let Some(device) = bearer_device(headers, state) else {
        return json(401, &err("unauthorized", "Missing bearer token"));
    };
    let parsed: serde_json::Value = serde_json::from_slice(body).unwrap_or(serde_json::Value::Null);
    let token = parsed.get("refresh_token").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let mut state = lock(state);
    if let Some(row) = state.refresh.get_mut(&token)
        && row.device_id == device
    {
        row.replaced = true;
    }
    state.revoked.insert(device);
    empty(204)
}

fn me(headers: &HashMap<String, String>, state: &Mutex<State>) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    if bearer_device(headers, state).is_none() {
        return json(401, &err("unauthorized", "Missing bearer token"));
    }
    json(200, &user_json())
}

fn admin_me(headers: &HashMap<String, String>, state: &Mutex<State>) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    if bearer_device(headers, state).is_none() {
        return json(401, &err("unauthorized", "Missing bearer token"));
    }
    if !USER.is_admin {
        return json(403, &err("forbidden", "Admin access required"));
    }
    json(200, &user_json())
}

fn admin_stats(headers: &HashMap<String, String>, state: &Mutex<State>) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    if bearer_device(headers, state).is_none() {
        return json(401, &err("unauthorized", "Missing bearer token"));
    }
    if !USER.is_admin {
        return json(403, &err("forbidden", "Admin access required"));
    }
    let state = lock(state);
    json(
        200,
        &serde_json::json!({
            "users_total": 1,
            "users_active_30d": 1,
            "devices_total": state.access.len().max(1),
            "installs_by_app": []
        }),
    )
}

fn admin_users(headers: &HashMap<String, String>, state: &Mutex<State>) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    if bearer_device(headers, state).is_none() {
        return json(401, &err("unauthorized", "Missing bearer token"));
    }
    json(
        200,
        &serde_json::json!({
            "items": [{"id": USER.id, "email": USER.email, "name": USER.name, "created_at": "2026-11-10T00:00:00Z", "last_seen_at": now_rfc3339(), "devices": 1, "apps": []}],
            "next_cursor": null
        }),
    )
}

fn heartbeat(headers: &HashMap<String, String>, body: &[u8], state: &Mutex<State>) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let Some(device) = bearer_device(headers, state) else {
        return json(401, &err("unauthorized", "Missing bearer token"));
    };
    let parsed: serde_json::Value = match serde_json::from_slice(body) {
        Ok(value) => value,
        Err(_) => return json(400, &err("bad_request", "invalid JSON")),
    };
    let claimed = parsed.get("device_id").and_then(|v| v.as_str()).unwrap_or("");
    if claimed != device {
        return json(403, &err("device_mismatch", "device_id does not match the access token"));
    }
    json(200, &serde_json::json!({"ok": true, "server_time": now_rfc3339()}))
}

fn install_event(headers: &HashMap<String, String>, body: &[u8], state: &Mutex<State>) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let Some(device) = bearer_device(headers, state) else {
        return json(401, &err("unauthorized", "Missing bearer token"));
    };
    let key = headers.get("idempotency-key").map(String::as_str).unwrap_or("").trim();
    if key.is_empty() {
        return json(400, &err("idempotency_key_required", "Idempotency-Key header is required"));
    }
    let parsed: serde_json::Value = match serde_json::from_slice(body) {
        Ok(value) => value,
        Err(_) => return json(400, &err("bad_request", "invalid JSON")),
    };
    let claimed = parsed.get("device_id").and_then(|v| v.as_str()).unwrap_or("");
    if claimed != device {
        return json(403, &err("device_mismatch", "device_id does not match the access token"));
    }
    let mut state = lock(state);
    if !state.seen_keys.insert(key.to_string()) {
        return json(202, &serde_json::json!({"ok": true, "duplicate": true}));
    }
    state.events.push(parsed);
    json(202, &serde_json::json!({"ok": true}))
}

fn bearer_device(headers: &HashMap<String, String>, state: &Mutex<State>) -> Option<String> {
    let header = headers.get("authorization")?;
    let token = header.strip_prefix("Bearer ").or_else(|| header.strip_prefix("bearer "))?;
    let state = lock(state);
    let device = state.access.get(token)?.clone();
    if state.revoked.contains(&device) {
        return None;
    }
    Some(device)
}

fn user_json() -> serde_json::Value {
    serde_json::json!({
        "id": USER.id,
        "email": USER.email,
        "name": USER.name,
        "picture": null,
        "created_at": "2026-11-10T00:00:00Z",
        "is_admin": USER.is_admin
    })
}

fn build_catalog(base: &str) -> (HashMap<String, Vec<u8>>, Catalog) {
    let metas = smithcc_catalog::app_meta().unwrap_or_default();
    let mut packages = HashMap::new();
    let mut apps = Vec::new();
    for meta in metas {
        let entry = format!("{}.exe", meta.id);
        let content = format!("dummy {} 1.0.0\n", meta.id);
        let Ok(bytes) = zip_bytes(&[(entry.as_str(), content.as_bytes())]) else { continue };
        let sha = sha256(&bytes);
        let name = format!("{}/1.0.0.zip", meta.id);
        packages.insert(name.clone(), bytes.clone());
        apps.push(AppRelease {
            id: meta.id.clone(),
            name: meta.name,
            tagline: meta.tagline,
            icon_url: String::new(),
            version: "1.0.0".into(),
            released_at: "2026-11-10T00:00:00Z".into(),
            notes: "Mock build".into(),
            package: Package { format: PackageFormat::Zip, url: format!("{base}/packages/{name}"), sha256: sha, size: bytes.len() as u64, entry },
            min_launcher: "1.0.0".into(),
        });
    }
    let catalog = Catalog { generated_at: "2026-11-10T00:00:00Z".into(), launcher: None, apps };
    (packages, catalog)
}

fn sha256(bytes: &[u8]) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(bytes);
    crate::api::hex_encode(hasher.finalize().as_slice())
}

fn query_value(query: &str, key: &str) -> String {
    query.split('&').filter_map(|pair| pair.split_once('=')).find(|(k, _)| *k == key).map(|(_, v)| v.to_string()).unwrap_or_default()
}

fn err(code: &str, message: &str) -> serde_json::Value {
    serde_json::json!({"error": {"code": code, "message": message}})
}

fn json(status: u16, value: &serde_json::Value) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let bytes = serde_json::to_vec(value).unwrap_or_else(|_| b"{}".to_vec());
    bytes_response(status, bytes, "application/json")
}

fn html(body: &str) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    bytes_response(200, body.as_bytes().to_vec(), "text/html; charset=utf-8")
}

fn empty(status: u16) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    bytes_response(status, Vec::new(), "application/json")
}

fn with_retry(response: tiny_http::Response<std::io::Cursor<Vec<u8>>>, secs: u64) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    if let Ok(header) = tiny_http::Header::from_bytes(b"Retry-After", secs.to_string().as_bytes()) { response.with_header(header) } else { response }
}

fn bytes_response(status: u16, bytes: Vec<u8>, content_type: &str) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let mut response = tiny_http::Response::from_data(bytes);
    response = response.with_status_code(tiny_http::StatusCode::from(status));
    if let Ok(header) = tiny_http::Header::from_bytes(b"Content-Type", content_type.as_bytes()) {
        response = response.with_header(header);
    }
    if let Ok(header) = tiny_http::Header::from_bytes(b"Cache-Control", b"no-store") {
        response = response.with_header(header);
    }
    response
}
