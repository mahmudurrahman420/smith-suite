use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::paths::Paths;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub os: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceFile {
    device_id: String,
}

impl Device {
    pub fn load_or_create(paths: &Paths) -> Result<Self> {
        let id = load_or_create_id(&paths.device_file())?;
        Ok(Self { id, name: device_name(), os: os_label() })
    }
}

fn load_or_create_id(path: &Path) -> Result<String> {
    if let Some(existing) = read_id(path)? {
        return Ok(existing);
    }
    let id = uuid::Uuid::new_v4().to_string();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let body = serde_json::to_vec_pretty(&DeviceFile { device_id: id.clone() }).map_err(|e| Error::msg(e.to_string()))?;
    std::fs::write(path, body)?;
    Ok(id)
}

fn read_id(path: &Path) -> Result<Option<String>> {
    if !path.exists() {
        return Ok(None);
    }
    let text = std::fs::read_to_string(path)?;
    let parsed: DeviceFile = match serde_json::from_str(&text) {
        Ok(value) => value,
        Err(_) => return Ok(None),
    };
    if uuid::Uuid::parse_str(&parsed.device_id).is_ok() { Ok(Some(parsed.device_id)) } else { Ok(None) }
}

fn device_name() -> String {
    for key in ["COMPUTERNAME", "HOSTNAME"] {
        if let Ok(value) = std::env::var(key) {
            let value = value.trim();
            if !value.is_empty() {
                return value.to_string();
            }
        }
    }
    if let Ok(text) = std::fs::read_to_string("/etc/hostname") {
        let text = text.trim();
        if !text.is_empty() {
            return text.to_string();
        }
    }
    "Windows-PC".to_string()
}

pub fn os_label() -> String {
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x64",
        "aarch64" => "arm64",
        other => other,
    };
    #[cfg(windows)]
    {
        let version = std::process::Command::new("cmd")
            .args(["/C", "ver"])
            .output()
            .ok()
            .and_then(|out| String::from_utf8(out.stdout).ok())
            .as_deref()
            .and_then(parse_windows_ver)
            .unwrap_or_else(|| "10.0.0".to_string());
        return format!("windows-{version}-{arch}");
    }
    #[cfg(not(windows))]
    {
        let release = std::fs::read_to_string("/proc/sys/kernel/osrelease").unwrap_or_default();
        let release = release.trim();
        let release = if release.is_empty() { "unknown" } else { release };
        format!("linux-{release}-{arch}")
    }
}

#[cfg_attr(not(windows), allow(dead_code))]
pub fn parse_windows_ver(text: &str) -> Option<String> {
    let idx = text.find("Version ")?;
    let rest = text.get(idx + "Version ".len()..)?;
    let token = rest.split_whitespace().next()?;
    let token = token.trim_end_matches(|c: char| !c.is_ascii_digit() && c != '.');
    let mut parts = token.split('.');
    let major = parts.next()?;
    if major.is_empty() || !major.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let minor = parts.next().unwrap_or("0");
    let build = parts.next().unwrap_or("0");
    if !minor.chars().all(|c| c.is_ascii_digit()) || !build.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some(format!("{major}.{minor}.{build}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_windows_ver_output() {
        let text = "\r\nMicrosoft Windows [Version 10.0.22631.2861]\r\n";
        assert_eq!(parse_windows_ver(text).as_deref(), Some("10.0.22631"));
    }

    #[test]
    fn device_id_is_stable() {
        let dir = tempfile::tempdir().unwrap();
        let paths = Paths::hermetic(dir.path().to_path_buf());
        let first = Device::load_or_create(&paths).unwrap();
        let second = Device::load_or_create(&paths).unwrap();
        assert_eq!(first.id, second.id);
        assert!(uuid::Uuid::parse_str(&first.id).is_ok());
    }
}
