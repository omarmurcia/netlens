//! Helpers for spawning child processes without flashing console windows on Windows.

/// Builder-style extension: `Command::new("x").no_window()`.
/// No-op on non-Windows platforms.
pub trait NoWindow: Sized {
    fn no_window(self) -> Self;
}

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

impl NoWindow for std::process::Command {
    #[cfg(windows)]
    fn no_window(mut self) -> Self {
        use std::os::windows::process::CommandExt;
        self.creation_flags(CREATE_NO_WINDOW);
        self
    }
    #[cfg(not(windows))]
    fn no_window(self) -> Self {
        self
    }
}

impl NoWindow for tokio::process::Command {
    #[cfg(windows)]
    fn no_window(mut self) -> Self {
        self.creation_flags(CREATE_NO_WINDOW);
        self
    }
    #[cfg(not(windows))]
    fn no_window(self) -> Self {
        self
    }
}

/// Platform-specific `ping` arguments for `count` echo requests with a 1 s timeout.
pub fn ping_args(count: u32, ip: &str) -> Vec<String> {
    #[cfg(windows)]
    {
        vec!["-n".into(), count.to_string(), "-w".into(), "1000".into(), ip.into()]
    }
    #[cfg(not(windows))]
    {
        vec!["-c".into(), count.to_string(), "-W".into(), "1".into(), ip.into()]
    }
}

/// Extracts the RTT in ms from one line of `ping` output.
/// Handles Linux (`time=12.3 ms`) and Windows (`time=12ms`, `time<1ms`, localized `tiempo=12ms`).
pub fn parse_ping_reply_ms(line: &str) -> Option<f32> {
    if let Some(pos) = line.find("time=") {
        let num: String = line[pos + 5..]
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        if let Ok(v) = num.parse::<f32>() {
            return Some(v);
        }
    }
    // Windows (possibly localized): a reply line carries a TTL
    if !line.to_ascii_uppercase().contains("TTL") {
        return None;
    }
    for token in line.split_whitespace() {
        let lower = token.to_lowercase();
        if lower.starts_with("ttl") || lower.starts_with("bytes") || lower.starts_with("octetos") {
            continue;
        }
        if let Some(idx) = lower.find(|c| c == '=' || c == '<') {
            let rest = &lower[idx + 1..];
            let num: String = rest.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
            let tail = &rest[num.len()..];
            if !num.is_empty() && tail.starts_with('m') {
                let v = num.parse::<f32>().ok()?;
                // "time<1ms" means sub-millisecond
                return Some(if lower[idx..].starts_with('<') { v.max(1.0) * 0.5 } else { v });
            }
        }
    }
    None
}

/// Command that opens a file/folder/URL with the default desktop handler (`xdg-open` / `explorer`).
pub fn opener() -> std::process::Command {
    #[cfg(windows)]
    let cmd = std::process::Command::new("explorer");
    #[cfg(not(windows))]
    let cmd = std::process::Command::new("xdg-open");
    NoWindow::no_window(cmd)
}

/// User home directory (`USERPROFILE` on Windows, `HOME` elsewhere).
pub fn home_dir() -> Option<std::path::PathBuf> {
    #[cfg(windows)]
    let var = "USERPROFILE";
    #[cfg(not(windows))]
    let var = "HOME";
    std::env::var_os(var)
        .filter(|v| !v.is_empty())
        .map(std::path::PathBuf::from)
}

/// Runs a PowerShell script (Windows only helper). Dynamic values must be passed via `envs` (read as `$env:NAME`) to avoid quoting issues.
#[cfg(windows)]
pub fn powershell_command(script: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("powershell");
    cmd.args(["-NoProfile", "-NonInteractive", "-STA", "-ExecutionPolicy", "Bypass", "-Command", script]);
    NoWindow::no_window(cmd)
}

/// Shows a native Windows toast notification through PowerShell.
#[cfg(windows)]
pub fn windows_toast(title: &str, body: &str) -> std::io::Result<std::process::Child> {
    const SCRIPT: &str = r#"
[Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] | Out-Null
[Windows.Data.Xml.Dom.XmlDocument, Windows.Data.Xml.Dom.XmlDocument, ContentType = WindowsRuntime] | Out-Null
$x = [Windows.UI.Notifications.ToastNotificationManager]::GetTemplateContent([Windows.UI.Notifications.ToastTemplateType]::ToastText02)
$n = $x.GetElementsByTagName('text')
$n.Item(0).AppendChild($x.CreateTextNode($env:NL_TITLE)) | Out-Null
$n.Item(1).AppendChild($x.CreateTextNode($env:NL_BODY)) | Out-Null
$toast = [Windows.UI.Notifications.ToastNotification]::new($x)
[Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier('{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\WindowsPowerShell\v1.0\powershell.exe').Show($toast)
"#;
    powershell_command(SCRIPT)
        .env("NL_TITLE", title)
        .env("NL_BODY", body)
        .spawn()
}

/// Shows the standard Windows "Save as" dialog. Returns `Ok(None)` if the user cancelled.
#[cfg(windows)]
pub fn windows_save_dialog(default_path: &std::path::Path, filter_name: &str, filter_ext: &str, title: &str) -> Option<Option<std::path::PathBuf>> {
    const SCRIPT: &str = r#"
Add-Type -AssemblyName System.Windows.Forms
$d = New-Object System.Windows.Forms.SaveFileDialog
$d.Title = $env:NL_TITLE
$d.InitialDirectory = Split-Path -Parent $env:NL_DEFAULT
$d.FileName = Split-Path -Leaf $env:NL_DEFAULT
$d.Filter = "$($env:NL_FILTER) (*.$($env:NL_EXT))|*.$($env:NL_EXT)"
$d.OverwritePrompt = $true
$f = New-Object System.Windows.Forms.Form
$f.TopMost = $true
if ($d.ShowDialog($f) -eq [System.Windows.Forms.DialogResult]::OK) { [Console]::Out.Write($d.FileName) }
"#;
    let out = powershell_command(SCRIPT)
        .env("NL_TITLE", title)
        .env("NL_DEFAULT", default_path)
        .env("NL_FILTER", filter_name)
        .env("NL_EXT", filter_ext)
        .output()
        .ok()?;
    if !out.status.success() {
        return None; // dialog unavailable: caller falls back to the default path
    }
    let chosen = String::from_utf8_lossy(&out.stdout).trim().to_string();
    Some(if chosen.is_empty() { None } else { Some(std::path::PathBuf::from(chosen)) })
}

/// Sends one ICMP echo with a 1 s timeout and reports whether a real reply came back.
/// (Windows `ping` exits with 0 even for "Destination host unreachable", so the output is checked for a TTL.)
pub async fn ping_alive(ip: std::net::Ipv4Addr) -> bool {
    let out = NoWindow::no_window(tokio::process::Command::new("ping"))
        .args(ping_args(1, &ip.to_string()))
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output()
        .await;
    match out {
        Ok(o) => {
            if cfg!(windows) {
                o.status.success() && String::from_utf8_lossy(&o.stdout).to_ascii_uppercase().contains("TTL")
            } else {
                o.status.success()
            }
        }
        Err(_) => false,
    }
}
