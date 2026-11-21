use super::*;
use crate::export::Exporter;
use crate::history::{HistoryManager, NetworkEventType};
use crate::i18n::{Language, Translations};
use crate::model::{DeviceCategory, DeviceStatus};
use crate::notification::NotificationUrgency;
use crate::scanner::{ScanEvent, Scanner};
use crate::security_audit::SecurityAuditor;
use crate::storage::Storage;
use iced::futures::channel::mpsc;
use iced::Task;
use std::time::Instant;

impl App {
    pub(super) fn update_network(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AddDeviceSampleRequest(ip) => {
                if let Some(dev) = self.devices.iter_mut().find(|d| d.ip == ip) {
                    let (sample_q, sample_cat, sample_proto) = match dev.category {
                        DeviceCategory::SmartTv => ("api-global.netflix.com", "Streaming Video", "DNS A"),
                        DeviceCategory::Smartphone => ("www.instagram.com", "Red Social", "HTTPS SNI"),
                        DeviceCategory::Tablet => ("mesu.apple.com", "Tablet Sync / OS", "DNS A"),
                        DeviceCategory::Computer => ("github.com", "Desarrollo / Web", "DNS A"),
                        DeviceCategory::StreamingDevice => ("api-global.netflix.com", "Streaming / Video Player", "DNS A"),
                        DeviceCategory::GamingConsole => ("auth.api.sonyentertainmentnetwork.com", "Red de Consola / Gaming", "DNS A"),
                        DeviceCategory::SmartSpeaker => ("msmetrics.ws.sonos.com", "Audio Streaming / Voice", "HTTPS / TLS"),
                        DeviceCategory::SecurityCamera => ("api.ring.com", "Video Cloud Stream", "RTSP / TLS"),
                        DeviceCategory::StorageNas => ("checkip.synology.com", "Almacenamiento / Backup", "HTTPS"),
                        DeviceCategory::Server => ("registry-1.docker.io", "Servidor / Contenedores", "HTTPS"),
                        DeviceCategory::Printer => ("epsonconnect.com", "Servicio de Impresión", "IPP / DNS"),
                        DeviceCategory::Wearable => ("gateway.icloud.com", "Salud / Notificaciones", "HTTPS"),
                        DeviceCategory::NetworkBridge => ("tplinkwifi.net", "Extensor / Switch LAN", "DNS A"),
                        DeviceCategory::Iot => ("mqtt.eclipseprojects.io", "Telemetría IoT", "TCP 1883"),
                        DeviceCategory::Router => ("dns.google", "DNS Resolver", "DNS"),
                        _ => ("connectivitycheck.android.com", "Verificación Red", "DNS A"),
                    };
                    dev.add_request(sample_q.to_string(), sample_cat.to_string(), sample_proto.to_string());
                    Storage::update_device_requests(&mut self.device_metadata, &dev.mac, &dev.request_history);
                    self.save_state();
                    self.notification = Some((format!("Petición registrada: {}", sample_q), true));
                }
                Task::none()
            }

            Message::RefreshNetwork => {
                self.network = Scanner::detect_network_context();
                self.telemetry.update_sample(&self.network.interface);
                self.status = AppStatus::ContextUpdated {
                    iface: self.network.interface.clone(),
                    subnet: self.network.subnet_cidr.clone(),
                };
                Task::none()
            }

            Message::StartScan => {
                if self.is_scanning {
                    // If a background scan was already running, make it visible immediately
                    self.is_background_scan = false;
                    self.status = AppStatus::ScanningSubnet(self.network.subnet_cidr.clone());
                    return Task::none();
                }

                self.is_scanning = true;
                self.is_background_scan = false; // Manual scan: shows progress bar in UI
                self.scan_progress = 0.0;
                self.status = AppStatus::ScanningSubnet(self.network.subnet_cidr.clone());
                self.notification = None;
                self.sync_tray_state();

                let ctx = self.network.clone();
                let oui_db = self.oui_db.clone();

                let (sender, receiver) = mpsc::unbounded::<ScanEvent>();

                tokio::spawn(async move {
                    Scanner::execute_scan(ctx, oui_db, sender).await;
                });

                Task::run(receiver, Message::ScanEventReceived)
            }

            Message::StartBackgroundScan => {
                if self.is_scanning {
                    return Task::none();
                }

                self.is_scanning = true;
                self.is_background_scan = true; // Background scan: does NOT show progress bar
                self.scan_progress = 0.0;
                self.sync_tray_state();

                let ctx = self.network.clone();
                let oui_db = self.oui_db.clone();

                let (sender, receiver) = mpsc::unbounded::<ScanEvent>();

                tokio::spawn(async move {
                    Scanner::execute_scan(ctx, oui_db, sender).await;
                });

                Task::run(receiver, Message::ScanEventReceived)
            }

            Message::ScanEventReceived(event) => match event {
                ScanEvent::Started { total_hosts, subnet } => {
                    self.scanned_macs_current_cycle.clear();
                    if !self.is_background_scan {
                        self.status = AppStatus::ProbingHosts {
                            total: total_hosts,
                            subnet,
                        };
                    }
                    Task::none()
                }
                ScanEvent::Progress { scanned, total } => {
                    self.scan_progress = scanned as f32 / total as f32;
                    Task::none()
                }
                ScanEvent::HostFound(mut device) => {
                    let mac_key = device.mac.trim().to_lowercase();
                    self.scanned_macs_current_cycle.insert(mac_key.clone());

                    // Reapply persisted metadata if it exists for this MAC
                    if let Some(meta) = self.device_metadata.get(&mac_key) {
                        if let Some(alias) = &meta.custom_alias {
                            if !alias.trim().is_empty() {
                                device.custom_alias = Some(alias.clone());
                            }
                        }
                        if let Some(cat) = meta.category {
                            device.category = cat;
                        }
                        device.is_whitelisted = meta.is_whitelisted;
                        device.is_blocked = meta.is_blocked;
                        device.speed_limit_kbps = meta.speed_limit_kbps;
                    }

                    // Enrich with router topology if already available in cache
                    let norm_mac = device.mac.trim().to_uppercase();
                    if let Some(conn) = self.router_connections.get(&norm_mac) {
                        if let Some(w) = &conn.wireless {
                            device.connection_type = w.band.clone();
                            device.wifi_signal_dbm = w.signal_decibels;
                        } else if conn.negotiated_mbps.is_some() {
                            device.connection_type = Some("Ethernet".to_string());
                        }
                        device.negotiated_mbps = conn.negotiated_mbps;
                    }

                    let mut intruder_task = Task::none();

                    // Find previous device in list by MAC or by IP
                    let existing_idx = self.devices.iter().position(|d| d.mac.trim().to_lowercase() == mac_key || d.ip == device.ip);
                    let was_offline = existing_idx.map(|idx| self.devices[idx].status == DeviceStatus::Offline).unwrap_or(true);
                    let was_not_active = !self.active_online_macs.contains(&mac_key);
                    let is_reconnection = (was_offline || was_not_active) && self.initial_scan_completed;
                    let is_new_device = !self.known_macs.contains(&mac_key);

                    self.active_online_macs.insert(mac_key.clone());
                    self.known_macs.insert(mac_key.clone());
                    self.missed_heartbeats.insert(device.ip, 0);

                    // Ensure detected device state is Online
                    device.status = DeviceStatus::Online;

                    if is_new_device {
                        let event = HistoryManager::create_event(
                            NetworkEventType::NewDeviceDetected,
                            "Nuevo Dispositivo Detectado".to_string(),
                            format!("{} ({}) visto por primera vez en la red.", device.display_name(), device.ip),
                            Some(device.ip),
                            Some(device.mac.clone()),
                        );
                        self.network_events.insert(0, event);

                        // If new unknown device not on whitelist: Intruder Alert
                        if !device.is_current_host && !device.is_gateway && !device.is_whitelisted {
                            if self.alerted_macs.insert(mac_key.clone()) {
                                self.pending_intruder_alert = Some(IntruderAlert {
                                    ip: device.ip,
                                    mac: device.mac.clone(),
                                    vendor: device.vendor.clone(),
                                    hostname: device.hostname.clone(),
                                });

                                if self.notification_settings.enabled && self.notification_settings.notify_unknown_intruders {
                                    let dev_ip = device.ip;
                                    let dev_mac = device.mac.clone();
                                    let alert_title = Translations::intruder_alert_title(self.language);
                                    let trust_label = Translations::intruder_btn_trust(self.language);
                                    let block_label = Translations::intruder_btn_block(self.language);
                                    let body = self.notification_settings.format_device_message(
                                        "Nuevo Dispositivo Desconocido",
                                        &device.display_name(),
                                        device.ip,
                                        &device.mac,
                                        &device.vendor,
                                        Some("Detectado por primera vez en la red local"),
                                    );

                                    let notif_settings = self.notification_settings.clone();
                                    intruder_task = Task::future(async move {
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
                    } else if is_reconnection {
                        // Device was previously known and reconnected to network
                        if !device.is_current_host && !device.is_gateway && !device.is_whitelisted {
                            // *** CRITICAL: RECONNECTION OF KNOWN UNAUTHORIZED DEVICE (NOT ON WHITELIST) ***
                            let event = HistoryManager::create_event(
                                NetworkEventType::SecurityAlert,
                                "Reconexión No Autorizada".to_string(),
                                format!("{} ({}) no está en la Lista Blanca y se ha reconectado a la red.", device.display_name(), device.ip),
                                Some(device.ip),
                                Some(device.mac.clone()),
                            );
                            self.network_events.insert(0, event);

                            if self.notification_settings.enabled && self.notification_settings.notify_known_unwhitelisted_reconnect {
                                if self.pending_intruder_alert.is_none() {
                                    self.pending_intruder_alert = Some(IntruderAlert {
                                        ip: device.ip,
                                        mac: device.mac.clone(),
                                        vendor: device.vendor.clone(),
                                        hostname: device.hostname.clone(),
                                    });
                                }

                                let dev_ip = device.ip;
                                let dev_mac = device.mac.clone();
                                let alert_title = Translations::notif_reconnect_unwhitelisted_title(self.language);
                                let trust_label = Translations::intruder_btn_trust(self.language);
                                let block_label = Translations::intruder_btn_block(self.language);
                                let body = self.notification_settings.format_device_message(
                                    "Reconexión No Autorizada",
                                    &device.display_name(),
                                    device.ip,
                                    &device.mac,
                                    &device.vendor,
                                    Some("Dispositivo conocido fuera de la Lista Blanca"),
                                );

                                let notif_settings = self.notification_settings.clone();
                                intruder_task = Task::future(async move {
                                    let action = notif_settings.dispatch_interactive_alert(alert_title, &body, trust_label, block_label).await;
                                    Message::DesktopNotificationAction {
                                        ip: dev_ip,
                                        mac: dev_mac,
                                        action,
                                    }
                                });
                            }
                        } else if device.is_whitelisted {
                            // Reconnection of trusted device (on whitelist)
                            let event = HistoryManager::create_event(
                                NetworkEventType::DeviceConnected,
                                "Dispositivo Conectado".to_string(),
                                format!("{} ({}) activo en la red local.", device.display_name(), device.ip),
                                Some(device.ip),
                                Some(device.mac.clone()),
                            );
                            self.network_events.insert(0, event);

                            if self.notification_settings.enabled && self.notification_settings.notify_whitelisted_reconnect {
                                let title = Translations::notif_reconnect_whitelisted_title(self.language);
                                let body = self.notification_settings.format_device_message(
                                    "Conectado",
                                    &device.display_name(),
                                    device.ip,
                                    &device.mac,
                                    &device.vendor,
                                    Some("Dispositivo de confianza en Lista Blanca"),
                                );
                                self.notification_settings.dispatch(title, &body, "network-wireless", None);
                            }
                        } else {
                            // Gateway or Host reconnected
                            let event = HistoryManager::create_event(
                                NetworkEventType::DeviceConnected,
                                "Dispositivo Conectado".to_string(),
                                format!("{} ({}) activo en la red local.", device.display_name(), device.ip),
                                Some(device.ip),
                                Some(device.mac.clone()),
                            );
                            self.network_events.insert(0, event);
                        }
                    }

                    if self.network_events.len() > 150 {
                        self.network_events.truncate(150);
                    }

                    if let Some(idx) = existing_idx {
                        let prev_alias = self.devices[idx].custom_alias.clone();
                        let prev_category = self.devices[idx].category;
                        let prev_wl = self.devices[idx].is_whitelisted;
                        let prev_bl = self.devices[idx].is_blocked;
                        let prev_lim = self.devices[idx].speed_limit_kbps;
                        let prev_requests = self.devices[idx].request_history.clone();

                        self.devices[idx] = device;

                        if self.devices[idx].custom_alias.is_none() {
                            self.devices[idx].custom_alias = prev_alias;
                        }
                        if prev_category != DeviceCategory::Unknown && self.devices[idx].category == DeviceCategory::Unknown {
                            self.devices[idx].category = prev_category;
                        }
                        if !self.device_metadata.contains_key(&mac_key) {
                            if prev_wl {
                                self.devices[idx].is_whitelisted = true;
                            }
                            if prev_bl {
                                self.devices[idx].is_blocked = true;
                            }
                        }
                        if self.devices[idx].speed_limit_kbps.is_none() {
                            self.devices[idx].speed_limit_kbps = prev_lim;
                        }
                        if !prev_requests.is_empty() {
                            for r in prev_requests {
                                if !self.devices[idx].request_history.iter().any(|e| e.query == r.query) {
                                    self.devices[idx].request_history.push(r);
                                }
                            }
                            if self.devices[idx].request_history.len() > 60 {
                                self.devices[idx].request_history.truncate(60);
                            }
                        }
                    } else {
                        self.devices.push(device);
                    }
                    self.rebuild_device_caches();
                    intruder_task
                }
                ScanEvent::Completed {
                    total_found,
                    duration_ms,
                } => {
                    // Detect disconnected devices (were online and did not respond in this cycle)
                    for dev in self.devices.iter_mut() {
                        let dev_mac = dev.mac.trim().to_lowercase();
                        if dev.status == DeviceStatus::Online && !self.scanned_macs_current_cycle.contains(&dev_mac) {
                            dev.status = DeviceStatus::Offline;
                            self.active_online_macs.remove(&dev_mac);

                            if self.initial_scan_completed && !dev.is_gateway && !dev.is_current_host {
                                if self.notification_settings.enabled && self.notification_settings.notify_device_disconnected {
                                    let title = Translations::notif_device_disconnected_title(self.language);
                                    let body = self.notification_settings.format_device_message(
                                        "Desconectado",
                                        &dev.display_name(),
                                        dev.ip,
                                        &dev.mac,
                                        &dev.vendor,
                                        Some("El dispositivo ya no responde en la red local"),
                                    );
                                    self.notification_settings.dispatch(title, &body, "network-offline", None);
                                }

                                let event = HistoryManager::create_event(
                                    NetworkEventType::DeviceDisconnected,
                                    "Dispositivo Desconectado".to_string(),
                                    format!("{} ({}) ya no responde en la red local.", dev.display_name(), dev.ip),
                                    Some(dev.ip),
                                    Some(dev.mac.clone()),
                                );
                                self.network_events.insert(0, event);
                            }
                        }
                    }
                    if self.network_events.len() > 150 {
                        self.network_events.truncate(150);
                    }
                    self.initial_scan_completed = true;

                    self.is_scanning = false;
                    self.is_background_scan = false;
                    self.scan_progress = 1.0;
                    self.last_scan_time = Instant::now();
                    self.status = AppStatus::ScanFinished {
                        total_found,
                        duration_secs: duration_ms as f32 / 1000.0,
                    };
                    self.rebuild_device_caches();
                    self.sync_tray_state();
                    self.save_state();
                    crate::trim_memory();

                    // Reevaluate network security audit
                    let port_mappings = self.upnp_status.as_ref().map(|u| u.port_mappings.as_slice()).unwrap_or(&[]);
                    let has_router_pw = !self.router_password_input.trim().is_empty();
                    let audit = SecurityAuditor::audit(&self.devices, port_mappings, has_router_pw);
                    if self.notification_settings.enabled && self.notification_settings.notify_security_critical {
                        if audit.critical_count > 0 {
                            let title = Translations::notif_trigger_security(self.language);
                            let body = format!("Se detectaron {} problemas críticos de seguridad en la red local.", audit.critical_count);
                            self.notification_settings.dispatch(title, &body, "dialog-warning", Some(NotificationUrgency::Critical));
                        }
                    }
                    self.security_audit_report = Some(audit);

                    let mut tasks = Vec::new();
                    if !self.router_password_input.trim().is_empty() {
                        tasks.push(Task::done(Message::FetchRouterTopology));
                    }
                    if self.upnp_status.is_none() && self.network.gateway_ip.is_some() {
                        tasks.push(Task::done(Message::QueryUpnpRouter));
                    }
                    if !tasks.is_empty() {
                        Task::batch(tasks)
                    } else {
                        Task::none()
                    }
                }
                ScanEvent::Error(err) => {
                    let was_background = self.is_background_scan;
                    self.is_scanning = false;
                    self.is_background_scan = false;
                    self.sync_tray_state();
                    crate::trim_memory();
                    if !was_background {
                        self.status = AppStatus::Error(err.clone());
                        self.notification = Some((err, false));
                    }
                    Task::none()
                }
            },

            Message::SearchChanged(query) => {
                self.search_query = query;
                self.rebuild_device_caches();
                Task::none()
            }

            Message::CategorySelected(cat) => {
                self.selected_category = cat;
                if cat.is_none() {
                    self.security_filter = SecurityFilter::All;
                }
                self.rebuild_device_caches();
                Task::none()
            }

            Message::SortChanged(sort) => {
                self.sort_by = sort;
                self.rebuild_device_caches();
                Task::none()
            }

            Message::SelectDevice(ip) => {
                self.selected_device_ip = Some(ip);
                self.inspector_ping_stats = None;
                self.deep_ports.clear();
                if let Some(dev) = self.devices.iter().find(|d| d.ip == ip) {
                    self.alias_input = dev.custom_alias.clone().unwrap_or_default();
                    self.deep_ports = dev.open_ports.clone();
                }
                Task::none()
            }

            Message::CloseInspector => {
                self.selected_device_ip = None;
                self.inspector_ping_stats = None;
                self.deep_ports.clear();
                Task::none()
            }

            Message::StartPingTest(ip) => {
                self.is_pinging = true;
                self.inspector_ping_stats = None;
                let handle = tokio::spawn(async move { Scanner::ping_test_detailed(ip).await });
                Task::future(async move {
                    let res = match handle.await {
                        Ok(inner) => inner,
                        Err(e) => Err(format!("Error en tarea asíncrona: {}", e)),
                    };
                    Message::PingTestFinished(res)
                })
            }

            Message::PingTestFinished(result) => {
                self.is_pinging = false;
                match result {
                    Ok(stats) => {
                        if let Some(dev) = self.devices.iter_mut().find(|d| d.ip == stats.ip) {
                            dev.latency_ms = Some(stats.avg_ms);
                        }
                        self.ping_history.push_back(stats.avg_ms);
                        if self.ping_history.len() > 30 {
                            self.ping_history.pop_front();
                        }
                        self.ping_sparkline_cache.clear();
                        self.inspector_ping_stats = Some(stats);
                        self.rebuild_device_caches();
                    }
                    Err(err) => {
                        self.notification = Some((format!("Ping error: {}", err), false));
                    }
                }
                Task::none()
            }

            Message::StartDeepScan(ip) => {
                self.is_deep_scanning = true;
                let handle = tokio::spawn(async move { Scanner::deep_port_scan(ip).await });
                Task::future(async move {
                    let ports = handle.await.unwrap_or_default();
                    Message::DeepScanFinished(ports)
                })
            }

            Message::DeepScanFinished(ports) => {
                self.is_deep_scanning = false;
                self.deep_ports = ports.clone();

                if let Some(selected_ip) = self.selected_device_ip {
                    if let Some(dev) = self.devices.iter_mut().find(|d| d.ip == selected_ip) {
                        dev.open_ports = ports;
                    }
                }
                self.rebuild_device_caches();
                Task::none()
            }

            Message::SendWol(mac) => {
                let res = Scanner::send_wake_on_lan(&mac);
                Task::done(Message::WolFinished(res))
            }

            Message::WolFinished(res) => {
                match res {
                    Ok(_) => {
                        let msg = match self.language {
                            Language::Spanish => "Paquete mágico Wake-on-LAN enviado con éxito.",
                            Language::English => "Wake-on-LAN magic packet sent successfully.",
                            Language::Chinese => "Wake-on-LAN 唤醒数据包发送成功。",
                            Language::Hindi => "वेक-ऑन-लैन पैकेट सफलतापूर्वक भेजा गया।",
                            Language::Arabic => "تم إرسال حزمة Wake-on-LAN بنجاح.",
                            Language::French => "Paquet magique Wake-on-LAN envoyé avec succès.",
                            Language::Bengali => "Wake-on-LAN প্যাকেট সফলভাবে পাঠানো হয়েছে।",
                            Language::Portuguese => "Pacote mágico Wake-on-LAN enviado com sucesso.",
                            Language::Russian => "Пакет Wake-on-LAN успешно отправлен.",
                            Language::Urdu => "Wake-on-LAN جادوئی پیکٹ کامیابی سے بھیج دیا گیا۔",
                        };
                        self.notification = Some((msg.to_string(), true));
                    }
                    Err(err) => {
                        self.notification = Some((format!("WoL error: {}", err), false));
                    }
                }
                Task::none()
            }

            Message::OpenInBrowser(ip) => {
                let url = format!("http://{}", ip);
                let _ = crate::procutil::opener().arg(&url).spawn();
                let notif_msg = match self.language {
                    Language::Spanish => format!("Abriendo {} en el navegador...", url),
                    Language::English => format!("Opening {} in browser...", url),
                    Language::Chinese => format!("正在浏览器中打开 {}...", url),
                    Language::Hindi => format!("ब्राउज़र में {} खोला जा रहा है...", url),
                    Language::Arabic => format!("جاري فتح {} في المتصفح...", url),
                    Language::French => format!("Ouverture de {} dans le navigateur...", url),
                    Language::Bengali => format!("ব্রাউজারে {} খোলা হচ্ছে...", url),
                    Language::Portuguese => format!("Abrindo {} no navegador...", url),
                    Language::Russian => format!("Открытие {} в браузере...", url),
                    Language::Urdu => format!("براؤزر میں {} کھولا جا رہا ہے...", url),
                };
                self.notification = Some((notif_msg, true));
                Task::none()
            }

            Message::AliasInputChanged(input) => {
                self.alias_input = input;
                Task::none()
            }

            Message::SaveAlias(ip) => {
                let trimmed = self.alias_input.trim().to_string();
                if let Some(dev) = self.devices.iter_mut().find(|d| d.ip == ip) {
                    let alias_opt = if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed.clone())
                    };
                    dev.custom_alias = alias_opt.clone();
                    Storage::update_device_meta(
                        &mut self.device_metadata,
                        &dev.mac,
                        alias_opt,
                        None,
                    );
                }
                self.rebuild_device_caches();
                self.save_state();
                let notif_msg = match self.language {
                    Language::Spanish => format!("Alias guardado para {}.", ip),
                    Language::English => format!("Alias saved for {}.", ip),
                    Language::Chinese => format!("已为 {} 保存别名。", ip),
                    Language::Hindi => format!("{} के लिए उपनाम सहेजा गया।", ip),
                    Language::Arabic => format!("تم حفظ اللقب لـ {}.", ip),
                    Language::French => format!("Surnom enregistré pour {}.", ip),
                    Language::Bengali => format!("{} এর জন্য ডাকনাম সংরক্ষিত হয়েছে।", ip),
                    Language::Portuguese => format!("Apelido salvo para {}.", ip),
                    Language::Russian => format!("Псевдоним сохранен для {}.", ip),
                    Language::Urdu => format!("{} کے لیے عرفیت محفوظ کر لی گئی۔", ip),
                };
                self.notification = Some((notif_msg, true));
                Task::none()
            }

            Message::CopyToClipboard(text_to_copy, label) => {
                let notif_msg = if text_to_copy.contains("install-repo.sh") {
                    Translations::update_copied_notif(self.language).to_string()
                } else {
                    match self.language {
                        Language::Spanish => format!("{} copiado al portapapeles.", label),
                        Language::English => format!("{} copied to clipboard.", label),
                        Language::Chinese => format!("{} 已复制到剪贴板。", label),
                        Language::Hindi => format!("{} क्लिपबोर्ड पर कॉपी किया गया।", label),
                        Language::Arabic => format!("تم نسخ {} إلى الحافظة.", label),
                        Language::French => format!("{} copié dans le presse-papiers.", label),
                        Language::Bengali => format!("{} ক্লিপবোর্ডে কপি করা হয়েছে।", label),
                        Language::Portuguese => format!("{} copiado para a área de transferência.", label),
                        Language::Russian => format!("{} скопировано в буфер обмена.", label),
                        Language::Urdu => format!("{} کلپ بورڈ پر کاپی ہو گیا۔", label),
                    }
                };
                self.notification = Some((notif_msg, true));
                iced::clipboard::write(text_to_copy)
            }

            Message::ExportJson => {
                let default_name = Exporter::generate_json_filename();
                let default_path = Exporter::get_export_path(&default_name);
                let title = Translations::export_json_dialog_title(self.language);
                Task::future(async move {
                    let picked = tokio::task::spawn_blocking(move || {
                        Exporter::pick_save_path_with_file_dialog(&default_path, "JSON", "json", title)
                    })
                    .await
                    .unwrap_or(None);
                    Message::ExportJsonSelected(picked)
                })
            }

            Message::ExportJsonSelected(Some(path)) => {
                match Exporter::save_json_to_path(&path, &self.devices, &self.network) {
                    Ok(()) => {
                        let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("JSON");
                        let notif_msg = Translations::export_success_msg(fname, self.language);
                        self.notification = Some((notif_msg, true));
                        Task::none()
                    }
                    Err(err) => {
                        self.notification = Some((err, false));
                        Task::none()
                    }
                }
            }

            Message::ExportJsonSelected(None) => Task::none(),

            Message::ExportCsv => {
                let default_name = Exporter::generate_csv_filename();
                let default_path = Exporter::get_export_path(&default_name);
                let title = Translations::export_csv_dialog_title(self.language);
                Task::future(async move {
                    let picked = tokio::task::spawn_blocking(move || {
                        Exporter::pick_save_path_with_file_dialog(&default_path, "CSV", "csv", title)
                    })
                    .await
                    .unwrap_or(None);
                    Message::ExportCsvSelected(picked)
                })
            }

            Message::ExportCsvSelected(Some(path)) => {
                match Exporter::save_csv_to_path(&path, &self.devices) {
                    Ok(()) => {
                        let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("CSV");
                        let notif_msg = Translations::export_csv_success_msg(fname, self.language);
                        self.notification = Some((notif_msg, true));
                        Task::none()
                    }
                    Err(err) => {
                        self.notification = Some((err, false));
                        Task::none()
                    }
                }
            }

            Message::ExportCsvSelected(None) => Task::none(),

            Message::DismissNotification => {
                self.notification = None;
                Task::none()
            }
            _ => Task::none(),
        }
    }
}
