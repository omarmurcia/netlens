use chrono::Local;
use mac_oui::Oui;
use netlens::export::Exporter;
use netlens::model::{Device, DeviceCategory, DeviceStatus, NetworkContext, PortInfo};
use netlens::scanner::Scanner;
use std::fs;
use std::net::Ipv4Addr;

#[test]
fn test_is_randomized_mac() {
    // Direcciones MAC globales conocidas de fabricantes
    assert!(!Scanner::is_randomized_mac("d8:ec:5e:9c:77:1b")); // Belkin
    assert!(!Scanner::is_randomized_mac("c0:23:8d:66:79:fe")); // Samsung
    assert!(!Scanner::is_randomized_mac("28:92:00:09:72:02")); // Intel
    assert!(!Scanner::is_randomized_mac("fc:3c:d7:d2:b7:f7")); // Tuya

    // Direcciones MAC administradas localmente (LAA) / Aleatorias de privacidad
    // Bit 1 del primer byte debe ser 1: 0x76 = 0b01110110 (bit 1 es 1)
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
    // 1. Router / Puerta de enlace
    let cat_gw = Scanner::categorize_device(true, false, Some("Linksys02323"), "Belkin", &[]);
    assert_eq!(cat_gw, DeviceCategory::Router);

    // 2. Móvil Android
    let cat_phone = Scanner::categorize_device(false, false, Some("Marvin-s-A25"), "Samsung", &[]);
    assert_eq!(cat_phone, DeviceCategory::Smartphone);

    // 3. Smart TV
    let cat_tv = Scanner::categorize_device(false, false, Some("Samsung"), "Samsung Electronics", &[]);
    assert_eq!(cat_tv, DeviceCategory::SmartTv);

    // 4. Domótica / IoT
    let cat_iot = Scanner::categorize_device(false, false, Some("smart-plug"), "Tuya Smart Inc.", &[]);
    assert_eq!(cat_iot, DeviceCategory::Iot);

    // 5. Impresora (con puerto 631)
    let ports_print = vec![PortInfo {
        port: 631,
        service: "IPP".to_string(),
        description: "Printer".to_string(),
    }];
    let cat_printer = Scanner::categorize_device(false, false, Some("office-laser"), "HP Inc.", &ports_print);
    assert_eq!(cat_printer, DeviceCategory::Printer);

    // 6. Ordenador personal
    let cat_pc = Scanner::categorize_device(false, true, Some("cachyos-x8664"), "Intel", &[]);
    assert_eq!(cat_pc, DeviceCategory::Computer);
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
    };

    let ctx = NetworkContext {
        interface: "eno1".to_string(),
        local_ip: Some(Ipv4Addr::new(192, 168, 1, 176)),
        local_mac: Some("6c:0b:5e:72:c8:d8".to_string()),
        gateway_ip: Some(Ipv4Addr::new(192, 168, 1, 1)),
        gateway_mac: Some("d8:ec:5e:9c:77:1b".to_string()),
        subnet_cidr: "192.168.1.0/24".to_string(),
    };

    // Probar exportación a JSON
    let json_path = Exporter::export_json(&[sample_device.clone()], &ctx).expect("JSON export failed");
    assert!(json_path.exists());
    let json_content = fs::read_to_string(&json_path).expect("Failed to read JSON");
    assert!(json_content.contains("192.168.1.1"));
    assert!(json_content.contains("Linksys02323"));
    assert!(json_content.contains("Router Principal"));
    let _ = fs::remove_file(json_path);

    // Probar exportación a CSV
    let csv_path = Exporter::export_csv(&[sample_device]).expect("CSV export failed");
    assert!(csv_path.exists());
    let csv_content = fs::read_to_string(&csv_path).expect("Failed to read CSV");
    assert!(csv_content.contains("192.168.1.1"));
    assert!(csv_content.contains("d8:ec:5e:9c:77:1b"));
    assert!(csv_content.contains("Router / Puerta de Enlace"));
    let _ = fs::remove_file(csv_path);
}

#[test]
fn test_wol_validation() {
    // MAC válida
    let valid_res = Scanner::send_wake_on_lan("d8:ec:5e:9c:77:1b");
    assert!(valid_res.is_ok());

    // MAC con longitud inválida
    let invalid_res = Scanner::send_wake_on_lan("12:34:56");
    assert!(invalid_res.is_err());
}

#[test]
fn test_telemetry_functions() {
    use netlens::telemetry::Telemetry;

    // 1. Conversión de dBm a porcentaje
    assert_eq!(Telemetry::dbm_to_percent(-35), 100);
    assert_eq!(Telemetry::dbm_to_percent(-50), 100);
    assert_eq!(Telemetry::dbm_to_percent(-60), 80);
    assert_eq!(Telemetry::dbm_to_percent(-75), 50);
    assert_eq!(Telemetry::dbm_to_percent(-100), 0);
    assert_eq!(Telemetry::dbm_to_percent(-110), 0);

    // 2. Formato de velocidad
    assert_eq!(Telemetry::format_speed(500.0), "500 B/s");
    assert_eq!(Telemetry::format_speed(1536.0), "1.5 KB/s");
    assert_eq!(Telemetry::format_speed(2.5 * 1024.0 * 1024.0), "2.50 MB/s");

    // 3. Formato de bytes
    assert_eq!(Telemetry::format_bytes(50 * 1024 * 1024), "50.0 MB");

    // 4. Muestreo de tráfico
    let (rx, tx) = Telemetry::sample_total_traffic();
    assert!(rx > 0 || tx > 0, "Debe haber tráfico de red en el sistema");
}
