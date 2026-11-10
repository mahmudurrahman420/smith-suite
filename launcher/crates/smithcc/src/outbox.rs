//! Offline queue for heartbeats and install events.
//!
//! Each install event keeps the Idempotency-Key it was created with, so a retry is the same
//! event and the server can answer `{ok:true, duplicate:true}`.

use serde::{Deserialize, Serialize};

use crate::api::{ApiClient, EventAck, Heartbeat, InstallEvent};
use crate::error::{Error, Result};
use crate::paths::Paths;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Outbox {
    #[serde(default)]
    pub jobs: Vec<Queued>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Queued {
    Event { idempotency_key: String, body: InstallEvent, attempts: u32 },
    Heartbeat { body: Heartbeat, attempts: u32 },
}

impl Outbox {
    pub fn load(paths: &Paths) -> Result<Self> {
        let path = paths.outbox_file();
        if !path.exists() {
            return Ok(Self::default());
        }
        let text = std::fs::read_to_string(path)?;
        serde_json::from_str(&text).map_err(|e| Error::msg(format!("the offline queue is unreadable ({e})")))
    }

    pub fn save(&self, paths: &Paths) -> Result<()> {
        if let Some(parent) = paths.outbox_file().parent() {
            std::fs::create_dir_all(parent)?;
        }
        let body = serde_json::to_vec_pretty(self).map_err(|e| Error::msg(e.to_string()))?;
        std::fs::write(paths.outbox_file(), body)?;
        Ok(())
    }

    pub fn enqueue_event(&mut self, body: InstallEvent) {
        let idempotency_key = uuid::Uuid::new_v4().to_string();
        self.jobs.push(Queued::Event { idempotency_key, body, attempts: 0 });
    }

    pub fn enqueue_heartbeat(&mut self, body: Heartbeat) {
        self.jobs.push(Queued::Heartbeat { body, attempts: 0 });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlushStop {
    Done,
    Auth,
    Later,
}

pub fn flush(api: &ApiClient, access: &str, outbox: &mut Outbox) -> Result<FlushStop> {
    let mut kept = Vec::new();
    let mut stop = FlushStop::Done;
    for job in outbox.jobs.drain(..) {
        if stop != FlushStop::Done {
            kept.push(job);
            continue;
        }
        match send_one(api, access, job) {
            Send::Gone => {}
            Send::Keep(job) => kept.push(job),
            Send::Stop(reason, job) => {
                kept.push(job);
                stop = reason;
            }
        }
    }
    outbox.jobs = kept;
    Ok(stop)
}

enum Send {
    Gone,
    Keep(Queued),
    Stop(FlushStop, Queued),
}

fn send_one(api: &ApiClient, access: &str, job: Queued) -> Send {
    match job {
        Queued::Event { idempotency_key, body, attempts } => match api.post_event(access, &idempotency_key, &body) {
            Ok(EventAck { ok: true, .. }) => Send::Gone,
            Ok(_) => Send::Keep(bump_event(idempotency_key, body, attempts)),
            Err(err) if err.is_unauthorized() || err.is_device_mismatch() => Send::Stop(FlushStop::Auth, Queued::Event { idempotency_key, body, attempts }),
            Err(err) if err.is_network() || err.is_rate_limited() => Send::Stop(FlushStop::Later, Queued::Event { idempotency_key, body, attempts }),
            Err(_) if attempts + 1 >= 8 => Send::Gone,
            Err(_) => Send::Keep(bump_event(idempotency_key, body, attempts)),
        },
        Queued::Heartbeat { body, attempts } => match api.heartbeat(access, &body) {
            Ok(ack) if ack.ok => Send::Gone,
            Ok(_) => Send::Keep(Queued::Heartbeat { body, attempts: attempts.saturating_add(1) }),
            Err(err) if err.is_unauthorized() || err.is_device_mismatch() => Send::Stop(FlushStop::Auth, Queued::Heartbeat { body, attempts }),
            Err(err) if err.is_network() || err.is_rate_limited() => Send::Stop(FlushStop::Later, Queued::Heartbeat { body, attempts }),
            Err(_) if attempts + 1 >= 8 => Send::Gone,
            Err(_) => Send::Keep(Queued::Heartbeat { body, attempts: attempts.saturating_add(1) }),
        },
    }
}

fn bump_event(idempotency_key: String, body: InstallEvent, attempts: u32) -> Queued {
    Queued::Event { idempotency_key, body, attempts: attempts.saturating_add(1) }
}
