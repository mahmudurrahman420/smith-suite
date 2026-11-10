use std::path::{Component, Path, PathBuf};

use crate::endpoint::Environment;
use crate::error::{Error, Result};

const RESERVED: &[&str] = &["_launcher", "shortcuts", "downloads", "pending"];

#[derive(Debug, Clone)]
pub struct Paths {
    root: PathBuf,
    hermetic: bool,
}

impl Paths {
    pub fn resolve(home: Option<&Path>) -> Result<Self> {
        if let Some(home) = home {
            return Ok(Self::hermetic(home.to_path_buf()));
        }
        if let Ok(value) = std::env::var("SMITH_CC_HOME")
            && !value.is_empty()
        {
            return Ok(Self::hermetic(PathBuf::from(value)));
        }
        let base = dirs::data_local_dir().ok_or_else(|| Error::msg("this PC has no local app data folder"))?;
        Ok(Self { root: base.join("SmithSuite"), hermetic: false })
    }

    pub fn hermetic(root: PathBuf) -> Self {
        Self { root, hermetic: true }
    }

    pub fn state_dir(&self) -> PathBuf {
        self.root.join("_launcher")
    }

    pub fn app_dir(&self, app_id: &str) -> Result<PathBuf> {
        validate_app_id(app_id)?;
        Ok(self.root.join(app_id))
    }

    pub fn shortcuts(&self) -> PathBuf {
        if !self.hermetic
            && let Some(menu) = start_menu_dir()
        {
            return menu;
        }
        self.root.join("shortcuts")
    }

    pub fn token_file(&self, environment: Environment) -> PathBuf {
        self.state_dir().join(format!("token-{}.json", environment.as_str()))
    }

    pub fn user_file(&self, environment: Environment) -> PathBuf {
        self.state_dir().join(format!("user-{}.json", environment.as_str()))
    }

    pub fn downloads(&self) -> PathBuf {
        self.state_dir().join("downloads")
    }

    pub fn pending_launcher(&self) -> PathBuf {
        self.state_dir().join("pending").join("SmithCreativeCloud.exe")
    }

    pub fn catalog_cache(&self) -> PathBuf {
        self.state_dir().join("catalog.json")
    }

    pub fn installs_file(&self) -> PathBuf {
        self.state_dir().join("installs.json")
    }

    pub fn outbox_file(&self) -> PathBuf {
        self.state_dir().join("outbox.json")
    }

    pub fn settings_file(&self) -> PathBuf {
        self.state_dir().join("settings.json")
    }

    pub fn device_file(&self) -> PathBuf {
        self.state_dir().join("device.json")
    }

    pub fn notified_file(&self) -> PathBuf {
        self.state_dir().join("notified.json")
    }

    pub fn log_file(&self) -> PathBuf {
        self.state_dir().join("launcher.log")
    }

    pub fn ui_dir(&self) -> PathBuf {
        self.state_dir().join("ui")
    }
}

pub fn validate_app_id(id: &str) -> Result<()> {
    let ok = !id.is_empty()
        && id.len() <= 64
        && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
        && id.chars().next().is_some_and(|c| c.is_ascii_lowercase())
        && !RESERVED.contains(&id);
    if ok { Ok(()) } else { Err(Error::msg("that app id is not a Smith Suite app")) }
}

pub fn safe_relative(entry: &str) -> Result<PathBuf> {
    if !smithcc_catalog::is_safe_entry(entry) {
        return Err(Error::msg("the package entry path is not safe to run"));
    }
    let path = Path::new(entry);
    if path.components().any(|c| !matches!(c, Component::Normal(_))) {
        return Err(Error::msg("the package entry path is not safe to run"));
    }
    Ok(path.to_path_buf())
}

fn start_menu_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        dirs::data_dir().map(|dir| dir.join("Microsoft").join("Windows").join("Start Menu").join("Programs").join("Smith Suite"))
    }
    #[cfg(not(windows))]
    {
        None
    }
}

pub fn log_line(paths: &Paths, line: &str) {
    let path = paths.log_file();
    let _ = std::fs::create_dir_all(paths.state_dir());
    if let Ok(meta) = std::fs::metadata(&path)
        && meta.len() > 1_000_000
    {
        let _ = std::fs::rename(&path, paths.state_dir().join("launcher.log.1"));
    }
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        use std::io::Write;
        let _ = writeln!(file, "{} {line}", crate::api::now_rfc3339());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_path_escape_ids() {
        assert!(validate_app_id("pixelsmith").is_ok());
        assert!(validate_app_id("../windows").is_err());
        assert!(validate_app_id("_launcher").is_err());
        assert!(validate_app_id("Pixelsmith").is_err());
    }

    #[test]
    fn entry_must_stay_inside_the_install_dir() {
        assert!(safe_relative("pixelsmith.exe").is_ok());
        assert!(safe_relative("bin/pixelsmith.exe").is_ok());
        assert!(safe_relative("../pixelsmith.exe").is_err());
        assert!(safe_relative("/tmp/pixelsmith.exe").is_err());
        assert!(safe_relative("C:pixelsmith.exe").is_err());
    }
}
