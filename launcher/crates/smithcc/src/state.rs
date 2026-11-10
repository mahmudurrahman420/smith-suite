use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::api::{InstalledReport, User};
use crate::endpoint::Environment;
use crate::error::{Error, Result};
use crate::paths::Paths;
use smithcc_catalog::Catalog;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct InstallDb {
    #[serde(default)]
    pub apps: BTreeMap<String, InstalledApp>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstalledApp {
    pub version: String,
    pub entry: String,
    pub installed_at: String,
}

impl InstallDb {
    pub fn load(paths: &Paths) -> Result<Self> {
        read_json(&paths.installs_file()).map(|v| v.unwrap_or_default())
    }

    pub fn save(&self, paths: &Paths) -> Result<()> {
        write_json(&paths.installs_file(), self)
    }

    pub fn reports(&self) -> Vec<InstalledReport> {
        self.apps.iter().map(|(id, app)| InstalledReport { app_id: id.clone(), version: app.version.clone() }).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Settings {
    pub environment: Environment,
}

impl Default for Settings {
    fn default() -> Self {
        Self { environment: Environment::Prod }
    }
}

impl Settings {
    pub fn load(paths: &Paths) -> Result<Self> {
        Ok(read_json(&paths.settings_file())?.unwrap_or_default())
    }

    pub fn save(&self, paths: &Paths) -> Result<()> {
        write_json(&paths.settings_file(), self)
    }
}

pub fn load_catalog(paths: &Paths) -> Result<Option<Catalog>> {
    read_json(&paths.catalog_cache())
}

pub fn save_catalog(paths: &Paths, catalog: &Catalog) -> Result<()> {
    write_json(&paths.catalog_cache(), catalog)
}

pub fn load_user(paths: &Paths, environment: Environment) -> Result<Option<User>> {
    read_json(&paths.user_file(environment))
}

pub fn save_user(paths: &Paths, environment: Environment, user: &User) -> Result<()> {
    write_json(&paths.user_file(environment), user)
}

pub fn clear_user(paths: &Paths, environment: Environment) -> Result<()> {
    let path = paths.user_file(environment);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Notified {
    #[serde(default)]
    pub versions: BTreeMap<String, String>,
}

impl Notified {
    pub fn load(paths: &Paths) -> Result<Self> {
        Ok(read_json(&paths.notified_file())?.unwrap_or_default())
    }

    pub fn save(&self, paths: &Paths) -> Result<()> {
        write_json(&paths.notified_file(), self)
    }
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<Option<T>> {
    if !path.exists() {
        return Ok(None);
    }
    let text = fs::read_to_string(path)?;
    serde_json::from_str(&text).map(Some).map_err(|e| Error::msg(format!("could not read {}: {e}", path.display())))
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let body = serde_json::to_vec_pretty(value).map_err(|e| Error::msg(e.to_string()))?;
    let tmp = {
        let mut name = path.as_os_str().to_os_string();
        name.push(".tmp");
        std::path::PathBuf::from(name)
    };
    fs::write(&tmp, body)?;
    fs::rename(&tmp, path).or_else(|_| {
        fs::copy(&tmp, path)?;
        let _ = fs::remove_file(&tmp);
        Ok::<(), std::io::Error>(())
    })?;
    Ok(())
}
