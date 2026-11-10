//! Turn GitHub Release JSON into a [`Catalog`](crate::Catalog).

use std::collections::BTreeMap;

use semver::Version;

use crate::{APP_IDS, AppMeta, AppRelease, Catalog, LAUNCHER_ID, LauncherRelease, Package, PackageFormat, icon_url, is_safe_entry};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseAsset {
    pub name: String,
    pub url: String,
    pub size: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseRecord {
    pub tag: String,
    pub draft: bool,
    pub prerelease: bool,
    pub body: String,
    pub published_at: String,
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, thiserror::Error)]
pub enum AssembleError {
    #[error("release JSON is not an array")]
    NotAnArray,
    #[error("release {0} is missing {1}")]
    MissingField(usize, &'static str),
}

pub fn releases_from_github_json(value: &serde_json::Value) -> Result<Vec<ReleaseRecord>, AssembleError> {
    let rows = value.as_array().ok_or(AssembleError::NotAnArray)?;
    let mut out = Vec::with_capacity(rows.len());
    for (index, row) in rows.iter().enumerate() {
        out.push(record_from_value(index, row)?);
    }
    Ok(out)
}

fn record_from_value(index: usize, row: &serde_json::Value) -> Result<ReleaseRecord, AssembleError> {
    let tag = string_field(row, "tag_name").ok_or(AssembleError::MissingField(index, "tag_name"))?;
    let body = string_field(row, "body").unwrap_or_default();
    let published_at = string_field(row, "published_at").unwrap_or_default();
    let draft = row.get("draft").and_then(serde_json::Value::as_bool).unwrap_or(false);
    let prerelease = row.get("prerelease").and_then(serde_json::Value::as_bool).unwrap_or(false);
    let mut assets = Vec::new();
    if let Some(list) = row.get("assets").and_then(serde_json::Value::as_array) {
        for asset in list {
            let Some(name) = string_field(asset, "name") else { continue };
            let Some(url) = string_field(asset, "browser_download_url") else { continue };
            let size = asset.get("size").and_then(serde_json::Value::as_u64).unwrap_or(0);
            let sha256 = asset.get("digest").and_then(serde_json::Value::as_str).map(normalize_sha256).unwrap_or_default();
            assets.push(ReleaseAsset { name, url, size, sha256 });
        }
    }
    Ok(ReleaseRecord { tag, draft, prerelease, body, published_at, assets })
}

fn string_field(value: &serde_json::Value, key: &str) -> Option<String> {
    value.get(key).and_then(serde_json::Value::as_str).map(str::trim).filter(|s| !s.is_empty()).map(str::to_string)
}

pub fn normalize_sha256(digest: &str) -> String {
    let hex = digest.trim().strip_prefix("sha256:").unwrap_or(digest.trim());
    if hex.len() == 64 && hex.bytes().all(|b| b.is_ascii_hexdigit()) { hex.to_ascii_lowercase() } else { String::new() }
}

/// Latest stable release of each known app, plus the launcher exe when one exists.
pub fn assemble(records: &[ReleaseRecord], apps: &[AppMeta], generated_at: &str, repo: &str) -> Catalog {
    let metas: BTreeMap<&str, &AppMeta> = apps.iter().map(|a| (a.id.as_str(), a)).collect();
    let mut best_apps: BTreeMap<String, Candidate> = BTreeMap::new();
    let mut best_launcher: Option<Candidate> = None;

    for record in records {
        if record.draft || record.prerelease || record.published_at.is_empty() {
            continue;
        }
        let Some((id, version)) = parse_tag(&record.tag) else { continue };
        let Some(version_parsed) = Version::parse(&version).ok() else { continue };
        if id == LAUNCHER_ID {
            let Some(asset) = pick_asset(&record.assets, launcher_rank) else { continue };
            let next =
                Candidate { version: version_parsed, version_text: version, published_at: record.published_at.clone(), asset, body: record.body.clone() };
            consider(&mut best_launcher, next);
        } else if metas.contains_key(id.as_str()) {
            let Some(asset) = pick_asset(&record.assets, app_rank) else { continue };
            let next =
                Candidate { version: version_parsed, version_text: version, published_at: record.published_at.clone(), asset, body: record.body.clone() };
            let replace = match best_apps.get(&id) {
                Some(current) => newer(&next, current),
                None => true,
            };
            if replace {
                best_apps.insert(id, next);
            }
        }
    }

    let mut app_releases = Vec::new();
    for id in APP_IDS {
        let Some(candidate) = best_apps.get(id) else { continue };
        let Some(meta) = metas.get(id) else { continue };
        let Some(format) = format_of(&candidate.asset.name) else { continue };
        let directives = directives(&candidate.body);
        let entry = directives.entry.filter(|e| is_safe_entry(e)).unwrap_or_else(|| default_entry(id, format, &candidate.asset.name));
        if !is_safe_entry(&entry) {
            continue;
        }
        let min_launcher = directives.min_launcher.unwrap_or_else(|| meta.min_launcher.clone());
        app_releases.push(AppRelease {
            id: id.to_string(),
            name: meta.name.clone(),
            tagline: meta.tagline.clone(),
            icon_url: icon_url(repo, &meta.icon),
            version: candidate.version_text.clone(),
            released_at: candidate.published_at.clone(),
            notes: directives.notes,
            package: Package { format, url: candidate.asset.url.clone(), sha256: candidate.asset.sha256.clone(), size: candidate.asset.size, entry },
            min_launcher,
        });
    }

    let launcher = best_launcher.map(|candidate| {
        let directives = directives(&candidate.body);
        LauncherRelease {
            version: candidate.version_text,
            url: candidate.asset.url,
            sha256: candidate.asset.sha256,
            size: candidate.asset.size,
            notes: directives.notes,
            mandatory: directives.mandatory,
        }
    });

    Catalog { generated_at: generated_at.to_string(), launcher, apps: app_releases }
}

struct Candidate {
    version: Version,
    version_text: String,
    published_at: String,
    asset: ReleaseAsset,
    body: String,
}

fn consider(slot: &mut Option<Candidate>, next: Candidate) {
    match slot {
        None => *slot = Some(next),
        Some(current) if newer(&next, current) => *slot = Some(next),
        Some(_) => {}
    }
}

fn newer(next: &Candidate, current: &Candidate) -> bool {
    next.version > current.version || (next.version == current.version && next.published_at > current.published_at)
}

fn parse_tag(tag: &str) -> Option<(String, String)> {
    let (name, version) = tag.rsplit_once("-v")?;
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) {
        return None;
    }
    let mut parts = version.split('.');
    let major = parts.next()?;
    let minor = parts.next()?;
    let patch = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    if !is_plain_number(major) || !is_plain_number(minor) || !is_plain_number(patch) {
        return None;
    }
    Some((name.to_string(), version.to_string()))
}

fn is_plain_number(part: &str) -> bool {
    !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()) && (part.len() == 1 || !part.starts_with('0'))
}

fn format_of(name: &str) -> Option<PackageFormat> {
    let lower = name.to_ascii_lowercase();
    if lower.ends_with(".zip") {
        Some(PackageFormat::Zip)
    } else if lower.ends_with(".exe") {
        Some(PackageFormat::Exe)
    } else {
        None
    }
}

fn app_rank(name: &str) -> Option<u8> {
    let lower = name.to_ascii_lowercase();
    if lower.ends_with("-win-x64.zip") {
        Some(0)
    } else if lower.ends_with(".zip") && lower.contains("win") && lower.contains("x64") {
        Some(1)
    } else if lower.ends_with(".zip") {
        Some(2)
    } else if lower.ends_with("-win-x64.exe") {
        Some(3)
    } else if lower.ends_with(".exe") && lower.contains("win") && lower.contains("x64") {
        Some(4)
    } else if lower.ends_with(".exe") {
        Some(5)
    } else {
        None
    }
}

fn launcher_rank(name: &str) -> Option<u8> {
    if name.eq_ignore_ascii_case("SmithCreativeCloud.exe") {
        Some(0)
    } else {
        let lower = name.to_ascii_lowercase();
        if lower.contains("setup") || lower.contains("uninstall") {
            None
        } else if lower.ends_with(".exe") {
            Some(1)
        } else {
            None
        }
    }
}

fn pick_asset(assets: &[ReleaseAsset], rank: fn(&str) -> Option<u8>) -> Option<ReleaseAsset> {
    let mut best: Option<(u8, &ReleaseAsset)> = None;
    for asset in assets {
        if asset.sha256.len() != 64 || asset.url.is_empty() {
            continue;
        }
        let Some(score) = rank(&asset.name) else { continue };
        let replace = match &best {
            None => true,
            Some((current, _)) => score < *current,
        };
        if replace {
            best = Some((score, asset));
        }
    }
    best.map(|(_, asset)| asset.clone())
}

struct Directives {
    notes: String,
    entry: Option<String>,
    min_launcher: Option<String>,
    mandatory: bool,
}

fn directives(body: &str) -> Directives {
    let mut notes = Vec::new();
    let mut entry = None;
    let mut min_launcher = None;
    let mut mandatory = false;
    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("entry:") {
            let value = value.trim();
            if is_safe_entry(value) {
                entry = Some(value.to_string());
            }
        } else if let Some(value) = trimmed.strip_prefix("min_launcher:") {
            let value = value.trim();
            if Version::parse(value).is_ok() {
                min_launcher = Some(value.to_string());
            }
        } else if let Some(value) = trimmed.strip_prefix("mandatory:") {
            mandatory = matches!(value.trim().to_ascii_lowercase().as_str(), "true" | "yes" | "1");
        } else {
            notes.push(line);
        }
    }
    while notes.last().is_some_and(|l| l.trim().is_empty()) {
        notes.pop();
    }
    Directives { notes: notes.join("\n"), entry, min_launcher, mandatory }
}

fn default_entry(app_id: &str, format: PackageFormat, asset_name: &str) -> String {
    match format {
        PackageFormat::Exe => asset_name.to_string(),
        PackageFormat::Zip => format!("{app_id}.exe"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_meta;

    fn asset(name: &str, url: &str, digest: &str, size: u64) -> serde_json::Value {
        serde_json::json!({
            "name": name,
            "browser_download_url": url,
            "size": size,
            "digest": digest
        })
    }

    #[test]
    fn picks_latest_zip_and_launcher_exe() {
        let raw = serde_json::json!([
            {
                "tag_name": "pixelsmith-v1.0.0",
                "draft": false,
                "prerelease": false,
                "body": "First\nentry: pixelsmith.exe\n",
                "published_at": "2026-11-01T00:00:00Z",
                "assets": [asset(
                    "pixelsmith-1.0.0-win-x64.zip",
                    "https://example.invalid/old.zip",
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    10
                )]
            },
            {
                "tag_name": "pixelsmith-v1.2.0",
                "draft": false,
                "prerelease": false,
                "body": "Layers and masks\nmin_launcher: 1.0.0\n",
                "published_at": "2026-11-10T00:00:00Z",
                "assets": [
                    asset("pixelsmith.exe", "https://example.invalid/pixelsmith.exe", "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", 5),
                    asset("pixelsmith-1.2.0-win-x64.zip", "https://example.invalid/pixelsmith.zip", "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc", 9)
                ]
            },
            {
                "tag_name": "pixelsmith-v9.0.0",
                "draft": true,
                "prerelease": false,
                "body": "draft",
                "published_at": "2026-11-11T00:00:00Z",
                "assets": [asset("pixelsmith-9.0.0-win-x64.zip", "https://example.invalid/draft.zip", "sha256:dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd", 1)]
            },
            {
                "tag_name": "pixelsmith-v1.3.0-rc1",
                "draft": false,
                "prerelease": true,
                "body": "",
                "published_at": "2026-11-12T00:00:00Z",
                "assets": []
            },
            {
                "tag_name": "pathsmith-v0.3.0",
                "draft": false,
                "prerelease": false,
                "body": "Vectors\nentry: bin/pathsmith.exe\n",
                "published_at": "2026-11-02T00:00:00Z",
                "assets": [asset("Pathsmith.exe", "https://example.invalid/Pathsmith.exe", "sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", 8)]
            },
            {
                "tag_name": "launcher-v1.0.1",
                "draft": false,
                "prerelease": false,
                "body": "Fixes the updater\nmandatory: true\n",
                "published_at": "2026-11-10T08:00:00Z",
                "assets": [
                    asset("SmithCreativeCloud-Setup.exe", "https://example.invalid/setup.exe", "sha256:1111111111111111111111111111111111111111111111111111111111111111", 3),
                    asset("SmithCreativeCloud.exe", "https://example.invalid/SmithCreativeCloud.exe", "sha256:2222222222222222222222222222222222222222222222222222222222222222", 4)
                ]
            },
            {
                "tag_name": "not-a-release",
                "draft": false,
                "prerelease": false,
                "body": "",
                "published_at": "2026-11-01T00:00:00Z",
                "assets": []
            }
        ]);
        let records = releases_from_github_json(&raw).expect("parse");
        let catalog = assemble(&records, &app_meta().expect("meta"), "2026-11-10T08:00:00Z", "mahmudurrahman420/smith-suite");
        assert_eq!(catalog.apps.len(), 2);
        let pixels = &catalog.apps[0];
        assert_eq!(pixels.id, "pixelsmith");
        assert_eq!(pixels.version, "1.2.0");
        assert_eq!(pixels.package.format, PackageFormat::Zip);
        assert_eq!(pixels.package.entry, "pixelsmith.exe");
        assert_eq!(pixels.package.sha256, "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc");
        assert_eq!(pixels.notes, "Layers and masks");
        assert!(pixels.icon_url.contains("pixelsmith/assets/app-icon"));
        let path = &catalog.apps[1];
        assert_eq!(path.package.format, PackageFormat::Exe);
        assert_eq!(path.package.entry, "bin/pathsmith.exe");
        let launcher = catalog.launcher.expect("launcher");
        assert_eq!(launcher.version, "1.0.1");
        assert!(launcher.mandatory);
        assert_eq!(launcher.notes, "Fixes the updater");
        assert!(launcher.url.ends_with("SmithCreativeCloud.exe"));
        assert!(!launcher.url.contains("Setup"));
    }

    #[test]
    fn skips_assets_without_a_digest() {
        let raw = serde_json::json!([{
            "tag_name": "wordsmith-v1.0.0",
            "draft": false,
            "prerelease": false,
            "body": "",
            "published_at": "2026-11-01T00:00:00Z",
            "assets": [{"name": "wordsmith-1.0.0-win-x64.zip", "browser_download_url": "https://example.invalid/w.zip", "size": 4}]
        }]);
        let records = releases_from_github_json(&raw).expect("parse");
        let catalog = assemble(&records, &app_meta().expect("meta"), "2026-11-10T08:00:00Z", "mahmudurrahman420/smith-suite");
        assert!(catalog.apps.is_empty());
        assert!(catalog.launcher.is_none());
    }
}
