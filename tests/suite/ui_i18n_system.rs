use chrono::Local;
use netlens::model::{Device, DeviceCategory, DeviceStatus};
use std::net::Ipv4Addr;

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

        for &cat in DeviceCategory::all() {
            assert!(!Translations::tip_filter_category(cat, lang).is_empty(), "Tip for {:?} in {:?}", cat, lang);
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

        // 7. System Tray Translations
        assert!(!Translations::tray_tooltip_title(lang).is_empty());
        assert!(!Translations::tray_tooltip_scanning(lang).is_empty());
        assert!(!Translations::tray_tooltip_with_gw(5, "192.168.1.1", lang).is_empty());
        assert!(!Translations::tray_tooltip_devices(5, lang).is_empty());
        assert!(!Translations::tray_menu_open(lang).is_empty());
        assert!(!Translations::tray_menu_scan_now(lang).is_empty());
        assert!(!Translations::tray_menu_scanning(lang).is_empty());
        assert!(!Translations::tray_menu_devices(5, lang).is_empty());
        assert!(!Translations::tray_menu_exit(lang).is_empty());

        // 8. Version Updates Translations
        assert!(!Translations::update_latest_version(lang).is_empty());
        assert!(!Translations::update_available(lang).is_empty());
        assert!(!Translations::update_btn(lang).is_empty());
        assert!(!Translations::check_for_updates(lang).is_empty());
        assert!(!Translations::checking_updates(lang).is_empty());
        assert!(!Translations::update_modal_title(lang).is_empty());
        assert!(!Translations::update_instructions(lang).is_empty());
        assert!(!Translations::update_copy_command(lang).is_empty());
        assert!(!Translations::update_copied_notif(lang).is_empty());
        assert!(!Translations::update_now_btn(lang).is_empty());
        assert!(!Translations::update_in_terminal_btn(lang).is_empty());
        assert!(!Translations::update_authorizing_and_installing(lang).is_empty());
        assert!(!Translations::update_success_banner(lang).is_empty());
        assert!(!Translations::update_restart_btn(lang).is_empty());
        assert!(!Translations::update_failed_banner(lang).is_empty());
        assert!(!Translations::update_retry_btn(lang).is_empty());
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

#[test]
fn test_updater_semver_comparison_and_logic() {
    use netlens::updater::UpdateChecker;

    // Remote is higher version
    assert!(UpdateChecker::is_newer("0.1.1", 1, "0.1.0", 2));
    assert!(UpdateChecker::is_newer("0.2.0", 1, "0.1.0", 2));
    assert!(UpdateChecker::is_newer("1.0.0", 1, "0.1.0", 2));

    // Remote is same version but higher pkgrel
    assert!(UpdateChecker::is_newer("0.1.0", 3, "0.1.0", 2));

    // Remote is identical version and pkgrel
    assert!(!UpdateChecker::is_newer("0.1.0", 2, "0.1.0", 2));

    // Remote is older version or older pkgrel
    assert!(!UpdateChecker::is_newer("0.1.0", 1, "0.1.0", 2));
    assert!(!UpdateChecker::is_newer("0.0.9", 5, "0.1.0", 2));
}

#[test]
fn test_in_app_update_progress_states() {
    use netlens::updater::InAppUpdateProgress;

    let p1 = InAppUpdateProgress::Idle;
    let p2 = InAppUpdateProgress::AuthorizingAndInstalling;
    let p3 = InAppUpdateProgress::Success;
    let p4 = InAppUpdateProgress::Failed("Auth error".to_string());

    assert_eq!(p1, InAppUpdateProgress::Idle);
    assert_ne!(p1, p2);
    assert_ne!(p2, p3);
    assert_eq!(p4, InAppUpdateProgress::Failed("Auth error".to_string()));
}

#[tokio::test]
async fn test_single_instance_lifecycle_and_secondary_detection() {
    use netlens::single_instance::{acquire_single_instance, InstanceStatus};

    let temp_dir = std::env::temp_dir().join(format!("netlens_test_si_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&temp_dir);

    let orig_xdg = std::env::var("XDG_RUNTIME_DIR").ok();
    unsafe {
        std::env::set_var("XDG_RUNTIME_DIR", &temp_dir);
    }

    // 1. Primary instance acquires the lock
    let status1 = acquire_single_instance(false, false).await;
    let guard = match status1 {
        InstanceStatus::Primary(g) => g,
        InstanceStatus::SecondaryNotified => panic!("First instance must be Primary"),
    };

    #[cfg(not(windows))]
    assert!(guard.socket_path.exists(), "Socket path should exist for primary instance");

    // 2. Secondary instance attempts acquisition -> should be SecondaryNotified
    let status2 = acquire_single_instance(false, false).await;
    match status2 {
        InstanceStatus::SecondaryNotified => {} // Expected
        InstanceStatus::Primary(_) => panic!("Second instance must not become Primary while first is running"),
    }

    // 3. Drop guard -> socket should be cleaned up
    drop(guard);
    assert!(!temp_dir.join("netlens.sock").exists(), "Socket file must be removed when guard is dropped");

    // 4. Stale socket recovery: create empty file to simulate ungraceful crash
    #[cfg(not(windows))]
    {
        let stale_socket = temp_dir.join("netlens.sock");
        std::fs::write(&stale_socket, b"stale").expect("Failed to write dummy stale socket file");
        assert!(stale_socket.exists());

        let status3 = acquire_single_instance(false, false).await;
        match status3 {
            InstanceStatus::Primary(g3) => {
                assert!(g3.socket_path.exists());
                drop(g3);
            }
            InstanceStatus::SecondaryNotified => panic!("Should recover from stale socket without listener"),
        }
    }

    // Cleanup environment
    unsafe {
        if let Some(orig) = orig_xdg {
            std::env::set_var("XDG_RUNTIME_DIR", orig);
        } else {
            std::env::remove_var("XDG_RUNTIME_DIR");
        }
    }
    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_all_16_device_categories_coverage_and_serde_compatibility() {
    use netlens::i18n::{Language, Translations};
    use netlens::model::DeviceCategory;

    let all_cats = DeviceCategory::all();
    assert_eq!(all_cats.len(), 16, "Must define exactly 16 granular device categories");

    for &cat in all_cats {
        assert!(!cat.icon().is_empty(), "Icon missing for category {:?}", cat);
        assert!(!cat.display_name().is_empty(), "Display name missing for category {:?}", cat);

        for &lang in Language::ALL {
            let name = Translations::category_name(cat, lang);
            let short = Translations::category_short_name(cat, lang);
            let tip = Translations::tip_filter_category(cat, lang);

            assert!(!name.is_empty(), "category_name empty for {:?} in {:?}", cat, lang);
            assert!(!short.is_empty(), "category_short_name empty for {:?} in {:?}", cat, lang);
            assert!(!tip.is_empty(), "tip_filter_category empty for {:?} in {:?}", cat, lang);
            assert!(short.contains(cat.icon()), "category_short_name must include icon for {:?} in {:?}", cat, lang);
        }
    }

    // Verify serde backwards compatibility aliases
    let legacy_smart_home: DeviceCategory = serde_json::from_str("\"SmartHomeIot\"").unwrap();
    assert_eq!(legacy_smart_home, DeviceCategory::Iot);

    let legacy_smart_home_alt: DeviceCategory = serde_json::from_str("\"SmartHome\"").unwrap();
    assert_eq!(legacy_smart_home_alt, DeviceCategory::Iot);

    let legacy_nas: DeviceCategory = serde_json::from_str("\"Nas\"").unwrap();
    assert_eq!(legacy_nas, DeviceCategory::StorageNas);

    let legacy_storage: DeviceCategory = serde_json::from_str("\"Storage\"").unwrap();
    assert_eq!(legacy_storage, DeviceCategory::StorageNas);

    // Verify round-trip serialization for new categories
    for &cat in all_cats {
        let serialized = serde_json::to_string(&cat).unwrap();
        let deserialized: DeviceCategory = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, cat);
    }
}

#[test]
fn test_performance_optimizations_and_bugfixes() {
    use netlens::app::{App, Message, SortOption};
    use netlens::storage::{AppState, AppStateRef, Storage};
    use netlens::telemetry::WifiStatus;
    #[cfg(not(windows))]
    use netlens::tray::get_icon_argb;
    use netlens::widget::CachedSparkline;
    use std::collections::{HashMap, VecDeque};

    // 1. Wi-Fi band_label ordering (6 GHz must not be shadowed by 5 GHz)
    let wifi_6e = WifiStatus {
        frequency_mhz: Some(6115.0),
        ..Default::default()
    };
    assert_eq!(wifi_6e.band_label(), "6 GHz (Wi-Fi 6E)");

    let wifi_5 = WifiStatus {
        frequency_mhz: Some(5180.0),
        ..Default::default()
    };
    assert_eq!(wifi_5.band_label(), "5 GHz");

    let wifi_24 = WifiStatus {
        frequency_mhz: Some(2437.0),
        ..Default::default()
    };
    assert_eq!(wifi_24.band_label(), "2.4 GHz");

    // 2. Compact 64x64 tray icon reduces D-Bus pixmap footprint
    #[cfg(not(windows))]
    {
        let tray_icon = get_icon_argb().expect("Tray icon should load");
        assert_eq!(tray_icon.width, 64);
        assert_eq!(tray_icon.height, 64);
        assert_eq!(tray_icon.data.len(), 64 * 64 * 4);
    }
    #[cfg(windows)]
    {
        assert!(netlens::tray::get_window_icon().is_some());
    }

    // 3. Zero-clone AppStateRef serializes identically to AppState
    let meta = HashMap::new();
    let mac = "aa:bb:cc:dd:ee:ff".to_string();
    let notif = netlens::notification::NotificationSettings::default();
    let state_ref = AppStateRef {
        language: netlens::i18n::Language::Spanish,
        theme: "TokyoNight",
        devices: &[],
        device_metadata: &meta,
        last_speed_test: None,
        speed_test_history: &[],
        router_ip: Some("192.168.1.1"),
        router_password: None,
        auto_scan_interval_secs: 30,
        known_macs: vec![&mac],
        autostart: false,
        minimize_to_tray_on_close: true,
        network_events: &[],
        notification_settings: &notif,
        storage_credentials: None,
    };
    let json = Storage::serialize_ref(&state_ref).expect("serialize AppStateRef");
    let decoded: AppState = serde_json::from_str(&json).expect("deserialize as AppState");
    assert_eq!(decoded.known_macs, vec!["aa:bb:cc:dd:ee:ff"]);
    assert_eq!(decoded.router_ip.as_deref(), Some("192.168.1.1"));

    // 4. CachedSparkline instantiation with VecDeque and Cache
    let mut pts = VecDeque::new();
    pts.push_back(10.0);
    pts.push_back(25.0);
    let cache = iced::widget::canvas::Cache::new();
    let cached = CachedSparkline::new(&pts, iced::Color::WHITE, &cache);
    assert_eq!(cached.points.len(), 2);

    // 5. UTF-8 multibyte vendor name rendering + cached filtering/sorting do not panic
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let (mut app, _) = App::new();
        let now = Local::now();
        app.devices.push(Device {
            ip: Ipv4Addr::new(192, 168, 1, 77),
            mac: "11:22:33:44:55:66".to_string(),
            vendor: "ShenZhen Tecnología Avanzada 中文测试设备".to_string(),
            hostname: Some("test-utf8".to_string()),
            category: DeviceCategory::Computer,
            is_randomized_mac: false,
            is_gateway: false,
            is_current_host: false,
            interface: "eth0".to_string(),
            latency_ms: Some(2.5),
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
        });
        let _ = app.update(Message::SortChanged(SortOption::Vendor));
        let _ = app.update(Message::SearchChanged("test-utf8".to_string()));
        assert!(!app.cached_filtered_indices.is_empty());
        let win_id = iced::window::Id::unique();
        let _view = app.view(win_id);
    });
}
