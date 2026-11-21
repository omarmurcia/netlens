mod transfer;
#[cfg(windows)]
mod windows;

use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::Command;

/// Information about a local file to transfer
#[derive(Debug, Clone)]
pub struct LocalFileInfo {
    pub abs_path: PathBuf,
    pub rel_path: String,
    pub size: u64,
}

/// Current upload progress state for files or folders
#[derive(Debug, Clone)]
pub struct StorageUploadProgress {
    pub is_folder: bool,
    pub title: String,
    pub target_dir: String,
    pub current_item: String,
    pub current_index: usize,
    pub total_items: usize,
    pub bytes_uploaded: u64,
    pub total_bytes: u64,
    pub speed_str: String,
    pub percent: f32, // 0.0 to 100.0
    pub is_finished: bool,
    pub error: Option<String>,
}

/// Events emitted during file or folder upload
#[derive(Debug, Clone)]
pub enum StorageUploadEvent {
    Started {
        is_folder: bool,
        title: String,
        total_items: usize,
        total_bytes: u64,
        target_dir: String,
    },
    Progress {
        current_item: String,
        current_index: usize,
        total_items: usize,
        bytes_uploaded: u64,
        total_bytes: u64,
        speed_str: String,
        percent: f32,
    },
    Completed {
        message: String,
        #[allow(dead_code)]
        total_items: usize,
        total_bytes: u64,
    },
    Error(String),
}

/// Helper to read bytes read by a process from /proc/<pid>/io on Linux
pub(super) fn get_process_read_bytes(pid: u32) -> Option<u64> {
    let path = format!("/proc/{}/io", pid);
    let content = std::fs::read_to_string(path).ok()?;
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("rchar:") {
            if let Ok(bytes) = rest.trim().parse::<u64>() {
                return Some(bytes);
            }
        }
    }
    None
}

/// Recognized file types in router storage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageFileType {
    Folder,
    Image,
    Video,
    Audio,
    Document,
    Archive,
    Executable,
    Code,
    Unknown,
}

impl StorageFileType {
    pub fn from_filename(name: &str, is_dir: bool) -> Self {
        if is_dir {
            return Self::Folder;
        }
        let ext = Path::new(name)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        match ext.as_str() {
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "svg" | "tiff" | "ico" => Self::Image,
            "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" => Self::Video,
            "mp3" | "flac" | "wav" | "aac" | "ogg" | "m4a" | "wma" => Self::Audio,
            "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "md" | "odt" | "csv" => {
                Self::Document
            }
            "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "iso" => Self::Archive,
            "sh" | "bin" | "run" | "appimage" | "exe" | "msi" | "apk" => Self::Executable,
            "rs" | "py" | "js" | "ts" | "html" | "css" | "json" | "xml" | "toml" | "yaml" | "yml" | "c"
            | "cpp" | "h" => Self::Code,
            _ => Self::Unknown,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Folder => "📁",
            Self::Image => "🖼️",
            Self::Video => "🎬",
            Self::Audio => "🎵",
            Self::Document => "📄",
            Self::Archive => "📦",
            Self::Executable => "⚙️",
            Self::Code => "💻",
            Self::Unknown => "📎",
        }
    }

    #[allow(dead_code)]
    pub fn type_name_es(&self) -> &'static str {
        self.type_name(crate::i18n::Language::Spanish)
    }

    pub fn type_name(&self, lang: crate::i18n::Language) -> &'static str {
        crate::i18n::Translations::storage_file_type_name(*self, lang)
    }
}

/// File or directory entry in router storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageFileEntry {
    pub name: String,
    pub is_dir: bool,
    pub size_bytes: u64,
    pub modified_str: String,
    pub file_type: StorageFileType,
    pub attributes: String,
}

impl StorageFileEntry {
    pub fn new(name: String, is_dir: bool, size_bytes: u64, modified_str: String, attributes: String) -> Self {
        let file_type = StorageFileType::from_filename(&name, is_dir);
        Self {
            name,
            is_dir,
            size_bytes,
            modified_str,
            file_type,
            attributes,
        }
    }

    pub fn formatted_size(&self) -> String {
        if self.is_dir {
            "—".to_string()
        } else {
            format_file_size(self.size_bytes)
        }
    }
}

/// Formats sizes into human-readable representation
pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Shared resource (Share) exposed by router (e.g. usbdisk, public)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageShare {
    pub name: String,
    pub share_type: String, // "Disk", "IPC", "Printer"
    pub comment: String,
}

/// Disk capacity and space info connected to router
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct StorageDiskInfo {
    pub total_bytes: u64,
    pub free_bytes: u64,
}

impl StorageDiskInfo {
    pub fn used_bytes(&self) -> u64 {
        self.total_bytes.saturating_sub(self.free_bytes)
    }

    pub fn used_percent(&self) -> f32 {
        if self.total_bytes == 0 {
            0.0
        } else {
            (self.used_bytes() as f32 / self.total_bytes as f32) * 100.0
        }
    }

    #[allow(dead_code)]
    pub fn free_percent(&self) -> f32 {
        100.0 - self.used_percent()
    }
}

/// Optional credentials to access router shared resource
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct StorageCredentials {
    pub username: String,
    pub password: String,
    pub workgroup: String,
}

impl StorageCredentials {
    pub fn is_empty(&self) -> bool {
        self.username.trim().is_empty() && self.password.trim().is_empty()
    }
}

/// Operations manager and client for router USB / NAS storage
pub struct RouterStorageManager;

impl RouterStorageManager {
    /// Quickly checks if router has active storage service (SMB/CIFS) on port 445 or 139
    #[allow(dead_code)]
    pub async fn probe_storage_service(ip: Ipv4Addr) -> bool {
        let probe_445 = tokio::time::timeout(
            std::time::Duration::from_millis(450),
            tokio::net::TcpStream::connect((ip, 445)),
        )
        .await
        .map(|res| res.is_ok())
        .unwrap_or(false);

        if probe_445 {
            return true;
        }

        tokio::time::timeout(
            std::time::Duration::from_millis(450),
            tokio::net::TcpStream::connect((ip, 139)),
        )
        .await
        .map(|res| res.is_ok())
        .unwrap_or(false)
    }

    /// Constructs authentication arguments for smbclient
    pub(super) fn build_auth_args(creds: &StorageCredentials) -> Vec<String> {
        let mut args = Vec::new();
        if creds.is_empty() {
            args.push("-N".to_string());
        } else {
            let user = if creds.username.trim().is_empty() {
                "guest"
            } else {
                creds.username.trim()
            };
            let pass = creds.password.trim();
            args.push("-U".to_string());
            args.push(format!("{}%{}", user, pass));
        }

        if !creds.workgroup.trim().is_empty() {
            args.push("-W".to_string());
            args.push(creds.workgroup.trim().to_string());
        }

        args
    }

    /// Parses output from `smbclient -L <ip> -g` or `-L <ip>` to extract shared disk names
    pub fn parse_shares_output(output: &str) -> Vec<StorageShare> {
        let mut shares = Vec::new();

        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // smbclient grepable format: Disk|share_name|comment
            if line.contains('|') {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 2 {
                    let stype = parts[0].trim();
                    let name = parts[1].trim();
                    let comment = parts.get(2).copied().unwrap_or("").trim();

                    if !name.is_empty() && !name.ends_with('$') {
                        shares.push(StorageShare {
                            name: name.to_string(),
                            share_type: stype.to_string(),
                            comment: comment.to_string(),
                        });
                    }
                }
                continue;
            }

            // Standard tabulated format of smbclient:
            // Sharename Type Comment
            // usbdisk Disk USB Storage
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].trim();
                let stype = parts[1].trim();
                if (stype.eq_ignore_ascii_case("Disk") || stype.eq_ignore_ascii_case("Disco"))
                    && !name.is_empty()
                    && !name.ends_with('$')
                    && !name.eq_ignore_ascii_case("Sharename")
                    && !name.starts_with("---")
                {
                    let comment = if parts.len() > 2 {
                        parts[2..].join(" ")
                    } else {
                        String::new()
                    };
                    shares.push(StorageShare {
                        name: name.to_string(),
                        share_type: "Disk".to_string(),
                        comment,
                    });
                }
            }
        }

        // If no share detected but output references a disk, provide default fallback
        if shares.is_empty() && output.to_lowercase().contains("disk") {
            shares.push(StorageShare {
                name: "usbdisk".to_string(),
                share_type: "Disk".to_string(),
                comment: "Router USB Storage".to_string(),
            });
        }

        shares
    }

    /// Parses output from `smbclient ... -c "ls"`, returning list of files/folders and disk space
    pub fn parse_directory_output(output: &str) -> (Vec<StorageFileEntry>, Option<StorageDiskInfo>) {
        let mut files = Vec::new();
        let mut disk_info = None;

        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Detection of disk block summary line:
            // e.g. "48828125 blocks of size 4096. 31250000 blocks available"
            if trimmed.contains("blocks of size") && trimmed.contains("available") {
                if let Some(info) = Self::parse_disk_blocks_line(trimmed) {
                    disk_info = Some(info);
                }
                continue;
            }

            // Skip relative directories "." and ".."
            if trimmed.starts_with(". ") || trimmed.starts_with(".. ") || trimmed == "." || trimmed == ".." {
                continue;
            }

            // Parse file / folder line:
            // smbclient format: "  Filename   ATTR   SIZE   DATE"
            // e.g.: "Photos          D        0  Sun Oct 12 14:30:10 2025"
            // e.g.: "backup.zip      A 104857600  Mon Nov 16 18:22:45 2026"
            if let Some(entry) = Self::parse_single_file_line(line) {
                files.push(entry);
            }
        }

        // Sort: folders first alphabetically, then files alphabetically
        files.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });

        (files, disk_info)
    }

    /// Parses disk block summary line
    fn parse_disk_blocks_line(line: &str) -> Option<StorageDiskInfo> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // [total_blocks, "blocks", "of", "size", block_size_with_dot, free_blocks, "blocks", "available"]
        if parts.len() >= 8 {
            let total_blocks = parts[0].parse::<u64>().ok()?;
            let block_size_str = parts[4].trim_end_matches('.');
            let block_size = block_size_str.parse::<u64>().ok()?;
            let free_blocks = parts[5].parse::<u64>().ok()?;

            let total_bytes = total_blocks.saturating_mul(block_size);
            let free_bytes = free_blocks.saturating_mul(block_size);

            return Some(StorageDiskInfo {
                total_bytes,
                free_bytes,
            });
        }
        None
    }

    /// Parses an individual file line returned by `smbclient`
    fn parse_single_file_line(line: &str) -> Option<StorageFileEntry> {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        // At least requires: name, attributes, size and date
        if tokens.len() < 4 {
            return None;
        }

        // In smbclient, the last tokens correspond to date:
        // e.g. "Sun Oct 12 14:30:10 2025" (5 tokens) or "2026-11-17 12:00:00" (2 tokens)
        // Search backwards for first number representing size (size_bytes)
        let mut size_idx = None;
        for i in (1..tokens.len()).rev() {
            if tokens[i].parse::<u64>().is_ok() {
                // Verify previous token is an attribute string (D, A, R, H, S, etc.)
                if i > 0 && tokens[i - 1].chars().all(|c| "DARHSN".contains(c)) {
                    size_idx = Some(i);
                    break;
                }
            }
        }

        let size_pos = size_idx?;
        let attr_pos = size_pos - 1;

        let name = tokens[..attr_pos].join(" ");
        if name.is_empty() || name == "." || name == ".." {
            return None;
        }

        let attributes = tokens[attr_pos].to_string();
        let is_dir = attributes.contains('D');
        let size_bytes = tokens[size_pos].parse::<u64>().unwrap_or(0);
        let modified_str = tokens[size_pos + 1..].join(" ");

        Some(StorageFileEntry::new(
            name,
            is_dir,
            size_bytes,
            modified_str,
            attributes,
        ))
    }

    /// Queries available shares on the router
    pub async fn list_shares(
        ip: Ipv4Addr,
        creds: &StorageCredentials,
    ) -> Result<Vec<StorageShare>, String> {
        #[cfg(windows)]
        {
            return Self::win_list_shares(ip, creds).await;
        }

        let mut cmd = crate::procutil::NoWindow::no_window(Command::new("smbclient"));
        cmd.arg("-L")
            .arg(ip.to_string())
            .arg("-g")
            .arg("-t")
            .arg("5");

        for arg in Self::build_auth_args(creds) {
            cmd.arg(arg);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Error executing smbclient: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() && stdout.trim().is_empty() {
            return Err(format!(
                "Failed to query shares on {}: {}",
                ip,
                if stderr.trim().is_empty() { "Connection refused or timeout" } else { stderr.trim() }
            ));
        }

        let shares = Self::parse_shares_output(&stdout);
        if shares.is_empty() {
            // Try without -g for compatibility with legacy samba servers
            let mut fallback_cmd = crate::procutil::NoWindow::no_window(Command::new("smbclient"));
            fallback_cmd
                .arg("-L")
                .arg(ip.to_string())
                .arg("-t")
                .arg("5");
            for arg in Self::build_auth_args(creds) {
                fallback_cmd.arg(arg);
            }
            if let Ok(fallback_out) = fallback_cmd.output().await {
                let fb_stdout = String::from_utf8_lossy(&fallback_out.stdout);
                let fb_shares = Self::parse_shares_output(&fb_stdout);
                if !fb_shares.is_empty() {
                    return Ok(fb_shares);
                }
            }
            return Err(format!("Router at {} does not have any visible disk or shared folder.", ip));
        }

        Ok(shares)
    }

    /// Lists files and subdirectories of a path within a shared resource
    pub async fn list_directory(
        ip: Ipv4Addr,
        share: &str,
        subpath: &str,
        creds: &StorageCredentials,
    ) -> Result<(Vec<StorageFileEntry>, Option<StorageDiskInfo>), String> {
        #[cfg(windows)]
        {
            return Self::win_list_directory(ip, share, subpath, creds).await;
        }

        let clean_subpath = subpath.trim_matches('/');
        let service = format!("//{}/{}", ip, share);

        let smb_command = if clean_subpath.is_empty() {
            "ls".to_string()
        } else {
            // Convert / to \ for smbclient
            let win_path = clean_subpath.replace('/', "\\");
            format!("cd \"{}\"; ls", win_path)
        };

        let mut cmd = crate::procutil::NoWindow::no_window(Command::new("smbclient"));
        cmd.arg(&service)
            .arg("-c")
            .arg(&smb_command)
            .arg("-t")
            .arg("6");

        for arg in Self::build_auth_args(creds) {
            cmd.arg(arg);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Error querying directory via smbclient: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() && stdout.trim().is_empty() {
            return Err(format!(
                "Error listing content of '{}/{}': {}",
                share,
                clean_subpath,
                if stderr.trim().is_empty() { "Access error" } else { stderr.trim() }
            ));
        }

        let (files, disk_info) = Self::parse_directory_output(&stdout);
        Ok((files, disk_info))
    }

    /// Creates a new directory in router storage
    pub async fn create_folder(
        ip: Ipv4Addr,
        share: &str,
        parent_path: &str,
        folder_name: &str,
        creds: &StorageCredentials,
    ) -> Result<(), String> {
        #[cfg(windows)]
        {
            return Self::win_create_folder(ip, share, parent_path, folder_name, creds).await;
        }

        let clean_parent = parent_path.trim_matches('/');
        let target_path = if clean_parent.is_empty() {
            folder_name.to_string()
        } else {
            format!("{}/{}", clean_parent, folder_name)
        };
        let win_path = target_path.replace('/', "\\");

        let service = format!("//{}/{}", ip, share);
        let smb_command = format!("mkdir \"{}\"", win_path);

        let mut cmd = crate::procutil::NoWindow::no_window(Command::new("smbclient"));
        cmd.arg(&service)
            .arg("-c")
            .arg(&smb_command)
            .arg("-t")
            .arg("6");

        for arg in Self::build_auth_args(creds) {
            cmd.arg(arg);
        }

        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Error creating folder: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Could not create folder: {}", stderr.trim()));
        }

        Ok(())
    }

    /// Deletes a file or directory in router storage
    pub async fn delete_entry(
        ip: Ipv4Addr,
        share: &str,
        relative_path: &str,
        is_dir: bool,
        creds: &StorageCredentials,
    ) -> Result<(), String> {
        #[cfg(windows)]
        {
            return Self::win_delete_entry(ip, share, relative_path, is_dir, creds).await;
        }

        let clean_path = relative_path.trim_matches('/');
        let parent_dir = Path::new(clean_path)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .trim_matches('/');
        let entry_name = Path::new(clean_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(clean_path);

        let service = format!("//{}/{}", ip, share);
        let win_parent = parent_dir.replace('/', "\\");

        // For directories: first attempt deltree (native recursive deletion in Samba 4.7+)
        // For files: del <file>.
        let smb_command = if is_dir {
            if win_parent.is_empty() {
                format!("deltree \"{}\"", entry_name)
            } else {
                format!("cd \"{}\"; deltree \"{}\"", win_parent, entry_name)
            }
        } else {
            if win_parent.is_empty() {
                format!("del \"{}\"", entry_name)
            } else {
                format!("cd \"{}\"; del \"{}\"", win_parent, entry_name)
            }
        };

        let mut cmd = crate::procutil::NoWindow::no_window(Command::new("smbclient"));
        cmd.arg(&service)
            .arg("-c")
            .arg(&smb_command)
            .arg("-t")
            .arg("25");

        for arg in Self::build_auth_args(creds) {
            cmd.arg(arg);
        }

        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Error deleting entry: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // If no longer exists (NT_STATUS_OBJECT_NAME_NOT_FOUND or NO_SUCH_FILE), deletion was successful
        let already_gone = stdout.contains("NT_STATUS_OBJECT_NAME_NOT_FOUND")
            || stderr.contains("NT_STATUS_OBJECT_NAME_NOT_FOUND")
            || stdout.contains("NT_STATUS_NO_SUCH_FILE")
            || stderr.contains("NT_STATUS_NO_SUCH_FILE");

        if output.status.success() || already_gone {
            // Allow router file system time to synchronize and update free space
            tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
            return Ok(());
        }

        // If directory and deltree failed (e.g. legacy Samba server without deltree),
        // try with recursive fallback mdel and rmdir
        if is_dir {
            let fallback_clean = if win_parent.is_empty() {
                format!("cd \"{}\"; recurse ON; prompt OFF; mdel *; cd ..; rmdir \"{}\"", entry_name, entry_name)
            } else {
                format!("cd \"{}\\{}\"; recurse ON; prompt OFF; mdel *; cd ..; rmdir \"{}\"", win_parent, entry_name, entry_name)
            };

            let mut fcmd = crate::procutil::NoWindow::no_window(Command::new("smbclient"));
            fcmd.arg(&service)
                .arg("-c")
                .arg(&fallback_clean)
                .arg("-t")
                .arg("25");

            for arg in Self::build_auth_args(creds) {
                fcmd.arg(arg);
            }

            if let Ok(fout) = fcmd.output().await {
                let fstdout = String::from_utf8_lossy(&fout.stdout);
                let fstderr = String::from_utf8_lossy(&fout.stderr);
                if fout.status.success()
                    || fstdout.contains("NT_STATUS_OBJECT_NAME_NOT_FOUND")
                    || fstderr.contains("NT_STATUS_OBJECT_NAME_NOT_FOUND")
                    || fstdout.contains("NT_STATUS_NO_SUCH_FILE")
                    || fstderr.contains("NT_STATUS_NO_SUCH_FILE")
                {
                    tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
                    return Ok(());
                }
            }

            // Simple fallback to rmdir if directory was already empty
            let fallback_cmd = if win_parent.is_empty() {
                format!("rmdir \"{}\"", entry_name)
            } else {
                format!("cd \"{}\"; rmdir \"{}\"", win_parent, entry_name)
            };

            let mut rcmd = crate::procutil::NoWindow::no_window(Command::new("smbclient"));
            rcmd.arg(&service)
                .arg("-c")
                .arg(&fallback_cmd)
                .arg("-t")
                .arg("10");

            for arg in Self::build_auth_args(creds) {
                rcmd.arg(arg);
            }

            if let Ok(rout) = rcmd.output().await {
                let rstdout = String::from_utf8_lossy(&rout.stdout);
                let rstderr = String::from_utf8_lossy(&rout.stderr);
                if rout.status.success()
                    || rstdout.contains("NT_STATUS_OBJECT_NAME_NOT_FOUND")
                    || rstderr.contains("NT_STATUS_OBJECT_NAME_NOT_FOUND")
                    || rstdout.contains("NT_STATUS_NO_SUCH_FILE")
                    || rstderr.contains("NT_STATUS_NO_SUCH_FILE")
                {
                    tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
                    return Ok(());
                }
            }
        }

        let err_msg = if !stderr.trim().is_empty() {
            stderr.trim()
        } else if !stdout.trim().is_empty() {
            stdout.trim()
        } else {
            "Error executing smbclient deletion"
        };
        Err(format!("Could not delete: {}", err_msg))
    }

    /// Downloads a file from the router to a local path
    pub async fn download_file(
        ip: Ipv4Addr,
        share: &str,
        remote_path: &str,
        local_dest: &Path,
        creds: &StorageCredentials,
    ) -> Result<PathBuf, String> {
        #[cfg(windows)]
        {
            return Self::win_download_file(ip, share, remote_path, local_dest, creds).await;
        }

        let clean_remote = remote_path.trim_matches('/');
        let win_remote = clean_remote.replace('/', "\\");

        let service = format!("//{}/{}", ip, share);
        let smb_command = format!("get \"{}\" \"{}\"", win_remote, local_dest.display());

        let mut cmd = crate::procutil::NoWindow::no_window(Command::new("smbclient"));
        cmd.arg(&service)
            .arg("-c")
            .arg(&smb_command)
            .arg("-t")
            .arg("30");

        for arg in Self::build_auth_args(creds) {
            cmd.arg(arg);
        }

        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Error downloading file: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Download failed: {}", stderr.trim()));
        }

        Ok(local_dest.to_path_buf())
    }

    /// Displays native OS picker (zenity / kdialog) to select a local file or folder
    pub async fn pick_path_with_file_dialog(is_directory: bool, title: &str) -> Option<PathBuf> {
        #[cfg(windows)]
        {
            Self::win_pick_path(is_directory, title).await
        }
        #[cfg(not(windows))]
        {
        // 1. Try with zenity (GTK / GNOME / Cinnamon / XFCE / MATE environments)
        let mut cmd = crate::procutil::NoWindow::no_window(Command::new("zenity"));
        cmd.arg("--file-selection");
        cmd.arg(format!("--title={}", title));
        if is_directory {
            cmd.arg("--directory");
        }

        if let Ok(output) = cmd.output().await {
            if output.status.success() {
                let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path_str.is_empty() {
                    let path = PathBuf::from(path_str);
                    if path.exists() {
                        return Some(path);
                    }
                }
            }
        }

        // 2. Try with kdialog (KDE Plasma / Qt environments)
        let mut kcmd = crate::procutil::NoWindow::no_window(Command::new("kdialog"));
        kcmd.arg(format!("--title={}", title));
        if is_directory {
            kcmd.arg("--getexistingdirectory");
            kcmd.arg("~");
        } else {
            kcmd.arg("--getopenfilename");
            kcmd.arg("~");
        }

        if let Ok(output) = kcmd.output().await {
            if output.status.success() {
                let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path_str.is_empty() {
                    let path = PathBuf::from(path_str);
                    if path.exists() {
                        return Some(path);
                    }
                }
            }
        }

        None
        }
    }

    /// Recursively collects all local files within a directory
    pub fn collect_files_recursive(
        dir: &Path,
        prefix: &str,
        out: &mut Vec<LocalFileInfo>,
    ) -> Result<(), String> {
        let entries = std::fs::read_dir(dir)
            .map_err(|e| format!("Error reading directory '{}': {}", dir.display(), e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            let name = match path.file_name().and_then(|s| s.to_str()) {
                Some(n) => n,
                None => continue,
            };

            let rel = if prefix.is_empty() {
                name.to_string()
            } else {
                format!("{}/{}", prefix, name)
            };

            if path.is_dir() {
                Self::collect_files_recursive(&path, &rel, out)?;
            } else if path.is_file() {
                let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                out.push(LocalFileInfo {
                    abs_path: path,
                    rel_path: rel,
                    size,
                });
            }
        }

        Ok(())
    }

}
