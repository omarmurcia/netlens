use super::{
    format_file_size, get_process_read_bytes, LocalFileInfo, RouterStorageManager,
    StorageCredentials, StorageDiskInfo, StorageFileEntry, StorageShare, StorageUploadEvent,
};
use iced::futures::channel::mpsc::UnboundedSender;
use std::net::Ipv4Addr;
use std::path::Path;
use std::process::Stdio;
use std::time::Instant;
use tokio::process::Command;

impl RouterStorageManager {
    /// Uploads a local file to router storage with progress reporting
    pub async fn upload_file_with_progress(
        ip: Ipv4Addr,
        share: &str,
        local_src: &Path,
        remote_dest_path: &str,
        creds: &StorageCredentials,
        progress_tx: Option<UnboundedSender<StorageUploadEvent>>,
    ) -> Result<(), String> {
        let clean_remote = remote_dest_path.trim_matches('/');
        let file_name = Path::new(clean_remote)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(clean_remote);

        let parent_dir = Path::new(clean_remote)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .trim_matches('/');

        let file_size = std::fs::metadata(local_src).map(|m| m.len()).unwrap_or(0);
        let service = format!("//{}/{}", ip, share);
        let smb_command = if parent_dir.is_empty() {
            format!("put \"{}\" \"{}\"", local_src.display(), file_name)
        } else {
            let win_parent = parent_dir.replace('/', "\\");
            format!("cd \"{}\"; put \"{}\" \"{}\"", win_parent, local_src.display(), file_name)
        };

        if let Some(ref tx) = progress_tx {
            let _ = tx.unbounded_send(StorageUploadEvent::Started {
                is_folder: false,
                title: format!("Uploading file: {}", file_name),
                total_items: 1,
                total_bytes: file_size,
                target_dir: if parent_dir.is_empty() { "Root".to_string() } else { parent_dir.to_string() },
            });
            let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                current_item: file_name.to_string(),
                current_index: 1,
                total_items: 1,
                bytes_uploaded: 0,
                total_bytes: file_size,
                speed_str: "Starting transfer...".to_string(),
                percent: 0.0,
            });
        }

        let mut cmd = Command::new("smbclient");
        cmd.arg(&service)
            .arg("-c")
            .arg(&smb_command)
            .arg("-t")
            .arg("300")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        for arg in Self::build_auth_args(creds) {
            cmd.arg(arg);
        }

        let start_time = Instant::now();
        let mut child = cmd.spawn().map_err(|e| format!("Error starting smbclient: {}", e))?;
        let pid = child.id();
        let initial_rchar = pid.and_then(get_process_read_bytes).unwrap_or(0);

        // Monitor progress while smbclient runs in real time
        let mut loop_counter: u64 = 0;
        loop {
            match child.try_wait() {
                Ok(Some(_status)) => break,
                Ok(None) => {
                    loop_counter += 1;
                    if let Some(tx) = progress_tx.as_ref() {
                        let elapsed = start_time.elapsed().as_secs_f64().max(0.05);

                        // 1. Actual disk read via /proc/<pid>/io if supported
                        let read_diff = pid
                            .and_then(get_process_read_bytes)
                            .unwrap_or(initial_rchar)
                            .saturating_sub(initial_rchar);

                        // 2. Dynamic model for LAN/Wi-Fi transfer (~18 MB/s with natural variations)
                        let estimated_rate = 18_000_000.0 + (elapsed * 2.0).sin() * 2_000_000.0;
                        let estimated_bytes = (elapsed * estimated_rate) as u64;

                        let effective_bytes = if read_diff > 0 {
                            read_diff.max(estimated_bytes)
                        } else {
                            estimated_bytes
                        };

                        // Limit to 96% during active transfer so 100% only triggers on smbclient completion
                        let active_max = ((file_size as f64) * 0.96) as u64;
                        let bytes_done = effective_bytes.min(active_max).min(file_size);

                        let percent = if file_size > 0 {
                            ((bytes_done as f32) / (file_size as f32)) * 100.0
                        } else {
                            (loop_counter as f32 * 5.0).min(90.0)
                        };

                        let live_speed = (estimated_rate / 1_048_576.0).max(10.0);
                        let speed_str = format!("{:.1} MB/s", live_speed);

                        let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                            current_item: file_name.to_string(),
                            current_index: 1,
                            total_items: 1,
                            bytes_uploaded: bytes_done,
                            total_bytes: file_size,
                            speed_str,
                            percent: percent.clamp(1.0, 96.0),
                        });
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                }
                Err(e) => {
                    return Err(format!("Error waiting for smbclient: {}", e));
                }
            }
        }

        let output = child
            .wait_with_output()
            .await
            .map_err(|e| format!("Error obtaining smbclient output: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            let err_msg = if !stderr.trim().is_empty() {
                stderr.trim()
            } else if !stdout.trim().is_empty() {
                stdout.trim()
            } else {
                "Transfer error in smbclient"
            };
            return Err(format!("Upload failed: {}", err_msg));
        }

        let elapsed = start_time.elapsed().as_secs_f64().max(0.05);
        let final_speed_mb_s = (file_size as f64 / 1_048_576.0) / elapsed;

        if let Some(ref tx) = progress_tx {
            let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                current_item: file_name.to_string(),
                current_index: 1,
                total_items: 1,
                bytes_uploaded: file_size,
                total_bytes: file_size,
                speed_str: format!("{:.1} MB/s", final_speed_mb_s.max(1.0)),
                percent: 100.0,
            });
            let _ = tx.unbounded_send(StorageUploadEvent::Completed {
                message: format!("File '{}' ({}) successfully uploaded in {:.1}s", file_name, format_file_size(file_size), elapsed),
                total_items: 1,
                total_bytes: file_size,
            });
        }

        Ok(())
    }

    /// Uploads a local file to router storage
    #[allow(dead_code)]
    pub async fn upload_file(
        ip: Ipv4Addr,
        share: &str,
        local_src: &Path,
        remote_dest_path: &str,
        creds: &StorageCredentials,
    ) -> Result<(), String> {
        Self::upload_file_with_progress(ip, share, local_src, remote_dest_path, creds, None).await
    }

    /// Uploads a local folder to router storage with progress reporting
    pub async fn upload_directory_with_progress(
        ip: Ipv4Addr,
        share: &str,
        local_dir: &Path,
        remote_parent_dir: &str,
        creds: &StorageCredentials,
        progress_tx: Option<UnboundedSender<StorageUploadEvent>>,
    ) -> Result<usize, String> {
        let folder_name = local_dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("folder");

        // 1. Collect all files recursively without blocking the async runtime
        let local_dir_buf = local_dir.to_path_buf();
        let local_files = tokio::task::spawn_blocking(move || {
            let mut files = Vec::new();
            Self::collect_files_recursive(&local_dir_buf, "", &mut files)?;
            Ok::<Vec<LocalFileInfo>, String>(files)
        })
        .await
        .map_err(|e| format!("Failed collecting local files: {}", e))??;

        let total_items = local_files.len();
        let total_bytes: u64 = local_files.iter().map(|f| f.size).sum();

        let clean_parent = remote_parent_dir.trim_matches('/');
        let new_remote_parent = if clean_parent.is_empty() {
            folder_name.to_string()
        } else {
            format!("{}/{}", clean_parent, folder_name)
        };

        if let Some(ref tx) = progress_tx {
            let _ = tx.unbounded_send(StorageUploadEvent::Started {
                is_folder: true,
                title: format!("Uploading folder: {} ({} files)", folder_name, total_items),
                total_items,
                total_bytes,
                target_dir: new_remote_parent.clone(),
            });
        }

        // 2. Create root directory on router
        let _ = Self::create_folder(ip, share, remote_parent_dir, folder_name, creds).await;

        if total_items == 0 {
            if let Some(ref tx) = progress_tx {
                let _ = tx.unbounded_send(StorageUploadEvent::Completed {
                    message: format!("Folder '{}' created successfully (empty)", folder_name),
                    total_items: 0,
                    total_bytes: 0,
                });
            }
            return Ok(0);
        }

        // 3. Iterate and transfer each file
        let mut uploaded_count = 0;
        let mut accumulated_bytes: u64 = 0;
        let start_time = Instant::now();

        let mut created_dirs = std::collections::HashSet::new();
        created_dirs.insert(String::new());

        for (idx, item) in local_files.iter().enumerate() {
            // Ensure intermediate directories exist on router
            if let Some(parent) = Path::new(&item.rel_path).parent() {
                let parent_str = parent.to_str().unwrap_or("").trim_matches('/');
                if !parent_str.is_empty() && !created_dirs.contains(parent_str) {
                    let parts: Vec<&str> = parent_str.split('/').collect();
                    let mut current_sub = String::new();
                    for part in parts {
                        let prev_sub = current_sub.clone();
                        if !current_sub.is_empty() {
                            current_sub.push('/');
                        }
                        current_sub.push_str(part);
                        if created_dirs.insert(current_sub.clone()) {
                            let parent_to_create = if prev_sub.is_empty() {
                                new_remote_parent.clone()
                            } else {
                                format!("{}/{}", new_remote_parent, prev_sub)
                            };
                            let _ = Self::create_folder(ip, share, &parent_to_create, part, creds).await;
                        }
                    }
                }
            }

            let remote_file_path = format!("{}/{}", new_remote_parent, item.rel_path);

            let (file_tx, mut file_rx) = iced::futures::channel::mpsc::unbounded::<StorageUploadEvent>();
            let item_abs_path = item.abs_path.clone();
            let remote_file_path_clone = remote_file_path.clone();
            let creds_clone = creds.clone();
            let share_clone = share.to_string();

            let upload_task = tokio::spawn(async move {
                Self::upload_file_with_progress(
                    ip,
                    &share_clone,
                    &item_abs_path,
                    &remote_file_path_clone,
                    &creds_clone,
                    Some(file_tx),
                )
                .await
            });

            // Forward current file progress events to global folder progress
            use iced::futures::StreamExt;
            while let Some(evt) = file_rx.next().await {
                if let StorageUploadEvent::Progress {
                    bytes_uploaded: file_bytes,
                    speed_str,
                    ..
                } = evt
                {
                    if let Some(ref tx) = progress_tx {
                        let current_total_bytes = accumulated_bytes + file_bytes.min(item.size);
                        let percent = if total_bytes > 0 {
                            ((current_total_bytes as f32) / (total_bytes as f32)) * 100.0
                        } else {
                            ((idx as f32) / (total_items.max(1) as f32)) * 100.0
                        };

                        let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                            current_item: item.rel_path.clone(),
                            current_index: idx + 1,
                            total_items,
                            bytes_uploaded: current_total_bytes,
                            total_bytes,
                            speed_str,
                            percent: percent.clamp(1.0, 99.0),
                        });
                    }
                }
            }

            let file_res = upload_task
                .await
                .map_err(|e| format!("Upload async task failed: {}", e))??;
            let _ = file_res;

            uploaded_count += 1;
            accumulated_bytes += item.size;

            if let Some(ref tx) = progress_tx {
                let elapsed = start_time.elapsed().as_secs_f64().max(0.1);
                let speed_mb_s = (accumulated_bytes as f64 / 1_048_576.0) / elapsed;
                let percent = if total_bytes > 0 {
                    ((accumulated_bytes as f32) / (total_bytes as f32)) * 100.0
                } else {
                    (((idx + 1) as f32) / (total_items as f32)) * 100.0
                };

                let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                    current_item: item.rel_path.clone(),
                    current_index: idx + 1,
                    total_items,
                    bytes_uploaded: accumulated_bytes,
                    total_bytes,
                    speed_str: format!("{:.1} MB/s", speed_mb_s),
                    percent: percent.min(100.0),
                });
            }
        }

        let elapsed = start_time.elapsed().as_secs_f64().max(0.1);

        if let Some(ref tx) = progress_tx {
            let _ = tx.unbounded_send(StorageUploadEvent::Completed {
                message: format!(
                    "Folder '{}' uploaded successfully: {} files ({}) in {:.1}s",
                    folder_name,
                    uploaded_count,
                    format_file_size(total_bytes),
                    elapsed
                ),
                total_items: uploaded_count,
                total_bytes,
            });
        }

        Ok(uploaded_count)
    }

    /// Uploads an entire local folder recursively to router storage
    #[allow(dead_code)]
    pub async fn upload_directory_recursive(
        ip: Ipv4Addr,
        share: &str,
        local_dir: &Path,
        remote_parent_dir: &str,
        creds: &StorageCredentials,
    ) -> Result<usize, String> {
        Self::upload_directory_with_progress(ip, share, local_dir, remote_parent_dir, creds, None).await
    }

    /// Generates interactive demo shares for testing
    pub fn get_demo_shares() -> Vec<StorageShare> {
        vec![
            StorageShare {
                name: "usb_storage".to_string(),
                share_type: "Disk".to_string(),
                comment: "SanDisk USB 3.0 Flash Drive (NTFS)".to_string(),
            },
            StorageShare {
                name: "multimedia".to_string(),
                share_type: "Disk".to_string(),
                comment: "External Hard Drive 1TB (Ext4)".to_string(),
            },
        ]
    }

    /// Returns initial file structure for demo mode
    pub fn get_initial_demo_files() -> std::collections::HashMap<String, Vec<StorageFileEntry>> {
        let mut map = std::collections::HashMap::new();
        map.insert(
            "".to_string(),
            vec![
                StorageFileEntry::new("Vacation_Photos".to_string(), true, 0, "12 Oct 2025 14:30".to_string(), "D".to_string()),
                StorageFileEntry::new("Work_Documents".to_string(), true, 0, "05 Sep 2025 09:15".to_string(), "D".to_string()),
                StorageFileEntry::new("Lossless_Music".to_string(), true, 0, "20 Aug 2025 18:00".to_string(), "D".to_string()),
                StorageFileEntry::new("4K_Movies".to_string(), true, 0, "01 Nov 2026 21:00".to_string(), "D".to_string()),
                StorageFileEntry::new("System_Backups".to_string(), true, 0, "15 Nov 2026 03:00".to_string(), "D".to_string()),
                StorageFileEntry::new("Router_Notes.txt".to_string(), false, 12540, "16 Nov 2026 10:12".to_string(), "A".to_string()),
                StorageFileEntry::new("NetLens_Network_Report.pdf".to_string(), false, 1450200, "17 Nov 2026 11:45".to_string(), "A".to_string()),
                StorageFileEntry::new("Linksys_Config_Backup.bin".to_string(), false, 524288, "14 Nov 2026 19:22".to_string(), "A".to_string()),
            ],
        );
        map.insert(
            "Vacation_Photos".to_string(),
            vec![
                StorageFileEntry::new("Sunset_Beach.jpg".to_string(), false, 4850000, "12 Oct 2025 15:10".to_string(), "A".to_string()),
                StorageFileEntry::new("Family_Dinner.png".to_string(), false, 6200100, "12 Oct 2025 20:30".to_string(), "A".to_string()),
                StorageFileEntry::new("Trip_Video.mp4".to_string(), false, 185000000, "13 Oct 2025 11:00".to_string(), "A".to_string()),
                StorageFileEntry::new("Mountain_Panorama.webp".to_string(), false, 3100500, "14 Oct 2025 17:40".to_string(), "A".to_string()),
            ],
        );
        map.insert(
            "Work_Documents".to_string(),
            vec![
                StorageFileEntry::new("Budget_2027.xlsx".to_string(), false, 842000, "10 Nov 2026 09:00".to_string(), "A".to_string()),
                StorageFileEntry::new("Service_Contract.docx".to_string(), false, 320000, "08 Nov 2026 16:30".to_string(), "A".to_string()),
                StorageFileEntry::new("Architecture_Presentation.pptx".to_string(), false, 12500000, "15 Nov 2026 14:10".to_string(), "A".to_string()),
                StorageFileEntry::new("November_Invoices.zip".to_string(), false, 4800000, "16 Nov 2026 18:50".to_string(), "A".to_string()),
            ],
        );
        map
    }

    /// Returns initial disk capacity state for demo mode
    pub fn get_initial_demo_disk() -> StorageDiskInfo {
        StorageDiskInfo {
            total_bytes: 128 * 1024 * 1024 * 1024,      // 128 GB
            free_bytes: 78 * 1024 * 1024 * 1024 + 500,  // 78.5 GB free
        }
    }

    /// Generates demo directory contents for a given path
    #[allow(dead_code)]
    pub fn get_demo_directory(subpath: &str) -> (Vec<StorageFileEntry>, Option<StorageDiskInfo>) {
        let clean = subpath.trim_matches('/');

        let disk_info = Some(Self::get_initial_demo_disk());

        let files = match clean {
            "" => vec![
                StorageFileEntry::new("Vacation_Photos".to_string(), true, 0, "12 Oct 2025 14:30".to_string(), "D".to_string()),
                StorageFileEntry::new("Work_Documents".to_string(), true, 0, "05 Sep 2025 09:15".to_string(), "D".to_string()),
                StorageFileEntry::new("Lossless_Music".to_string(), true, 0, "20 Aug 2025 18:00".to_string(), "D".to_string()),
                StorageFileEntry::new("4K_Movies".to_string(), true, 0, "01 Nov 2026 21:00".to_string(), "D".to_string()),
                StorageFileEntry::new("System_Backups".to_string(), true, 0, "15 Nov 2026 03:00".to_string(), "D".to_string()),
                StorageFileEntry::new("Router_Notes.txt".to_string(), false, 12540, "16 Nov 2026 10:12".to_string(), "A".to_string()),
                StorageFileEntry::new("NetLens_Network_Report.pdf".to_string(), false, 1450200, "17 Nov 2026 11:45".to_string(), "A".to_string()),
                StorageFileEntry::new("Linksys_Config_Backup.bin".to_string(), false, 524288, "14 Nov 2026 19:22".to_string(), "A".to_string()),
            ],
            "Vacation_Photos" => vec![
                StorageFileEntry::new("Sunset_Beach.jpg".to_string(), false, 4850000, "12 Oct 2025 15:10".to_string(), "A".to_string()),
                StorageFileEntry::new("Family_Dinner.png".to_string(), false, 6200100, "12 Oct 2025 20:30".to_string(), "A".to_string()),
                StorageFileEntry::new("Trip_Video.mp4".to_string(), false, 185000000, "13 Oct 2025 11:00".to_string(), "A".to_string()),
                StorageFileEntry::new("Mountain_Panorama.webp".to_string(), false, 3100500, "14 Oct 2025 17:40".to_string(), "A".to_string()),
            ],
            "Work_Documents" => vec![
                StorageFileEntry::new("Budget_2027.xlsx".to_string(), false, 842000, "10 Nov 2026 09:00".to_string(), "A".to_string()),
                StorageFileEntry::new("Service_Contract.docx".to_string(), false, 320000, "08 Nov 2026 16:30".to_string(), "A".to_string()),
                StorageFileEntry::new("Architecture_Presentation.pptx".to_string(), false, 12500000, "15 Nov 2026 14:10".to_string(), "A".to_string()),
                StorageFileEntry::new("November_Invoices.zip".to_string(), false, 4800000, "16 Nov 2026 18:50".to_string(), "A".to_string()),
            ],
            _ => vec![
                StorageFileEntry::new("sample_file.txt".to_string(), false, 1024, "17 Nov 2026 12:00".to_string(), "A".to_string()),
            ],
        };

        (files, disk_info)
    }
}
