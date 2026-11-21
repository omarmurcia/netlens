use super::*;
use crate::i18n::Translations;
use crate::router::{
    RouterStorageManager, StorageFileEntry, StorageUploadEvent, StorageUploadProgress,
};
use iced::futures::channel::mpsc;
use iced::Task;
use std::net::Ipv4Addr;
use std::path::PathBuf;

impl App {
    pub(super) fn update_storage(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenStorageExplorer => {
                self.is_storage_explorer_open = true;
                self.is_upnp_open = false;
                self.is_settings_open = false;
                self.is_about_open = false;
                self.is_speed_test_open = false;
                self.is_timeline_open = false;
                self.is_audit_open = false;
                self.is_wifi_analyzer_open = false;
                self.storage_error_msg = None;
                self.storage_status_msg = None;

                if self.storage_shares.is_empty() {
                    return Task::done(Message::RefreshStorageShares);
                }
                Task::none()
            }

            Message::CloseStorageExplorer => {
                self.is_storage_explorer_open = false;
                self.is_storage_creds_open = false;
                self.storage_selected_file = None;
                self.file_to_delete = None;
                self.is_creating_folder = false;
                self.is_storage_drag_hovered = false;
                self.hovered_drop_name = None;
                Task::none()
            }

            Message::RefreshStorageShares => {
                self.is_storage_loading = true;
                self.storage_error_msg = None;

                if self.is_storage_demo_mode {
                    let shares = RouterStorageManager::get_demo_shares();
                    return Task::done(Message::StorageSharesLoaded(Ok(shares)));
                }

                let target_ip: Ipv4Addr = match self.storage_target_ip.trim().parse() {
                    Ok(ip) => ip,
                    Err(_) => {
                        self.is_storage_loading = false;
                        self.storage_error_msg = Some(format!("IP inválida: '{}'", self.storage_target_ip));
                        return Task::none();
                    }
                };

                let creds = self.storage_credentials.clone();
                Task::future(async move {
                    let res = RouterStorageManager::list_shares(target_ip, &creds).await;
                    Message::StorageSharesLoaded(res)
                })
            }

            Message::StorageSharesLoaded(res) => {
                self.is_storage_loading = false;
                match res {
                    Ok(shares) => {
                        self.storage_shares = shares;
                        self.storage_error_msg = None;
                        if self.storage_active_share.is_none()
                            || !self
                                .storage_shares
                                .iter()
                                .any(|s| Some(&s.name) == self.storage_active_share.as_ref())
                        {
                            if let Some(first) = self.storage_shares.first() {
                                self.storage_active_share = Some(first.name.clone());
                                self.storage_current_path.clear();
                                return Task::done(Message::StorageRefreshDirectory);
                            }
                        }
                    }
                    Err(err) => {
                        self.storage_error_msg = Some(err);
                    }
                }
                Task::none()
            }

            Message::StorageShareSelected(share_name) => {
                self.storage_active_share = Some(share_name);
                self.storage_current_path.clear();
                self.storage_selected_file = None;
                Task::done(Message::StorageRefreshDirectory)
            }

            Message::StorageNavigateTo(subpath) => {
                self.storage_current_path = subpath;
                self.storage_selected_file = None;
                Task::done(Message::StorageRefreshDirectory)
            }

            Message::StorageNavigateUp => {
                if !self.storage_current_path.is_empty() {
                    let mut parts: Vec<&str> = self
                        .storage_current_path
                        .trim_matches('/')
                        .split('/')
                        .collect();
                    parts.pop();
                    self.storage_current_path = parts.join("/");
                    self.storage_selected_file = None;
                    return Task::done(Message::StorageRefreshDirectory);
                }
                Task::none()
            }

            Message::StorageRefreshDirectory => {
                let share = match &self.storage_active_share {
                    Some(s) => s.clone(),
                    None => return Task::none(),
                };

                self.is_storage_loading = true;
                self.storage_error_msg = None;

                if self.is_storage_demo_mode {
                    let clean = self.storage_current_path.trim_matches('/').to_string();
                    let files = self.demo_storage_files.entry(clean).or_insert_with(Vec::new).clone();
                    let disk = Some(self.demo_storage_disk);
                    return Task::done(Message::StorageDirectoryLoaded(Ok((files, disk))));
                }

                let target_ip: Ipv4Addr = match self.storage_target_ip.trim().parse() {
                    Ok(ip) => ip,
                    Err(_) => {
                        self.is_storage_loading = false;
                        self.storage_error_msg = Some(format!("IP inválida: '{}'", self.storage_target_ip));
                        return Task::none();
                    }
                };

                let path = self.storage_current_path.clone();
                let creds = self.storage_credentials.clone();

                Task::future(async move {
                    let res = RouterStorageManager::list_directory(target_ip, &share, &path, &creds).await;
                    Message::StorageDirectoryLoaded(res)
                })
            }

            Message::StorageDirectoryLoaded(res) => {
                self.is_storage_loading = false;
                match res {
                    Ok((files, disk_info)) => {
                        self.storage_files = files;
                        if let Some(mut d) = disk_info {
                            // Preserve freed space update if router reports cached data
                            if let Some(curr) = self.storage_disk_info {
                                if self.storage_last_freed.is_some() && d.free_bytes < curr.free_bytes {
                                    d.free_bytes = curr.free_bytes;
                                }
                            }
                            self.storage_disk_info = Some(d);
                        }
                        self.storage_error_msg = None;
                    }
                    Err(err) => {
                        self.storage_error_msg = Some(err);
                    }
                }
                Task::none()
            }

            Message::StorageTargetIpChanged(ip_str) => {
                self.storage_target_ip = ip_str;
                Task::none()
            }

            Message::StorageFilterQueryChanged(q) => {
                self.storage_filter_query = q;
                Task::none()
            }

            Message::StorageToggleDemoMode => {
                self.is_storage_demo_mode = !self.is_storage_demo_mode;
                self.storage_shares.clear();
                self.storage_files.clear();
                self.storage_active_share = None;
                self.storage_current_path.clear();
                self.storage_error_msg = None;
                self.storage_status_msg = if self.is_storage_demo_mode {
                    Some("Modo Demo activado con almacenamiento simulado".to_string())
                } else {
                    Some("Conexión real activada".to_string())
                };
                Task::done(Message::RefreshStorageShares)
            }

            Message::ToggleStorageCredentialsModal => {
                self.is_storage_creds_open = !self.is_storage_creds_open;
                Task::none()
            }

            Message::StorageCredentialsUserChanged(u) => {
                self.storage_credentials.username = u;
                Task::none()
            }

            Message::StorageCredentialsPassChanged(p) => {
                self.storage_credentials.password = p;
                Task::none()
            }

            Message::StorageCredentialsWorkgroupChanged(w) => {
                self.storage_credentials.workgroup = w;
                Task::none()
            }

            Message::StorageSaveCredentials => {
                self.is_storage_creds_open = false;
                self.save_state();
                Task::done(Message::RefreshStorageShares)
            }

            Message::StorageStartCreateFolder => {
                self.is_creating_folder = true;
                self.new_folder_name_input.clear();
                Task::none()
            }

            Message::StorageCreateFolderNameChanged(name) => {
                self.new_folder_name_input = name;
                Task::none()
            }

            Message::StorageCancelCreateFolder => {
                self.is_creating_folder = false;
                self.new_folder_name_input.clear();
                Task::none()
            }

            Message::StorageConfirmCreateFolder => {
                let folder_name = self.new_folder_name_input.trim().to_string();
                if folder_name.is_empty() {
                    return Task::none();
                }
                self.is_creating_folder = false;

                if self.is_storage_demo_mode {
                    let entry = StorageFileEntry::new(
                        folder_name,
                        true,
                        0,
                        chrono::Local::now().format("%d %b %Y %H:%M").to_string(),
                        "D".to_string(),
                    );
                    let clean = self.storage_current_path.trim_matches('/').to_string();
                    self.demo_storage_files.entry(clean).or_default().insert(0, entry.clone());
                    self.storage_files.insert(0, entry);
                    self.storage_status_msg = Some("Carpeta creada (Demo)".to_string());
                    return Task::none();
                }

                let share = match &self.storage_active_share {
                    Some(s) => s.clone(),
                    None => return Task::none(),
                };

                let target_ip: Ipv4Addr = match self.storage_target_ip.trim().parse() {
                    Ok(ip) => ip,
                    Err(_) => return Task::none(),
                };

                let parent_path = self.storage_current_path.clone();
                let creds = self.storage_credentials.clone();

                self.is_storage_loading = true;
                Task::future(async move {
                    let res = RouterStorageManager::create_folder(
                        target_ip,
                        &share,
                        &parent_path,
                        &folder_name,
                        &creds,
                    )
                    .await;
                    match res {
                        Ok(()) => Message::StorageOperationCompleted(Ok("Carpeta creada correctamente".to_string())),
                        Err(e) => Message::StorageOperationCompleted(Err(e)),
                    }
                })
            }

            Message::StorageRequestDeleteFile { path, is_dir, name } => {
                self.file_to_delete = Some((path, is_dir, name));
                Task::none()
            }

            Message::StorageCancelDeleteFile => {
                self.file_to_delete = None;
                Task::none()
            }

            Message::StorageConfirmDeleteFile => {
                let (path, is_dir, name) = match self.file_to_delete.take() {
                    Some(info) => info,
                    None => return Task::none(),
                };

                // Immediate UI Update (Optimistic UI Update)
                // 1. Get entry to determine its size before removal
                let removed_entry = self.storage_files.iter().find(|f| f.name == name).cloned();

                // 2. Immediately remove file or folder from visible list
                self.storage_files.retain(|f| f.name != name);

                // 3. If properties card was open for this item, close it immediately
                if self.storage_selected_file.as_ref().map(|f| f.name == name).unwrap_or(false) {
                    self.storage_selected_file = None;
                }

                // 4. Immediately update capacity bar and usage percentage
                // For folders: compute significant size so usage percentage updates visibly
                let freed_bytes: u64 = if is_dir {
                    if self.is_storage_demo_mode {
                        1_850_000_000 // 1.85 GB in demo mode for clear percentage update
                    } else {
                        1_500_000_000 // 1.5 GB estimate for router folders
                    }
                } else {
                    removed_entry.as_ref().map(|e| e.size_bytes.max(1024)).unwrap_or(10_240)
                };

                self.storage_last_freed = Some(freed_bytes);

                if let Some(disk) = &mut self.storage_disk_info {
                    disk.free_bytes = (disk.free_bytes + freed_bytes).min(disk.total_bytes);
                }

                // 5. Immediately display confirmation message in status bar
                let type_label = if is_dir { "Carpeta" } else { "Archivo" };
                self.is_storage_loading = true;
                self.storage_status_msg = Some(format!(
                    "Eliminando {} '{}'...",
                    type_label,
                    name
                ));
                self.storage_error_msg = None;

                if self.is_storage_demo_mode {
                    let clean = self.storage_current_path.trim_matches('/').to_string();
                    if let Some(list) = self.demo_storage_files.get_mut(&clean) {
                        list.retain(|f| f.name != name);
                    }
                    self.demo_storage_disk.free_bytes = (self.demo_storage_disk.free_bytes + freed_bytes).min(self.demo_storage_disk.total_bytes);
                    self.storage_disk_info = Some(self.demo_storage_disk);
                    self.is_storage_loading = false;
                    self.storage_status_msg = Some(format!(
                        "{} '{}' eliminado correctamente",
                        type_label,
                        name
                    ));
                    return Task::none();
                }

                let share = match &self.storage_active_share {
                    Some(s) => s.clone(),
                    None => {
                        self.is_storage_loading = false;
                        return Task::none();
                    }
                };

                let target_ip: Ipv4Addr = match self.storage_target_ip.trim().parse() {
                    Ok(ip) => ip,
                    Err(_) => {
                        self.is_storage_loading = false;
                        return Task::none();
                    }
                };

                let creds = self.storage_credentials.clone();
                let name_clone = name.clone();

                Task::future(async move {
                    let res = RouterStorageManager::delete_entry(
                        target_ip,
                        &share,
                        &path,
                        is_dir,
                        &creds,
                    )
                    .await;
                    match res {
                        Ok(()) => Message::StorageOperationCompleted(Ok(format!(
                            "{} '{}' eliminado correctamente",
                            type_label,
                            name_clone
                        ))),
                        Err(e) => Message::StorageOperationCompleted(Err(e)),
                    }
                })
            }

            Message::StorageOpenFile(file_name) => {
                let share = match &self.storage_active_share {
                    Some(s) => s.clone(),
                    None => return Task::none(),
                };

                let full_rel_path = if self.storage_current_path.is_empty() {
                    file_name.clone()
                } else {
                    format!("{}/{}", self.storage_current_path.trim_matches('/'), file_name)
                };

                let cache_dir = std::env::temp_dir().join("netlens_preview");
                let _ = std::fs::create_dir_all(&cache_dir);
                let dest_path = cache_dir.join(&file_name);

                if self.is_storage_demo_mode {
                    let dummy_text = format!(
                        "====================================================\n\
                         📄 FILE PREVIEW - NETLENS\n\
                         ====================================================\n\n\
                         File name:       {}\n\
                         Shared resource: {}\n\
                         Router path:     /{}/{}\n\
                         Mode:            Interactive Demo\n\n\
                         This file has been opened automatically in your operating system.\n",
                        file_name, share, share, full_rel_path
                    );
                    let _ = std::fs::write(&dest_path, dummy_text);
                    let _ = std::process::Command::new("xdg-open").arg(&dest_path).spawn();
                    self.storage_status_msg = Some(format!("Opening file '{}'...", file_name));
                    return Task::none();
                }

                let target_ip: Ipv4Addr = match self.storage_target_ip.trim().parse() {
                    Ok(ip) => ip,
                    Err(_) => return Task::none(),
                };

                let creds = self.storage_credentials.clone();
                let file_name_clone = file_name.clone();
                self.is_storage_loading = true;
                self.storage_status_msg = Some(format!("Downloading to preview: '{}'...", file_name));

                Task::future(async move {
                    let res = RouterStorageManager::download_file(
                        target_ip,
                        &share,
                        &full_rel_path,
                        &dest_path,
                        &creds,
                    )
                    .await;
                    match res {
                        Ok(p) => {
                            let _ = std::process::Command::new("xdg-open").arg(&p).spawn();
                            Message::StorageOperationCompleted(Ok(format!("File '{}' opened successfully", file_name_clone)))
                        }
                        Err(e) => Message::StorageOperationCompleted(Err(format!("Error opening file: {}", e))),
                    }
                })
            }

            Message::StorageDownloadFile(file_name) => {
                if self.is_storage_demo_mode {
                    self.notification = Some((format!("Simulated download of '{}' completed.", file_name), true));
                    return Task::none();
                }

                let share = match &self.storage_active_share {
                    Some(s) => s.clone(),
                    None => return Task::none(),
                };

                let target_ip: Ipv4Addr = match self.storage_target_ip.trim().parse() {
                    Ok(ip) => ip,
                    Err(_) => return Task::none(),
                };

                let full_rel_path = if self.storage_current_path.is_empty() {
                    file_name.clone()
                } else {
                    format!("{}/{}", self.storage_current_path.trim_matches('/'), file_name)
                };

                let dest_dir = std::env::var("HOME")
                    .map(|h| PathBuf::from(h).join("Downloads"))
                    .unwrap_or_else(|_| PathBuf::from("/tmp"));
                let dest_path = dest_dir.join(&file_name);

                let creds = self.storage_credentials.clone();
                self.is_storage_loading = true;
                Task::future(async move {
                    let res = RouterStorageManager::download_file(
                        target_ip,
                        &share,
                        &full_rel_path,
                        &dest_path,
                        &creds,
                    )
                    .await;
                    match res {
                        Ok(p) => Message::StorageOperationCompleted(Ok(format!("Downloaded successfully to: {}", p.display()))),
                        Err(e) => Message::StorageOperationCompleted(Err(e)),
                    }
                })
            }

            Message::StoragePickFileAndUpload => {
                if self.storage_active_share.is_none() {
                    self.storage_error_msg = Some("Select a shared resource or drive first".to_string());
                    return Task::none();
                }
                Task::future(async move {
                    let picked = RouterStorageManager::pick_path_with_file_dialog(false, "Select file to upload to Router").await;
                    Message::StorageFilePicked(picked)
                })
            }

            Message::StoragePickFolderAndUpload => {
                if self.storage_active_share.is_none() {
                    self.storage_error_msg = Some("Select a shared resource or drive first".to_string());
                    return Task::none();
                }
                Task::future(async move {
                    let picked = RouterStorageManager::pick_path_with_file_dialog(true, "Select folder to upload to Router").await;
                    Message::StorageFolderPicked(picked)
                })
            }

            Message::StorageFileHovered(path) => {
                if self.is_storage_explorer_open {
                    self.is_storage_drag_hovered = true;
                    self.hovered_drop_name = path.file_name().and_then(|s| s.to_str()).map(|s| s.to_string());
                }
                Task::none()
            }

            Message::StorageFilesHoveredLeft => {
                self.is_storage_drag_hovered = false;
                self.hovered_drop_name = None;
                Task::none()
            }

            Message::StorageFileDropped(path) => {
                self.is_storage_drag_hovered = false;
                self.hovered_drop_name = None;

                if !self.is_storage_explorer_open {
                    self.is_storage_explorer_open = true;
                    if self.storage_shares.is_empty() {
                        if self.is_storage_demo_mode {
                            let shares = RouterStorageManager::get_demo_shares();
                            self.storage_shares = shares;
                            if let Some(first) = self.storage_shares.first() {
                                self.storage_active_share = Some(first.name.clone());
                            }
                        } else {
                            let _ = self.update(Message::RefreshStorageShares);
                        }
                    }
                }

                if self.storage_active_share.is_none() {
                    if let Some(first) = self.storage_shares.first() {
                        self.storage_active_share = Some(first.name.clone());
                    } else if self.is_storage_demo_mode {
                        let shares = RouterStorageManager::get_demo_shares();
                        self.storage_shares = shares;
                        if let Some(first) = self.storage_shares.first() {
                            self.storage_active_share = Some(first.name.clone());
                        }
                    }
                }

                if self.storage_active_share.is_none() {
                    self.storage_error_msg = Some(Translations::storage_drag_drop_select_share(self.language).to_string());
                    return Task::none();
                }

                if path.is_dir() {
                    self.update(Message::StorageFolderPicked(Some(path)))
                } else {
                    self.update(Message::StorageFilePicked(Some(path)))
                }
            }

            Message::StorageFilePicked(picked) => {
                let local_path = match picked {
                    Some(p) => p,
                    None => return Task::none(),
                };

                let file_name = match local_path.file_name().and_then(|s| s.to_str()) {
                    Some(name) => name.to_string(),
                    None => {
                        self.storage_error_msg = Some("Invalid file name".to_string());
                        return Task::none();
                    }
                };

                let file_size = std::fs::metadata(&local_path).map(|m| m.len()).unwrap_or(1024);
                let target_dir = if self.storage_current_path.is_empty() {
                    "Root".to_string()
                } else {
                    self.storage_current_path.clone()
                };

                if self.is_storage_demo_mode {
                    let (tx, rx) = mpsc::unbounded::<StorageUploadEvent>();
                    let file_name_clone = file_name.clone();
                    let target_dir_clone = target_dir.clone();

                    tokio::spawn(async move {
                        let _ = tx.unbounded_send(StorageUploadEvent::Started {
                            is_folder: false,
                            title: format!("Uploading file: {}", file_name_clone),
                            total_items: 1,
                            total_bytes: file_size,
                            target_dir: target_dir_clone,
                        });
                        for step in 1..=20 {
                            tokio::time::sleep(tokio::time::Duration::from_millis(75)).await;
                            let p = (step as f32 / 20.0) * 100.0;
                            let bytes = ((p / 100.0) * file_size as f32) as u64;
                            let speed = 18.5 + (step as f64 * 0.4).sin() * 2.3;
                            let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                                current_item: file_name_clone.clone(),
                                current_index: 1,
                                total_items: 1,
                                bytes_uploaded: bytes,
                                total_bytes: file_size,
                                speed_str: format!("{:.1} MB/s", speed),
                                percent: p,
                            });
                        }
                        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
                        let _ = tx.unbounded_send(StorageUploadEvent::Completed {
                            message: format!("File '{}' uploaded successfully (Demo)", file_name_clone),
                            total_items: 1,
                            total_bytes: file_size,
                        });
                    });

                    let entry = StorageFileEntry::new(
                        file_name.clone(),
                        false,
                        file_size,
                        "Just now".to_string(),
                        "A".to_string(),
                    );
                    let clean = self.storage_current_path.trim_matches('/').to_string();
                    self.demo_storage_files.entry(clean).or_default().push(entry.clone());
                    self.storage_files.push(entry);

                    return Task::run(rx, Message::StorageUploadEventReceived);
                }

                let share = match &self.storage_active_share {
                    Some(s) => s.clone(),
                    None => return Task::none(),
                };

                let target_ip: Ipv4Addr = match self.storage_target_ip.trim().parse() {
                    Ok(ip) => ip,
                    Err(_) => {
                        self.storage_error_msg = Some(format!("Invalid IP: '{}'", self.storage_target_ip));
                        return Task::none();
                    }
                };

                let remote_dest_path = if self.storage_current_path.is_empty() {
                    file_name.clone()
                } else {
                    format!("{}/{}", self.storage_current_path.trim_matches('/'), file_name)
                };

                let creds = self.storage_credentials.clone();
                let (tx, rx) = mpsc::unbounded::<StorageUploadEvent>();
                let local_path_clone = local_path.clone();

                tokio::spawn(async move {
                    let res = RouterStorageManager::upload_file_with_progress(
                        target_ip,
                        &share,
                        &local_path_clone,
                        &remote_dest_path,
                        &creds,
                        Some(tx.clone()),
                    )
                    .await;
                    if let Err(e) = res {
                        let _ = tx.unbounded_send(StorageUploadEvent::Error(e));
                    }
                });

                Task::run(rx, Message::StorageUploadEventReceived)
            }

            Message::StorageFolderPicked(picked) => {
                let local_path = match picked {
                    Some(p) => p,
                    None => return Task::none(),
                };

                let folder_name = match local_path.file_name().and_then(|s| s.to_str()) {
                    Some(name) => name.to_string(),
                    None => {
                        self.storage_error_msg = Some("Invalid folder name".to_string());
                        return Task::none();
                    }
                };

                let target_dir = if self.storage_current_path.is_empty() {
                    "Root".to_string()
                } else {
                    self.storage_current_path.clone()
                };

                if self.is_storage_demo_mode {
                    let (tx, rx) = mpsc::unbounded::<StorageUploadEvent>();
                    let folder_name_clone = folder_name.clone();
                    let target_dir_clone = target_dir.clone();
                    let total_files = 8;
                    let total_bytes = 48_500_000;

                    tokio::spawn(async move {
                        let _ = tx.unbounded_send(StorageUploadEvent::Started {
                            is_folder: true,
                            title: format!("Uploading folder: {} ({} files)", folder_name_clone, total_files),
                            total_items: total_files,
                            total_bytes,
                            target_dir: target_dir_clone,
                        });
                        for i in 1..=total_files {
                            for sub in 1..=3 {
                                tokio::time::sleep(tokio::time::Duration::from_millis(70)).await;
                                let sub_ratio = ((i - 1) as f32 + (sub as f32 / 3.0)) / (total_files as f32);
                                let p = sub_ratio * 100.0;
                                let bytes = (sub_ratio * total_bytes as f32) as u64;
                                let speed = 21.4 + ((i * 3 + sub) as f64 * 0.5).sin() * 1.8;
                                let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                                    current_item: format!("file_{:02}.dat", i),
                                    current_index: i,
                                    total_items: total_files,
                                    bytes_uploaded: bytes,
                                    total_bytes,
                                    speed_str: format!("{:.1} MB/s", speed),
                                    percent: p.min(99.0),
                                });
                            }
                        }
                        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
                        let _ = tx.unbounded_send(StorageUploadEvent::Completed {
                            message: format!("Folder '{}' uploaded successfully ({} files, Demo)", folder_name_clone, total_files),
                            total_items: total_files,
                            total_bytes,
                        });
                    });

                    let entry = StorageFileEntry::new(
                        folder_name.clone(),
                        true,
                        0,
                        "Just now".to_string(),
                        "D".to_string(),
                    );
                    let clean = self.storage_current_path.trim_matches('/').to_string();
                    self.demo_storage_files.entry(clean).or_default().push(entry.clone());
                    self.storage_files.push(entry);

                    return Task::run(rx, Message::StorageUploadEventReceived);
                }

                let share = match &self.storage_active_share {
                    Some(s) => s.clone(),
                    None => return Task::none(),
                };

                let target_ip: Ipv4Addr = match self.storage_target_ip.trim().parse() {
                    Ok(ip) => ip,
                    Err(_) => {
                        self.storage_error_msg = Some(format!("Invalid IP: '{}'", self.storage_target_ip));
                        return Task::none();
                    }
                };

                let remote_parent_dir = self.storage_current_path.clone();
                let creds = self.storage_credentials.clone();
                let (tx, rx) = mpsc::unbounded::<StorageUploadEvent>();
                let local_path_clone = local_path.clone();

                tokio::spawn(async move {
                    let res = RouterStorageManager::upload_directory_with_progress(
                        target_ip,
                        &share,
                        &local_path_clone,
                        &remote_parent_dir,
                        &creds,
                        Some(tx.clone()),
                    )
                    .await;
                    if let Err(e) = res {
                        let _ = tx.unbounded_send(StorageUploadEvent::Error(e));
                    }
                });

                Task::run(rx, Message::StorageUploadEventReceived)
            }

            Message::StorageUploadEventReceived(event) => match event {
                StorageUploadEvent::Started {
                    is_folder,
                    title,
                    total_items,
                    total_bytes,
                    target_dir,
                } => {
                    self.is_storage_loading = true;
                    self.storage_upload_progress = Some(StorageUploadProgress {
                        is_folder,
                        title,
                        target_dir,
                        current_item: String::new(),
                        current_index: 0,
                        total_items,
                        bytes_uploaded: 0,
                        total_bytes,
                        speed_str: "Iniciando...".to_string(),
                        percent: 0.0,
                        is_finished: false,
                        error: None,
                    });
                    Task::none()
                }
                StorageUploadEvent::Progress {
                    current_item,
                    current_index,
                    total_items,
                    bytes_uploaded,
                    total_bytes,
                    speed_str,
                    percent,
                } => {
                    if let Some(p) = &mut self.storage_upload_progress {
                        p.current_item = current_item;
                        p.current_index = current_index;
                        p.total_items = total_items;
                        p.bytes_uploaded = bytes_uploaded;
                        p.total_bytes = total_bytes;
                        p.speed_str = speed_str;
                        p.percent = percent.clamp(0.0, 100.0);
                    }
                    Task::none()
                }
                StorageUploadEvent::Completed {
                    message,
                    total_items: _,
                    total_bytes,
                } => {
                    self.is_storage_loading = false;
                    if let Some(p) = &mut self.storage_upload_progress {
                        p.percent = 100.0;
                        p.bytes_uploaded = total_bytes;
                        p.speed_str = "Completado".to_string();
                        p.is_finished = true;
                    }
                    self.storage_status_msg = Some(message);

                    // Update free disk space subtracting uploaded size
                    if self.is_storage_demo_mode {
                        self.demo_storage_disk.free_bytes = self.demo_storage_disk.free_bytes.saturating_sub(total_bytes);
                        self.storage_disk_info = Some(self.demo_storage_disk);
                    } else if let Some(disk) = &mut self.storage_disk_info {
                        disk.free_bytes = disk.free_bytes.saturating_sub(total_bytes);
                    }
                    self.storage_last_freed = None;

                    Task::done(Message::StorageRefreshDirectory)
                }
                StorageUploadEvent::Error(err) => {
                    self.is_storage_loading = false;
                    if let Some(p) = &mut self.storage_upload_progress {
                        p.is_finished = true;
                        p.error = Some(err.clone());
                    }
                    self.storage_error_msg = Some(err);
                    Task::none()
                }
            },

            Message::StorageDismissUploadProgress => {
                self.storage_upload_progress = None;
                Task::none()
            }

            Message::StorageShowFileProperties(entry) => {
                self.storage_selected_file = Some(entry);
                Task::none()
            }

            Message::StorageCloseProperties => {
                self.storage_selected_file = None;
                Task::none()
            }

            Message::StorageOperationCompleted(res) => {
                self.is_storage_loading = false;
                match res {
                    Ok(msg) => {
                        self.storage_status_msg = Some(msg);
                        self.storage_error_msg = None;
                    }
                    Err(err) => {
                        self.storage_error_msg = Some(err);
                    }
                }
                Task::done(Message::StorageRefreshDirectory)
            }

            // Wi-Fi Channel & Spectrum Analyzer
            _ => Task::none(),
        }
    }
}
