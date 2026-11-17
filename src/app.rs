use crate::autostart::AutostartManager;
use crate::export::Exporter;
use crate::i18n::{Language, Translations};
use crate::model::{Device, DeviceCategory, NetworkContext, PortInfo};
use crate::router::{LinksysRouter, RouterConnection};
use crate::scanner::{PingStats, ScanEvent, Scanner};
use crate::speedtest::{SpeedTest, SpeedTestEvent, SpeedTestResult, SpeedTestStage};
use crate::storage::{AppState, DeviceMetadata, Storage};
use crate::telemetry::Telemetry;
use crate::theme::CustomStyles;
use crate::traceroute::{HopInfo, Traceroute};
use crate::tray::{spawn_tray, NetLensTray, TrayAction, TrayState};
use crate::widget::Sparkline;
use iced::futures::channel::mpsc;
use iced::widget::canvas::Canvas;
use iced::widget::{
    button, center, column, container, pick_list, progress_bar, row, scrollable, space, text,
    text_input, Column,
};
use iced::{Alignment, Color, Element, Length, Padding, Size, Subscription, Task, Theme};
use crate::oui::Oui;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::net::Ipv4Addr;
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

/// Filtro de seguridad (Todos, Lista Blanca, Bloqueados)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SecurityFilter {
    #[default]
    All,
    Whitelist,
    Blocked,
}

/// Envoltorio para mostrar opciones de ordenación localizadas dinámicamente
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

/// Envoltorio para mostrar categorías de dispositivos localizadas dinámicamente
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

/// Estado descriptivo de la aplicación para renderizar reactivamente en cualquier idioma
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
    pub notification: Option<(String, bool)>, // (Mensaje, es_exito)
    pub alias_input: String,
    pub current_theme: Theme,
    pub language: Language,
    pub is_settings_open: bool,
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

    // Telemetría con Sparklines en tiempo real
    pub download_history: VecDeque<f32>,
    pub upload_history: VecDeque<f32>,
    pub ping_history: VecDeque<f32>,

    // Auto-Discovery periódico
    pub auto_scan_interval_secs: u32,
    pub last_scan_time: Instant,

    // Detección de Intrusos / Rogue Device Alert
    pub known_macs: HashSet<String>,
    pub alerted_macs: HashSet<String>,
    pub pending_intruder_alert: Option<IntruderAlert>,

    // Topología Linksys Mesh
    pub router_connections: HashMap<String, RouterConnection>,

    // Traceroute Visual
    pub is_tracing_route: bool,
    pub traceroute_hops: Option<Vec<HopInfo>>,
    pub traceroute_error: Option<String>,

    // Segundo Plano & Inicio Automático
    pub autostart_enabled: bool,
    pub minimize_on_close: bool,

    // Ventana y Ciclo de Vida (Daemon)
    pub active_window_id: Option<iced::window::Id>,

    // System Tray (Bandeja del Sistema)
    pub tray_state: Arc<Mutex<TrayState>>,
    pub tray_handle: Arc<Mutex<Option<ksni::Handle<NetLensTray>>>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Escaneo
    StartScan,
    StartBackgroundScan,
    ScanEventReceived(ScanEvent),
    RefreshNetwork,

    // Ventana, System Tray y Segundo Plano
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

    // Temporizador periódico (1 segundo para telemetría de velocidad y señal)
    Tick,

    // Ajustes / Configuración
    OpenSettings,
    CloseSettings,
    LanguageChanged(Language),
    ThemeChanged(Theme),
    AutoScanIntervalChanged(u32),

    // Test de Velocidad de Internet
    OpenSpeedTest,
    CloseSpeedTest,
    StartSpeedTest,
    SpeedTestEventReceived(SpeedTestEvent),
    ClearSpeedTestHistory,

    // Filtros y Búsqueda
    SearchChanged(String),
    CategorySelected(Option<DeviceCategory>),
    SecurityFilterChanged(SecurityFilter),
    SortChanged(SortOption),

    // Inspector de Dispositivo
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

    // Control de Seguridad, Intrusos y Router
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

    // Portapapeles
    CopyToClipboard(String, String),

    // Exportación
    ExportJson,
    ExportCsv,

    // Notificaciones
    DismissNotification,
}

/// Configuración de ventana predeterminada de NetLens (con icono oficial de la app)
pub fn default_window_settings() -> iced::window::Settings {
    let icon = crate::tray::get_window_icon();
    iced::window::Settings {
        size: Size::new(1220.0, 820.0),
        position: iced::window::Position::Centered,
        icon,
        exit_on_close_request: false,
        ..Default::default()
    }
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let network = Scanner::detect_network_context();
        let oui_db = Oui::default().ok().map(Arc::new);
        let telemetry = Telemetry::new(&network.interface);

        // Cargar estado guardado de configuración persistente (idioma, tema, dispositivos, metadata)
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
        let autostart_enabled = saved.autostart || AutostartManager::is_enabled();
        if autostart_enabled {
            AutostartManager::sync_if_needed(true);
        }
        let minimize_on_close = saved.minimize_to_tray_on_close;
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
            is_settings_open: false,
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
            })),
            tray_handle: Arc::new(Mutex::new(None)),
        };

        // Iniciar servicio StatusNotifierItem de System Tray (Bandeja del Sistema)
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

        (app, initial_task)
    }

    /// Punto de arranque para la infraestructura iced::daemon
    pub fn boot() -> (Self, Task<Message>) {
        Self::new()
    }

    /// Sincroniza el estado actual de escaneo y dispositivos con el System Tray
    pub fn sync_tray_state(&self) {
        if let Ok(mut s) = self.tray_state.lock() {
            s.device_count = self.devices.len();
            s.is_scanning = self.is_scanning;
            s.gateway_ip = self.network.gateway_ip.map(|g| g.to_string());
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

    /// Guarda el estado actual de la aplicación de manera persistente en disco
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
        };
        if let Err(e) = Storage::save(&state) {
            eprintln!("Aviso: No se pudo guardar estado persistente: {}", e);
        }
    }

    /// Suscripción periódica de 1 segundo para medir velocidad de subida/bajada, eventos de ventana y System Tray
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

        Subscription::batch(vec![tick_sub, close_req_sub, close_evt_sub, tray_sub])
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.telemetry.update_sample(&self.network.interface);

                // Telemetría con Sparklines: registrar muestras de los últimos 30 segundos
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

                // Auto-Discovery: si el intervalo está activo y transcurrió el tiempo, re-escanear silenciosamente en segundo plano
                if self.auto_scan_interval_secs > 0
                    && !self.is_scanning
                    && self.last_scan_time.elapsed().as_secs() >= self.auto_scan_interval_secs as u64
                {
                    self.last_scan_time = Instant::now();
                    return Task::done(Message::StartBackgroundScan);
                }

                self.sync_tray_state();
                Task::none()
            }

            Message::OpenSettings => {
                self.is_settings_open = true;
                Task::none()
            }

            Message::CloseSettings => {
                self.is_settings_open = false;
                Task::none()
            }

            Message::LanguageChanged(lang) => {
                self.language = lang;
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

            Message::WindowCloseRequested(id) => {
                if self.minimize_on_close {
                    // Cerrar y destruir la ventana completamente para que desaparezca de la barra de tareas
                    // y solo quede el icono de la bandeja del sistema (System Tray real).
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
                    // Si la ventana estaba cerrada al minimizar a segundo plano, abrirla nuevamente de forma limpia
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
                    tokio::spawn(async move {
                        let _ = tokio::process::Command::new("notify-send")
                            .arg("-u")
                            .arg("normal")
                            .arg("-i")
                            .arg("network-wireless")
                            .arg("-a")
                            .arg("NetLens")
                            .arg(title)
                            .arg(body)
                            .spawn();
                    });
                    Task::done(Message::TrustDevice(mac))
                } else if action == "block" {
                    if self.router_password_input.trim().is_empty() {
                        let title = Translations::notif_require_pass_title(self.language);
                        let body = Translations::notif_require_pass_body(self.language);
                        tokio::spawn(async move {
                            let _ = tokio::process::Command::new("notify-send")
                                .arg("-u")
                                .arg("critical")
                                .arg("-i")
                                .arg("dialog-warning")
                                .arg("-a")
                                .arg("NetLens")
                                .arg(title)
                                .arg(body)
                                .spawn();
                        });
                        Task::none()
                    } else {
                        let mac_clone = mac.clone();
                        let title = Translations::notif_block_success_title(self.language);
                        let body = Translations::notif_block_success_body(self.language, &mac_clone);
                        tokio::spawn(async move {
                            let _ = tokio::process::Command::new("notify-send")
                                .arg("-u")
                                .arg("normal")
                                .arg("-i")
                                .arg("network-wireless")
                                .arg("-a")
                                .arg("NetLens")
                                .arg(title)
                                .arg(body)
                                .spawn();
                        });
                        Task::done(Message::ToggleDeviceBlocked(ip, mac))
                    }
                } else {
                    Task::none()
                }
            }

            Message::OpenSpeedTest => {
                self.is_speed_test_open = true;
                self.is_settings_open = false;
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
                    // Si ya había un escaneo en segundo plano corriendo, hacerlo visible inmediatamente
                    self.is_background_scan = false;
                    self.status = AppStatus::ScanningSubnet(self.network.subnet_cidr.clone());
                    return Task::none();
                }

                self.is_scanning = true;
                self.is_background_scan = false; // Escaneo manual: muestra barra de progreso en la UI
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
                self.is_background_scan = true; // Escaneo en segundo plano: NO muestra barra de progreso
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
                    // Reaplicar metadatos guardados si existen para esta MAC
                    let mac_key = device.mac.trim().to_lowercase();
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

                    // Enriquecer con topología del router si ya está disponible en cache
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

                    // Detección de Intrusos / Dispositivos Nuevos Desconocidos
                    if !device.is_current_host && !device.is_gateway && !device.is_whitelisted {
                        if !self.known_macs.contains(&mac_key) {
                            if self.alerted_macs.insert(mac_key.clone()) {
                                let dev_name = device.display_name();
                                let dev_ip = device.ip;
                                let dev_mac = device.mac.clone();
                                let dev_vendor = device.vendor.clone();
                                let alert_title = Translations::intruder_alert_title(self.language);
                                let trust_label = Translations::intruder_btn_trust(self.language);
                                let block_label = Translations::intruder_btn_block(self.language);

                                self.pending_intruder_alert = Some(IntruderAlert {
                                    ip: device.ip,
                                    mac: device.mac.clone(),
                                    vendor: device.vendor.clone(),
                                    hostname: device.hostname.clone(),
                                });

                                intruder_task = Task::future(async move {
                                    let output = tokio::process::Command::new("notify-send")
                                        .arg("-u")
                                        .arg("critical")
                                        .arg("-i")
                                        .arg("network-wireless")
                                        .arg("-a")
                                        .arg("NetLens")
                                        .arg("-t")
                                        .arg("45000")
                                        .arg("-A")
                                        .arg(format!("trust={}", trust_label))
                                        .arg("-A")
                                        .arg(format!("block={}", block_label))
                                        .arg(alert_title)
                                        .arg(format!("{} ({})\nMAC: {}\nVendor: {}", dev_name, dev_ip, dev_mac, dev_vendor))
                                        .output()
                                        .await;

                                    let action = match output {
                                        Ok(out) => String::from_utf8_lossy(&out.stdout).trim().to_string(),
                                        Err(_) => String::new(),
                                    };

                                    Message::DesktopNotificationAction {
                                        ip: dev_ip,
                                        mac: dev_mac,
                                        action,
                                    }
                                });
                            }
                        }
                    }

                    if let Some(existing) = self.devices.iter_mut().find(|d| d.ip == device.ip) {
                        let prev_alias = existing.custom_alias.clone();
                        let prev_category = existing.category;
                        let prev_wl = existing.is_whitelisted;
                        let prev_bl = existing.is_blocked;
                        let prev_lim = existing.speed_limit_kbps;
                        *existing = device;
                        if existing.custom_alias.is_none() {
                            existing.custom_alias = prev_alias;
                        }
                        if prev_category != DeviceCategory::Unknown && existing.category == DeviceCategory::Unknown {
                            existing.category = prev_category;
                        }
                        if !self.device_metadata.contains_key(&mac_key) {
                            if prev_wl {
                                existing.is_whitelisted = true;
                            }
                            if prev_bl {
                                existing.is_blocked = true;
                            }
                        }
                        if existing.speed_limit_kbps.is_none() {
                            existing.speed_limit_kbps = prev_lim;
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

                    if !self.router_password_input.trim().is_empty() {
                        Task::done(Message::FetchRouterTopology)
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
                let notif_msg = match self.language {
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
                };
                self.notification = Some((notif_msg, true));
                iced::clipboard::write(text_to_copy)
            }

            Message::ExportJson => match Exporter::export_json(&self.devices, &self.network) {
                Ok(path) => {
                    let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("JSON");
                    let notif_msg = match self.language {
                        Language::Spanish => format!("Exportado con éxito a {}", fname),
                        Language::English => format!("Successfully exported to {}", fname),
                        Language::Chinese => format!("已成功导出至 {}", fname),
                        Language::Hindi => format!("सफलतापूर्वक निर्यात किया गया: {}", fname),
                        Language::Arabic => format!("تم التصدير بنجاح إلى {}", fname),
                        Language::French => format!("Exporté avec succès vers {}", fname),
                        Language::Bengali => format!("সফলভাবে রপ্তানি হয়েছে: {}", fname),
                        Language::Portuguese => format!("Exportado com sucesso para {}", fname),
                        Language::Russian => format!("Успешно экспортировано в {}", fname),
                        Language::Urdu => format!("کامیابی سے برآمد کر دیا گیا: {}", fname),
                    };
                    self.notification = Some((notif_msg, true));
                    Task::none()
                }
                Err(err) => {
                    self.notification = Some((err, false));
                    Task::none()
                }
            },

            Message::ExportCsv => match Exporter::export_csv(&self.devices) {
                Ok(path) => {
                    let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("CSV");
                    let notif_msg = match self.language {
                        Language::Spanish => format!("Exportado CSV a {}", fname),
                        Language::English => format!("CSV exported to {}", fname),
                        Language::Chinese => format!("CSV 文件已导出至 {}", fname),
                        Language::Hindi => format!("सीएसवी निर्यात किया गया: {}", fname),
                        Language::Arabic => format!("تم تصدير CSV إلى {}", fname),
                        Language::French => format!("CSV exporté vers {}", fname),
                        Language::Bengali => format!("CSV রপ্তানি হয়েছে: {}", fname),
                        Language::Portuguese => format!("CSV exportado para {}", fname),
                        Language::Russian => format!("CSV экспортирован в {}", fname),
                        Language::Urdu => format!("CSV برآمد کر دیا گیا: {}", fname),
                    };
                    self.notification = Some((notif_msg, true));
                    Task::none()
                }
                Err(err) => {
                    self.notification = Some((err, false));
                    Task::none()
                }
            },

            Message::DismissNotification => {
                self.notification = None;
                Task::none()
            }
        }
    }

    pub fn view(&self, _window: iced::window::Id) -> Element<'_, Message> {
        let mut content = Column::new().spacing(14).padding(16);

        // 1. Cabecera principal (Título, botón de Ajustes, botones de escaneo y exportación)
        content = content.push(self.view_header());

        // Barra de progreso de escaneo: SOLO se muestra si es escaneo manual iniciado por el botón
        if self.is_scanning && !self.is_background_scan {
            content = content.push(
                container(progress_bar(0.0..=1.0, self.scan_progress))
            );
        }

        // Notificación flotante si está activa
        if let Some((msg, is_success)) = &self.notification {
            content = content.push(self.view_notification(msg, *is_success));
        }

        // Si los ajustes están abiertos, mostrar la vista de configuración
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

        // Si el test de velocidad está abierto, mostrar la vista modal de velocidad
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

        // 2. Dashboard de Telemetría en Vivo (Velocidad de bajada, subida, señal Wi-Fi y red)
        content = content.push(self.view_telemetry_dashboard());

        // Banner de Alerta de Intrusos (Rogue Device) si se detecta un dispositivo desconocido
        if let Some(alert) = &self.pending_intruder_alert {
            content = content.push(self.view_intruder_alert(alert));
        }

        // 3. Barra de filtros, categorías, buscador y ordenación
        content = content.push(self.view_controls());

        // 4. Área de contenido principal: lista de dispositivos + inspector lateral (si está abierto)
        let filtered_devices = self.get_filtered_devices();
        let device_list = self.view_device_list(&filtered_devices);

        let main_area: Element<'_, Message> = if let Some(selected_ip) = self.selected_device_ip {
            if let Some(selected_device) = self.devices.iter().find(|d| d.ip == selected_ip) {
                row![
                    container(device_list).width(Length::Fill),
                    space::horizontal().width(12),
                    container(self.view_inspector(selected_device))
                        .width(Length::Fixed(390.0))
                        .style(|_| CustomStyles::inspector_panel())
                        .padding(14),
                ]
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

    /// Banner destacado de advertencia cuando se detecta un nuevo dispositivo no reconocido en la red local
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

    /// Cabecera de la aplicación
    fn view_header(&self) -> Element<'_, Message> {
        let title_col = column![
            row![
                text("⚡ NetLens").size(22).color(CustomStyles::accent_cyan()),
                text(format!(" | {}", Translations::app_subtitle(self.language))).size(14).color(CustomStyles::text_secondary()),
            ]
            .align_y(Alignment::Center)
            .spacing(8),
            row![
                text(self.status.render(self.language))
                    .size(12)
                    .color(CustomStyles::text_muted()),
                button(text("🔄").size(11))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([1, 5]))
                    .on_press(Message::RefreshNetwork),
            ]
            .align_y(Alignment::Center)
            .spacing(6),
        ]
        .spacing(2);

        let mut actions = row![].spacing(8).align_y(Alignment::Center);

        // Botón de Ajustes (Settings)
        actions = actions.push(
            button(text(Translations::settings(self.language)).size(12))
                .style(|_, _| {
                    if self.is_settings_open {
                        CustomStyles::primary_button()
                    } else {
                        CustomStyles::secondary_button()
                    }
                })
                .padding(Padding::from([6, 12]))
                .on_press(if self.is_settings_open {
                    Message::CloseSettings
                } else {
                    Message::OpenSettings
                }),
        );

        // Botón Test de Velocidad (Speed Test)
        actions = actions.push(
            button(text(Translations::speed_test_btn(self.language)).size(12))
                .style(|_, _| {
                    if self.is_speed_test_open {
                        CustomStyles::primary_button()
                    } else {
                        CustomStyles::secondary_button()
                    }
                })
                .padding(Padding::from([6, 11]))
                .on_press(if self.is_speed_test_open {
                    Message::CloseSpeedTest
                } else {
                    Message::OpenSpeedTest
                }),
        );

        // Botón Exportar CSV
        actions = actions.push(
            button(text("📊 CSV").size(12))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([6, 10]))
                .on_press(Message::ExportCsv),
        );

        // Botón Exportar JSON
        actions = actions.push(
            button(text("📥 JSON").size(12))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([6, 10]))
                .on_press(Message::ExportJson),
        );

        // Botón Escanear Red: Solo muestra estado de "Escaneando..." si es un escaneo manual visible
        let scan_btn = if self.is_scanning && !self.is_background_scan {
            button(text(Translations::scanning(self.language)).size(13))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([7, 14]))
        } else {
            button(text(Translations::scan_network(self.language)).size(13))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([7, 14]))
                .on_press(Message::StartScan)
        };

        actions = actions.push(scan_btn);

        row![
            title_col,
            space::horizontal(),
            actions,
        ]
        .align_y(Alignment::Center)
        .into()
    }

    /// Panel de Ajustes / Configuración (Tema de UI e Idioma entre los 10 más hablados del mundo)
    fn view_settings_panel(&self) -> Element<'_, Message> {
        let title_row = row![
            text(Translations::settings(self.language))
                .size(20)
                .color(CustomStyles::accent_cyan()),
            space::horizontal(),
            button(text("✕").size(14))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([2, 8]))
                .on_press(Message::CloseSettings),
        ]
        .align_y(Alignment::Center);

        // Ajuste 1: Tema del UI
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

        // Ajuste 2: Idioma de la aplicación (Top 10 idiomas más hablados del mundo)
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

        // Ajuste 3: Escaneo Automático Periódico (Auto-Discovery)
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

        // Ajuste 4: Segundo Plano & Inicio Automático con el Sistema
        let autostart_btn = button(
            text(if self.autostart_enabled { "✓ Activado" } else { "Desactivado" }).size(11)
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
            text(if self.minimize_on_close { "✓ Activado" } else { "Desactivado" }).size(11)
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

        // Ajuste 5: Control de Router (Linksys MR73 Mesh / JNAP)
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

        // Resumen técnico del sistema
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

        let close_btn = button(text(Translations::close(self.language)).size(13))
            .style(|_, _| CustomStyles::primary_button())
            .padding(Padding::from([7, 20]))
            .on_press(Message::CloseSettings);

        let content = column![
            title_row,
            theme_row,
            lang_row,
            auto_scan_card,
            background_card,
            router_card,
            container(info_col).padding(12).style(|_| CustomStyles::card_container()),
            row![space::horizontal(), close_btn],
        ]
        .spacing(14);

        container(content)
            .padding(22)
            .width(560)
            .style(|_| CustomStyles::settings_panel())
            .into()
    }

    /// Modal de Test de Velocidad de Internet (Ping, Descarga y Subida en tiempo real)
    fn view_speed_test_modal(&self) -> Element<'_, Message> {
        let is_running = self.speed_test_stage == SpeedTestStage::TestingPing
            || self.speed_test_stage == SpeedTestStage::TestingDownload
            || self.speed_test_stage == SpeedTestStage::TestingUpload;

        let title_row = row![
            text(Translations::speed_test_title(self.language))
                .size(20)
                .color(CustomStyles::accent_cyan()),
            space::horizontal(),
            button(text("✕").size(14))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([2, 8]))
                .on_press(Message::CloseSpeedTest),
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

        // Tarjetas de Métricas (Ping, Descarga, Subida)
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

        // Información de red / servidor
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

        // Botones de acción
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

        // Sección: Historial Persistente de Pruebas de Velocidad
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
            .padding(22)
            .width(620)
            .style(|_| CustomStyles::settings_panel())
            .into()
    }

    /// Dashboard superior de telemetría en vivo: Velocidad de bajada, subida, señal Wi-Fi y router
    fn view_telemetry_dashboard(&self) -> Element<'_, Message> {
        // Card 1: Velocidad de Descarga (Download con Sparkline)
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

        // Card 2: Velocidad de Subida (Upload con Sparkline)
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

        // Card 3: Intensidad y Calidad de Señal Wi-Fi / Conexión de red
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

        // Card 4: Router / Puerta de Enlace y Red Local
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

        let card_gateway = container(
            column![
                row![
                    text(Translations::network_and_router(self.language)).size(11).color(CustomStyles::accent_cyan()),
                    space::horizontal(),
                    text(&self.network.subnet_cidr).size(10).color(CustomStyles::text_muted()),
                ]
                .align_y(Alignment::Center),
                row![
                    text(Translations::devices_count(total_devices, self.language)).size(18).color(CustomStyles::text_primary()),
                    text(format!("(GW: {})", gw_ip_str)).size(12).color(CustomStyles::accent_cyan()),
                ]
                .align_y(Alignment::Center)
                .spacing(6),
                text(gw_name).size(11).color(CustomStyles::text_secondary()),
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

    /// Controles de filtrado, búsqueda y ordenación
    fn view_controls(&self) -> Element<'_, Message> {
        let search_bar = text_input(
            Translations::search_placeholder(self.language),
            &self.search_query,
        )
        .on_input(Message::SearchChanged)
        .padding(Padding::from([7, 10]))
        .width(Length::FillPortion(1));

        // Botones de categoría tipo "pills" compactos y estilizados
        let mut category_pills = row![].spacing(6).align_y(Alignment::Center);

        let is_all_active = self.selected_category.is_none() && self.security_filter == SecurityFilter::All;
        category_pills = category_pills.push(
            button(text(Translations::all_filter(self.language)).size(12))
                .style(move |_, _| {
                    if is_all_active {
                        CustomStyles::filter_pill_active()
                    } else {
                        CustomStyles::filter_pill_inactive()
                    }
                })
                .padding(Padding::from([4, 8]))
                .on_press(Message::CategorySelected(None)),
        );

        for &cat in &[
            DeviceCategory::Router,
            DeviceCategory::Smartphone,
            DeviceCategory::Computer,
            DeviceCategory::SmartTv,
            DeviceCategory::Iot,
        ] {
            let is_active = self.selected_category == Some(cat);
            let label = Translations::category_short_name(cat, self.language);
            category_pills = category_pills.push(
                button(text(label).size(12))
                    .style(move |_, _| {
                        if is_active {
                            CustomStyles::filter_pill_active()
                        } else {
                            CustomStyles::filter_pill_inactive()
                        }
                    })
                    .padding(Padding::from([4, 8]))
                    .on_press(Message::CategorySelected(Some(cat))),
            );
        }

        // Filtros de Seguridad y Listas (Lista Blanca y Bloqueados)
        let whitelist_count = self.devices.iter().filter(|d| d.is_whitelisted).count();
        let blocked_count = self.devices.iter().filter(|d| d.is_blocked).count();

        let is_whitelist_active = self.security_filter == SecurityFilter::Whitelist;
        let is_blocked_active = self.security_filter == SecurityFilter::Blocked;

        if whitelist_count > 0 || is_whitelist_active {
            category_pills = category_pills.push(
                button(text(format!("🛡️ {} ({})", Translations::filter_whitelist(self.language), whitelist_count)).size(11))
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
                    )),
            );
        }

        if blocked_count > 0 || is_blocked_active {
            category_pills = category_pills.push(
                button(text(format!("🚫 {} ({})", Translations::filter_blocked(self.language), blocked_count)).size(11))
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
                    )),
            );
        }

        // Selector de ordenación reactivo localizado
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

        row![
            search_bar,
            category_pills,
            space::horizontal(),
            sort_picker,
        ]
        .spacing(10)
        .align_y(Alignment::Center)
        .into()
    }

    /// Genera la lista scrolleable de tarjetas de dispositivos
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
            top: 0.0,
            right: 4.0,
            bottom: 0.0,
            left: 0.0,
        });

        for device in devices {
            list_col = list_col.push(self.view_device_card(device));
        }

        scrollable(list_col).height(Length::Fill).into()
    }

    /// Tarjeta individual para cada dispositivo con jerarquía visual de 2 niveles (sin desbordamientos ni solapamientos)
    fn view_device_card<'a>(&'a self, device: &'a Device) -> Element<'a, Message> {
        let is_selected = self.selected_device_ip == Some(device.ip);

        // Icono de categoría
        let icon_widget = text(device.category.icon()).size(28);

        // Nombre principal del dispositivo
        let name_widget = text(device.display_name())
            .size(15)
            .color(if device.is_gateway || device.is_current_host {
                CustomStyles::accent_cyan()
            } else {
                CustomStyles::text_primary()
            });

        // Fila 1 de Badges (Roles especiales: ROUTER, ESTE EQUIPO, Privacidad, Seguridad)
        let mut role_tags = row![].spacing(6).align_y(Alignment::Center);

        if device.is_blocked {
            role_tags = role_tags.push(
                container(text(Translations::badge_blocked(self.language)).size(9).color(Color::WHITE))
                    .padding(Padding::from([2, 6]))
                    .style(|_| CustomStyles::badge_pill(Color::from_rgb8(185, 28, 28), Color::WHITE)),
            );
        } else if device.is_whitelisted {
            role_tags = role_tags.push(
                container(text(Translations::badge_whitelisted(self.language)).size(9).color(Color::WHITE))
                    .padding(Padding::from([2, 6]))
                    .style(|_| CustomStyles::badge_pill(Color::from_rgb8(16, 120, 70), Color::WHITE)),
            );
        }

        if device.is_gateway {
            role_tags = role_tags.push(
                container(text(Translations::badge_router(self.language)).size(9).color(Color::WHITE))
                    .padding(Padding::from([2, 6]))
                    .style(|_| CustomStyles::badge_pill(CustomStyles::accent_cyan(), Color::WHITE)),
            );
        }
        if device.is_current_host {
            role_tags = role_tags.push(
                container(text(Translations::badge_local_host(self.language)).size(9).color(Color::WHITE))
                    .padding(Padding::from([2, 6]))
                    .style(|_| CustomStyles::badge_pill(CustomStyles::accent_purple(), Color::WHITE)),
            );
        }
        if device.is_randomized_mac {
            role_tags = role_tags.push(
                container(text(Translations::badge_privacy(self.language)).size(9).color(CustomStyles::accent_amber()))
                    .padding(Padding::from([2, 6]))
                    .style(|_| CustomStyles::badge_pill(Color::from_rgb8(69, 45, 12), CustomStyles::accent_amber())),
            );
        }

        // Fila 2 de Badges: Categoría asignada + Fabricante + Puertos abiertos
        let mut sub_tags = row![].spacing(6).align_y(Alignment::Center);

        // Badge de Tipo / Categoría asignada
        let cat_label = Translations::category_name(device.category, self.language)
            .split('/')
            .next()
            .unwrap_or("")
            .trim();
        sub_tags = sub_tags.push(
            container(text(cat_label).size(10).color(CustomStyles::accent_cyan()))
                .padding(Padding::from([2, 6]))
                .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 35, 55), CustomStyles::accent_cyan())),
        );

        // Badge de Fabricante
        let vendor_str = if !device.vendor.is_empty() && device.vendor != "Desconocido" {
            device.vendor.clone()
        } else if device.is_randomized_mac {
            Translations::vendor_private(self.language).to_string()
        } else {
            Translations::vendor_generic(self.language).to_string()
        };

        let vendor_truncated = if vendor_str.len() > 24 {
            format!("{}...", &vendor_str[..22])
        } else {
            vendor_str
        };

        sub_tags = sub_tags.push(
            container(text(vendor_truncated).size(10).color(CustomStyles::text_secondary()))
                .padding(Padding::from([2, 6]))
                .style(|_| CustomStyles::badge_pill(Color::from_rgb8(40, 50, 70), CustomStyles::text_secondary())),
        );

        // Badge de Topología de Conexión Router (Mesh / Wi-Fi / Ethernet)
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

            sub_tags = sub_tags.push(
                container(text(top_label).size(10).color(top_color))
                    .padding(Padding::from([2, 6]))
                    .style(move |_| CustomStyles::badge_pill(top_bg, top_color)),
            );
        }

        // Servicios / Puertos detectados
        for p in device.open_ports.iter().take(2) {
            sub_tags = sub_tags.push(
                container(text(format!("{}: {}", p.port, p.service)).size(9).color(CustomStyles::accent_cyan()))
                    .padding(Padding::from([1, 4]))
                    .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 40, 60), CustomStyles::accent_cyan())),
            );
        }
        if device.open_ports.len() > 2 {
            sub_tags = sub_tags.push(
                text(format!("+{}", device.open_ports.len() - 2))
                    .size(9)
                    .color(CustomStyles::text_muted()),
            );
        }

        // Columna de identidad del dispositivo (ocupa todo el espacio disponible en la izquierda)
        let identity_col = column![
            row![icon_widget, space::horizontal().width(4), name_widget, role_tags]
                .spacing(6)
                .align_y(Alignment::Center),
            sub_tags,
        ]
        .spacing(4)
        .width(Length::Fill);

        // Columna de red (IP + MAC con ancho fijo de 190 px para que nunca se solape)
        let network_col = column![
            row![
                text("IP: ").size(11).color(CustomStyles::text_muted()),
                text(device.ip.to_string()).size(12).color(CustomStyles::text_primary()),
                button(text("📋").size(10))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([1, 3]))
                    .on_press(Message::CopyToClipboard(device.ip.to_string(), "IP".to_string())),
            ]
            .align_y(Alignment::Center)
            .spacing(4),
            row![
                text("MAC: ").size(11).color(CustomStyles::text_muted()),
                text(&device.mac).size(11).color(CustomStyles::text_secondary()),
                button(text("📋").size(10))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([1, 3]))
                    .on_press(Message::CopyToClipboard(device.mac.clone(), "MAC".to_string())),
            ]
            .align_y(Alignment::Center)
            .spacing(4),
        ]
        .spacing(2)
        .width(Length::Fixed(190.0));

        // Estimación de Calidad de Conexión y Latencia (ancho fijo de 125 px)
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

        let latency_col = column![
            row![
                text("🟢").size(9),
                text(lat_str).size(12).color(quality_color),
            ]
            .spacing(4)
            .align_y(Alignment::Center),
            text(quality_label).size(10).color(CustomStyles::text_muted()),
        ]
        .spacing(2)
        .width(Length::Fixed(125.0));

        // Botón Detalles
        let btn_details = button(text(Translations::details_btn(self.language)).size(12))
            .style(move |_, _| {
                if is_selected {
                    CustomStyles::primary_button()
                } else {
                    CustomStyles::secondary_button()
                }
            })
            .padding(Padding::from([6, 12]))
            .on_press(Message::SelectDevice(device.ip));

        let card_content = row![
            identity_col,
            network_col,
            latency_col,
            btn_details,
        ]
        .spacing(12)
        .align_y(Alignment::Center);

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

    /// Inspector de detalles para el dispositivo seleccionado (panel lateral con ancho fijo y reactividad total)
    fn view_inspector<'a>(&'a self, device: &'a Device) -> Element<'a, Message> {
        let mut insp = Column::new().spacing(12);

        // Barra superior del inspector (Título + Cerrar)
        let top_bar = row![
            text(format!("{} {}", device.category.icon(), Translations::device_details(self.language)))
                .size(15)
                .color(CustomStyles::accent_cyan()),
            space::horizontal(),
            button(text("✕").size(13))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([2, 6]))
                .on_press(Message::CloseInspector),
        ]
        .align_y(Alignment::Center);

        insp = insp.push(top_bar);

        // Nombre y subtítulo traducido
        insp = insp.push(
            column![
                text(device.display_name()).size(16).color(CustomStyles::text_primary()),
                text(Translations::device_subtitle(device, self.language)).size(11).color(CustomStyles::text_muted()),
            ]
            .spacing(1),
        );

        // Selector para indicar/cambiar el tipo de dispositivo (LOCALIZADO!)
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

        insp = insp.push(category_selector);

        // Editor de alias personalizado
        let alias_row = row![
            text_input(Translations::alias_placeholder(self.language), &self.alias_input)
                .on_input(Message::AliasInputChanged)
                .padding(Padding::from([5, 8]))
                .width(Length::Fill),
            button(text(Translations::save(self.language)).size(11))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([5, 8]))
                .on_press(Message::SaveAlias(device.ip)),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        insp = insp.push(alias_row);

        // Ficha técnica (LOCALIZADA EN SU TOTALIDAD)
        let mut specs_col = Column::new().spacing(5);

        specs_col = specs_col.push(Self::spec_line(Translations::spec_ip(self.language), &device.ip.to_string()));
        specs_col = specs_col.push(Self::spec_line(Translations::spec_mac(self.language), &device.mac));
        specs_col = specs_col.push(Self::spec_line(Translations::spec_vendor(self.language), &device.vendor));
        specs_col = specs_col.push(Self::spec_line(
            Translations::spec_hostname(self.language),
            device.hostname.as_deref().unwrap_or(Translations::spec_not_assigned(self.language)),
        ));
        specs_col = specs_col.push(Self::spec_line(
            Translations::spec_mac_type(self.language),
            if device.is_randomized_mac {
                Translations::mac_type_private(self.language)
            } else {
                Translations::mac_type_global(self.language)
            },
        ));

        // Calidad estimada de enlace Wi-Fi / Red
        specs_col = specs_col.push(Self::spec_line(
            Translations::spec_link_quality(self.language),
            Translations::link_quality_desc(device.latency_ms, self.language),
        ));

        // Información de topología Linksys Mesh si está disponible
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

        insp = insp.push(
            container(specs_col)
                .padding(8)
                .style(|_| CustomStyles::card_container()),
        );

        // Acciones rápidas (Copiar, Abrir en navegador, WoL)
        let mut quick_actions = row![].spacing(6).align_y(Alignment::Center);

        quick_actions = quick_actions.push(
            button(text("📋 IP").size(11))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([4, 7]))
                .on_press(Message::CopyToClipboard(device.ip.to_string(), "IP".to_string())),
        );

        quick_actions = quick_actions.push(
            button(text("📋 MAC").size(11))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([4, 7]))
                .on_press(Message::CopyToClipboard(device.mac.clone(), "MAC".to_string())),
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

        if !device.is_current_host {
            quick_actions = quick_actions.push(
                button(text("⏰ WoL").size(11))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([4, 7]))
                    .on_press(Message::SendWol(device.mac.clone())),
            );
        }

        insp = insp.push(quick_actions);

        // Acciones de Seguridad y Control de Router (Lista Blanca y Bloqueo Permanente)
        let mut security_actions = row![].spacing(6).align_y(Alignment::Center);

        let (wl_label, wl_style) = if device.is_whitelisted {
            (Translations::btn_remove_whitelist(self.language), CustomStyles::secondary_button())
        } else {
            (Translations::btn_add_whitelist(self.language), CustomStyles::primary_button())
        };
        security_actions = security_actions.push(
            button(text(wl_label).size(11))
                .style(move |_, _| wl_style.clone())
                .padding(Padding::from([4, 7]))
                .on_press(Message::ToggleDeviceWhitelist(device.ip, device.mac.clone())),
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

            security_actions = security_actions.push(block_btn);
        }

        insp = insp.push(security_actions);

        // Herramienta 1: Test de Ping en tiempo real (LOCALIZADA!)
        let ping_btn_text = if self.is_pinging {
            Translations::btn_measuring(self.language)
        } else {
            Translations::btn_ping_test(self.language)
        };

        let ping_btn = button(text(ping_btn_text).size(11))
            .style(|_, _| CustomStyles::secondary_button())
            .padding(Padding::from([5, 8]))
            .on_press(Message::StartPingTest(device.ip));

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

        insp = insp.push(
            container(ping_section)
                .padding(8)
                .style(|_| CustomStyles::card_container()),
        );

        // Herramienta 2: Escáner de Puertos (LOCALIZADA!)
        let port_btn_text = if self.is_deep_scanning {
            Translations::btn_scanning_ports(self.language)
        } else {
            Translations::btn_scan_ports(self.language)
        };

        let port_btn = button(text(port_btn_text).size(11))
            .style(|_, _| CustomStyles::secondary_button())
            .padding(Padding::from([5, 8]))
            .on_press(Message::StartDeepScan(device.ip));

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
            let mut port_list = Column::new().spacing(4);
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
            ports_section = ports_section.push(scrollable(port_list).height(110));
        }

        insp = insp.push(
            container(ports_section)
                .padding(8)
                .style(|_| CustomStyles::card_container()),
        );

        // Herramienta 3: Diagnóstico de Ruta / Traceroute Visual
        let trace_btn_text = if self.is_tracing_route {
            Translations::traceroute_btn_tracing(self.language)
        } else {
            Translations::traceroute_btn_start(self.language)
        };

        let trace_btn = button(text(trace_btn_text).size(11))
            .style(|_, _| CustomStyles::secondary_button())
            .padding(Padding::from([5, 8]))
            .on_press(Message::StartTraceroute(device.ip));

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
                text(Translations::traceroute_ip_col(self.language)).size(10).color(CustomStyles::text_muted()).width(200),
                space::horizontal(),
                text(Translations::traceroute_latency_col(self.language)).size(10).color(CustomStyles::text_muted()),
            ]
            .spacing(6);
            trace_section = trace_section.push(hops_header);

            let mut hops_col = Column::new().spacing(4);
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
                        text(ip_str).size(10).color(CustomStyles::text_primary()).width(200),
                        space::horizontal(),
                        text(latency_str).size(10).color(color),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                );
            }
            trace_section = trace_section.push(scrollable(hops_col).height(120));
        }

        insp = insp.push(
            container(trace_section)
                .padding(8)
                .style(|_| CustomStyles::card_container()),
        );

        scrollable(insp).height(Length::Fill).into()
    }

    /// Línea de especificación técnica clave-valor
    fn spec_line(key: &'static str, val: &str) -> Element<'static, Message> {
        row![
            text(key).size(11).color(CustomStyles::text_muted()).width(120),
            text(val.to_string()).size(11).color(CustomStyles::text_primary()),
        ]
        .spacing(4)
        .align_y(Alignment::Center)
        .into()
    }

    /// Notificación flotante de resultado
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

    /// Filtra y ordena los dispositivos según las preferencias actuales del usuario
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
}
