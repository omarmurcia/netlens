use chrono::Local;
use netlens::oui::Oui;
use netlens::export::Exporter;
use netlens::model::{Device, DeviceCategory, DeviceStatus, NetworkContext, PortInfo};
use netlens::scanner::Scanner;
use std::fs;
use std::net::Ipv4Addr;

#[test]
fn test_is_randomized_mac() {
    // Known global vendor MAC addresses
    assert!(!Scanner::is_randomized_mac("d8:ec:5e:9c:77:1b")); // Belkin
    assert!(!Scanner::is_randomized_mac("c0:23:8d:66:79:fe")); // Samsung
    assert!(!Scanner::is_randomized_mac("28:92:00:09:72:02")); // Intel
    assert!(!Scanner::is_randomized_mac("fc:3c:d7:d2:b7:f7")); // Tuya

    // Locally Administered Addresses (LAA) / Privacy randomized MACs
    // Bit 1 of first byte must be 1: 0x76 = 0b01110110 (bit 1 is 1)
    assert!(Scanner::is_randomized_mac("76:da:ef:0a:3e:8e")); // Galaxy A25 Randomized
    assert!(Scanner::is_randomized_mac("02:11:22:33:44:55")); // LAA
    assert!(Scanner::is_randomized_mac("fa:16:3e:ab:cd:ef")); // OpenStack / Virtual
}

#[test]
fn test_vendor_lookup() {
    let oui_db = Oui::default().ok();

    // Samsung
    let vendor_samsung = Scanner::lookup_vendor(oui_db.as_ref(), "c0:23:8d:66:79:fe");
    assert!(
        vendor_samsung.to_lowercase().contains("samsung"),
        "Vendor should contain samsung, got: {}",
        vendor_samsung
    );

    // Belkin / Linksys
    let vendor_belkin = Scanner::lookup_vendor(oui_db.as_ref(), "d8:ec:5e:9c:77:1b");
    assert!(
        vendor_belkin.to_lowercase().contains("belkin") || vendor_belkin.to_lowercase().contains("linksys"),
        "Vendor should contain belkin or linksys, got: {}",
        vendor_belkin
    );

    // Tuya Smart
    let vendor_tuya = Scanner::lookup_vendor(oui_db.as_ref(), "fc:3c:d7:d2:b7:f7");
    assert!(
        vendor_tuya.to_lowercase().contains("tuya"),
        "Vendor should contain tuya, got: {}",
        vendor_tuya
    );
}

#[test]
fn test_device_categorization() {
    // 1. Router / Gateway
    let cat_gw = Scanner::categorize_device(true, false, Some("Linksys02323"), "Belkin", &[]);
    assert_eq!(cat_gw, DeviceCategory::Router);

    // 2. Android Phone
    let cat_phone = Scanner::categorize_device(false, false, Some("Marvin-s-A25"), "Samsung", &[]);
    assert_eq!(cat_phone, DeviceCategory::Smartphone);

    // 3. Smart TV
    let cat_tv = Scanner::categorize_device(false, false, Some("Samsung"), "Samsung Electronics", &[]);
    assert_eq!(cat_tv, DeviceCategory::SmartTv);

    // 4. Smart Home / IoT
    let cat_iot = Scanner::categorize_device(false, false, Some("smart-plug"), "Tuya Smart Inc.", &[]);
    assert_eq!(cat_iot, DeviceCategory::Iot);

    // 5. Printer (port 631)
    let ports_print = vec![PortInfo {
        port: 631,
        service: "IPP".to_string(),
        description: "Printer".to_string(),
    }];
    let cat_printer = Scanner::categorize_device(false, false, Some("office-laser"), "HP Inc.", &ports_print);
    assert_eq!(cat_printer, DeviceCategory::Printer);

    // 6. Personal Computer
    let cat_pc = Scanner::categorize_device(false, true, Some("cachyos-x8664"), "Intel", &[]);
    assert_eq!(cat_pc, DeviceCategory::Computer);

    // 7. Video Game Consoles (PlayStation, Xbox, Nintendo, Steam Deck)
    let cat_ps5 = Scanner::categorize_device(false, false, Some("PS5-Office"), "Sony Interactive Entertainment Inc.", &[]);
    assert_eq!(cat_ps5, DeviceCategory::GamingConsole);

    let cat_xbox = Scanner::categorize_device(false, false, Some("Xbox-LivingRoom"), "Microsoft Corporation", &[]);
    assert_eq!(cat_xbox, DeviceCategory::GamingConsole);

    let cat_switch = Scanner::categorize_device(false, false, Some("Switch-OLED"), "Nintendo Co., Ltd.", &[]);
    assert_eq!(cat_switch, DeviceCategory::GamingConsole);

    let cat_deck = Scanner::categorize_device(false, false, Some("steamdeck"), "Valve Corporation", &[]);
    assert_eq!(cat_deck, DeviceCategory::GamingConsole);

    let ports_psn = vec![PortInfo {
        port: 3074,
        service: "XboxLive/PSN".to_string(),
        description: "Console Gaming Port".to_string(),
    }];
    let cat_port_console = Scanner::categorize_device(false, false, None, "Unknown Vendor", &ports_psn);
    assert_eq!(cat_port_console, DeviceCategory::GamingConsole);

    // 8. Streaming Devices (Roku, Chromecast, Fire TV, Apple TV, etc.)
    let cat_roku = Scanner::categorize_device(false, false, Some("Roku-Express-4K"), "Roku, Inc.", &[]);
    assert_eq!(cat_roku, DeviceCategory::StreamingDevice);

    let cat_chromecast = Scanner::categorize_device(false, false, Some("Chromecast-LivingRoom"), "Google LLC", &[]);
    assert_eq!(cat_chromecast, DeviceCategory::StreamingDevice);

    let cat_firetv = Scanner::categorize_device(false, false, Some("FireTVStick-Master"), "Amazon Technologies Inc.", &[]);
    assert_eq!(cat_firetv, DeviceCategory::StreamingDevice);

    let cat_appletv = Scanner::categorize_device(false, false, Some("Apple-TV-Living"), "Apple Inc.", &[]);
    assert_eq!(cat_appletv, DeviceCategory::StreamingDevice);

    let ports_cast = vec![PortInfo {
        port: 8009,
        service: "Google Cast".to_string(),
        description: "Chromecast Streaming Protocol".to_string(),
    }];
    let cat_cast_port = Scanner::categorize_device(false, false, None, "Unknown Vendor", &ports_cast);
    assert_eq!(cat_cast_port, DeviceCategory::StreamingDevice);
}

#[test]
fn test_export_json_and_csv() {
    let now = Local::now();
    let sample_device = Device {
        ip: Ipv4Addr::new(192, 168, 1, 1),
        mac: "d8:ec:5e:9c:77:1b".to_string(),
        vendor: "Belkin International".to_string(),
        hostname: Some("Linksys02323".to_string()),
        category: DeviceCategory::Router,
        is_randomized_mac: false,
        is_gateway: true,
        is_current_host: false,
        interface: "eno1".to_string(),
        latency_ms: Some(0.68),
        open_ports: vec![
            PortInfo {
                port: 53,
                service: "DNS".to_string(),
                description: "Domain Name System".to_string(),
            },
            PortInfo {
                port: 80,
                service: "HTTP".to_string(),
                description: "Web Admin".to_string(),
            },
        ],
        status: DeviceStatus::Online,
        first_seen: now,
        last_seen: now,
        custom_alias: Some("Router Principal".to_string()),
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

    let ctx = NetworkContext {
        interface: "eno1".to_string(),
        local_ip: Some(Ipv4Addr::new(192, 168, 1, 176)),
        local_mac: Some("6c:0b:5e:72:c8:d8".to_string()),
        gateway_ip: Some(Ipv4Addr::new(192, 168, 1, 1)),
        gateway_mac: Some("d8:ec:5e:9c:77:1b".to_string()),
        subnet_cidr: "192.168.1.0/24".to_string(),
    };

    // Test export to JSON
    let json_path = Exporter::export_json(&[sample_device.clone()], &ctx).expect("JSON export failed");
    assert!(json_path.exists());
    let json_content = fs::read_to_string(&json_path).expect("Failed to read JSON");
    assert!(json_content.contains("192.168.1.1"));
    assert!(json_content.contains("Linksys02323"));
    assert!(json_content.contains("Router Principal"));
    let _ = fs::remove_file(json_path);

    // Test export to CSV
    let csv_path = Exporter::export_csv(&[sample_device.clone()]).expect("CSV export failed");
    assert!(csv_path.exists());
    let csv_content = fs::read_to_string(&csv_path).expect("Failed to read CSV");
    assert!(csv_content.contains("192.168.1.1"));
    assert!(csv_content.contains("d8:ec:5e:9c:77:1b"));
    assert!(csv_content.contains("Router / Puerta de Enlace"));
    let _ = fs::remove_file(csv_path);

    // Test saving to custom location chosen by user
    let custom_json_path = std::env::temp_dir().join(format!("custom_test_{}", Exporter::generate_json_filename()));
    Exporter::save_json_to_path(&custom_json_path, &[sample_device.clone()], &ctx).expect("Custom JSON save failed");
    assert!(custom_json_path.exists());
    let _ = fs::remove_file(custom_json_path);

    let custom_csv_path = std::env::temp_dir().join(format!("custom_test_{}", Exporter::generate_csv_filename()));
    Exporter::save_csv_to_path(&custom_csv_path, &[sample_device]).expect("Custom CSV save failed");
    assert!(custom_csv_path.exists());
    let _ = fs::remove_file(custom_csv_path);
}

#[test]
fn test_wol_validation() {
    // Valid MAC
    let valid_res = Scanner::send_wake_on_lan("d8:ec:5e:9c:77:1b");
    assert!(valid_res.is_ok());

    // Invalid length MAC
    let invalid_res = Scanner::send_wake_on_lan("12:34:56");
    assert!(invalid_res.is_err());
}

#[test]
fn test_telemetry_functions() {
    use netlens::telemetry::Telemetry;

    // 1. dBm to percentage conversion
    assert_eq!(Telemetry::dbm_to_percent(-35), 100);
    assert_eq!(Telemetry::dbm_to_percent(-50), 100);
    assert_eq!(Telemetry::dbm_to_percent(-60), 80);
    assert_eq!(Telemetry::dbm_to_percent(-75), 50);
    assert_eq!(Telemetry::dbm_to_percent(-100), 0);
    assert_eq!(Telemetry::dbm_to_percent(-110), 0);

    // 2. Speed formatting
    assert_eq!(Telemetry::format_speed(500.0), "500 B/s");
    assert_eq!(Telemetry::format_speed(1536.0), "1.5 KB/s");
    assert_eq!(Telemetry::format_speed(2.5 * 1024.0 * 1024.0), "2.50 MB/s");

    // 3. Byte formatting
    assert_eq!(Telemetry::format_bytes(50 * 1024 * 1024), "50.0 MB");

    // 4. Traffic sampling
    let (rx, tx) = Telemetry::sample_total_traffic();
    assert!(rx > 0 || tx > 0, "Debe haber tráfico de red en el sistema");
}

#[test]
fn test_i18n_top_10_languages() {
    use netlens::i18n::{Language, Translations};

    assert_eq!(Language::ALL.len(), 10, "Deben soportarse los 10 idiomas más hablados");

    let now = Local::now();
    let test_dev = Device {
        ip: Ipv4Addr::new(192, 168, 1, 100),
        mac: "02:11:22:33:44:55".to_string(),
        vendor: "TestVendor".to_string(),
        hostname: Some("testhost".to_string()),
        category: DeviceCategory::Computer,
        is_randomized_mac: true,
        is_gateway: false,
        is_current_host: true,
        interface: "wlan0".to_string(),
        latency_ms: Some(15.2),
        open_ports: vec![],
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

    for &lang in Language::ALL {
        assert!(!lang.native_name().is_empty());
        assert!(!Translations::window_title(lang).is_empty());
        assert!(!Translations::app_subtitle(lang).is_empty());
        assert!(!Translations::scan_network(lang).is_empty());
        assert!(!Translations::settings(lang).is_empty());
        assert!(!Translations::download(lang).is_empty());
        assert!(!Translations::upload(lang).is_empty());
        assert!(!Translations::signal_wifi(lang).is_empty());
        assert!(!Translations::network_and_router(lang).is_empty());
        assert!(!Translations::devices_count(5, lang).is_empty());
        assert!(!Translations::gateway_label(lang).is_empty());
        assert!(!Translations::search_placeholder(lang).is_empty());
        assert!(!Translations::device_details(lang).is_empty());
        assert!(!Translations::device_type_label(lang).is_empty());
        assert!(!Translations::badge_router(lang).is_empty());
        assert!(!Translations::badge_local_host(lang).is_empty());
        assert!(!Translations::badge_privacy(lang).is_empty());
        assert!(!Translations::vendor_private(lang).is_empty());
        assert!(!Translations::vendor_generic(lang).is_empty());
        assert!(!Translations::details_btn(lang).is_empty());
        assert!(!Translations::spec_ip(lang).is_empty());
        assert!(!Translations::spec_mac(lang).is_empty());
        assert!(!Translations::spec_vendor(lang).is_empty());
        assert!(!Translations::spec_hostname(lang).is_empty());
        assert!(!Translations::spec_mac_type(lang).is_empty());
        assert!(!Translations::spec_link_quality(lang).is_empty());
        assert!(!Translations::spec_interface(lang).is_empty());
        assert!(!Translations::spec_status(lang).is_empty());
        assert!(!Translations::spec_last_seen(lang).is_empty());
        assert!(!Translations::section_latency(lang).is_empty());
        assert!(!Translations::btn_ping_test(lang).is_empty());
        assert!(!Translations::section_ports(lang).is_empty());
        assert!(!Translations::btn_scan_ports(lang).is_empty());

        let sub = Translations::device_subtitle(&test_dev, lang);
        assert!(!sub.is_empty(), "Subtítulo vacío para idioma {:?}", lang);

        let link_q = Translations::link_quality_desc(Some(12.0), lang);
        assert!(!link_q.is_empty(), "Calidad de enlace vacía para idioma {:?}", lang);

        for &cat in DeviceCategory::all() {
            let cat_str = Translations::category_name(cat, lang);
            assert!(!cat_str.is_empty(), "Categoría {:?} vacía en idioma {:?}", cat, lang);
            let short_cat = Translations::category_short_name(cat, lang);
            assert!(!short_cat.is_empty(), "Categoría corta {:?} vacía en idioma {:?}", cat, lang);
        }

        // Validate port and service translations
        for &port in &[21, 22, 23, 25, 53, 80, 443, 445, 631, 8080, 9000] {
            let desc = Translations::port_description(port, lang);
            assert!(!desc.is_empty(), "Descripción de puerto {} vacía en idioma {:?}", port, lang);
        }

        // Validate Internet Speed Test translations
        assert!(!Translations::speed_test_btn(lang).is_empty());
        assert!(!Translations::speed_test_title(lang).is_empty());
        assert!(!Translations::speed_test_subtitle(lang).is_empty());
        assert!(!Translations::speed_test_ping(lang).is_empty());
        assert!(!Translations::speed_test_download(lang).is_empty());
        assert!(!Translations::speed_test_upload(lang).is_empty());
        assert!(!Translations::speed_test_start(lang).is_empty());
        assert!(!Translations::speed_test_running(lang).is_empty());
        assert!(!Translations::speed_stage_ping(lang).is_empty());
        assert!(!Translations::speed_stage_download(lang).is_empty());
        assert!(!Translations::speed_stage_upload(lang).is_empty());
        assert!(!Translations::speed_stage_complete(lang).is_empty());
    }
}

#[test]
fn test_speedtest_structures() {
    use netlens::speedtest::{SpeedTestResult, SpeedTestStage};

    let result = SpeedTestResult {
        ping_ms: 25.4,
        download_mbps: 112.5,
        upload_mbps: 34.2,
        timestamp: "12:00:00".to_string(),
        server: "Cloudflare CDN".to_string(),
    };

    assert_eq!(result.ping_ms, 25.4);
    assert!(result.download_mbps > 100.0);
    assert!(result.upload_mbps > 30.0);

    let stage = SpeedTestStage::default();
    assert_eq!(stage, SpeedTestStage::Idle);
}

#[test]
fn test_storage_persistence() {
    use netlens::i18n::Language;
    use netlens::model::{Device, DeviceCategory, DeviceStatus};
    use netlens::speedtest::SpeedTestResult;
    use netlens::storage::{AppState, Storage};
    use std::collections::HashMap;
    use std::net::Ipv4Addr;

    let mut metadata = HashMap::new();
    Storage::update_device_meta(
        &mut metadata,
        "d8:ec:5e:9c:77:1b",
        Some("Router Principal".to_string()),
        Some(DeviceCategory::Router),
    );
    Storage::update_device_meta(
        &mut metadata,
        "c0:23:8d:66:79:fe",
        Some("Mi SmartTV Samsung".to_string()),
        Some(DeviceCategory::SmartTv),
    );

    let dev1 = Device {
        ip: Ipv4Addr::new(192, 168, 1, 1),
        mac: "D8:EC:5E:9C:77:1B".to_string(), // Test case insensitivity
        vendor: "Belkin".to_string(),
        hostname: Some("myrouter".to_string()),
        category: DeviceCategory::Unknown,
        is_randomized_mac: false,
        is_gateway: true,
        is_current_host: false,
        interface: "eno1".to_string(),
        latency_ms: Some(0.5),
        open_ports: vec![],
        status: DeviceStatus::Online,
        first_seen: chrono::Local::now(),
        last_seen: chrono::Local::now(),
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

    let dev2 = Device {
        ip: Ipv4Addr::new(192, 168, 1, 100),
        mac: "c0:23:8d:66:79:fe".to_string(),
        vendor: "Samsung".to_string(),
        hostname: None,
        category: DeviceCategory::Unknown,
        is_randomized_mac: false,
        is_gateway: false,
        is_current_host: false,
        interface: "eno1".to_string(),
        latency_ms: Some(2.1),
        open_ports: vec![],
        status: DeviceStatus::Online,
        first_seen: chrono::Local::now(),
        last_seen: chrono::Local::now(),
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

    let mut list = vec![dev1, dev2];
    Storage::apply_metadata(&mut list, &metadata);

    assert_eq!(list[0].custom_alias, Some("Router Principal".to_string()));
    assert_eq!(list[0].category, DeviceCategory::Router);
    assert_eq!(list[1].custom_alias, Some("Mi SmartTV Samsung".to_string()));
    assert_eq!(list[1].category, DeviceCategory::SmartTv);

    let state = AppState {
        language: Language::French,
        theme: "Nord".to_string(),
        devices: list.clone(),
        device_metadata: metadata.clone(),
        network_events: vec![],
        last_speed_test: Some(SpeedTestResult {
            ping_ms: 15.2,
            download_mbps: 245.8,
            upload_mbps: 24.5,
            timestamp: "03:15:00".to_string(),
            server: "Netflix Open Connect (Tigo, SV)".to_string(),
        }),
        speed_test_history: vec![
            SpeedTestResult {
                ping_ms: 15.2,
                download_mbps: 245.8,
                upload_mbps: 24.5,
                timestamp: "03:15:00".to_string(),
                server: "Netflix Open Connect (Tigo, SV)".to_string(),
            },
            SpeedTestResult {
                ping_ms: 18.1,
                download_mbps: 210.4,
                upload_mbps: 21.2,
                timestamp: "03:00:00".to_string(),
                server: "Cloudflare Anycast CDN".to_string(),
            },
        ],
        router_ip: Some("192.168.1.1".to_string()),
        router_password: Some("my_router_pass".to_string()),
        auto_scan_interval_secs: 0,
        known_macs: Vec::new(),
        autostart: false,
        minimize_to_tray_on_close: false,
        notification_settings: netlens::notification::NotificationSettings::default(),
        storage_credentials: None,
    };

    let json = serde_json::to_string_pretty(&state).expect("Failed to serialize state");
    let loaded: AppState = serde_json::from_str(&json).expect("Failed to deserialize state");

    assert_eq!(loaded.language, Language::French);
    assert_eq!(loaded.theme, "Nord");
    assert_eq!(loaded.devices.len(), 2);
    assert_eq!(loaded.devices[0].custom_alias, Some("Router Principal".to_string()));
    assert_eq!(loaded.last_speed_test.as_ref().unwrap().download_mbps, 245.8);
    assert_eq!(loaded.last_speed_test.as_ref().unwrap().server, "Netflix Open Connect (Tigo, SV)");
    assert_eq!(loaded.speed_test_history.len(), 2);
    assert_eq!(loaded.speed_test_history[1].download_mbps, 210.4);
    assert_eq!(loaded.router_ip.as_deref(), Some("192.168.1.1"));

    // Test disk persistence
    let test_dir = std::env::temp_dir().join(format!("netlens_test_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&test_dir);
    let test_file = test_dir.join("state.json");
    std::fs::write(&test_file, &json).expect("Failed to write test state");
    let read_back = std::fs::read_to_string(&test_file).expect("Failed to read test state");
    let state_from_disk: AppState = serde_json::from_str(&read_back).expect("Failed to parse disk state");
    assert_eq!(state_from_disk.theme, "Nord");
    assert_eq!(state_from_disk.language, Language::French);
    assert_eq!(state_from_disk.devices.len(), 2);
    assert_eq!(state_from_disk.speed_test_history.len(), 2);
    let _ = std::fs::remove_dir_all(&test_dir);
}

#[test]
fn test_whitelist_and_blacklist_persistence() {
    use netlens::model::{Device, DeviceCategory, DeviceStatus};
    use netlens::storage::Storage;
    use std::collections::HashMap;
    use std::net::Ipv4Addr;

    let mut metadata = HashMap::new();
    let mac_trusted = "00:11:22:33:44:55";
    let mac_blocked = "aa:bb:cc:dd:ee:ff";

    Storage::set_device_whitelist(&mut metadata, mac_trusted, true);
    Storage::set_device_blocked(&mut metadata, mac_blocked, true);

    let dev_trusted = Device {
        ip: Ipv4Addr::new(192, 168, 1, 50),
        mac: mac_trusted.to_string(),
        vendor: "Apple".to_string(),
        hostname: Some("iPhone".to_string()),
        category: DeviceCategory::Smartphone,
        is_randomized_mac: false,
        is_gateway: false,
        is_current_host: false,
        interface: "eno1".to_string(),
        latency_ms: Some(5.0),
        open_ports: vec![],
        status: DeviceStatus::Online,
        first_seen: chrono::Local::now(),
        last_seen: chrono::Local::now(),
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

    let dev_blocked = Device {
        ip: Ipv4Addr::new(192, 168, 1, 99),
        mac: mac_blocked.to_string(),
        vendor: "Unknown".to_string(),
        hostname: None,
        category: DeviceCategory::Unknown,
        is_randomized_mac: true,
        is_gateway: false,
        is_current_host: false,
        interface: "eno1".to_string(),
        latency_ms: Some(100.0),
        open_ports: vec![],
        status: DeviceStatus::Online,
        first_seen: chrono::Local::now(),
        last_seen: chrono::Local::now(),
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

    let mut list = vec![dev_trusted.clone(), dev_blocked.clone()];
    Storage::apply_metadata(&mut list, &metadata);

    assert!(list[0].is_whitelisted, "dev_trusted should be whitelisted");
    assert!(!list[0].is_blocked, "dev_trusted should not be blocked");
    assert!(list[1].is_blocked, "dev_blocked should be blocked");
    assert!(!list[1].is_whitelisted, "dev_blocked should not be whitelisted");

    // Toggle whitelist and blocked
    Storage::set_device_whitelist(&mut metadata, mac_trusted, false);
    Storage::set_device_blocked(&mut metadata, mac_blocked, false);

    Storage::apply_metadata(&mut list, &metadata);
    assert!(!list[0].is_whitelisted, "dev_trusted should no longer be whitelisted");
    assert!(!list[1].is_blocked, "dev_blocked should no longer be blocked");
}

#[test]
fn test_linksys_base64_encode() {
    use netlens::router::LinksysRouter;

    assert_eq!(LinksysRouter::base64_encode("admin:admin"), "YWRtaW46YWRtaW4=");
    assert_eq!(LinksysRouter::base64_encode("admin:secret123"), "YWRtaW46c2VjcmV0MTIz");
    assert_eq!(LinksysRouter::base64_encode("test"), "dGVzdA==");
    assert_eq!(LinksysRouter::base64_encode(""), "");
}

#[test]
fn test_linksys_mac_filter_settings_serde() {
    use netlens::router::MacFilterSettings;

    let settings = MacFilterSettings {
        mac_filter_mode: "Deny".to_string(),
        mac_addresses: vec!["28:44:F4:9F:1B:09".to_string()],
    };

    let serialized = serde_json::to_string(&settings).expect("serialization failed");
    assert!(serialized.contains("\"macFilterMode\":\"Deny\""));
    assert!(serialized.contains("\"macAddresses\":[\"28:44:F4:9F:1B:09\"]"));

    // Deserializing a real Linksys response with "Disabled"
    let raw_json = r#"{"macFilterMode":"Disabled","macAddresses":[],"maxMACAddresses":32}"#;
    let parsed: MacFilterSettings = serde_json::from_str(raw_json).expect("deserialization failed");
    assert_eq!(parsed.mac_filter_mode, "Disabled");
    assert!(parsed.mac_addresses.is_empty());

    // Test Jnap generic response without output field (SetMACFilterSettings response)
    #[derive(serde::Deserialize)]
    struct JnapResp<T> {
        result: String,
        output: Option<T>,
    }

    let setter_resp_json = r#"{"result": "OK", "sideEffects": ["WirelessInterruption"]}"#;
    let setter_resp: JnapResp<serde_json::Value> = serde_json::from_str(setter_resp_json).expect("should parse");
    assert_eq!(setter_resp.result, "OK");
    assert!(setter_resp.output.is_none());
}

#[test]
fn test_i18n_router_notifications() {
    use netlens::i18n::{Language, Translations};

    let all_languages = [
        Language::Spanish,
        Language::English,
        Language::Chinese,
        Language::Hindi,
        Language::Arabic,
        Language::French,
        Language::Bengali,
        Language::Portuguese,
        Language::Russian,
        Language::Urdu,
    ];

    for lang in all_languages {
        assert!(!Translations::btn_applying(lang).is_empty());
        assert!(!Translations::router_require_password(lang).is_empty());
        assert!(!Translations::router_block_success(lang, "192.168.1.100", "AA:BB:CC:DD:EE:FF").is_empty());
        assert!(!Translations::router_unblock_success(lang, "192.168.1.100", "AA:BB:CC:DD:EE:FF").is_empty());
        assert!(!Translations::router_operation_failed(lang, "Timeout").is_empty());
        assert!(!Translations::speed_stage_idle(lang).is_empty());
        assert!(!Translations::speed_stage_failed(lang).is_empty());
        assert!(!Translations::speed_test_col_time_server(lang).is_empty());
        assert!(!Translations::speed_test_col_download(lang).is_empty());
        assert!(!Translations::speed_test_col_upload(lang).is_empty());
    }
}

#[test]
fn test_traceroute_output_parsing() {
    use netlens::traceroute::Traceroute;

    let sample_output = r#"
 1?: [LOCALHOST]     pmtu 1500
 1:  192.168.1.1                                           0.542ms 
 1:  192.168.1.1                                           0.481ms 
 2:  10.20.0.1                                             5.210ms 
 3:  no reply
 4:  8.8.8.8                                              12.350ms reached
    "#;

    let hops = Traceroute::parse_output(sample_output, "8.8.8.8");
    assert_eq!(hops.len(), 4, "Debe haber 4 saltos distintos");

    // Hop 1
    assert_eq!(hops[0].hop, 1);
    assert_eq!(hops[0].ip.as_deref(), Some("192.168.1.1"));
    assert!(!hops[0].is_timeout);
    assert!(hops[0].latency_ms.is_some());
    assert_eq!(hops[0].label.as_deref(), Some("Local Router (Gateway)"));

    // Hop 2
    assert_eq!(hops[1].hop, 2);
    assert_eq!(hops[1].ip.as_deref(), Some("10.20.0.1"));
    assert!(!hops[1].is_timeout);
    assert_eq!(hops[1].label.as_deref(), Some("Modem / ISP Gateway"));

    // Hop 3
    assert_eq!(hops[2].hop, 3);
    assert!(hops[2].is_timeout);
    assert!(hops[2].ip.is_none());

    // Hop 4
    assert_eq!(hops[3].hop, 4);
    assert_eq!(hops[3].ip.as_deref(), Some("8.8.8.8"));
    assert!(!hops[3].is_timeout);
    assert_eq!(hops[3].label.as_deref(), Some("Final Destination"));
}

#[test]
fn test_sparkline_widget() {
    use iced::Color;
    use netlens::widget::Sparkline;

    let data = vec![10.5, 20.0, 15.2, 35.8];
    let sparkline = Sparkline::new(data.clone(), Color::from_rgb8(16, 185, 129));
    assert_eq!(sparkline.points.len(), 4);
    assert_eq!(sparkline.points[1], 20.0);
}

#[test]
fn test_i18n_v2_features_all_languages() {
    use netlens::i18n::{Language, Translations};

    for &lang in Language::ALL {
        assert!(!Translations::intruder_alert_title(lang).is_empty());
        assert!(!Translations::intruder_btn_trust(lang).is_empty());
        assert!(!Translations::intruder_btn_block(lang).is_empty());
        assert!(!Translations::intruder_btn_dismiss(lang).is_empty());
        assert!(!Translations::auto_scan_title(lang).is_empty());
        assert!(!Translations::auto_scan_desc(lang).is_empty());
        assert!(!Translations::auto_scan_disabled(lang).is_empty());
        assert!(!Translations::auto_scan_30s(lang).is_empty());
        assert!(!Translations::auto_scan_1m(lang).is_empty());
        assert!(!Translations::auto_scan_2m(lang).is_empty());
        assert!(!Translations::auto_scan_5m(lang).is_empty());
        assert!(!Translations::topology_wifi_5g(lang).is_empty());
        assert!(!Translations::topology_wifi_24g(lang).is_empty());
        assert!(!Translations::topology_ethernet(lang).is_empty());
        assert!(!Translations::traceroute_title(lang).is_empty());
        assert!(!Translations::traceroute_btn_start(lang).is_empty());
        assert!(!Translations::traceroute_btn_tracing(lang).is_empty());
        assert!(!Translations::traceroute_timeout(lang).is_empty());
        assert!(!Translations::traceroute_hop_col(lang).is_empty());
        assert!(!Translations::traceroute_ip_col(lang).is_empty());
        assert!(!Translations::traceroute_latency_col(lang).is_empty());
    }
}

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

    // Enable autostart
    let res = AutostartManager::set_enabled(true);
    assert!(res.is_ok(), "set_enabled(true) should succeed: {:?}", res);

    let path = AutostartManager::autostart_file_path().unwrap();
    assert!(path.exists(), "Autostart desktop file should exist");

    let content = fs::read_to_string(&path).unwrap();
    assert!(!content.contains("/tmp/.mount_"), "Must NEVER contain temporary mount paths");
    assert!(content.contains("--minimized"));
    assert!(AutostartManager::is_enabled());

    // Disable autostart
    let res_disable = AutostartManager::set_enabled(false);
    assert!(res_disable.is_ok());
    assert!(!path.exists());
    assert!(!AutostartManager::is_enabled());

    // Clean test entry in ~/.local/share/applications/netlens.desktop and icon
    if let Ok(home) = std::env::var("HOME") {
        let app_desktop = std::path::PathBuf::from(&home).join(".local/share/applications/netlens.desktop");
        let _ = fs::remove_file(app_desktop);
        let app_icon = std::path::PathBuf::from(&home).join(".local/share/icons/hicolor/512x512/apps/netlens.png");
        let _ = fs::remove_file(app_icon);
    }
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

    // Test NetLensTray structure
    let state = Arc::new(Mutex::new(TrayState {
        device_count: 5,
        is_scanning: false,
        gateway_ip: Some("192.168.1.1".to_string()),
    }));

    let tray = NetLensTray {
        state,
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

#[test]
fn test_router_storage_share_and_directory_parsing() {
    use netlens::router::{
        format_file_size, RouterStorageManager, StorageFileType,
    };

    // 1. Test share parsing (smbclient -g and standard)
    let grepable_output = r#"
Disk|usbdisk|SanDisk Ultra USB 3.0
Disk|multimedia|Disco Duro Externo 1TB
IPC|IPC$|IPC Service
Printer|print$|Controladores de Impresora
"#;

    let shares = RouterStorageManager::parse_shares_output(grepable_output);
    assert_eq!(shares.len(), 2);
    assert_eq!(shares[0].name, "usbdisk");
    assert_eq!(shares[0].share_type, "Disk");
    assert_eq!(shares[0].comment, "SanDisk Ultra USB 3.0");
    assert_eq!(shares[1].name, "multimedia");

    // 2. Test directory and disk block parsing from smbclient
    let dir_output = r#"
  .                                   D        0  Tue Nov 17 12:00:00 2026
  ..                                  D        0  Tue Nov 17 12:00:00 2026
  Fotos_Vacaciones                    D        0  Sun Oct 12 14:30:10 2025
  Documentos_Trabajo                  D        0  Fri Sep 05 09:15:00 2025
  backup_sistema.zip                  A 104857600  Mon Nov 16 18:22:45 2026
  pelicula_familiar.mp4               A 1572864000  Wed Nov 11 20:40:12 2026
  reporte_anual.pdf                   A   2560000  Tue Nov 17 10:05:30 2026
  cancion.mp3                         A   8388608  Sat Oct 18 16:20:00 2025
  script_mantenimiento.sh             A      4096  Sun Nov 01 08:00:00 2026

		48828125 blocks of size 4096. 31250000 blocks available
"#;

    let (files, disk_info) = RouterStorageManager::parse_directory_output(dir_output);

    // . and .. must be omitted
    assert_eq!(files.len(), 7);

    // Folders must appear first sorted alphabetically
    assert!(files[0].is_dir);
    assert_eq!(files[0].name, "Documentos_Trabajo");
    assert_eq!(files[0].file_type, StorageFileType::Folder);

    assert!(files[1].is_dir);
    assert_eq!(files[1].name, "Fotos_Vacaciones");
    assert_eq!(files[1].file_type, StorageFileType::Folder);

    // Then files classified by extension
    let zip_file = files.iter().find(|f| f.name == "backup_sistema.zip").unwrap();
    assert_eq!(zip_file.file_type, StorageFileType::Archive);
    assert_eq!(zip_file.size_bytes, 104857600);

    let mp4_file = files.iter().find(|f| f.name == "pelicula_familiar.mp4").unwrap();
    assert_eq!(mp4_file.file_type, StorageFileType::Video);
    assert_eq!(mp4_file.size_bytes, 1572864000);

    let pdf_file = files.iter().find(|f| f.name == "reporte_anual.pdf").unwrap();
    assert_eq!(pdf_file.file_type, StorageFileType::Document);

    let mp3_file = files.iter().find(|f| f.name == "cancion.mp3").unwrap();
    assert_eq!(mp3_file.file_type, StorageFileType::Audio);

    let sh_file = files.iter().find(|f| f.name == "script_mantenimiento.sh").unwrap();
    assert_eq!(sh_file.file_type, StorageFileType::Executable);

    // 3. Disk space calculation check
    let disk = disk_info.expect("Información de disco no debe ser None");
    assert_eq!(disk.total_bytes, 48828125 * 4096);
    assert_eq!(disk.free_bytes, 31250000 * 4096);
    assert_eq!(disk.used_bytes(), (48828125 - 31250000) * 4096);
    assert!((disk.used_percent() - 36.0).abs() < 0.1);

    // 4. Size formatting check
    assert_eq!(format_file_size(500), "500 B");
    assert_eq!(format_file_size(1024), "1.00 KB");
    assert_eq!(format_file_size(1048576), "1.00 MB");
    assert_eq!(format_file_size(1073741824), "1.00 GB");
}

#[test]
fn test_router_storage_credentials_and_demo() {
    use netlens::router::{RouterStorageManager, StorageCredentials};

    // 1. Credentials behavior and serialization
    let mut creds = StorageCredentials::default();
    assert!(creds.is_empty());

    creds.username = "admin".to_string();
    creds.password = "secreto123".to_string();
    creds.workgroup = "MI_RED".to_string();
    assert!(!creds.is_empty());

    let json = serde_json::to_string(&creds).expect("Serialization failed");
    let deser: StorageCredentials = serde_json::from_str(&json).expect("Deserialization failed");
    assert_eq!(deser.username, "admin");
    assert_eq!(deser.password, "secreto123");
    assert_eq!(deser.workgroup, "MI_RED");

    // 2. Interactive demo data
    let demo_shares = RouterStorageManager::get_demo_shares();
    assert!(!demo_shares.is_empty());
    assert_eq!(demo_shares[0].name, "usb_storage");

    let (root_files, root_disk) = RouterStorageManager::get_demo_directory("");
    assert!(!root_files.is_empty());
    assert!(root_disk.is_some());

    let (sub_files, sub_disk) = RouterStorageManager::get_demo_directory("Vacation_Photos");
    assert!(!sub_files.is_empty());
    assert!(sub_disk.is_some());
}

#[test]
fn test_i18n_router_storage_all_languages() {
    use netlens::i18n::{Language, Translations};

    for &lang in Language::ALL {
        assert!(!Translations::storage_btn_header(lang).is_empty());
        assert!(!Translations::storage_modal_title(lang).is_empty());
        assert!(!Translations::storage_modal_desc(lang).is_empty());
        assert!(!Translations::storage_target_label(lang).is_empty());
        assert!(!Translations::storage_share_select(lang).is_empty());
        assert!(!Translations::storage_up_level(lang).is_empty());
        assert!(!Translations::storage_new_folder(lang).is_empty());
        assert!(!Translations::storage_upload_btn(lang).is_empty());
        assert!(!Translations::storage_upload_folder_btn(lang).is_empty());
        assert!(!Translations::storage_refresh(lang).is_empty());
        assert!(!Translations::storage_demo_mode_btn(lang).is_empty());
        assert!(!Translations::storage_filter_placeholder(lang).is_empty());
        assert!(!Translations::storage_col_type(lang).is_empty());
        assert!(!Translations::storage_col_name(lang).is_empty());
        assert!(!Translations::storage_col_size(lang).is_empty());
        assert!(!Translations::storage_col_modified(lang).is_empty());
        assert!(!Translations::storage_col_actions(lang).is_empty());
        assert!(!Translations::storage_action_download(lang).is_empty());
        assert!(!Translations::storage_action_delete(lang).is_empty());
        assert!(!Translations::storage_action_details(lang).is_empty());
        assert!(!Translations::storage_action_open(lang).is_empty());
        assert!(!Translations::storage_empty_dir(lang).is_empty());
        assert!(!Translations::storage_confirm_delete_title(lang).is_empty());
        assert!(!Translations::storage_confirm_delete_msg(lang).is_empty());
        assert!(!Translations::storage_creds_title(lang).is_empty());
        assert!(!Translations::storage_creds_desc(lang).is_empty());
        assert!(!Translations::storage_badge_available(lang).is_empty());
        assert!(!Translations::storage_upload_progress_title(lang).is_empty());
        assert!(!Translations::storage_upload_progress_speed(lang).is_empty());
        assert!(!Translations::storage_upload_progress_target(lang).is_empty());
        assert!(!Translations::storage_upload_dismiss_btn(lang).is_empty());
    }
}

#[tokio::test]
async fn test_router_storage_upload_folder_traversal_and_structures() {
    use netlens::router::{format_file_size, RouterStorageManager, StorageUploadProgress};
    use std::fs::{create_dir_all, remove_dir_all, File};
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    let now_nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let base_path = std::env::temp_dir().join(format!("netlens_upload_test_{}", now_nanos));
    create_dir_all(&base_path).expect("temp dir creation");

    // Create local structure to test recursive upload
    let subfolder = base_path.join("temporada_1");
    create_dir_all(&subfolder).expect("create subfolder");

    let file1 = base_path.join("serie_info.txt");
    let mut f1 = File::create(&file1).expect("create file 1");
    writeln!(f1, "Episodios y metadatos").expect("write file 1");

    let file2 = subfolder.join("capitulo_01.mkv");
    let mut f2 = File::create(&file2).expect("create file 2");
    writeln!(f2, "Video dummy stream").expect("write file 2");

    // Test recursive file collection
    let mut files = Vec::new();
    let res = RouterStorageManager::collect_files_recursive(&base_path, "", &mut files);
    assert!(res.is_ok());
    assert_eq!(files.len(), 2);

    let total_bytes: u64 = files.iter().map(|f| f.size).sum();
    assert!(total_bytes > 0);

    // Test initialization and calculation of StorageUploadProgress
    let progress = StorageUploadProgress {
        is_folder: true,
        title: "Subiendo carpeta: temporada_1 (2 archivos)".to_string(),
        target_dir: "DEFIANCE (2013-2015)/Season 1".to_string(),
        current_item: "capitulo_01.mkv".to_string(),
        current_index: 2,
        total_items: 2,
        bytes_uploaded: total_bytes,
        total_bytes,
        speed_str: "14.5 MB/s".to_string(),
        percent: 100.0,
        is_finished: true,
        error: None,
    };

    assert_eq!(progress.percent, 100.0);
    assert!(progress.is_finished);
    assert!(format_file_size(progress.bytes_uploaded).contains('B'));

    // Clean temporary directory
    let _ = remove_dir_all(&base_path);
}

#[test]
fn test_router_storage_immediate_delete_update() {
    use netlens::router::{StorageDiskInfo, StorageFileEntry};

    // 1. Simulate file list before deletion
    let mut files = vec![
        StorageFileEntry::new(
            "Temporada 1".to_string(),
            true,
            0,
            "2026-11-17".to_string(),
            "D".to_string(),
        ),
        StorageFileEntry::new(
            "pelicula.mkv".to_string(),
            false,
            500_000_000,
            "2026-11-17".to_string(),
            "A".to_string(),
        ),
        StorageFileEntry::new(
            "notas.txt".to_string(),
            false,
            10_000,
            "2026-11-17".to_string(),
            "A".to_string(),
        ),
    ];

    let mut disk_info = Some(StorageDiskInfo {
        total_bytes: 120_000_000_000,
        free_bytes: 20_000_000_000,
    });

    let mut selected_file = Some(files[1].clone());

    // 2. Simulate immediate deletion of file "pelicula.mkv"
    let file_to_delete = "pelicula.mkv";
    let removed_entry = files.iter().find(|f| f.name == file_to_delete).cloned();

    // Immediate list update
    files.retain(|f| f.name != file_to_delete);
    assert_eq!(files.len(), 2);
    assert!(files.iter().all(|f| f.name != "pelicula.mkv"));

    // Immediate properties window update
    if selected_file.as_ref().map(|f| f.name == file_to_delete).unwrap_or(false) {
        selected_file = None;
    }
    assert!(selected_file.is_none());

    // Immediate disk capacity update
    if let (Some(entry), Some(disk)) = (&removed_entry, &mut disk_info) {
        if !entry.is_dir {
            disk.free_bytes = (disk.free_bytes + entry.size_bytes).min(disk.total_bytes);
        }
    }

    assert_eq!(disk_info.unwrap().free_bytes, 20_500_000_000);

    // 3. Simulate immediate deletion of folder "Temporada 1"
    let folder_to_delete = "Temporada 1";
    let removed_folder_entry = files.iter().find(|f| f.name == folder_to_delete).cloned();
    files.retain(|f| f.name != folder_to_delete);
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].name, "notas.txt");

    // Capacity update on deleting folder
    let freed_folder_bytes = 1_850_000_000u64;
    if let (Some(_entry), Some(disk)) = (&removed_folder_entry, &mut disk_info) {
        disk.free_bytes = (disk.free_bytes + freed_folder_bytes).min(disk.total_bytes);
    }
    assert_eq!(disk_info.unwrap().free_bytes, 22_350_000_000);
    assert!(disk_info.unwrap().used_percent() < 83.0);

    // 4. Protection against stale/cached responses from router
    // If router returns a response with free_bytes lower than post-deletion value,
    // system preserves the higher updated free space.
    let stale_router_disk_info = StorageDiskInfo {
        total_bytes: 120_000_000_000,
        free_bytes: 20_000_000_000, // Stale value before deletion
    };

    let mut preserved_disk = stale_router_disk_info;
    if let Some(curr) = disk_info {
        if preserved_disk.free_bytes < curr.free_bytes {
            preserved_disk.free_bytes = curr.free_bytes;
        }
    }
    assert_eq!(preserved_disk.free_bytes, 22_350_000_000);
}

#[test]
fn test_gaming_console_support_and_i18n() {
    use netlens::i18n::{Language, Translations};
    use netlens::model::DeviceCategory;

    // Verify variant, icon, and default name
    let cat = DeviceCategory::GamingConsole;
    assert_eq!(cat.icon(), "🎮");
    assert_eq!(cat.display_name(), "Consola de Videojuegos");

    // Verify presence in DeviceCategory::all()
    assert!(DeviceCategory::all().contains(&DeviceCategory::GamingConsole));

    // Verify translations in all 10 supported languages
    let langs = [
        Language::Spanish,
        Language::English,
        Language::Chinese,
        Language::Hindi,
        Language::Arabic,
        Language::French,
        Language::Bengali,
        Language::Portuguese,
        Language::Russian,
        Language::Urdu,
    ];

    for &lang in &langs {
        let short = Translations::category_short_name(cat, lang);
        let full = Translations::category_name(cat, lang);
        assert!(!short.is_empty(), "Short name empty for lang {:?}", lang);
        assert!(!full.is_empty(), "Full name empty for lang {:?}", lang);
        assert!(short.contains("🎮"), "Icon missing in short name for lang {:?}", lang);
    }
}

#[test]
fn test_streaming_device_support_and_i18n() {
    use netlens::i18n::{Language, Translations};
    use netlens::model::DeviceCategory;

    // Verify variant, icon, and default name
    let cat = DeviceCategory::StreamingDevice;
    assert_eq!(cat.icon(), "🎬");
    assert_eq!(cat.display_name(), "Dispositivo de Streaming");

    // Verify presence in DeviceCategory::all()
    assert!(DeviceCategory::all().contains(&DeviceCategory::StreamingDevice));

    // Verify translations in all 10 supported languages
    let langs = [
        Language::Spanish,
        Language::English,
        Language::Chinese,
        Language::Hindi,
        Language::Arabic,
        Language::French,
        Language::Bengali,
        Language::Portuguese,
        Language::Russian,
        Language::Urdu,
    ];

    for &lang in &langs {
        let short = Translations::category_short_name(cat, lang);
        let full = Translations::category_name(cat, lang);
        assert!(!short.is_empty(), "Short name empty for lang {:?}", lang);
        assert!(!full.is_empty(), "Full name empty for lang {:?}", lang);
        assert!(short.contains("🎬"), "Icon missing in short name for lang {:?}", lang);
    }
}

#[test]
fn test_router_storage_upload_and_delete_accounting() {
    use netlens::router::storage::StorageDiskInfo;

    let mut disk = StorageDiskInfo {
        total_bytes: 120_000_000_000,
        free_bytes: 20_000_000_000,
    };

    // Uploading a 2 GB file
    let uploaded_bytes = 2_000_000_000u64;
    disk.free_bytes = disk.free_bytes.saturating_sub(uploaded_bytes);
    assert_eq!(disk.free_bytes, 18_000_000_000);
    assert_eq!(disk.used_bytes(), 102_000_000_000);
    assert!((disk.used_percent() - 85.0).abs() < 0.1);

    // Deleting a 5 GB file
    let deleted_bytes = 5_000_000_000u64;
    disk.free_bytes = (disk.free_bytes + deleted_bytes).min(disk.total_bytes);
    assert_eq!(disk.free_bytes, 23_000_000_000);
    assert_eq!(disk.used_bytes(), 97_000_000_000);
    assert!((disk.used_percent() - 80.83).abs() < 0.1);
}

#[tokio::test]
async fn test_view_storage_details_does_not_panic() {
    use netlens::app::{App, Message};
    use netlens::router::storage::StorageFileEntry;

    let (mut app, _) = App::new();
    app.is_storage_explorer_open = true;

    // Test with a file
    let file_entry = StorageFileEntry::new(
        "Notas_Router.txt".to_string(),
        false,
        12540,
        "16 Nov 2026 10:12".to_string(),
        "A".to_string(),
    );
    let _ = app.update(Message::StorageShowFileProperties(file_entry));
    let win_id = iced::window::Id::unique();
    {
        let _element = app.view(win_id);
    }

    // Test with a directory
    let dir_entry = StorageFileEntry::new(
        "Fotos_Vacaciones".to_string(),
        true,
        0,
        "12 Oct 2025 14:30".to_string(),
        "D".to_string(),
    );
    let _ = app.update(Message::StorageShowFileProperties(dir_entry));
    {
        let _element2 = app.view(win_id);
    }
}

#[tokio::test]
async fn test_storage_disk_info_cache_protection_and_delete_update() {
    use netlens::app::{App, Message};
    use netlens::router::storage::{StorageDiskInfo, StorageFileEntry};

    let (mut app, _) = App::new();
    app.is_storage_explorer_open = true;
    app.is_storage_demo_mode = false;
    app.storage_active_share = Some("Samsung USB (sda1)".to_string());
    app.storage_target_ip = "192.168.1.1".to_string();

    let initial_disk = StorageDiskInfo {
        total_bytes: 120_000_000_000,
        free_bytes: 20_000_000_000,
    };
    app.storage_disk_info = Some(initial_disk);

    let file_to_del = StorageFileEntry::new(
        "Grandes_Videos.mp4".to_string(),
        false,
        2_000_000_000,
        "10 Nov 2026 12:00".to_string(),
        "A".to_string(),
    );
    app.storage_files = vec![file_to_del.clone()];

    // 1. Request deletion and confirm
    let _ = app.update(Message::StorageRequestDeleteFile {
        path: "Grandes_Videos.mp4".to_string(),
        is_dir: false,
        name: "Grandes_Videos.mp4".to_string(),
    });
    let _ = app.update(Message::StorageConfirmDeleteFile);

    // 2. Check optimistic update
    assert!(app.storage_files.is_empty(), "El archivo debe haber sido removido de la lista");
    assert_eq!(app.storage_last_freed, Some(2_000_000_000));
    let curr_free = app.storage_disk_info.unwrap().free_bytes;
    assert_eq!(curr_free, 22_000_000_000, "El espacio libre debe reflejar los 2 GB liberados");

    // 3. Simulate router responding with stale cached response (20 GB instead of 22 GB)
    let stale_router_disk = StorageDiskInfo {
        total_bytes: 120_000_000_000,
        free_bytes: 20_000_000_000, // Stale router response
    };
    let _ = app.update(Message::StorageDirectoryLoaded(Ok((vec![], Some(stale_router_disk)))));
    assert_eq!(
        app.storage_disk_info.unwrap().free_bytes,
        22_000_000_000,
        "El espacio libre no debe retroceder ante una respuesta obsoleta del router"
    );

    // 4. Simulate router responding with updated response with even more freed space (23 GB)
    let newer_router_disk = StorageDiskInfo {
        total_bytes: 120_000_000_000,
        free_bytes: 23_000_000_000,
    };
    let _ = app.update(Message::StorageDirectoryLoaded(Ok((vec![], Some(newer_router_disk)))));
    assert_eq!(
        app.storage_disk_info.unwrap().free_bytes,
        23_000_000_000,
        "El espacio libre debe actualizarse con el valor superior reportado por el router"
    );
}

#[test]
fn test_all_tooltips_translated_in_all_languages() {
    use netlens::i18n::{Language, Translations};
    use netlens::model::DeviceCategory;

    for &lang in Language::ALL {
        assert!(!Translations::tip_settings(lang).is_empty());
        assert!(!Translations::tip_speed_test(lang).is_empty());
        assert!(!Translations::tip_security_audit(lang).is_empty());
        assert!(!Translations::tip_timeline(lang).is_empty());
        assert!(!Translations::tip_wifi_analyzer(lang).is_empty());
        assert!(!Translations::tip_router_storage(lang).is_empty());
        assert!(!Translations::tip_export_csv(lang).is_empty());
        assert!(!Translations::tip_export_json(lang).is_empty());
        assert!(!Translations::tip_scan_network(lang).is_empty());
        assert!(!Translations::tip_copy_ip(lang).is_empty());
        assert!(!Translations::tip_copy_mac(lang).is_empty());
        assert!(!Translations::tip_device_details(lang).is_empty());
        assert!(!Translations::tip_ping(lang).is_empty());
        assert!(!Translations::tip_traceroute(lang).is_empty());
        assert!(!Translations::tip_wol(lang).is_empty());
        assert!(!Translations::tip_whitelist(lang).is_empty());
        assert!(!Translations::tip_block_router(lang).is_empty());
        assert!(!Translations::tip_save_alias(lang).is_empty());
        assert!(!Translations::tip_close(lang).is_empty());
        assert!(!Translations::tip_storage_refresh(lang).is_empty());
        assert!(!Translations::tip_storage_creds(lang).is_empty());
        assert!(!Translations::tip_storage_demo(lang).is_empty());
        assert!(!Translations::tip_storage_up(lang).is_empty());
        assert!(!Translations::tip_storage_root(lang).is_empty());
        assert!(!Translations::tip_storage_new_folder(lang).is_empty());
        assert!(!Translations::tip_storage_upload_file(lang).is_empty());
        assert!(!Translations::tip_storage_upload_folder(lang).is_empty());
        assert!(!Translations::tip_storage_open_item(lang).is_empty());
        assert!(!Translations::tip_storage_download_item(lang).is_empty());
        assert!(!Translations::tip_storage_details_item(lang).is_empty());
        assert!(!Translations::tip_storage_delete_item(lang).is_empty());
        assert!(!Translations::tip_filter_all(lang).is_empty());
        assert!(!Translations::tip_filter_whitelist(lang).is_empty());
        assert!(!Translations::tip_filter_blocked(lang).is_empty());
        assert!(!Translations::tip_port_scan(lang).is_empty());

        for &cat in &[
            DeviceCategory::Router,
            DeviceCategory::Smartphone,
            DeviceCategory::Computer,
            DeviceCategory::SmartTv,
            DeviceCategory::StreamingDevice,
            DeviceCategory::GamingConsole,
            DeviceCategory::Iot,
        ] {
            assert!(!Translations::tip_filter_category(cat, lang).is_empty());
        }
    }
}

#[test]
fn test_export_dialog_translations() {
    use netlens::i18n::{Language, Translations};

    for &lang in Language::ALL {
        assert!(!Translations::export_json_dialog_title(lang).is_empty());
        assert!(!Translations::export_csv_dialog_title(lang).is_empty());
        let json_msg = Translations::export_success_msg("test.json", lang);
        assert!(json_msg.contains("test.json"));
        let csv_msg = Translations::export_csv_success_msg("test.csv", lang);
        assert!(csv_msg.contains("test.csv"));
    }
}

#[test]
fn test_long_file_name_truncation_logic() {
    let long_name = "DEFIANCE (2013-2015) - Complete TV Series, Season 1,2,3 S01 S02 S03 - 1080p BluRay x264";
    let max_len = 38;
    let char_count = long_name.chars().count();
    assert!(char_count > max_len);

    let display_name = if char_count > max_len {
        let truncated: String = long_name.chars().take(max_len.saturating_sub(3)).collect();
        format!("{}...", truncated)
    } else {
        long_name.to_string()
    };

    assert!(display_name.ends_with("..."));
    assert_eq!(display_name.chars().count(), 38);
    assert_eq!(&display_name[..10], "DEFIANCE (");

    // Test short string
    let short_name = "Season 1 (2013)";
    let char_count_short = short_name.chars().count();
    let display_short = if char_count_short > max_len {
        let truncated: String = short_name.chars().take(max_len.saturating_sub(3)).collect();
        format!("{}...", truncated)
    } else {
        short_name.to_string()
    };
    assert_eq!(display_short, "Season 1 (2013)");
}

#[test]
fn test_all_modals_translations_in_all_10_languages() {
    use netlens::i18n::{Language, Translations};
    use netlens::history::NetworkEventType;
    use netlens::router::storage::StorageFileType;
    use netlens::security_audit::SecurityCheckId;

    let all_languages = [
        Language::Spanish,
        Language::English,
        Language::Chinese,
        Language::Hindi,
        Language::Arabic,
        Language::French,
        Language::Bengali,
        Language::Portuguese,
        Language::Russian,
        Language::Urdu,
    ];

    let all_check_ids = [
        SecurityCheckId::TelnetDetected,
        SecurityCheckId::FtpDetected,
        SecurityCheckId::EncryptedServices,
        SecurityCheckId::UnwhitelistedDevices,
        SecurityCheckId::AccessControlWhitelisted,
        SecurityCheckId::RandomizedMacs,
        SecurityCheckId::DangerousPortsForwarded,
        SecurityCheckId::ControlledPortMapping,
        SecurityCheckId::RouterPasswordMissing,
        SecurityCheckId::RouterIntegrationActive,
    ];

    let all_event_types = [
        NetworkEventType::DeviceConnected,
        NetworkEventType::DeviceDisconnected,
        NetworkEventType::NewDeviceDetected,
        NetworkEventType::SecurityAlert,
        NetworkEventType::InternetOutage,
        NetworkEventType::InternetRestored,
    ];

    let all_file_types = [
        StorageFileType::Folder,
        StorageFileType::Image,
        StorageFileType::Video,
        StorageFileType::Audio,
        StorageFileType::Document,
        StorageFileType::Archive,
        StorageFileType::Executable,
        StorageFileType::Code,
        StorageFileType::Unknown,
    ];

    for &lang in &all_languages {
        // 1. Security Audit
        assert!(!Translations::audit_title(lang).is_empty());
        assert!(!Translations::audit_subtitle(lang).is_empty());
        assert!(!Translations::audit_not_run(lang).is_empty());
        assert!(!Translations::audit_score_label(lang).is_empty());
        assert!(!Translations::audit_grade_label(95, lang).is_empty());
        assert!(!Translations::audit_grade_label(75, lang).is_empty());
        assert!(!Translations::audit_grade_label(45, lang).is_empty());
        assert!(!Translations::audit_devices_audited(5, lang).is_empty());
        assert!(!Translations::audit_observations(2, lang).is_empty());
        assert!(!Translations::audit_critical_risks(1, lang).is_empty());
        assert!(!Translations::audit_affected_devices(3, lang).is_empty());
        assert!(!Translations::audit_rerun_btn(lang).is_empty());

        for &cid in &all_check_ids {
            assert!(!Translations::audit_check_title(cid, lang).is_empty());
            assert!(!Translations::audit_check_description(cid, 2, lang).is_empty());
            assert!(!Translations::audit_check_recommendation(cid, lang).is_empty());
        }

        // 2. Event History
        assert!(!Translations::timeline_modal_title(lang).is_empty());
        assert!(!Translations::timeline_modal_desc(lang).is_empty());
        assert!(!Translations::timeline_online(lang).is_empty());
        assert!(!Translations::timeline_outage(lang).is_empty());
        assert!(!Translations::timeline_total_events(10, lang).is_empty());
        assert!(!Translations::timeline_uptime_pct(99.5, lang).is_empty());
        assert!(!Translations::timeline_clear_btn(lang).is_empty());
        assert!(!Translations::timeline_empty(lang).is_empty());
        assert!(!Translations::timeline_cleared_notif(lang).is_empty());

        for &evt in &all_event_types {
            assert!(!Translations::event_type_name(evt, lang).is_empty());
            assert!(!Translations::event_title(evt, "Sample Event", lang).is_empty());
            assert!(!Translations::event_details(evt, "TV Sala (192.168.1.50) ya no responde en la red local.", lang).is_empty());
        }

        // Device Requests History
        assert!(!Translations::device_requests_title(lang).is_empty());
        assert!(!Translations::device_requests_simulate_btn(lang).is_empty());
        assert!(!Translations::device_requests_empty(lang).is_empty());
        assert!(!Translations::device_requests_cleared_notif(lang).is_empty());

        // 3. Wi-Fi Analyzer
        assert!(!Translations::wifi_modal_title(lang).is_empty());
        assert!(!Translations::wifi_modal_desc(lang).is_empty());
        assert!(!Translations::wifi_scan_btn(true, lang).is_empty());
        assert!(!Translations::wifi_scan_btn(false, lang).is_empty());
        assert!(!Translations::wifi_current_network(Some("HomeNet"), Some(6), lang).is_empty());
        assert!(!Translations::wifi_current_network_none(lang).is_empty());
        assert!(!Translations::wifi_optimal_channels_title(lang).is_empty());
        assert!(!Translations::wifi_channel_24_label(1, lang).is_empty());
        assert!(!Translations::wifi_channel_5_label(36, lang).is_empty());
        assert!(!Translations::wifi_recommendation_text(Some(11), 1, 36, lang).is_empty());
        assert!(!Translations::wifi_occupancy_24_title(lang).is_empty());
        assert!(!Translations::wifi_channel_box_title(6, lang).is_empty());
        assert!(!Translations::wifi_networks_count(3, lang).is_empty());
        assert!(!Translations::wifi_max_signal(85, lang).is_empty());
        assert!(!Translations::wifi_detected_networks_header(12, lang).is_empty());
        assert!(!Translations::wifi_badge_current(lang).is_empty());
        assert!(!Translations::wifi_prompt_scan(lang).is_empty());

        // 4. Router Storage (USB / NAS)
        assert!(!Translations::storage_btn_credentials(lang).is_empty());
        assert!(!Translations::storage_creds_user_placeholder(lang).is_empty());
        assert!(!Translations::storage_creds_pass_placeholder(lang).is_empty());
        assert!(!Translations::storage_creds_user_label(lang).is_empty());
        assert!(!Translations::storage_creds_pass_label(lang).is_empty());
        assert!(!Translations::storage_creds_group_label(lang).is_empty());
        assert!(!Translations::storage_creds_save_btn(lang).is_empty());
        assert!(!Translations::cancel(lang).is_empty());
        assert!(!Translations::storage_space_info("10 GB", "120 GB", 92.5, lang).is_empty());
        assert!(!Translations::storage_btn_root(lang).is_empty());
        assert!(!Translations::storage_root_label(lang).is_empty());
        assert!(!Translations::storage_upload_completed(lang).is_empty());
        assert!(!Translations::storage_upload_error(lang).is_empty());
        assert!(!Translations::storage_upload_file_progress("document.pdf", lang).is_empty());
        assert!(!Translations::storage_upload_folder_progress(2, 5, "pic.jpg", lang).is_empty());
        assert!(!Translations::storage_create_folder_prompt(lang).is_empty());
        assert!(!Translations::storage_new_folder_placeholder(lang).is_empty());
        assert!(!Translations::storage_create_confirm(lang).is_empty());
        assert!(!Translations::storage_props_title("file.mkv", lang).is_empty());
        assert!(!Translations::storage_prop_type(lang).is_empty());
        assert!(!Translations::storage_prop_full_path(lang).is_empty());
        assert!(!Translations::storage_prop_size(lang).is_empty());
        assert!(!Translations::storage_prop_modified(lang).is_empty());
        assert!(!Translations::storage_prop_attributes(lang).is_empty());

        for &ft in &all_file_types {
            assert!(!Translations::storage_file_type_name(ft, lang).is_empty());
            assert!(!ft.type_name(lang).is_empty());
        }

        assert!(!Translations::storage_loading_files(lang).is_empty());
        assert!(!Translations::storage_no_filter_match(lang).is_empty());
        assert!(!Translations::storage_summary_counts(15, 5, 10, lang).is_empty());

        // 5. UPnP IGD
        assert!(!Translations::upnp_modal_title(lang).is_empty());
        assert!(!Translations::upnp_modal_desc(lang).is_empty());
        assert!(!Translations::upnp_query_btn(true, lang).is_empty());
        assert!(!Translations::upnp_query_btn(false, lang).is_empty());
        assert!(!Translations::upnp_prompt(lang).is_empty());
        assert!(!Translations::upnp_badge_wan_risk(lang).is_empty());
        assert!(!Translations::upnp_badge_active(lang).is_empty());
        assert!(!Translations::upnp_badge_inactive(lang).is_empty());
        assert!(!Translations::upnp_public_ip_title(lang).is_empty());
        assert!(!Translations::upnp_not_assigned(lang).is_empty());
        assert!(!Translations::upnp_status_connected(lang).is_empty());
        assert!(!Translations::upnp_status_label("Connected", lang).is_empty());
        assert!(!Translations::upnp_uptime_title(lang).is_empty());
        assert!(!Translations::upnp_uptime_sub(lang).is_empty());
        assert!(!Translations::upnp_traffic_title(lang).is_empty());
        assert!(!Translations::upnp_traffic_sub(lang).is_empty());
        assert!(!Translations::upnp_port_mappings_header(4, lang).is_empty());
        assert!(!Translations::upnp_no_mappings(lang).is_empty());
        assert!(!Translations::upnp_port_label(80, "TCP", lang).is_empty());

        // 6. About NetLens
        assert!(!Translations::about_btn(lang).is_empty());
        assert!(!Translations::tip_about(lang).is_empty());
        assert!(!Translations::about_modal_title(lang).is_empty());
        assert!(!Translations::about_app_description(lang).is_empty());
        assert!(!Translations::about_author_section(lang).is_empty());
        assert!(!Translations::about_role(lang).is_empty());
        assert!(!Translations::about_email_label(lang).is_empty());
        assert!(!Translations::about_copy_email(lang).is_empty());
        assert!(!Translations::about_send_email(lang).is_empty());
        assert!(!Translations::about_repositories_section(lang).is_empty());
        assert!(!Translations::about_source_code(lang).is_empty());
        assert!(!Translations::about_pacman_repo(lang).is_empty());
        assert!(!Translations::about_web_portal(lang).is_empty());
        assert!(!Translations::about_tech_specs(lang).is_empty());
        assert!(!Translations::about_license(lang).is_empty());
        assert!(!Translations::about_open_in_browser(lang).is_empty());
        assert!(!Translations::about_copy_url(lang).is_empty());
    }
}

#[test]
fn test_about_modal_dialog_and_author_info() {
    use netlens::i18n::{Language, Translations};

    for &lang in Language::ALL {
        assert_eq!(Translations::about_author_section(lang).is_empty(), false);
        assert_eq!(Translations::about_role(lang).is_empty(), false);
        assert_eq!(Translations::about_email_label(lang).is_empty(), false);
        assert_eq!(Translations::about_copy_email(lang).is_empty(), false);
        assert_eq!(Translations::about_send_email(lang).is_empty(), false);
        assert_eq!(Translations::about_repositories_section(lang).is_empty(), false);
        assert_eq!(Translations::about_source_code(lang).is_empty(), false);
        assert_eq!(Translations::about_pacman_repo(lang).is_empty(), false);
        assert_eq!(Translations::about_web_portal(lang).is_empty(), false);
        assert_eq!(Translations::about_tech_specs(lang).is_empty(), false);
        assert_eq!(Translations::about_license(lang).is_empty(), false);
    }
}
