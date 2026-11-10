//! Download, verify, and install a zip or a single exe into the per-user app folder.

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use smithcc_catalog::PackageFormat;

use crate::error::{Error, Result};
use crate::lnk::{remove_shortcut, write_shortcut};
use crate::paths::{Paths, safe_relative};

const MAX_FILES: usize = 4_096;
const MAX_UNPACKED: u64 = 4 * 1024 * 1024 * 1024;

pub fn install_package(paths: &Paths, app_id: &str, display_name: &str, format: PackageFormat, package: &Path, entry: &str) -> Result<PathBuf> {
    let entry_rel = safe_relative(entry)?;
    let dest = paths.app_dir(app_id)?;
    let staging = sibling(&dest, "staging");
    let backup = sibling(&dest, "old");
    if staging.exists() {
        fs::remove_dir_all(&staging)?;
    }
    fs::create_dir_all(&staging)?;
    let install_result = match format {
        PackageFormat::Zip => unpack_zip(package, &staging),
        PackageFormat::Exe => copy_exe(package, &staging, &entry_rel),
    };
    if let Err(err) = install_result {
        let _ = fs::remove_dir_all(&staging);
        return Err(err);
    }
    let entry_path = staging.join(&entry_rel);
    if !entry_path.is_file() {
        let _ = fs::remove_dir_all(&staging);
        return Err(Error::msg(format!("the package does not contain {entry}")));
    }
    make_executable(&entry_path)?;
    if backup.exists() {
        fs::remove_dir_all(&backup)?;
    }
    if dest.exists() {
        fs::rename(&dest, &backup).map_err(|_| Error::msg(format!("close {display_name} and try the update again")))?;
    }
    if let Err(err) = fs::rename(&staging, &dest) {
        if backup.exists() && !dest.exists() {
            let _ = fs::rename(&backup, &dest);
        }
        return Err(Error::Io(err));
    }
    let _ = fs::remove_dir_all(&backup);
    let final_entry = dest.join(&entry_rel);
    write_shortcut(&paths.shortcuts(), display_name, &final_entry, &dest)?;
    Ok(final_entry)
}

pub fn uninstall_app(paths: &Paths, app_id: &str, display_name: &str) -> Result<()> {
    let dest = paths.app_dir(app_id)?;
    if dest.exists() {
        fs::remove_dir_all(&dest)?;
    }
    remove_shortcut(&paths.shortcuts(), display_name)?;
    Ok(())
}

pub fn open_entry(entry: &Path, workdir: &Path) -> Result<()> {
    if !entry.is_file() {
        return Err(Error::msg("the installed app is missing; install it again"));
    }
    let mut cmd = std::process::Command::new(entry);
    cmd.current_dir(workdir).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // DETACHED_PROCESS so the app keeps running after the launcher exits.
        cmd.creation_flags(0x0000_0008);
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    let child = cmd.spawn()?;
    std::thread::spawn(move || {
        let mut child = child;
        let _ = child.wait();
    });
    Ok(())
}

pub fn recover_interrupted(paths: &Paths, app_id: &str) -> Result<()> {
    let dest = paths.app_dir(app_id)?;
    let staging = sibling(&dest, "staging");
    let backup = sibling(&dest, "old");
    if staging.exists() {
        let _ = fs::remove_dir_all(&staging);
    }
    if !dest.exists() && backup.exists() {
        fs::rename(&backup, &dest)?;
    }
    Ok(())
}

fn sibling(dest: &Path, suffix: &str) -> PathBuf {
    let name = dest.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| "app".into());
    dest.parent().unwrap_or(dest).join(format!("{name}.{suffix}"))
}

fn copy_exe(package: &Path, staging: &Path, entry: &Path) -> Result<()> {
    let dest = staging.join(entry);
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(package, &dest)?;
    Ok(())
}

fn unpack_zip(package: &Path, staging: &Path) -> Result<()> {
    let file = File::open(package)?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| Error::msg(format!("the zip package could not be opened ({e})")))?;
    if archive.is_empty() {
        return Err(Error::msg("the package is empty"));
    }
    if archive.len() > MAX_FILES {
        return Err(Error::msg("the package has too many files"));
    }
    let mut planned = Vec::new();
    let mut total = 0u64;
    for index in 0..archive.len() {
        let entry = archive.by_index(index).map_err(|e| Error::msg(format!("the zip package could not be read ({e})")))?;
        if let Some(mode) = entry.unix_mode()
            && mode & 0o170000 == 0o120000
        {
            return Err(Error::msg("the package contains a symlink, which is not installed"));
        }
        if entry.is_dir() {
            continue;
        }
        let Some(name) = entry.enclosed_name() else {
            return Err(Error::msg("the package contains a path that escapes the install folder"));
        };
        if !name.components().all(|c| matches!(c, std::path::Component::Normal(_))) {
            return Err(Error::msg("the package contains a path that escapes the install folder"));
        }
        total = total.saturating_add(entry.size());
        if total > MAX_UNPACKED {
            return Err(Error::msg("the package unpacks to more than 4 GB"));
        }
        planned.push(index);
    }
    if planned.is_empty() {
        return Err(Error::msg("the package is empty"));
    }
    for index in planned {
        let mut entry = archive.by_index(index).map_err(|e| Error::msg(format!("the zip package could not be read ({e})")))?;
        let Some(name) = entry.enclosed_name().map(|p| p.to_path_buf()) else {
            return Err(Error::msg("the package contains a path that escapes the install folder"));
        };
        let dest = staging.join(&name);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut out = File::create(&dest)?;
        let mut buf = [0u8; 64 * 1024];
        loop {
            let n = entry.read(&mut buf)?;
            if n == 0 {
                break;
            }
            out.write_all(&buf[..n])?;
        }
    }
    Ok(())
}

fn make_executable(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(perms.mode() | 0o755);
        fs::set_permissions(path, perms)?;
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
    Ok(())
}

pub fn zip_bytes(files: &[(&str, &[u8])]) -> Result<Vec<u8>> {
    let mut cursor = std::io::Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(&mut cursor);
    let opts = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (name, bytes) in files {
        writer.start_file(*name, opts).map_err(|e| Error::msg(e.to_string()))?;
        writer.write_all(bytes)?;
    }
    writer.finish().map_err(|e| Error::msg(e.to_string()))?;
    Ok(cursor.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::Digest;

    fn sha(bytes: &[u8]) -> String {
        let mut hasher = sha2::Sha256::new();
        hasher.update(bytes);
        crate::api::hex_encode(&hasher.finalize())
    }

    #[test]
    fn zip_install_update_and_uninstall() {
        let dir = tempfile::tempdir().unwrap();
        let paths = Paths::hermetic(dir.path().to_path_buf());
        let v1 = zip_bytes(&[("pixelsmith.exe", b"version-one")]).unwrap();
        let pkg = dir.path().join("v1.zip");
        fs::write(&pkg, &v1).unwrap();
        let _ = sha(&v1);
        let entry = install_package(&paths, "pixelsmith", "Pixelsmith", PackageFormat::Zip, &pkg, "pixelsmith.exe").unwrap();
        assert_eq!(fs::read(&entry).unwrap(), b"version-one");
        let link = paths.shortcuts().join("Pixelsmith.desktop");
        if cfg!(not(windows)) {
            assert!(link.exists() || paths.shortcuts().join("Pixelsmith.lnk").exists());
        }

        let v2 = zip_bytes(&[("pixelsmith.exe", b"version-two")]).unwrap();
        let pkg2 = dir.path().join("v2.zip");
        fs::write(&pkg2, &v2).unwrap();
        install_package(&paths, "pixelsmith", "Pixelsmith", PackageFormat::Zip, &pkg2, "pixelsmith.exe").unwrap();
        assert_eq!(fs::read(paths.app_dir("pixelsmith").unwrap().join("pixelsmith.exe")).unwrap(), b"version-two");

        uninstall_app(&paths, "pixelsmith", "Pixelsmith").unwrap();
        assert!(!paths.app_dir("pixelsmith").unwrap().exists());
    }

    #[test]
    fn zip_slip_is_rejected_and_leaves_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let paths = Paths::hermetic(dir.path().to_path_buf());
        let mut cursor = std::io::Cursor::new(Vec::new());
        let mut writer = zip::ZipWriter::new(&mut cursor);
        let opts = zip::write::SimpleFileOptions::default();
        writer.start_file("../evil.txt", opts).unwrap();
        writer.write_all(b"nope").unwrap();
        writer.finish().unwrap();
        let pkg = dir.path().join("bad.zip");
        fs::write(&pkg, cursor.into_inner()).unwrap();
        let err = install_package(&paths, "pixelsmith", "Pixelsmith", PackageFormat::Zip, &pkg, "pixelsmith.exe").unwrap_err();
        assert!(err.to_string().contains("escapes") || err.to_string().contains("does not contain"));
        assert!(!paths.app_dir("pixelsmith").unwrap().exists());
    }

    #[test]
    fn exe_package_is_copied_to_the_entry_name() {
        let dir = tempfile::tempdir().unwrap();
        let paths = Paths::hermetic(dir.path().to_path_buf());
        let pkg = dir.path().join("app.bin");
        fs::write(&pkg, b"#!/bin/sh\necho hi\n").unwrap();
        let entry = install_package(&paths, "pathsmith", "Pathsmith", PackageFormat::Exe, &pkg, "pathsmith.exe").unwrap();
        assert_eq!(fs::read(&entry).unwrap(), b"#!/bin/sh\necho hi\n");
    }
}
