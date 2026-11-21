use super::*;
use crate::autostart::AutostartManager;
use crate::history::{HistoryManager, NetworkEventType};
use crate::i18n::{Language, Translations};
use crate::model::{Device, DeviceCategory, DeviceStatus};
use crate::notification::NotificationUrgency;
use crate::realtime_monitor::NeighborAction;
use crate::scanner::Scanner;
use crate::telemetry::Telemetry;
use crate::updater::{InAppUpdateProgress, UpdateChecker, UpdateStatus};
use crate::uptime::UptimeTracker;
use iced::Task;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Fast, lightweight probe for Heartbeat Keep-Alive (detects disconnections in seconds)
async fn probe_heartbeat_device(ip: Ipv4Addr) -> bool {
    // 1. ICMP Ping probe with 600ms timeout
    let ping_ok = tokio::time::timeout(Duration::from_millis(600), async {
        tokio::process::Command::new("ping")
            .args(["-c", "1", "-W", "1", &ip.to_string()])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await
            .map(|s| s.success())
            .unwrap_or(false)
    })
    .await
    .unwrap_or(false);

    if ping_ok {
        return true;
    }

    // 2. If ping fails (e.g. firewalled devices that drop ICMP but are connected), probe standard TCP ports concurrently
    let ports = [80u16, 443, 53, 8080, 22];
    let futs = ports.into_iter().map(|port| async move {
        tokio::time::timeout(Duration::from_millis(350), tokio::net::TcpStream::connect((ip, port)))
            .await
            .map(|res| res.is_ok())
            .unwrap_or(false)
    });

    iced::futures::future::join_all(futs).await.into_iter().any(|ok| ok)
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.telemetry_tick_counter = self.telemetry_tick_counter.wrapping_add(1);

                // Flush debounced persistence if idle for >= 1.2s
                if self.state_dirty {
                    if let Some(t) = self.last_state_dirty {
                        if t.elapsed() >= Duration::from_millis(1200) {
                            self.save_state();
                        }
                    }
                }

                // Auto-Discovery: if interval is active and time elapsed, re-scan silently in background
                if self.auto_scan_interval_secs > 0
                    && !self.is_scanning
                    && self.last_scan_time.elapsed().as_secs() >= self.auto_scan_interval_secs as u64
                {
                    self.last_scan_time = Instant::now();
                    return Task::done(Message::StartBackgroundScan);
                }

                self.sync_tray_state();

                let mut tick_tasks = Vec::new();

                // 0. Non-blocking async telemetry sampling (skipped when window is closed/minimized to tray)
                if self.active_window_id.is_some() && !self.telemetry_in_flight {
                    self.telemetry_in_flight = true;
                    let iface = self.network.interface.clone();
                    let wifi_iface = self.telemetry.cached_wifi_iface.clone();
                    let query_iw = self.telemetry_tick_counter % 3 == 1;
                    tick_tasks.push(Task::future(async move {
                        let snap = Telemetry::collect_snapshot(iface, wifi_iface, query_iw).await;
                        Message::TelemetryUpdated(snap)
                    }));
                }

                // 1. Periodic Internet stability probe every 5 seconds
                if self.last_uptime_check.elapsed() >= Duration::from_secs(5) {
                    self.last_uptime_check = Instant::now();
                    let gw = self.network.gateway_ip;
                    tick_tasks.push(Task::future(async move {
                        let (gw_ok, internet_ok) = UptimeTracker::probe_connectivity(gw).await;
                        Message::ConnectivityChecked { gw_ok, internet_ok }
                    }));
                }

                // 2. Fast Disconnection Detection (Heartbeat Keep-Alive every 4 seconds)
                if self.notification_settings.enabled
                    && self.notification_settings.fast_disconnection_heartbeat
                    && !self.is_scanning
                    && self.last_heartbeat_time.elapsed() >= Duration::from_secs(4)
                {
                    self.last_heartbeat_time = Instant::now();
                    let targets: Vec<(Ipv4Addr, String, String)> = self.devices
                        .iter()
                        .filter(|d| d.status == DeviceStatus::Online && !d.is_gateway && !d.is_current_host)
                        .map(|d| (d.ip, d.mac.clone(), d.display_name()))
                        .collect();

                    if !targets.is_empty() {
                        tick_tasks.push(Task::future(async move {
                            let sem = Arc::new(tokio::sync::Semaphore::new(16));
                            let mut join_set = tokio::task::JoinSet::new();
                            for (ip, mac, name) in targets {
                                let sem_c = Arc::clone(&sem);
                                join_set.spawn(async move {
                                    let _permit = sem_c.acquire().await;
                                    let is_alive = probe_heartbeat_device(ip).await;
                                    (ip, mac, name, is_alive)
                                });
                            }
                            let mut results = Vec::with_capacity(join_set.len());
                            while let Some(res) = join_set.join_next().await {
                                if let Ok(entry) = res {
                                    results.push(entry);
                                }
                            }
                            Message::HeartbeatResultsReceived(results)
                        }));
                    }
                }

                if tick_tasks.is_empty() {
                    Task::none()
                } else {
                    Task::batch(tick_tasks)
                }
            }

            Message::TelemetryUpdated(snap) => {
                self.telemetry_in_flight = false;
                self.telemetry.apply_snapshot(snap);

                self.download_history
                    .push_back(self.telemetry.download_speed_bps as f32);
                if self.download_history.len() > 30 {
                    self.download_history.pop_front();
                }
                self.download_sparkline_cache.clear();

                self.upload_history
                    .push_back(self.telemetry.upload_speed_bps as f32);
                if self.upload_history.len() > 30 {
                    self.upload_history.pop_front();
                }
                self.upload_sparkline_cache.clear();

                if let Some(stats) = &self.inspector_ping_stats {
                    self.ping_history.push_back(stats.avg_ms);
                    if self.ping_history.len() > 30 {
                        self.ping_history.pop_front();
                    }
                    self.ping_sparkline_cache.clear();
                }

                Task::none()
            }

            Message::OpenSettings => {
                self.is_settings_open = true;
                self.is_about_open = false;
                self.is_update_modal_open = false;
                Task::none()
            }

            Message::CloseSettings => {
                self.is_settings_open = false;
                Task::none()
            }

            Message::OpenAbout => {
                self.is_about_open = true;
                self.is_settings_open = false;
                self.is_update_modal_open = false;
                self.is_speed_test_open = false;
                self.is_timeline_open = false;
                self.is_audit_open = false;
                self.is_wifi_analyzer_open = false;
                self.is_upnp_open = false;
                self.is_storage_explorer_open = false;
                Task::none()
            }

            Message::CloseAbout => {
                self.is_about_open = false;
                Task::none()
            }

            Message::OpenExternalUrl(url) => {
                let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
                let notif_msg = match self.language {
                    Language::Spanish => "Abriendo enlace en el navegador...".to_string(),
                    Language::English => "Opening link in browser...".to_string(),
                    Language::Chinese => "正在浏览器中打开链接...".to_string(),
                    Language::Hindi => "ब्राउज़र में लिंक खोला जा रहा है...".to_string(),
                    Language::Arabic => "جاري فتح الرابط في المتصفح...".to_string(),
                    Language::French => "Ouverture du lien dans le navigateur...".to_string(),
                    Language::Bengali => "ব্রাউজারে লিঙ্ক খোলা হচ্ছে...".to_string(),
                    Language::Portuguese => "Abrindo link no navegador...".to_string(),
                    Language::Russian => "Открытие ссылки в браузере...".to_string(),
                    Language::Urdu => "براؤزر میں لنک کھولا جا رہا ہے...".to_string(),
                };
                self.notification = Some((notif_msg, true));
                Task::none()
            }

            Message::CheckForUpdates => {
                self.update_status = UpdateStatus::Checking;
                Task::future(async {
                    let status = UpdateChecker::check_for_updates().await;
                    Message::UpdateStatusReceived(status)
                })
            }

            Message::UpdateStatusReceived(status) => {
                self.update_status = status;
                Task::none()
            }

            Message::OpenUpdateModal => {
                self.is_update_modal_open = true;
                self.is_about_open = false;
                self.is_settings_open = false;
                self.is_speed_test_open = false;
                self.is_timeline_open = false;
                self.is_audit_open = false;
                self.is_wifi_analyzer_open = false;
                self.is_upnp_open = false;
                self.is_storage_explorer_open = false;
                Task::none()
            }

            Message::CloseUpdateModal => {
                self.is_update_modal_open = false;
                self.update_progress = InAppUpdateProgress::Idle;
                Task::none()
            }

            Message::StartInAppUpdate => {
                self.update_progress = InAppUpdateProgress::AuthorizingAndInstalling;
                Task::future(async {
                    let res = UpdateChecker::apply_update_in_app().await;
                    Message::InAppUpdateResult(res)
                })
            }

            Message::InAppUpdateResult(result) => {
                match result {
                    Ok(msg) => {
                        self.update_progress = InAppUpdateProgress::Success;
                        self.notification = Some((msg, true));
                    }
                    Err(err) => {
                        self.update_progress = InAppUpdateProgress::Failed(err.clone());
                        self.notification = Some((err, false));
                    }
                }
                Task::none()
            }

            Message::OpenTerminalUpdater => {
                if let Err(e) = UpdateChecker::launch_terminal_updater() {
                    self.notification = Some((e, false));
                }
                Task::none()
            }

            Message::RestartNetLens => {
                self.flush_state_sync();
                UpdateChecker::restart_application();
                Task::none()
            }

            Message::LanguageChanged(lang) => {
                self.language = lang;
                self.rebuild_device_caches();
                self.sync_tray_state();
                self.save_state();
                let notif_msg = match lang {
                    Language::Spanish => format!("Idioma cambiado a {}", lang.native_name()),
                    Language::English => format!("Language changed to {}", lang.native_name()),
                    Language::Chinese => format!("语言已切换为 {}", lang.native_name()),
                    Language::Hindi => format!("भाषा बदलकर {} कर दी गई", lang.native_name()),
                    Language::Arabic => format!("تم تغيير اللغة إلى {}", lang.native_name()),
                    Language::French => format!("Langue changée en {}", lang.native_name()),
                    Language::Bengali => format!("ভাষা পরিবর্তন করে {} করা হয়েছে", lang.native_name()),
                    Language::Portuguese => format!("Idioma alterado para {}", lang.native_name()),
                    Language::Russian => format!("Язык изменен на {}", lang.native_name()),
                    Language::Urdu => format!("زبان تبدیل ہو کر {} ہو گئی", lang.native_name()),
                };
                self.notification = Some((notif_msg, true));
                Task::none()
            }

            Message::ThemeChanged(theme) => {
                self.current_theme = theme;
                self.save_state();
                Task::none()
            }

            Message::AutoScanIntervalChanged(secs) => {
                self.auto_scan_interval_secs = secs;
                self.last_scan_time = Instant::now();
                self.save_state();
                Task::none()
            }

            Message::ToggleAutostart(enabled) => {
                self.autostart_enabled = enabled;
                if let Err(e) = AutostartManager::set_enabled(enabled) {
                    self.notification = Some((format!("Error autostart: {}", e), false));
                }
                self.save_state();
                Task::none()
            }

            Message::ToggleMinimizeOnClose(enabled) => {
                self.minimize_on_close = enabled;
                self.save_state();
                Task::none()
            }

            // Fast Disconnection Detection (Heartbeat)
            Message::ToggleHeartbeat(enabled) => {
                self.notification_settings.fast_disconnection_heartbeat = enabled;
                self.save_state();
                Task::none()
            }

            Message::HeartbeatResultsReceived(results) => {
                let mut state_changed = false;

                for (ip, mac, name, is_alive) in results {
                    let mac_key = mac.trim().to_lowercase();
                    if is_alive {
                        self.missed_heartbeats.insert(ip, 0);
                    } else {
                        let misses = self.missed_heartbeats.entry(ip).or_insert(0);
                        *misses += 1;

                        if *misses >= 2 {
                            let was_online = self.active_online_macs.remove(&mac_key);

                            let vendor = self
                                .devices
                                .iter()
                                .find(|d| d.mac.trim().to_lowercase() == mac_key || d.ip == ip)
                                .map(|d| d.vendor.clone())
                                .unwrap_or_default();

                            if let Some(dev) = self.devices.iter_mut().find(|d| d.mac.trim().to_lowercase() == mac_key || d.ip == ip) {
                                if dev.status == DeviceStatus::Online {
                                    dev.status = DeviceStatus::Offline;
                                    dev.last_seen = chrono::Local::now();
                                    state_changed = true;
                                }
                            }

                            if was_online {
                                state_changed = true;
                                if self.initial_scan_completed {
                                    if self.notification_settings.enabled && self.notification_settings.notify_device_disconnected {
                                        let title = Translations::notif_device_disconnected_title(self.language);
                                        let body = self.notification_settings.format_device_message(
                                            "Desconectado",
                                            &name,
                                            ip,
                                            &mac,
                                            &vendor,
                                            Some("El dispositivo no respondió a los sondeos Heartbeat"),
                                        );
                                        self.notification_settings.dispatch(title, &body, "network-offline", None);
                                    }

                                    let event = HistoryManager::create_event(
                                        NetworkEventType::DeviceDisconnected,
                                        "Dispositivo Desconectado (Heartbeat)".to_string(),
                                        format!("{} ({}) ya no responde en la red local.", name, ip),
                                        Some(ip),
                                        Some(mac.clone()),
                                    );
                                    self.network_events.insert(0, event);
                                }
                            }
                        }
                    }
                }

                if self.network_events.len() > 150 {
                    self.network_events.truncate(150);
                }

                if state_changed {
                    self.rebuild_device_caches();
                    self.sync_tray_state();
                    self.save_state();
                }

                Task::none()
            }

            // Real-Time Monitoring (Kernel Netlink)
            Message::ToggleRealtimeMonitor(enabled) => {
                self.notification_settings.realtime_netlink_monitor = enabled;
                self.save_state();
                Task::none()
            }

            Message::RealtimeNeighborEvent(action) => {
                if !self.notification_settings.realtime_netlink_monitor {
                    return Task::none();
                }

                match action {
                    NeighborAction::Appeared { ip, mac, iface } => {
                        let mac_key = mac.trim().to_lowercase();
                        if self.network.local_ip == Some(ip) {
                            return Task::none();
                        }

                        // Debounce: prevent processing multiple transitions of the same device within 4 seconds
                        if let Some(prev) = self.last_realtime_events.get(&mac_key) {
                            if prev.elapsed() < Duration::from_secs(4) {
                                return Task::none();
                            }
                        }
                        self.last_realtime_events.insert(mac_key.clone(), Instant::now());

                        let existing_idx = self.devices.iter().position(|d| d.mac.trim().to_lowercase() == mac_key || d.ip == ip);
                        let is_already_online = existing_idx.map(|idx| self.devices[idx].status == DeviceStatus::Online).unwrap_or(false);
                        let is_in_active_macs = self.active_online_macs.contains(&mac_key);

                        if is_already_online && is_in_active_macs {
                            if let Some(idx) = existing_idx {
                                self.devices[idx].last_seen = chrono::Local::now();
                            }
                            self.missed_heartbeats.insert(ip, 0);
                            return Task::none();
                        }

                        // Device just appeared or reconnected: quickly enrich
                        let oui_db = self.oui_db.clone();
                        Task::future(async move {
                            let vendor = Scanner::lookup_vendor(oui_db.as_deref(), &mac);
                            let discovery = crate::discovery::Discovery::probe(ip).await;
                            Message::RealtimeDeviceEnriched {
                                ip,
                                mac,
                                iface,
                                vendor,
                                hostname: discovery.hostname,
                                model_name: discovery.model_or_friendly_name,
                                services: discovery.services,
                            }
                        })
                    }
                    NeighborAction::Disappeared { ip, mac, .. } => {
                        let target_idx = self.devices.iter().position(|d| {
                            if let Some(m) = &mac {
                                d.mac.trim().to_lowercase() == m.trim().to_lowercase() || d.ip == ip
                            } else {
                                d.ip == ip
                            }
                        });

                        if let Some(idx) = target_idx {
                            if self.devices[idx].status == DeviceStatus::Online {
                                self.devices[idx].status = DeviceStatus::Offline;
                                let dev_mac = self.devices[idx].mac.trim().to_lowercase();
                                self.active_online_macs.remove(&dev_mac);

                                if self.initial_scan_completed && !self.devices[idx].is_gateway && !self.devices[idx].is_current_host {
                                    if self.notification_settings.enabled && self.notification_settings.notify_device_disconnected {
                                        let title = Translations::notif_device_disconnected_title(self.language);
                                        let body = self.notification_settings.format_device_message(
                                            "Desconectado",
                                            &self.devices[idx].display_name(),
                                            self.devices[idx].ip,
                                            &self.devices[idx].mac,
                                            &self.devices[idx].vendor,
                                            Some("El kernel Linux detectó la desconexión del equipo"),
                                        );
                                        self.notification_settings.dispatch(title, &body, "network-offline", None);
                                    }

                                    let event = HistoryManager::create_event(
                                        NetworkEventType::DeviceDisconnected,
                                        "Dispositivo Desconectado (Tiempo Real)".to_string(),
                                        format!("{} ({}) ya no responde en la red local.", self.devices[idx].display_name(), self.devices[idx].ip),
                                        Some(self.devices[idx].ip),
                                        Some(self.devices[idx].mac.clone()),
                                    );
                                    self.network_events.insert(0, event);
                                    if self.network_events.len() > 150 {
                                        self.network_events.truncate(150);
                                    }
                                }
                                self.sync_tray_state();
                            }
                        }
                        Task::none()
                    }
                }
            }

            Message::RealtimeDeviceEnriched { ip, mac, iface, vendor, hostname, model_name, services } => {
                let mac_key = mac.trim().to_lowercase();
                let is_new_device = !self.known_macs.contains(&mac_key);
                let is_gateway = self.network.gateway_ip == Some(ip);
                let is_current = self.network.local_ip == Some(ip);

                let now = chrono::Local::now();
                let is_randomized = Scanner::is_randomized_mac(&mac);

                let existing_idx = self.devices.iter().position(|d| d.mac.trim().to_lowercase() == mac_key || d.ip == ip);
                let (custom_alias, is_whitelisted, is_blocked, speed_limit_kbps, prev_category) = if let Some(meta) = self.device_metadata.get(&mac_key) {
                    (meta.custom_alias.clone(), meta.is_whitelisted, meta.is_blocked, meta.speed_limit_kbps, meta.category.unwrap_or(DeviceCategory::Unknown))
                } else if let Some(idx) = existing_idx {
                    (self.devices[idx].custom_alias.clone(), self.devices[idx].is_whitelisted, self.devices[idx].is_blocked, self.devices[idx].speed_limit_kbps, self.devices[idx].category)
                } else {
                    (None, false, false, None, DeviceCategory::Unknown)
                };

                let category = if prev_category != DeviceCategory::Unknown {
                    prev_category
                } else {
                    Scanner::categorize_device(is_gateway, is_current, hostname.as_deref(), &vendor, &[])
                };

                let dev = Device {
                    ip,
                    mac: mac.clone(),
                    vendor: vendor.clone(),
                    hostname: hostname.clone(),
                    category,
                    is_randomized_mac: is_randomized,
                    is_gateway,
                    is_current_host: is_current,
                    interface: iface.clone(),
                    latency_ms: Some(1.0),
                    open_ports: vec![],
                    status: DeviceStatus::Online,
                    first_seen: now,
                    last_seen: now,
                    custom_alias,
                    is_whitelisted,
                    is_blocked,
                    speed_limit_kbps,
                    connection_type: None,
                    wifi_signal_dbm: None,
                    negotiated_mbps: None,
                    model_name,
                    discovered_services: services,
                    request_history: vec![],
                };

                self.active_online_macs.insert(mac_key.clone());
                self.known_macs.insert(mac_key.clone());
                self.missed_heartbeats.insert(ip, 0);

                let mut notif_task = Task::none();

                if is_new_device {
                    let event = HistoryManager::create_event(
                        NetworkEventType::NewDeviceDetected,
                        "Nuevo Dispositivo (Tiempo Real)".to_string(),
                        format!("{} ({}) detectado instantáneamente.", dev.display_name(), dev.ip),
                        Some(dev.ip),
                        Some(dev.mac.clone()),
                    );
                    self.network_events.insert(0, event);

                    if !dev.is_current_host && !dev.is_gateway && !dev.is_whitelisted {
                        if self.alerted_macs.insert(mac_key.clone()) {
                            self.pending_intruder_alert = Some(IntruderAlert {
                                ip: dev.ip,
                                mac: dev.mac.clone(),
                                vendor: dev.vendor.clone(),
                                hostname: dev.hostname.clone(),
                            });

                            if self.notification_settings.enabled && self.notification_settings.notify_unknown_intruders {
                                let dev_ip = dev.ip;
                                let dev_mac = dev.mac.clone();
                                let alert_title = Translations::intruder_alert_title(self.language);
                                let trust_label = Translations::intruder_btn_trust(self.language);
                                let block_label = Translations::intruder_btn_block(self.language);
                                let body = self.notification_settings.format_device_message(
                                    "Nuevo Dispositivo Desconocido",
                                    &dev.display_name(),
                                    dev.ip,
                                    &dev.mac,
                                    &dev.vendor,
                                    Some("Detectado instantáneamente por el kernel"),
                                );

                                let notif_settings = self.notification_settings.clone();
                                notif_task = Task::future(async move {
                                    let action = notif_settings.dispatch_interactive_alert(&alert_title, &body, trust_label, block_label).await;
                                    Message::DesktopNotificationAction {
                                        ip: dev_ip,
                                        mac: dev_mac,
                                        action,
                                    }
                                });
                            }
                        }
                    }
                } else {
                    // Known device reconnecting in real time
                    if !dev.is_current_host && !dev.is_gateway && !dev.is_whitelisted {
                        // *** INSTANT ALERT: NON-WHITELIST RECONNECTION ***
                        let event = HistoryManager::create_event(
                            NetworkEventType::SecurityAlert,
                            "Reconexión No Autorizada (Tiempo Real)".to_string(),
                            format!("{} ({}) no está en la Lista Blanca y se ha reconectado.", dev.display_name(), dev.ip),
                            Some(dev.ip),
                            Some(dev.mac.clone()),
                        );
                        self.network_events.insert(0, event);

                        if self.notification_settings.enabled && self.notification_settings.notify_known_unwhitelisted_reconnect {
                            if self.pending_intruder_alert.is_none() {
                                self.pending_intruder_alert = Some(IntruderAlert {
                                    ip: dev.ip,
                                    mac: dev.mac.clone(),
                                    vendor: dev.vendor.clone(),
                                    hostname: dev.hostname.clone(),
                                });
                            }

                            let dev_ip = dev.ip;
                            let dev_mac = dev.mac.clone();
                            let alert_title = Translations::notif_reconnect_unwhitelisted_title(self.language);
                            let trust_label = Translations::intruder_btn_trust(self.language);
                            let block_label = Translations::intruder_btn_block(self.language);
                            let body = self.notification_settings.format_device_message(
                                "Reconexión No Autorizada",
                                &dev.display_name(),
                                dev.ip,
                                &dev.mac,
                                &dev.vendor,
                                Some("Detección instantánea en tiempo real"),
                            );

                            let notif_settings = self.notification_settings.clone();
                            notif_task = Task::future(async move {
                                let action = notif_settings.dispatch_interactive_alert(alert_title, &body, trust_label, block_label).await;
                                Message::DesktopNotificationAction {
                                    ip: dev_ip,
                                    mac: dev_mac,
                                    action,
                                }
                            });
                        }
                    } else if dev.is_whitelisted {
                        let event = HistoryManager::create_event(
                            NetworkEventType::DeviceConnected,
                            "Dispositivo Conectado (Tiempo Real)".to_string(),
                            format!("{} ({}) conectado a la red.", dev.display_name(), dev.ip),
                            Some(dev.ip),
                            Some(dev.mac.clone()),
                        );
                        self.network_events.insert(0, event);

                        if self.notification_settings.enabled && self.notification_settings.notify_whitelisted_reconnect {
                            let title = Translations::notif_reconnect_whitelisted_title(self.language);
                            let body = self.notification_settings.format_device_message(
                                "Conectado",
                                &dev.display_name(),
                                dev.ip,
                                &dev.mac,
                                &dev.vendor,
                                Some("Dispositivo de confianza en tiempo real"),
                            );
                            self.notification_settings.dispatch(title, &body, "network-wireless", None);
                        }
                    }
                }

                if let Some(idx) = existing_idx {
                    let prev_req = self.devices[idx].request_history.clone();
                    self.devices[idx] = dev;
                    if !prev_req.is_empty() {
                        self.devices[idx].request_history = prev_req;
                    }
                } else {
                    self.devices.push(dev);
                }

                if self.network_events.len() > 150 {
                    self.network_events.truncate(150);
                }
                self.rebuild_device_caches();
                self.sync_tray_state();
                notif_task
            }

            // Desktop Notification Management & Settings
            Message::ToggleNotificationsMaster(enabled) => {
                self.notification_settings.enabled = enabled;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotifyUnknown(enabled) => {
                self.notification_settings.notify_unknown_intruders = enabled;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotifyKnownUnwhitelisted(enabled) => {
                self.notification_settings.notify_known_unwhitelisted_reconnect = enabled;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotifyWhitelisted(enabled) => {
                self.notification_settings.notify_whitelisted_reconnect = enabled;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotifyDisconnected(enabled) => {
                self.notification_settings.notify_device_disconnected = enabled;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotifyOutages(enabled) => {
                self.notification_settings.notify_internet_outages = enabled;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotifySecurity(enabled) => {
                self.notification_settings.notify_security_critical = enabled;
                self.save_state();
                Task::none()
            }

            Message::ChangeNotificationUrgency(urgency) => {
                self.notification_settings.urgency = urgency;
                self.save_state();
                Task::none()
            }

            Message::ChangeNotificationDetail(detail) => {
                self.notification_settings.detail_level = detail;
                self.save_state();
                Task::none()
            }

            Message::ChangeNotificationTimeout(secs) => {
                self.notification_settings.timeout_secs = secs;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotificationShowMac(enabled) => {
                self.notification_settings.show_mac = enabled;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotificationShowVendor(enabled) => {
                self.notification_settings.show_vendor = enabled;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotificationPlaySound(enabled) => {
                self.notification_settings.play_sound = enabled;
                self.save_state();
                Task::none()
            }

            Message::NotificationPrefixChanged(prefix) => {
                self.notification_prefix_input = prefix.clone();
                self.notification_settings.custom_prefix = prefix;
                self.mark_state_dirty();
                Task::none()
            }

            Message::SendTestNotification => {
                let title = "Notificación de Prueba";
                let body = self.notification_settings.format_device_message(
                    "Alerta de Prueba",
                    "Equipo-NetLens",
                    Ipv4Addr::new(192, 168, 1, 99),
                    "AA:BB:CC:DD:EE:FF",
                    "NetLens Technology",
                    Some("Verificación de configuración y estilo de notificación"),
                );
                self.notification_settings.dispatch(title, &body, "network-wireless", None);
                self.notification = Some(("Notificación de prueba enviada al escritorio.".to_string(), true));
                Task::none()
            }

            Message::WindowCloseRequested(id) => {
                self.flush_state_sync();
                if self.minimize_on_close {
                    // Close and destroy window completely so it disappears from taskbar,
                    // leaving only the system tray icon.
                    self.active_window_id = None;
                    crate::trim_memory();
                    iced::window::close(id)
                } else {
                    std::process::exit(0);
                }
            }

            Message::WindowClosed(id) => {
                if self.active_window_id == Some(id) {
                    self.active_window_id = None;
                }
                crate::trim_memory();
                Task::none()
            }

            Message::RestoreWindowFromTray => {
                if let Some(id) = self.active_window_id {
                    Task::batch(vec![
                        iced::window::minimize(id, false),
                        iced::window::gain_focus(id),
                    ])
                } else {
                    // If window was closed when minimizing to background, reopen cleanly
                    let (id, open_task) = iced::window::open(default_window_settings());
                    self.active_window_id = Some(id);
                    Task::batch(vec![
                        open_task.discard(),
                        iced::window::gain_focus(id),
                    ])
                }
            }

            Message::ExitApp => {
                self.flush_state_sync();
                std::process::exit(0);
            }

            Message::DesktopNotificationAction { ip, mac, action } => {
                if action == "trust" {
                    let mac_clone = mac.clone();
                    let title = Translations::notif_trust_success_title(self.language);
                    let body = Translations::notif_trust_success_body(self.language, &mac_clone);
                    self.notification_settings.dispatch(title, &body, "network-wireless", Some(NotificationUrgency::Normal));
                    Task::done(Message::TrustDevice(mac))
                } else if action == "block" {
                    if self.router_password_input.trim().is_empty() {
                        let title = Translations::notif_require_pass_title(self.language);
                        let body = Translations::notif_require_pass_body(self.language);
                        self.notification_settings.dispatch(title, body, "dialog-warning", Some(NotificationUrgency::Critical));
                        Task::none()
                    } else {
                        let mac_clone = mac.clone();
                        let title = Translations::notif_block_success_title(self.language);
                        let body = Translations::notif_block_success_body(self.language, &mac_clone);
                        self.notification_settings.dispatch(title, &body, "network-wireless", Some(NotificationUrgency::Normal));
                        Task::done(Message::ToggleDeviceBlocked(ip, mac))
                    }
                } else {
                    Task::none()
                }
            }


            // Diagnostics, Router, Speed Test, Security Audit, UPnP & Wi-Fi Analyzer
            Message::OpenSpeedTest
            | Message::CloseSpeedTest
            | Message::StartSpeedTest
            | Message::SpeedTestEventReceived(_)
            | Message::ClearSpeedTestHistory
            | Message::SecurityFilterChanged(_)
            | Message::ToggleDeviceWhitelist(_, _)
            | Message::ToggleDeviceBlocked(_, _)
            | Message::RouterOperationFinished { .. }
            | Message::RouterIpChanged(_)
            | Message::RouterPasswordChanged(_)
            | Message::TestRouterConnection
            | Message::RouterTestFinished(_)
            | Message::TrustDevice(_)
            | Message::DismissIntruderAlert
            | Message::FetchRouterTopology
            | Message::RouterTopologyReceived(_)
            | Message::StartTraceroute(_)
            | Message::TracerouteFinished(_)
            | Message::ChangeDeviceCategory(_, _)
            | Message::OpenTimeline
            | Message::CloseTimeline
            | Message::ClearTimeline
            | Message::OpenSecurityAudit
            | Message::CloseSecurityAudit
            | Message::RunSecurityAudit
            | Message::OpenUpnpModal
            | Message::CloseUpnpModal
            | Message::QueryUpnpRouter
            | Message::UpnpRouterResult(_)
            | Message::OpenWifiAnalyzer
            | Message::CloseWifiAnalyzer
            | Message::ScanWifi
            | Message::WifiScanCompleted(_)
            | Message::ConnectivityChecked { .. }
            | Message::ClearDeviceRequestHistory(_) => self.update_diagnostics(message),

            // Router USB Storage / NAS Explorer
            Message::OpenStorageExplorer
            | Message::CloseStorageExplorer
            | Message::RefreshStorageShares
            | Message::StorageTargetIpChanged(_)
            | Message::StorageCredentialsUserChanged(_)
            | Message::StorageCredentialsPassChanged(_)
            | Message::StorageCredentialsWorkgroupChanged(_)
            | Message::ToggleStorageCredentialsModal
            | Message::StorageSaveCredentials
            | Message::StorageShareSelected(_)
            | Message::StorageNavigateTo(_)
            | Message::StorageNavigateUp
            | Message::StorageRefreshDirectory
            | Message::StorageFilterQueryChanged(_)
            | Message::StorageStartCreateFolder
            | Message::StorageCreateFolderNameChanged(_)
            | Message::StorageCancelCreateFolder
            | Message::StorageConfirmCreateFolder
            | Message::StorageRequestDeleteFile { .. }
            | Message::StorageConfirmDeleteFile
            | Message::StorageCancelDeleteFile
            | Message::StorageOpenFile(_)
            | Message::StorageDownloadFile(_)
            | Message::StoragePickFileAndUpload
            | Message::StoragePickFolderAndUpload
            | Message::StorageFilePicked(_)
            | Message::StorageFolderPicked(_)
            | Message::StorageFileHovered(_)
            | Message::StorageFileDropped(_)
            | Message::StorageFilesHoveredLeft
            | Message::StorageShowFileProperties(_)
            | Message::StorageCloseProperties
            | Message::StorageToggleDemoMode
            | Message::StorageUploadEventReceived(_)
            | Message::StorageDismissUploadProgress
            | Message::StorageSharesLoaded(_)
            | Message::StorageDirectoryLoaded(_)
            | Message::StorageOperationCompleted(_) => self.update_storage(message),

            // Network Scan, Device Selection, Ping/Port Probes, Alias, Clipboard & Export
            Message::AddDeviceSampleRequest(_)
            | Message::RefreshNetwork
            | Message::StartScan
            | Message::StartBackgroundScan
            | Message::ScanEventReceived(_)
            | Message::SearchChanged(_)
            | Message::CategorySelected(_)
            | Message::SortChanged(_)
            | Message::SelectDevice(_)
            | Message::CloseInspector
            | Message::StartPingTest(_)
            | Message::PingTestFinished(_)
            | Message::StartDeepScan(_)
            | Message::DeepScanFinished(_)
            | Message::SendWol(_)
            | Message::WolFinished(_)
            | Message::OpenInBrowser(_)
            | Message::AliasInputChanged(_)
            | Message::SaveAlias(_)
            | Message::CopyToClipboard(_, _)
            | Message::ExportJson
            | Message::ExportJsonSelected(_)
            | Message::ExportCsv
            | Message::ExportCsvSelected(_)
            | Message::DismissNotification => self.update_network(message),
        }
    }
}
