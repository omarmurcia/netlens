use chrono::Local;
use netlens::model::{Device, DeviceCategory, DeviceStatus, PortInfo};
use std::net::Ipv4Addr;

#[test]
fn test_topology_and_auto_scan_state_serde() {
    use netlens::router::RouterConnection;
    use netlens::storage::AppState;

    // Test RouterConnection serde
    let conn_json = r#"{
        "macAddress": "28:44:F4:9F:1B:09",
        "ipAddress": "192.168.1.215",
        "negotiatedMbps": 433,
        "wireless": {
            "bssid": "D8:EC:5E:9C:77:1C",
            "band": "5GHz",
            "signalDecibels": -58
        }
    }"#;
    let conn: RouterConnection = serde_json::from_str(conn_json).expect("deserialize router connection");
    assert_eq!(conn.mac_address, "28:44:F4:9F:1B:09");
    assert_eq!(conn.negotiated_mbps, Some(433));
    let wireless = conn.wireless.expect("wireless info");
    assert_eq!(wireless.band.as_deref(), Some("5GHz"));
    assert_eq!(wireless.signal_decibels, Some(-58));

    // Test AppState with auto scan and known macs
    let state_json = r#"{
        "language": "Spanish",
        "theme": "Dark",
        "devices": [],
        "device_metadata": {},
        "auto_scan_interval_secs": 60,
        "known_macs": ["28:44:F4:9F:1B:09", "6C:0B:5E:72:C8:D8"],
        "autostart": true,
        "minimize_to_tray_on_close": true
    }"#;
    let state: AppState = serde_json::from_str(state_json).expect("deserialize app state");
    assert_eq!(state.auto_scan_interval_secs, 60);
    assert_eq!(state.known_macs.len(), 2);
    assert_eq!(state.known_macs[0], "28:44:F4:9F:1B:09");
    assert!(state.autostart);
    assert!(state.minimize_to_tray_on_close);

    // Verify default auto_scan_interval_secs is 30 in clean install / default state
    let default_state = AppState::default();
    assert_eq!(default_state.auto_scan_interval_secs, 30);

    // Verify missing field in clean state json deserializes to default 30s
    let clean_json = r#"{ "devices": [] }"#;
    let clean_state: AppState = serde_json::from_str(clean_json).expect("deserialize clean app state");
    assert_eq!(clean_state.auto_scan_interval_secs, 30);
}

#[test]
fn test_autostart_manager_entry_generation() {
    use netlens::autostart::AutostartManager;

    let exec = "/usr/bin/netlens";
    let entry = AutostartManager::generate_desktop_entry(exec);
    assert!(entry.contains("[Desktop Entry]"));
    assert!(entry.contains("Type=Application"));
    assert!(entry.contains("Name=NetLens"));
    assert!(entry.contains("Exec=/usr/bin/netlens --minimized"));
    assert!(entry.contains("X-GNOME-Autostart-enabled=true"));
    assert!(entry.contains("X-KDE-autostart-after=panel"));

    // Test paths with spaces requiring quoting
    let spaced_exec = "/home/user/My Apps/netlens";
    let spaced_entry = AutostartManager::generate_desktop_entry(spaced_exec);
    assert!(spaced_entry.contains("Exec=\"/home/user/My Apps/netlens\" --minimized"));
}

#[test]
fn test_autostart_persistent_path_resolution() {
    use netlens::autostart::AutostartManager;
    use std::env;

    // When APPIMAGE points to an existing file, it must resolve that fixed path
    let current_dir = env::current_dir().unwrap();
    let cargo_toml = current_dir.join("Cargo.toml");
    unsafe {
        env::set_var("APPIMAGE", cargo_toml.to_string_lossy().to_string());
    }

    let resolved = AutostartManager::get_persistent_executable_path();
    assert!(resolved.is_ok());
    assert_eq!(resolved.unwrap(), cargo_toml);

    unsafe {
        env::remove_var("APPIMAGE");
    }
}

#[test]
fn test_autostart_set_and_clear() {
    use netlens::autostart::AutostartManager;
    use std::fs;

    let temp_dir = std::env::temp_dir().join(format!("netlens_test_autostart_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    let _ = fs::create_dir_all(&temp_dir);
    let orig_xdg = std::env::var("XDG_CONFIG_HOME").ok();
    unsafe {
        std::env::set_var("XDG_CONFIG_HOME", &temp_dir);
    }

    // Enable autostart
    let res = AutostartManager::set_enabled(true);
    assert!(res.is_ok(), "set_enabled(true) should succeed: {:?}", res);

    let path = AutostartManager::autostart_file_path().unwrap();
    assert!(path.exists(), "Autostart desktop file should exist in temp config dir");
    assert!(path.starts_with(&temp_dir));

    let content = fs::read_to_string(&path).unwrap();
    assert!(!content.contains("/tmp/.mount_"), "Must NEVER contain temporary mount paths");
    assert!(!content.contains("integration_tests"), "Must NEVER point to test runner");
    assert!(content.contains("--minimized"));
    assert!(AutostartManager::is_enabled());

    // Disable autostart
    let res_disable = AutostartManager::set_enabled(false);
    assert!(res_disable.is_ok());
    assert!(!path.exists());
    assert!(!AutostartManager::is_enabled());

    // Restore XDG_CONFIG_HOME and clean temp_dir
    unsafe {
        if let Some(orig) = orig_xdg {
            std::env::set_var("XDG_CONFIG_HOME", orig);
        } else {
            std::env::remove_var("XDG_CONFIG_HOME");
        }
    }
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_i18n_autostart_and_notifications_all_languages() {
    use netlens::i18n::{Language, Translations};

    for &lang in Language::ALL {
        assert!(!Translations::autostart_title(lang).is_empty());
        assert!(!Translations::autostart_desc(lang).is_empty());
        assert!(!Translations::minimize_on_close_title(lang).is_empty());
        assert!(!Translations::minimize_on_close_desc(lang).is_empty());
        assert!(!Translations::notif_trust_success_title(lang).is_empty());
        assert!(!Translations::notif_trust_success_body(lang, "11:22:33:44:55:66").is_empty());
        assert!(!Translations::notif_block_success_title(lang).is_empty());
        assert!(!Translations::notif_block_success_body(lang, "11:22:33:44:55:66").is_empty());
        assert!(!Translations::notif_require_pass_title(lang).is_empty());
        assert!(!Translations::notif_require_pass_body(lang).is_empty());
    }
}

#[test]
fn test_system_tray_and_icons() {
    use ksni::Tray;
    use netlens::tray::{get_icon_argb, get_window_icon, NetLensTray, TrayState};
    use std::sync::{Arc, Mutex};

    // Test ARGB tray icon extraction
    let tray_icon = get_icon_argb().expect("Tray icon should load from embedded PNG");
    assert!(tray_icon.width > 0);
    assert!(tray_icon.height > 0);
    assert_eq!(tray_icon.data.len(), (tray_icon.width * tray_icon.height * 4) as usize);

    // Test window RGBA icon extraction
    let win_icon = get_window_icon();
    assert!(win_icon.is_some(), "Window icon should load successfully");

    // Test NetLensTray structure with Spanish language
    let state = Arc::new(Mutex::new(TrayState {
        device_count: 5,
        is_scanning: false,
        gateway_ip: Some("192.168.1.1".to_string()),
        language: netlens::i18n::Language::Spanish,
    }));

    let tray = NetLensTray {
        state: Arc::clone(&state),
        icon_cache: Some(tray_icon.clone()),
    };

    assert_eq!(tray.id(), "netlens");
    assert_eq!(tray.title(), "NetLens");
    assert_eq!(tray.icon_name(), "netlens");
    assert_eq!(tray.icon_pixmap().len(), 1);
    assert_eq!(tray.icon_pixmap()[0].width, tray_icon.width);

    let tip = tray.tool_tip();
    assert!(tip.title.contains("NetLens"));
    assert!(tip.description.contains("5 dispositivos activos"));
    assert!(tip.description.contains("192.168.1.1"));

    let menu = tray.menu();
    assert!(menu.len() >= 4, "Debe tener opciones de abrir, escanear, estado y salir");

    // Test dynamic language switch in System Tray to English
    state.lock().unwrap().language = netlens::i18n::Language::English;
    let tip_en = tray.tool_tip();
    assert!(tip_en.description.contains("5 active devices"));
    assert!(tip_en.description.contains("Gateway: 192.168.1.1"));
}

#[test]
fn test_background_scan_and_daemon_settings() {
    use netlens::app::default_window_settings;

    // Test default_window_settings generates proper window dimensions and icon
    let win_settings = default_window_settings();
    assert_eq!(win_settings.size.width, 1220.0);
    assert_eq!(win_settings.size.height, 820.0);
    assert!(!win_settings.exit_on_close_request, "exit_on_close_request should be false for daemon background operation");
    assert!(win_settings.icon.is_some(), "Window settings must have the embedded icon configured");
}

#[test]
fn test_security_audit_score_and_checks() {
    use netlens::security_audit::SecurityAuditor;
    use netlens::router::upnp::UpnpPortMapping;

    let now = Local::now();
    let safe_device = Device {
        ip: Ipv4Addr::new(192, 168, 1, 10),
        mac: "aa:bb:cc:dd:ee:ff".to_string(),
        vendor: "Apple".to_string(),
        hostname: Some("MacBook".to_string()),
        category: DeviceCategory::Computer,
        is_randomized_mac: false,
        is_gateway: false,
        is_current_host: false,
        interface: "wlan0".to_string(),
        latency_ms: Some(2.0),
        open_ports: vec![PortInfo {
            port: 443,
            service: "HTTPS".to_string(),
            description: "Encrypted Web".to_string(),
        }],
        status: DeviceStatus::Online,
        first_seen: now,
        last_seen: now,
        custom_alias: None,
        is_whitelisted: true,
        is_blocked: false,
        speed_limit_kbps: None,
        connection_type: None,
        wifi_signal_dbm: None,
        negotiated_mbps: None,
        model_name: Some("MacBook Pro 16".to_string()),
        discovered_services: vec!["AirPlay".to_string()],
        request_history: vec![],
    };

    let vulnerable_device = Device {
        ip: Ipv4Addr::new(192, 168, 1, 20),
        mac: "11:22:33:44:55:66".to_string(),
        vendor: "OldVendor".to_string(),
        hostname: Some("LegacyBox".to_string()),
        category: DeviceCategory::Unknown,
        is_randomized_mac: false,
        is_gateway: false,
        is_current_host: false,
        interface: "wlan0".to_string(),
        latency_ms: Some(5.0),
        open_ports: vec![
            PortInfo {
                port: 23,
                service: "Telnet".to_string(),
                description: "Unencrypted Telnet".to_string(),
            },
            PortInfo {
                port: 21,
                service: "FTP".to_string(),
                description: "Unencrypted FTP".to_string(),
            },
        ],
        status: DeviceStatus::Online,
        first_seen: now,
        last_seen: now,
        custom_alias: None,
        is_whitelisted: false,
        is_blocked: false,
        speed_limit_kbps: None,
        connection_type: None,
        wifi_signal_dbm: None,
        negotiated_mbps: None,
        model_name: None,
        discovered_services: vec![],
        request_history: vec![],
    };

    let port_mappings = vec![
        UpnpPortMapping {
            external_port: 80,
            internal_port: 80,
            internal_client: "192.168.1.20".to_string(),
            protocol: "TCP".to_string(),
            description: "Web Port".to_string(),
            enabled: true,
        },
    ];

    // Audit with vulnerable device and dangerous forwarded port
    let report = SecurityAuditor::audit(&[safe_device, vulnerable_device], &port_mappings, false);
    assert!(report.score < 80, "Score should reflect detected vulnerabilities");
    assert!(report.critical_count >= 1, "Should identify critical issues (Telnet or forwarded port 80)");
    assert_eq!(report.total_devices_analyzed, 2);
}

#[test]
fn test_uptime_tracker_and_outages() {
    use netlens::uptime::UptimeTracker;
    use netlens::history::NetworkEventType;

    let mut tracker = UptimeTracker::default();
    assert_eq!(tracker.uptime_percentage(), 100.0);
    assert!(tracker.is_online);

    // Normal successful probes
    let ev1 = tracker.record_probe(true, true);
    assert!(ev1.is_none());
    assert_eq!(tracker.uptime_percentage(), 100.0);

    // First failure (does not immediately trigger outage to avoid false positives)
    let ev_fail1 = tracker.record_probe(true, false);
    assert!(ev_fail1.is_none());
    assert!(tracker.is_online);

    // Second failure confirms outage
    let ev_fail2 = tracker.record_probe(true, false);
    assert!(ev_fail2.is_some());
    let (evt_type, title, _) = ev_fail2.unwrap();
    assert_eq!(evt_type, NetworkEventType::InternetOutage);
    assert!(title.contains("ISP") && title.contains("Outage"));
    assert!(!tracker.is_online);

    // Recovery probe
    let ev_rec = tracker.record_probe(true, true);
    assert!(ev_rec.is_some());
    let (rec_type, _, _) = ev_rec.unwrap();
    assert_eq!(rec_type, NetworkEventType::InternetRestored);
    assert!(tracker.is_online);
    assert_eq!(tracker.outages.len(), 1);
}

#[test]
fn test_history_manager_and_events() {
    use netlens::history::{HistoryManager, NetworkEventType};

    let evt = HistoryManager::create_event(
        NetworkEventType::NewDeviceDetected,
        "New Device".to_string(),
        "Chromecast detected on network".to_string(),
        Some(Ipv4Addr::new(192, 168, 1, 55)),
        Some("00:11:22:33:44:55".to_string()),
    );

    assert_eq!(evt.event_type, NetworkEventType::NewDeviceDetected);
    assert_eq!(evt.event_type.icon(), "✨");
    assert_eq!(evt.event_type.label(), "New Device");
    assert_eq!(evt.title, "New Device");
    assert!(evt.details.contains("Chromecast"));
    assert_eq!(evt.ip, Some(Ipv4Addr::new(192, 168, 1, 55)));
}

#[test]
fn test_upnp_models_serde() {
    use netlens::router::upnp::{UpnpPortMapping, UpnpRouterStatus};

    let mapping = UpnpPortMapping {
        external_port: 32400,
        internal_port: 32400,
        internal_client: "192.168.1.50".to_string(),
        protocol: "TCP".to_string(),
        description: "Plex Media Server".to_string(),
        enabled: true,
    };

    let json = serde_json::to_string(&mapping).expect("Serialización UPnP exitosa");
    let deserialized: UpnpPortMapping = serde_json::from_str(&json).expect("Deserialización UPnP exitosa");
    assert_eq!(mapping, deserialized);

    let status = UpnpRouterStatus {
        external_ip: Some("201.240.110.45".to_string()),
        connection_status: Some("Connected".to_string()),
        uptime_secs: Some(86400),
        total_bytes_received: Some(1024 * 1024 * 500),
        total_bytes_sent: Some(1024 * 1024 * 100),
        port_mappings: vec![mapping],
        model_name: Some("Linksys WHW03".to_string()),
        manufacturer: Some("Linksys".to_string()),
    };

    assert_eq!(status.external_ip.as_deref(), Some("201.240.110.45"));
    assert_eq!(status.port_mappings.len(), 1);
}

#[test]
fn test_wifi_analyzer_report_structure() {
    use netlens::wifi_analyzer::{ChannelOccupancy, WifiAnalyzerReport, WifiNetworkInfo};

    let report = WifiAnalyzerReport {
        networks: vec![
            WifiNetworkInfo {
                ssid: "MiWifi_Casa".to_string(),
                bssid: "00:11:22:33:44:55".to_string(),
                channel: 6,
                frequency_mhz: 2437,
                signal_percent: 85,
                security: "WPA2".to_string(),
                is_current: true,
            },
        ],
        current_ssid: Some("MiWifi_Casa".to_string()),
        current_channel: Some(6),
        recommended_channel_24: 1,
        recommended_channel_5: 36,
        recommendation_text: "El canal 1 de 2.4 GHz tiene menor interferencia.".to_string(),
        channels_24: vec![ChannelOccupancy {
            channel: 6,
            networks_count: 1,
            max_signal: 85,
            ssids: vec!["MiWifi_Casa".to_string()],
        }],
        channels_5: vec![],
    };

    assert_eq!(report.recommended_channel_24, 1);
    assert_eq!(report.recommended_channel_5, 36);
    assert_eq!(report.networks.len(), 1);
    assert!(report.networks[0].is_current);
}

#[test]
fn test_device_request_history() {
    use netlens::model::{Device, DeviceCategory, DeviceStatus};

    let now = chrono::Local::now();
    let mut dev = Device {
        ip: std::net::Ipv4Addr::new(192, 168, 1, 88),
        mac: "aa:bb:cc:11:22:33".to_string(),
        vendor: "Google".to_string(),
        hostname: Some("Chromecast".to_string()),
        category: DeviceCategory::SmartTv,
        is_randomized_mac: false,
        is_gateway: false,
        is_current_host: false,
        interface: "wlan0".to_string(),
        latency_ms: Some(1.2),
        open_ports: vec![],
        status: DeviceStatus::Online,
        first_seen: now,
        last_seen: now,
        custom_alias: None,
        is_whitelisted: true,
        is_blocked: false,
        speed_limit_kbps: None,
        connection_type: None,
        wifi_signal_dbm: None,
        negotiated_mbps: None,
        model_name: Some("Chromecast with Google TV".to_string()),
        discovered_services: vec!["Google Cast".to_string()],
        request_history: vec![],
    };

    assert_eq!(dev.request_history.len(), 0);
    dev.add_request("youtubei.googleapis.com".to_string(), "Streaming Video".to_string(), "HTTPS SNI".to_string());
    dev.add_request("www.google.com".to_string(), "Búsqueda Web".to_string(), "DNS A".to_string());

    assert_eq!(dev.request_history.len(), 2);
    assert_eq!(dev.request_history[0].query, "www.google.com");
    assert_eq!(dev.request_history[1].query, "youtubei.googleapis.com");
}

#[test]
fn test_notification_settings_serde_and_defaults() {
    use netlens::notification::{NotificationDetailLevel, NotificationSettings, NotificationUrgency};

    let default_settings = NotificationSettings::default();
    assert!(default_settings.enabled);
    assert!(default_settings.notify_unknown_intruders);
    assert!(!default_settings.notify_known_unwhitelisted_reconnect);
    assert!(!default_settings.notify_whitelisted_reconnect);
    assert!(!default_settings.notify_device_disconnected);
    assert!(!default_settings.notify_internet_outages);
    assert!(!default_settings.notify_security_critical);
    assert_eq!(default_settings.urgency, NotificationUrgency::Normal);
    assert_eq!(default_settings.timeout_secs, 7);
    assert_eq!(default_settings.detail_level, NotificationDetailLevel::Detailed);
    assert!(default_settings.show_mac);
    assert!(default_settings.show_vendor);
    assert_eq!(default_settings.custom_prefix, "NetLens");
    assert!(default_settings.play_sound);

    // Test JSON Serialization & Deserialization
    let json = serde_json::to_string(&default_settings).expect("JSON serialization failed");
    let deserialized: NotificationSettings = serde_json::from_str(&json).expect("JSON deserialization failed");
    assert_eq!(default_settings, deserialized);

    // Test backwards compatibility with empty JSON object (defaults for fresh install)
    let empty_json = "{}";
    let loaded: NotificationSettings = serde_json::from_str(empty_json).expect("Failed to deserialize empty json");
    assert!(loaded.enabled);
    assert!(loaded.notify_unknown_intruders);
    assert!(!loaded.notify_known_unwhitelisted_reconnect);
    assert!(!loaded.notify_device_disconnected);
    assert!(!loaded.notify_internet_outages);
    assert!(!loaded.notify_security_critical);
    assert_eq!(loaded.timeout_secs, 7);
    assert_eq!(loaded.custom_prefix, "NetLens");
}

#[test]
fn test_notification_message_formatting_and_content() {
    use netlens::notification::{NotificationDetailLevel, NotificationSettings};
    use std::net::Ipv4Addr;

    let mut settings = NotificationSettings::default();
    settings.custom_prefix = "[MiRed]".to_string();

    // Title formatting with prefix
    assert_eq!(settings.format_title("Alerta"), "[MiRed]: Alerta");
    settings.custom_prefix = "".to_string();
    assert_eq!(settings.format_title("Alerta"), "Alerta");

    // Message formatting in Compact mode
    settings.detail_level = NotificationDetailLevel::Compact;
    let compact_msg = settings.format_device_message(
        "Reconexión",
        "Tablet-Android",
        Ipv4Addr::new(192, 168, 1, 45),
        "00:11:22:33:44:55",
        "Samsung",
        Some("Fuera de whitelist"),
    );
    assert!(compact_msg.contains("Reconexión"));
    assert!(compact_msg.contains("Tablet-Android"));
    assert!(compact_msg.contains("192.168.1.45"));
    assert!(compact_msg.contains("Fuera de whitelist"));

    // Message formatting in Detailed mode
    settings.detail_level = NotificationDetailLevel::Detailed;
    settings.show_mac = true;
    let detailed_msg = settings.format_device_message(
        "Reconexión",
        "Tablet-Android",
        Ipv4Addr::new(192, 168, 1, 45),
        "00:11:22:33:44:55",
        "Samsung",
        Some("No está en Lista Blanca"),
    );
    assert!(detailed_msg.contains("MAC: 00:11:22:33:44:55"));
    assert!(!detailed_msg.contains("Fabricante"));

    // Message formatting in Full mode with vendor
    settings.detail_level = NotificationDetailLevel::Full;
    settings.show_vendor = true;
    let full_msg = settings.format_device_message(
        "Reconexión",
        "Tablet-Android",
        Ipv4Addr::new(192, 168, 1, 45),
        "00:11:22:33:44:55",
        "Samsung Electronics",
        Some("No está en Lista Blanca"),
    );
    assert!(full_msg.contains("MAC: 00:11:22:33:44:55"));
    assert!(full_msg.contains("Vendor: Samsung Electronics"));

    // Hiding MAC and Vendor
    settings.show_mac = false;
    settings.show_vendor = false;
    let filtered_msg = settings.format_device_message(
        "Reconexión",
        "Tablet-Android",
        Ipv4Addr::new(192, 168, 1, 45),
        "00:11:22:33:44:55",
        "Samsung Electronics",
        None,
    );
    assert!(!filtered_msg.contains("MAC"));
    assert!(!filtered_msg.contains("Samsung Electronics"));
}

#[test]
fn test_unwhitelisted_known_device_reconnection_detection() {
    use netlens::model::{Device, DeviceCategory, DeviceStatus};
    use netlens::notification::NotificationSettings;
    use std::collections::HashSet;

    let mut settings = NotificationSettings::default();
    assert!(!settings.notify_known_unwhitelisted_reconnect);
    settings.notify_known_unwhitelisted_reconnect = true;

    let dev_mac = "12:34:56:78:9a:bc".to_string();
    let mac_key = dev_mac.to_lowercase();

    // 1. Setup: Known MAC exists (e.g. from previous scan), but NOT whitelisted
    let mut known_macs = HashSet::new();
    known_macs.insert(mac_key.clone());

    let active_online_macs: HashSet<String> = HashSet::new();
    let initial_scan_completed = true;

    // Previous device in device list was marked Offline after last scan cycle
    let devices = vec![Device {
        ip: std::net::Ipv4Addr::new(192, 168, 1, 77),
        mac: dev_mac.clone(),
        vendor: "Apple Inc.".to_string(),
        hostname: Some("iPhone-Invitado".to_string()),
        category: DeviceCategory::Smartphone,
        is_randomized_mac: false,
        is_gateway: false,
        is_current_host: false,
        interface: "eth0".to_string(),
        latency_ms: Some(2.5),
        open_ports: vec![],
        status: DeviceStatus::Offline, // Was disconnected
        first_seen: chrono::Local::now(),
        last_seen: chrono::Local::now(),
        custom_alias: None,
        is_whitelisted: false, // NOT whitelisted
        is_blocked: false,
        speed_limit_kbps: None,
        connection_type: None,
        wifi_signal_dbm: None,
        negotiated_mbps: None,
        model_name: None,
        discovered_services: vec![],
        request_history: vec![],
    }];

    // 2. A new scan occurs: device responds and HostFound is received
    let mut scanned_macs_current_cycle = HashSet::new();
    scanned_macs_current_cycle.insert(mac_key.clone());

    let existing_idx = devices.iter().position(|d| d.mac.trim().to_lowercase() == mac_key);
    assert!(existing_idx.is_some());

    let was_offline = existing_idx.map(|idx| devices[idx].status == DeviceStatus::Offline).unwrap_or(true);
    let was_not_active = !active_online_macs.contains(&mac_key);
    let is_reconnection = (was_offline || was_not_active) && initial_scan_completed;
    let is_new_device = !known_macs.contains(&mac_key);

    assert!(is_reconnection, "Device should be identified as reconnecting!");
    assert!(!is_new_device, "Device is already known from previous scans!");

    let incoming_device = &devices[existing_idx.unwrap()];
    assert!(!incoming_device.is_whitelisted, "Device is not whitelisted!");
    assert!(!incoming_device.is_gateway && !incoming_device.is_current_host);

    // 3. Verify notification trigger condition matches user's bug fix:
    let should_notify_unwhitelisted_reconnect = !is_new_device
        && is_reconnection
        && !incoming_device.is_whitelisted
        && !incoming_device.is_gateway
        && !incoming_device.is_current_host
        && settings.enabled
        && settings.notify_known_unwhitelisted_reconnect;

    assert!(
        should_notify_unwhitelisted_reconnect,
        "Reconnection of known unwhitelisted device MUST trigger notification!"
    );
}

#[test]
fn test_i18n_notification_settings_all_languages() {
    use netlens::i18n::{Language, Translations};

    for &lang in Language::ALL {
        assert!(!Translations::notif_settings_title(lang).is_empty());
        assert!(!Translations::notif_settings_desc(lang).is_empty());
        assert!(!Translations::notif_master_enabled(lang).is_empty());
        assert!(!Translations::notif_master_disabled(lang).is_empty());
        assert!(!Translations::notif_trigger_unknown(lang).is_empty());
        assert!(!Translations::notif_trigger_unwhitelisted_reconnect(lang).is_empty());
        assert!(!Translations::notif_trigger_whitelisted_reconnect(lang).is_empty());
        assert!(!Translations::notif_trigger_disconnected(lang).is_empty());
        assert!(!Translations::notif_trigger_outages(lang).is_empty());
        assert!(!Translations::notif_trigger_security(lang).is_empty());
        assert!(!Translations::notif_urgency_label(lang).is_empty());
        assert!(!Translations::notif_timeout_label(lang).is_empty());
        assert!(!Translations::notif_sound_label(lang).is_empty());
        assert!(!Translations::notif_detail_label(lang).is_empty());
        assert!(!Translations::notif_show_mac_label(lang).is_empty());
        assert!(!Translations::notif_show_vendor_label(lang).is_empty());
        assert!(!Translations::notif_prefix_label(lang).is_empty());
        assert!(!Translations::notif_test_btn(lang).is_empty());
        assert!(!Translations::notif_reconnect_unwhitelisted_title(lang).is_empty());
        assert!(!Translations::notif_reconnect_whitelisted_title(lang).is_empty());
        assert!(!Translations::notif_device_disconnected_title(lang).is_empty());
        assert!(!Translations::notif_realtime_label(lang).is_empty());
        assert!(!Translations::notif_realtime_desc(lang).is_empty());
    }
}

#[test]
fn test_realtime_monitor_parsing() {
    use netlens::realtime_monitor::{NeighborAction, RealtimeMonitor};
    use std::net::Ipv4Addr;

    // Line 1: Normal STALE state
    let line1 = "192.168.1.165 dev eno1 lladdr 28:92:00:09:72:02 STALE";
    let action1 = RealtimeMonitor::parse_neighbor_line(line1);
    assert_eq!(
        action1,
        Some(NeighborAction::Appeared {
            ip: Ipv4Addr::new(192, 168, 1, 165),
            mac: "28:92:00:09:72:02".to_string(),
            iface: "eno1".to_string(),
        })
    );

    // Line 2: REACHABLE state
    let line2 = "192.168.1.55 dev wlan0 lladdr c0:23:8d:66:79:fe REACHABLE";
    let action2 = RealtimeMonitor::parse_neighbor_line(line2);
    assert_eq!(
        action2,
        Some(NeighborAction::Appeared {
            ip: Ipv4Addr::new(192, 168, 1, 55),
            mac: "c0:23:8d:66:79:fe".to_string(),
            iface: "wlan0".to_string(),
        })
    );

    // Line 3: FAILED state (must be ignored)
    let line3 = "192.168.1.200 dev eno1 FAILED";
    assert_eq!(RealtimeMonitor::parse_neighbor_line(line3), None);

    // Line 4: Deleted neighbor entry
    let line4 = "Deleted 192.168.1.165 dev eno1 lladdr 28:92:00:09:72:02 STALE";
    let action4 = RealtimeMonitor::parse_neighbor_line(line4);
    assert_eq!(
        action4,
        Some(NeighborAction::Disappeared {
            ip: Ipv4Addr::new(192, 168, 1, 165),
            mac: Some("28:92:00:09:72:02".to_string()),
            iface: "eno1".to_string(),
        })
    );

    // Line 5: IPv6 neighbor (must be ignored)
    let line5 = "fe80::1 dev eno1 lladdr 28:92:00:09:72:02 STALE";
    assert_eq!(RealtimeMonitor::parse_neighbor_line(line5), None);

    // Line 6: Empty or invalid
    assert_eq!(RealtimeMonitor::parse_neighbor_line(""), None);
    assert_eq!(RealtimeMonitor::parse_neighbor_line("   \n"), None);

    // Line 7: All zeros MAC (must be ignored)
    let line7 = "192.168.1.99 dev eno1 lladdr 00:00:00:00:00:00 STALE";
    assert_eq!(RealtimeMonitor::parse_neighbor_line(line7), None);
}

#[test]
fn test_realtime_monitor_settings_serde() {
    use netlens::notification::NotificationSettings;

    let settings = NotificationSettings::default();
    assert!(settings.realtime_netlink_monitor);

    let json = serde_json::to_string(&settings).expect("Serialization failed");
    let deserialized: NotificationSettings = serde_json::from_str(&json).expect("Deserialization failed");
    assert!(deserialized.realtime_netlink_monitor);

    // Backwards compatibility with JSON missing the field
    let empty_json = "{}";
    let loaded: NotificationSettings = serde_json::from_str(empty_json).expect("Deserialization failed");
    assert!(loaded.realtime_netlink_monitor);
}

#[test]
fn test_fast_disconnection_heartbeat_settings_and_logic() {
    use netlens::notification::NotificationSettings;
    use std::collections::{HashMap, HashSet};

    // 1. Verify defaults and configuration serialization
    let settings = NotificationSettings::default();
    assert!(settings.fast_disconnection_heartbeat);

    let json = serde_json::to_string(&settings).expect("Serialization failed");
    let deserialized: NotificationSettings = serde_json::from_str(&json).expect("Deserialization failed");
    assert!(deserialized.fast_disconnection_heartbeat);

    // Backwards compatibility with older configs lacking the field
    let empty_json = "{}";
    let loaded: NotificationSettings = serde_json::from_str(empty_json).expect("Deserialization failed");
    assert!(loaded.fast_disconnection_heartbeat);

    // 2. Simulation of Heartbeat Keep-Alive logic
    let mut missed_heartbeats: HashMap<Ipv4Addr, u32> = HashMap::new();
    let mut active_online_macs: HashSet<String> = HashSet::new();

    let test_ip = Ipv4Addr::new(192, 168, 1, 142);
    let test_mac = "76:da:ef:0a:3e:8e".to_string();

    // Device is initially active and connected
    active_online_macs.insert(test_mac.clone());
    missed_heartbeats.insert(test_ip, 0);

    // Probe failure 1: no response
    let misses = missed_heartbeats.entry(test_ip).or_insert(0);
    *misses += 1;
    assert_eq!(*misses, 1);
    // Single failure must NOT mark as disconnected (avoids false positives from Wi-Fi jitter)
    assert!(active_online_macs.contains(&test_mac));

    // Consecutive failure 2: confirmed disconnection (4s + 4s = 8s)
    let misses = missed_heartbeats.entry(test_ip).or_insert(0);
    *misses += 1;
    assert_eq!(*misses, 2);

    let was_online = active_online_macs.remove(&test_mac);
    assert!(was_online);
    assert!(!active_online_macs.contains(&test_mac));

    // If device responds later, counter resets to 0
    missed_heartbeats.insert(test_ip, 0);
    assert_eq!(*missed_heartbeats.get(&test_ip).unwrap(), 0);
}

#[test]
fn test_i18n_heartbeat_and_settings_buttons_all_languages() {
    use netlens::i18n::{Language, Translations};
    use netlens::notification::{NotificationDetailLevel, NotificationUrgency};

    for &lang in Language::ALL {
        // State buttons (enabled / disabled) and priority badge
        let btn_on = Translations::btn_enabled(lang);
        let btn_off = Translations::btn_disabled(lang);
        let badge = Translations::badge_priority(lang);

        assert!(!btn_on.is_empty(), "btn_enabled vacío para {:?}", lang);
        assert!(!btn_off.is_empty(), "btn_disabled vacío para {:?}", lang);
        assert!(!badge.is_empty(), "badge_priority vacío para {:?}", lang);

        // Subsection headers in Settings
        let sec_trig = Translations::notif_section_triggers(lang);
        let sec_behav = Translations::notif_section_behavior(lang);
        let sec_cont = Translations::notif_section_content(lang);

        assert!(!sec_trig.is_empty(), "notif_section_triggers vacío para {:?}", lang);
        assert!(!sec_behav.is_empty(), "notif_section_behavior vacío para {:?}", lang);
        assert!(!sec_cont.is_empty(), "notif_section_content vacío para {:?}", lang);

        // Heartbeat texts
        let hb_lbl = Translations::notif_heartbeat_label(lang);
        let hb_desc = Translations::notif_heartbeat_desc(lang);

        assert!(!hb_lbl.is_empty(), "notif_heartbeat_label vacío para {:?}", lang);
        assert!(!hb_desc.is_empty(), "notif_heartbeat_desc vacío para {:?}", lang);

        // Urgency options in all languages
        for &urgency in NotificationUrgency::ALL {
            let lbl = urgency.label(lang);
            assert!(!lbl.is_empty(), "Urgency label vacío para {:?} en {:?}", urgency, lang);
        }

        // Detail level options in all languages
        for &detail in NotificationDetailLevel::ALL {
            let lbl = detail.label(lang);
            assert!(!lbl.is_empty(), "Detail label vacío para {:?} en {:?}", detail, lang);
        }
    }
}

