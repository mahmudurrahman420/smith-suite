//! Background work: sign-in, catalog, downloads, heartbeats. The window thread only draws.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::api::{ApiClient, Heartbeat, InstallEvent, LAUNCHER_VERSION, Poll, SessionRequest, User, now_rfc3339};
use crate::device::Device;
use crate::endpoint::Environment;
use crate::error::{Error, Result, backoff_secs};
use crate::install::{install_package, open_entry, recover_interrupted, uninstall_app};
use crate::outbox::{FlushStop, Outbox, flush};
use crate::paths::{Paths, log_line};
use crate::state::{self, InstallDb};
use crate::store::open_store;
use crate::vault::Vault;

pub const CHECK_EVERY: Duration = Duration::from_secs(6 * 60 * 60);

pub enum Job {
    Restore,
    SignIn { dev_user: bool },
    Cancel,
    SignOut,
    RefreshCatalog,
    Install { app_id: String },
    Uninstall { app_id: String },
    Open { app_id: String },
    SelfUpdate,
    LoadAdmin,
    Heartbeat,
}

pub enum Event {
    Restoring,
    SignedOut { message: String },
    Ready { user: User },
    Health(crate::api::Health),
    Catalog(smithcc_catalog::Catalog),
    CatalogUnavailable { message: String },
    Banner(String),
    Status(String),
    Polling(bool),
    Progress { app_id: String, fraction: f32 },
    Installs(InstallDb),
    SelfUpdate(Option<smithcc_catalog::LauncherRelease>),
    Admin(crate::api::AdminStats),
    Users(Vec<crate::api::AdminUser>),
    Exit,
}

pub struct Worker {
    pub tx: Sender<Job>,
    pub rx: Receiver<Event>,
    join: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn disconnected() -> Self {
        let (tx, _rx) = std::sync::mpsc::channel();
        let (_tx, rx) = std::sync::mpsc::channel();
        Self { tx, rx, join: None }
    }
}

impl Worker {
    pub fn spawn(paths: Paths, device: Device, environment: Environment, base: String) -> Result<Self> {
        let (job_tx, job_rx) = std::sync::mpsc::channel();
        let (event_tx, event_rx) = std::sync::mpsc::channel();
        let api = ApiClient::new(&base, LAUNCHER_VERSION)?;
        let join = thread::spawn(move || {
            if let Err(err) = run(job_rx, event_tx.clone(), paths, device, environment, api) {
                let _ = event_tx.send(Event::Banner(err.to_string()));
            }
        });
        Ok(Self { tx: job_tx, rx: event_rx, join: Some(join) })
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        // Detach. Joining here would wait out a download while the window is closing.
        drop(self.join.take());
    }
}

struct Ctx {
    paths: Paths,
    device: Device,
    environment: Environment,
    api: ApiClient,
    vault: Vault,
    outbox: Outbox,
    installs: InstallDb,
    catalog: Option<smithcc_catalog::Catalog>,
    cancel: Arc<AtomicBool>,
    events: Sender<Event>,
}

fn run(jobs: Receiver<Job>, events: Sender<Event>, paths: Paths, device: Device, environment: Environment, api: ApiClient) -> Result<()> {
    let mut vault = Vault::new(open_store(&paths, environment));
    vault.load()?;
    let outbox = Outbox::load(&paths)?;
    let installs = InstallDb::load(&paths)?;
    for id in installs.apps.keys() {
        let _ = recover_interrupted(&paths, id);
    }
    let catalog = state::load_catalog(&paths)?;
    let mut ctx = Ctx { paths, device, environment, api, vault, outbox, installs, catalog, cancel: Arc::new(AtomicBool::new(false)), events };
    while let Ok(job) = jobs.recv() {
        if let Err(err) = ctx.handle(job) {
            log_line(&ctx.paths, &err.to_string());
            ctx.emit(Event::Banner(err.to_string()));
        }
    }
    Ok(())
}

impl Ctx {
    fn handle(&mut self, job: Job) -> Result<()> {
        match job {
            Job::Restore => self.restore(),
            Job::SignIn { dev_user } => self.sign_in(dev_user),
            Job::Cancel => {
                self.cancel.store(true, Ordering::Relaxed);
                Ok(())
            }
            Job::SignOut => self.sign_out(),
            Job::RefreshCatalog => self.refresh_catalog(),
            Job::Install { app_id } => self.install(&app_id, false),
            Job::Uninstall { app_id } => self.uninstall(&app_id),
            Job::Open { app_id } => self.open_app(&app_id),
            Job::SelfUpdate => self.self_update(),
            Job::LoadAdmin => self.load_admin(),
            Job::Heartbeat => self.heartbeat(true),
        }
    }

    fn restore(&mut self) -> Result<()> {
        self.emit(Event::Restoring);
        let _ = self.check_health();
        if self.vault.has_refresh() {
            match self.vault.rotate(&self.api, &self.device.id) {
                Ok(()) => self.finish_sign_in()?,
                Err(err) if err.is_connect() => {
                    if let Some(user) = state::load_user(&self.paths, self.environment)? {
                        self.emit(Event::Ready { user });
                        self.emit(Event::Banner("Offline. Saved installs still open, and activity is reported when you reconnect.".into()));
                        self.publish_cached_catalog();
                    } else {
                        self.emit(Event::SignedOut { message: "Couldn't reach Smith Creative Cloud.".into() });
                    }
                }
                Err(err) => {
                    self.vault.clear()?;
                    self.emit(Event::SignedOut { message: err.to_string() });
                }
            }
        } else {
            self.emit(Event::SignedOut { message: String::new() });
        }
        Ok(())
    }

    fn sign_in(&mut self, dev_user: bool) -> Result<()> {
        self.cancel.store(false, Ordering::Relaxed);
        self.emit(Event::Polling(true));
        self.emit(Event::Status("Starting sign-in…".into()));
        let created = self.create_session_with_backoff()?;
        if dev_user {
            self.emit(Event::Status("Approving the staging test user…".into()));
            self.api.approve_dev_session(&created.session_id, Some("smithcc-launcher@example.com"), Some("Smith Creative Cloud"))?;
        } else {
            self.emit(Event::Status("Waiting for the browser…".into()));
            if let Err(err) = open::that(&created.login_url) {
                self.emit(Event::Banner(format!("Open this link in your browser: {}", created.login_url)));
                log_line(&self.paths, &err.to_string());
            }
        }
        let interval = Duration::from_secs(created.poll_interval.clamp(1, 30));
        let deadline = Instant::now() + Duration::from_secs(created.expires_in.max(30));
        let issued = loop {
            if self.cancel.load(Ordering::Relaxed) {
                self.emit(Event::Polling(false));
                self.emit(Event::SignedOut { message: "Sign-in cancelled.".into() });
                return Ok(());
            }
            if Instant::now() >= deadline {
                self.emit(Event::Polling(false));
                self.emit(Event::SignedOut { message: "The sign-in window expired. Try again.".into() });
                return Ok(());
            }
            match self.api.poll_session(&created.session_id) {
                Ok(Poll::Pending) => self.sleep_interruptible(interval)?,
                Ok(Poll::Approved(issued)) => break issued,
                Ok(Poll::Denied) => {
                    self.emit(Event::Polling(false));
                    self.emit(Event::SignedOut { message: "Sign-in was denied.".into() });
                    return Ok(());
                }
                Ok(Poll::Expired) => {
                    self.emit(Event::Polling(false));
                    self.emit(Event::SignedOut { message: "The sign-in window expired. Try again.".into() });
                    return Ok(());
                }
                Err(err) if err.is_rate_limited() => self.sleep_interruptible(Duration::from_secs(backoff_secs(&err, created.poll_interval.max(1))))?,
                Err(err) => return Err(err),
            }
        };
        self.vault.accept_issued(&issued)?;
        self.emit(Event::Polling(false));
        self.finish_sign_in()
    }

    fn create_session_with_backoff(&mut self) -> Result<crate::api::SessionCreated> {
        let body = SessionRequest {
            device_id: self.device.id.clone(),
            device_name: self.device.name.clone(),
            launcher_version: LAUNCHER_VERSION.to_string(),
            os: self.device.os.clone(),
        };
        match self.api.create_session(&body) {
            Ok(created) => Ok(created),
            Err(err) if err.is_rate_limited() => {
                let wait = backoff_secs(&err, 60);
                self.emit(Event::Status(format!("Too many sign-in attempts. Trying again in {wait}s…")));
                self.sleep_interruptible(Duration::from_secs(wait))?;
                self.api.create_session(&body)
            }
            Err(err) => Err(err),
        }
    }

    fn finish_sign_in(&mut self) -> Result<()> {
        let access = self.vault.access(&self.api, &self.device.id)?;
        let user = self.api.me(&access)?;
        state::save_user(&self.paths, self.environment, &user)?;
        self.emit(Event::Ready { user });
        let _ = self.refresh_catalog();
        let _ = self.heartbeat(true);
        let _ = self.flush_outbox();
        Ok(())
    }

    fn sign_out(&mut self) -> Result<()> {
        if let Ok(access) = self.vault.access(&self.api, &self.device.id)
            && let Some(refresh) = self.current_refresh_for_logout()
        {
            let _ = self.api.logout(&access, &refresh);
        }
        self.vault.clear()?;
        state::clear_user(&self.paths, self.environment)?;
        self.emit(Event::SignedOut { message: String::new() });
        Ok(())
    }

    fn current_refresh_for_logout(&mut self) -> Option<String> {
        // The vault does not expose the refresh token. Load the copy that was just saved.
        open_store(&self.paths, self.environment).load().ok().flatten()
    }

    fn check_health(&mut self) -> Result<()> {
        match self.api.health() {
            Ok(health) => {
                self.emit(Event::Health(health));
                Ok(())
            }
            Err(err) => {
                log_line(&self.paths, &format!("health: {err}"));
                Err(err)
            }
        }
    }

    fn refresh_catalog(&mut self) -> Result<()> {
        match self.api.catalog() {
            Ok(catalog) => {
                state::save_catalog(&self.paths, &catalog)?;
                self.note_launcher(&catalog);
                self.catalog = Some(catalog.clone());
                self.emit(Event::Catalog(catalog));
                Ok(())
            }
            Err(err) if err.is_catalog_unavailable() => {
                self.publish_cached_catalog();
                self.emit(Event::CatalogUnavailable {
                    message:
                        "The app catalog isn't available yet. It appears after the first Windows release is published. Smith Creative Cloud will keep checking."
                            .into(),
                });
                Ok(())
            }
            Err(err) if err.is_network() => {
                self.publish_cached_catalog();
                self.emit(Event::Banner("Couldn't refresh the catalog. Showing the last copy saved on this PC.".into()));
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn publish_cached_catalog(&mut self) {
        if let Some(catalog) = &self.catalog {
            self.emit(Event::Catalog(catalog.clone()));
        }
    }

    fn note_launcher(&mut self, catalog: &smithcc_catalog::Catalog) {
        let update = catalog.launcher.as_ref().filter(|rel| crate::version::is_newer(&rel.version, LAUNCHER_VERSION)).cloned();
        self.emit(Event::SelfUpdate(update));
    }

    fn install(&mut self, app_id: &str, _force: bool) -> Result<()> {
        let Some(catalog) = self.catalog.clone() else {
            return Err(Error::msg("the catalog isn't available yet"));
        };
        let Some(app) = catalog.apps.into_iter().find(|app| app.id == app_id) else {
            return Err(Error::msg("that app has not been published"));
        };
        let from = self.installs.apps.get(&app.id).map(|row| row.version.clone());
        let event_name = if from.is_some() { "update" } else { "install" };
        self.emit(Event::Progress { app_id: app.id.clone(), fraction: 0.0 });
        let dest = self.paths.downloads().join(format!("{}-{}.part", app.id, app.version));
        let cancel = Arc::clone(&self.cancel);
        self.cancel.store(false, Ordering::Relaxed);
        let events = self.events.clone();
        let id = app.id.clone();
        let mut last = Instant::now() - Duration::from_secs(1);
        let result = self.api.download(crate::api::Download {
            url: &app.package.url,
            dest: &dest,
            expected_sha256: &app.package.sha256,
            expected_size: app.package.size,
            cancel: &cancel,
            on_progress: &mut |got, total| {
                if last.elapsed() < Duration::from_millis(150) && got != total {
                    return;
                }
                last = Instant::now();
                let fraction = if total == 0 { 0.0 } else { got as f32 / total as f32 };
                let _ = events.send(Event::Progress { app_id: id.clone(), fraction });
            },
        });
        if let Err(err) = result {
            let _ = std::fs::remove_file(&dest);
            self.fail_event(&app.id, &app.version, from.as_deref(), &err.to_string());
            self.emit(Event::Progress { app_id: app.id, fraction: -1.0 });
            return Err(err);
        }
        let format = app.package.format;
        let entry = install_package(&self.paths, &app.id, &app.name, format, &dest, &app.package.entry);
        let _ = std::fs::remove_file(&dest);
        let entry = match entry {
            Ok(path) => path,
            Err(err) => {
                self.fail_event(&app.id, &app.version, from.as_deref(), &err.to_string());
                self.emit(Event::Progress { app_id: app.id, fraction: -1.0 });
                return Err(err);
            }
        };
        let _ = entry;
        self.installs
            .apps
            .insert(app.id.clone(), state::InstalledApp { version: app.version.clone(), entry: app.package.entry.clone(), installed_at: now_rfc3339() });
        self.installs.save(&self.paths)?;
        self.emit(Event::Installs(self.installs.clone()));
        self.emit(Event::Progress { app_id: app.id.clone(), fraction: -1.0 });
        self.queue_event(&app.id, event_name, &app.version, from.as_deref(), None);
        let _ = self.flush_outbox();
        let _ = self.heartbeat(false);
        Ok(())
    }

    fn uninstall(&mut self, app_id: &str) -> Result<()> {
        let name = self.catalog.as_ref().and_then(|c| c.apps.iter().find(|a| a.id == app_id)).map(|a| a.name.clone()).unwrap_or_else(|| app_id.to_string());
        let version = self.installs.apps.get(app_id).map(|a| a.version.clone()).unwrap_or_else(|| "0.0.0".into());
        uninstall_app(&self.paths, app_id, &name)?;
        self.installs.apps.remove(app_id);
        self.installs.save(&self.paths)?;
        self.emit(Event::Installs(self.installs.clone()));
        self.queue_event(app_id, "uninstall", &version, None, None);
        let _ = self.flush_outbox();
        let _ = self.heartbeat(false);
        Ok(())
    }

    fn open_app(&mut self, app_id: &str) -> Result<()> {
        let Some(row) = self.installs.apps.get(app_id).cloned() else {
            return Err(Error::msg("install the app first"));
        };
        let dir = self.paths.app_dir(app_id)?;
        let entry = dir.join(&row.entry);
        match open_entry(&entry, &dir) {
            Ok(()) => {
                self.queue_event(app_id, "launch", &row.version, None, None);
                let _ = self.flush_outbox();
                Ok(())
            }
            Err(err) => {
                self.fail_event(app_id, &row.version, None, &err.to_string());
                Err(err)
            }
        }
    }

    fn self_update(&mut self) -> Result<()> {
        let Some(release) = self.catalog.as_ref().and_then(|c| c.launcher.clone()) else {
            return Err(Error::msg("no launcher update is listed"));
        };
        let dest = self.paths.pending_launcher();
        let cancel = Arc::clone(&self.cancel);
        self.api.download(crate::api::Download {
            url: &release.url,
            dest: &dest,
            expected_sha256: &release.sha256,
            expected_size: release.size,
            cancel: &cancel,
            on_progress: &mut |_, _| {},
        })?;
        let current = std::env::current_exe().map_err(|e| Error::msg(e.to_string()))?;
        crate::self_update::spawn_swap(&dest, &current)?;
        self.emit(Event::Exit);
        Ok(())
    }

    fn load_admin(&mut self) -> Result<()> {
        let access = self.vault.access(&self.api, &self.device.id)?;
        match self.api.admin_me(&access) {
            Ok(_) => {}
            Err(err) => {
                self.emit(Event::Banner(err.to_string()));
                return Ok(());
            }
        }
        let stats = self.api.admin_stats(&access)?;
        self.emit(Event::Admin(stats));
        let page = self.api.admin_users(&access, &crate::api::UserQuery { cursor: None, limit: 50, q: None })?;
        self.emit(Event::Users(page.items));
        Ok(())
    }

    fn heartbeat(&mut self, queue_on_failure: bool) -> Result<()> {
        let body = Heartbeat {
            device_id: self.device.id.clone(),
            device_name: self.device.name.clone(),
            launcher_version: LAUNCHER_VERSION.to_string(),
            os: self.device.os.clone(),
            installed: self.installs.reports(),
        };
        let access = match self.vault.access(&self.api, &self.device.id) {
            Ok(token) => token,
            Err(err) if err.is_network() && queue_on_failure => {
                self.outbox.enqueue_heartbeat(body);
                self.outbox.save(&self.paths)?;
                return Ok(());
            }
            Err(err) => return Err(err),
        };
        match self.api.heartbeat(&access, &body) {
            Ok(_) => Ok(()),
            Err(err) if (err.is_network() || err.is_rate_limited()) && queue_on_failure => {
                self.outbox.enqueue_heartbeat(body);
                self.outbox.save(&self.paths)?;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn queue_event(&mut self, app_id: &str, event: &str, version: &str, from: Option<&str>, error: Option<&str>) {
        self.outbox.enqueue_event(InstallEvent {
            device_id: self.device.id.clone(),
            app_id: app_id.to_string(),
            event: event.to_string(),
            version: version.to_string(),
            from_version: from.map(str::to_string),
            error: error.map(str::to_string),
            at: now_rfc3339(),
        });
        if let Err(err) = self.outbox.save(&self.paths) {
            log_line(&self.paths, &err.to_string());
        }
    }

    fn fail_event(&mut self, app_id: &str, version: &str, from: Option<&str>, error: &str) {
        self.queue_event(app_id, "fail", version, from, Some(error));
        let _ = self.flush_outbox();
    }

    fn flush_outbox(&mut self) -> Result<()> {
        let access = match self.vault.access(&self.api, &self.device.id) {
            Ok(token) => token,
            Err(err) if err.is_network() => return Ok(()),
            Err(err) => return Err(err),
        };
        match flush(&self.api, &access, &mut self.outbox)? {
            FlushStop::Done | FlushStop::Later => {}
            FlushStop::Auth => {
                if self.vault.rotate(&self.api, &self.device.id).is_ok()
                    && let Ok(access) = self.vault.access(&self.api, &self.device.id)
                {
                    let _ = flush(&self.api, &access, &mut self.outbox)?;
                }
            }
        }
        self.outbox.save(&self.paths)?;
        Ok(())
    }

    fn sleep_interruptible(&self, total: Duration) -> Result<()> {
        let start = Instant::now();
        while start.elapsed() < total {
            if self.cancel.load(Ordering::Relaxed) {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(200).min(total.saturating_sub(start.elapsed())));
        }
        Ok(())
    }

    fn emit(&self, event: Event) {
        let _ = self.events.send(event);
    }
}
