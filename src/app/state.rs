use super::*;
use crate::autostart::AutostartManager;
use crate::i18n::Translations;
use crate::model::{Device, DeviceCategory};
use crate::oui::Oui;
use crate::realtime_monitor::RealtimeMonitor;
use crate::router::RouterStorageManager;
use crate::scanner::Scanner;
use crate::speedtest::SpeedTestStage;
use crate::storage::{AppStateRef, Storage};
use crate::telemetry::Telemetry;
use crate::theme::CustomStyles;
use crate::tray::{spawn_tray, TrayAction, TrayState};
use crate::updater::{InAppUpdateProgress, UpdateChecker, UpdateStatus};
use crate::uptime::UptimeTracker;
use iced::futures::StreamExt;
use iced::widget::canvas::Cache;
use iced::widget::{
    container, row, text, tooltip,
};
use iced::{Alignment, Element, Padding, Subscription, Task};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let network = Scanner::detect_network_context();
        let oui_db = Oui::default().ok().map(Arc::new);
        let telemetry = Telemetry::new(&network.interface);

        // Load persisted configuration state (language, theme, devices, metadata)
        let saved = Storage::load();
        let current_theme = theme_from_string(&saved.theme);
        let language = saved.language;
        let mut devices = saved.devices;
        let device_metadata = saved.device_metadata;
        Storage::apply_metadata(&mut devices, &device_metadata);

        let speed_test_result = saved.last_speed_test;
        let (speed_test_stage, speed_test_ping, speed_test_download, speed_test_upload) =
            if let Some(res) = &speed_test_result {
                (
                    SpeedTestStage::Completed,
                    Some(res.ping_ms),
                    Some(res.download_mbps),
                    Some(res.upload_mbps),
                )
            } else {
                (SpeedTestStage::Idle, None, None, None)
            };

        let router_ip_input = saved
            .router_ip
            .unwrap_or_else(|| {
                network
                    .gateway_ip
                    .map(|g| g.to_string())
                    .unwrap_or_else(|| "192.168.1.1".to_string())
            });
        let router_password_input = saved.router_password.unwrap_or_default();
        let speed_test_history = saved.speed_test_history;
        let auto_scan_interval_secs = saved.auto_scan_interval_secs;
        let autostart_enabled = saved.autostart;
        if autostart_enabled {
            AutostartManager::sync_if_needed(true);
        } else {
            AutostartManager::sync_if_needed(false);
        }
        let minimize_on_close = saved.minimize_to_tray_on_close;
        let notification_settings = saved.notification_settings;
        let notification_prefix_input = notification_settings.custom_prefix.clone();
        let mut known_macs: HashSet<String> = saved
            .known_macs
            .into_iter()
            .map(|m| m.trim().to_lowercase())
            .collect();
        for dev in &devices {
            known_macs.insert(dev.mac.trim().to_lowercase());
        }

        let status = if let Some(gw) = network.gateway_ip {
            AppStatus::ConnectedTo {
                gw: gw.to_string(),
                iface: network.interface.clone(),
                subnet: network.subnet_cidr.clone(),
            }
        } else {
            AppStatus::Ready
        };

        let initial_device_count = devices.len();
        let initial_gateway_ip = network.gateway_ip.map(|g| g.to_string());
        let storage_target_ip = initial_gateway_ip.clone().unwrap_or_else(|| "192.168.1.1".to_string());

        let logo_handle = iced::widget::image::Handle::from_bytes(
            include_bytes!("../../assets/netlens.png").as_slice(),
        );

        let (persist_tx, persist_rx) = std::sync::mpsc::channel::<String>();
        let _ = std::thread::Builder::new()
            .name("netlens-persist".to_string())
            .spawn(move || {
                while let Ok(mut latest_json) = persist_rx.recv() {
                    // Coalesce bursts so only the latest serialized state is written to disk
                    while let Ok(newer) = persist_rx.try_recv() {
                        latest_json = newer;
                    }
                    if let Err(e) = Storage::write_json_atomic(&latest_json) {
                        eprintln!("Aviso: No se pudo guardar estado persistente: {}", e);
                    }
                }
            });

        let mut app = Self {
            network,
            devices,
            oui_db,
            telemetry,
            is_scanning: false,
            is_background_scan: false,
            scan_progress: 0.0,
            status,
            search_query: String::new(),
            selected_category: None,
            sort_by: SortOption::Ip,
            selected_device_ip: None,
            inspector_ping_stats: None,
            is_pinging: false,
            is_deep_scanning: false,
            deep_ports: Vec::new(),
            notification: None,
            alias_input: String::new(),
            current_theme,
            language,
            logo_handle,
            is_settings_open: false,
            is_about_open: false,
            update_status: UpdateStatus::Idle,
            update_progress: InAppUpdateProgress::Idle,
            is_update_modal_open: false,
            is_speed_test_open: false,
            speed_test_stage,
            speed_test_ping,
            speed_test_download,
            speed_test_upload,
            speed_test_result,
            speed_test_history,
            device_metadata,
            security_filter: SecurityFilter::All,
            router_ip_input,
            router_password_input,
            router_status_msg: None,
            is_testing_router: false,
            is_operating_device: None,
            download_history: VecDeque::with_capacity(30),
            upload_history: VecDeque::with_capacity(30),
            ping_history: VecDeque::with_capacity(30),
            download_sparkline_cache: Cache::new(),
            upload_sparkline_cache: Cache::new(),
            ping_sparkline_cache: Cache::new(),
            auto_scan_interval_secs,
            last_scan_time: Instant::now(),
            last_uptime_check: Instant::now() - Duration::from_secs(5),
            telemetry_in_flight: false,
            telemetry_tick_counter: 0,
            state_dirty: false,
            last_state_dirty: None,
            persist_tx,
            last_tray_snapshot: (initial_device_count, false, initial_gateway_ip.clone(), language),
            cached_filtered_indices: Vec::new(),
            cached_visible_categories: Vec::new(),
            cached_whitelist_count: 0,
            cached_blocked_count: 0,
            known_macs,
            alerted_macs: HashSet::new(),
            pending_intruder_alert: None,
            router_connections: HashMap::new(),
            is_tracing_route: false,
            traceroute_hops: None,
            traceroute_error: None,
            autostart_enabled,
            minimize_on_close,
            active_window_id: None,
            tray_state: Arc::new(Mutex::new(TrayState {
                device_count: initial_device_count,
                is_scanning: false,
                gateway_ip: initial_gateway_ip,
                language,
            })),
            tray_handle: Arc::new(Mutex::new(None)),
            network_events: saved.network_events,
            is_timeline_open: false,
            security_audit_report: None,
            is_audit_open: false,
            upnp_status: None,
            is_upnp_open: false,
            uptime_tracker: UptimeTracker::default(),
            wifi_analyzer_report: None,
            is_wifi_analyzer_open: false,
            is_scanning_wifi: false,
            notification_settings,
            notification_prefix_input,
            scanned_macs_current_cycle: HashSet::new(),
            active_online_macs: HashSet::new(),
            initial_scan_completed: false,
            last_realtime_events: HashMap::new(),
            missed_heartbeats: HashMap::new(),
            last_heartbeat_time: Instant::now(),
            is_storage_explorer_open: false,
            is_storage_creds_open: false,
            is_storage_loading: false,
            storage_target_ip,
            storage_credentials: saved.storage_credentials.unwrap_or_default(),
            storage_shares: Vec::new(),
            storage_active_share: None,
            storage_current_path: String::new(),
            storage_files: Vec::new(),
            storage_disk_info: None,
            storage_filter_query: String::new(),
            storage_error_msg: None,
            storage_status_msg: None,
            storage_selected_file: None,
            is_creating_folder: false,
            new_folder_name_input: String::new(),
            file_to_delete: None,
            is_storage_demo_mode: false,
            storage_upload_progress: None,
            storage_last_freed: None,
            demo_storage_files: RouterStorageManager::get_initial_demo_files(),
            demo_storage_disk: RouterStorageManager::get_initial_demo_disk(),
            is_storage_drag_hovered: false,
            hovered_drop_name: None,
            router_has_storage: false,
        };

        // Build initial cached device filter/sort/category structures
        app.rebuild_device_caches();

        // Start System Tray StatusNotifierItem service
        spawn_tray(Arc::clone(&app.tray_state), Arc::clone(&app.tray_handle));

        let start_minimized = std::env::args().any(|a| a == "--minimized" || a == "--background");
        let initial_task = if start_minimized {
            Task::done(Message::StartBackgroundScan)
        } else {
            let (id, open_task) = iced::window::open(default_window_settings());
            app.active_window_id = Some(id);
            Task::batch(vec![
                open_task.discard(),
                Task::done(Message::StartBackgroundScan),
            ])
        };

        let update_task = Task::future(async {
            let status = UpdateChecker::check_for_updates().await;
            Message::UpdateStatusReceived(status)
        });

        (app, Task::batch(vec![initial_task, update_task]))
    }

    /// Entry point for iced::daemon infrastructure
    pub fn boot() -> (Self, Task<Message>) {
        Self::new()
    }

    /// Synchronizes current scan state and devices with System Tray (only when changed)
    pub fn sync_tray_state(&mut self) {
        let gw_str = self.network.gateway_ip.map(|g| g.to_string());
        let snapshot = (self.devices.len(), self.is_scanning, gw_str, self.language);
        if snapshot == self.last_tray_snapshot {
            return;
        }
        self.last_tray_snapshot = snapshot.clone();

        if let Ok(mut s) = self.tray_state.lock() {
            s.device_count = snapshot.0;
            s.is_scanning = snapshot.1;
            s.gateway_ip = snapshot.2;
            s.language = snapshot.3;
        }
        if let Ok(lock) = self.tray_handle.lock() {
            if let Some(h) = lock.as_ref() {
                h.refresh();
            }
        }
    }

    pub(super) fn serialize_current_state(&self) -> Result<String, String> {
        let theme_str = theme_to_string(&self.current_theme);
        let router_pw = if self.router_password_input.trim().is_empty() {
            None
        } else {
            Some(self.router_password_input.as_str())
        };
        let known_macs: Vec<&String> = self.known_macs.iter().collect();

        let state_ref = AppStateRef {
            language: self.language,
            theme: theme_str,
            devices: &self.devices,
            device_metadata: &self.device_metadata,
            last_speed_test: self.speed_test_result.as_ref(),
            speed_test_history: &self.speed_test_history,
            router_ip: Some(self.router_ip_input.as_str()),
            router_password: router_pw,
            auto_scan_interval_secs: self.auto_scan_interval_secs,
            known_macs,
            autostart: self.autostart_enabled,
            minimize_to_tray_on_close: self.minimize_on_close,
            network_events: &self.network_events,
            notification_settings: &self.notification_settings,
            storage_credentials: Some(&self.storage_credentials),
        };
        Storage::serialize_ref(&state_ref)
    }

    /// Queues an asynchronous, zero-clone state save on the background persistence thread
    pub fn save_state(&mut self) {
        self.state_dirty = false;
        self.last_state_dirty = None;
        match self.serialize_current_state() {
            Ok(json) => {
                if self.persist_tx.send(json).is_err() {
                    // Fallback if writer thread stopped
                    if let Err(e) = self.save_state_sync_inner() {
                        eprintln!("Aviso: No se pudo guardar estado persistente: {}", e);
                    }
                }
            }
            Err(e) => eprintln!("Aviso: No se pudo serializar estado persistente: {}", e),
        }
    }

    pub(super) fn save_state_sync_inner(&self) -> Result<(), String> {
        let json = self.serialize_current_state()?;
        Storage::write_json_atomic(&json)
    }

    /// Marks state dirty for debounced persistence (used on keystroke inputs)
    pub fn mark_state_dirty(&mut self) {
        self.state_dirty = true;
        self.last_state_dirty = Some(Instant::now());
    }

    /// Flushes pending state synchronously (for clean process exit)
    pub fn flush_state_sync(&mut self) {
        if self.state_dirty {
            self.state_dirty = false;
            self.last_state_dirty = None;
            if let Err(e) = self.save_state_sync_inner() {
                eprintln!("Aviso: No se pudo guardar estado persistente al salir: {}", e);
            }
        }
    }

    /// Recomputes filtered/sorted device indices, category pills, and security counts once per state change
    pub fn rebuild_device_caches(&mut self) {
        // 1. Single-pass metadata counts & visible categories
        self.cached_visible_categories.clear();
        self.cached_whitelist_count = 0;
        self.cached_blocked_count = 0;

        for d in &self.devices {
            if d.is_whitelisted {
                self.cached_whitelist_count += 1;
            }
            if d.is_blocked {
                self.cached_blocked_count += 1;
            }
            if d.category != DeviceCategory::Unknown && !self.cached_visible_categories.contains(&d.category) {
                self.cached_visible_categories.push(d.category);
            }
        }
        if let Some(sel) = self.selected_category {
            if sel != DeviceCategory::Unknown && !self.cached_visible_categories.contains(&sel) {
                self.cached_visible_categories.push(sel);
            }
        }
        self.cached_visible_categories.sort_by_key(|c| {
            DeviceCategory::all().iter().position(|x| x == c).unwrap_or(99)
        });

        // 2. Filtered & sorted indices
        let q = self.search_query.trim().to_lowercase();
        let mut indices: Vec<usize> = self
            .devices
            .iter()
            .enumerate()
            .filter(|(_, d)| {
                match self.security_filter {
                    SecurityFilter::All => {}
                    SecurityFilter::Whitelist => {
                        if !d.is_whitelisted {
                            return false;
                        }
                    }
                    SecurityFilter::Blocked => {
                        if !d.is_blocked {
                            return false;
                        }
                    }
                }

                if let Some(cat) = self.selected_category {
                    if d.category != cat {
                        return false;
                    }
                }

                if !q.is_empty() {
                    let matches_ip = d.ip.to_string().contains(&q);
                    let matches_mac = d.mac.to_lowercase().contains(&q);
                    let matches_vendor = d.vendor.to_lowercase().contains(&q);
                    let matches_host = d
                        .hostname
                        .as_deref()
                        .map(|h| h.to_lowercase().contains(&q))
                        .unwrap_or(false);
                    let matches_alias = d
                        .custom_alias
                        .as_deref()
                        .map(|a| a.to_lowercase().contains(&q))
                        .unwrap_or(false);

                    if !matches_ip && !matches_mac && !matches_vendor && !matches_host && !matches_alias {
                        return false;
                    }
                }

                true
            })
            .map(|(i, _)| i)
            .collect();

        let devices = &self.devices;
        let lang = self.language;
        match self.sort_by {
            SortOption::Ip => {
                indices.sort_by_key(|&i| devices[i].ip);
            }
            SortOption::Latency => {
                indices.sort_by(|&a, &b| {
                    let lat_a = devices[a].latency_ms.unwrap_or(f32::MAX);
                    let lat_b = devices[b].latency_ms.unwrap_or(f32::MAX);
                    lat_a.partial_cmp(&lat_b).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            SortOption::Name => {
                indices.sort_by_cached_key(|&i| devices[i].display_name().to_lowercase());
            }
            SortOption::Vendor => {
                indices.sort_by_cached_key(|&i| devices[i].vendor.to_lowercase());
            }
            SortOption::Category => {
                indices.sort_by_key(|&i| Translations::category_name(devices[i].category, lang));
            }
        }

        self.cached_filtered_indices = indices;
    }

    /// Periodic timer (1s when window visible, 5s when minimized to tray), window events, and System Tray
    pub fn subscription(&self) -> Subscription<Message> {
        let tick_duration = if self.active_window_id.is_some() {
            Duration::from_millis(1000)
        } else {
            Duration::from_millis(5000)
        };
        let tick_sub = iced::time::every(tick_duration).map(|_| Message::Tick);

        let close_req_sub = iced::window::close_requests().map(Message::WindowCloseRequested);
        let close_evt_sub = iced::window::close_events().map(Message::WindowClosed);

        let tray_sub = Subscription::run(|| {
            let rx = crate::tray::get_tray_sender().subscribe();
            iced::futures::stream::unfold(rx, |mut rx| async move {
                loop {
                    match rx.recv().await {
                        Ok(TrayAction::RestoreWindow) => {
                            return Some((Message::RestoreWindowFromTray, rx));
                        }
                        Ok(TrayAction::TriggerScan) => {
                            return Some((Message::StartScan, rx));
                        }
                        Ok(TrayAction::Exit) => {
                            return Some((Message::ExitApp, rx));
                        }
                        Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                        Err(tokio::sync::broadcast::error::RecvError::Closed) => return None,
                    }
                }
            })
        });

        let realtime_sub = if self.notification_settings.realtime_netlink_monitor {
            Subscription::run(|| {
                let (tx, rx) = iced::futures::channel::mpsc::unbounded();
                RealtimeMonitor::spawn_listener(tx);
                rx.map(Message::RealtimeNeighborEvent)
            })
        } else {
            Subscription::none()
        };

        let drag_drop_sub = iced::event::listen_with(|event, _status, _window| match event {
            iced::Event::Window(iced::window::Event::FileHovered(path)) => {
                Some(Message::StorageFileHovered(path))
            }
            iced::Event::Window(iced::window::Event::FileDropped(path)) => {
                Some(Message::StorageFileDropped(path))
            }
            iced::Event::Window(iced::window::Event::FilesHoveredLeft) => {
                Some(Message::StorageFilesHoveredLeft)
            }
            _ => None,
        });

        Subscription::batch(vec![tick_sub, close_req_sub, close_evt_sub, tray_sub, realtime_sub, drag_drop_sub])
    }


    /// Wraps a visual element with a styled tooltip
    pub(super) fn tip<'a>(
        &self,
        content: impl Into<Element<'a, Message>>,
        tip_text: &'a str,
        pos: tooltip::Position,
    ) -> Element<'a, Message> {
        tooltip(
            content,
            container(text(tip_text).size(10).color(CustomStyles::text_primary()))
                .padding(Padding::from([3, 7]))
                .style(|_| CustomStyles::tooltip_style()),
            pos,
        )
        .gap(4.0)
        .into()
    }


    /// Key-value technical spec row
    pub(super) fn spec_line(key: &'static str, val: &str) -> Element<'static, Message> {
        row![
            text(key).size(11).color(CustomStyles::text_muted()).width(120),
            text(val.to_string()).size(11).color(CustomStyles::text_primary()),
        ]
        .spacing(4)
        .align_y(Alignment::Center)
        .into()
    }


    /// Returns cached filtered and sorted device references in O(N) pointer copies without allocations
    pub(super) fn get_filtered_devices(&self) -> Vec<&Device> {
        self.cached_filtered_indices
            .iter()
            .filter_map(|&idx| self.devices.get(idx))
            .collect()
    }

}
