//! Best-effort Windows balloon notification. The in-app badge is the reliable signal;
//! a failure here is logged and ignored.

pub fn notify(title: &str, body: &str) {
    #[cfg(windows)]
    {
        let script = balloon_script(title, body);
        let _ = std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-WindowStyle", "Hidden", "-Command", &script])
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
    }
    #[cfg(not(windows))]
    {
        let _ = (title, body);
    }
}

#[cfg_attr(not(windows), allow(dead_code))]
pub fn balloon_script(title: &str, body: &str) -> String {
    let title = escape(&clip(title, 120));
    let body = escape(&clip(body, 240));
    format!(
        "Add-Type -AssemblyName System.Windows.Forms; Add-Type -AssemblyName System.Drawing; \
         $n = New-Object System.Windows.Forms.NotifyIcon; \
         $n.Icon = [System.Drawing.SystemIcons]::Information; $n.Visible = $true; \
         $n.ShowBalloonTip(8000, '{title}', '{body}', 'Info'); Start-Sleep -Seconds 8; $n.Dispose()"
    )
}

#[cfg_attr(not(windows), allow(dead_code))]
fn clip(text: &str, max: usize) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        if out.chars().count() >= max {
            break;
        }
        if ch == '\n' || ch == '\r' {
            out.push(' ');
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg_attr(not(windows), allow(dead_code))]
fn escape(text: &str) -> String {
    text.replace('\'', "''")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotes_in_a_release_name_cannot_break_the_script() {
        let script = balloon_script("It's ready", "Pixelsmith'); Remove-Item");
        assert!(script.contains("It''s ready"));
        assert!(script.contains("Pixelsmith''); Remove-Item"));
        assert!(!script.contains("Pixelsmith'); Remove-Item"));
    }
}
