use crate::autostart::AutostartManager;
use crate::export::Exporter;
use crate::history::{HistoryManager, NetworkEvent, NetworkEventType};
use crate::i18n::{Language, Translations};
use crate::model::{Device, DeviceCategory, DeviceStatus, NetworkContext, PortInfo};
use crate::notification::{NotificationDetailLevel, NotificationSettings, NotificationUrgency};
use crate::router::{
    format_file_size, LinksysRouter, RouterConnection, RouterStorageManager, StorageCredentials,
    StorageDiskInfo, StorageFileEntry, StorageShare, StorageUploadEvent, StorageUploadProgress,
    UpnpClient, UpnpRouterStatus,
};
use crate::scanner::{PingStats, ScanEvent, Scanner};
use crate::security_audit::{AuditSeverity, SecurityAuditReport, SecurityAuditor};
use crate::speedtest::{SpeedTest, SpeedTestEvent, SpeedTestResult, SpeedTestStage};
use crate::storage::{AppState, DeviceMetadata, Storage};
use crate::telemetry::Telemetry;
use crate::theme::CustomStyles;
use crate::traceroute::{HopInfo, Traceroute};
use crate::realtime_monitor::{NeighborAction, RealtimeMonitor};
use crate::tray::{spawn_tray, NetLensTray, TrayAction, TrayState};
use crate::uptime::UptimeTracker;
use crate::widget::Sparkline;
use crate::wifi_analyzer::{WifiAnalyzer, WifiAnalyzerReport};
use crate::updater::{InAppUpdateProgress, UpdateChecker, UpdateStatus};
use iced::futures::channel::mpsc;
use iced::futures::StreamExt;
use iced::widget::canvas::Canvas;
use iced::widget::{
    button, center, column, container, pick_list, progress_bar, row, scrollable, space, text,
    text_input, tooltip, Column,
};
use iced::{Alignment, Color, Element, Length, Padding, Size, Subscription, Task, Theme};
use crate::oui::Oui;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct IntruderAlert {
    pub ip: Ipv4Addr,
    pub mac: String,
    pub vendor: String,
    pub hostname: Option<String>,
}

pub const AVAILABLE_THEMES: &[Theme] = &[
    Theme::TokyoNight,
    Theme::CatppuccinMocha,
    Theme::Nord,
    Theme::Dracula,
    Theme::Dark,
    Theme::Light,
    Theme::SolarizedDark,
    Theme::GruvboxDark,
];

pub fn theme_to_string(theme: &Theme) -> &'static str {
    match theme {
        Theme::TokyoNight => "TokyoNight",
        Theme::CatppuccinMocha => "CatppuccinMocha",
        Theme::Nord => "Nord",
        Theme::Dracula => "Dracula",
        Theme::Dark => "Dark",
        Theme::Light => "Light",
        Theme::SolarizedDark => "SolarizedDark",
        Theme::GruvboxDark => "GruvboxDark",
        _ => "TokyoNight",
    }
}

pub fn theme_from_string(s: &str) -> Theme {
    match s {
        "TokyoNight" => Theme::TokyoNight,
        "CatppuccinMocha" => Theme::CatppuccinMocha,
        "Nord" => Theme::Nord,
        "Dracula" => Theme::Dracula,
        "Dark" => Theme::Dark,
        "Light" => Theme::Light,
        "SolarizedDark" => Theme::SolarizedDark,
        "GruvboxDark" => Theme::GruvboxDark,
        _ => Theme::TokyoNight,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOption {
    Ip,
    Latency,
    Name,
    Vendor,
    Category,
}

impl SortOption {
    pub const ALL: &[SortOption] = &[
        SortOption::Ip,
        SortOption::Latency,
        SortOption::Name,
        SortOption::Vendor,
        SortOption::Category,
    ];
}

impl fmt::Display for SortOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortOption::Ip => write!(f, "🌐 IP"),
            SortOption::Latency => write!(f, "⚡ Ping"),
            SortOption::Name => write!(f, "📝 Name"),
            SortOption::Vendor => write!(f, "🏭 Vendor"),
            SortOption::Category => write!(f, "🏷️ Type"),
        }
    }
}

/// Security filter (All, Whitelist, Blocked)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SecurityFilter {
    #[default]
    All,
    Whitelist,
    Blocked,
}

/// Wrapper for dynamically localized sorting options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalizedSortOption {
    pub sort: SortOption,
    pub lang: Language,
}

impl fmt::Display for LocalizedSortOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Translations::sort_label(self.sort, self.lang))
    }
}

/// Wrapper for dynamically localized device categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalizedCategory {
    pub category: DeviceCategory,
    pub lang: Language,
}

impl fmt::Display for LocalizedCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.category.icon(),
            Translations::category_name(self.category, self.lang)
        )
    }
}

/// Descriptive application state for reactive rendering in any language
#[derive(Debug, Clone)]
pub enum AppStatus {
    Ready,
    ScanningSubnet(String),
    ProbingHosts { total: usize, subnet: String },
    ScanFinished { total_found: usize, duration_secs: f32 },
    ConnectedTo { gw: String, iface: String, subnet: String },
    ContextUpdated { iface: String, subnet: String },
    Error(String),
}

impl AppStatus {
    pub fn render(&self, lang: Language) -> String {
        match self {
            AppStatus::Ready => Translations::empty_initial(lang).to_string(),
            AppStatus::ScanningSubnet(subnet) => match lang {
                Language::Spanish => format!("Escaneando subred {}...", subnet),
                Language::English => format!("Scanning subnet {}...", subnet),
                Language::Chinese => format!("正在扫描子网 {}...", subnet),
                Language::Hindi => format!("सबनेट {} स्कैन हो रहा है...", subnet),
                Language::Arabic => format!("جاري فحص الشبكة الفرعية {}...", subnet),
                Language::French => format!("Scan du sous-réseau {}...", subnet),
                Language::Bengali => format!("সাবনেট {} স্ক্যান হচ্ছে...", subnet),
                Language::Portuguese => format!("Escaneando sub-rede {}...", subnet),
                Language::Russian => format!("Сканирование подсети {}...", subnet),
                Language::Urdu => format!("سب نیٹ {} اسکین ہو رہا ہے...", subnet),
            },
            AppStatus::ProbingHosts { total, subnet } => match lang {
                Language::Spanish => format!("Sondeando {} hosts en {}...", total, subnet),
                Language::English => format!("Probing {} hosts on {}...", total, subnet),
                Language::Chinese => format!("正在探测 {} 上的 {} 台主机...", subnet, total),
                Language::Hindi => format!("{} पर {} होस्ट की जांच...", subnet, total),
                Language::Arabic => format!("جاري فحص {} جهاز على {}...", total, subnet),
                Language::French => format!("Sondage de {} hôtes sur {}...", total, subnet),
                Language::Bengali => format!("{} তে {} টি হোস্ট স্ক্যান হচ্ছে...", subnet, total),
                Language::Portuguese => format!("Sondando {} hosts em {}...", total, subnet),
                Language::Russian => format!("Опрос {} узлов в {}...", total, subnet),
                Language::Urdu => format!("{} پر {} ہوسٹس کی جانچ...", subnet, total),
            },
            AppStatus::ScanFinished { total_found, duration_secs } => match lang {
                Language::Spanish => format!("Escaneo finalizado: {} dispositivos detectados en {:.2} s", total_found, duration_secs),
                Language::English => format!("Scan complete: {} devices detected in {:.2} s", total_found, duration_secs),
                Language::Chinese => format!("扫描完成：在 {:.2} 秒内发现 {} 台设备", duration_secs, total_found),
                Language::Hindi => format!("स्कैन पूर्ण: {:.2} सेकंड में {} उपकरण मिले", duration_secs, total_found),
                Language::Arabic => format!("اكتمل الفحص: تم اكتشاف {} جهاز خلال {:.2} ثانية", total_found, duration_secs),
                Language::French => format!("Scan terminé : {} appareils détectés en {:.2} s", total_found, duration_secs),
                Language::Bengali => format!("স্ক্যান সম্পন্ন: {:.2} সেকেন্ডে {} টি ডিভাইস শনাক্ত", duration_secs, total_found),
                Language::Portuguese => format!("Escaneamento concluído: {} dispositivos detectados em {:.2} s", total_found, duration_secs),
                Language::Russian => format!("Сканирование завершено: обнаружено {} устройств за {:.2} с", total_found, duration_secs),
                Language::Urdu => format!("اسکین مکمل: {:.2} سیکنڈ میں {} آلات ملے", duration_secs, total_found),
            },
            AppStatus::ConnectedTo { gw, iface, subnet } => match lang {
                Language::Spanish => format!("Conectado a {} mediante {} (Subred {})", gw, iface, subnet),
                Language::English => format!("Connected to {} via {} (Subnet {})", gw, iface, subnet),
                Language::Chinese => format!("已通过 {} 连接至 {} (子网 {})", iface, gw, subnet),
                Language::Hindi => format!("{} के माध्यम से {} से कनेक्टेड (सबनेट {})", iface, gw, subnet),
                Language::Arabic => format!("متصل بـ {} عبر {} (الشبكة الفرعية {})", gw, iface, subnet),
                Language::French => format!("Connecté à {} via {} (Sous-réseau {})", gw, iface, subnet),
                Language::Bengali => format!("{} এর মাধ্যমে {} এর সাথে সংযুক্ত (সাবনেট {})", iface, gw, subnet),
                Language::Portuguese => format!("Conectado a {} via {} (Sub-rede {})", gw, iface, subnet),
                Language::Russian => format!("Подключено к {} через {} (Подсеть {})", gw, iface, subnet),
                Language::Urdu => format!("{} کے ذریعے {} سے منسلک (سب نیٹ {})", iface, gw, subnet),
            },
            AppStatus::ContextUpdated { iface, subnet } => match lang {
                Language::Spanish => format!("Contexto de red actualizado: {} ({})", iface, subnet),
                Language::English => format!("Network context updated: {} ({})", iface, subnet),
                Language::Chinese => format!("网络状态已更新：{} ({})", iface, subnet),
                Language::Hindi => format!("नेटवर्क संदर्भ अपडेट किया गया: {} ({})", iface, subnet),
                Language::Arabic => format!("تم تحديث سياق الشبكة: {} ({})", iface, subnet),
                Language::French => format!("Contexte réseau actualisé : {} ({})", iface, subnet),
                Language::Bengali => format!("নেটওয়ার্ক স্থিতি আপডেট হয়েছে: {} ({})", iface, subnet),
                Language::Portuguese => format!("Contexto de rede atualizado: {} ({})", iface, subnet),
                Language::Russian => format!("Контекст сети обновлен: {} ({})", iface, subnet),
                Language::Urdu => format!("نیٹ ورک کی صورتحال اپ ڈیٹ ہو گئی: {} ({})", iface, subnet),
            },
            AppStatus::Error(e) => format!("Error: {}", e),
        }
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

    // Periodic Auto-Discovery
    pub auto_scan_interval_secs: u32,
    pub last_scan_time: Instant,

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
    pub tray_handle: Arc<Mutex<Option<ksni::Handle<NetLensTray>>>>,

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

    // Periodic timer (1 second for speed and signal telemetry)
    Tick,

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

/// Fast, lightweight probe for Heartbeat Keep-Alive (detects disconnections in seconds)
async fn probe_heartbeat_device(ip: Ipv4Addr) -> bool {
    // 1. ICMP Ping probe with 600ms timeout
    let ping_ok = tokio::time::timeout(Duration::from_millis(600), async {
        tokio::process::Command::new("ping")
            .args(["-c", "1", "-W", "1", &ip.to_string()])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await
            .map(|s| s.success())
            .unwrap_or(false)
    })
    .await
    .unwrap_or(false);

    if ping_ok {
        return true;
    }

    // 2. If ping fails (e.g. firewalled devices that drop ICMP but are connected), probe standard TCP ports
    let tcp_ok = tokio::time::timeout(Duration::from_millis(400), async {
        for port in [80, 443, 53, 8080, 22] {
            if tokio::net::TcpStream::connect((ip, port)).await.is_ok() {
                return true;
            }
        }
        false
    })
    .await
    .unwrap_or(false);

    tcp_ok
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
            include_bytes!("../assets/netlens.png").as_slice(),
        );

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
            auto_scan_interval_secs,
            last_scan_time: Instant::now(),
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
            router_has_storage: false,
        };

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

    /// Synchronizes current scan state and devices with System Tray
    pub fn sync_tray_state(&self) {
        if let Ok(mut s) = self.tray_state.lock() {
            s.device_count = self.devices.len();
            s.is_scanning = self.is_scanning;
            s.gateway_ip = self.network.gateway_ip.map(|g| g.to_string());
            s.language = self.language;
        }
        if let Ok(lock) = self.tray_handle.lock() {
            if let Some(h) = lock.as_ref() {
                let h_clone = h.clone();
                tokio::spawn(async move {
                    let _ = h_clone.update(|_| ()).await;
                });
            }
        }
    }

    /// Persists current application state to disk
    pub fn save_state(&self) {
        let state = AppState {
            language: self.language,
            theme: theme_to_string(&self.current_theme).to_string(),
            devices: self.devices.clone(),
            device_metadata: self.device_metadata.clone(),
            last_speed_test: self.speed_test_result.clone(),
            speed_test_history: self.speed_test_history.clone(),
            router_ip: Some(self.router_ip_input.clone()),
            router_password: if self.router_password_input.trim().is_empty() {
                None
            } else {
                Some(self.router_password_input.clone())
            },
            auto_scan_interval_secs: self.auto_scan_interval_secs,
            known_macs: self.known_macs.iter().cloned().collect(),
            autostart: self.autostart_enabled,
            minimize_to_tray_on_close: self.minimize_on_close,
            network_events: self.network_events.clone(),
            notification_settings: self.notification_settings.clone(),
            storage_credentials: Some(self.storage_credentials.clone()),
        };
        if let Err(e) = Storage::save(&state) {
            eprintln!("Aviso: No se pudo guardar estado persistente: {}", e);
        }
    }

    /// Periodic 1-second subscription for measuring upload/download speed, window events, and System Tray
    pub fn subscription(&self) -> Subscription<Message> {
        let tick_sub = Subscription::run(|| {
            iced::futures::stream::unfold(
                tokio::time::interval(Duration::from_millis(1000)),
                |mut interval| async move {
                    interval.tick().await;
                    Some((Message::Tick, interval))
                },
            )
        });

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

        Subscription::batch(vec![tick_sub, close_req_sub, close_evt_sub, tray_sub, realtime_sub])
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.telemetry.update_sample(&self.network.interface);

                // Telemetry sparklines: record samples of the last 30 seconds
                self.download_history
                    .push_back(self.telemetry.download_speed_bps as f32);
                if self.download_history.len() > 30 {
                    self.download_history.pop_front();
                }

                self.upload_history
                    .push_back(self.telemetry.upload_speed_bps as f32);
                if self.upload_history.len() > 30 {
                    self.upload_history.pop_front();
                }

                if let Some(stats) = &self.inspector_ping_stats {
                    self.ping_history.push_back(stats.avg_ms);
                    if self.ping_history.len() > 30 {
                        self.ping_history.pop_front();
                    }
                }

                // Auto-Discovery: if interval is active and time elapsed, re-scan silently in background
                if self.auto_scan_interval_secs > 0
                    && !self.is_scanning
                    && self.last_scan_time.elapsed().as_secs() >= self.auto_scan_interval_secs as u64
                {
                    self.last_scan_time = Instant::now();
                    return Task::done(Message::StartBackgroundScan);
                }

                self.sync_tray_state();

                let mut tick_tasks = Vec::new();

                // 1. Periodic Internet stability probe every 5 seconds
                let gw = self.network.gateway_ip;
                tick_tasks.push(Task::future(async move {
                    let (gw_ok, internet_ok) = UptimeTracker::probe_connectivity(gw).await;
                    Message::ConnectivityChecked { gw_ok, internet_ok }
                }));

                // 2. Fast Disconnection Detection (Heartbeat Keep-Alive every 4 seconds)
                if self.notification_settings.enabled
                    && self.notification_settings.fast_disconnection_heartbeat
                    && !self.is_scanning
                    && self.last_heartbeat_time.elapsed() >= Duration::from_secs(4)
                {
                    self.last_heartbeat_time = Instant::now();
                    let targets: Vec<(Ipv4Addr, String, String)> = self.devices
                        .iter()
                        .filter(|d| d.status == DeviceStatus::Online && !d.is_gateway && !d.is_current_host)
                        .map(|d| (d.ip, d.mac.clone(), d.display_name()))
                        .collect();

                    if !targets.is_empty() {
                        tick_tasks.push(Task::future(async move {
                            let mut join_set = tokio::task::JoinSet::new();
                            for (ip, mac, name) in targets {
                                join_set.spawn(async move {
                                    let is_alive = probe_heartbeat_device(ip).await;
                                    (ip, mac, name, is_alive)
                                });
                            }
                            let mut results = Vec::new();
                            while let Some(res) = join_set.join_next().await {
                                if let Ok(entry) = res {
                                    results.push(entry);
                                }
                            }
                            Message::HeartbeatResultsReceived(results)
                        }));
                    }
                }

                Task::batch(tick_tasks)
            }

            Message::OpenSettings => {
                self.is_settings_open = true;
                self.is_about_open = false;
                self.is_update_modal_open = false;
                Task::none()
            }

            Message::CloseSettings => {
                self.is_settings_open = false;
                Task::none()
            }

            Message::OpenAbout => {
                self.is_about_open = true;
                self.is_settings_open = false;
                self.is_update_modal_open = false;
                self.is_speed_test_open = false;
                self.is_timeline_open = false;
                self.is_audit_open = false;
                self.is_wifi_analyzer_open = false;
                self.is_upnp_open = false;
                self.is_storage_explorer_open = false;
                Task::none()
            }

            Message::CloseAbout => {
                self.is_about_open = false;
                Task::none()
            }

            Message::OpenExternalUrl(url) => {
                let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
                let notif_msg = match self.language {
                    Language::Spanish => "Abriendo enlace en el navegador...".to_string(),
                    Language::English => "Opening link in browser...".to_string(),
                    Language::Chinese => "正在浏览器中打开链接...".to_string(),
                    Language::Hindi => "ब्राउज़र में लिंक खोला जा रहा है...".to_string(),
                    Language::Arabic => "جاري فتح الرابط في المتصفح...".to_string(),
                    Language::French => "Ouverture du lien dans le navigateur...".to_string(),
                    Language::Bengali => "ব্রাউজারে লিঙ্ক খোলা হচ্ছে...".to_string(),
                    Language::Portuguese => "Abrindo link no navegador...".to_string(),
                    Language::Russian => "Открытие ссылки в браузере...".to_string(),
                    Language::Urdu => "براؤزر میں لنک کھولا جا رہا ہے...".to_string(),
                };
                self.notification = Some((notif_msg, true));
                Task::none()
            }

            Message::CheckForUpdates => {
                self.update_status = UpdateStatus::Checking;
                Task::future(async {
                    let status = UpdateChecker::check_for_updates().await;
                    Message::UpdateStatusReceived(status)
                })
            }

            Message::UpdateStatusReceived(status) => {
                self.update_status = status;
                Task::none()
            }

            Message::OpenUpdateModal => {
                self.is_update_modal_open = true;
                self.is_about_open = false;
                self.is_settings_open = false;
                self.is_speed_test_open = false;
                self.is_timeline_open = false;
                self.is_audit_open = false;
                self.is_wifi_analyzer_open = false;
                self.is_upnp_open = false;
                self.is_storage_explorer_open = false;
                Task::none()
            }

            Message::CloseUpdateModal => {
                self.is_update_modal_open = false;
                self.update_progress = InAppUpdateProgress::Idle;
                Task::none()
            }

            Message::StartInAppUpdate => {
                self.update_progress = InAppUpdateProgress::AuthorizingAndInstalling;
                Task::future(async {
                    let res = UpdateChecker::apply_update_in_app().await;
                    Message::InAppUpdateResult(res)
                })
            }

            Message::InAppUpdateResult(result) => {
                match result {
                    Ok(msg) => {
                        self.update_progress = InAppUpdateProgress::Success;
                        self.notification = Some((msg, true));
                    }
                    Err(err) => {
                        self.update_progress = InAppUpdateProgress::Failed(err.clone());
                        self.notification = Some((err, false));
                    }
                }
                Task::none()
            }

            Message::OpenTerminalUpdater => {
                if let Err(e) = UpdateChecker::launch_terminal_updater() {
                    self.notification = Some((e, false));
                }
                Task::none()
            }

            Message::RestartNetLens => {
                UpdateChecker::restart_application();
                Task::none()
            }

            Message::LanguageChanged(lang) => {
                self.language = lang;
                self.sync_tray_state();
                self.save_state();
                let notif_msg = match lang {
                    Language::Spanish => format!("Idioma cambiado a {}", lang.native_name()),
                    Language::English => format!("Language changed to {}", lang.native_name()),
                    Language::Chinese => format!("语言已切换为 {}", lang.native_name()),
                    Language::Hindi => format!("भाषा बदलकर {} कर दी गई", lang.native_name()),
                    Language::Arabic => format!("تم تغيير اللغة إلى {}", lang.native_name()),
                    Language::French => format!("Langue changée en {}", lang.native_name()),
                    Language::Bengali => format!("ভাষা পরিবর্তন করে {} করা হয়েছে", lang.native_name()),
                    Language::Portuguese => format!("Idioma alterado para {}", lang.native_name()),
                    Language::Russian => format!("Язык изменен на {}", lang.native_name()),
                    Language::Urdu => format!("زبان تبدیل ہو کر {} ہو گئی", lang.native_name()),
                };
                self.notification = Some((notif_msg, true));
                Task::none()
            }

            Message::ThemeChanged(theme) => {
                self.current_theme = theme;
                self.save_state();
                Task::none()
            }

            Message::AutoScanIntervalChanged(secs) => {
                self.auto_scan_interval_secs = secs;
                self.last_scan_time = Instant::now();
                self.save_state();
                Task::none()
            }

            Message::ToggleAutostart(enabled) => {
                self.autostart_enabled = enabled;
                if let Err(e) = AutostartManager::set_enabled(enabled) {
                    self.notification = Some((format!("Error autostart: {}", e), false));
                }
                self.save_state();
                Task::none()
            }

            Message::ToggleMinimizeOnClose(enabled) => {
                self.minimize_on_close = enabled;
                self.save_state();
                Task::none()
            }

            // Fast Disconnection Detection (Heartbeat)
            Message::ToggleHeartbeat(enabled) => {
                self.notification_settings.fast_disconnection_heartbeat = enabled;
                self.save_state();
                Task::none()
            }

            Message::HeartbeatResultsReceived(results) => {
                let mut state_changed = false;

                for (ip, mac, name, is_alive) in results {
                    let mac_key = mac.trim().to_lowercase();
                    if is_alive {
                        self.missed_heartbeats.insert(ip, 0);
                    } else {
                        let misses = self.missed_heartbeats.entry(ip).or_insert(0);
                        *misses += 1;

                        if *misses >= 2 {
                            let was_online = self.active_online_macs.remove(&mac_key);

                            let vendor = self
                                .devices
                                .iter()
                                .find(|d| d.mac.trim().to_lowercase() == mac_key || d.ip == ip)
                                .map(|d| d.vendor.clone())
                                .unwrap_or_default();

                            if let Some(dev) = self.devices.iter_mut().find(|d| d.mac.trim().to_lowercase() == mac_key || d.ip == ip) {
                                if dev.status == DeviceStatus::Online {
                                    dev.status = DeviceStatus::Offline;
                                    dev.last_seen = chrono::Local::now();
                                    state_changed = true;
                                }
                            }

                            if was_online {
                                state_changed = true;
                                if self.initial_scan_completed {
                                    if self.notification_settings.enabled && self.notification_settings.notify_device_disconnected {
                                        let title = Translations::notif_device_disconnected_title(self.language);
                                        let body = self.notification_settings.format_device_message(
                                            "Desconectado",
                                            &name,
                                            ip,
                                            &mac,
                                            &vendor,
                                            Some("El dispositivo no respondió a los sondeos Heartbeat"),
                                        );
                                        self.notification_settings.dispatch(title, &body, "network-offline", None);
                                    }

                                    let event = HistoryManager::create_event(
                                        NetworkEventType::DeviceDisconnected,
                                        "Dispositivo Desconectado (Heartbeat)".to_string(),
                                        format!("{} ({}) ya no responde en la red local.", name, ip),
                                        Some(ip),
                                        Some(mac.clone()),
                                    );
                                    self.network_events.insert(0, event);
                                }
                            }
                        }
                    }
                }

                if self.network_events.len() > 150 {
                    self.network_events.truncate(150);
                }

                if state_changed {
                    self.sync_tray_state();
                    self.save_state();
                }

                Task::none()
            }

            // Real-Time Monitoring (Kernel Netlink)
            Message::ToggleRealtimeMonitor(enabled) => {
                self.notification_settings.realtime_netlink_monitor = enabled;
                self.save_state();
                Task::none()
            }

            Message::RealtimeNeighborEvent(action) => {
                if !self.notification_settings.realtime_netlink_monitor {
                    return Task::none();
                }

                match action {
                    NeighborAction::Appeared { ip, mac, iface } => {
                        let mac_key = mac.trim().to_lowercase();
                        if self.network.local_ip == Some(ip) {
                            return Task::none();
                        }

                        // Debounce: prevent processing multiple transitions of the same device within 4 seconds
                        if let Some(prev) = self.last_realtime_events.get(&mac_key) {
                            if prev.elapsed() < Duration::from_secs(4) {
                                return Task::none();
                            }
                        }
                        self.last_realtime_events.insert(mac_key.clone(), Instant::now());

                        let existing_idx = self.devices.iter().position(|d| d.mac.trim().to_lowercase() == mac_key || d.ip == ip);
                        let is_already_online = existing_idx.map(|idx| self.devices[idx].status == DeviceStatus::Online).unwrap_or(false);
                        let is_in_active_macs = self.active_online_macs.contains(&mac_key);

                        if is_already_online && is_in_active_macs {
                            if let Some(idx) = existing_idx {
                                self.devices[idx].last_seen = chrono::Local::now();
                            }
                            self.missed_heartbeats.insert(ip, 0);
                            return Task::none();
                        }

                        // Device just appeared or reconnected: quickly enrich
                        let oui_db = self.oui_db.clone();
                        Task::future(async move {
                            let vendor = Scanner::lookup_vendor(oui_db.as_deref(), &mac);
                            let discovery = crate::discovery::Discovery::probe(ip).await;
                            Message::RealtimeDeviceEnriched {
                                ip,
                                mac,
                                iface,
                                vendor,
                                hostname: discovery.hostname,
                                model_name: discovery.model_or_friendly_name,
                                services: discovery.services,
                            }
                        })
                    }
                    NeighborAction::Disappeared { ip, mac, .. } => {
                        let target_idx = self.devices.iter().position(|d| {
                            if let Some(m) = &mac {
                                d.mac.trim().to_lowercase() == m.trim().to_lowercase() || d.ip == ip
                            } else {
                                d.ip == ip
                            }
                        });

                        if let Some(idx) = target_idx {
                            if self.devices[idx].status == DeviceStatus::Online {
                                self.devices[idx].status = DeviceStatus::Offline;
                                let dev_mac = self.devices[idx].mac.trim().to_lowercase();
                                self.active_online_macs.remove(&dev_mac);

                                if self.initial_scan_completed && !self.devices[idx].is_gateway && !self.devices[idx].is_current_host {
                                    if self.notification_settings.enabled && self.notification_settings.notify_device_disconnected {
                                        let title = Translations::notif_device_disconnected_title(self.language);
                                        let body = self.notification_settings.format_device_message(
                                            "Desconectado",
                                            &self.devices[idx].display_name(),
                                            self.devices[idx].ip,
                                            &self.devices[idx].mac,
                                            &self.devices[idx].vendor,
                                            Some("El kernel Linux detectó la desconexión del equipo"),
                                        );
                                        self.notification_settings.dispatch(title, &body, "network-offline", None);
                                    }

                                    let event = HistoryManager::create_event(
                                        NetworkEventType::DeviceDisconnected,
                                        "Dispositivo Desconectado (Tiempo Real)".to_string(),
                                        format!("{} ({}) ya no responde en la red local.", self.devices[idx].display_name(), self.devices[idx].ip),
                                        Some(self.devices[idx].ip),
                                        Some(self.devices[idx].mac.clone()),
                                    );
                                    self.network_events.insert(0, event);
                                    if self.network_events.len() > 150 {
                                        self.network_events.truncate(150);
                                    }
                                }
                                self.sync_tray_state();
                            }
                        }
                        Task::none()
                    }
                }
            }

            Message::RealtimeDeviceEnriched { ip, mac, iface, vendor, hostname, model_name, services } => {
                let mac_key = mac.trim().to_lowercase();
                let is_new_device = !self.known_macs.contains(&mac_key);
                let is_gateway = self.network.gateway_ip == Some(ip);
                let is_current = self.network.local_ip == Some(ip);

                let now = chrono::Local::now();
                let is_randomized = Scanner::is_randomized_mac(&mac);

                let existing_idx = self.devices.iter().position(|d| d.mac.trim().to_lowercase() == mac_key || d.ip == ip);
                let (custom_alias, is_whitelisted, is_blocked, speed_limit_kbps, prev_category) = if let Some(meta) = self.device_metadata.get(&mac_key) {
                    (meta.custom_alias.clone(), meta.is_whitelisted, meta.is_blocked, meta.speed_limit_kbps, meta.category.unwrap_or(DeviceCategory::Unknown))
                } else if let Some(idx) = existing_idx {
                    (self.devices[idx].custom_alias.clone(), self.devices[idx].is_whitelisted, self.devices[idx].is_blocked, self.devices[idx].speed_limit_kbps, self.devices[idx].category)
                } else {
                    (None, false, false, None, DeviceCategory::Unknown)
                };

                let category = if prev_category != DeviceCategory::Unknown {
                    prev_category
                } else {
                    Scanner::categorize_device(is_gateway, is_current, hostname.as_deref(), &vendor, &[])
                };

                let dev = Device {
                    ip,
                    mac: mac.clone(),
                    vendor: vendor.clone(),
                    hostname: hostname.clone(),
                    category,
                    is_randomized_mac: is_randomized,
                    is_gateway,
                    is_current_host: is_current,
                    interface: iface.clone(),
                    latency_ms: Some(1.0),
                    open_ports: vec![],
                    status: DeviceStatus::Online,
                    first_seen: now,
                    last_seen: now,
                    custom_alias,
                    is_whitelisted,
                    is_blocked,
                    speed_limit_kbps,
                    connection_type: None,
                    wifi_signal_dbm: None,
                    negotiated_mbps: None,
                    model_name,
                    discovered_services: services,
                    request_history: vec![],
                };

                self.active_online_macs.insert(mac_key.clone());
                self.known_macs.insert(mac_key.clone());
                self.missed_heartbeats.insert(ip, 0);

                let mut notif_task = Task::none();

                if is_new_device {
                    let event = HistoryManager::create_event(
                        NetworkEventType::NewDeviceDetected,
                        "Nuevo Dispositivo (Tiempo Real)".to_string(),
                        format!("{} ({}) detectado instantáneamente.", dev.display_name(), dev.ip),
                        Some(dev.ip),
                        Some(dev.mac.clone()),
                    );
                    self.network_events.insert(0, event);

                    if !dev.is_current_host && !dev.is_gateway && !dev.is_whitelisted {
                        if self.alerted_macs.insert(mac_key.clone()) {
                            self.pending_intruder_alert = Some(IntruderAlert {
                                ip: dev.ip,
                                mac: dev.mac.clone(),
                                vendor: dev.vendor.clone(),
                                hostname: dev.hostname.clone(),
                            });

                            if self.notification_settings.enabled && self.notification_settings.notify_unknown_intruders {
                                let dev_ip = dev.ip;
                                let dev_mac = dev.mac.clone();
                                let alert_title = Translations::intruder_alert_title(self.language);
                                let trust_label = Translations::intruder_btn_trust(self.language);
                                let block_label = Translations::intruder_btn_block(self.language);
                                let body = self.notification_settings.format_device_message(
                                    "Nuevo Dispositivo Desconocido",
                                    &dev.display_name(),
                                    dev.ip,
                                    &dev.mac,
                                    &dev.vendor,
                                    Some("Detectado instantáneamente por el kernel"),
                                );

                                let notif_settings = self.notification_settings.clone();
                                notif_task = Task::future(async move {
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
                } else {
                    // Known device reconnecting in real time
                    if !dev.is_current_host && !dev.is_gateway && !dev.is_whitelisted {
                        // *** INSTANT ALERT: NON-WHITELIST RECONNECTION ***
                        let event = HistoryManager::create_event(
                            NetworkEventType::SecurityAlert,
                            "Reconexión No Autorizada (Tiempo Real)".to_string(),
                            format!("{} ({}) no está en la Lista Blanca y se ha reconectado.", dev.display_name(), dev.ip),
                            Some(dev.ip),
                            Some(dev.mac.clone()),
                        );
                        self.network_events.insert(0, event);

                        if self.notification_settings.enabled && self.notification_settings.notify_known_unwhitelisted_reconnect {
                            if self.pending_intruder_alert.is_none() {
                                self.pending_intruder_alert = Some(IntruderAlert {
                                    ip: dev.ip,
                                    mac: dev.mac.clone(),
                                    vendor: dev.vendor.clone(),
                                    hostname: dev.hostname.clone(),
                                });
                            }

                            let dev_ip = dev.ip;
                            let dev_mac = dev.mac.clone();
                            let alert_title = Translations::notif_reconnect_unwhitelisted_title(self.language);
                            let trust_label = Translations::intruder_btn_trust(self.language);
                            let block_label = Translations::intruder_btn_block(self.language);
                            let body = self.notification_settings.format_device_message(
                                "Reconexión No Autorizada",
                                &dev.display_name(),
                                dev.ip,
                                &dev.mac,
                                &dev.vendor,
                                Some("Detección instantánea en tiempo real"),
                            );

                            let notif_settings = self.notification_settings.clone();
                            notif_task = Task::future(async move {
                                let action = notif_settings.dispatch_interactive_alert(alert_title, &body, trust_label, block_label).await;
                                Message::DesktopNotificationAction {
                                    ip: dev_ip,
                                    mac: dev_mac,
                                    action,
                                }
                            });
                        }
                    } else if dev.is_whitelisted {
                        let event = HistoryManager::create_event(
                            NetworkEventType::DeviceConnected,
                            "Dispositivo Conectado (Tiempo Real)".to_string(),
                            format!("{} ({}) conectado a la red.", dev.display_name(), dev.ip),
                            Some(dev.ip),
                            Some(dev.mac.clone()),
                        );
                        self.network_events.insert(0, event);

                        if self.notification_settings.enabled && self.notification_settings.notify_whitelisted_reconnect {
                            let title = Translations::notif_reconnect_whitelisted_title(self.language);
                            let body = self.notification_settings.format_device_message(
                                "Conectado",
                                &dev.display_name(),
                                dev.ip,
                                &dev.mac,
                                &dev.vendor,
                                Some("Dispositivo de confianza en tiempo real"),
                            );
                            self.notification_settings.dispatch(title, &body, "network-wireless", None);
                        }
                    }
                }

                if let Some(idx) = existing_idx {
                    let prev_req = self.devices[idx].request_history.clone();
                    self.devices[idx] = dev;
                    if !prev_req.is_empty() {
                        self.devices[idx].request_history = prev_req;
                    }
                } else {
                    self.devices.push(dev);
                }

                if self.network_events.len() > 150 {
                    self.network_events.truncate(150);
                }
                self.sync_tray_state();
                notif_task
            }

            // Desktop Notification Management & Settings
            Message::ToggleNotificationsMaster(enabled) => {
                self.notification_settings.enabled = enabled;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotifyUnknown(enabled) => {
                self.notification_settings.notify_unknown_intruders = enabled;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotifyKnownUnwhitelisted(enabled) => {
                self.notification_settings.notify_known_unwhitelisted_reconnect = enabled;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotifyWhitelisted(enabled) => {
                self.notification_settings.notify_whitelisted_reconnect = enabled;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotifyDisconnected(enabled) => {
                self.notification_settings.notify_device_disconnected = enabled;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotifyOutages(enabled) => {
                self.notification_settings.notify_internet_outages = enabled;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotifySecurity(enabled) => {
                self.notification_settings.notify_security_critical = enabled;
                self.save_state();
                Task::none()
            }

            Message::ChangeNotificationUrgency(urgency) => {
                self.notification_settings.urgency = urgency;
                self.save_state();
                Task::none()
            }

            Message::ChangeNotificationDetail(detail) => {
                self.notification_settings.detail_level = detail;
                self.save_state();
                Task::none()
            }

            Message::ChangeNotificationTimeout(secs) => {
                self.notification_settings.timeout_secs = secs;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotificationShowMac(enabled) => {
                self.notification_settings.show_mac = enabled;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotificationShowVendor(enabled) => {
                self.notification_settings.show_vendor = enabled;
                self.save_state();
                Task::none()
            }

            Message::ToggleNotificationPlaySound(enabled) => {
                self.notification_settings.play_sound = enabled;
                self.save_state();
                Task::none()
            }

            Message::NotificationPrefixChanged(prefix) => {
                self.notification_prefix_input = prefix.clone();
                self.notification_settings.custom_prefix = prefix;
                self.save_state();
                Task::none()
            }

            Message::SendTestNotification => {
                let title = "Notificación de Prueba";
                let body = self.notification_settings.format_device_message(
                    "Alerta de Prueba",
                    "Equipo-NetLens",
                    Ipv4Addr::new(192, 168, 1, 99),
                    "AA:BB:CC:DD:EE:FF",
                    "NetLens Technology",
                    Some("Verificación de configuración y estilo de notificación"),
                );
                self.notification_settings.dispatch(title, &body, "network-wireless", None);
                self.notification = Some(("Notificación de prueba enviada al escritorio.".to_string(), true));
                Task::none()
            }

            Message::WindowCloseRequested(id) => {
                if self.minimize_on_close {
                    // Close and destroy window completely so it disappears from taskbar,
                    // leaving only the system tray icon.
                    self.active_window_id = None;
                    crate::trim_memory();
                    iced::window::close(id)
                } else {
                    std::process::exit(0);
                }
            }

            Message::WindowClosed(id) => {
                if self.active_window_id == Some(id) {
                    self.active_window_id = None;
                }
                crate::trim_memory();
                Task::none()
            }

            Message::RestoreWindowFromTray => {
                if let Some(id) = self.active_window_id {
                    Task::batch(vec![
                        iced::window::minimize(id, false),
                        iced::window::gain_focus(id),
                    ])
                } else {
                    // If window was closed when minimizing to background, reopen cleanly
                    let (id, open_task) = iced::window::open(default_window_settings());
                    self.active_window_id = Some(id);
                    Task::batch(vec![
                        open_task.discard(),
                        iced::window::gain_focus(id),
                    ])
                }
            }

            Message::ExitApp => {
                std::process::exit(0);
            }

            Message::DesktopNotificationAction { ip, mac, action } => {
                if action == "trust" {
                    let mac_clone = mac.clone();
                    let title = Translations::notif_trust_success_title(self.language);
                    let body = Translations::notif_trust_success_body(self.language, &mac_clone);
                    self.notification_settings.dispatch(title, &body, "network-wireless", Some(NotificationUrgency::Normal));
                    Task::done(Message::TrustDevice(mac))
                } else if action == "block" {
                    if self.router_password_input.trim().is_empty() {
                        let title = Translations::notif_require_pass_title(self.language);
                        let body = Translations::notif_require_pass_body(self.language);
                        self.notification_settings.dispatch(title, body, "dialog-warning", Some(NotificationUrgency::Critical));
                        Task::none()
                    } else {
                        let mac_clone = mac.clone();
                        let title = Translations::notif_block_success_title(self.language);
                        let body = Translations::notif_block_success_body(self.language, &mac_clone);
                        self.notification_settings.dispatch(title, &body, "network-wireless", Some(NotificationUrgency::Normal));
                        Task::done(Message::ToggleDeviceBlocked(ip, mac))
                    }
                } else {
                    Task::none()
                }
            }

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
                    if self.speed_test_history.len() > 30 {
                        self.speed_test_history.truncate(30);
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
                self.save_state();
                Task::none()
            }

            Message::RouterPasswordChanged(pass) => {
                self.router_password_input = pass;
                self.save_state();
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
                self.save_state();
                self.notification = Some(("Dispositivo verificado y añadido a Lista Blanca.".to_string(), true));
                Task::none()
            }

            Message::DismissIntruderAlert => {
                if let Some(alert) = &self.pending_intruder_alert {
                    self.known_macs.insert(alert.mac.trim().to_lowercase());
                }
                self.pending_intruder_alert = None;
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
            Message::OpenStorageExplorer => {
                self.is_storage_explorer_open = true;
                self.is_upnp_open = false;
                self.is_settings_open = false;
                self.is_about_open = false;
                self.is_speed_test_open = false;
                self.is_timeline_open = false;
                self.is_audit_open = false;
                self.is_wifi_analyzer_open = false;
                self.storage_error_msg = None;
                self.storage_status_msg = None;

                if self.storage_shares.is_empty() {
                    return Task::done(Message::RefreshStorageShares);
                }
                Task::none()
            }

            Message::CloseStorageExplorer => {
                self.is_storage_explorer_open = false;
                self.is_storage_creds_open = false;
                self.storage_selected_file = None;
                self.file_to_delete = None;
                self.is_creating_folder = false;
                Task::none()
            }

            Message::RefreshStorageShares => {
                self.is_storage_loading = true;
                self.storage_error_msg = None;

                if self.is_storage_demo_mode {
                    let shares = RouterStorageManager::get_demo_shares();
                    return Task::done(Message::StorageSharesLoaded(Ok(shares)));
                }

                let target_ip: Ipv4Addr = match self.storage_target_ip.trim().parse() {
                    Ok(ip) => ip,
                    Err(_) => {
                        self.is_storage_loading = false;
                        self.storage_error_msg = Some(format!("IP inválida: '{}'", self.storage_target_ip));
                        return Task::none();
                    }
                };

                let creds = self.storage_credentials.clone();
                Task::future(async move {
                    let res = RouterStorageManager::list_shares(target_ip, &creds).await;
                    Message::StorageSharesLoaded(res)
                })
            }

            Message::StorageSharesLoaded(res) => {
                self.is_storage_loading = false;
                match res {
                    Ok(shares) => {
                        self.storage_shares = shares;
                        self.storage_error_msg = None;
                        if self.storage_active_share.is_none()
                            || !self
                                .storage_shares
                                .iter()
                                .any(|s| Some(&s.name) == self.storage_active_share.as_ref())
                        {
                            if let Some(first) = self.storage_shares.first() {
                                self.storage_active_share = Some(first.name.clone());
                                self.storage_current_path.clear();
                                return Task::done(Message::StorageRefreshDirectory);
                            }
                        }
                    }
                    Err(err) => {
                        self.storage_error_msg = Some(err);
                    }
                }
                Task::none()
            }

            Message::StorageShareSelected(share_name) => {
                self.storage_active_share = Some(share_name);
                self.storage_current_path.clear();
                self.storage_selected_file = None;
                Task::done(Message::StorageRefreshDirectory)
            }

            Message::StorageNavigateTo(subpath) => {
                self.storage_current_path = subpath;
                self.storage_selected_file = None;
                Task::done(Message::StorageRefreshDirectory)
            }

            Message::StorageNavigateUp => {
                if !self.storage_current_path.is_empty() {
                    let mut parts: Vec<&str> = self
                        .storage_current_path
                        .trim_matches('/')
                        .split('/')
                        .collect();
                    parts.pop();
                    self.storage_current_path = parts.join("/");
                    self.storage_selected_file = None;
                    return Task::done(Message::StorageRefreshDirectory);
                }
                Task::none()
            }

            Message::StorageRefreshDirectory => {
                let share = match &self.storage_active_share {
                    Some(s) => s.clone(),
                    None => return Task::none(),
                };

                self.is_storage_loading = true;
                self.storage_error_msg = None;

                if self.is_storage_demo_mode {
                    let clean = self.storage_current_path.trim_matches('/').to_string();
                    let files = self.demo_storage_files.entry(clean).or_insert_with(Vec::new).clone();
                    let disk = Some(self.demo_storage_disk);
                    return Task::done(Message::StorageDirectoryLoaded(Ok((files, disk))));
                }

                let target_ip: Ipv4Addr = match self.storage_target_ip.trim().parse() {
                    Ok(ip) => ip,
                    Err(_) => {
                        self.is_storage_loading = false;
                        self.storage_error_msg = Some(format!("IP inválida: '{}'", self.storage_target_ip));
                        return Task::none();
                    }
                };

                let path = self.storage_current_path.clone();
                let creds = self.storage_credentials.clone();

                Task::future(async move {
                    let res = RouterStorageManager::list_directory(target_ip, &share, &path, &creds).await;
                    Message::StorageDirectoryLoaded(res)
                })
            }

            Message::StorageDirectoryLoaded(res) => {
                self.is_storage_loading = false;
                match res {
                    Ok((files, disk_info)) => {
                        self.storage_files = files;
                        if let Some(mut d) = disk_info {
                            // Preserve freed space update if router reports cached data
                            if let Some(curr) = self.storage_disk_info {
                                if self.storage_last_freed.is_some() && d.free_bytes < curr.free_bytes {
                                    d.free_bytes = curr.free_bytes;
                                }
                            }
                            self.storage_disk_info = Some(d);
                        }
                        self.storage_error_msg = None;
                    }
                    Err(err) => {
                        self.storage_error_msg = Some(err);
                    }
                }
                Task::none()
            }

            Message::StorageTargetIpChanged(ip_str) => {
                self.storage_target_ip = ip_str;
                Task::none()
            }

            Message::StorageFilterQueryChanged(q) => {
                self.storage_filter_query = q;
                Task::none()
            }

            Message::StorageToggleDemoMode => {
                self.is_storage_demo_mode = !self.is_storage_demo_mode;
                self.storage_shares.clear();
                self.storage_files.clear();
                self.storage_active_share = None;
                self.storage_current_path.clear();
                self.storage_error_msg = None;
                self.storage_status_msg = if self.is_storage_demo_mode {
                    Some("Modo Demo activado con almacenamiento simulado".to_string())
                } else {
                    Some("Conexión real activada".to_string())
                };
                Task::done(Message::RefreshStorageShares)
            }

            Message::ToggleStorageCredentialsModal => {
                self.is_storage_creds_open = !self.is_storage_creds_open;
                Task::none()
            }

            Message::StorageCredentialsUserChanged(u) => {
                self.storage_credentials.username = u;
                Task::none()
            }

            Message::StorageCredentialsPassChanged(p) => {
                self.storage_credentials.password = p;
                Task::none()
            }

            Message::StorageCredentialsWorkgroupChanged(w) => {
                self.storage_credentials.workgroup = w;
                Task::none()
            }

            Message::StorageSaveCredentials => {
                self.is_storage_creds_open = false;
                self.save_state();
                Task::done(Message::RefreshStorageShares)
            }

            Message::StorageStartCreateFolder => {
                self.is_creating_folder = true;
                self.new_folder_name_input.clear();
                Task::none()
            }

            Message::StorageCreateFolderNameChanged(name) => {
                self.new_folder_name_input = name;
                Task::none()
            }

            Message::StorageCancelCreateFolder => {
                self.is_creating_folder = false;
                self.new_folder_name_input.clear();
                Task::none()
            }

            Message::StorageConfirmCreateFolder => {
                let folder_name = self.new_folder_name_input.trim().to_string();
                if folder_name.is_empty() {
                    return Task::none();
                }
                self.is_creating_folder = false;

                if self.is_storage_demo_mode {
                    let entry = StorageFileEntry::new(
                        folder_name,
                        true,
                        0,
                        chrono::Local::now().format("%d %b %Y %H:%M").to_string(),
                        "D".to_string(),
                    );
                    let clean = self.storage_current_path.trim_matches('/').to_string();
                    self.demo_storage_files.entry(clean).or_default().insert(0, entry.clone());
                    self.storage_files.insert(0, entry);
                    self.storage_status_msg = Some("Carpeta creada (Demo)".to_string());
                    return Task::none();
                }

                let share = match &self.storage_active_share {
                    Some(s) => s.clone(),
                    None => return Task::none(),
                };

                let target_ip: Ipv4Addr = match self.storage_target_ip.trim().parse() {
                    Ok(ip) => ip,
                    Err(_) => return Task::none(),
                };

                let parent_path = self.storage_current_path.clone();
                let creds = self.storage_credentials.clone();

                self.is_storage_loading = true;
                Task::future(async move {
                    let res = RouterStorageManager::create_folder(
                        target_ip,
                        &share,
                        &parent_path,
                        &folder_name,
                        &creds,
                    )
                    .await;
                    match res {
                        Ok(()) => Message::StorageOperationCompleted(Ok("Carpeta creada correctamente".to_string())),
                        Err(e) => Message::StorageOperationCompleted(Err(e)),
                    }
                })
            }

            Message::StorageRequestDeleteFile { path, is_dir, name } => {
                self.file_to_delete = Some((path, is_dir, name));
                Task::none()
            }

            Message::StorageCancelDeleteFile => {
                self.file_to_delete = None;
                Task::none()
            }

            Message::StorageConfirmDeleteFile => {
                let (path, is_dir, name) = match self.file_to_delete.take() {
                    Some(info) => info,
                    None => return Task::none(),
                };

                // Immediate UI Update (Optimistic UI Update)
                // 1. Get entry to determine its size before removal
                let removed_entry = self.storage_files.iter().find(|f| f.name == name).cloned();

                // 2. Immediately remove file or folder from visible list
                self.storage_files.retain(|f| f.name != name);

                // 3. If properties card was open for this item, close it immediately
                if self.storage_selected_file.as_ref().map(|f| f.name == name).unwrap_or(false) {
                    self.storage_selected_file = None;
                }

                // 4. Immediately update capacity bar and usage percentage
                // For folders: compute significant size so usage percentage updates visibly
                let freed_bytes: u64 = if is_dir {
                    if self.is_storage_demo_mode {
                        1_850_000_000 // 1.85 GB in demo mode for clear percentage update
                    } else {
                        1_500_000_000 // 1.5 GB estimate for router folders
                    }
                } else {
                    removed_entry.as_ref().map(|e| e.size_bytes.max(1024)).unwrap_or(10_240)
                };

                self.storage_last_freed = Some(freed_bytes);

                if let Some(disk) = &mut self.storage_disk_info {
                    disk.free_bytes = (disk.free_bytes + freed_bytes).min(disk.total_bytes);
                }

                // 5. Immediately display confirmation message in status bar
                let type_label = if is_dir { "Carpeta" } else { "Archivo" };
                self.is_storage_loading = true;
                self.storage_status_msg = Some(format!(
                    "Eliminando {} '{}'...",
                    type_label,
                    name
                ));
                self.storage_error_msg = None;

                if self.is_storage_demo_mode {
                    let clean = self.storage_current_path.trim_matches('/').to_string();
                    if let Some(list) = self.demo_storage_files.get_mut(&clean) {
                        list.retain(|f| f.name != name);
                    }
                    self.demo_storage_disk.free_bytes = (self.demo_storage_disk.free_bytes + freed_bytes).min(self.demo_storage_disk.total_bytes);
                    self.storage_disk_info = Some(self.demo_storage_disk);
                    self.is_storage_loading = false;
                    self.storage_status_msg = Some(format!(
                        "{} '{}' eliminado correctamente",
                        type_label,
                        name
                    ));
                    return Task::none();
                }

                let share = match &self.storage_active_share {
                    Some(s) => s.clone(),
                    None => {
                        self.is_storage_loading = false;
                        return Task::none();
                    }
                };

                let target_ip: Ipv4Addr = match self.storage_target_ip.trim().parse() {
                    Ok(ip) => ip,
                    Err(_) => {
                        self.is_storage_loading = false;
                        return Task::none();
                    }
                };

                let creds = self.storage_credentials.clone();
                let name_clone = name.clone();

                Task::future(async move {
                    let res = RouterStorageManager::delete_entry(
                        target_ip,
                        &share,
                        &path,
                        is_dir,
                        &creds,
                    )
                    .await;
                    match res {
                        Ok(()) => Message::StorageOperationCompleted(Ok(format!(
                            "{} '{}' eliminado correctamente",
                            type_label,
                            name_clone
                        ))),
                        Err(e) => Message::StorageOperationCompleted(Err(e)),
                    }
                })
            }

            Message::StorageOpenFile(file_name) => {
                let share = match &self.storage_active_share {
                    Some(s) => s.clone(),
                    None => return Task::none(),
                };

                let full_rel_path = if self.storage_current_path.is_empty() {
                    file_name.clone()
                } else {
                    format!("{}/{}", self.storage_current_path.trim_matches('/'), file_name)
                };

                let cache_dir = std::env::temp_dir().join("netlens_preview");
                let _ = std::fs::create_dir_all(&cache_dir);
                let dest_path = cache_dir.join(&file_name);

                if self.is_storage_demo_mode {
                    let dummy_text = format!(
                        "====================================================\n\
                         📄 FILE PREVIEW - NETLENS\n\
                         ====================================================\n\n\
                         File name:       {}\n\
                         Shared resource: {}\n\
                         Router path:     /{}/{}\n\
                         Mode:            Interactive Demo\n\n\
                         This file has been opened automatically in your operating system.\n",
                        file_name, share, share, full_rel_path
                    );
                    let _ = std::fs::write(&dest_path, dummy_text);
                    let _ = std::process::Command::new("xdg-open").arg(&dest_path).spawn();
                    self.storage_status_msg = Some(format!("Opening file '{}'...", file_name));
                    return Task::none();
                }

                let target_ip: Ipv4Addr = match self.storage_target_ip.trim().parse() {
                    Ok(ip) => ip,
                    Err(_) => return Task::none(),
                };

                let creds = self.storage_credentials.clone();
                let file_name_clone = file_name.clone();
                self.is_storage_loading = true;
                self.storage_status_msg = Some(format!("Downloading to preview: '{}'...", file_name));

                Task::future(async move {
                    let res = RouterStorageManager::download_file(
                        target_ip,
                        &share,
                        &full_rel_path,
                        &dest_path,
                        &creds,
                    )
                    .await;
                    match res {
                        Ok(p) => {
                            let _ = std::process::Command::new("xdg-open").arg(&p).spawn();
                            Message::StorageOperationCompleted(Ok(format!("File '{}' opened successfully", file_name_clone)))
                        }
                        Err(e) => Message::StorageOperationCompleted(Err(format!("Error opening file: {}", e))),
                    }
                })
            }

            Message::StorageDownloadFile(file_name) => {
                if self.is_storage_demo_mode {
                    self.notification = Some((format!("Simulated download of '{}' completed.", file_name), true));
                    return Task::none();
                }

                let share = match &self.storage_active_share {
                    Some(s) => s.clone(),
                    None => return Task::none(),
                };

                let target_ip: Ipv4Addr = match self.storage_target_ip.trim().parse() {
                    Ok(ip) => ip,
                    Err(_) => return Task::none(),
                };

                let full_rel_path = if self.storage_current_path.is_empty() {
                    file_name.clone()
                } else {
                    format!("{}/{}", self.storage_current_path.trim_matches('/'), file_name)
                };

                let dest_dir = std::env::var("HOME")
                    .map(|h| PathBuf::from(h).join("Downloads"))
                    .unwrap_or_else(|_| PathBuf::from("/tmp"));
                let dest_path = dest_dir.join(&file_name);

                let creds = self.storage_credentials.clone();
                self.is_storage_loading = true;
                Task::future(async move {
                    let res = RouterStorageManager::download_file(
                        target_ip,
                        &share,
                        &full_rel_path,
                        &dest_path,
                        &creds,
                    )
                    .await;
                    match res {
                        Ok(p) => Message::StorageOperationCompleted(Ok(format!("Downloaded successfully to: {}", p.display()))),
                        Err(e) => Message::StorageOperationCompleted(Err(e)),
                    }
                })
            }

            Message::StoragePickFileAndUpload => {
                if self.storage_active_share.is_none() {
                    self.storage_error_msg = Some("Select a shared resource or drive first".to_string());
                    return Task::none();
                }
                Task::future(async move {
                    let picked = RouterStorageManager::pick_path_with_file_dialog(false, "Select file to upload to Router").await;
                    Message::StorageFilePicked(picked)
                })
            }

            Message::StoragePickFolderAndUpload => {
                if self.storage_active_share.is_none() {
                    self.storage_error_msg = Some("Select a shared resource or drive first".to_string());
                    return Task::none();
                }
                Task::future(async move {
                    let picked = RouterStorageManager::pick_path_with_file_dialog(true, "Select folder to upload to Router").await;
                    Message::StorageFolderPicked(picked)
                })
            }

            Message::StorageFilePicked(picked) => {
                let local_path = match picked {
                    Some(p) => p,
                    None => return Task::none(),
                };

                let file_name = match local_path.file_name().and_then(|s| s.to_str()) {
                    Some(name) => name.to_string(),
                    None => {
                        self.storage_error_msg = Some("Invalid file name".to_string());
                        return Task::none();
                    }
                };

                let file_size = std::fs::metadata(&local_path).map(|m| m.len()).unwrap_or(1024);
                let target_dir = if self.storage_current_path.is_empty() {
                    "Root".to_string()
                } else {
                    self.storage_current_path.clone()
                };

                if self.is_storage_demo_mode {
                    let (tx, rx) = mpsc::unbounded::<StorageUploadEvent>();
                    let file_name_clone = file_name.clone();
                    let target_dir_clone = target_dir.clone();

                    tokio::spawn(async move {
                        let _ = tx.unbounded_send(StorageUploadEvent::Started {
                            is_folder: false,
                            title: format!("Uploading file: {}", file_name_clone),
                            total_items: 1,
                            total_bytes: file_size,
                            target_dir: target_dir_clone,
                        });
                        for step in 1..=20 {
                            tokio::time::sleep(tokio::time::Duration::from_millis(75)).await;
                            let p = (step as f32 / 20.0) * 100.0;
                            let bytes = ((p / 100.0) * file_size as f32) as u64;
                            let speed = 18.5 + (step as f64 * 0.4).sin() * 2.3;
                            let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                                current_item: file_name_clone.clone(),
                                current_index: 1,
                                total_items: 1,
                                bytes_uploaded: bytes,
                                total_bytes: file_size,
                                speed_str: format!("{:.1} MB/s", speed),
                                percent: p,
                            });
                        }
                        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
                        let _ = tx.unbounded_send(StorageUploadEvent::Completed {
                            message: format!("File '{}' uploaded successfully (Demo)", file_name_clone),
                            total_items: 1,
                            total_bytes: file_size,
                        });
                    });

                    let entry = StorageFileEntry::new(
                        file_name.clone(),
                        false,
                        file_size,
                        "Just now".to_string(),
                        "A".to_string(),
                    );
                    let clean = self.storage_current_path.trim_matches('/').to_string();
                    self.demo_storage_files.entry(clean).or_default().push(entry.clone());
                    self.storage_files.push(entry);

                    return Task::run(rx, Message::StorageUploadEventReceived);
                }

                let share = match &self.storage_active_share {
                    Some(s) => s.clone(),
                    None => return Task::none(),
                };

                let target_ip: Ipv4Addr = match self.storage_target_ip.trim().parse() {
                    Ok(ip) => ip,
                    Err(_) => {
                        self.storage_error_msg = Some(format!("Invalid IP: '{}'", self.storage_target_ip));
                        return Task::none();
                    }
                };

                let remote_dest_path = if self.storage_current_path.is_empty() {
                    file_name.clone()
                } else {
                    format!("{}/{}", self.storage_current_path.trim_matches('/'), file_name)
                };

                let creds = self.storage_credentials.clone();
                let (tx, rx) = mpsc::unbounded::<StorageUploadEvent>();
                let local_path_clone = local_path.clone();

                tokio::spawn(async move {
                    let res = RouterStorageManager::upload_file_with_progress(
                        target_ip,
                        &share,
                        &local_path_clone,
                        &remote_dest_path,
                        &creds,
                        Some(tx.clone()),
                    )
                    .await;
                    if let Err(e) = res {
                        let _ = tx.unbounded_send(StorageUploadEvent::Error(e));
                    }
                });

                Task::run(rx, Message::StorageUploadEventReceived)
            }

            Message::StorageFolderPicked(picked) => {
                let local_path = match picked {
                    Some(p) => p,
                    None => return Task::none(),
                };

                let folder_name = match local_path.file_name().and_then(|s| s.to_str()) {
                    Some(name) => name.to_string(),
                    None => {
                        self.storage_error_msg = Some("Invalid folder name".to_string());
                        return Task::none();
                    }
                };

                let target_dir = if self.storage_current_path.is_empty() {
                    "Root".to_string()
                } else {
                    self.storage_current_path.clone()
                };

                if self.is_storage_demo_mode {
                    let (tx, rx) = mpsc::unbounded::<StorageUploadEvent>();
                    let folder_name_clone = folder_name.clone();
                    let target_dir_clone = target_dir.clone();
                    let total_files = 8;
                    let total_bytes = 48_500_000;

                    tokio::spawn(async move {
                        let _ = tx.unbounded_send(StorageUploadEvent::Started {
                            is_folder: true,
                            title: format!("Uploading folder: {} ({} files)", folder_name_clone, total_files),
                            total_items: total_files,
                            total_bytes,
                            target_dir: target_dir_clone,
                        });
                        for i in 1..=total_files {
                            for sub in 1..=3 {
                                tokio::time::sleep(tokio::time::Duration::from_millis(70)).await;
                                let sub_ratio = ((i - 1) as f32 + (sub as f32 / 3.0)) / (total_files as f32);
                                let p = sub_ratio * 100.0;
                                let bytes = (sub_ratio * total_bytes as f32) as u64;
                                let speed = 21.4 + ((i * 3 + sub) as f64 * 0.5).sin() * 1.8;
                                let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                                    current_item: format!("file_{:02}.dat", i),
                                    current_index: i,
                                    total_items: total_files,
                                    bytes_uploaded: bytes,
                                    total_bytes,
                                    speed_str: format!("{:.1} MB/s", speed),
                                    percent: p.min(99.0),
                                });
                            }
                        }
                        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
                        let _ = tx.unbounded_send(StorageUploadEvent::Completed {
                            message: format!("Folder '{}' uploaded successfully ({} files, Demo)", folder_name_clone, total_files),
                            total_items: total_files,
                            total_bytes,
                        });
                    });

                    let entry = StorageFileEntry::new(
                        folder_name.clone(),
                        true,
                        0,
                        "Just now".to_string(),
                        "D".to_string(),
                    );
                    let clean = self.storage_current_path.trim_matches('/').to_string();
                    self.demo_storage_files.entry(clean).or_default().push(entry.clone());
                    self.storage_files.push(entry);

                    return Task::run(rx, Message::StorageUploadEventReceived);
                }

                let share = match &self.storage_active_share {
                    Some(s) => s.clone(),
                    None => return Task::none(),
                };

                let target_ip: Ipv4Addr = match self.storage_target_ip.trim().parse() {
                    Ok(ip) => ip,
                    Err(_) => {
                        self.storage_error_msg = Some(format!("Invalid IP: '{}'", self.storage_target_ip));
                        return Task::none();
                    }
                };

                let remote_parent_dir = self.storage_current_path.clone();
                let creds = self.storage_credentials.clone();
                let (tx, rx) = mpsc::unbounded::<StorageUploadEvent>();
                let local_path_clone = local_path.clone();

                tokio::spawn(async move {
                    let res = RouterStorageManager::upload_directory_with_progress(
                        target_ip,
                        &share,
                        &local_path_clone,
                        &remote_parent_dir,
                        &creds,
                        Some(tx.clone()),
                    )
                    .await;
                    if let Err(e) = res {
                        let _ = tx.unbounded_send(StorageUploadEvent::Error(e));
                    }
                });

                Task::run(rx, Message::StorageUploadEventReceived)
            }

            Message::StorageUploadEventReceived(event) => match event {
                StorageUploadEvent::Started {
                    is_folder,
                    title,
                    total_items,
                    total_bytes,
                    target_dir,
                } => {
                    self.is_storage_loading = true;
                    self.storage_upload_progress = Some(StorageUploadProgress {
                        is_folder,
                        title,
                        target_dir,
                        current_item: String::new(),
                        current_index: 0,
                        total_items,
                        bytes_uploaded: 0,
                        total_bytes,
                        speed_str: "Iniciando...".to_string(),
                        percent: 0.0,
                        is_finished: false,
                        error: None,
                    });
                    Task::none()
                }
                StorageUploadEvent::Progress {
                    current_item,
                    current_index,
                    total_items,
                    bytes_uploaded,
                    total_bytes,
                    speed_str,
                    percent,
                } => {
                    if let Some(p) = &mut self.storage_upload_progress {
                        p.current_item = current_item;
                        p.current_index = current_index;
                        p.total_items = total_items;
                        p.bytes_uploaded = bytes_uploaded;
                        p.total_bytes = total_bytes;
                        p.speed_str = speed_str;
                        p.percent = percent.clamp(0.0, 100.0);
                    }
                    Task::none()
                }
                StorageUploadEvent::Completed {
                    message,
                    total_items: _,
                    total_bytes,
                } => {
                    self.is_storage_loading = false;
                    if let Some(p) = &mut self.storage_upload_progress {
                        p.percent = 100.0;
                        p.bytes_uploaded = total_bytes;
                        p.speed_str = "Completado".to_string();
                        p.is_finished = true;
                    }
                    self.storage_status_msg = Some(message);

                    // Update free disk space subtracting uploaded size
                    if self.is_storage_demo_mode {
                        self.demo_storage_disk.free_bytes = self.demo_storage_disk.free_bytes.saturating_sub(total_bytes);
                        self.storage_disk_info = Some(self.demo_storage_disk);
                    } else if let Some(disk) = &mut self.storage_disk_info {
                        disk.free_bytes = disk.free_bytes.saturating_sub(total_bytes);
                    }
                    self.storage_last_freed = None;

                    Task::done(Message::StorageRefreshDirectory)
                }
                StorageUploadEvent::Error(err) => {
                    self.is_storage_loading = false;
                    if let Some(p) = &mut self.storage_upload_progress {
                        p.is_finished = true;
                        p.error = Some(err.clone());
                    }
                    self.storage_error_msg = Some(err);
                    Task::none()
                }
            },

            Message::StorageDismissUploadProgress => {
                self.storage_upload_progress = None;
                Task::none()
            }

            Message::StorageShowFileProperties(entry) => {
                self.storage_selected_file = Some(entry);
                Task::none()
            }

            Message::StorageCloseProperties => {
                self.storage_selected_file = None;
                Task::none()
            }

            Message::StorageOperationCompleted(res) => {
                self.is_storage_loading = false;
                match res {
                    Ok(msg) => {
                        self.storage_status_msg = Some(msg);
                        self.storage_error_msg = None;
                    }
                    Err(err) => {
                        self.storage_error_msg = Some(err);
                    }
                }
                Task::done(Message::StorageRefreshDirectory)
            }

            // Wi-Fi Channel & Spectrum Analyzer
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
                Task::none()
            }

            Message::CategorySelected(cat) => {
                self.selected_category = cat;
                if cat.is_none() {
                    self.security_filter = SecurityFilter::All;
                }
                Task::none()
            }

            Message::SortChanged(sort) => {
                self.sort_by = sort;
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
                        self.inspector_ping_stats = Some(stats);
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
                let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
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
        }
    }

    pub fn view(&self, _window: iced::window::Id) -> Element<'_, Message> {
        let mut content = Column::new().spacing(14).padding(16);

        // 1. Main header (Title, Settings button, scan and export buttons)
        content = content.push(self.view_header());

        // Scan progress bar: ONLY shown for manual scan started by button
        if self.is_scanning && !self.is_background_scan {
            content = content.push(
                container(progress_bar(0.0..=1.0, self.scan_progress))
            );
        }

        // Floating notification if active
        if let Some((msg, is_success)) = &self.notification {
            content = content.push(self.view_notification(msg, *is_success));
        }

        // If update modal open, show update instructions modal view (top priority overlay)
        if self.is_update_modal_open {
            content = content.push(
                container(center(self.view_update_modal()))
                    .width(Length::Fill)
                    .height(Length::Fill)
            );

            return container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .into();
        }

        // If settings open, show configuration view
        if self.is_settings_open {
            content = content.push(
                container(center(self.view_settings_panel()))
                    .width(Length::Fill)
                    .height(Length::Fill)
            );

            return container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .into();
        }

        // If about open, show classic About dialog view
        if self.is_about_open {
            content = content.push(
                container(center(self.view_about_modal()))
                    .width(Length::Fill)
                    .height(Length::Fill)
            );

            return container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .into();
        }

        // If speed test open, show speed test modal view
        if self.is_speed_test_open {
            content = content.push(
                container(center(self.view_speed_test_modal()))
                    .width(Length::Fill)
                    .height(Length::Fill)
            );

            return container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .into();
        }

        // If security audit open, show audit modal view
        if self.is_audit_open {
            content = content.push(
                container(center(self.view_security_audit_modal()))
                    .width(Length::Fill)
                    .height(Length::Fill)
            );

            return container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .into();
        }

        // If event history open, show event history modal view
        if self.is_timeline_open {
            content = content.push(
                container(center(self.view_timeline_modal()))
                    .width(Length::Fill)
                    .height(Length::Fill)
            );

            return container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .into();
        }

        // If Wi-Fi analyzer open, show Wi-Fi spectrum modal view
        if self.is_wifi_analyzer_open {
            content = content.push(
                container(center(self.view_wifi_analyzer_modal()))
                    .width(Length::Fill)
                    .height(Length::Fill)
            );

            return container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .into();
        }

        // If router UPnP panel open, show UPnP modal view
        if self.is_upnp_open {
            content = content.push(
                container(center(self.view_upnp_modal()))
                    .width(Length::Fill)
                    .height(Length::Fill)
            );

            return container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .into();
        }

        // If router USB storage explorer open, show storage modal view
        if self.is_storage_explorer_open {
            content = content.push(
                container(center(self.view_storage_explorer_modal()))
                    .width(Length::Fill)
                    .height(Length::Fill)
            );

            return container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .into();
        }

        // 2. Live Telemetry Dashboard (Download, upload speed, Wi-Fi signal, and network)
        content = content.push(self.view_telemetry_dashboard());

        // Intruder Alert Banner (Rogue Device) if unknown device is detected
        if let Some(alert) = &self.pending_intruder_alert {
            content = content.push(self.view_intruder_alert(alert));
        }

        // 3. Filter bar, categories, search, and sorting
        content = content.push(self.view_controls());

        // 4. Main content area: device list + side inspector (if open)
        let filtered_devices = self.get_filtered_devices();
        let device_list = self.view_device_list(&filtered_devices);

        let main_area: Element<'_, Message> = if let Some(selected_ip) = self.selected_device_ip {
            if let Some(selected_device) = self.devices.iter().find(|d| d.ip == selected_ip) {
                row![
                    container(device_list).width(Length::Fill).height(Length::Fill),
                    space::horizontal().width(12),
                    container(self.view_inspector(selected_device))
                        .width(Length::Fixed(450.0))
                        .height(Length::Fill)
                        .style(|_| CustomStyles::inspector_panel())
                        .padding(Padding {
                            top: 14.0,
                            right: 10.0,
                            bottom: 14.0,
                            left: 14.0,
                        }),
                ]
                .height(Length::Fill)
                .into()
            } else {
                device_list
            }
        } else {
            device_list
        };

        content = content.push(main_area);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    /// Prominent warning banner when an unrecognized device is detected on local network
    fn view_intruder_alert(&self, alert: &IntruderAlert) -> Element<'_, Message> {
        let title = text(Translations::intruder_alert_title(self.language))
            .size(12)
            .color(Color::from_rgb8(248, 113, 113));

        let hostname_str = alert.hostname.as_deref().unwrap_or("Sin nombre DNS");
        let info = text(format!(
            "IP: {}   |   MAC: {}   |   {}   |   Fabricante: {}",
            alert.ip,
            alert.mac,
            hostname_str,
            alert.vendor
        ))
        .size(11)
        .color(CustomStyles::text_primary());

        let btn_trust = button(text(Translations::intruder_btn_trust(self.language)).size(11))
            .style(|_, _| CustomStyles::success_button())
            .padding(Padding::from([4, 8]))
            .on_press(Message::TrustDevice(alert.mac.clone()));

        let btn_block = button(text(Translations::intruder_btn_block(self.language)).size(11))
            .style(|_, _| CustomStyles::danger_button())
            .padding(Padding::from([4, 8]))
            .on_press(Message::ToggleDeviceBlocked(alert.ip, alert.mac.clone()));

        let btn_dismiss = button(text(Translations::intruder_btn_dismiss(self.language)).size(11))
            .style(|_, _| CustomStyles::secondary_button())
            .padding(Padding::from([4, 8]))
            .on_press(Message::DismissIntruderAlert);

        let row_content = row![
            column![title, info].spacing(2),
            space::horizontal(),
            btn_trust,
            btn_block,
            btn_dismiss,
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        container(row_content)
            .padding(10)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_danger())
            .into()
    }

    /// Wraps a visual element with a styled tooltip
    fn tip<'a>(
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

    /// Application header and redesigned modular toolbar
    fn view_header(&self) -> Element<'_, Message> {
        // 1. Official app icon in top-left corner
        let app_icon = iced::widget::image(self.logo_handle.clone())
            .width(30)
            .height(30);

        let title_row = row![
            app_icon,
            text("NetLens").size(22).color(CustomStyles::accent_cyan()),
            text(format!("| {}", Translations::app_subtitle(self.language)))
                .size(13)
                .color(CustomStyles::text_secondary()),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let status_row = row![
            text(self.status.render(self.language))
                .size(11)
                .color(CustomStyles::text_muted()),
            button(text("🔄").size(10))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([1, 4]))
                .on_press(Message::RefreshNetwork),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        let title_col = column![title_row, status_row].spacing(2);

        // Primary top-right actions (Export + Scan)
        let scan_btn = if self.is_scanning && !self.is_background_scan {
            button(
                row![
                    text("⏳").size(12),
                    text(Translations::scanning(self.language)).size(12),
                ]
                .spacing(5)
                .align_y(Alignment::Center),
            )
            .style(|_, _| CustomStyles::secondary_button())
            .padding(Padding::from([6, 12]))
        } else {
            button(
                row![
                    text("🔍").size(12),
                    text(Translations::scan_network(self.language)).size(12),
                ]
                .spacing(5)
                .align_y(Alignment::Center),
            )
            .style(|_, _| CustomStyles::primary_button())
            .padding(Padding::from([6, 14]))
            .on_press(Message::StartScan)
        };

        let top_right_actions = row![
            self.tip(
                button(text("📊 CSV").size(11))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([5, 9]))
                    .on_press(Message::ExportCsv),
                Translations::tip_export_csv(self.language),
                tooltip::Position::Bottom,
            ),
            self.tip(
                button(text("📥 JSON").size(11))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([5, 9]))
                    .on_press(Message::ExportJson),
                Translations::tip_export_json(self.language),
                tooltip::Position::Bottom,
            ),
            self.tip(
                scan_btn,
                Translations::tip_scan_network(self.language),
                tooltip::Position::Bottom,
            ),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        let mut top_bar = row![
            title_col,
            space::horizontal(),
        ]
        .align_y(Alignment::Center);

        if let UpdateStatus::UpdateAvailable { ref remote_version, remote_pkgrel, .. } = self.update_status {
            top_bar = top_bar.push(
                button(
                    row![
                        text("🚀").size(12),
                        text(format!("{} (v{}-{})", Translations::update_available(self.language), remote_version, remote_pkgrel))
                            .size(11),
                    ]
                    .spacing(5)
                    .align_y(Alignment::Center)
                )
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([5, 10]))
                .on_press(Message::OpenUpdateModal)
            ).push(space::horizontal());
        }

        top_bar = top_bar.push(top_right_actions);
        let header_top_bar = top_bar.width(Length::Fill);

        // 2. Modular navigation toolbar (Tools Bar)
        // Redesigned dedicated bar so buttons never overflow when resizing window
        let audit_label = if let Some(audit) = &self.security_audit_report {
            format!("{} ({})", Translations::security_audit_btn(self.language), audit.score)
        } else {
            Translations::security_audit_btn(self.language).to_string()
        };

        let timeline_label = format!("{} ({})", Translations::timeline_btn(self.language), self.network_events.len());

        let nav_tools = row![
            // Settings button
            self.tip(
                button(text(Translations::settings(self.language)).size(11))
                    .style(|_, _| {
                        if self.is_settings_open {
                            CustomStyles::primary_button()
                        } else {
                            CustomStyles::secondary_button()
                        }
                    })
                    .padding(Padding::from([5, 10]))
                    .on_press(if self.is_settings_open {
                        Message::CloseSettings
                    } else {
                        Message::OpenSettings
                    }),
                Translations::tip_settings(self.language),
                tooltip::Position::Bottom,
            ),

            // Speed Test button
            self.tip(
                button(text(Translations::speed_test_btn(self.language)).size(11))
                    .style(|_, _| {
                        if self.is_speed_test_open {
                            CustomStyles::primary_button()
                        } else {
                            CustomStyles::secondary_button()
                        }
                    })
                    .padding(Padding::from([5, 10]))
                    .on_press(if self.is_speed_test_open {
                        Message::CloseSpeedTest
                    } else {
                        Message::OpenSpeedTest
                    }),
                Translations::tip_speed_test(self.language),
                tooltip::Position::Bottom,
            ),

            // Security Audit button
            self.tip(
                button(text(audit_label).size(11))
                    .style(|_, _| {
                        if self.is_audit_open {
                            CustomStyles::primary_button()
                        } else {
                            CustomStyles::secondary_button()
                        }
                    })
                    .padding(Padding::from([5, 10]))
                    .on_press(if self.is_audit_open {
                        Message::CloseSecurityAudit
                    } else {
                        Message::OpenSecurityAudit
                    }),
                Translations::tip_security_audit(self.language),
                tooltip::Position::Bottom,
            ),

            // Event History button (Timeline)
            self.tip(
                button(text(timeline_label).size(11))
                    .style(|_, _| {
                        if self.is_timeline_open {
                            CustomStyles::primary_button()
                        } else {
                            CustomStyles::secondary_button()
                        }
                    })
                    .padding(Padding::from([5, 10]))
                    .on_press(if self.is_timeline_open {
                        Message::CloseTimeline
                    } else {
                        Message::OpenTimeline
                    }),
                Translations::tip_timeline(self.language),
                tooltip::Position::Bottom,
            ),

            // Wi-Fi Spectrum Analyzer button
            self.tip(
                button(text(Translations::wifi_analyzer_btn(self.language)).size(11))
                    .style(|_, _| {
                        if self.is_wifi_analyzer_open {
                            CustomStyles::primary_button()
                        } else {
                            CustomStyles::secondary_button()
                        }
                    })
                    .padding(Padding::from([5, 10]))
                    .on_press(if self.is_wifi_analyzer_open {
                        Message::CloseWifiAnalyzer
                    } else {
                        Message::OpenWifiAnalyzer
                    }),
                Translations::tip_wifi_analyzer(self.language),
                tooltip::Position::Bottom,
            ),

            // Router USB Storage Explorer button (NAS)
            self.tip(
                button(text(Translations::storage_btn_header(self.language)).size(11))
                    .style(|_, _| {
                        if self.is_storage_explorer_open {
                            CustomStyles::primary_button()
                        } else {
                            CustomStyles::secondary_button()
                        }
                    })
                    .padding(Padding::from([5, 10]))
                    .on_press(if self.is_storage_explorer_open {
                        Message::CloseStorageExplorer
                    } else {
                        Message::OpenStorageExplorer
                    }),
                Translations::tip_router_storage(self.language),
                tooltip::Position::Bottom,
            ),

            // Classic About NetLens button
            self.tip(
                button(text(Translations::about_btn(self.language)).size(11))
                    .style(|_, _| {
                        if self.is_about_open {
                            CustomStyles::primary_button()
                        } else {
                            CustomStyles::secondary_button()
                        }
                    })
                    .padding(Padding::from([5, 10]))
                    .on_press(if self.is_about_open {
                        Message::CloseAbout
                    } else {
                        Message::OpenAbout
                    }),
                Translations::tip_about(self.language),
                tooltip::Position::Bottom,
            ),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        let tools_toolbar = container(nav_tools)
            .padding(Padding::from([4, 8]))
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

        column![
            header_top_bar,
            tools_toolbar,
        ]
        .spacing(8)
        .into()
    }

    /// Settings / Configuration Panel (UI Theme and Language among top 10 world languages)
    fn view_settings_panel(&self) -> Element<'_, Message> {
        let title_row = row![
            text(Translations::settings(self.language))
                .size(20)
                .color(CustomStyles::accent_cyan()),
            space::horizontal(),
            self.tip(
                button(text("✕").size(14))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([2, 8]))
                    .on_press(Message::CloseSettings),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .align_y(Alignment::Center);

        // Setting 1: UI Theme
        let theme_row = row![
            column![
                text(Translations::theme_label(self.language)).size(14).color(CustomStyles::text_primary()),
                text(Translations::theme_desc(self.language)).size(11).color(CustomStyles::text_muted()),
            ]
            .width(Length::Fill),
            pick_list(
                AVAILABLE_THEMES,
                Some(&self.current_theme),
                |t| Message::ThemeChanged(t.clone()),
            ),
        ]
        .align_y(Alignment::Center)
        .spacing(12);

        // Setting 2: Application Language (Top 10 most spoken languages)
        let lang_row = row![
            column![
                text(Translations::language_label(self.language)).size(14).color(CustomStyles::text_primary()),
                text(Translations::language_desc(self.language)).size(11).color(CustomStyles::text_muted()),
            ]
            .width(Length::Fill),
            pick_list(
                Language::ALL,
                Some(self.language),
                Message::LanguageChanged,
            ),
        ]
        .align_y(Alignment::Center)
        .spacing(12);

        // Setting 3: Periodic Automatic Scan (Auto-Discovery)
        let intervals: &[(u32, &'static str)] = &[
            (0, Translations::auto_scan_disabled(self.language)),
            (30, Translations::auto_scan_30s(self.language)),
            (60, Translations::auto_scan_1m(self.language)),
            (120, Translations::auto_scan_2m(self.language)),
            (300, Translations::auto_scan_5m(self.language)),
        ];

        let mut auto_scan_buttons = row![].spacing(6).align_y(Alignment::Center);
        for &(secs, label) in intervals {
            let is_active = self.auto_scan_interval_secs == secs;
            let btn = button(text(label).size(11))
                .style(move |_, _| {
                    if is_active {
                        CustomStyles::primary_button()
                    } else {
                        CustomStyles::secondary_button()
                    }
                })
                .padding(Padding::from([4, 8]))
                .on_press(Message::AutoScanIntervalChanged(secs));
            auto_scan_buttons = auto_scan_buttons.push(btn);
        }

        let auto_scan_section = column![
            text(Translations::auto_scan_title(self.language))
                .size(13)
                .color(CustomStyles::accent_cyan()),
            text(Translations::auto_scan_desc(self.language))
                .size(11)
                .color(CustomStyles::text_muted()),
            auto_scan_buttons,
        ]
        .spacing(6);

        let auto_scan_card = container(auto_scan_section)
            .padding(12)
            .style(|_| CustomStyles::card_container());

        // Setting 4: Background & System Autostart
        let autostart_btn = button(
            text(if self.autostart_enabled {
                Translations::btn_enabled(self.language)
            } else {
                Translations::btn_disabled(self.language)
            })
            .size(11),
        )
        .style(move |_, _| {
            if self.autostart_enabled {
                CustomStyles::primary_button()
            } else {
                CustomStyles::secondary_button()
            }
        })
        .padding(Padding::from([4, 10]))
        .on_press(Message::ToggleAutostart(!self.autostart_enabled));

        let minimize_btn = button(
            text(if self.minimize_on_close {
                Translations::btn_enabled(self.language)
            } else {
                Translations::btn_disabled(self.language)
            })
            .size(11),
        )
        .style(move |_, _| {
            if self.minimize_on_close {
                CustomStyles::primary_button()
            } else {
                CustomStyles::secondary_button()
            }
        })
        .padding(Padding::from([4, 10]))
        .on_press(Message::ToggleMinimizeOnClose(!self.minimize_on_close));

        let background_section = column![
            row![
                column![
                    text(Translations::autostart_title(self.language)).size(12).color(CustomStyles::text_primary()),
                    text(Translations::autostart_desc(self.language)).size(10).color(CustomStyles::text_muted()),
                ]
                .width(Length::Fill),
                autostart_btn,
            ]
            .align_y(Alignment::Center)
            .spacing(10),
            row![
                column![
                    text(Translations::minimize_on_close_title(self.language)).size(12).color(CustomStyles::text_primary()),
                    text(Translations::minimize_on_close_desc(self.language)).size(10).color(CustomStyles::text_muted()),
                ]
                .width(Length::Fill),
                minimize_btn,
            ]
            .align_y(Alignment::Center)
            .spacing(10),
        ]
        .spacing(10);

        let background_card = container(background_section)
            .padding(12)
            .style(|_| CustomStyles::card_container());

        // Setting 5: Router Control (Linksys MR73 Mesh / JNAP)
        let router_ip_input = text_input("192.168.1.1", &self.router_ip_input)
            .on_input(Message::RouterIpChanged)
            .padding(Padding::from([5, 8]))
            .width(140);

        let router_pass_input = text_input(Translations::router_pass_label(self.language), &self.router_password_input)
            .on_input(Message::RouterPasswordChanged)
            .secure(true)
            .padding(Padding::from([5, 8]))
            .width(Length::Fill);

        let test_router_btn = if self.is_testing_router {
            button(text("...").size(11))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([5, 10]))
        } else {
            button(text(Translations::router_test_btn(self.language)).size(11))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([5, 10]))
                .on_press(Message::TestRouterConnection)
        };

        let mut router_section = column![
            text(Translations::router_settings_title(self.language))
                .size(13)
                .color(CustomStyles::accent_cyan()),
            row![
                text(Translations::router_ip_label(self.language)).size(12).color(CustomStyles::text_muted()).width(140),
                router_ip_input,
            ]
            .align_y(Alignment::Center)
            .spacing(8),
            row![
                text(Translations::router_pass_label(self.language)).size(12).color(CustomStyles::text_muted()).width(140),
                router_pass_input,
                test_router_btn,
            ]
            .align_y(Alignment::Center)
            .spacing(8),
        ]
        .spacing(8);

        if let Some((msg, is_ok)) = &self.router_status_msg {
            let color = if *is_ok { CustomStyles::accent_emerald() } else { Color::from_rgb8(239, 68, 68) };
            router_section = router_section.push(text(msg).size(11).color(color));
        }

        let router_card = container(router_section)
            .padding(12)
            .style(|_| CustomStyles::card_container());

        // Setting 6: System Notifications & Parameterizable Alerts
        let notif_master_btn = button(
            text(if self.notification_settings.enabled {
                Translations::notif_master_enabled(self.language)
            } else {
                Translations::notif_master_disabled(self.language)
            })
            .size(11),
        )
        .style(move |_, _| {
            if self.notification_settings.enabled {
                CustomStyles::primary_button()
            } else {
                CustomStyles::secondary_button()
            }
        })
        .padding(Padding::from([4, 10]))
        .on_press(Message::ToggleNotificationsMaster(!self.notification_settings.enabled));

        let mut notif_section = column![
            row![
                column![
                    text(Translations::notif_settings_title(self.language))
                        .size(13)
                        .color(CustomStyles::accent_cyan()),
                    text(Translations::notif_settings_desc(self.language))
                        .size(11)
                        .color(CustomStyles::text_muted()),
                ]
                .width(Length::Fill),
                notif_master_btn,
            ]
            .align_y(Alignment::Center)
            .spacing(10),
        ]
        .spacing(10);

        if self.notification_settings.enabled {
            // Subsection A: Event Triggers
            let trigger_btn = |active: bool, msg: Message| {
                button(text(if active {
                    Translations::btn_enabled(self.language)
                } else {
                    Translations::btn_disabled(self.language)
                }).size(10))
                .style(move |_, _| {
                    if active {
                        CustomStyles::primary_button()
                    } else {
                        CustomStyles::secondary_button()
                    }
                })
                .padding(Padding::from([3, 8]))
                .on_press(msg)
            };

            let triggers_col = column![
                text(Translations::notif_section_triggers(self.language)).size(11).color(CustomStyles::accent_cyan()),
                row![
                    column![
                        text(Translations::notif_realtime_label(self.language)).size(11).color(CustomStyles::accent_cyan()),
                        text(Translations::notif_realtime_desc(self.language)).size(9).color(CustomStyles::text_muted()),
                    ].width(Length::Fill),
                    trigger_btn(self.notification_settings.realtime_netlink_monitor, Message::ToggleRealtimeMonitor(!self.notification_settings.realtime_netlink_monitor)),
                ].align_y(Alignment::Center),
                row![
                    column![
                        text(Translations::notif_heartbeat_label(self.language)).size(11).color(CustomStyles::accent_cyan()),
                        text(Translations::notif_heartbeat_desc(self.language)).size(9).color(CustomStyles::text_muted()),
                    ].width(Length::Fill),
                    trigger_btn(self.notification_settings.fast_disconnection_heartbeat, Message::ToggleHeartbeat(!self.notification_settings.fast_disconnection_heartbeat)),
                ].align_y(Alignment::Center),
                row![
                    text(Translations::notif_trigger_unknown(self.language)).size(11).color(CustomStyles::text_primary()).width(Length::Fill),
                    trigger_btn(self.notification_settings.notify_unknown_intruders, Message::ToggleNotifyUnknown(!self.notification_settings.notify_unknown_intruders)),
                ].align_y(Alignment::Center),
                row![
                    row![
                        text(Translations::notif_trigger_unwhitelisted_reconnect(self.language)).size(11).color(Color::from_rgb8(245, 158, 11)),
                        container(text(Translations::badge_priority(self.language)).size(9).color(Color::WHITE))
                            .padding(Padding::from([1, 6]))
                            .style(|_| CustomStyles::badge_pill(Color::from_rgb8(180, 83, 9), Color::WHITE)),
                    ].spacing(6).width(Length::Fill).align_y(Alignment::Center),
                    trigger_btn(self.notification_settings.notify_known_unwhitelisted_reconnect, Message::ToggleNotifyKnownUnwhitelisted(!self.notification_settings.notify_known_unwhitelisted_reconnect)),
                ].align_y(Alignment::Center),
                row![
                    text(Translations::notif_trigger_whitelisted_reconnect(self.language)).size(11).color(CustomStyles::text_primary()).width(Length::Fill),
                    trigger_btn(self.notification_settings.notify_whitelisted_reconnect, Message::ToggleNotifyWhitelisted(!self.notification_settings.notify_whitelisted_reconnect)),
                ].align_y(Alignment::Center),
                row![
                    text(Translations::notif_trigger_disconnected(self.language)).size(11).color(CustomStyles::text_primary()).width(Length::Fill),
                    trigger_btn(self.notification_settings.notify_device_disconnected, Message::ToggleNotifyDisconnected(!self.notification_settings.notify_device_disconnected)),
                ].align_y(Alignment::Center),
                row![
                    text(Translations::notif_trigger_outages(self.language)).size(11).color(CustomStyles::text_primary()).width(Length::Fill),
                    trigger_btn(self.notification_settings.notify_internet_outages, Message::ToggleNotifyOutages(!self.notification_settings.notify_internet_outages)),
                ].align_y(Alignment::Center),
                row![
                    text(Translations::notif_trigger_security(self.language)).size(11).color(CustomStyles::text_primary()).width(Length::Fill),
                    trigger_btn(self.notification_settings.notify_security_critical, Message::ToggleNotifySecurity(!self.notification_settings.notify_security_critical)),
                ].align_y(Alignment::Center),
            ]
            .spacing(6);

            // Subsection B: Behavior (Urgency, Duration, and Sound)
            let urgencies = NotificationUrgency::ALL;
            let mut urgency_buttons = row![].spacing(6).align_y(Alignment::Center);
            for &u in urgencies {
                let is_active = self.notification_settings.urgency == u;
                let btn = button(text(u.label(self.language)).size(10))
                    .style(move |_, _| {
                        if is_active {
                            CustomStyles::primary_button()
                        } else {
                            CustomStyles::secondary_button()
                        }
                    })
                    .padding(Padding::from([3, 8]))
                    .on_press(Message::ChangeNotificationUrgency(u));
                urgency_buttons = urgency_buttons.push(btn);
            }

            let timeouts: &[(u32, &'static str)] = &[
                (3, "3s"),
                (5, "5s"),
                (7, "7s"),
                (15, "15s"),
                (30, "30s"),
            ];
            let mut timeout_buttons = row![].spacing(6).align_y(Alignment::Center);
            for &(s, label) in timeouts {
                let is_active = self.notification_settings.timeout_secs == s;
                let btn = button(text(label).size(10))
                    .style(move |_, _| {
                        if is_active {
                            CustomStyles::primary_button()
                        } else {
                            CustomStyles::secondary_button()
                        }
                    })
                    .padding(Padding::from([3, 8]))
                    .on_press(Message::ChangeNotificationTimeout(s));
                timeout_buttons = timeout_buttons.push(btn);
            }

            let sound_btn = trigger_btn(self.notification_settings.play_sound, Message::ToggleNotificationPlaySound(!self.notification_settings.play_sound));

            let behavior_col = column![
                text(Translations::notif_section_behavior(self.language)).size(11).color(CustomStyles::accent_cyan()),
                row![
                    text(Translations::notif_urgency_label(self.language)).size(11).color(CustomStyles::text_muted()).width(140),
                    urgency_buttons,
                ].align_y(Alignment::Center),
                row![
                    text(Translations::notif_timeout_label(self.language)).size(11).color(CustomStyles::text_muted()).width(140),
                    timeout_buttons,
                ].align_y(Alignment::Center),
                row![
                    text(Translations::notif_sound_label(self.language)).size(11).color(CustomStyles::text_muted()).width(Length::Fill),
                    sound_btn,
                ].align_y(Alignment::Center),
            ]
            .spacing(6);

            // Subsection C: Content & Format
            let details = NotificationDetailLevel::ALL;
            let mut detail_buttons = row![].spacing(6).align_y(Alignment::Center);
            for &d in details {
                let is_active = self.notification_settings.detail_level == d;
                let btn = button(text(d.label(self.language)).size(10))
                    .style(move |_, _| {
                        if is_active {
                            CustomStyles::primary_button()
                        } else {
                            CustomStyles::secondary_button()
                        }
                    })
                    .padding(Padding::from([3, 8]))
                    .on_press(Message::ChangeNotificationDetail(d));
                detail_buttons = detail_buttons.push(btn);
            }

            let mac_btn = trigger_btn(self.notification_settings.show_mac, Message::ToggleNotificationShowMac(!self.notification_settings.show_mac));
            let vendor_btn = trigger_btn(self.notification_settings.show_vendor, Message::ToggleNotificationShowVendor(!self.notification_settings.show_vendor));
            let prefix_input = text_input("NetLens", &self.notification_prefix_input)
                .on_input(Message::NotificationPrefixChanged)
                .padding(Padding::from([3, 8]))
                .width(160);

            let test_btn = button(text(Translations::notif_test_btn(self.language)).size(10))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([4, 10]))
                .on_press(Message::SendTestNotification);

            let content_col = column![
                text(Translations::notif_section_content(self.language)).size(11).color(CustomStyles::accent_cyan()),
                row![
                    text(Translations::notif_detail_label(self.language)).size(11).color(CustomStyles::text_muted()).width(140),
                    detail_buttons,
                ].align_y(Alignment::Center),
                row![
                    text(Translations::notif_show_mac_label(self.language)).size(11).color(CustomStyles::text_muted()).width(Length::Fill),
                    mac_btn,
                ].align_y(Alignment::Center),
                row![
                    text(Translations::notif_show_vendor_label(self.language)).size(11).color(CustomStyles::text_muted()).width(Length::Fill),
                    vendor_btn,
                ].align_y(Alignment::Center),
                row![
                    text(Translations::notif_prefix_label(self.language)).size(11).color(CustomStyles::text_muted()).width(140),
                    prefix_input,
                    space::horizontal(),
                    test_btn,
                ].align_y(Alignment::Center),
            ]
            .spacing(6);

            notif_section = notif_section
                .push(container(triggers_col).padding(8).style(|_| CustomStyles::card_container()))
                .push(container(behavior_col).padding(8).style(|_| CustomStyles::card_container()))
                .push(container(content_col).padding(8).style(|_| CustomStyles::card_container()));
        }

        let notifications_card = container(notif_section)
            .padding(12)
            .style(|_| CustomStyles::card_container());

        // Technical system summary
        let info_col = column![
            text(Translations::sys_info_title(self.language)).size(13).color(CustomStyles::accent_cyan()),
            row![
                text(Translations::sys_active_iface(self.language)).size(12).color(CustomStyles::text_muted()).width(140),
                text(&self.network.interface).size(12).color(CustomStyles::text_primary()),
            ],
            row![
                text(Translations::sys_subnet(self.language)).size(12).color(CustomStyles::text_muted()).width(140),
                text(&self.network.subnet_cidr).size(12).color(CustomStyles::text_primary()),
            ],
            row![
                text(Translations::sys_gateway(self.language)).size(12).color(CustomStyles::text_muted()).width(140),
                text(self.network.gateway_ip.map(|g| g.to_string()).unwrap_or_else(|| Translations::not_detected(self.language).to_string())).size(12).color(CustomStyles::text_primary()),
            ],
        ]
        .spacing(6);

        // Setting 7: About NetLens & Author Info
        let about_card = container(
            row![
                column![
                    text(Translations::about_modal_title(self.language)).size(13).color(CustomStyles::accent_cyan()),
                    text(format!("Omar Murcia • MIT License • v{}", env!("CARGO_PKG_VERSION"))).size(11).color(CustomStyles::text_muted()),
                ]
                .width(Length::Fill),
                button(text(Translations::about_btn(self.language)).size(11))
                    .style(|_, _| CustomStyles::primary_button())
                    .padding(Padding::from([5, 12]))
                    .on_press(Message::OpenAbout),
            ]
            .align_y(Alignment::Center)
        )
        .padding(12)
        .style(|_| CustomStyles::card_container());

        let close_btn = button(text(Translations::close(self.language)).size(13))
            .style(|_, _| CustomStyles::primary_button())
            .padding(Padding::from([7, 20]))
            .on_press(Message::CloseSettings);

        let footer_row = row![space::horizontal(), close_btn].align_y(Alignment::Center);

        let body_content = column![
            theme_row,
            lang_row,
            auto_scan_card,
            background_card,
            router_card,
            notifications_card,
            container(info_col).padding(12).style(|_| CustomStyles::card_container()),
            about_card,
        ]
        .spacing(14)
        .padding(Padding {
            top: 0.0,
            right: 16.0,
            bottom: 0.0,
            left: 0.0,
        });

        let scrollable_body = scrollable(body_content)
            .height(Length::Fill);

        let panel_layout = column![
            title_row,
            scrollable_body,
            footer_row,
        ]
        .spacing(14);

        container(panel_layout)
            .padding(24)
            .width(820)
            .max_height(720)
            .style(|_| CustomStyles::settings_panel())
            .into()
    }

    /// Modal: Classic "About NetLens" with author information, repositories, logo and dynamic language switching
    fn view_about_modal(&self) -> Element<'_, Message> {
        let title_row = row![
            text(Translations::about_modal_title(self.language))
                .size(20)
                .color(CustomStyles::accent_cyan()),
            space::horizontal(),
            self.tip(
                button(text("✕").size(14))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([2, 8]))
                    .on_press(Message::CloseAbout),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .align_y(Alignment::Center);

        // 1. App Header Card with official Logo
        let app_icon = iced::widget::image(self.logo_handle.clone())
            .width(68)
            .height(68);

        let update_status_element: Element<'_, Message> = match &self.update_status {
            UpdateStatus::Checking => container(
                text(Translations::checking_updates(self.language))
                    .size(10)
                    .color(CustomStyles::text_muted()),
            )
            .padding(Padding::from([3, 8]))
            .style(|_| CustomStyles::badge_pill(Color::from_rgb8(25, 30, 45), CustomStyles::text_muted()))
            .into(),
            UpdateStatus::UpdateAvailable {
                remote_version,
                remote_pkgrel,
                ..
            } => row![
                container(
                    row![
                        text("🚀").size(10),
                        text(format!(
                            "{}: v{}-{}",
                            Translations::update_available(self.language),
                            remote_version,
                            remote_pkgrel
                        ))
                        .size(10)
                        .color(CustomStyles::accent_cyan()),
                    ]
                    .spacing(4)
                    .align_y(Alignment::Center),
                )
                .padding(Padding::from([3, 8]))
                .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 35, 55), CustomStyles::accent_cyan())),
                button(text(Translations::update_btn(self.language)).size(10))
                    .style(|_, _| CustomStyles::primary_button())
                    .padding(Padding::from([3, 8]))
                    .on_press(Message::OpenUpdateModal),
            ]
            .spacing(6)
            .align_y(Alignment::Center)
            .into(),
            UpdateStatus::UpToDate | UpdateStatus::Idle | UpdateStatus::CheckFailed => container(
                row![
                    text("✓").size(10).color(CustomStyles::accent_emerald()),
                    text(Translations::update_latest_version(self.language))
                        .size(10)
                        .color(CustomStyles::accent_emerald()),
                ]
                .spacing(4)
                .align_y(Alignment::Center),
            )
            .padding(Padding::from([3, 8]))
            .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 45, 30), CustomStyles::accent_emerald()))
            .into(),
        };

        let check_updates_btn = self.tip(
            button(text("🔄").size(10))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([2, 5]))
                .on_press(Message::CheckForUpdates),
            Translations::check_for_updates(self.language),
            tooltip::Position::Top,
        );

        let app_info = column![
            row![
                text("NetLens").size(24).color(CustomStyles::accent_cyan()),
                container(text(format!("v{}-{}", env!("CARGO_PKG_VERSION"), UpdateChecker::CURRENT_PKGREL)).size(11).color(CustomStyles::accent_emerald()))
                    .padding(Padding::from([3, 9]))
                    .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 45, 30), CustomStyles::accent_emerald())),
                container(text("CachyOS / Arch Linux").size(10).color(CustomStyles::accent_purple()))
                    .padding(Padding::from([3, 9]))
                    .style(|_| CustomStyles::badge_pill(Color::from_rgb8(35, 20, 50), CustomStyles::accent_purple())),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            row![
                update_status_element,
                check_updates_btn,
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(Translations::app_subtitle(self.language))
                .size(12)
                .color(CustomStyles::text_secondary()),
            text(Translations::about_app_description(self.language))
                .size(11)
                .color(CustomStyles::text_muted()),
        ]
        .spacing(4)
        .width(Length::Fill);

        let branding_card = container(
            row![
                app_icon,
                app_info,
            ]
            .spacing(16)
            .align_y(Alignment::Center)
        )
        .padding(14)
        .width(Length::Fill)
        .style(|_| CustomStyles::card_container());

        // 2. Author & Developer Card
        let author_col = column![
            text(Translations::about_author_section(self.language))
                .size(13)
                .color(CustomStyles::accent_cyan()),
            row![
                column![
                    text("Omar Murcia").size(16).color(CustomStyles::text_primary()),
                    text(Translations::about_role(self.language)).size(11).color(CustomStyles::text_muted()),
                ]
                .width(Length::Fill),
                row![
                    text(format!("✉️ {}: ing.omarmurcia@gmail.com", Translations::about_email_label(self.language)))
                        .size(12)
                        .color(CustomStyles::text_secondary()),
                    button(text(format!("📋 {}", Translations::about_copy_email(self.language))).size(10))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([4, 8]))
                        .on_press(Message::CopyToClipboard("ing.omarmurcia@gmail.com".to_string(), "Email".to_string())),
                    button(text(format!("✉️ {}", Translations::about_send_email(self.language))).size(10))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([4, 8]))
                        .on_press(Message::OpenExternalUrl("mailto:ing.omarmurcia@gmail.com".to_string())),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(6);

        let author_card = container(author_col)
            .padding(14)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

        // 3. Repositories & Distribution Card
        let make_repo_row = |icon: &'static str, title: &'static str, url: &'static str, open_label: &'static str, copy_label: &'static str| {
            row![
                column![
                    text(format!("{} {}", icon, title)).size(12).color(CustomStyles::text_primary()),
                    text(url).size(10).color(CustomStyles::accent_cyan()),
                ]
                .width(Length::Fill),
                row![
                    button(text(format!("🌐 {}", open_label)).size(10))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([3, 8]))
                        .on_press(Message::OpenExternalUrl(url.to_string())),
                    button(text(format!("📋 {}", copy_label)).size(10))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([3, 8]))
                        .on_press(Message::CopyToClipboard(url.to_string(), title.to_string())),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            ]
            .align_y(Alignment::Center)
            .spacing(8)
        };

        let repos_col = column![
            text(Translations::about_repositories_section(self.language))
                .size(13)
                .color(CustomStyles::accent_cyan()),
            make_repo_row("💻", Translations::about_source_code(self.language), "https://github.com/omarmurcia/netlens", Translations::about_open_in_browser(self.language), Translations::about_copy_url(self.language)),
            make_repo_row("📦", Translations::about_pacman_repo(self.language), "https://github.com/omarmurcia/netlens-repo", Translations::about_open_in_browser(self.language), Translations::about_copy_url(self.language)),
            make_repo_row("🚀", Translations::about_web_portal(self.language), "https://omarmurcia.github.io/netlens-repo", Translations::about_open_in_browser(self.language), Translations::about_copy_url(self.language)),
        ]
        .spacing(8);

        let repos_card = container(repos_col)
            .padding(14)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

        // 4. Technical Specs & License Card
        let specs_col = column![
            text(Translations::about_tech_specs(self.language))
                .size(13)
                .color(CustomStyles::accent_cyan()),
            row![
                column![
                    text(format!("⚖️ {}", Translations::about_license(self.language))).size(11).color(CustomStyles::text_muted()),
                    text("MIT License (Free & Open Source)").size(11).color(CustomStyles::text_primary()),
                ]
                .width(Length::Fill),
                column![
                    text("⚡ Target OS").size(11).color(CustomStyles::text_muted()),
                    text("Linux x86_64 (CachyOS / Arch / System Tray)").size(11).color(CustomStyles::text_primary()),
                ]
                .width(Length::Fill),
                column![
                    text("🦀 Core Engine").size(11).color(CustomStyles::text_muted()),
                    text("Rust + Iced 0.13 + Tokio Runtime").size(11).color(CustomStyles::text_primary()),
                ]
                .width(Length::Fill),
            ]
            .spacing(12),
            text("© 2026 Omar Murcia. All rights reserved.")
                .size(10)
                .color(CustomStyles::text_muted()),
        ]
        .spacing(6);

        let specs_card = container(specs_col)
            .padding(14)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

        // 5. Footer with Language Switcher and Close Button
        let footer_row = row![
            text(format!("🌐 {}", Translations::language_label(self.language)))
                .size(12)
                .color(CustomStyles::text_muted()),
            pick_list(
                Language::ALL,
                Some(self.language),
                Message::LanguageChanged,
            ),
            space::horizontal(),
            button(text(Translations::close(self.language)).size(12))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([6, 20]))
                .on_press(Message::CloseAbout),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let content = column![
            title_row,
            branding_card,
            author_card,
            repos_card,
            specs_card,
            footer_row,
        ]
        .spacing(12);

        container(content)
            .padding(22)
            .width(760)
            .style(|_| CustomStyles::modal_panel(CustomStyles::accent_cyan()))
            .into()
    }

    /// Modal: Software Update Available Dialog with 1-line installer command and changelog
    fn view_update_modal(&self) -> Element<'_, Message> {
        let (ver, rel, changelog, url) = match &self.update_status {
            UpdateStatus::UpdateAvailable {
                remote_version,
                remote_pkgrel,
                changelog,
                url,
            } => (
                remote_version.as_str(),
                *remote_pkgrel,
                changelog.as_deref().unwrap_or(""),
                url.as_str(),
            ),
            _ => (UpdateChecker::CURRENT_VERSION, UpdateChecker::CURRENT_PKGREL, "", "https://omarmurcia.github.io/netlens-repo"),
        };

        let title_row = row![
            text(Translations::update_modal_title(self.language))
                .size(20)
                .color(CustomStyles::accent_cyan()),
            space::horizontal(),
            self.tip(
                button(text("✕").size(14))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([2, 8]))
                    .on_press(Message::CloseUpdateModal),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .align_y(Alignment::Center);

        let version_banner = container(
            row![
                column![
                    text(format!("NetLens v{}-{}", ver, rel))
                        .size(16)
                        .color(CustomStyles::accent_cyan()),
                    text(if changelog.is_empty() {
                        Translations::app_subtitle(self.language).to_string()
                    } else {
                        changelog.to_string()
                    })
                    .size(11)
                    .color(CustomStyles::text_secondary()),
                ]
                .width(Length::Fill),
                button(text(format!("🌐 {}", Translations::about_web_portal(self.language))).size(11))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([4, 10]))
                    .on_press(Message::OpenExternalUrl(url.to_string())),
            ]
            .align_y(Alignment::Center)
        )
        .padding(14)
        .style(|_| CustomStyles::card_highlighted());

        let in_app_action_box: Element<'_, Message> = match &self.update_progress {
            InAppUpdateProgress::Idle => container(
                row![
                    column![
                        text(Translations::update_now_btn(self.language))
                            .size(13)
                            .color(CustomStyles::accent_cyan()),
                        text(match self.language {
                            Language::Spanish => "Actualiza automáticamente con 1 clic mediante la autenticación del sistema.",
                            Language::English => "Update automatically in 1 click using system authentication.",
                            Language::Chinese => "使用系统身份验证一键自动更新。",
                            Language::Hindi => "सिस्टम प्रमाणीकरण का उपयोग करके 1-क्लिक में स्वचालित रूप से अपडेट करें।",
                            Language::Arabic => "تحديث تلقائي بنقرة واحدة باستخدام مصادقة النظام.",
                            Language::French => "Mise à jour automatique en 1 clic via l'authentification système.",
                            Language::Bengali => "সিস্টেম প্রমাণীকরণ ব্যবহার করে ১-ক্লিকে স্বয়ংক্রিয়ভাবে আপডেট করুন।",
                            Language::Portuguese => "Atualize automaticamente em 1 clique usando a autenticação do sistema.",
                            Language::Russian => "Автоматическое обновление в 1 клик с системной аутентификацией.",
                            Language::Urdu => "سسٹم کی تصدیق کا استعمال کرتے ہوئے 1 کلک میں خودکار طور پر اپ ڈیٹ کریں۔",
                        })
                        .size(11)
                        .color(CustomStyles::text_muted()),
                    ]
                    .width(Length::Fill),
                    row![
                        button(text(Translations::update_in_terminal_btn(self.language)).size(11))
                            .style(|_, _| CustomStyles::secondary_button())
                            .padding(Padding::from([6, 12]))
                            .on_press(Message::OpenTerminalUpdater),
                        button(text(Translations::update_now_btn(self.language)).size(11))
                            .style(|_, _| CustomStyles::primary_button())
                            .padding(Padding::from([6, 16]))
                            .on_press(Message::StartInAppUpdate),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                ]
                .spacing(12)
                .align_y(Alignment::Center)
            )
            .padding(14)
            .style(|_| CustomStyles::card_highlighted())
            .into(),

            InAppUpdateProgress::AuthorizingAndInstalling => container(
                row![
                    text("⏳").size(16),
                    text(Translations::update_authorizing_and_installing(self.language))
                        .size(12)
                        .color(CustomStyles::accent_cyan()),
                ]
                .spacing(10)
                .align_y(Alignment::Center)
            )
            .padding(14)
            .style(|_| CustomStyles::card_container())
            .into(),

            InAppUpdateProgress::Success => container(
                row![
                    column![
                        text(Translations::update_success_banner(self.language))
                            .size(13)
                            .color(CustomStyles::accent_emerald()),
                        text(match self.language {
                            Language::Spanish => "La nueva versión está instalada y lista para iniciarse.",
                            Language::English => "The new version is installed and ready to launch.",
                            Language::Chinese => "新版本已安装并可启动。",
                            Language::Hindi => "नया संस्करण स्थापित हो गया है और लॉन्च के लिए तैयार है।",
                            Language::Arabic => "الإصدار الجديد مثبت وجاهز للتشغيل.",
                            Language::French => "La nouvelle version est installée et prête à être lancée.",
                            Language::Bengali => "নতুন সংস্করণটি ইনস্টল করা হয়েছে এবং চালু করার জন্য প্রস্তুত।",
                            Language::Portuguese => "A nova versão está instalada e pronta para iniciar.",
                            Language::Russian => "Новая версия установлена и готова к запуску.",
                            Language::Urdu => "نیا ورژن انسٹال ہو چکا ہے اور چلانے کے لیے تیار ہے۔",
                        })
                        .size(11)
                        .color(CustomStyles::text_secondary()),
                    ]
                    .width(Length::Fill),
                    button(text(Translations::update_restart_btn(self.language)).size(11))
                        .style(|_, _| CustomStyles::primary_button())
                        .padding(Padding::from([6, 16]))
                        .on_press(Message::RestartNetLens),
                ]
                .spacing(12)
                .align_y(Alignment::Center)
            )
            .padding(14)
            .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 45, 30), CustomStyles::accent_emerald()))
            .into(),

            InAppUpdateProgress::Failed(err) => container(
                row![
                    column![
                        text(format!("{} {}", Translations::update_failed_banner(self.language), err))
                            .size(12)
                            .color(CustomStyles::accent_red()),
                    ]
                    .width(Length::Fill),
                    row![
                        button(text(Translations::update_in_terminal_btn(self.language)).size(11))
                            .style(|_, _| CustomStyles::secondary_button())
                            .padding(Padding::from([6, 12]))
                            .on_press(Message::OpenTerminalUpdater),
                        button(text(format!("🔄 {}", Translations::update_retry_btn(self.language))).size(11))
                            .style(|_, _| CustomStyles::primary_button())
                            .padding(Padding::from([6, 14]))
                            .on_press(Message::StartInAppUpdate),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                ]
                .spacing(12)
                .align_y(Alignment::Center)
            )
            .padding(14)
            .style(|_| CustomStyles::card_container())
            .into(),
        };

        let cmd_str = "curl -sSL https://omarmurcia.github.io/netlens-repo/install-repo.sh | bash";
        let cmd_box = container(
            row![
                text(cmd_str).size(12).color(CustomStyles::accent_emerald()),
                space::horizontal(),
                button(text(format!("📋 {}", Translations::update_copy_command(self.language))).size(11))
                    .style(|_, _| CustomStyles::primary_button())
                    .padding(Padding::from([5, 12]))
                    .on_press(Message::CopyToClipboard(
                        cmd_str.to_string(),
                        Translations::update_copy_command(self.language).to_string(),
                    )),
            ]
            .align_y(Alignment::Center)
        )
        .padding(12)
        .style(|_| CustomStyles::card_container());

        let instructions_col = column![
            text(Translations::update_instructions(self.language))
                .size(12)
                .color(CustomStyles::text_primary()),
            cmd_box,
        ]
        .spacing(8);

        let footer_row = row![
            space::horizontal(),
            button(text(Translations::close(self.language)).size(12))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([6, 18]))
                .on_press(Message::CloseUpdateModal),
        ]
        .align_y(Alignment::Center);

        let layout = column![
            title_row,
            version_banner,
            in_app_action_box,
            instructions_col,
            footer_row,
        ]
        .spacing(14);

        container(layout)
            .padding(24)
            .width(720)
            .style(|_| CustomStyles::modal_panel(CustomStyles::accent_cyan()))
            .into()
    }

    /// Internet Speed Test Modal (Real-time Ping, Download, and Upload)
    fn view_speed_test_modal(&self) -> Element<'_, Message> {
        let is_running = self.speed_test_stage == SpeedTestStage::TestingPing
            || self.speed_test_stage == SpeedTestStage::TestingDownload
            || self.speed_test_stage == SpeedTestStage::TestingUpload;

        let title_row = row![
            text(Translations::speed_test_title(self.language))
                .size(20)
                .color(CustomStyles::accent_cyan()),
            space::horizontal(),
            self.tip(
                button(text("✕").size(14))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([2, 8]))
                    .on_press(Message::CloseSpeedTest),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .align_y(Alignment::Center);

        let subtitle = text(Translations::speed_test_subtitle(self.language))
            .size(11)
            .color(CustomStyles::text_muted());

        let (progress, stage_text, stage_color) = match self.speed_test_stage {
            SpeedTestStage::Idle => (0.0, Translations::speed_stage_idle(self.language), CustomStyles::text_muted()),
            SpeedTestStage::TestingPing => (0.25, Translations::speed_stage_ping(self.language), CustomStyles::accent_amber()),
            SpeedTestStage::TestingDownload => (0.65, Translations::speed_stage_download(self.language), CustomStyles::accent_emerald()),
            SpeedTestStage::TestingUpload => (0.90, Translations::speed_stage_upload(self.language), CustomStyles::accent_purple()),
            SpeedTestStage::Completed => (1.0, Translations::speed_stage_complete(self.language), CustomStyles::accent_emerald()),
            SpeedTestStage::Failed => (0.0, Translations::speed_stage_failed(self.language), Color::from_rgb8(239, 68, 68)),
        };

        let status_row = row![
            text(stage_text).size(13).color(stage_color),
        ]
        .align_y(Alignment::Center);

        // Metric Cards (Ping, Download, Upload)
        let ping_str = self
            .speed_test_ping
            .map(|p| format!("{:.1} ms", p))
            .unwrap_or_else(|| "-- ms".to_string());

        let card_ping = container(
            column![
                text(Translations::speed_test_ping(self.language))
                    .size(11)
                    .color(CustomStyles::accent_amber()),
                text(ping_str)
                    .size(22)
                    .color(CustomStyles::accent_amber()),
                text("Round-Trip Time")
                    .size(10)
                    .color(CustomStyles::text_muted()),
            ]
            .spacing(4),
        )
        .padding(14)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_signal());

        let dl_str = self
            .speed_test_download
            .map(|d| format!("{:.2} Mbps", d))
            .unwrap_or_else(|| "-- Mbps".to_string());
        let dl_sub = self
            .speed_test_download
            .map(|d| format!("{:.2} MB/s", d / 8.0))
            .unwrap_or_else(|| "-- MB/s".to_string());

        let card_dl = container(
            column![
                text(Translations::speed_test_download(self.language))
                    .size(11)
                    .color(CustomStyles::accent_emerald()),
                text(dl_str)
                    .size(22)
                    .color(CustomStyles::accent_emerald()),
                text(dl_sub)
                    .size(11)
                    .color(CustomStyles::text_secondary()),
            ]
            .spacing(4),
        )
        .padding(14)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_download());

        let ul_str = self
            .speed_test_upload
            .map(|u| format!("{:.2} Mbps", u))
            .unwrap_or_else(|| "-- Mbps".to_string());
        let ul_sub = self
            .speed_test_upload
            .map(|u| format!("{:.2} MB/s", u / 8.0))
            .unwrap_or_else(|| "-- MB/s".to_string());

        let card_ul = container(
            column![
                text(Translations::speed_test_upload(self.language))
                    .size(11)
                    .color(CustomStyles::accent_purple()),
                text(ul_str)
                    .size(22)
                    .color(CustomStyles::accent_purple()),
                text(ul_sub)
                    .size(11)
                    .color(CustomStyles::text_secondary()),
            ]
            .spacing(4),
        )
        .padding(14)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_upload());

        let metrics_row = row![card_ping, card_dl, card_ul].spacing(10);

        // Network / server information
        let server_info = column![
            row![
                text(Translations::speed_test_server(self.language))
                    .size(12)
                    .color(CustomStyles::text_muted())
                    .width(130),
                text(
                    self.speed_test_result
                        .as_ref()
                        .map(|r| r.server.as_str())
                        .unwrap_or("Netflix Open Connect / Cloudflare CDN"),
                )
                    .size(12)
                    .color(CustomStyles::text_primary()),
            ],
            row![
                text(Translations::sys_active_iface(self.language))
                    .size(12)
                    .color(CustomStyles::text_muted())
                    .width(130),
                text(&self.network.interface)
                    .size(12)
                    .color(CustomStyles::accent_cyan()),
            ],
        ]
        .spacing(6);

        // Action buttons
        let start_btn = if is_running {
            button(text(Translations::speed_test_running(self.language)).size(13))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([8, 18]))
        } else {
            button(text(Translations::speed_test_start(self.language)).size(13))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([8, 18]))
                .on_press(Message::StartSpeedTest)
        };

        let close_btn = button(text(Translations::close(self.language)).size(13))
            .style(|_, _| CustomStyles::secondary_button())
            .padding(Padding::from([8, 16]))
            .on_press(Message::CloseSpeedTest);

        // Section: Persistent Speed Test History
        let mut history_header = row![
            text(Translations::speed_test_history_title(self.language))
                .size(13)
                .color(CustomStyles::accent_cyan()),
            space::horizontal(),
        ]
        .align_y(Alignment::Center);

        if !self.speed_test_history.is_empty() {
            history_header = history_header.push(
                button(text(Translations::speed_test_history_clear(self.language)).size(10))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([2, 6]))
                    .on_press(Message::ClearSpeedTestHistory),
            );
        }

        let mut history_col = Column::new().spacing(4);
        history_col = history_col.push(history_header);

        if self.speed_test_history.is_empty() {
            history_col = history_col.push(
                text(Translations::speed_test_history_empty(self.language))
                    .size(11)
                    .color(CustomStyles::text_muted()),
            );
        } else {
            history_col = history_col.push(
                row![
                    text(Translations::speed_test_col_time_server(self.language)).size(10).color(CustomStyles::text_muted()).width(160),
                    text(Translations::speed_test_ping(self.language)).size(10).color(CustomStyles::accent_amber()).width(70),
                    text(Translations::speed_test_col_download(self.language)).size(10).color(CustomStyles::accent_emerald()).width(90),
                    text(Translations::speed_test_col_upload(self.language)).size(10).color(CustomStyles::accent_purple()).width(90),
                ]
                .spacing(8),
            );

            for test in self.speed_test_history.iter().take(4) {
                let srv_truncated = if test.server.len() > 20 {
                    format!("{}...", &test.server[..18])
                } else {
                    test.server.clone()
                };
                history_col = history_col.push(
                    row![
                        column![
                            text(&test.timestamp).size(11).color(CustomStyles::text_primary()),
                            text(srv_truncated).size(9).color(CustomStyles::text_muted()),
                        ]
                        .width(160),
                        text(format!("{:.1} ms", test.ping_ms))
                            .size(11)
                            .color(CustomStyles::accent_amber())
                            .width(70),
                        text(format!("{:.1} Mbps", test.download_mbps))
                            .size(11)
                            .color(CustomStyles::accent_emerald())
                            .width(90),
                        text(format!("{:.1} Mbps", test.upload_mbps))
                            .size(11)
                            .color(CustomStyles::accent_purple())
                            .width(90),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                );
            }
        }

        let history_card = container(history_col)
            .padding(12)
            .style(|_| CustomStyles::card_container());

        let content = column![
            title_row,
            subtitle,
            status_row,
            progress_bar(0.0..=1.0, progress),
            metrics_row,
            container(server_info)
                .padding(12)
                .style(|_| CustomStyles::card_container()),
            history_card,
            row![space::horizontal(), close_btn, start_btn].spacing(10),
        ]
        .spacing(14);

        container(content)
            .padding(24)
            .width(850)
            .style(|_| CustomStyles::settings_panel())
            .into()
    }

    /// Top live telemetry dashboard: Download, upload speed, Wi-Fi signal, and router
    fn view_telemetry_dashboard(&self) -> Element<'_, Message> {
        // Card 1: Download Speed (with Sparkline)
        let rx_speed_str = Telemetry::format_speed(self.telemetry.download_speed_bps);
        let rx_session_str = Telemetry::format_bytes(self.telemetry.session_download_bytes());

        let sparkline_dl = Canvas::new(Sparkline::new(
            self.download_history.iter().copied().collect(),
            CustomStyles::accent_emerald(),
        ))
        .width(Length::Fill)
        .height(20);

        let card_download = container(
            column![
                row![
                    text(Translations::download(self.language)).size(11).color(CustomStyles::accent_emerald()),
                    space::horizontal(),
                    text(Translations::live(self.language)).size(9).color(CustomStyles::text_muted()),
                ]
                .align_y(Alignment::Center),
                text(rx_speed_str).size(20).color(CustomStyles::accent_emerald()),
                sparkline_dl,
                text(format!("{}: {}", Translations::session(self.language), rx_session_str)).size(11).color(CustomStyles::text_secondary()),
            ]
            .spacing(3),
        )
        .padding(12)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_download());

        // Card 2: Upload Speed (with Sparkline)
        let tx_speed_str = Telemetry::format_speed(self.telemetry.upload_speed_bps);
        let tx_session_str = Telemetry::format_bytes(self.telemetry.session_upload_bytes());

        let sparkline_ul = Canvas::new(Sparkline::new(
            self.upload_history.iter().copied().collect(),
            CustomStyles::accent_purple(),
        ))
        .width(Length::Fill)
        .height(20);

        let card_upload = container(
            column![
                row![
                    text(Translations::upload(self.language)).size(11).color(CustomStyles::accent_purple()),
                    space::horizontal(),
                    text(Translations::live(self.language)).size(9).color(CustomStyles::text_muted()),
                ]
                .align_y(Alignment::Center),
                text(tx_speed_str).size(20).color(CustomStyles::accent_purple()),
                sparkline_ul,
                text(format!("{}: {}", Translations::session(self.language), tx_session_str)).size(11).color(CustomStyles::text_secondary()),
            ]
            .spacing(3),
        )
        .padding(12)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_upload());

        // Card 3: Wi-Fi Signal Strength & Quality / Network Connection
        let (signal_title, signal_big_str, signal_sub_str) = if let Some(wifi) = &self.telemetry.wifi {
            let dbm_part = wifi.signal_dbm.map(|s| format!(" ({} dBm)", s)).unwrap_or_default();
            let big = format!("{}%{}", wifi.signal_percent, dbm_part);
            let sub = format!(
                "{} • {} • {}",
                wifi.ssid.as_deref().unwrap_or("Wi-Fi"),
                wifi.band_label(),
                Translations::wifi_quality_label(wifi.signal_percent, self.language)
            );
            (Translations::signal_wifi(self.language), big, sub)
        } else if let Some(eth) = &self.telemetry.ethernet {
            let big = eth.display_summary();
            let sub = format!("{} ({})", eth.interface, Translations::ethernet_cable(self.language));
            (Translations::ethernet_cable(self.language), big, sub)
        } else {
            (
                "🌐 LINK",
                format!("{} {}", self.network.interface, Translations::spec_interface(self.language)),
                Translations::link_quality_desc(None, self.language).to_string(),
            )
        };

        let card_signal = container(
            column![
                row![
                    text(signal_title).size(11).color(CustomStyles::accent_amber()),
                    space::horizontal(),
                    text("STATUS").size(9).color(CustomStyles::text_muted()),
                ]
                .align_y(Alignment::Center),
                text(signal_big_str).size(18).color(CustomStyles::accent_amber()),
                text(signal_sub_str).size(11).color(CustomStyles::text_secondary()),
            ]
            .spacing(3),
        )
        .padding(12)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_signal());

        // Card 4: Router / Gateway & Local Network
        let total_devices = self.devices.len();
        let gw_ip_str = self
            .network
            .gateway_ip
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| Translations::not_detected(self.language).to_string());

        let gw_device = self.devices.iter().find(|d| d.is_gateway);
        let gw_name = gw_device
            .map(|d| d.display_name())
            .unwrap_or_else(|| Translations::gateway_label(self.language).to_string());

        let wan_status_text = if let Some(upnp) = &self.upnp_status {
            if let Some(ext_ip) = &upnp.external_ip {
                format!("WAN: {}", ext_ip)
            } else {
                "UPnP Activo".to_string()
            }
        } else {
            format!("Estabilidad: {:.0}%", self.uptime_tracker.uptime_percentage())
        };

        let card_gateway = container(
            column![
                row![
                    text(Translations::network_and_router(self.language)).size(11).color(CustomStyles::accent_cyan()),
                    space::horizontal(),
                    button(text(Translations::upnp_btn(self.language)).size(9))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([1, 6]))
                        .on_press(Message::OpenUpnpModal),
                ]
                .align_y(Alignment::Center),
                row![
                    text(Translations::devices_count(total_devices, self.language)).size(18).color(CustomStyles::text_primary()),
                    text(format!("(GW: {})", gw_ip_str)).size(12).color(CustomStyles::accent_cyan()),
                ]
                .align_y(Alignment::Center)
                .spacing(6),
                row![
                    text(gw_name).size(11).color(CustomStyles::text_secondary()),
                    space::horizontal(),
                    text(wan_status_text).size(10).color(CustomStyles::accent_emerald()),
                ]
                .align_y(Alignment::Center),
            ]
            .spacing(3),
        )
        .padding(12)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_highlighted());

        row![card_download, card_upload, card_signal, card_gateway]
            .spacing(10)
            .into()
    }

    /// Filter, search, and sorting controls
    fn view_controls(&self) -> Element<'_, Message> {
        let search_bar = text_input(
            Translations::search_placeholder(self.language),
            &self.search_query,
        )
        .on_input(Message::SearchChanged)
        .size(13)
        .padding(Padding::from([8, 12]))
        .width(Length::Fixed(300.0));

        // Category compact styled pill buttons in an elegant scrollable horizontal row
        let mut category_pills = row![].spacing(6).align_y(Alignment::Center).padding(Padding {
            top: 3.0,
            right: 6.0,
            bottom: 9.0,
            left: 4.0,
        });

        let is_all_active = self.selected_category.is_none() && self.security_filter == SecurityFilter::All;
        let all_btn = button(text(Translations::all_filter(self.language)).size(12))
            .style(move |_, _| {
                if is_all_active {
                    CustomStyles::filter_pill_active()
                } else {
                    CustomStyles::filter_pill_inactive()
                }
            })
            .padding(Padding::from([4, 10]))
            .on_press(Message::CategorySelected(None));
        category_pills = category_pills.push(
            self.tip(all_btn, Translations::tip_filter_all(self.language), tooltip::Position::Bottom)
        );

        for &cat in DeviceCategory::all() {
            if cat == DeviceCategory::Unknown {
                continue;
            }
            let is_active = self.selected_category == Some(cat);
            let label = Translations::category_short_name(cat, self.language);
            let cat_btn = button(text(label).size(12))
                .style(move |_, _| {
                    if is_active {
                        CustomStyles::filter_pill_active()
                    } else {
                        CustomStyles::filter_pill_inactive()
                    }
                })
                .padding(Padding::from([4, 8]))
                .on_press(Message::CategorySelected(Some(cat)));
            category_pills = category_pills.push(
                self.tip(cat_btn, Translations::tip_filter_category(cat, self.language), tooltip::Position::Bottom)
            );
        }

        // Security & List Filters (Whitelist and Blocked)
        let whitelist_count = self.devices.iter().filter(|d| d.is_whitelisted).count();
        let blocked_count = self.devices.iter().filter(|d| d.is_blocked).count();

        let is_whitelist_active = self.security_filter == SecurityFilter::Whitelist;
        let is_blocked_active = self.security_filter == SecurityFilter::Blocked;

        if whitelist_count > 0 || is_whitelist_active {
            let wl_btn = button(text(format!("🛡️ {} ({})", Translations::filter_whitelist(self.language), whitelist_count)).size(11))
                .style(move |_, _| {
                    if is_whitelist_active {
                        CustomStyles::filter_pill_active()
                    } else {
                        CustomStyles::filter_pill_inactive()
                    }
                })
                .padding(Padding::from([4, 8]))
                .on_press(Message::SecurityFilterChanged(
                    if is_whitelist_active { SecurityFilter::All } else { SecurityFilter::Whitelist }
                ));
            category_pills = category_pills.push(
                self.tip(wl_btn, Translations::tip_filter_whitelist(self.language), tooltip::Position::Bottom)
            );
        }

        if blocked_count > 0 || is_blocked_active {
            let blk_btn = button(text(format!("🚫 {} ({})", Translations::filter_blocked(self.language), blocked_count)).size(11))
                .style(move |_, _| {
                    if is_blocked_active {
                        CustomStyles::filter_pill_active()
                    } else {
                        CustomStyles::filter_pill_inactive()
                    }
                })
                .padding(Padding::from([4, 8]))
                .on_press(Message::SecurityFilterChanged(
                    if is_blocked_active { SecurityFilter::All } else { SecurityFilter::Blocked }
                ));
            category_pills = category_pills.push(
                self.tip(blk_btn, Translations::tip_filter_blocked(self.language), tooltip::Position::Bottom)
            );
        }

        // Localized reactive sort selector
        let sort_options: Vec<LocalizedSortOption> = SortOption::ALL
            .iter()
            .map(|&s| LocalizedSortOption {
                sort: s,
                lang: self.language,
            })
            .collect();

        let selected_sort = LocalizedSortOption {
            sort: self.sort_by,
            lang: self.language,
        };

        let sort_picker = pick_list(
            sort_options,
            Some(selected_sort),
            |loc| Message::SortChanged(loc.sort),
        );

        let category_scroll = scrollable(category_pills)
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::new()
                    .width(3)
                    .margin(1)
                    .scroller_width(3),
            ))
            .style(CustomStyles::discrete_horizontal_scrollbar_style())
            .width(Length::Fill);

        row![
            search_bar,
            category_scroll,
            sort_picker,
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .into()
    }

    /// Generates scrollable device cards list
    fn view_device_list<'a>(&'a self, devices: &[&'a Device]) -> Element<'a, Message> {
        if devices.is_empty() {
            let empty_text = if self.is_scanning && !self.is_background_scan {
                Translations::empty_scanning(self.language)
            } else if !self.search_query.is_empty() || self.selected_category.is_some() {
                Translations::empty_filtered(self.language)
            } else {
                Translations::empty_initial(self.language)
            };

            return container(
                column![
                    text("🔍").size(40),
                    text(empty_text).size(16).color(CustomStyles::text_muted()),
                ]
                .spacing(12)
                .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(40)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into();
        }

        let mut list_col = Column::new().spacing(8).padding(Padding {
            top: 2.0,
            right: 6.0,
            bottom: 12.0,
            left: 2.0,
        });

        for device in devices {
            list_col = list_col.push(self.view_device_card(device));
        }

        scrollable(list_col)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(6.0)
                    .scroller_width(6.0)
                    .margin(2.0)
                    .spacing(8.0),
            ))
            .style(CustomStyles::scrollbar_style())
            .height(Length::Fill)
            .into()
    }

    /// Individual device card with responsive 2-level layout (no overflow or overlap)
    fn view_device_card<'a>(&'a self, device: &'a Device) -> Element<'a, Message> {
        let is_selected = self.selected_device_ip == Some(device.ip);

        // Category icon
        let icon_widget = text(device.category.icon()).size(24);

        // Device name
        let name_widget = text(device.display_name())
            .size(14)
            .color(if device.is_gateway || device.is_current_host {
                CustomStyles::accent_cyan()
            } else {
                CustomStyles::text_primary()
            });

        // Special role badges
        let mut role_tags = row![].spacing(4).align_y(Alignment::Center);

        if device.is_blocked {
            role_tags = role_tags.push(
                container(text(Translations::badge_blocked(self.language)).size(9).color(Color::WHITE))
                    .padding(Padding::from([1, 5]))
                    .style(|_| CustomStyles::badge_pill(Color::from_rgb8(185, 28, 28), Color::WHITE)),
            );
        } else if device.is_whitelisted {
            role_tags = role_tags.push(
                container(text(Translations::badge_whitelisted(self.language)).size(9).color(Color::WHITE))
                    .padding(Padding::from([1, 5]))
                    .style(|_| CustomStyles::badge_pill(Color::from_rgb8(16, 120, 70), Color::WHITE)),
            );
        }

        if device.is_gateway {
            role_tags = role_tags.push(
                container(text(Translations::badge_router(self.language)).size(9).color(Color::WHITE))
                    .padding(Padding::from([1, 5]))
                    .style(|_| CustomStyles::badge_pill(CustomStyles::accent_cyan(), Color::WHITE)),
            );
        }
        if device.is_current_host {
            role_tags = role_tags.push(
                container(text(Translations::badge_local_host(self.language)).size(9).color(Color::WHITE))
                    .padding(Padding::from([1, 5]))
                    .style(|_| CustomStyles::badge_pill(CustomStyles::accent_purple(), Color::WHITE)),
            );
        }
        if device.is_randomized_mac {
            role_tags = role_tags.push(
                container(text(Translations::badge_privacy(self.language)).size(9).color(CustomStyles::accent_amber()))
                    .padding(Padding::from([1, 5]))
                    .style(|_| CustomStyles::badge_pill(Color::from_rgb8(69, 45, 12), CustomStyles::accent_amber())),
            );
        }

        // Latency & Connection Quality
        let (quality_label, quality_color, lat_str) = if let Some(lat) = device.latency_ms {
            if lat < 1.0 {
                (Translations::latency_cable(self.language), CustomStyles::accent_cyan(), format!("{:.1} ms", lat))
            } else if lat < 20.0 {
                (Translations::latency_excellent(self.language), CustomStyles::accent_emerald(), format!("{:.1} ms", lat))
            } else if lat < 70.0 {
                (Translations::latency_good(self.language), CustomStyles::accent_emerald(), format!("{:.1} ms", lat))
            } else if lat < 140.0 {
                (Translations::latency_fair(self.language), CustomStyles::accent_amber(), format!("{:.1} ms", lat))
            } else {
                (Translations::latency_weak(self.language), Color::from_rgb8(239, 68, 68), format!("{:.1} ms", lat))
            }
        } else {
            (Translations::latency_no_response(self.language), CustomStyles::text_muted(), "-".to_string())
        };

        let latency_widget = container(
            row![
                text("🟢").size(8),
                text(lat_str).size(11).color(quality_color),
                text(format!("({})", quality_label)).size(10).color(CustomStyles::text_muted()),
            ]
            .spacing(4)
            .align_y(Alignment::Center),
        )
        .padding(Padding::from([2, 6]))
        .style(move |_| CustomStyles::badge_pill(Color::from_rgb8(18, 25, 38), quality_color));

        // Details button
        let btn_details = self.tip(
            button(
                text(Translations::details_btn(self.language)).size(11),
            )
            .style(move |_, _| {
                if is_selected {
                    CustomStyles::primary_button()
                } else {
                    CustomStyles::secondary_button()
                }
            })
            .padding(Padding::from([4, 10]))
            .on_press(Message::SelectDevice(device.ip)),
            Translations::tip_device_details(self.language),
            tooltip::Position::Top,
        );

        // Row 1 (Top): Icon + Name + Role badges + Spacer + Latency + Details button
        let top_row = row![
            icon_widget,
            name_widget,
            role_tags,
            space::horizontal(),
            latency_widget,
            btn_details,
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .width(Length::Fill);

        // Row 2 (Bottom): Metadata in clean pill layout
        // 1. IP with copy button
        let ip_pill = container(
            row![
                text("IP: ").size(10).color(CustomStyles::text_muted()),
                text(device.ip.to_string()).size(11).color(CustomStyles::text_primary()),
                self.tip(
                    button(text("📋").size(9))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([1, 3]))
                        .on_press(Message::CopyToClipboard(device.ip.to_string(), "IP".to_string())),
                    Translations::tip_copy_ip(self.language),
                    tooltip::Position::Top,
                ),
            ]
            .spacing(2)
            .align_y(Alignment::Center),
        )
        .padding(Padding::from([2, 6]))
        .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 25, 40), CustomStyles::text_primary()));

        // 2. MAC with copy button
        let mac_pill = container(
            row![
                text("MAC: ").size(10).color(CustomStyles::text_muted()),
                text(&device.mac).size(10).color(CustomStyles::text_secondary()),
                self.tip(
                    button(text("📋").size(9))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([1, 3]))
                        .on_press(Message::CopyToClipboard(device.mac.clone(), "MAC".to_string())),
                    Translations::tip_copy_mac(self.language),
                    tooltip::Position::Top,
                ),
            ]
            .spacing(2)
            .align_y(Alignment::Center),
        )
        .padding(Padding::from([2, 6]))
        .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 25, 40), CustomStyles::text_secondary()));

        // 3. Category
        let cat_label = Translations::category_name(device.category, self.language)
            .split('/')
            .next()
            .unwrap_or("")
            .trim();
        let cat_pill = container(text(cat_label).size(10).color(CustomStyles::accent_cyan()))
            .padding(Padding::from([2, 6]))
            .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 35, 55), CustomStyles::accent_cyan()));

        // 4. Vendor
        let vendor_str = if !device.vendor.is_empty() && device.vendor != "Desconocido" {
            device.vendor.clone()
        } else if device.is_randomized_mac {
            Translations::vendor_private(self.language).to_string()
        } else {
            Translations::vendor_generic(self.language).to_string()
        };
        let vendor_truncated = if vendor_str.len() > 20 {
            format!("{}...", &vendor_str[..18])
        } else {
            vendor_str
        };
        let vendor_pill = container(text(vendor_truncated).size(10).color(CustomStyles::text_secondary()))
            .padding(Padding::from([2, 6]))
            .style(|_| CustomStyles::badge_pill(Color::from_rgb8(25, 30, 45), CustomStyles::text_secondary()));

        let mut bottom_row = row![
            ip_pill,
            mac_pill,
            cat_pill,
            vendor_pill,
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        // 5. Router topology (if available)
        if let Some(conn_type) = &device.connection_type {
            let (top_label, top_bg, top_color) = if conn_type.contains("5GHz") || conn_type.contains("5 GHz") {
                let sig = device.wifi_signal_dbm.map(|s| format!(" ({} dBm)", s)).unwrap_or_default();
                (format!("📶 5GHz{}", sig), Color::from_rgb8(15, 45, 40), CustomStyles::accent_emerald())
            } else if conn_type.contains("2.4GHz") || conn_type.contains("2.4 GHz") {
                let sig = device.wifi_signal_dbm.map(|s| format!(" ({} dBm)", s)).unwrap_or_default();
                (format!("📶 2.4GHz{}", sig), Color::from_rgb8(45, 35, 15), CustomStyles::accent_amber())
            } else {
                let mbps = device.negotiated_mbps.map(|m| format!(" {}M", m)).unwrap_or_default();
                (format!("🔌 LAN{}", mbps), Color::from_rgb8(25, 30, 50), CustomStyles::accent_cyan())
            };

            bottom_row = bottom_row.push(
                container(text(top_label).size(10).color(top_color))
                    .padding(Padding::from([2, 6]))
                    .style(move |_| CustomStyles::badge_pill(top_bg, top_color)),
            );
        }

        // Detected ports/services (max 2 on card to avoid clutter)
        for p in device.open_ports.iter().take(2) {
            bottom_row = bottom_row.push(
                container(text(format!("{}: {}", p.port, p.service)).size(9).color(CustomStyles::accent_cyan()))
                    .padding(Padding::from([1, 4]))
                    .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 40, 60), CustomStyles::accent_cyan())),
            );
        }

        let card_content = column![
            top_row,
            bottom_row,
        ]
        .spacing(8)
        .width(Length::Fill);

        container(card_content)
            .padding(10)
            .width(Length::Fill)
            .style(move |_| {
                if is_selected {
                    CustomStyles::card_highlighted()
                } else {
                    CustomStyles::card_container()
                }
            })
            .into()
    }

    /// Detail inspector for selected device (fixed-width reactive side panel)
    fn view_inspector<'a>(&'a self, device: &'a Device) -> Element<'a, Message> {
        // Fixed top bar of inspector (Title + Close)
        // Kept pinned to top so it is never hidden while scrolling
        let close_btn = self.tip(
            button(text("✕").size(13))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([2, 6]))
                .on_press(Message::CloseInspector),
            Translations::tip_close(self.language),
            tooltip::Position::Bottom,
        );

        let top_bar = row![
            text(format!("{} {}", device.category.icon(), Translations::device_details(self.language)))
                .size(15)
                .color(CustomStyles::accent_cyan()),
            space::horizontal(),
            close_btn,
        ]
        .align_y(Alignment::Center);

        let mut insp_body = Column::new().spacing(12).padding(Padding {
            top: 2.0,
            right: 6.0,
            bottom: 12.0,
            left: 2.0,
        });

        // Name and localized subtitle
        insp_body = insp_body.push(
            column![
                text(device.display_name()).size(16).color(CustomStyles::text_primary()),
                text(Translations::device_subtitle(device, self.language)).size(11).color(CustomStyles::text_muted()),
            ]
            .spacing(1),
        );

        // Device type selector (Localized)
        let category_options: Vec<LocalizedCategory> = DeviceCategory::all()
            .iter()
            .map(|&c| LocalizedCategory {
                category: c,
                lang: self.language,
            })
            .collect();

        let selected_category = LocalizedCategory {
            category: device.category,
            lang: self.language,
        };

        let category_selector = row![
            text(Translations::device_type_label(self.language))
                .size(11)
                .color(CustomStyles::text_muted())
                .width(110),
            pick_list(
                category_options,
                Some(selected_category),
                move |loc| Message::ChangeDeviceCategory(device.ip, loc.category),
            ),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        insp_body = insp_body.push(category_selector);

        // Custom alias editor
        let save_alias_btn = self.tip(
            button(text(Translations::save(self.language)).size(11))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([5, 8]))
                .on_press(Message::SaveAlias(device.ip)),
            Translations::tip_save_alias(self.language),
            tooltip::Position::Top,
        );

        let alias_row = row![
            text_input(Translations::alias_placeholder(self.language), &self.alias_input)
                .on_input(Message::AliasInputChanged)
                .padding(Padding::from([5, 8]))
                .width(Length::Fill),
            save_alias_btn,
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        insp_body = insp_body.push(alias_row);

        // Technical specs (Fully localized)
        let mut specs_col = Column::new().spacing(5);

        specs_col = specs_col.push(Self::spec_line(Translations::spec_ip(self.language), &device.ip.to_string()));
        specs_col = specs_col.push(Self::spec_line(Translations::spec_mac(self.language), &device.mac));
        specs_col = specs_col.push(Self::spec_line(Translations::spec_vendor(self.language), &device.vendor));
        specs_col = specs_col.push(Self::spec_line(
            Translations::spec_hostname(self.language),
            device.hostname.as_deref().unwrap_or(Translations::spec_not_assigned(self.language)),
        ));
        if let Some(model) = &device.model_name {
            specs_col = specs_col.push(Self::spec_line("Modelo / Dispositivo", model));
        }
        if !device.discovered_services.is_empty() {
            let services_str = device.discovered_services.join(", ");
            specs_col = specs_col.push(Self::spec_line("Servicios mDNS/SSDP", &services_str));
        }
        specs_col = specs_col.push(Self::spec_line(
            Translations::spec_mac_type(self.language),
            if device.is_randomized_mac {
                Translations::mac_type_private(self.language)
            } else {
                Translations::mac_type_global(self.language)
            },
        ));

        // Estimated Wi-Fi link / network quality
        specs_col = specs_col.push(Self::spec_line(
            Translations::spec_link_quality(self.language),
            Translations::link_quality_desc(device.latency_ms, self.language),
        ));

        // Linksys Mesh topology info if available
        if let Some(conn_type) = &device.connection_type {
            let label = if conn_type.contains("5GHz") || conn_type.contains("5 GHz") {
                let sig = device.wifi_signal_dbm.map(|s| format!(" ({} dBm)", s)).unwrap_or_default();
                format!("{} {}", Translations::topology_wifi_5g(self.language), sig)
            } else if conn_type.contains("2.4GHz") || conn_type.contains("2.4 GHz") {
                let sig = device.wifi_signal_dbm.map(|s| format!(" ({} dBm)", s)).unwrap_or_default();
                format!("{} {}", Translations::topology_wifi_24g(self.language), sig)
            } else {
                let mbps = device.negotiated_mbps.map(|m| format!(" ({} Mbps)", m)).unwrap_or_default();
                format!("{}{}", Translations::topology_ethernet(self.language), mbps)
            };
            specs_col = specs_col.push(Self::spec_line("Topología Router", &label));
        }

        specs_col = specs_col.push(Self::spec_line(Translations::spec_interface(self.language), &device.interface));
        specs_col = specs_col.push(Self::spec_line(
            Translations::spec_status(self.language),
            Translations::status_label(device.status, self.language),
        ));
        specs_col = specs_col.push(Self::spec_line(
            Translations::spec_last_seen(self.language),
            &device.last_seen.format("%H:%M:%S").to_string(),
        ));

        insp_body = insp_body.push(
            container(specs_col)
                .padding(8)
                .style(|_| CustomStyles::card_container()),
        );

        // Quick actions (Copy, Open in browser, WoL)
        let mut quick_actions = row![].spacing(6).align_y(Alignment::Center);

        quick_actions = quick_actions.push(
            self.tip(
                button(text("📋 IP").size(11))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([4, 7]))
                    .on_press(Message::CopyToClipboard(device.ip.to_string(), "IP".to_string())),
                Translations::tip_copy_ip(self.language),
                tooltip::Position::Top,
            ),
        );

        quick_actions = quick_actions.push(
            self.tip(
                button(text("📋 MAC").size(11))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([4, 7]))
                    .on_press(Message::CopyToClipboard(device.mac.clone(), "MAC".to_string())),
                Translations::tip_copy_mac(self.language),
                tooltip::Position::Top,
            ),
        );

        let has_web = device.open_ports.iter().any(|p| p.port == 80 || p.port == 443 || p.port == 8080);
        if has_web || device.is_gateway {
            quick_actions = quick_actions.push(
                button(text(Translations::action_open_web(self.language)).size(11))
                    .style(|_, _| CustomStyles::primary_button())
                    .padding(Padding::from([4, 7]))
                    .on_press(Message::OpenInBrowser(device.ip)),
            );
        }

        if device.is_gateway || device.open_ports.iter().any(|p| p.port == 445 || p.port == 139) {
            quick_actions = quick_actions.push(
                self.tip(
                    button(text(Translations::storage_badge_available(self.language)).size(11))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([4, 7]))
                        .on_press(Message::OpenStorageExplorer),
                    Translations::tip_router_storage(self.language),
                    tooltip::Position::Top,
                ),
            );
        }

        if !device.is_current_host {
            quick_actions = quick_actions.push(
                self.tip(
                    button(text("⏰ WoL").size(11))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([4, 7]))
                        .on_press(Message::SendWol(device.mac.clone())),
                    Translations::tip_wol(self.language),
                    tooltip::Position::Top,
                ),
            );
        }

        insp_body = insp_body.push(quick_actions);

        // Security & Router Control Actions (Whitelist & Permanent Block)
        let mut security_actions = row![].spacing(6).align_y(Alignment::Center);

        let (wl_label, wl_style) = if device.is_whitelisted {
            (Translations::btn_remove_whitelist(self.language), CustomStyles::secondary_button())
        } else {
            (Translations::btn_add_whitelist(self.language), CustomStyles::primary_button())
        };
        security_actions = security_actions.push(
            self.tip(
                button(text(wl_label).size(11))
                    .style(move |_, _| wl_style.clone())
                    .padding(Padding::from([4, 7]))
                    .on_press(Message::ToggleDeviceWhitelist(device.ip, device.mac.clone())),
                Translations::tip_whitelist(self.language),
                tooltip::Position::Top,
            ),
        );

        if !device.is_current_host && !device.is_gateway {
            let is_this_device_operating = self.is_operating_device == Some(device.ip);
            let (block_label, block_style) = if is_this_device_operating {
                (Translations::btn_applying(self.language), CustomStyles::secondary_button())
            } else if device.is_blocked {
                (Translations::btn_unblock_router(self.language), CustomStyles::success_button())
            } else {
                (Translations::btn_block_router(self.language), CustomStyles::danger_button())
            };
            let mut block_btn = button(text(block_label).size(11))
                .style(move |_, _| block_style.clone())
                .padding(Padding::from([4, 7]));

            if !is_this_device_operating {
                block_btn = block_btn.on_press(Message::ToggleDeviceBlocked(device.ip, device.mac.clone()));
            }

            security_actions = security_actions.push(
                self.tip(block_btn, Translations::tip_block_router(self.language), tooltip::Position::Top),
            );
        }

        insp_body = insp_body.push(security_actions);

        // Tool 1: Real-time Ping Test (Localized)
        let ping_btn_text = if self.is_pinging {
            Translations::btn_measuring(self.language)
        } else {
            Translations::btn_ping_test(self.language)
        };

        let ping_btn = self.tip(
            button(text(ping_btn_text).size(11))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([5, 8]))
                .on_press(Message::StartPingTest(device.ip)),
            Translations::tip_ping(self.language),
            tooltip::Position::Top,
        );

        let mut ping_section = column![
            row![
                text(Translations::section_latency(self.language)).size(11).color(CustomStyles::text_muted()),
                space::horizontal(),
                ping_btn,
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(5);

        if let Some(stats) = &self.inspector_ping_stats {
            let ping_sparkline = Canvas::new(Sparkline::new(
                self.ping_history.iter().copied().collect(),
                CustomStyles::accent_cyan(),
            ))
            .width(Length::Fill)
            .height(20);

            ping_section = ping_section.push(
                column![
                    ping_sparkline,
                    row![
                        text(format!("{} {}", Translations::ping_sent(self.language), stats.transmitted)).size(11).color(CustomStyles::text_secondary()),
                        text(format!("{} {}", Translations::ping_received(self.language), stats.received)).size(11).color(CustomStyles::accent_emerald()),
                        text(format!("{} {:.0}%", Translations::ping_loss(self.language), stats.loss_percent)).size(11).color(if stats.loss_percent > 0.0 { CustomStyles::accent_amber() } else { CustomStyles::accent_emerald() }),
                    ]
                    .spacing(8),
                    row![
                        text(format!("{} {:.2} ms", Translations::ping_min(self.language), stats.min_ms)).size(11).color(CustomStyles::text_secondary()),
                        text(format!("{} {:.2} ms", Translations::ping_avg(self.language), stats.avg_ms)).size(11).color(CustomStyles::accent_cyan()),
                        text(format!("{} {:.2} ms", Translations::ping_max(self.language), stats.max_ms)).size(11).color(CustomStyles::text_secondary()),
                    ]
                    .spacing(8),
                ]
                .spacing(3),
            );
        }

        insp_body = insp_body.push(
            container(ping_section)
                .padding(8)
                .style(|_| CustomStyles::card_container()),
        );

        // Tool 2: Port Scanner (Localized)
        let port_btn_text = if self.is_deep_scanning {
            Translations::btn_scanning_ports(self.language)
        } else {
            Translations::btn_scan_ports(self.language)
        };

        let port_btn = self.tip(
            button(text(port_btn_text).size(11))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([5, 8]))
                .on_press(Message::StartDeepScan(device.ip)),
            Translations::tip_port_scan(self.language),
            tooltip::Position::Top,
        );

        let mut ports_section = column![
            row![
                text(Translations::section_ports(self.language)).size(11).color(CustomStyles::text_muted()),
                space::horizontal(),
                port_btn,
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(5);

        if self.deep_ports.is_empty() {
            ports_section = ports_section.push(
                text(if self.is_deep_scanning {
                    Translations::ports_checking(self.language)
                } else {
                    Translations::no_open_ports(self.language)
                })
                .size(11)
                .color(CustomStyles::text_muted()),
            );
        } else {
            let mut port_list = Column::new().spacing(4).padding(Padding {
                top: 0.0,
                right: 4.0,
                bottom: 2.0,
                left: 0.0,
            });
            for p in &self.deep_ports {
                port_list = port_list.push(
                    row![
                        container(text(format!("TCP {}", p.port)).size(9).color(CustomStyles::accent_cyan()))
                            .padding(Padding::from([1, 4]))
                            .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 30, 50), CustomStyles::accent_cyan())),
                        text(&p.service).size(11).color(CustomStyles::text_primary()),
                        text(format!("({})", Translations::port_description(p.port, self.language))).size(10).color(CustomStyles::text_muted()),
                    ]
                    .spacing(5)
                    .align_y(Alignment::Center),
                );
            }
            ports_section = ports_section.push(
                scrollable(port_list)
                    .direction(scrollable::Direction::Vertical(
                        scrollable::Scrollbar::new()
                            .width(5.0)
                            .scroller_width(5.0)
                            .margin(1.0)
                            .spacing(6.0),
                    ))
                    .style(CustomStyles::scrollbar_style())
                    .height(110),
            );
        }

        insp_body = insp_body.push(
            container(ports_section)
                .padding(8)
                .style(|_| CustomStyles::card_container()),
        );

        // Tool 3: Route Diagnostics / Visual Traceroute
        let trace_btn_text = if self.is_tracing_route {
            Translations::traceroute_btn_tracing(self.language)
        } else {
            Translations::traceroute_btn_start(self.language)
        };

        let trace_btn = self.tip(
            button(text(trace_btn_text).size(11))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([5, 8]))
                .on_press(Message::StartTraceroute(device.ip)),
            Translations::tip_traceroute(self.language),
            tooltip::Position::Top,
        );

        let mut trace_section = column![
            row![
                text(Translations::traceroute_title(self.language)).size(11).color(CustomStyles::text_muted()),
                space::horizontal(),
                trace_btn,
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(5);

        if let Some(err) = &self.traceroute_error {
            trace_section = trace_section.push(text(err).size(11).color(Color::from_rgb8(239, 68, 68)));
        } else if let Some(hops) = &self.traceroute_hops {
            let hops_header = row![
                text(Translations::traceroute_hop_col(self.language)).size(10).color(CustomStyles::text_muted()).width(36),
                text(Translations::traceroute_ip_col(self.language)).size(10).color(CustomStyles::text_muted()).width(190),
                space::horizontal(),
                text(Translations::traceroute_latency_col(self.language)).size(10).color(CustomStyles::text_muted()),
            ]
            .spacing(6);
            trace_section = trace_section.push(hops_header);

            let mut hops_col = Column::new().spacing(4).padding(Padding {
                top: 0.0,
                right: 4.0,
                bottom: 2.0,
                left: 0.0,
            });
            for h in hops {
                let (ip_str, latency_str, color) = if h.is_timeout {
                    ("---".to_string(), Translations::traceroute_timeout(self.language).to_string(), Color::from_rgb8(239, 68, 68))
                } else {
                    let lat = h.latency_ms.map(|ms| format!("{:.1} ms", ms)).unwrap_or_else(|| "-- ms".to_string());
                    let col = if h.latency_ms.map(|ms| ms > 50.0).unwrap_or(false) {
                        CustomStyles::accent_amber()
                    } else {
                        CustomStyles::accent_emerald()
                    };
                    let ip = h.ip.as_deref().unwrap_or("?");
                    let label = h.label.as_deref().map(|l| format!(" ({})", l)).unwrap_or_default();
                    (format!("{}{}", ip, label), lat, col)
                };

                hops_col = hops_col.push(
                    row![
                        container(text(format!("#{}", h.hop)).size(9).color(CustomStyles::accent_cyan()))
                            .padding(Padding::from([1, 4]))
                            .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 30, 50), CustomStyles::accent_cyan())),
                        text(ip_str).size(10).color(CustomStyles::text_primary()).width(190),
                        space::horizontal(),
                        text(latency_str).size(10).color(color),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                );
            }
            trace_section = trace_section.push(
                scrollable(hops_col)
                    .direction(scrollable::Direction::Vertical(
                        scrollable::Scrollbar::new()
                            .width(5.0)
                            .scroller_width(5.0)
                            .margin(1.0)
                            .spacing(6.0),
                    ))
                    .style(CustomStyles::scrollbar_style())
                    .height(120),
            );
        }

        insp_body = insp_body.push(
            container(trace_section)
                .padding(8)
                .style(|_| CustomStyles::card_container()),
        );

        // Tool 4: Device Requests & Domain History
        let req_count = device.request_history.len();
        let mut req_header = row![
            text(Translations::device_requests_title(self.language)).size(12).color(CustomStyles::accent_cyan()),
            container(text(format!("{}", req_count)).size(9).color(CustomStyles::accent_cyan()))
                .padding(Padding::from([1, 5]))
                .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 30, 50), CustomStyles::accent_cyan())),
            space::horizontal(),
            button(text(Translations::device_requests_simulate_btn(self.language)).size(10))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([2, 6]))
                .on_press(Message::AddDeviceSampleRequest(device.ip)),
        ]
        .align_y(Alignment::Center)
        .spacing(6);

        if req_count > 0 {
            req_header = req_header.push(
                button(text("🗑️").size(10))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([2, 5]))
                    .on_press(Message::ClearDeviceRequestHistory(device.ip)),
            );
        }

        let mut req_section = column![req_header].spacing(6);

        if device.request_history.is_empty() {
            req_section = req_section.push(
                text(Translations::device_requests_empty(self.language))
                    .size(10)
                    .color(CustomStyles::text_muted()),
            );
        } else {
            let mut req_list = Column::new().spacing(5).padding(Padding {
                top: 0.0,
                right: 4.0,
                bottom: 2.0,
                left: 0.0,
            });

            for entry in &device.request_history {
                let time_str = entry.timestamp.format("%H:%M:%S").to_string();
                let cat_badge_color = if entry.category.contains("Streaming") {
                    CustomStyles::accent_purple()
                } else if entry.category.contains("Búsqueda") {
                    CustomStyles::accent_emerald()
                } else if entry.category.contains("Red") || entry.category.contains("DNS") {
                    CustomStyles::accent_cyan()
                } else {
                    CustomStyles::accent_amber()
                };

                let item = container(
                    column![
                        row![
                            text(time_str).size(9).color(CustomStyles::text_muted()),
                            container(text(&entry.protocol).size(8).color(CustomStyles::accent_cyan()))
                                .padding(Padding::from([1, 4]))
                                .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 25, 40), CustomStyles::accent_cyan())),
                            container(text(&entry.category).size(8).color(cat_badge_color))
                                .padding(Padding::from([1, 4]))
                                .style(move |_| CustomStyles::badge_pill(Color::from_rgb8(20, 30, 45), cat_badge_color)),
                        ]
                        .spacing(5)
                        .align_y(Alignment::Center),
                        text(&entry.query).size(11).color(CustomStyles::text_primary()),
                    ]
                    .spacing(2),
                )
                .padding(6)
                .width(Length::Fill)
                .style(|_| CustomStyles::card_container());

                req_list = req_list.push(item);
            }

            req_section = req_section.push(
                scrollable(req_list)
                    .direction(scrollable::Direction::Vertical(
                        scrollable::Scrollbar::new()
                            .width(5.0)
                            .scroller_width(5.0)
                            .margin(1.0)
                            .spacing(6.0),
                    ))
                    .style(CustomStyles::scrollbar_style())
                    .height(160),
            );
        }

        insp_body = insp_body.push(
            container(req_section)
                .padding(8)
                .style(|_| CustomStyles::card_container()),
        );

        let body_scrollable = scrollable(insp_body)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(6.0)
                    .scroller_width(6.0)
                    .margin(2.0)
                    .spacing(8.0),
            ))
            .style(CustomStyles::scrollbar_style())
            .height(Length::Fill);

        column![
            top_bar,
            body_scrollable,
        ]
        .spacing(12)
        .height(Length::Fill)
        .into()
    }

    /// Key-value technical spec row
    fn spec_line(key: &'static str, val: &str) -> Element<'static, Message> {
        row![
            text(key).size(11).color(CustomStyles::text_muted()).width(120),
            text(val.to_string()).size(11).color(CustomStyles::text_primary()),
        ]
        .spacing(4)
        .align_y(Alignment::Center)
        .into()
    }

    /// Floating result notification
    fn view_notification<'a>(&self, message: &'a str, is_success: bool) -> Element<'a, Message> {
        let style_fn = if is_success {
            CustomStyles::notification_success
        } else {
            CustomStyles::notification_error
        };

        let notif_row = row![
            text(if is_success { "✓" } else { "⚠" }).size(13),
            text(message).size(12),
            space::horizontal(),
            button(text("✕").size(11))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([2, 5]))
                .on_press(Message::DismissNotification),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        container(notif_row)
            .padding(6)
            .width(Length::Fill)
            .style(move |_| style_fn())
            .into()
    }

    /// Filters and sorts devices according to current user preferences
    fn get_filtered_devices(&self) -> Vec<&Device> {
        let q = self.search_query.trim().to_lowercase();

        let mut list: Vec<&Device> = self
            .devices
            .iter()
            .filter(|d| {
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
            .collect();

        match self.sort_by {
            SortOption::Ip => {
                list.sort_by_key(|d| d.ip);
            }
            SortOption::Latency => {
                list.sort_by(|a, b| {
                    let lat_a = a.latency_ms.unwrap_or(f32::MAX);
                    let lat_b = b.latency_ms.unwrap_or(f32::MAX);
                    lat_a.partial_cmp(&lat_b).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            SortOption::Name => {
                list.sort_by(|a, b| a.display_name().to_lowercase().cmp(&b.display_name().to_lowercase()));
            }
            SortOption::Vendor => {
                list.sort_by(|a, b| a.vendor.to_lowercase().cmp(&b.vendor.to_lowercase()));
            }
            SortOption::Category => {
                list.sort_by_key(|d| Translations::category_name(d.category, self.language));
            }
        }

        list
    }

    /// Modal: Comprehensive Network Security Audit
    fn view_security_audit_modal(&self) -> Element<'_, Message> {
        let title_row = row![
            text(Translations::audit_title(self.language))
                .size(20)
                .color(CustomStyles::accent_cyan()),
            space::horizontal(),
            self.tip(
                button(text("✕").size(14))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([2, 8]))
                    .on_press(Message::CloseSecurityAudit),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .align_y(Alignment::Center);

        let subtitle = text(Translations::audit_subtitle(self.language))
            .size(11)
            .color(CustomStyles::text_muted());

        let (score_val, grade_str, issues_cnt, crit_cnt) = if let Some(audit) = &self.security_audit_report {
            (audit.score, Translations::audit_grade_label(audit.score, self.language), audit.issues_count, audit.critical_count)
        } else {
            (100, Translations::audit_not_run(self.language), 0, 0)
        };
        let checks_list = self.security_audit_report.as_ref().map(|a| a.checks.as_slice()).unwrap_or(&[]);

        let score_color = if score_val >= 85 {
            CustomStyles::accent_emerald()
        } else if score_val >= 60 {
            CustomStyles::accent_amber()
        } else {
            Color::from_rgb8(239, 68, 68)
        };

        // Score Card
        let score_card = container(
            row![
                column![
                    text(format!("{}/100", score_val)).size(32).color(score_color),
                    text(format!("{}: {}", Translations::audit_score_label(self.language), grade_str)).size(12).color(score_color),
                ]
                .spacing(2),
                space::horizontal(),
                column![
                    text(Translations::audit_devices_audited(self.devices.len(), self.language)).size(11).color(CustomStyles::text_secondary()),
                    text(Translations::audit_observations(issues_cnt, self.language)).size(11).color(if issues_cnt > 0 { CustomStyles::accent_amber() } else { CustomStyles::accent_emerald() }),
                    text(Translations::audit_critical_risks(crit_cnt, self.language)).size(11).color(if crit_cnt > 0 { Color::from_rgb8(239, 68, 68) } else { CustomStyles::accent_emerald() }),
                ]
                .spacing(3),
            ]
            .align_y(Alignment::Center)
            .padding(14),
        )
        .width(Length::Fill)
        .style(|_| CustomStyles::card_container());

        let mut checks_col = Column::new().spacing(10).padding(Padding {
            top: 4.0,
            right: 12.0,
            bottom: 8.0,
            left: 4.0,
        });

        for check in checks_list {
            let (icon, icon_color, border_bg) = match check.severity {
                AuditSeverity::Pass => ("✅", CustomStyles::accent_emerald(), Color::from_rgb8(15, 35, 25)),
                AuditSeverity::Info => ("ℹ️", CustomStyles::accent_cyan(), Color::from_rgb8(15, 30, 45)),
                AuditSeverity::Warning => ("⚠️", CustomStyles::accent_amber(), Color::from_rgb8(45, 35, 15)),
                AuditSeverity::Critical => ("🚨", Color::from_rgb8(239, 68, 68), Color::from_rgb8(50, 15, 15)),
            };

            let mut check_body = column![
                row![
                    text(icon).size(14),
                    text(check.title_localized(self.language)).size(13).color(icon_color),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
                text(check.description_localized(self.language)).size(11).color(CustomStyles::text_secondary()),
            ]
            .spacing(4);

            if !check.affected_devices.is_empty() {
                let mut dev_col = Column::new().spacing(5);
                let count_label = Translations::audit_affected_devices(check.affected_devices.len(), self.language);
                dev_col = dev_col.push(
                    text(count_label).size(11).color(CustomStyles::text_muted())
                );

                for chunk in check.affected_devices.chunks(3) {
                    let mut chunk_row = row![].spacing(6).align_y(Alignment::Center);
                    for d in chunk {
                        chunk_row = chunk_row.push(
                            container(text(d).size(10).color(CustomStyles::accent_amber()))
                                .padding(Padding::from([3, 8]))
                                .style(|_| CustomStyles::badge_pill(Color::from_rgb8(45, 35, 15), CustomStyles::accent_amber())),
                        );
                    }
                    dev_col = dev_col.push(chunk_row);
                }
                check_body = check_body.push(dev_col);
            }

            check_body = check_body.push(
                row![
                    text("💡").size(11),
                    text(check.recommendation_localized(self.language)).size(10).color(CustomStyles::accent_cyan()),
                ]
                .spacing(4)
                .align_y(Alignment::Center),
            );

            let check_card = container(check_body)
                .padding(14)
                .width(Length::Fill)
                .style(move |_| CustomStyles::badge_pill(border_bg, CustomStyles::text_primary()));

            checks_col = checks_col.push(check_card);
        }

        let checks_scrollable = scrollable(checks_col)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(6.0)
                    .scroller_width(6.0)
                    .margin(2.0)
                    .spacing(8.0),
            ))
            .style(CustomStyles::scrollbar_style())
            .height(Length::Fixed(480.0));

        let actions_row = row![
            button(text(Translations::audit_rerun_btn(self.language)).size(12))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([6, 14]))
                .on_press(Message::RunSecurityAudit),
            space::horizontal(),
            button(text(Translations::close(self.language)).size(12))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([6, 18]))
                .on_press(Message::CloseSecurityAudit),
        ]
        .align_y(Alignment::Center);

        let content = column![
            title_row,
            subtitle,
            score_card,
            checks_scrollable,
            actions_row,
        ]
        .spacing(14);

        container(content)
            .padding(24)
            .width(960)
            .style(|_| CustomStyles::modal_panel(CustomStyles::accent_cyan()))
            .into()
    }

    /// Modal: Network Event History & Timeline
    fn view_timeline_modal(&self) -> Element<'_, Message> {
        let title_row = row![
            text(Translations::timeline_modal_title(self.language))
                .size(20)
                .color(CustomStyles::accent_purple()),
            space::horizontal(),
            self.tip(
                button(text("✕").size(14))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([2, 8]))
                    .on_press(Message::CloseTimeline),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .align_y(Alignment::Center);

        let subtitle = text(Translations::timeline_modal_desc(self.language))
            .size(11)
            .color(CustomStyles::text_muted());

        let uptime_pct = self.uptime_tracker.uptime_percentage();
        let online_status = if self.uptime_tracker.is_online {
            (Translations::timeline_online(self.language), CustomStyles::accent_emerald())
        } else {
            (Translations::timeline_outage(self.language), Color::from_rgb8(239, 68, 68))
        };

        let summary_row = container(
            row![
                text(Translations::timeline_total_events(self.network_events.len(), self.language)).size(12).color(CustomStyles::text_primary()),
                text("•").size(10).color(CustomStyles::text_muted()),
                text(online_status.0).size(12).color(online_status.1),
                text("•").size(10).color(CustomStyles::text_muted()),
                text(Translations::timeline_uptime_pct(uptime_pct as f64, self.language)).size(12).color(CustomStyles::accent_cyan()),
                space::horizontal(),
                button(text(Translations::timeline_clear_btn(self.language)).size(11))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([3, 8]))
                    .on_press(Message::ClearTimeline),
            ]
            .spacing(8)
            .align_y(Alignment::Center)
            .padding(10),
        )
        .width(Length::Fill)
        .style(|_| CustomStyles::card_container());

        let mut events_col = Column::new().spacing(8).padding(Padding {
            top: 4.0,
            right: 12.0,
            bottom: 8.0,
            left: 4.0,
        });

        if self.network_events.is_empty() {
            events_col = events_col.push(
                container(
                    text(Translations::timeline_empty(self.language))
                        .size(12)
                        .color(CustomStyles::text_muted()),
                )
                .padding(20)
                .align_x(Alignment::Center)
                .width(Length::Fill),
            );
        } else {
            for evt in &self.network_events {
                let time_str = evt.timestamp.format("%H:%M:%S (%d/%m)").to_string();
                let icon = evt.event_type.icon();
                let type_label = evt.event_type.label_localized(self.language);

                let mut meta_row = row![
                    text(icon).size(12),
                    text(type_label).size(11).color(CustomStyles::accent_purple()),
                    space::horizontal(),
                    text(time_str).size(10).color(CustomStyles::text_muted()),
                ]
                .align_y(Alignment::Center)
                .spacing(6);

                if let Some(ip) = evt.ip {
                    meta_row = meta_row.push(
                        container(text(ip.to_string()).size(9).color(CustomStyles::accent_cyan()))
                            .padding(Padding::from([1, 4]))
                            .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 30, 45), CustomStyles::accent_cyan())),
                    );
                }

                let evt_card = container(
                    column![
                        meta_row,
                        text(evt.title_localized(self.language)).size(12).color(CustomStyles::text_primary()),
                        text(evt.details_localized(self.language)).size(11).color(CustomStyles::text_secondary()),
                    ]
                    .spacing(3),
                )
                .padding(10)
                .width(Length::Fill)
                .style(|_| CustomStyles::card_container());

                events_col = events_col.push(evt_card);
            }
        }

        let events_scrollable = scrollable(events_col)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(6.0)
                    .scroller_width(6.0)
                    .margin(2.0)
                    .spacing(8.0),
            ))
            .style(CustomStyles::scrollbar_style())
            .height(Length::Fixed(480.0));

        let close_row = row![
            space::horizontal(),
            button(text(Translations::close(self.language)).size(12))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([6, 18]))
                .on_press(Message::CloseTimeline),
        ]
        .align_y(Alignment::Center);

        let content = column![
            title_row,
            subtitle,
            summary_row,
            events_scrollable,
            close_row,
        ]
        .spacing(14);

        container(content)
            .padding(24)
            .width(960)
            .style(|_| CustomStyles::modal_panel(CustomStyles::accent_purple()))
            .into()
    }

    /// Modal: Wi-Fi Channel & RF Spectrum Analyzer
    fn view_wifi_analyzer_modal(&self) -> Element<'_, Message> {
        let title_row = row![
            text(Translations::wifi_modal_title(self.language))
                .size(20)
                .color(CustomStyles::accent_amber()),
            space::horizontal(),
            self.tip(
                button(text("✕").size(14))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([2, 8]))
                    .on_press(Message::CloseWifiAnalyzer),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .align_y(Alignment::Center);

        let subtitle = text(Translations::wifi_modal_desc(self.language))
            .size(11)
            .color(CustomStyles::text_muted());

        let scan_btn_label = Translations::wifi_scan_btn(self.is_scanning_wifi, self.language);

        let current_conn_info = if let Some(report) = &self.wifi_analyzer_report {
            Translations::wifi_current_network(report.current_ssid.as_deref(), report.current_channel, self.language)
        } else {
            Translations::wifi_current_network_none(self.language)
        };

        let action_row = row![
            text(current_conn_info).size(12).color(CustomStyles::accent_cyan()),
            space::horizontal(),
            button(text(scan_btn_label).size(12))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([6, 14]))
                .on_press(Message::ScanWifi),
        ]
        .align_y(Alignment::Center);

        let mut modal_body = Column::new().spacing(10).padding(Padding {
            top: 4.0,
            right: 12.0,
            bottom: 8.0,
            left: 4.0,
        });

        if let Some(report) = &self.wifi_analyzer_report {
            // Recommendation Card
            let reco_card = container(
                column![
                    row![
                        text(Translations::wifi_optimal_channels_title(self.language)).size(13).color(CustomStyles::accent_emerald()),
                        space::horizontal(),
                        text(Translations::wifi_channel_24_label(report.recommended_channel_24, self.language)).size(12).color(CustomStyles::accent_emerald()),
                        text(" | ").size(10).color(CustomStyles::text_muted()),
                        text(Translations::wifi_channel_5_label(report.recommended_channel_5, self.language)).size(12).color(CustomStyles::accent_cyan()),
                    ]
                    .align_y(Alignment::Center),
                    text(Translations::wifi_recommendation_text(report.current_channel, report.recommended_channel_24, report.recommended_channel_5, self.language)).size(11).color(CustomStyles::text_secondary()),
                ]
                .spacing(5),
            )
            .padding(12)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_download());

            modal_body = modal_body.push(reco_card);

            // 2.4 GHz Channel Summary
            modal_body = modal_body.push(
                text(Translations::wifi_occupancy_24_title(self.language))
                    .size(12)
                    .color(CustomStyles::accent_amber()),
            );

            let mut ch24_row = row![].spacing(8);
            for ch in &report.channels_24 {
                let ch_card = container(
                    column![
                        text(Translations::wifi_channel_box_title(ch.channel, self.language)).size(12).color(CustomStyles::accent_amber()),
                        text(Translations::wifi_networks_count(ch.networks_count, self.language)).size(14).color(CustomStyles::text_primary()),
                        text(Translations::wifi_max_signal(ch.max_signal, self.language)).size(10).color(CustomStyles::text_muted()),
                    ]
                    .spacing(2),
                )
                .padding(8)
                .width(Length::FillPortion(1))
                .style(|_| CustomStyles::card_container());

                ch24_row = ch24_row.push(ch_card);
            }
            modal_body = modal_body.push(ch24_row);

            // Detected Wi-Fi Networks
            modal_body = modal_body.push(
                text(Translations::wifi_detected_networks_header(report.networks.len(), self.language))
                    .size(12)
                    .color(CustomStyles::text_primary()),
            );

            for net in &report.networks {
                let freq_label = if net.frequency_mhz >= 5000 { "5 GHz" } else { "2.4 GHz" };
                let is_curr = net.is_current;

                let mut row_items = row![
                    column![
                        row![
                            text(&net.ssid).size(13).color(if is_curr { CustomStyles::accent_emerald() } else { CustomStyles::text_primary() }),
                        ]
                        .spacing(6)
                        .align_y(Alignment::Center),
                        text(format!("BSSID: {} | Seg: {}", net.bssid, net.security)).size(10).color(CustomStyles::text_muted()),
                    ]
                    .spacing(2)
                    .width(Length::Fill),
                ].align_y(Alignment::Center).spacing(8);

                if is_curr {
                    row_items = row_items.push(
                        container(text(Translations::wifi_badge_current(self.language)).size(9).color(CustomStyles::accent_emerald()))
                            .padding(Padding::from([1, 4]))
                            .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 45, 30), CustomStyles::accent_emerald())),
                    );
                }

                row_items = row_items.push(
                    container(text(format!("Ch {} ({})", net.channel, freq_label)).size(10).color(CustomStyles::accent_cyan()))
                        .padding(Padding::from([2, 6]))
                        .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 30, 45), CustomStyles::accent_cyan())),
                );

                row_items = row_items.push(
                    text(format!("{}%", net.signal_percent)).size(12).color(CustomStyles::accent_amber()),
                );

                let row_item = container(row_items)
                    .padding(8)
                    .width(Length::Fill)
                    .style(|_| CustomStyles::card_container());

                modal_body = modal_body.push(row_item);
            }
        } else {
            modal_body = modal_body.push(
                container(
                    text(Translations::wifi_prompt_scan(self.language))
                        .size(12)
                        .color(CustomStyles::text_muted()),
                )
                .padding(20)
                .align_x(Alignment::Center)
                .width(Length::Fill),
            );
        }

        let scrollable_area = scrollable(modal_body)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(6.0)
                    .scroller_width(6.0)
                    .margin(2.0)
                    .spacing(8.0),
            ))
            .style(CustomStyles::scrollbar_style())
            .height(Length::Fixed(480.0));

        let close_row = row![
            space::horizontal(),
            button(text(Translations::close(self.language)).size(12))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([6, 18]))
                .on_press(Message::CloseWifiAnalyzer),
        ]
        .align_y(Alignment::Center);

        let content = column![
            title_row,
            subtitle,
            action_row,
            scrollable_area,
            close_row,
        ]
        .spacing(14);

        container(content)
            .padding(24)
            .width(980)
            .style(|_| CustomStyles::modal_panel(CustomStyles::accent_amber()))
            .into()
    }

    /// Modal: Universal Router UPnP / IGD & Port Forwarding
    fn view_upnp_modal(&self) -> Element<'_, Message> {
        let title_row = row![
            text(Translations::upnp_modal_title(self.language))
                .size(20)
                .color(CustomStyles::accent_cyan()),
            space::horizontal(),
            self.tip(
                button(text("✕").size(14))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([2, 8]))
                    .on_press(Message::CloseUpnpModal),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .align_y(Alignment::Center);

        let subtitle = text(Translations::upnp_modal_desc(self.language))
            .size(11)
            .color(CustomStyles::text_muted());

        let gw_ip_str = self
            .network
            .gateway_ip
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| Translations::not_detected(self.language).to_string());

        let refresh_btn = button(text(Translations::upnp_query_btn(false, self.language)).size(12))
            .style(|_, _| CustomStyles::secondary_button())
            .padding(Padding::from([5, 12]))
            .on_press(Message::QueryUpnpRouter);

        let action_row = row![
            text(format!("{}: {}", Translations::gateway_label(self.language), gw_ip_str)).size(12).color(CustomStyles::text_secondary()),
            space::horizontal(),
            refresh_btn,
        ]
        .align_y(Alignment::Center);

        let mut modal_body = Column::new().spacing(10).padding(Padding {
            top: 4.0,
            right: 12.0,
            bottom: 8.0,
            left: 4.0,
        });

        if let Some(upnp) = &self.upnp_status {
            let wan_ip = upnp.external_ip.as_deref().unwrap_or(Translations::upnp_not_assigned(self.language));
            let conn_status = upnp.connection_status.as_deref().unwrap_or(Translations::upnp_status_connected(self.language));
            let uptime_str = upnp
                .uptime_secs
                .map(|s| format!("{}h {}m", s / 3600, (s % 3600) / 60))
                .unwrap_or_else(|| "-".to_string());
            let rx_bytes = upnp.total_bytes_received.map(Telemetry::format_bytes).unwrap_or_else(|| "-".to_string());
            let tx_bytes = upnp.total_bytes_sent.map(Telemetry::format_bytes).unwrap_or_else(|| "-".to_string());

            let stat_card_ip = container(
                column![
                    text(Translations::upnp_public_ip_title(self.language)).size(10).color(CustomStyles::accent_cyan()),
                    text(wan_ip).size(16).color(CustomStyles::accent_cyan()),
                    text(Translations::upnp_status_label(conn_status, self.language)).size(10).color(CustomStyles::text_muted()),
                ]
                .spacing(3),
            )
            .padding(10)
            .width(Length::FillPortion(1))
            .style(|_| CustomStyles::card_container());

            let stat_card_uptime = container(
                column![
                    text(Translations::upnp_uptime_title(self.language)).size(10).color(CustomStyles::accent_emerald()),
                    text(uptime_str).size(16).color(CustomStyles::accent_emerald()),
                    text(Translations::upnp_uptime_sub(self.language)).size(10).color(CustomStyles::text_muted()),
                ]
                .spacing(3),
            )
            .padding(10)
            .width(Length::FillPortion(1))
            .style(|_| CustomStyles::card_container());

            let stat_card_traffic = container(
                column![
                    text(Translations::upnp_traffic_title(self.language)).size(10).color(CustomStyles::accent_purple()),
                    text(format!("↓ {} | ↑ {}", rx_bytes, tx_bytes)).size(13).color(CustomStyles::text_primary()),
                    text(Translations::upnp_traffic_sub(self.language)).size(10).color(CustomStyles::text_muted()),
                ]
                .spacing(3),
            )
            .padding(10)
            .width(Length::FillPortion(1))
            .style(|_| CustomStyles::card_container());

            let stats_grid = row![stat_card_ip, stat_card_uptime, stat_card_traffic].spacing(8);
            modal_body = modal_body.push(stats_grid);

            // Port Forwarding
            modal_body = modal_body.push(
                text(Translations::upnp_port_mappings_header(upnp.port_mappings.len(), self.language))
                    .size(13)
                    .color(CustomStyles::accent_cyan()),
            );

            if upnp.port_mappings.is_empty() {
                modal_body = modal_body.push(
                    container(
                        text(Translations::upnp_no_mappings(self.language))
                            .size(11)
                            .color(CustomStyles::text_muted()),
                    )
                    .padding(14)
                    .align_x(Alignment::Center)
                    .width(Length::Fill),
                );
            } else {
                for m in &upnp.port_mappings {
                    let is_critical = [22, 23, 80, 445, 3389, 5900].contains(&m.external_port);

                    let port_rule_card = container(
                        row![
                            column![
                                row![
                                    text(Translations::upnp_port_label(m.external_port, &m.protocol, self.language))
                                        .size(13)
                                        .color(if is_critical { Color::from_rgb8(239, 68, 68) } else { CustomStyles::accent_cyan() }),
                                    text("➜").size(11).color(CustomStyles::text_muted()),
                                    text(format!("{}:{}", m.internal_client, m.internal_port))
                                        .size(12)
                                        .color(CustomStyles::text_primary()),
                                ]
                                .spacing(6)
                                .align_y(Alignment::Center),
                                text(if m.description.is_empty() { "UPnP Mapped Port" } else { &m.description })
                                    .size(10)
                                    .color(CustomStyles::text_muted()),
                            ]
                            .spacing(2)
                            .width(Length::Fill),
                            if is_critical {
                                container(text(Translations::upnp_badge_wan_risk(self.language)).size(9).color(Color::WHITE))
                                    .padding(Padding::from([2, 6]))
                                    .style(|_| CustomStyles::badge_pill(Color::from_rgb8(185, 28, 28), Color::WHITE))
                            } else {
                                container(text(if m.enabled { Translations::upnp_badge_active(self.language) } else { Translations::upnp_badge_inactive(self.language) }).size(9).color(CustomStyles::accent_emerald()))
                                    .padding(Padding::from([2, 6]))
                                    .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 45, 30), CustomStyles::accent_emerald()))
                            },
                        ]
                        .align_y(Alignment::Center)
                        .padding(8),
                    )
                    .width(Length::Fill)
                    .style(move |_| {
                        if is_critical {
                            CustomStyles::card_danger()
                        } else {
                            CustomStyles::card_container()
                        }
                    });

                    modal_body = modal_body.push(port_rule_card);
                }
            }
        } else {
            modal_body = modal_body.push(
                container(
                    text(Translations::upnp_prompt(self.language))
                        .size(12)
                        .color(CustomStyles::text_muted()),
                )
                .padding(20)
                .align_x(Alignment::Center)
                .width(Length::Fill),
            );
        }

        let scrollable_area = scrollable(modal_body)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(6.0)
                    .scroller_width(6.0)
                    .margin(2.0)
                    .spacing(8.0),
            ))
            .style(CustomStyles::scrollbar_style())
            .height(Length::Fixed(480.0));

        let close_row = row![
            space::horizontal(),
            button(text(Translations::close(self.language)).size(12))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([6, 18]))
                .on_press(Message::CloseUpnpModal),
        ]
        .align_y(Alignment::Center);

        let content = column![
            title_row,
            subtitle,
            action_row,
            scrollable_area,
            close_row,
        ]
        .spacing(14);

        container(content)
            .padding(24)
            .width(980)
            .style(|_| CustomStyles::modal_panel(CustomStyles::accent_cyan()))
            .into()
    }

    /// Router USB Storage / NAS Explorer
    fn view_storage_explorer_modal(&self) -> Element<'_, Message> {
        let title_row = row![
            text(Translations::storage_modal_title(self.language))
                .size(18)
                .color(CustomStyles::accent_cyan()),
            space::horizontal(),
            self.tip(
                button(text("✕").size(14))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([2, 8]))
                    .on_press(Message::CloseStorageExplorer),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .align_y(Alignment::Center);

        let subtitle = text(Translations::storage_modal_desc(self.language))
            .size(11)
            .color(CustomStyles::text_muted());

        // Row 1: Connection Control, Target IP, Credentials, and Demo Mode
        let ip_input = text_input("192.168.1.1", &self.storage_target_ip)
            .on_input(Message::StorageTargetIpChanged)
            .padding(Padding::from([4, 8]))
            .width(130);

        let refresh_btn = self.tip(
            button(text(Translations::storage_refresh(self.language)).size(11))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([4, 10]))
                .on_press(Message::RefreshStorageShares),
            Translations::tip_storage_refresh(self.language),
            tooltip::Position::Bottom,
        );

        let creds_btn = self.tip(
            button(text(Translations::storage_btn_credentials(self.language)).size(11))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([4, 10]))
                .on_press(Message::ToggleStorageCredentialsModal),
            Translations::tip_storage_creds(self.language),
            tooltip::Position::Bottom,
        );

        let is_demo = self.is_storage_demo_mode;
        let demo_btn = self.tip(
            button(text(Translations::storage_demo_mode_btn(self.language)).size(11))
                .style(move |_, _| {
                    if is_demo {
                        CustomStyles::primary_button()
                    } else {
                        CustomStyles::secondary_button()
                    }
                })
                .padding(Padding::from([4, 10]))
                .on_press(Message::StorageToggleDemoMode),
            Translations::tip_storage_demo(self.language),
            tooltip::Position::Bottom,
        );

        let connection_bar = row![
            text(Translations::storage_target_label(self.language)).size(11).color(CustomStyles::text_secondary()),
            ip_input,
            refresh_btn,
            creds_btn,
            space::horizontal(),
            demo_btn,
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let mut modal_body = column![connection_bar].spacing(10);

        // SMB / NAS Credentials dialog if open
        if self.is_storage_creds_open {
            let user_in = text_input(Translations::storage_creds_user_placeholder(self.language), &self.storage_credentials.username)
                .on_input(Message::StorageCredentialsUserChanged)
                .padding(Padding::from([4, 8]))
                .width(180);

            let pass_in = text_input(Translations::storage_creds_pass_placeholder(self.language), &self.storage_credentials.password)
                .on_input(Message::StorageCredentialsPassChanged)
                .secure(true)
                .padding(Padding::from([4, 8]))
                .width(180);

            let wg_in = text_input("WORKGROUP", &self.storage_credentials.workgroup)
                .on_input(Message::StorageCredentialsWorkgroupChanged)
                .padding(Padding::from([4, 8]))
                .width(140);

            let creds_card = container(
                column![
                    text(Translations::storage_creds_title(self.language)).size(12).color(CustomStyles::accent_cyan()),
                    text(Translations::storage_creds_desc(self.language)).size(10).color(CustomStyles::text_muted()),
                    row![
                        text(Translations::storage_creds_user_label(self.language)).size(11).color(CustomStyles::text_secondary()),
                        user_in,
                        text(Translations::storage_creds_pass_label(self.language)).size(11).color(CustomStyles::text_secondary()),
                        pass_in,
                        text(Translations::storage_creds_group_label(self.language)).size(11).color(CustomStyles::text_secondary()),
                        wg_in,
                        button(text(Translations::storage_creds_save_btn(self.language)).size(11))
                            .style(|_, _| CustomStyles::primary_button())
                            .padding(Padding::from([4, 10]))
                            .on_press(Message::StorageSaveCredentials),
                        button(text(Translations::cancel(self.language)).size(11))
                            .style(|_, _| CustomStyles::secondary_button())
                            .padding(Padding::from([4, 10]))
                            .on_press(Message::ToggleStorageCredentialsModal),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                ]
                .spacing(6),
            )
            .padding(10)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

            modal_body = modal_body.push(creds_card);
        }

        // Capacity & Disk Space Bar (clean update without redundant badges)
        if let Some(disk) = self.storage_disk_info {
            let used_str = format_file_size(disk.used_bytes());
            let total_str = format_file_size(disk.total_bytes);
            let free_str = format_file_size(disk.free_bytes);
            let percent = disk.used_percent();

            let info_row = row![
                text(Translations::storage_space_info(&free_str, &total_str, percent as f64, self.language))
                    .size(11)
                    .color(CustomStyles::accent_cyan()),
            ]
            .spacing(8)
            .align_y(Alignment::Center);

            let capacity_card = container(
                row![
                    info_row,
                    container(progress_bar(0.0..=100.0, percent))
                        .width(Length::Fill),
                    text(format!("{}/{}", used_str, total_str)).size(10).color(CustomStyles::text_muted()),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            )
            .padding(Padding::from([6, 12]))
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

            modal_body = modal_body.push(capacity_card);
        }

        // Share / Resource Selector
        if !self.storage_shares.is_empty() {
            let mut share_chips = row![
                text(Translations::storage_share_select(self.language)).size(11).color(CustomStyles::text_muted()),
            ]
            .spacing(6)
            .align_y(Alignment::Center);

            for share in &self.storage_shares {
                let is_active = self.storage_active_share.as_deref() == Some(&share.name);
                let share_name = share.name.clone();
                let label = if share.comment.is_empty() {
                    format!("📁 {}", share.name)
                } else {
                    format!("📁 {} ({})", share.name, share.comment)
                };

                let btn = button(text(label).size(10))
                    .style(move |_, _| {
                        if is_active {
                            CustomStyles::primary_button()
                        } else {
                            CustomStyles::secondary_button()
                        }
                    })
                    .padding(Padding::from([3, 8]))
                    .on_press(Message::StorageShareSelected(share_name));

                share_chips = share_chips.push(btn);
            }

            modal_body = modal_body.push(share_chips);
        }

        // Navigation & Toolbar (Breadcrumbs, Upload, New Folder, Upload File, Filter)
        let is_root = self.storage_current_path.trim_matches('/').is_empty();
        let up_btn = self.tip(
            button(text(Translations::storage_up_level(self.language)).size(10))
                .style(move |_, _| {
                    if is_root {
                        CustomStyles::secondary_button()
                    } else {
                        CustomStyles::primary_button()
                    }
                })
                .padding(Padding::from([3, 8]))
                .on_press(if is_root {
                    Message::CloseStorageExplorer // No-op when at root
                } else {
                    Message::StorageNavigateUp
                }),
            Translations::tip_storage_up(self.language),
            tooltip::Position::Bottom,
        );

        // Generate Breadcrumbs
        let root_btn = self.tip(
            button(text(Translations::storage_btn_root(self.language)).size(11))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([2, 6]))
                .on_press(Message::StorageNavigateTo(String::new())),
            Translations::tip_storage_root(self.language),
            tooltip::Position::Bottom,
        );

        let mut breadcrumbs = row![
            up_btn,
            root_btn,
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        if !is_root {
            let parts: Vec<&str> = self.storage_current_path.trim_matches('/').split('/').collect();
            let mut accum = String::new();
            for part in parts {
                if !accum.is_empty() {
                    accum.push('/');
                }
                accum.push_str(part);
                let path_target = accum.clone();

                breadcrumbs = breadcrumbs
                    .push(text("/").size(11).color(CustomStyles::text_muted()))
                    .push(
                        button(text(part).size(11))
                            .style(|_, _| CustomStyles::secondary_button())
                            .padding(Padding::from([2, 6]))
                            .on_press(Message::StorageNavigateTo(path_target)),
                    );
            }
        }

        let new_folder_btn = self.tip(
            button(text(Translations::storage_new_folder(self.language)).size(10))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([3, 8]))
                .on_press(Message::StorageStartCreateFolder),
            Translations::tip_storage_new_folder(self.language),
            tooltip::Position::Bottom,
        );

        let upload_file_btn = self.tip(
            button(text(Translations::storage_upload_btn(self.language)).size(10))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([3, 8]))
                .on_press(Message::StoragePickFileAndUpload),
            Translations::tip_storage_upload_file(self.language),
            tooltip::Position::Bottom,
        );

        let upload_folder_btn = self.tip(
            button(text(Translations::storage_upload_folder_btn(self.language)).size(10))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([3, 8]))
                .on_press(Message::StoragePickFolderAndUpload),
            Translations::tip_storage_upload_folder(self.language),
            tooltip::Position::Bottom,
        );

        let filter_in = text_input(Translations::storage_filter_placeholder(self.language), &self.storage_filter_query)
            .on_input(Message::StorageFilterQueryChanged)
            .padding(Padding::from([3, 8]))
            .width(180);

        let toolbar = row![
            breadcrumbs,
            space::horizontal(),
            new_folder_btn,
            upload_file_btn,
            upload_folder_btn,
            filter_in,
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        modal_body = modal_body.push(toolbar);

        // Upload Progress Bar & Card (File or Folder)
        if let Some(upload) = &self.storage_upload_progress {
            let percent = upload.percent.clamp(0.0, 100.0);
            let percent_str = format!("{:.1}%", percent);
            let icon = if upload.is_folder { "📁" } else { "📄" };

            let title_color = if upload.is_finished {
                if upload.error.is_some() {
                    Color::from_rgb8(239, 68, 68)
                } else {
                    CustomStyles::accent_emerald()
                }
            } else {
                CustomStyles::accent_cyan()
            };

            let title_row = row![
                text(format!("{} {} | {}", icon, Translations::storage_upload_progress_title(self.language), upload.title))
                    .size(12)
                    .color(title_color)
                    .width(Length::Fill),
                if !upload.is_finished {
                    text(format!("⚡ {}: {}", Translations::storage_upload_progress_speed(self.language), upload.speed_str))
                        .size(11)
                        .color(CustomStyles::accent_emerald())
                } else if upload.error.is_some() {
                    text(Translations::storage_upload_error(self.language))
                        .size(11)
                        .color(Color::from_rgb8(239, 68, 68))
                } else {
                    text(Translations::storage_upload_completed(self.language))
                        .size(11)
                        .color(CustomStyles::accent_emerald())
                },
                button(text(Translations::storage_upload_dismiss_btn(self.language)).size(10))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([2, 6]))
                    .on_press(Message::StorageDismissUploadProgress),
            ]
            .spacing(10)
            .align_y(Alignment::Center);

            let pbar = progress_bar(0.0..=100.0, percent);

            let uploaded_str = format_file_size(upload.bytes_uploaded);
            let total_str = format_file_size(upload.total_bytes);

            let metrics_row = if upload.is_folder {
                row![
                    text(Translations::storage_upload_folder_progress(
                        upload.current_index, upload.total_items, &upload.current_item, self.language
                    ))
                    .size(10)
                    .color(CustomStyles::text_primary())
                    .width(Length::Fill),
                    text(format!("{} / {} ({})", uploaded_str, total_str, percent_str))
                        .size(10)
                        .color(CustomStyles::accent_cyan()),
                ]
                .align_y(Alignment::Center)
            } else {
                row![
                    text(Translations::storage_upload_file_progress(&upload.current_item, self.language))
                        .size(10)
                        .color(CustomStyles::text_primary())
                        .width(Length::Fill),
                    text(format!("{} / {} ({})", uploaded_str, total_str, percent_str))
                        .size(10)
                        .color(CustomStyles::accent_cyan()),
                ]
                .align_y(Alignment::Center)
            };

            let dest_share = self.storage_active_share.as_deref().unwrap_or("");
            let dest_dir = if upload.target_dir.is_empty() { Translations::storage_root_label(self.language) } else { &upload.target_dir };
            let dest_row = row![
                text(format!("{} /{}/{}", Translations::storage_upload_progress_target(self.language), dest_share, dest_dir))
                    .size(10)
                    .color(CustomStyles::text_muted()),
            ];

            let card = container(
                column![
                    title_row,
                    container(pbar).width(Length::Fill),
                    metrics_row,
                    dest_row,
                ]
                .spacing(6),
            )
            .padding(10)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_upload());

            modal_body = modal_body.push(card);
        }

        // Inline form for creating new folder
        if self.is_creating_folder {
            let folder_input = text_input(Translations::storage_new_folder_placeholder(self.language), &self.new_folder_name_input)
                .on_input(Message::StorageCreateFolderNameChanged)
                .padding(Padding::from([3, 8]))
                .width(220);

            let confirm_btn = button(text(Translations::storage_create_confirm(self.language)).size(10))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([3, 8]))
                .on_press(Message::StorageConfirmCreateFolder);

            let cancel_btn = button(text(Translations::cancel(self.language)).size(10))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([3, 8]))
                .on_press(Message::StorageCancelCreateFolder);

            let create_row = row![
                text(Translations::storage_create_folder_prompt(self.language)).size(11).color(CustomStyles::accent_cyan()),
                folder_input,
                confirm_btn,
                cancel_btn,
            ]
            .spacing(8)
            .align_y(Alignment::Center);

            modal_body = modal_body.push(container(create_row).padding(8).style(|_| CustomStyles::card_container()));
        }

        // Delete Confirmation Dialog
        if let Some((_, _is_dir, name)) = &self.file_to_delete {
            let msg = format!("{}: '{}'?", Translations::storage_confirm_delete_msg(self.language), name);
            let confirm_del_card = container(
                row![
                    text("⚠️").size(16),
                    column![
                        text(Translations::storage_confirm_delete_title(self.language)).size(12).color(Color::from_rgb8(239, 68, 68)),
                        text(msg).size(11).color(CustomStyles::text_primary()),
                    ].width(Length::Fill),
                    button(text(Translations::cancel(self.language)).size(10))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([4, 10]))
                        .on_press(Message::StorageCancelDeleteFile),
                    button(text(Translations::storage_action_delete(self.language)).size(10))
                        .style(|_, _| CustomStyles::danger_button())
                        .padding(Padding::from([4, 10]))
                        .on_press(Message::StorageConfirmDeleteFile),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            )
            .padding(10)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_danger());

            modal_body = modal_body.push(confirm_del_card);
        }

        // File Properties & Metadata Modal / Card
        if let Some(f) = &self.storage_selected_file {
            let full_path = if self.storage_current_path.is_empty() {
                f.name.clone()
            } else {
                format!("{}/{}", self.storage_current_path.trim_matches('/'), f.name)
            };

            let mut actions_row = row![].spacing(8).align_y(Alignment::Center);
            if !f.is_dir {
                actions_row = actions_row.push(
                    button(text(format!("👁 {}", Translations::storage_action_open(self.language))).size(10))
                        .style(|_, _| CustomStyles::primary_button())
                        .padding(Padding::from([3, 10]))
                        .on_press(Message::StorageOpenFile(f.name.clone())),
                );
                actions_row = actions_row.push(
                    button(text(format!("💾 {}", Translations::storage_action_download(self.language))).size(10))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([3, 10]))
                        .on_press(Message::StorageDownloadFile(f.name.clone())),
                );
            } else {
                actions_row = actions_row.push(
                    button(text(format!("📁 {}", Translations::storage_action_open(self.language))).size(10))
                        .style(|_, _| CustomStyles::primary_button())
                        .padding(Padding::from([3, 10]))
                        .on_press(Message::StorageNavigateTo(full_path.clone())),
                );
            }

            let props_card = container(
                column![
                    row![
                        text(format!("{} {}", f.file_type.icon(), Translations::storage_props_title(&f.name, self.language))).size(12).color(CustomStyles::accent_cyan()),
                        space::horizontal(),
                        self.tip(
                            button(text("✕").size(10))
                                .style(|_, _| CustomStyles::secondary_button())
                                .padding(Padding::from([2, 6]))
                                .on_press(Message::StorageCloseProperties),
                            Translations::tip_close(self.language),
                            tooltip::Position::Left,
                        ),
                    ].align_y(Alignment::Center),
                    row![
                        text(Translations::storage_prop_type(self.language)).size(11).color(CustomStyles::text_muted()).width(100),
                        text(f.file_type.type_name(self.language)).size(11).color(CustomStyles::text_primary()),
                    ],
                    row![
                        text(Translations::storage_prop_full_path(self.language)).size(11).color(CustomStyles::text_muted()).width(100),
                        text(full_path.clone()).size(11).color(CustomStyles::accent_cyan()),
                    ],
                    row![
                        text(Translations::storage_prop_size(self.language)).size(11).color(CustomStyles::text_muted()).width(100),
                        text(format!("{} ({} bytes)", f.formatted_size(), f.size_bytes)).size(11).color(CustomStyles::text_primary()),
                    ],
                    row![
                        text(Translations::storage_prop_modified(self.language)).size(11).color(CustomStyles::text_muted()).width(100),
                        text(&f.modified_str).size(11).color(CustomStyles::text_primary()),
                    ],
                    row![
                        text(Translations::storage_prop_attributes(self.language)).size(11).color(CustomStyles::text_muted()).width(100),
                        text(&f.attributes).size(11).color(CustomStyles::text_primary()),
                    ],
                    actions_row,
                ]
                .spacing(4),
            )
            .padding(10)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

            modal_body = modal_body.push(props_card);
        }

        // Status or error message
        if let Some(err) = &self.storage_error_msg {
            let err_card = container(
                row![
                    text("⚠️").size(14),
                    text(err).size(11).color(Color::from_rgb8(239, 68, 68)).width(Length::Fill),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .padding(8)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_danger());

            modal_body = modal_body.push(err_card);
        }

        if let Some(status) = &self.storage_status_msg {
            let status_card = container(
                row![
                    text("✓").size(14).color(CustomStyles::accent_emerald()),
                    text(status).size(11).color(CustomStyles::accent_emerald()).width(Length::Fill),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .padding(8)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

            modal_body = modal_body.push(status_card);
        }

        // File table header with strictly aligned widths
        let table_header = container(
            row![
                container(text(Translations::storage_col_type(self.language)).size(11).color(CustomStyles::text_muted())).width(45).align_x(Alignment::Center),
                container(text(Translations::storage_col_name(self.language)).size(11).color(CustomStyles::text_muted())).width(Length::Fill).align_x(Alignment::Start),
                container(text(Translations::storage_col_size(self.language)).size(11).color(CustomStyles::text_muted())).width(100).align_x(Alignment::Center),
                container(text(Translations::storage_col_modified(self.language)).size(11).color(CustomStyles::text_muted())).width(160).align_x(Alignment::Center),
                container(text(Translations::storage_col_actions(self.language)).size(11).color(CustomStyles::text_muted())).width(230).align_x(Alignment::Center),
            ]
            .align_y(Alignment::Center)
            .padding(Padding::from([4, 8])),
        )
        .width(Length::Fill)
        .style(|_| CustomStyles::card_container());

        modal_body = modal_body.push(table_header);

        // Files / folders content
        let query = self.storage_filter_query.trim().to_lowercase();
        let filtered_files: Vec<&StorageFileEntry> = self
            .storage_files
            .iter()
            .filter(|f| query.is_empty() || f.name.to_lowercase().contains(&query))
            .collect();

        let mut files_column = column![].spacing(3);

        if filtered_files.is_empty() {
            let empty_text = if self.is_storage_loading {
                Translations::storage_loading_files(self.language)
            } else if !query.is_empty() {
                Translations::storage_no_filter_match(self.language)
            } else {
                Translations::storage_empty_dir(self.language)
            };

            files_column = files_column.push(
                container(
                    text(empty_text)
                        .size(12)
                        .color(CustomStyles::text_muted()),
                )
                .padding(24)
                .align_x(Alignment::Center)
                .width(Length::Fill),
            );
        } else {
            for file in filtered_files {
                let name_clone = file.name.clone();
                let is_dir = file.is_dir;
                let target_subpath = if self.storage_current_path.is_empty() {
                    name_clone.clone()
                } else {
                    format!("{}/{}", self.storage_current_path.trim_matches('/'), name_clone)
                };

                let max_name_len = 38;
                let char_count = file.name.chars().count();
                let display_name = if char_count > max_name_len {
                    let truncated: String = file.name.chars().take(max_name_len.saturating_sub(3)).collect();
                    format!("{}...", truncated)
                } else {
                    file.name.clone()
                };

                let name_button = if is_dir {
                    button(text(display_name).size(12).color(CustomStyles::accent_cyan()))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([2, 6]))
                        .on_press(Message::StorageNavigateTo(target_subpath.clone()))
                } else {
                    button(text(display_name).size(12).color(CustomStyles::text_primary()))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([2, 6]))
                        .on_press(Message::StorageOpenFile(name_clone.clone()))
                };

                let name_element: Element<'_, Message> = self.tip(
                    name_button,
                    &file.name,
                    tooltip::Position::Top,
                );

                // Centered row action buttons
                let mut row_actions = row![].spacing(4).align_y(Alignment::Center);

                if is_dir {
                    let open_btn = button(text(Translations::storage_action_open(self.language)).size(10))
                        .style(|_, _| CustomStyles::primary_button())
                        .padding(Padding::from([2, 6]))
                        .on_press(Message::StorageNavigateTo(target_subpath.clone()));
                    row_actions = row_actions.push(
                        self.tip(open_btn, Translations::tip_storage_open_item(self.language), tooltip::Position::Top)
                    );
                } else {
                    let open_name = file.name.clone();
                    let dl_name = file.name.clone();
                    let open_btn = button(text(Translations::storage_action_open(self.language)).size(10))
                        .style(|_, _| CustomStyles::primary_button())
                        .padding(Padding::from([2, 6]))
                        .on_press(Message::StorageOpenFile(open_name));
                    row_actions = row_actions.push(
                        self.tip(open_btn, Translations::tip_storage_open_item(self.language), tooltip::Position::Top)
                    );

                    let dl_btn = button(text(Translations::storage_action_download(self.language)).size(10))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([2, 6]))
                        .on_press(Message::StorageDownloadFile(dl_name));
                    row_actions = row_actions.push(
                        self.tip(dl_btn, Translations::tip_storage_download_item(self.language), tooltip::Position::Top)
                    );
                }

                let props_entry = file.clone();
                let details_btn = button(text(Translations::storage_action_details(self.language)).size(10))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([2, 6]))
                    .on_press(Message::StorageShowFileProperties(props_entry));
                row_actions = row_actions.push(
                    self.tip(details_btn, Translations::tip_storage_details_item(self.language), tooltip::Position::Top)
                );

                let del_name = file.name.clone();
                let del_path = target_subpath.clone();
                let del_btn = button(text("🗑").size(10))
                    .style(|_, _| CustomStyles::danger_button())
                    .padding(Padding::from([2, 6]))
                    .on_press(Message::StorageRequestDeleteFile {
                        path: del_path,
                        is_dir,
                        name: del_name,
                    });
                row_actions = row_actions.push(
                    self.tip(del_btn, Translations::tip_storage_delete_item(self.language), tooltip::Position::Top)
                );

                let file_row = container(
                    row![
                        container(text(file.file_type.icon()).size(14)).width(45).align_x(Alignment::Center),
                        container(name_element).width(Length::Fill).align_x(Alignment::Start),
                        container(text(file.formatted_size()).size(11).color(CustomStyles::text_muted())).width(100).align_x(Alignment::Center),
                        container(text(&file.modified_str).size(10).color(CustomStyles::text_muted())).width(160).align_x(Alignment::Center),
                        container(row_actions).width(230).align_x(Alignment::Center),
                    ]
                    .align_y(Alignment::Center)
                    .padding(Padding::from([4, 8])),
                )
                .width(Length::Fill)
                .style(|_| CustomStyles::card_container());

                files_column = files_column.push(file_row);
            }
        }

        let scrollable_files = scrollable(files_column)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(6.0)
                    .scroller_width(6.0)
                    .margin(2.0)
                    .spacing(8.0),
            ))
            .style(CustomStyles::scrollbar_style())
            .height(Length::Fixed(360.0));

        modal_body = modal_body.push(scrollable_files);

        // Bottom bar: item summary and close button
        let total_count = self.storage_files.len();
        let dirs_count = self.storage_files.iter().filter(|f| f.is_dir).count();
        let regular_count = total_count - dirs_count;
        let summary_text = Translations::storage_summary_counts(total_count, dirs_count, regular_count, self.language);

        let footer = row![
            text(summary_text).size(11).color(CustomStyles::text_muted()),
            space::horizontal(),
            self.tip(
                button(text(Translations::close(self.language)).size(12))
                    .style(|_, _| CustomStyles::primary_button())
                    .padding(Padding::from([6, 18]))
                    .on_press(Message::CloseStorageExplorer),
                Translations::tip_close(self.language),
                tooltip::Position::Top,
            ),
        ]
        .align_y(Alignment::Center);

        modal_body = modal_body.push(footer);

        let content = column![
            title_row,
            subtitle,
            modal_body,
        ]
        .spacing(12);

        container(content)
            .padding(20)
            .width(1080)
            .max_height(780)
            .style(|_| CustomStyles::modal_panel(CustomStyles::accent_cyan()))
            .into()
    }
}
