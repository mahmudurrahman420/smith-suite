//! Replace this executable after the process that is using it has exited.

use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use crate::error::{Error, Result};

pub fn spawn_swap(source: &Path, target: &Path) -> Result<()> {
    let mut cmd = std::process::Command::new(source);
    cmd.arg("--apply-self-update")
        .arg("--pid")
        .arg(std::process::id().to_string())
        .arg("--target")
        .arg(target)
        .arg("--source")
        .arg(source)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    cmd.spawn()?;
    Ok(())
}

pub fn apply_swap(pid: u32, source: &Path, target: &Path) -> Result<()> {
    wait_until_dead(pid, Duration::from_secs(60))?;
    replace_file(source, target)?;
    std::process::Command::new(target).stdin(std::process::Stdio::null()).spawn()?;
    Ok(())
}

pub fn replace_file(source: &Path, target: &Path) -> Result<()> {
    if source == target {
        return Ok(());
    }
    let old = {
        let mut name = target.as_os_str().to_os_string();
        name.push(".old");
        PathBuf::from(name)
    };
    let _ = std::fs::remove_file(&old);
    if target.exists() && std::fs::rename(target, &old).is_err() {
        for _ in 0..20 {
            if std::fs::copy(source, target).is_ok() {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(250));
        }
        return Err(Error::msg("could not replace the launcher; close other copies and try again"));
    }
    if let Err(err) = std::fs::copy(source, target) {
        if old.exists() && !target.exists() {
            let _ = std::fs::rename(&old, target);
        }
        return Err(err.into());
    }
    let _ = std::fs::remove_file(&old);
    Ok(())
}

fn wait_until_dead(pid: u32, timeout: Duration) -> Result<()> {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if !pid_alive(pid) {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(200));
    }
    Err(Error::msg("the previous launcher did not exit"))
}

fn pid_alive(pid: u32) -> bool {
    if pid == 0 {
        return false;
    }
    #[cfg(unix)]
    {
        Path::new(&format!("/proc/{pid}")).exists()
    }
    #[cfg(windows)]
    {
        let Ok(output) = std::process::Command::new("tasklist").args(["/FI", &format!("PID eq {pid}"), "/NH"]).output() else {
            return false;
        };
        let text = String::from_utf8_lossy(&output.stdout);
        text.split_whitespace().any(|part| part == pid.to_string())
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = pid;
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_file_swaps_contents() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("new.bin");
        let target = dir.path().join("old.bin");
        std::fs::write(&target, b"old").unwrap();
        std::fs::write(&source, b"new").unwrap();
        replace_file(&source, &target).unwrap();
        assert_eq!(std::fs::read(&target).unwrap(), b"new");
    }

    #[test]
    fn a_dead_pid_is_not_waited_on() {
        wait_until_dead(u32::MAX, Duration::from_secs(2)).unwrap();
    }
}
