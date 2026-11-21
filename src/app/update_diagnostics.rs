use super::*;
use crate::history::{HistoryManager, NetworkEventType};
use crate::i18n::{Language, Translations};
use crate::notification::NotificationUrgency;
use crate::router::{
    LinksysRouter,
    UpnpClient,
};
use crate::security_audit::SecurityAuditor;
use crate::speedtest::{SpeedTest, SpeedTestEvent, SpeedTestStage};
use crate::storage::Storage;
use crate::traceroute::Traceroute;
use crate::wifi_analyzer::WifiAnalyzer;
use iced::futures::channel::mpsc;
use iced::Task;

impl App {
    pub(super) fn update_diagnostics(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenSpeedTest => {
                self.is_speed_test_open = true;
                self.is_settings_open = false;
                self.is_about_open = false;
                Task::none()
            }

            Message::CloseSpeedTest => {
                self.is_speed_test_open = false;
                Task::none()
            }

            Message::StartSpeedTest => {
                if self.speed_test_stage == SpeedTestStage::TestingPing
                    || self.speed_test_stage == SpeedTestStage::TestingDownload
                    || self.speed_test_stage == SpeedTestStage::TestingUpload
                {
                    return Task::none();
                }

                self.speed_test_stage = SpeedTestStage::TestingPing;
                self.speed_test_ping = None;
                self.speed_test_download = None;
                self.speed_test_upload = None;

                let (tx, rx) = mpsc::unbounded::<SpeedTestEvent>();
                tokio::spawn(async move {
                    SpeedTest::execute(tx).await;
                });

                Task::run(rx, Message::SpeedTestEventReceived)
            }

            Message::SpeedTestEventReceived(event) => match event {
                SpeedTestEvent::PingStarted => {
                    self.speed_test_stage = SpeedTestStage::TestingPing;
                    Task::none()
                }
                SpeedTestEvent::PingDone(ping) => {
                    self.speed_test_ping = Some(ping);
                    self.speed_test_stage = SpeedTestStage::TestingDownload;
                    Task::none()
                }
                SpeedTestEvent::DownloadStarted => {
                    self.speed_test_stage = SpeedTestStage::TestingDownload;
                    Task::none()
                }
                SpeedTestEvent::DownloadDone(dl) => {
                    self.speed_test_download = Some(dl);
                    self.speed_test_stage = SpeedTestStage::TestingUpload;
                    Task::none()
                }
                SpeedTestEvent::UploadStarted => {
                    self.speed_test_stage = SpeedTestStage::TestingUpload;
                    Task::none()
                }
                SpeedTestEvent::UploadDone(ul) => {
                    self.speed_test_upload = Some(ul);
                    Task::none()
                }
                SpeedTestEvent::DownloadProgress { .. } | SpeedTestEvent::UploadProgress { .. } => {
                    Task::none()
                }
                SpeedTestEvent::Finished(result) => {
                    self.speed_test_stage = SpeedTestStage::Completed;
                    self.speed_test_ping = Some(result.ping_ms);
                    self.speed_test_download = Some(result.download_mbps);
                    self.speed_test_upload = Some(result.upload_mbps);
                    self.speed_test_result = Some(result.clone());
                    self.speed_test_history.insert(0, result);
                    if self.speed_test_history.len() > 100 {
                        self.speed_test_history.truncate(100);
                    }
                    self.save_state();
                    Task::none()
                }
                SpeedTestEvent::Error(err) => {
                    self.speed_test_stage = SpeedTestStage::Failed;
                    self.notification = Some((err, false));
                    Task::none()
                }
            },

            Message::ClearSpeedTestHistory => {
                self.speed_test_history.clear();
                self.save_state();
                let notif_msg = Translations::speed_test_history_empty(self.language);
                self.notification = Some((notif_msg.to_string(), true));
                Task::none()
            }

            Message::SecurityFilterChanged(filter) => {
                self.security_filter = filter;
                self.rebuild_device_caches();
                Task::none()
            }

            Message::ToggleDeviceWhitelist(ip, mac) => {
                let mut is_now_whitelisted = false;
                if let Some(dev) = self.devices.iter_mut().find(|d| d.ip == ip) {
                    dev.is_whitelisted = !dev.is_whitelisted;
                    is_now_whitelisted = dev.is_whitelisted;
                    Storage::set_device_whitelist(
                        &mut self.device_metadata,
                        &mac,
                        dev.is_whitelisted,
                    );
                }
                self.rebuild_device_caches();
                self.save_state();
                let notif_msg = if is_now_whitelisted {
                    format!("Dispositivo {} añadido a Lista Blanca.", ip)
                } else {
                    format!("Dispositivo {} retirado de Lista Blanca.", ip)
                };
                self.notification = Some((notif_msg, true));
                Task::none()
            }

            Message::ToggleDeviceBlocked(ip, mac) => {
                if let Some(dev) = self.devices.iter().find(|d| d.ip == ip) {
                    if dev.is_current_host {
                        self.notification = Some((Translations::cannot_block_self(self.language).to_string(), false));
                        return Task::none();
                    }
                    if dev.is_gateway {
                        self.notification = Some((Translations::cannot_block_gateway(self.language).to_string(), false));
                        return Task::none();
                    }
                }

                if self.router_password_input.trim().is_empty() {
                    self.notification = Some((Translations::router_require_password(self.language).to_string(), false));
                    return Task::none();
                }

                if self.is_operating_device.is_some() {
                    return Task::none();
                }

                let current_blocked = self
                    .devices
                    .iter()
                    .find(|d| d.ip == ip)
                    .map(|d| d.is_blocked)
                    .unwrap_or(false);
                let target_blocked = !current_blocked;

                self.is_operating_device = Some(ip);

                let router_ip = self.router_ip_input.clone();
                let router_pass = self.router_password_input.clone();
                let mac_clone = mac.clone();

                Task::future(async move {
                    let res = if target_blocked {
                        LinksysRouter::block_mac(&router_ip, &router_pass, &mac_clone).await
                    } else {
                        LinksysRouter::unblock_mac(&router_ip, &router_pass, &mac_clone).await
                    };
                    Message::RouterOperationFinished {
                        ip,
                        mac: mac_clone,
                        target_blocked,
                        result: res,
                    }
                })
            }

            Message::RouterOperationFinished {
                ip,
                mac,
                target_blocked,
                result,
            } => {
                self.is_operating_device = None;
                match result {
                    Ok(()) => {
                        if let Some(dev) = self.devices.iter_mut().find(|d| d.ip == ip) {
                            dev.is_blocked = target_blocked;
                        }
                        Storage::set_device_blocked(
                            &mut self.device_metadata,
                            &mac,
                            target_blocked,
                        );
                        self.rebuild_device_caches();
                        self.save_state();

                        let notif = if target_blocked {
                            Translations::router_block_success(self.language, &ip.to_string(), &mac)
                        } else {
                            Translations::router_unblock_success(self.language, &ip.to_string(), &mac)
                        };
                        self.notification = Some((notif, true));
                    }
                    Err(err) => {
                        let notif = Translations::router_operation_failed(self.language, &err);
                        self.notification = Some((notif, false));
                    }
                }
                Task::none()
            }

            Message::RouterIpChanged(ip) => {
                self.router_ip_input = ip;
                self.mark_state_dirty();
                Task::none()
            }

            Message::RouterPasswordChanged(pass) => {
                self.router_password_input = pass;
                self.mark_state_dirty();
                Task::none()
            }

            Message::TestRouterConnection => {
                self.is_testing_router = true;
                let ip = self.router_ip_input.clone();
                let pass = self.router_password_input.clone();
                Task::future(async move {
                    let res = LinksysRouter::verify_admin_password(&ip, &pass).await;
                    Message::RouterTestFinished(res)
                })
            }

            Message::RouterTestFinished(res) => {
                self.is_testing_router = false;
                match res {
                    Ok(true) => {
                        self.router_status_msg = Some((Translations::router_connected_ok(self.language).to_string(), true));
                        self.notification = Some((Translations::router_connected_ok(self.language).to_string(), true));
                    }
                    Ok(false) => {
                        self.router_status_msg = Some((Translations::router_connected_err(self.language).to_string(), false));
                        self.notification = Some((Translations::router_connected_err(self.language).to_string(), false));
                    }
                    Err(err) => {
                        let err_msg = format!("🔴 Error: {}", err);
                        self.router_status_msg = Some((err_msg.clone(), false));
                        self.notification = Some((err_msg, false));
                    }
                }
                Task::none()
            }

            Message::TrustDevice(mac) => {
                let mac_norm = mac.trim().to_lowercase();
                self.known_macs.insert(mac_norm.clone());
                Storage::set_device_whitelist(&mut self.device_metadata, &mac_norm, true);
                if let Some(dev) = self.devices.iter_mut().find(|d| d.mac.trim().eq_ignore_ascii_case(&mac_norm)) {
                    dev.is_whitelisted = true;
                }
                self.pending_intruder_alert = None;
                self.rebuild_device_caches();
                self.save_state();
                self.notification = Some(("Dispositivo verificado y añadido a Lista Blanca.".to_string(), true));
                Task::none()
            }

            Message::DismissIntruderAlert => {
                if let Some(alert) = &self.pending_intruder_alert {
                    self.known_macs.insert(alert.mac.trim().to_lowercase());
                }
                self.pending_intruder_alert = None;
                self.rebuild_device_caches();
                self.save_state();
                Task::none()
            }

            Message::FetchRouterTopology => {
                let ip = self.router_ip_input.clone();
                let pass = self.router_password_input.clone();
                if pass.trim().is_empty() {
                    return Task::none();
                }
                Task::future(async move {
                    let res = LinksysRouter::get_network_connections(&ip, &pass).await;
                    Message::RouterTopologyReceived(res)
                })
            }

            Message::RouterTopologyReceived(res) => {
                if let Ok(conns) = res {
                    for c in conns {
                        let key = c.mac_address.trim().to_uppercase();
                        if let Some(dev) = self.devices.iter_mut().find(|d| d.mac.trim().eq_ignore_ascii_case(&key)) {
                            if let Some(w) = &c.wireless {
                                dev.connection_type = w.band.clone();
                                dev.wifi_signal_dbm = w.signal_decibels;
                            } else if c.negotiated_mbps.is_some() {
                                dev.connection_type = Some("Ethernet".to_string());
                            }
                            dev.negotiated_mbps = c.negotiated_mbps;
                        }
                        self.router_connections.insert(key, c);
                    }
                    self.rebuild_device_caches();
                }
                Task::none()
            }

            Message::StartTraceroute(target_ip) => {
                self.is_tracing_route = true;
                self.traceroute_error = None;
                self.traceroute_hops = None;
                Task::future(async move {
                    let res = Traceroute::trace(&target_ip.to_string()).await;
                    Message::TracerouteFinished(res)
                })
            }

            Message::TracerouteFinished(res) => {
                self.is_tracing_route = false;
                match res {
                    Ok(hops) => {
                        self.traceroute_hops = Some(hops);
                    }
                    Err(err) => {
                        self.traceroute_error = Some(err);
                    }
                }
                Task::none()
            }

            Message::ChangeDeviceCategory(ip, new_category) => {
                if let Some(dev) = self.devices.iter_mut().find(|d| d.ip == ip) {
                    dev.category = new_category;
                    Storage::update_device_meta(
                        &mut self.device_metadata,
                        &dev.mac,
                        None,
                        Some(new_category),
                    );
                }
                self.rebuild_device_caches();
                self.save_state();
                let notif_msg = match self.language {
                    Language::Spanish => format!(
                        "Tipo de dispositivo actualizado a: {}",
                        Translations::category_name(new_category, self.language)
                    ),
                    Language::English => format!(
                        "Device type updated to: {}",
                        Translations::category_name(new_category, self.language)
                    ),
                    Language::Chinese => format!(
                        "设备类型已更新为：{}",
                        Translations::category_name(new_category, self.language)
                    ),
                    Language::Hindi => format!(
                        "डिवाइस प्रकार बदलकर किया गया: {}",
                        Translations::category_name(new_category, self.language)
                    ),
                    Language::Arabic => format!(
                        "تم تحديث نوع الجهاز إلى: {}",
                        Translations::category_name(new_category, self.language)
                    ),
                    Language::French => format!(
                        "Type d'appareil mis à jour : {}",
                        Translations::category_name(new_category, self.language)
                    ),
                    Language::Bengali => format!(
                        "ডিভাইসের ধরন আপডেট হয়েছে: {}",
                        Translations::category_name(new_category, self.language)
                    ),
                    Language::Portuguese => format!(
                        "Tipo de dispositivo atualizado para: {}",
                        Translations::category_name(new_category, self.language)
                    ),
                    Language::Russian => format!(
                        "Тип устройства обновлен: {}",
                        Translations::category_name(new_category, self.language)
                    ),
                    Language::Urdu => format!(
                        "ڈیوائس کی قسم تبدیل ہو گئی: {}",
                        Translations::category_name(new_category, self.language)
                    ),
                };
                self.notification = Some((notif_msg, true));
                Task::none()
            }

            // Event History & Timeline
            Message::OpenTimeline => {
                self.is_timeline_open = true;
                self.is_settings_open = false;
                self.is_about_open = false;
                self.is_speed_test_open = false;
                self.is_audit_open = false;
                self.is_wifi_analyzer_open = false;
                self.is_upnp_open = false;
                Task::none()
            }
            Message::CloseTimeline => {
                self.is_timeline_open = false;
                Task::none()
            }
            Message::ClearTimeline => {
                self.network_events.clear();
                self.save_state();
                self.notification = Some((Translations::timeline_cleared_notif(self.language).to_string(), true));
                Task::none()
            }

            // Health & Security Audit
            Message::OpenSecurityAudit => {
                self.is_audit_open = true;
                self.is_settings_open = false;
                self.is_about_open = false;
                self.is_speed_test_open = false;
                self.is_timeline_open = false;
                self.is_wifi_analyzer_open = false;
                self.is_upnp_open = false;
                if self.security_audit_report.is_none() {
                    return Task::done(Message::RunSecurityAudit);
                }
                Task::none()
            }
            Message::CloseSecurityAudit => {
                self.is_audit_open = false;
                Task::none()
            }
            Message::RunSecurityAudit => {
                let port_mappings = self.upnp_status.as_ref().map(|u| u.port_mappings.as_slice()).unwrap_or(&[]);
                let has_router_pw = !self.router_password_input.trim().is_empty();
                self.security_audit_report = Some(SecurityAuditor::audit(&self.devices, port_mappings, has_router_pw));
                Task::none()
            }

            // Universal Router UPnP & Port Forwarding
            Message::OpenUpnpModal => {
                self.is_upnp_open = true;
                self.is_settings_open = false;
                self.is_about_open = false;
                self.is_speed_test_open = false;
                self.is_timeline_open = false;
                self.is_audit_open = false;
                self.is_wifi_analyzer_open = false;
                if self.upnp_status.is_none() {
                    return Task::done(Message::QueryUpnpRouter);
                }
                Task::none()
            }
            Message::CloseUpnpModal => {
                self.is_upnp_open = false;
                Task::none()
            }
            Message::QueryUpnpRouter => {
                if let Some(gw) = self.network.gateway_ip {
                    Task::future(async move {
                        let res = UpnpClient::query_router(gw).await;
                        Message::UpnpRouterResult(res)
                    })
                } else {
                    self.notification = Some(("No se detectó puerta de enlace para consultar UPnP.".to_string(), false));
                    Task::none()
                }
            }
            Message::UpnpRouterResult(res) => {
                match res {
                    Ok(status) => {
                        self.upnp_status = Some(status);
                        let port_mappings = self.upnp_status.as_ref().map(|u| u.port_mappings.as_slice()).unwrap_or(&[]);
                        let has_router_pw = !self.router_password_input.trim().is_empty();
                        self.security_audit_report = Some(SecurityAuditor::audit(&self.devices, port_mappings, has_router_pw));
                    }
                    Err(err) => {
                        self.notification = Some((format!("Aviso UPnP: {}", err), false));
                    }
                }
                Task::none()
            }

            // Router USB Storage / NAS Explorer
            Message::OpenWifiAnalyzer => {
                self.is_wifi_analyzer_open = true;
                self.is_settings_open = false;
                self.is_about_open = false;
                self.is_speed_test_open = false;
                self.is_timeline_open = false;
                self.is_audit_open = false;
                self.is_upnp_open = false;
                if self.wifi_analyzer_report.is_none() && !self.is_scanning_wifi {
                    return Task::done(Message::ScanWifi);
                }
                Task::none()
            }
            Message::CloseWifiAnalyzer => {
                self.is_wifi_analyzer_open = false;
                Task::none()
            }
            Message::ScanWifi => {
                self.is_scanning_wifi = true;
                Task::future(async move {
                    let res = WifiAnalyzer::scan().await;
                    Message::WifiScanCompleted(res)
                })
            }
            Message::WifiScanCompleted(res) => {
                self.is_scanning_wifi = false;
                match res {
                    Ok(report) => {
                        self.wifi_analyzer_report = Some(report);
                    }
                    Err(err) => {
                        self.notification = Some((format!("Aviso Wi-Fi: {}", err), false));
                    }
                }
                Task::none()
            }

            // Connectivity Monitoring (Uptime)
            Message::ConnectivityChecked { gw_ok, internet_ok } => {
                if let Some((evt_type, title, details)) = self.uptime_tracker.record_probe(gw_ok, internet_ok) {
                    if self.notification_settings.enabled && self.notification_settings.notify_internet_outages {
                        let icon = if evt_type == NetworkEventType::InternetOutage { "network-error" } else { "network-transmit-receive" };
                        let urgency = if evt_type == NetworkEventType::InternetOutage { Some(NotificationUrgency::Critical) } else { None };
                        self.notification_settings.dispatch(&title, &details, icon, urgency);
                    }
                    let event = HistoryManager::create_event(evt_type, title, details, None, None);
                    self.network_events.insert(0, event);
                    if self.network_events.len() > 150 {
                        self.network_events.truncate(150);
                    }
                    self.save_state();
                }
                Task::none()
            }

            Message::ClearDeviceRequestHistory(ip) => {
                if let Some(dev) = self.devices.iter_mut().find(|d| d.ip == ip) {
                    dev.request_history.clear();
                    Storage::update_device_requests(&mut self.device_metadata, &dev.mac, &[]);
                }
                self.save_state();
                self.notification = Some((Translations::device_requests_cleared_notif(self.language).to_string(), true));
                Task::none()
            }

            _ => Task::none(),
        }
    }
}
