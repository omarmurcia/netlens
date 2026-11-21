
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
        assert!(!Translations::storage_drag_drop_prompt(lang).is_empty());
        assert!(!Translations::storage_drag_drop_hint(lang).is_empty());
        assert!(!Translations::storage_drag_drop_select_share(lang).is_empty());
    }
}

#[tokio::test]
async fn test_router_storage_drag_and_drop_handling() {
    use netlens::app::{App, Message};
    use std::path::PathBuf;

    let (mut app, _) = App::new();

    // Explorer is closed initially
    assert!(!app.is_storage_explorer_open);
    assert!(!app.is_storage_drag_hovered);
    assert!(app.hovered_drop_name.is_none());

    // File hovered while closed should not set drag hovered
    let test_path = PathBuf::from("/tmp/document.pdf");
    let _ = app.update(Message::StorageFileHovered(test_path.clone()));
    assert!(!app.is_storage_drag_hovered);

    // Open storage explorer
    let _ = app.update(Message::OpenStorageExplorer);
    assert!(app.is_storage_explorer_open);

    // Hover file now
    let _ = app.update(Message::StorageFileHovered(test_path.clone()));
    assert!(app.is_storage_drag_hovered);
    assert_eq!(app.hovered_drop_name.as_deref(), Some("document.pdf"));

    // Drag leaves
    let _ = app.update(Message::StorageFilesHoveredLeft);
    assert!(!app.is_storage_drag_hovered);
    assert!(app.hovered_drop_name.is_none());

    // Hover again
    let _ = app.update(Message::StorageFileHovered(test_path.clone()));
    assert!(app.is_storage_drag_hovered);

    // Drop without an active share: should set error message and reset hover state
    app.storage_active_share = None;
    let _ = app.update(Message::StorageFileDropped(test_path.clone()));
    assert!(!app.is_storage_drag_hovered);
    assert!(app.hovered_drop_name.is_none());
    assert!(app.storage_error_msg.is_some());

    // Close explorer resets drag states
    app.is_storage_drag_hovered = true;
    app.hovered_drop_name = Some("test.zip".to_string());
    let _ = app.update(Message::CloseStorageExplorer);
    assert!(!app.is_storage_drag_hovered);
    assert!(app.hovered_drop_name.is_none());
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

    let temp_dir = std::env::temp_dir().join(format!("netlens_test_view_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    let _ = std::fs::create_dir_all(&temp_dir);
    let orig_xdg = std::env::var("XDG_CONFIG_HOME").ok();
    unsafe {
        std::env::set_var("XDG_CONFIG_HOME", &temp_dir);
    }

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

    unsafe {
        if let Some(orig) = orig_xdg {
            std::env::set_var("XDG_CONFIG_HOME", orig);
        } else {
            std::env::remove_var("XDG_CONFIG_HOME");
        }
    }
}

#[tokio::test]
async fn test_storage_disk_info_cache_protection_and_delete_update() {
    use netlens::app::{App, Message};
    use netlens::router::storage::{StorageDiskInfo, StorageFileEntry};

    let temp_dir = std::env::temp_dir().join(format!("netlens_test_view_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    let _ = std::fs::create_dir_all(&temp_dir);
    let orig_xdg = std::env::var("XDG_CONFIG_HOME").ok();
    unsafe {
        std::env::set_var("XDG_CONFIG_HOME", &temp_dir);
    }

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

    unsafe {
        if let Some(orig) = orig_xdg {
            std::env::set_var("XDG_CONFIG_HOME", orig);
        } else {
            std::env::remove_var("XDG_CONFIG_HOME");
        }
    }
}

