//! Catalog types shared by the launcher and the release workflow.
//!
//! `catalog.json` is generated from GitHub Releases and served by `GET /catalog`.

mod assemble;

pub use assemble::{AssembleError, ReleaseAsset, ReleaseRecord, assemble, releases_from_github_json};

/// The suite apps, in the order the launcher shows them.
pub const APP_IDS: [&str; 16] = [
    "pixelsmith",
    "pathsmith",
    "rawsmith",
    "pagesmith",
    "motionsmith",
    "layoutsmith",
    "mocksmith",
    "wordsmith",
    "soundsmith",
    "slidesmith",
    "cadsmith",
    "cellsmith",
    "texsmith",
    "anysmith",
    "solidsmith",
    "framesmith",
];

pub const LAUNCHER_ID: &str = "launcher";
pub const DEFAULT_REPO: &str = "mahmudurrahman420/smith-suite";
pub const DEFAULT_MIN_LAUNCHER: &str = "1.0.0";

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Catalog {
    pub generated_at: String,
    #[serde(default)]
    pub launcher: Option<LauncherRelease>,
    pub apps: Vec<AppRelease>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LauncherRelease {
    pub version: String,
    pub url: String,
    pub sha256: String,
    pub size: u64,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub mandatory: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AppRelease {
    pub id: String,
    pub name: String,
    pub tagline: String,
    pub icon_url: String,
    pub version: String,
    pub released_at: String,
    #[serde(default)]
    pub notes: String,
    pub package: Package,
    #[serde(default = "default_min_launcher")]
    pub min_launcher: String,
}

fn default_min_launcher() -> String {
    DEFAULT_MIN_LAUNCHER.to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageFormat {
    Exe,
    Zip,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Package {
    pub format: PackageFormat,
    pub url: String,
    pub sha256: String,
    pub size: u64,
    pub entry: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct AppMeta {
    pub id: String,
    pub name: String,
    pub tagline: String,
    pub icon: String,
    pub color: String,
    pub monogram: String,
    #[serde(default = "default_min_launcher")]
    pub min_launcher: String,
}

#[derive(Debug, serde::Deserialize)]
struct MetaFile {
    app: Vec<AppMeta>,
}

pub fn app_meta() -> Result<Vec<AppMeta>, String> {
    let file: MetaFile = toml::from_str(include_str!("../../../catalog/apps.toml")).map_err(|e| e.to_string())?;
    if file.app.len() != APP_IDS.len() {
        return Err(format!("apps.toml has {} apps, expected {}", file.app.len(), APP_IDS.len()));
    }
    for (meta, id) in file.app.iter().zip(APP_IDS) {
        if meta.id != id {
            return Err(format!("apps.toml id {} is out of order (expected {id})", meta.id));
        }
        if parse_color(&meta.color).is_none() {
            return Err(format!("{} has a bad color {}", meta.id, meta.color));
        }
        if meta.monogram.is_empty() || meta.name.is_empty() {
            return Err(format!("{} is missing a name or monogram", meta.id));
        }
    }
    Ok(file.app)
}

pub fn parse_color(hex: &str) -> Option<[u8; 3]> {
    let h = hex.strip_prefix('#')?;
    if h.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(h.get(0..2)?, 16).ok()?;
    let g = u8::from_str_radix(h.get(2..4)?, 16).ok()?;
    let b = u8::from_str_radix(h.get(4..6)?, 16).ok()?;
    Some([r, g, b])
}

pub fn icon_url(repo: &str, icon_path: &str) -> String {
    format!("https://raw.githubusercontent.com/{repo}/main/{icon_path}")
}

pub fn is_safe_entry(entry: &str) -> bool {
    if entry.is_empty() || entry.len() > 240 {
        return false;
    }
    if entry.starts_with('/') || entry.starts_with('\\') || entry.contains(':') {
        return false;
    }
    let path = std::path::Path::new(entry);
    if path.is_absolute() {
        return false;
    }
    path.components().all(|c| matches!(c, std::path::Component::Normal(_)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meta_lists_the_suite_apps() {
        let apps = app_meta().expect("apps.toml");
        assert_eq!(apps.len(), 16);
        let mut grams = std::collections::BTreeSet::new();
        for app in &apps {
            assert!(grams.insert(app.monogram.clone()), "duplicate monogram {}", app.monogram);
            assert!(app.tagline.len() > 3);
        }
    }

    #[test]
    fn icon_files_exist_when_the_suite_tree_is_present() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        if !repo.join("pixelsmith").is_dir() {
            return;
        }
        let apps = app_meta().expect("apps.toml");
        for app in apps {
            let path = repo.join(&app.icon);
            assert!(path.is_file(), "missing icon for {}: {}", app.id, path.display());
        }
    }
}
