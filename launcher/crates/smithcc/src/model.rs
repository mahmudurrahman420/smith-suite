//! What the window shows. The worker fills it in; the widgets only draw it.

use std::collections::BTreeMap;

use smithcc_catalog::{AppMeta, Catalog, LauncherRelease, app_meta};

use crate::api::{AdminStats, AdminUser, Health, LAUNCHER_VERSION, User};
use crate::endpoint::Environment;
use crate::state::{InstallDb, Notified};
use crate::version::{is_newer, launcher_too_old};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Boot,
    SignedOut,
    Ready,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Apps,
    Updates,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Primary {
    Install,
    Update,
    Open,
    None,
}

#[derive(Debug, Clone)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub tagline: String,
    pub monogram: String,
    pub color: [u8; 3],
    pub installed: Option<String>,
    pub latest: Option<String>,
    pub notes: String,
    pub primary: Primary,
    pub can_open: bool,
    pub can_uninstall: bool,
    pub update_available: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub environment: Environment,
    pub phase: Phase,
    pub user: Option<User>,
    pub health: Option<Health>,
    pub catalog: Option<Catalog>,
    pub catalog_message: Option<String>,
    pub installs: InstallDb,
    pub busy: BTreeMap<String, f32>,
    pub banner: Option<String>,
    pub page: Page,
    pub search: String,
    pub notes_for: Option<String>,
    pub polling: bool,
    pub status_line: String,
    pub self_update: Option<LauncherRelease>,
    pub admin: Option<AdminStats>,
    pub users: Vec<AdminUser>,
    pub metas: Vec<AppMeta>,
    notified: Notified,
    pub fresh_notices: Vec<String>,
}

impl Model {
    pub fn new(environment: Environment, installs: InstallDb, notified: Notified) -> Self {
        let metas = app_meta().unwrap_or_default();
        Self {
            environment,
            phase: Phase::Boot,
            user: None,
            health: None,
            catalog: None,
            catalog_message: None,
            installs,
            busy: BTreeMap::new(),
            banner: None,
            page: Page::Apps,
            search: String::new(),
            notes_for: None,
            polling: false,
            status_line: "Checking your sign-in…".into(),
            self_update: None,
            admin: None,
            users: Vec::new(),
            metas,
            notified,
            fresh_notices: Vec::new(),
        }
    }

    pub fn show_dev_sign_in(&self) -> bool {
        self.environment == Environment::Staging && self.health.as_ref().is_none_or(|h| !h.oauth_configured)
    }

    pub fn cards(&self) -> Vec<Card> {
        let blocked = self.self_update.as_ref().is_some_and(|rel| rel.mandatory);
        self.metas.iter().map(|meta| self.card_for(meta, blocked)).collect()
    }

    pub fn visible_cards(&self) -> Vec<Card> {
        let query = self.search.trim().to_ascii_lowercase();
        self.cards()
            .into_iter()
            .filter(|card| {
                let page_ok = self.page != Page::Updates || card.update_available;
                let query_ok = query.is_empty() || card.name.to_ascii_lowercase().contains(&query) || card.tagline.to_ascii_lowercase().contains(&query);
                page_ok && query_ok
            })
            .collect()
    }

    pub fn update_count(&self) -> usize {
        self.cards().iter().filter(|card| card.update_available).count()
    }

    pub fn take_notices(&mut self) -> Vec<String> {
        std::mem::take(&mut self.fresh_notices)
    }

    pub fn record_notices(&mut self) {
        let mut app_updates = Vec::new();
        for card in self.cards() {
            let Some(latest) = card.latest.clone() else { continue };
            if !card.update_available {
                continue;
            }
            if self.notified.versions.get(&card.id) == Some(&latest) {
                continue;
            }
            app_updates.push((card.id, card.name, latest));
        }
        let mut messages = Vec::new();
        if app_updates.len() == 1 {
            let (_, name, latest) = &app_updates[0];
            messages.push(format!("{name} {latest} is ready to update."));
        } else if app_updates.len() > 1 {
            messages.push(format!("{} Smith apps have updates.", app_updates.len()));
        }
        for (id, _, latest) in app_updates {
            self.notified.versions.insert(id, latest);
        }
        if let Some(release) = &self.self_update
            && self.notified.versions.get("__launcher") != Some(&release.version)
        {
            messages.push(format!("Smith Creative Cloud {} is ready. Restart to update.", release.version));
            self.notified.versions.insert("__launcher".into(), release.version.clone());
        }
        self.fresh_notices = messages;
    }

    pub fn notified(&self) -> &Notified {
        &self.notified
    }

    fn card_for(&self, meta: &AppMeta, launcher_blocked: bool) -> Card {
        let color = smithcc_catalog::parse_color(&meta.color).unwrap_or([76, 141, 255]);
        let installed = self.installs.apps.get(&meta.id).map(|app| app.version.clone());
        let release = self.catalog.as_ref().and_then(|catalog| catalog.apps.iter().find(|app| app.id == meta.id));
        let latest = release.map(|app| app.version.clone());
        let notes = release.map(|app| app.notes.clone()).unwrap_or_default();
        let update_available = match (&installed, &latest) {
            (Some(have), Some(latest)) => is_newer(latest, have),
            _ => false,
        };
        let too_old = release.is_some_and(|app| launcher_too_old(LAUNCHER_VERSION, &app.min_launcher));
        let (primary, reason) = if self.busy.contains_key(&meta.id) {
            (Primary::None, None)
        } else if launcher_blocked {
            (Primary::None, Some("Update Smith Creative Cloud to continue.".into()))
        } else if self.catalog_message.is_some() && release.is_none() {
            (Primary::None, Some("Catalog not available yet".into()))
        } else if release.is_none() {
            (Primary::None, Some("Not published yet".into()))
        } else if too_old {
            (Primary::None, Some("Update Smith Creative Cloud to install this app.".into()))
        } else if installed.is_none() {
            (Primary::Install, None)
        } else if update_available {
            (Primary::Update, None)
        } else {
            (Primary::Open, None)
        };
        Card {
            id: meta.id.clone(),
            name: meta.name.clone(),
            tagline: meta.tagline.clone(),
            monogram: meta.monogram.clone(),
            color,
            can_open: installed.is_some() && !self.busy.contains_key(&meta.id),
            can_uninstall: installed.is_some() && !self.busy.contains_key(&meta.id),
            installed,
            latest,
            notes,
            primary,
            update_available,
            reason,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smithcc_catalog::{Package, PackageFormat};

    fn sample_catalog(version: &str, min_launcher: &str) -> Catalog {
        let meta = app_meta().unwrap().into_iter().next().unwrap();
        Catalog {
            generated_at: "2026-11-10T00:00:00Z".into(),
            launcher: None,
            apps: vec![smithcc_catalog::AppRelease {
                id: meta.id,
                name: meta.name,
                tagline: meta.tagline,
                icon_url: String::new(),
                version: version.into(),
                released_at: "2026-11-10T00:00:00Z".into(),
                notes: "Notes".into(),
                package: Package {
                    format: PackageFormat::Zip,
                    url: "https://example.invalid/a.zip".into(),
                    sha256: "ab".repeat(32),
                    size: 1,
                    entry: "pixelsmith.exe".into(),
                },
                min_launcher: min_launcher.into(),
            }],
        }
    }

    #[test]
    fn buttons_follow_install_state() {
        let mut model = Model::new(Environment::Prod, InstallDb::default(), Notified::default());
        model.phase = Phase::Ready;
        model.catalog = Some(sample_catalog("1.2.0", "1.0.0"));
        let card = model.cards().into_iter().find(|c| c.id == "pixelsmith").unwrap();
        assert_eq!(card.primary, Primary::Install);
        model
            .installs
            .apps
            .insert("pixelsmith".into(), crate::state::InstalledApp { version: "1.0.0".into(), entry: "pixelsmith.exe".into(), installed_at: String::new() });
        let card = model.cards().into_iter().find(|c| c.id == "pixelsmith").unwrap();
        assert_eq!(card.primary, Primary::Update);
        assert!(card.can_open);
        model.installs.apps.get_mut("pixelsmith").unwrap().version = "1.2.0".into();
        let card = model.cards().into_iter().find(|c| c.id == "pixelsmith").unwrap();
        assert_eq!(card.primary, Primary::Open);
        assert!(!card.update_available);
    }

    #[test]
    fn missing_catalog_lists_every_app_without_install() {
        let model = Model::new(Environment::Prod, InstallDb::default(), Notified::default());
        assert_eq!(model.cards().len(), 15);
        assert!(model.cards().iter().all(|c| c.primary == Primary::None));
    }

    #[test]
    fn min_launcher_blocks_install() {
        let mut model = Model::new(Environment::Prod, InstallDb::default(), Notified::default());
        model.catalog = Some(sample_catalog("1.2.0", "99.0.0"));
        let card = model.cards().into_iter().find(|c| c.id == "pixelsmith").unwrap();
        assert_eq!(card.primary, Primary::None);
        assert!(card.reason.unwrap().contains("Update Smith Creative Cloud"));
    }
}
