//! Windows backend for router storage (SMB shares).
//!
//! Windows ships a native SMB client, so instead of `smbclient` (Samba) this module authenticates with
//! `net use` and then accesses the share through regular UNC paths (`\\ip\share\folder`).

use super::{
    RouterStorageManager, StorageCredentials, StorageDiskInfo, StorageFileEntry, StorageShare,
    StorageUploadEvent,
};
use iced::futures::channel::mpsc::UnboundedSender;
use std::io::{Read, Write};
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};
use std::time::Instant;
use tokio::process::Command;

fn to_win_path(p: &str) -> String {
    p.trim_matches('/').replace('/', "\\")
}

fn io_err(context: &str, e: std::io::Error) -> String {
    format!("{}: {}", context, e)
}

/// Free/total space of the volume behind a UNC path (`GetDiskFreeSpaceExW`)
fn disk_space(path: &Path) -> Option<StorageDiskInfo> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;

    let mut s: Vec<u16> = path.as_os_str().encode_wide().collect();
    if s.last() != Some(&(b'\\' as u16)) {
        s.push(b'\\' as u16);
    }
    s.push(0);

    let (mut avail, mut total, mut free) = (0u64, 0u64, 0u64);
    let ok = unsafe { GetDiskFreeSpaceExW(s.as_ptr(), &mut avail, &mut total, &mut free) };
    if ok != 0 && total > 0 {
        Some(StorageDiskInfo {
            total_bytes: total,
            free_bytes: avail,
        })
    } else {
        None
    }
}

impl RouterStorageManager {
    /// Authenticates against `\\ip\share` (when credentials were supplied) and returns the UNC root path.
    pub(super) async fn win_connect(
        ip: Ipv4Addr,
        share: &str,
        creds: &StorageCredentials,
    ) -> Result<PathBuf, String> {
        let unc = format!(r"\\{}\{}", ip, share);
        if !creds.is_empty() {
            let user = if creds.username.trim().is_empty() {
                "guest".to_string()
            } else {
                creds.username.trim().to_string()
            };
            let user = if creds.workgroup.trim().is_empty() {
                user
            } else {
                format!("{}\\{}", creds.workgroup.trim(), user)
            };

            // Drop a stale session with other credentials (error 1219 otherwise)
            let _ = crate::procutil::NoWindow::no_window(Command::new("net"))
                .args(["use", &unc, "/delete", "/y"])
                .output()
                .await;

            let out = crate::procutil::NoWindow::no_window(Command::new("net"))
                .arg("use")
                .arg(&unc)
                .arg(creds.password.trim())
                .arg(format!("/user:{}", user))
                .arg("/persistent:no")
                .output()
                .await
                .map_err(|e| format!("Error executing net use: {}", e))?;

            if !out.status.success() {
                let msg = format!(
                    "{} {}",
                    String::from_utf8_lossy(&out.stdout).trim(),
                    String::from_utf8_lossy(&out.stderr).trim()
                );
                return Err(format!("Authentication failed for {}: {}", unc, msg.trim()));
            }
        }
        Ok(PathBuf::from(unc))
    }

    /// Lists disk shares with `net view \\ip /all` (output parsed by shape: header separator line + column groups)
    pub(super) async fn win_list_shares(
        ip: Ipv4Addr,
        creds: &StorageCredentials,
    ) -> Result<Vec<StorageShare>, String> {
        // Authenticate through IPC$ so `net view` uses the supplied credentials
        let _ = Self::win_connect(ip, "IPC$", creds).await;

        let out = crate::procutil::NoWindow::no_window(Command::new("net"))
            .arg("view")
            .arg(format!(r"\\{}", ip))
            .arg("/all")
            .output()
            .await
            .map_err(|e| format!("Error executing net view: {}", e))?;

        let text = String::from_utf8_lossy(&out.stdout);
        let mut shares = Vec::new();
        let mut in_table = false;
        for line in text.lines() {
            if line.trim_start().starts_with("-----") {
                in_table = true;
                continue;
            }
            if !in_table {
                continue;
            }
            if line.trim().is_empty() {
                break;
            }
            let cols: Vec<&str> = line.split("  ").map(|c| c.trim()).filter(|c| !c.is_empty()).collect();
            if cols.len() < 2 {
                continue;
            }
            let name = cols[0];
            let kind = cols[1].to_lowercase();
            let is_disk = kind.starts_with("disk")
                || kind.starts_with("disco")
                || kind.starts_with("disque")
                || kind.starts_with("platte")
                || kind.starts_with("datentr");
            if !is_disk || name.ends_with('$') {
                continue;
            }
            shares.push(StorageShare {
                name: name.to_string(),
                share_type: "Disk".to_string(),
                comment: if cols.len() >= 3 { cols[cols.len() - 1].to_string() } else { String::new() },
            });
        }

        if shares.is_empty() {
            if !out.status.success() {
                let err = String::from_utf8_lossy(&out.stderr);
                let out_s = text.trim();
                return Err(format!(
                    "Failed to query shares on {}: {}",
                    ip,
                    if err.trim().is_empty() { if out_s.is_empty() { "Connection refused or timeout" } else { out_s } } else { err.trim() }
                ));
            }
            return Err(format!("Router at {} does not have any visible disk or shared folder.", ip));
        }
        Ok(shares)
    }

    pub(super) async fn win_list_directory(
        ip: Ipv4Addr,
        share: &str,
        subpath: &str,
        creds: &StorageCredentials,
    ) -> Result<(Vec<StorageFileEntry>, Option<StorageDiskInfo>), String> {
        let root = Self::win_connect(ip, share, creds).await?;
        let dir = root.join(to_win_path(subpath));
        let share_owned = share.to_string();
        let subpath_owned = subpath.to_string();

        tokio::task::spawn_blocking(move || {
            let rd = std::fs::read_dir(&dir).map_err(|e| {
                format!(
                    "Error listing content of '{}/{}': {}",
                    share_owned,
                    subpath_owned.trim_matches('/'),
                    e
                )
            })?;

            let mut files = Vec::new();
            for entry in rd.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let Ok(meta) = entry.metadata() else { continue };
                let is_dir = meta.is_dir();
                let modified = meta
                    .modified()
                    .ok()
                    .map(|t| chrono::DateTime::<chrono::Local>::from(t).format("%d %b %Y %H:%M").to_string())
                    .unwrap_or_default();
                files.push(StorageFileEntry::new(
                    name,
                    is_dir,
                    if is_dir { 0 } else { meta.len() },
                    modified,
                    if is_dir { "D".to_string() } else { "A".to_string() },
                ));
            }
            files.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.name.to_lowercase().cmp(&b.name.to_lowercase())));

            Ok((files, disk_space(&dir)))
        })
        .await
        .map_err(|e| format!("Directory listing task failed: {}", e))?
    }

    pub(super) async fn win_create_folder(
        ip: Ipv4Addr,
        share: &str,
        parent_path: &str,
        folder_name: &str,
        creds: &StorageCredentials,
    ) -> Result<(), String> {
        let root = Self::win_connect(ip, share, creds).await?;
        let target = root.join(to_win_path(parent_path)).join(folder_name);
        tokio::task::spawn_blocking(move || {
            std::fs::create_dir(&target).map_err(|e| io_err("Could not create folder", e))
        })
        .await
        .map_err(|e| format!("Create folder task failed: {}", e))?
    }

    pub(super) async fn win_delete_entry(
        ip: Ipv4Addr,
        share: &str,
        relative_path: &str,
        is_dir: bool,
        creds: &StorageCredentials,
    ) -> Result<(), String> {
        let root = Self::win_connect(ip, share, creds).await?;
        let target = root.join(to_win_path(relative_path));
        let result = tokio::task::spawn_blocking(move || {
            let r = if is_dir {
                std::fs::remove_dir_all(&target)
            } else {
                std::fs::remove_file(&target)
            };
            match r {
                Ok(()) => Ok(()),
                // Already gone counts as success
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
                Err(e) => Err(io_err("Could not delete", e)),
            }
        })
        .await
        .map_err(|e| format!("Delete task failed: {}", e))?;
        if result.is_ok() {
            // Allow the router file system time to synchronize and update free space
            tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
        }
        result
    }

    pub(super) async fn win_download_file(
        ip: Ipv4Addr,
        share: &str,
        remote_path: &str,
        local_dest: &Path,
        creds: &StorageCredentials,
    ) -> Result<PathBuf, String> {
        let root = Self::win_connect(ip, share, creds).await?;
        let src = root.join(to_win_path(remote_path));
        let dest = local_dest.to_path_buf();
        tokio::task::spawn_blocking(move || {
            if let Some(parent) = dest.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            std::fs::copy(&src, &dest)
                .map(|_| dest.clone())
                .map_err(|e| io_err("Download failed", e))
        })
        .await
        .map_err(|e| format!("Download task failed: {}", e))?
    }

    pub(super) async fn win_upload_file_with_progress(
        ip: Ipv4Addr,
        share: &str,
        local_src: &Path,
        remote_dest_path: &str,
        creds: &StorageCredentials,
        progress_tx: Option<UnboundedSender<StorageUploadEvent>>,
    ) -> Result<(), String> {
        let clean_remote = to_win_path(remote_dest_path);
        let file_name = Path::new(&clean_remote)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(&clean_remote)
            .to_string();
        let parent_dir = Path::new(&clean_remote)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .replace('\\', "/");
        let file_size = std::fs::metadata(local_src).map(|m| m.len()).unwrap_or(0);

        if let Some(ref tx) = progress_tx {
            let _ = tx.unbounded_send(StorageUploadEvent::Started {
                is_folder: false,
                title: format!("Uploading file: {}", file_name),
                total_items: 1,
                total_bytes: file_size,
                target_dir: if parent_dir.is_empty() { "Root".to_string() } else { parent_dir.clone() },
            });
            let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                current_item: file_name.clone(),
                current_index: 1,
                total_items: 1,
                bytes_uploaded: 0,
                total_bytes: file_size,
                speed_str: "Starting transfer...".to_string(),
                percent: 0.0,
            });
        }

        let root = Self::win_connect(ip, share, creds).await?;
        let dest = root.join(&clean_remote);
        let src = local_src.to_path_buf();
        let tx_clone = progress_tx.clone();
        let name_clone = file_name.clone();

        let start_time = Instant::now();
        tokio::task::spawn_blocking(move || -> Result<(), String> {
            let mut input = std::fs::File::open(&src).map_err(|e| io_err("Upload failed (open source)", e))?;
            let mut output = std::fs::File::create(&dest).map_err(|e| io_err("Upload failed (create remote file)", e))?;
            let mut buf = vec![0u8; 1024 * 1024];
            let mut done: u64 = 0;
            let mut last_report = Instant::now();
            loop {
                let n = input.read(&mut buf).map_err(|e| io_err("Upload failed (read)", e))?;
                if n == 0 {
                    break;
                }
                output.write_all(&buf[..n]).map_err(|e| io_err("Upload failed (write)", e))?;
                done += n as u64;

                if let Some(ref tx) = tx_clone {
                    if last_report.elapsed().as_millis() >= 200 {
                        last_report = Instant::now();
                        let elapsed = start_time.elapsed().as_secs_f64().max(0.05);
                        let mb_s = (done as f64 / 1_048_576.0) / elapsed;
                        let percent = if file_size > 0 { (done as f32 / file_size as f32) * 100.0 } else { 50.0 };
                        let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                            current_item: name_clone.clone(),
                            current_index: 1,
                            total_items: 1,
                            bytes_uploaded: done,
                            total_bytes: file_size,
                            speed_str: format!("{:.1} MB/s", mb_s),
                            percent: percent.clamp(1.0, 99.0),
                        });
                    }
                }
            }
            output.flush().map_err(|e| io_err("Upload failed (flush)", e))?;
            Ok(())
        })
        .await
        .map_err(|e| format!("Upload task failed: {}", e))??;

        let elapsed = start_time.elapsed().as_secs_f64().max(0.05);
        let final_speed_mb_s = (file_size as f64 / 1_048_576.0) / elapsed;

        if let Some(ref tx) = progress_tx {
            let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                current_item: file_name.clone(),
                current_index: 1,
                total_items: 1,
                bytes_uploaded: file_size,
                total_bytes: file_size,
                speed_str: format!("{:.1} MB/s", final_speed_mb_s.max(1.0)),
                percent: 100.0,
            });
            let _ = tx.unbounded_send(StorageUploadEvent::Completed {
                message: format!(
                    "File '{}' ({}) successfully uploaded in {:.1}s",
                    file_name,
                    super::format_file_size(file_size),
                    elapsed
                ),
                total_items: 1,
                total_bytes: file_size,
            });
        }
        Ok(())
    }

    /// Native Windows open-file / folder picker (WinForms through PowerShell)
    pub(super) async fn win_pick_path(is_directory: bool, title: &str) -> Option<PathBuf> {
        const FILE_SCRIPT: &str = r#"
Add-Type -AssemblyName System.Windows.Forms
$d = New-Object System.Windows.Forms.OpenFileDialog
$d.Title = $env:NL_TITLE
$f = New-Object System.Windows.Forms.Form
$f.TopMost = $true
if ($d.ShowDialog($f) -eq [System.Windows.Forms.DialogResult]::OK) { [Console]::Out.Write($d.FileName) }
"#;
        const DIR_SCRIPT: &str = r#"
Add-Type -AssemblyName System.Windows.Forms
$d = New-Object System.Windows.Forms.FolderBrowserDialog
$d.Description = $env:NL_TITLE
$f = New-Object System.Windows.Forms.Form
$f.TopMost = $true
if ($d.ShowDialog($f) -eq [System.Windows.Forms.DialogResult]::OK) { [Console]::Out.Write($d.SelectedPath) }
"#;
        let script = if is_directory { DIR_SCRIPT } else { FILE_SCRIPT };
        let mut std_cmd = crate::procutil::powershell_command(script);
        std_cmd.env("NL_TITLE", title);
        let output = tokio::process::Command::from(std_cmd).output().await.ok()?;
        if !output.status.success() {
            return None;
        }
        let chosen = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if chosen.is_empty() {
            return None;
        }
        let p = PathBuf::from(chosen);
        p.exists().then_some(p)
    }
}
