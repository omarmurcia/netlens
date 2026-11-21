mod state;
mod types;
mod update;
mod update_diagnostics;
mod update_network;
mod update_storage;
mod view_about_update;
mod view_dashboard;
mod view_diagnostics;
mod view_header;
mod view_inspector;
mod view_settings;
mod view_storage;
mod view_wifi_upnp;

pub use types::*;

use crate::history::NetworkEvent;
use crate::i18n::Language;
use crate::model::{Device, DeviceCategory, NetworkContext, PortInfo};
use crate::notification::{NotificationDetailLevel, NotificationSettings, NotificationUrgency};
use crate::oui::Oui;
use crate::realtime_monitor::NeighborAction;
use crate::router::{
    RouterConnection, StorageCredentials, StorageDiskInfo, StorageFileEntry, StorageShare,
    StorageUploadEvent, StorageUploadProgress, UpnpRouterStatus,
};
use crate::scanner::{PingStats, ScanEvent};
use crate::security_audit::SecurityAuditReport;
use crate::speedtest::{SpeedTestEvent, SpeedTestResult, SpeedTestStage};
use crate::storage::DeviceMetadata;
use crate::telemetry::{Telemetry, TelemetrySnapshot};
use crate::traceroute::HopInfo;
use crate::tray::{TrayHandle, TrayState};
use crate::updater::{InAppUpdateProgress, UpdateStatus};
use crate::uptime::UptimeTracker;
use crate::wifi_analyzer::WifiAnalyzerReport;
use iced::widget::canvas::Cache;
use iced::{Size, Theme};
use std::collections::{HashMap, HashSet, VecDeque};
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Helper to strip leading icon/emoji from localized modal titles
pub(super) fn strip_title_icon(s: &str) -> &str {
    let trimmed = s.trim_start();
    if let Some((idx, _)) = trimmed.char_indices().find(|&(_, c)| c.is_alphanumeric()) {
        &trimmed[idx..]
    } else {
        trimmed
    }
}

pub struct App {
    pub network: NetworkContext,
    pub devices: Vec<Device>,
    pub oui_db: Option<Arc<Oui>>,
    pub telemetry: Telemetry,
    pub is_scanning: bool,
    pub is_background_scan: bool,
    pub scan_progress: f32,
    pub status: AppStatus,
    pub search_query: String,
    pub selected_category: Option<DeviceCategory>,
    pub sort_by: SortOption,
    pub selected_device_ip: Option<Ipv4Addr>,
    pub inspector_ping_stats: Option<PingStats>,
    pub is_pinging: bool,
    pub is_deep_scanning: bool,
    pub deep_ports: Vec<PortInfo>,
    pub notification: Option<(String, bool)>, // (Message, is_success)
    pub alias_input: String,
    pub current_theme: Theme,
    pub language: Language,
    pub logo_handle: iced::widget::image::Handle,
    pub is_settings_open: bool,
    pub is_about_open: bool,
    pub update_status: UpdateStatus,
    pub update_progress: InAppUpdateProgress,
    pub is_update_modal_open: bool,
    pub is_speed_test_open: bool,
    pub speed_test_stage: SpeedTestStage,
    pub speed_test_ping: Option<f32>,
    pub speed_test_download: Option<f64>,
    pub speed_test_upload: Option<f64>,
    pub speed_test_result: Option<SpeedTestResult>,
    pub speed_test_history: Vec<SpeedTestResult>,
    pub device_metadata: HashMap<String, DeviceMetadata>,
    pub security_filter: SecurityFilter,
    pub router_ip_input: String,
    pub router_password_input: String,
    pub router_status_msg: Option<(String, bool)>,
    pub is_testing_router: bool,
    pub is_operating_device: Option<Ipv4Addr>,

    // Real-time telemetry sparklines
    pub download_history: VecDeque<f32>,
    pub upload_history: VecDeque<f32>,
    pub ping_history: VecDeque<f32>,
    pub download_sparkline_cache: Cache,
    pub upload_sparkline_cache: Cache,
    pub ping_sparkline_cache: Cache,

    // Periodic Auto-Discovery & Tick throttling
    pub auto_scan_interval_secs: u32,
    pub last_scan_time: Instant,
    pub last_uptime_check: Instant,
    pub telemetry_in_flight: bool,
    pub telemetry_tick_counter: u64,

    // Debounced async state persistence & tray diffing
    pub state_dirty: bool,
    pub last_state_dirty: Option<Instant>,
    pub persist_tx: std::sync::mpsc::Sender<String>,
    pub last_tray_snapshot: (usize, bool, Option<String>, Language),

    // Cached device list & filter counts (avoid re-sorting/re-allocating per frame)
    pub cached_filtered_indices: Vec<usize>,
    pub cached_visible_categories: Vec<DeviceCategory>,
    pub cached_whitelist_count: usize,
    pub cached_blocked_count: usize,

    // Intruder detection / Rogue Device Alert
    pub known_macs: HashSet<String>,
    pub alerted_macs: HashSet<String>,
    pub pending_intruder_alert: Option<IntruderAlert>,

    // Linksys Mesh topology
    pub router_connections: HashMap<String, RouterConnection>,

    // Visual Traceroute
    pub is_tracing_route: bool,
    pub traceroute_hops: Option<Vec<HopInfo>>,
    pub traceroute_error: Option<String>,

    // Background & System Autostart
    pub autostart_enabled: bool,
    pub minimize_on_close: bool,

    // Window & Lifecycle (Daemon)
    pub active_window_id: Option<iced::window::Id>,

    // System Tray
    pub tray_state: Arc<Mutex<TrayState>>,
    pub tray_handle: Arc<Mutex<Option<TrayHandle>>>,

    // Event History & Timeline
    pub network_events: Vec<NetworkEvent>,
    pub is_timeline_open: bool,

    // Network Health & Security Audit
    pub security_audit_report: Option<SecurityAuditReport>,
    pub is_audit_open: bool,

    // Router UPnP & NAT Port Mapping
    pub upnp_status: Option<UpnpRouterStatus>,
    pub is_upnp_open: bool,

    // Internet Stability / Outage Monitor
    pub uptime_tracker: UptimeTracker,

    // Wi-Fi Channel & Spectrum Analyzer
    pub wifi_analyzer_report: Option<WifiAnalyzerReport>,
    pub is_wifi_analyzer_open: bool,
    pub is_scanning_wifi: bool,

    // Desktop Notification Settings & Dispatch
    pub notification_settings: NotificationSettings,
    pub notification_prefix_input: String,
    pub scanned_macs_current_cycle: HashSet<String>,
    pub active_online_macs: HashSet<String>,
    pub initial_scan_completed: bool,
    pub last_realtime_events: HashMap<String, Instant>,

    // Fast Disconnection Detection (Heartbeat Keep-Alive)
    pub missed_heartbeats: HashMap<Ipv4Addr, u32>,
    pub last_heartbeat_time: Instant,

    // Router USB Storage / NAS Explorer
    pub is_storage_explorer_open: bool,
    pub is_storage_creds_open: bool,
    pub is_storage_loading: bool,
    pub storage_target_ip: String,
    pub storage_credentials: StorageCredentials,
    pub storage_shares: Vec<StorageShare>,
    pub storage_active_share: Option<String>,
    pub storage_current_path: String,
    pub storage_files: Vec<StorageFileEntry>,
    pub storage_disk_info: Option<StorageDiskInfo>,
    pub storage_filter_query: String,
    pub storage_error_msg: Option<String>,
    pub storage_status_msg: Option<String>,
    pub storage_selected_file: Option<StorageFileEntry>,
    pub is_creating_folder: bool,
    pub new_folder_name_input: String,
    pub file_to_delete: Option<(String, bool, String)>,
    pub is_storage_demo_mode: bool,
    pub storage_upload_progress: Option<StorageUploadProgress>,
    pub storage_last_freed: Option<u64>,
    pub demo_storage_files: HashMap<String, Vec<StorageFileEntry>>,
    pub demo_storage_disk: StorageDiskInfo,
    pub is_storage_drag_hovered: bool,
    pub hovered_drop_name: Option<String>,
    #[allow(dead_code)]
    pub router_has_storage: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Router USB Storage / NAS Explorer
    OpenStorageExplorer,
    CloseStorageExplorer,
    RefreshStorageShares,
    StorageTargetIpChanged(String),
    StorageCredentialsUserChanged(String),
    StorageCredentialsPassChanged(String),
    StorageCredentialsWorkgroupChanged(String),
    ToggleStorageCredentialsModal,
    StorageSaveCredentials,
    StorageShareSelected(String),
    StorageNavigateTo(String),
    StorageNavigateUp,
    StorageRefreshDirectory,
    StorageFilterQueryChanged(String),
    StorageStartCreateFolder,
    StorageCreateFolderNameChanged(String),
    StorageConfirmCreateFolder,
    StorageCancelCreateFolder,
    StorageRequestDeleteFile { path: String, is_dir: bool, name: String },
    StorageConfirmDeleteFile,
    StorageCancelDeleteFile,
    StorageOpenFile(String),
    StorageDownloadFile(String),
    StoragePickFileAndUpload,
    StoragePickFolderAndUpload,
    StorageFilePicked(Option<PathBuf>),
    StorageFolderPicked(Option<PathBuf>),
    StorageFileHovered(PathBuf),
    StorageFileDropped(PathBuf),
    StorageFilesHoveredLeft,
    StorageShowFileProperties(StorageFileEntry),
    StorageCloseProperties,
    StorageToggleDemoMode,
    StorageUploadEventReceived(StorageUploadEvent),
    StorageDismissUploadProgress,
    StorageSharesLoaded(Result<Vec<StorageShare>, String>),
    StorageDirectoryLoaded(Result<(Vec<StorageFileEntry>, Option<StorageDiskInfo>), String>),
    StorageOperationCompleted(Result<String, String>),

    // Fast Disconnection Detection (Heartbeat)
    ToggleHeartbeat(bool),
    HeartbeatResultsReceived(Vec<(Ipv4Addr, String, String, bool)>),

    // Real-Time Monitoring (Kernel Netlink)
    ToggleRealtimeMonitor(bool),
    RealtimeNeighborEvent(NeighborAction),
    RealtimeDeviceEnriched {
        ip: Ipv4Addr,
        mac: String,
        iface: String,
        vendor: String,
        hostname: Option<String>,
        model_name: Option<String>,
        services: Vec<String>,
    },

    // Notification Parameters & Settings
    ToggleNotificationsMaster(bool),
    ToggleNotifyUnknown(bool),
    ToggleNotifyKnownUnwhitelisted(bool),
    ToggleNotifyWhitelisted(bool),
    ToggleNotifyDisconnected(bool),
    ToggleNotifyOutages(bool),
    ToggleNotifySecurity(bool),
    ChangeNotificationUrgency(NotificationUrgency),
    ChangeNotificationDetail(NotificationDetailLevel),
    ChangeNotificationTimeout(u32),
    ToggleNotificationShowMac(bool),
    ToggleNotificationShowVendor(bool),
    ToggleNotificationPlaySound(bool),
    NotificationPrefixChanged(String),
    SendTestNotification,

    // Scanning
    StartScan,
    StartBackgroundScan,
    ScanEventReceived(ScanEvent),
    RefreshNetwork,

    // Window, System Tray & Background
    WindowCloseRequested(iced::window::Id),
    WindowClosed(iced::window::Id),
    RestoreWindowFromTray,
    ExitApp,
    ToggleAutostart(bool),
    ToggleMinimizeOnClose(bool),
    DesktopNotificationAction {
        ip: Ipv4Addr,
        mac: String,
        action: String,
    },

    // Periodic timer (1 second active, 5 seconds in background) & async telemetry
    Tick,
    TelemetryUpdated(TelemetrySnapshot),

    // Settings / Configuration
    OpenSettings,
    CloseSettings,
    OpenAbout,
    CloseAbout,
    OpenExternalUrl(String),
    CheckForUpdates,
    UpdateStatusReceived(UpdateStatus),
    OpenUpdateModal,
    CloseUpdateModal,
    StartInAppUpdate,
    InAppUpdateResult(Result<String, String>),
    OpenTerminalUpdater,
    RestartNetLens,
    LanguageChanged(Language),
    ThemeChanged(Theme),
    AutoScanIntervalChanged(u32),

    // Internet Speed Test
    OpenSpeedTest,
    CloseSpeedTest,
    StartSpeedTest,
    SpeedTestEventReceived(SpeedTestEvent),
    ClearSpeedTestHistory,

    // Filters & Search
    SearchChanged(String),
    CategorySelected(Option<DeviceCategory>),
    SecurityFilterChanged(SecurityFilter),
    SortChanged(SortOption),

    // Device Inspector
    SelectDevice(Ipv4Addr),
    CloseInspector,
    StartPingTest(Ipv4Addr),
    PingTestFinished(Result<PingStats, String>),
    StartDeepScan(Ipv4Addr),
    DeepScanFinished(Vec<PortInfo>),
    StartTraceroute(Ipv4Addr),
    TracerouteFinished(Result<Vec<HopInfo>, String>),
    SendWol(String),
    WolFinished(Result<(), String>),
    OpenInBrowser(Ipv4Addr),
    ChangeDeviceCategory(Ipv4Addr, DeviceCategory),

    // Security Control, Intruders & Router
    ToggleDeviceBlocked(Ipv4Addr, String),
    ToggleDeviceWhitelist(Ipv4Addr, String),
    TrustDevice(String),
    DismissIntruderAlert,
    RouterOperationFinished {
        ip: Ipv4Addr,
        mac: String,
        target_blocked: bool,
        result: Result<(), String>,
    },
    RouterIpChanged(String),
    RouterPasswordChanged(String),
    TestRouterConnection,
    RouterTestFinished(Result<bool, String>),
    FetchRouterTopology,
    RouterTopologyReceived(Result<Vec<RouterConnection>, String>),

    // Alias
    AliasInputChanged(String),
    SaveAlias(Ipv4Addr),

    // Clipboard
    CopyToClipboard(String, String),

    // Export
    ExportJson,
    ExportJsonSelected(Option<PathBuf>),
    ExportCsv,
    ExportCsvSelected(Option<PathBuf>),

    // Notifications
    DismissNotification,

    // Event History & Timeline
    OpenTimeline,
    CloseTimeline,
    ClearTimeline,

    // Security Audit
    RunSecurityAudit,
    OpenSecurityAudit,
    CloseSecurityAudit,

    // Router UPnP & NAT Port Mapping
    QueryUpnpRouter,
    OpenUpnpModal,
    CloseUpnpModal,
    UpnpRouterResult(Result<UpnpRouterStatus, String>),

    // Wi-Fi Channel Analyzer
    ScanWifi,
    OpenWifiAnalyzer,
    CloseWifiAnalyzer,
    WifiScanCompleted(Result<WifiAnalyzerReport, String>),

    // Connectivity monitoring (Uptime)
    ConnectivityChecked { gw_ok: bool, internet_ok: bool },

    // Device requests and domains history
    ClearDeviceRequestHistory(Ipv4Addr),
    AddDeviceSampleRequest(Ipv4Addr),
}

/// Default window configuration for NetLens (with official app icon)
pub fn default_window_settings() -> iced::window::Settings {
    let icon = crate::tray::get_window_icon();
    iced::window::Settings {
        size: Size::new(1220.0, 820.0),
        position: iced::window::Position::Centered,
        icon,
        platform_specific: iced::window::settings::PlatformSpecific {
            application_id: "netlens".to_string(),
            override_redirect: false,
        },
        exit_on_close_request: false,
        ..Default::default()
    }
}
