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

    // 9. Tablets (iPad, Galaxy Tab, Kindle Fire)
    let cat_ipad = Scanner::categorize_device(false, false, Some("iPad-de-Omar"), "Apple Inc.", &[]);
    assert_eq!(cat_ipad, DeviceCategory::Tablet);

    let cat_tab = Scanner::categorize_device(false, false, Some("Galaxy-Tab-S8"), "Samsung Electronics", &[]);
    assert_eq!(cat_tab, DeviceCategory::Tablet);

    let cat_kindle = Scanner::categorize_device(false, false, Some("Kindle-Fire-HD8"), "Amazon", &[]);
    assert_eq!(cat_kindle, DeviceCategory::Tablet);

    // 10. Smart Speakers & Audio (Sonos, Echo/Alexa, HomePod, Nest Audio)
    let cat_sonos = Scanner::categorize_device(false, false, Some("Sonos-One-LivingRoom"), "Sonos Inc.", &[]);
    assert_eq!(cat_sonos, DeviceCategory::SmartSpeaker);

    let cat_echo = Scanner::categorize_device(false, false, Some("Echo-Dot-Kitchen"), "Amazon Technologies Inc.", &[]);
    assert_eq!(cat_echo, DeviceCategory::SmartSpeaker);

    let cat_homepod = Scanner::categorize_device(false, false, Some("HomePod-Mini"), "Apple Inc.", &[]);
    assert_eq!(cat_homepod, DeviceCategory::SmartSpeaker);

    let ports_sonos = vec![PortInfo {
        port: 1400,
        service: "Sonos".to_string(),
        description: "Sonos Control Port".to_string(),
    }];
    let cat_sonos_port = Scanner::categorize_device(false, false, None, "Unknown Vendor", &ports_sonos);
    assert_eq!(cat_sonos_port, DeviceCategory::SmartSpeaker);

    // 11. Security Cameras / Surveillance / NVR
    let cat_ring = Scanner::categorize_device(false, false, Some("Ring-Doorbell-Pro"), "Ring LLC", &[]);
    assert_eq!(cat_ring, DeviceCategory::SecurityCamera);

    let cat_reolink = Scanner::categorize_device(false, false, Some("Reolink-Driveway"), "Reolink", &[]);
    assert_eq!(cat_reolink, DeviceCategory::SecurityCamera);

    let cat_hik = Scanner::categorize_device(false, false, Some("Hikvision-NVR"), "Hikvision", &[]);
    assert_eq!(cat_hik, DeviceCategory::SecurityCamera);

    let ports_rtsp = vec![PortInfo {
        port: 554,
        service: "RTSP".to_string(),
        description: "Real Time Streaming Protocol".to_string(),
    }];
    let cat_rtsp_cam = Scanner::categorize_device(false, false, None, "Generic Vendor", &ports_rtsp);
    assert_eq!(cat_rtsp_cam, DeviceCategory::SecurityCamera);

    // 12. Storage NAS (Synology, QNAP, TrueNAS, Unraid)
    let cat_synology = Scanner::categorize_device(false, false, Some("Synology-DS920"), "Synology Incorporated", &[]);
    assert_eq!(cat_synology, DeviceCategory::StorageNas);

    let cat_qnap = Scanner::categorize_device(false, false, Some("QNAP-TS453D"), "QNAP Systems", &[]);
    assert_eq!(cat_qnap, DeviceCategory::StorageNas);

    let cat_truenas = Scanner::categorize_device(false, false, Some("truenas-vault"), "Unknown", &[]);
    assert_eq!(cat_truenas, DeviceCategory::StorageNas);

    let ports_nas = vec![PortInfo {
        port: 5000,
        service: "Synology-DSM".to_string(),
        description: "Synology Management Web UI".to_string(),
    }];
    let cat_nas_port = Scanner::categorize_device(false, false, None, "Unknown Vendor", &ports_nas);
    assert_eq!(cat_nas_port, DeviceCategory::StorageNas);

    // 13. Dedicated Servers & Homelab
    let cat_proxmox = Scanner::categorize_device(false, false, Some("proxmox-pve-01"), "Intel", &[]);
    assert_eq!(cat_proxmox, DeviceCategory::Server);

    let cat_docker = Scanner::categorize_device(false, false, Some("docker-homelab"), "Unknown", &[]);
    assert_eq!(cat_docker, DeviceCategory::Server);

    let cat_pihole = Scanner::categorize_device(false, false, Some("pihole"), "Raspberry Pi Trading Ltd", &[]);
    assert_eq!(cat_pihole, DeviceCategory::Server);

    // 14. Wearables (Apple Watch, Garmin, Fitbit, Galaxy Watch)
    let cat_apple_watch = Scanner::categorize_device(false, false, Some("Omars-Apple-Watch"), "Apple Inc.", &[]);
    assert_eq!(cat_apple_watch, DeviceCategory::Wearable);

    let cat_garmin = Scanner::categorize_device(false, false, Some("Garmin-Forerunner"), "Garmin International", &[]);
    assert_eq!(cat_garmin, DeviceCategory::Wearable);

    // 15. Network Bridges / Extenders / Managed Switches
    let cat_extender = Scanner::categorize_device(false, false, Some("TP-Link-Extender-RE305"), "TP-Link Corporation", &[]);
    assert_eq!(cat_extender, DeviceCategory::NetworkBridge);

    let cat_repeater = Scanner::categorize_device(false, false, Some("Wifi-Repeater-Living"), "Netgear", &[]);
    assert_eq!(cat_repeater, DeviceCategory::NetworkBridge);

    let cat_switch_net = Scanner::categorize_device(false, false, Some("unifi-switch-8p"), "Ubiquiti Inc.", &[]);
    assert_eq!(cat_switch_net, DeviceCategory::NetworkBridge);

    // 16. Multi-layer fingerprinting: DIAL/HTTP banners and mDNS service discovery
    let cat_adv_cast = Scanner::categorize_device_advanced(
        false,
        false,
        None,
        "Unknown (Randomized)",
        &[],
        &["Google Cast".to_string(), "_googlecast._tcp.local".to_string()],
        Some("Chromecast with Google TV"),
    );
    assert_eq!(cat_adv_cast, DeviceCategory::StreamingDevice);

    let cat_adv_speaker = Scanner::categorize_device_advanced(
        false,
        false,
        None,
        "Unknown (Randomized)",
        &[],
        &["Spotify Connect".to_string()],
        Some("Bose SoundTouch 10"),
    );
    assert_eq!(cat_adv_speaker, DeviceCategory::SmartSpeaker);

    let cat_adv_cam = Scanner::categorize_device_advanced(
        false,
        false,
        None,
        "Unknown (Randomized)",
        &[],
        &["_onvif._tcp.local".to_string()],
        Some("Hikvision Dome Camera"),
    );
    assert_eq!(cat_adv_cam, DeviceCategory::SecurityCamera);
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
            let tip = Translations::tip_filter_category(cat, lang);
            assert!(!tip.is_empty(), "Tooltip de categoría {:?} vacío en idioma {:?}", cat, lang);
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

