#![cfg_attr(not(windows), allow(dead_code))]
//! A small Shell Link writer. Windows Start-menu shortcuts are `.lnk` files.
//! The layout follows MS-SHLLINK: a header, a LinkInfo block with a Unicode local path,
//! then a description and working directory.

use std::path::Path;

use crate::error::Result;

pub fn write_shortcut(dir: &Path, name: &str, target: &Path, workdir: &Path) -> Result<std::path::PathBuf> {
    std::fs::create_dir_all(dir)?;
    let safe = sanitize_name(name);
    #[cfg(windows)]
    {
        let path = dir.join(format!("{safe}.lnk"));
        let bytes = shell_link(&target.display().to_string(), &workdir.display().to_string(), name);
        std::fs::write(&path, bytes)?;
        Ok(path)
    }
    #[cfg(not(windows))]
    {
        let path = dir.join(format!("{safe}.desktop"));
        let body = format!("[Desktop Entry]\nType=Application\nName={name}\nExec=\"{}\"\nPath={}\n", target.display(), workdir.display());
        std::fs::write(&path, body)?;
        Ok(path)
    }
}

pub fn sanitize_name(name: &str) -> String {
    let mut out = String::new();
    for ch in name.chars() {
        if matches!(ch, '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|') {
            continue;
        }
        out.push(ch);
        if out.len() >= 80 {
            break;
        }
    }
    let trimmed = out.trim();
    if trimmed.is_empty() { "App".to_string() } else { trimmed.to_string() }
}

pub fn shell_link(target: &str, workdir: &str, description: &str) -> Vec<u8> {
    let mut buf = Vec::with_capacity(512);
    push_u32(&mut buf, 0x0000_004C);
    buf.extend_from_slice(&[0x01, 0x14, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]);
    // HasLinkInfo | HasName | HasWorkingDir | IsUnicode
    push_u32(&mut buf, 0x0000_0002 | 0x0000_0004 | 0x0000_0010 | 0x0000_0080);
    push_u32(&mut buf, 0x0000_0020);
    for _ in 0..3 {
        push_u64(&mut buf, 0);
    }
    push_u32(&mut buf, 0);
    push_u32(&mut buf, 0);
    push_u32(&mut buf, 1);
    push_u16(&mut buf, 0);
    push_u16(&mut buf, 0);
    push_u32(&mut buf, 0);
    push_u32(&mut buf, 0);

    let link_info = link_info(target);
    buf.extend_from_slice(&link_info);
    push_counted(&mut buf, description);
    push_counted(&mut buf, workdir);
    buf
}

fn link_info(target: &str) -> Vec<u8> {
    let ansi = ascii_path(target);
    let wide = utf16_null(target);
    let volume = volume_id();
    let header = 0x24u32;
    let local_off = header + volume.len() as u32;
    let suffix_off = local_off + ansi.len() as u32;
    let wide_off = suffix_off + 1;
    let wide_suffix = wide_off + wide.len() as u32;
    let total = wide_suffix + 2;

    let mut buf = Vec::new();
    push_u32(&mut buf, total);
    push_u32(&mut buf, header);
    push_u32(&mut buf, 0x1);
    push_u32(&mut buf, header);
    push_u32(&mut buf, local_off);
    push_u32(&mut buf, 0);
    push_u32(&mut buf, suffix_off);
    push_u32(&mut buf, wide_off);
    push_u32(&mut buf, wide_suffix);
    buf.extend_from_slice(&volume);
    buf.extend_from_slice(&ansi);
    buf.push(0);
    buf.extend_from_slice(&wide);
    buf.extend_from_slice(&[0, 0]);
    buf
}

fn volume_id() -> Vec<u8> {
    let mut buf = Vec::new();
    push_u32(&mut buf, 17);
    push_u32(&mut buf, 3);
    push_u32(&mut buf, 0);
    push_u32(&mut buf, 16);
    buf.push(0);
    buf
}

fn ascii_path(target: &str) -> Vec<u8> {
    if target.is_ascii() { target.as_bytes().to_vec() } else { b"C:\\SmithSuite\\app.exe".to_vec() }
}

fn utf16_null(text: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    for unit in text.encode_utf16() {
        buf.extend_from_slice(&unit.to_le_bytes());
    }
    buf.extend_from_slice(&[0, 0]);
    buf
}

fn push_counted(buf: &mut Vec<u8>, text: &str) {
    let units: Vec<u16> = text.encode_utf16().collect();
    let count = u16::try_from(units.len()).unwrap_or(u16::MAX);
    push_u16(buf, count);
    for unit in units.into_iter().take(count as usize) {
        push_u16(buf, unit);
    }
}

fn push_u16(buf: &mut Vec<u8>, value: u16) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(buf: &mut Vec<u8>, value: u32) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn push_u64(buf: &mut Vec<u8>, value: u64) {
    buf.extend_from_slice(&value.to_le_bytes());
}

pub fn remove_shortcut(dir: &Path, name: &str) -> Result<()> {
    let safe = sanitize_name(name);
    for ext in ["lnk", "desktop"] {
        let path = dir.join(format!("{safe}.{ext}"));
        if path.exists() {
            std::fs::remove_file(path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortcut_contains_the_unicode_target() {
        let target = r"C:\Users\Ada\AppData\Local\SmithSuite\pixelsmith\pixelsmith.exe";
        let bytes = shell_link(target, r"C:\Users\Ada\AppData\Local\SmithSuite\pixelsmith", "Pixelsmith");
        assert_eq!(&bytes[0..4], &0x4Cu32.to_le_bytes());
        assert_eq!(&bytes[4..8], &[0x01, 0x14, 0x02, 0x00]);
        let needle = utf16_null(target);
        assert!(bytes.windows(needle.len()).any(|w| w == needle), "utf-16 path missing");
    }

    #[cfg(not(windows))]
    #[test]
    fn desktop_file_points_at_the_entry() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("pixelsmith.exe");
        std::fs::write(&target, b"hi").unwrap();
        let link = write_shortcut(dir.path(), "Pixelsmith", &target, dir.path()).unwrap();
        let text = std::fs::read_to_string(link).unwrap();
        assert!(text.contains("pixelsmith.exe"));
        remove_shortcut(dir.path(), "Pixelsmith").unwrap();
    }
}
