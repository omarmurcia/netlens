use crate::export::Exporter;
use crate::i18n::{Language, Translations};
use crate::model::{Device, DeviceCategory, NetworkContext, PortInfo};
use crate::scanner::{PingStats, ScanEvent, Scanner};
use crate::telemetry::Telemetry;
use crate::theme::CustomStyles;
use iced::futures::channel::mpsc;
use iced::widget::{
    button, center, column, container, pick_list, progress_bar, row, scrollable, space, text,
    text_input, Column,
};
use iced::{Alignment, Color, Element, Length, Padding, Subscription, Task, Theme};
use mac_oui::Oui;
use std::fmt;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::Duration;

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
            SortOption::Name => write!(f, "📝 Nombre"),
            SortOption::Vendor => write!(f, "🏭 Fabricante"),
            SortOption::Category => write!(f, "🏷️ Tipo"),
        }
    }
}

pub struct App {
    pub network: NetworkContext,
    pub devices: Vec<Device>,
    pub oui_db: Option<Arc<Oui>>,
    pub telemetry: Telemetry,
    pub is_scanning: bool,
    pub scan_progress: f32,
    pub status_text: String,
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
}

#[derive(Debug, Clone)]
pub enum Message {
    // Escaneo
    StartScan,
    ScanEventReceived(ScanEvent),
    RefreshNetwork,

    // Temporizador periódico (1 segundo para telemetría de velocidad y señal)
    Tick,

    // Ajustes / Configuración
    OpenSettings,
    CloseSettings,
    LanguageChanged(Language),
    ThemeChanged(Theme),

    // Filtros y Búsqueda
    SearchChanged(String),
    CategorySelected(Option<DeviceCategory>),
    SortChanged(SortOption),

    // Inspector de Dispositivo
    SelectDevice(Ipv4Addr),
    CloseInspector,
    StartPingTest(Ipv4Addr),
    PingTestFinished(Result<PingStats, String>),
    StartDeepScan(Ipv4Addr),
    DeepScanFinished(Vec<PortInfo>),
    SendWol(String),
    WolFinished(Result<(), String>),
    OpenInBrowser(Ipv4Addr),
    ChangeDeviceCategory(Ipv4Addr, DeviceCategory),

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

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let network = Scanner::detect_network_context();
        let oui_db = Oui::default().ok().map(Arc::new);
        let telemetry = Telemetry::new(&network.interface);
        let language = Language::Spanish;

        let initial_status = if let Some(gw) = network.gateway_ip {
            format!(
                "Conectado a {} mediante {} (Subred {})",
                gw, network.interface, network.subnet_cidr
            )
        } else {
            "Red detectada. Listo para iniciar escaneo.".to_string()
        };

        let app = Self {
            network,
            devices: Vec::new(),
            oui_db,
            telemetry,
            is_scanning: false,
            scan_progress: 0.0,
            status_text: initial_status,
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
            current_theme: Theme::TokyoNight,
            language,
            is_settings_open: false,
        };

        // Iniciar un escaneo automático al arrancar
        (app, Task::done(Message::StartScan))
    }

    /// Suscripción periódica de 1 segundo para medir velocidad de subida/bajada y señal
    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::run(|| {
            iced::futures::stream::unfold(
                tokio::time::interval(Duration::from_millis(1000)),
                |mut interval| async move {
                    interval.tick().await;
                    Some((Message::Tick, interval))
                },
            )
        })
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.telemetry.update_sample(&self.network.interface);
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
                self.notification = Some((
                    format!("Idioma cambiado a {}", lang.native_name()),
                    true,
                ));
                Task::none()
            }

            Message::ThemeChanged(theme) => {
                self.current_theme = theme;
                Task::none()
            }

            Message::ChangeDeviceCategory(ip, new_category) => {
                if let Some(dev) = self.devices.iter_mut().find(|d| d.ip == ip) {
                    dev.category = new_category;
                }
                self.notification = Some((
                    format!(
                        "Tipo de dispositivo actualizado a: {}",
                        Translations::category_name(new_category, self.language)
                    ),
                    true,
                ));
                Task::none()
            }

            Message::RefreshNetwork => {
                self.network = Scanner::detect_network_context();
                self.telemetry.update_sample(&self.network.interface);
                self.status_text = format!(
                    "Contexto de red actualizado: {} ({})",
                    self.network.interface, self.network.subnet_cidr
                );
                Task::none()
            }

            Message::StartScan => {
                if self.is_scanning {
                    return Task::none();
                }

                self.is_scanning = true;
                self.scan_progress = 0.0;
                self.status_text = format!("Escaneando subred {}...", self.network.subnet_cidr);
                self.notification = None;

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
                    self.status_text =
                        format!("Sondeando {} hosts en {}...", total_hosts, subnet);
                    Task::none()
                }
                ScanEvent::Progress { scanned, total } => {
                    self.scan_progress = scanned as f32 / total as f32;
                    Task::none()
                }
                ScanEvent::HostFound(device) => {
                    // Si ya existe la IP, actualizar datos conservando alias y categoría personalizada previos
                    if let Some(existing) = self.devices.iter_mut().find(|d| d.ip == device.ip) {
                        let prev_alias = existing.custom_alias.clone();
                        let prev_category = existing.category;
                        *existing = device;
                        if existing.custom_alias.is_none() {
                            existing.custom_alias = prev_alias;
                        }
                        if prev_category != DeviceCategory::Unknown {
                            existing.category = prev_category;
                        }
                    } else {
                        self.devices.push(device);
                    }
                    Task::none()
                }
                ScanEvent::Completed {
                    total_found,
                    duration_ms,
                } => {
                    self.is_scanning = false;
                    self.scan_progress = 1.0;
                    self.status_text = format!(
                        "Escaneo finalizado: {} dispositivos detectados en {:.2} s",
                        total_found,
                        duration_ms as f32 / 1000.0
                    );
                    Task::none()
                }
                ScanEvent::Error(err) => {
                    self.is_scanning = false;
                    self.status_text = format!("Error durante el escaneo: {}", err);
                    self.notification = Some((err, false));
                    Task::none()
                }
            },

            Message::SearchChanged(query) => {
                self.search_query = query;
                Task::none()
            }

            Message::CategorySelected(cat) => {
                self.selected_category = cat;
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
                        // Actualizar la latencia en el dispositivo
                        if let Some(dev) = self.devices.iter_mut().find(|d| d.ip == stats.ip) {
                            dev.latency_ms = Some(stats.avg_ms);
                        }
                        self.inspector_ping_stats = Some(stats);
                    }
                    Err(err) => {
                        self.notification = Some((format!("Error de ping: {}", err), false));
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
                        self.notification = Some((
                            "Paquete mágico Wake-on-LAN enviado con éxito.".to_string(),
                            true,
                        ));
                    }
                    Err(err) => {
                        self.notification = Some((format!("Fallo al enviar WoL: {}", err), false));
                    }
                }
                Task::none()
            }

            Message::OpenInBrowser(ip) => {
                let url = format!("http://{}", ip);
                let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
                self.notification = Some((format!("Abriendo {} en el navegador...", url), true));
                Task::none()
            }

            Message::AliasInputChanged(input) => {
                self.alias_input = input;
                Task::none()
            }

            Message::SaveAlias(ip) => {
                let trimmed = self.alias_input.trim().to_string();
                if let Some(dev) = self.devices.iter_mut().find(|d| d.ip == ip) {
                    dev.custom_alias = if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed.clone())
                    };
                }
                self.notification =
                    Some((format!("Alias guardado para {}.", ip), true));
                Task::none()
            }

            Message::CopyToClipboard(text_to_copy, label) => {
                self.notification = Some((format!("{} copiado al portapapeles.", label), true));
                iced::clipboard::write(text_to_copy)
            }

            Message::ExportJson => match Exporter::export_json(&self.devices, &self.network) {
                Ok(path) => {
                    self.notification = Some((
                        format!("Exportado con éxito a {:?}", path.file_name().unwrap_or_default()),
                        true,
                    ));
                    Task::none()
                }
                Err(err) => {
                    self.notification = Some((err, false));
                    Task::none()
                }
            },

            Message::ExportCsv => match Exporter::export_csv(&self.devices) {
                Ok(path) => {
                    self.notification = Some((
                        format!("Exportado CSV a {:?}", path.file_name().unwrap_or_default()),
                        true,
                    ));
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

    pub fn view(&self) -> Element<'_, Message> {
        let mut content = Column::new().spacing(14).padding(18);

        // 1. Cabecera principal (Título, botón de Ajustes, botones de escaneo y exportación)
        content = content.push(self.view_header());

        // Barra de progreso si está escaneando
        if self.is_scanning {
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

        // 2. Dashboard de Telemetría en Vivo (Velocidad de bajada, subida, señal Wi-Fi y red)
        content = content.push(self.view_telemetry_dashboard());

        // 3. Barra de filtros, categorías, buscador y ordenación
        content = content.push(self.view_controls());

        // 4. Área de contenido principal: lista de dispositivos + inspector lateral (si está abierto)
        let filtered_devices = self.get_filtered_devices();
        let device_list = self.view_device_list(&filtered_devices);

        let main_area: Element<'_, Message> = if let Some(selected_ip) = self.selected_device_ip {
            if let Some(selected_device) = self.devices.iter().find(|d| d.ip == selected_ip) {
                row![
                    container(device_list).width(Length::FillPortion(3)),
                    space::horizontal().width(14),
                    container(self.view_inspector(selected_device))
                        .width(Length::FillPortion(2))
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

    /// Cabecera de la aplicación
    fn view_header(&self) -> Element<'_, Message> {
        let title_col = column![
            row![
                text("⚡ NetLens").size(24).color(CustomStyles::accent_cyan()),
                text(format!(" | {}", Translations::app_subtitle(self.language))).size(15).color(CustomStyles::text_secondary()),
            ]
            .align_y(Alignment::Center)
            .spacing(8),
            row![
                text(&self.status_text)
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

        // Botón Escanear Red
        let scan_btn = if self.is_scanning {
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
                text("Personaliza los colores y el estilo visual de NetLens").size(11).color(CustomStyles::text_muted()),
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
                text("Soporte para los 10 idiomas con mayor número de hablantes del mundo").size(11).color(CustomStyles::text_muted()),
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

        // Resumen técnico del sistema
        let info_col = column![
            text("Información de Conectividad del Sistema").size(13).color(CustomStyles::accent_cyan()),
            row![
                text("Interfaz Activa:").size(12).color(CustomStyles::text_muted()).width(130),
                text(&self.network.interface).size(12).color(CustomStyles::text_primary()),
            ],
            row![
                text("Subred Local:").size(12).color(CustomStyles::text_muted()).width(130),
                text(&self.network.subnet_cidr).size(12).color(CustomStyles::text_primary()),
            ],
            row![
                text("Puerta de Enlace:").size(12).color(CustomStyles::text_muted()).width(130),
                text(self.network.gateway_ip.map(|g| g.to_string()).unwrap_or_else(|| "No detectado".to_string())).size(12).color(CustomStyles::text_primary()),
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
            container(info_col).padding(12).style(|_| CustomStyles::card_container()),
            row![space::horizontal(), close_btn],
        ]
        .spacing(18);

        container(content)
            .padding(22)
            .width(560)
            .style(|_| CustomStyles::settings_panel())
            .into()
    }

    /// Dashboard superior de telemetría en vivo: Velocidad de bajada, subida, señal Wi-Fi y router
    fn view_telemetry_dashboard(&self) -> Element<'_, Message> {
        // Card 1: Velocidad de Descarga (Download)
        let rx_speed_str = Telemetry::format_speed(self.telemetry.download_speed_bps);
        let rx_session_str = Telemetry::format_bytes(self.telemetry.session_download_bytes());

        let card_download = container(
            column![
                row![
                    text(Translations::download(self.language)).size(11).color(CustomStyles::accent_emerald()),
                    space::horizontal(),
                    text(Translations::live(self.language)).size(9).color(CustomStyles::text_muted()),
                ]
                .align_y(Alignment::Center),
                text(rx_speed_str).size(22).color(CustomStyles::accent_emerald()),
                text(format!("{}: {}", Translations::session(self.language), rx_session_str)).size(11).color(CustomStyles::text_secondary()),
            ]
            .spacing(3),
        )
        .padding(12)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_download());

        // Card 2: Velocidad de Subida (Upload)
        let tx_speed_str = Telemetry::format_speed(self.telemetry.upload_speed_bps);
        let tx_session_str = Telemetry::format_bytes(self.telemetry.session_upload_bytes());

        let card_upload = container(
            column![
                row![
                    text(Translations::upload(self.language)).size(11).color(CustomStyles::accent_purple()),
                    space::horizontal(),
                    text(Translations::live(self.language)).size(9).color(CustomStyles::text_muted()),
                ]
                .align_y(Alignment::Center),
                text(tx_speed_str).size(22).color(CustomStyles::accent_purple()),
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
                wifi.signal_label()
            );
            (Translations::signal_wifi(self.language), big, sub)
        } else if let Some(eth) = &self.telemetry.ethernet {
            let big = eth.display_summary();
            let sub = format!("Interfaz {} (Cable)", eth.interface);
            (Translations::ethernet_cable(self.language), big, sub)
        } else {
            (
                "🌐 CONEXIÓN",
                format!("Interfaz {}", self.network.interface),
                "Enlace activo".to_string(),
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
            .unwrap_or_else(|| "No detectado".to_string());

        let gw_device = self.devices.iter().find(|d| d.is_gateway);
        let gw_name = gw_device
            .map(|d| d.display_name())
            .unwrap_or_else(|| "Puerta de Enlace".to_string());

        let card_gateway = container(
            column![
                row![
                    text("🌐 RED Y ROUTER").size(11).color(CustomStyles::accent_cyan()),
                    space::horizontal(),
                    text(&self.network.subnet_cidr).size(10).color(CustomStyles::text_muted()),
                ]
                .align_y(Alignment::Center),
                row![
                    text(format!("{} Equipos", total_devices)).size(18).color(CustomStyles::text_primary()),
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
        .width(Length::FillPortion(2));

        // Botones de categoría tipo "pills"
        let mut category_pills = row![].spacing(6).align_y(Alignment::Center);

        // Botón "Todos"
        let is_all_active = self.selected_category.is_none();
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
            let cat_name = Translations::category_name(cat, self.language)
                .split('/')
                .next()
                .unwrap_or("");
            let label = format!("{} {}", cat.icon(), cat_name);
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

        // Selector de ordenación
        let sort_picker = pick_list(
            SortOption::ALL,
            Some(self.sort_by),
            Message::SortChanged,
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
            let empty_text = if self.is_scanning {
                "Escaneando la red en segundo plano... Los dispositivos aparecerán aquí pronto."
            } else if !self.search_query.is_empty() || self.selected_category.is_some() {
                "No hay dispositivos que coincidan con los filtros seleccionados."
            } else {
                "No se han detectado dispositivos aún. Haz clic en 'Escanear Red'."
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

    /// Tarjeta individual para cada dispositivo
    fn view_device_card<'a>(&'a self, device: &'a Device) -> Element<'a, Message> {
        let is_selected = self.selected_device_ip == Some(device.ip);

        // Icono y categoría
        let icon_text = text(device.category.icon()).size(26);

        // Nombre y subtítulo
        let name_text = text(device.display_name())
            .size(15)
            .color(if device.is_gateway || device.is_current_host {
                CustomStyles::accent_cyan()
            } else {
                CustomStyles::text_primary()
            });

        let mut tags_row = row![].spacing(6).align_y(Alignment::Center);

        // Badge de Puerta de enlace o Host Local
        if device.is_gateway {
            tags_row = tags_row.push(
                container(text("ROUTER").size(9).color(Color::WHITE))
                    .padding(Padding::from([2, 5]))
                    .style(|_| CustomStyles::badge_pill(CustomStyles::accent_cyan(), Color::WHITE)),
            );
        }
        if device.is_current_host {
            tags_row = tags_row.push(
                container(text("ESTE EQUIPO").size(9).color(Color::WHITE))
                    .padding(Padding::from([2, 5]))
                    .style(|_| CustomStyles::badge_pill(CustomStyles::accent_purple(), Color::WHITE)),
            );
        }

        // Badge de Tipo / Categoría asignada
        let cat_label = Translations::category_name(device.category, self.language)
            .split('/')
            .next()
            .unwrap_or("");
        tags_row = tags_row.push(
            container(text(cat_label).size(10).color(CustomStyles::accent_cyan()))
                .padding(Padding::from([2, 5]))
                .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 35, 55), CustomStyles::accent_cyan())),
        );

        // Badge de Fabricante
        let vendor_label = if !device.vendor.is_empty() && device.vendor != "Desconocido" {
            device.vendor.clone()
        } else if device.is_randomized_mac {
            "MAC Privada".to_string()
        } else {
            "Genérico".to_string()
        };

        tags_row = tags_row.push(
            container(text(vendor_label).size(10).color(CustomStyles::text_secondary()))
                .padding(Padding::from([2, 5]))
                .style(|_| CustomStyles::badge_pill(Color::from_rgb8(51, 65, 85), CustomStyles::text_secondary())),
        );

        // Badge de MAC aleatoria
        if device.is_randomized_mac {
            tags_row = tags_row.push(
                container(text("🛡️ Privacidad").size(9).color(CustomStyles::accent_amber()))
                    .padding(Padding::from([2, 5]))
                    .style(|_| CustomStyles::badge_pill(Color::from_rgb8(69, 45, 12), CustomStyles::accent_amber())),
            );
        }

        // Columna de información de red (IP + MAC)
        let network_col = column![
            row![
                text("IP: ").size(11).color(CustomStyles::text_muted()),
                text(device.ip.to_string()).size(12).color(CustomStyles::text_primary()),
                button(text("📋").size(10))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([1, 3]))
                    .on_press(Message::CopyToClipboard(device.ip.to_string(), "Dirección IP".to_string())),
            ]
            .align_y(Alignment::Center)
            .spacing(4),
            row![
                text("MAC: ").size(11).color(CustomStyles::text_muted()),
                text(&device.mac).size(11).color(CustomStyles::text_secondary()),
                button(text("📋").size(10))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([1, 3]))
                    .on_press(Message::CopyToClipboard(device.mac.clone(), "Dirección MAC".to_string())),
            ]
            .align_y(Alignment::Center)
            .spacing(4),
        ]
        .spacing(2);

        // Estimación de Calidad de Conexión y Latencia
        let (quality_label, quality_color, lat_str) = if let Some(lat) = device.latency_ms {
            if lat < 1.0 {
                ("Cable (<1ms)", CustomStyles::accent_cyan(), format!("{:.1} ms", lat))
            } else if lat < 20.0 {
                ("Wi-Fi Excelente", CustomStyles::accent_emerald(), format!("{:.1} ms", lat))
            } else if lat < 70.0 {
                ("Wi-Fi Bueno", CustomStyles::accent_emerald(), format!("{:.1} ms", lat))
            } else if lat < 140.0 {
                ("Wi-Fi Moderado", CustomStyles::accent_amber(), format!("{:.1} ms", lat))
            } else {
                ("Señal Débil", Color::from_rgb8(239, 68, 68), format!("{:.1} ms", lat))
            }
        } else {
            ("Sin respuesta ICMP", CustomStyles::text_muted(), "-".to_string())
        };

        let latency_element: Element<'a, Message> = column![
            row![
                text("🟢").size(9),
                text(lat_str).size(12).color(quality_color),
            ]
            .spacing(4)
            .align_y(Alignment::Center),
            text(quality_label).size(10).color(CustomStyles::text_muted()),
        ]
        .spacing(2)
        .into();

        // Servicios / Puertos detectados
        let mut services_row = row![].spacing(4).align_y(Alignment::Center);
        for p in device.open_ports.iter().take(3) {
            services_row = services_row.push(
                container(text(format!("{}: {}", p.port, p.service)).size(10).color(CustomStyles::accent_cyan()))
                    .padding(Padding::from([1, 4]))
                    .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 40, 60), CustomStyles::accent_cyan())),
            );
        }
        if device.open_ports.len() > 3 {
            services_row = services_row.push(
                text(format!("+{}", device.open_ports.len() - 3))
                    .size(9)
                    .color(CustomStyles::text_muted()),
            );
        }

        // Botón Detalles
        let btn_details = button(text("Detalles →").size(12))
            .style(move |_, _| {
                if is_selected {
                    CustomStyles::primary_button()
                } else {
                    CustomStyles::secondary_button()
                }
            })
            .padding(Padding::from([5, 10]))
            .on_press(Message::SelectDevice(device.ip));

        let card_content = row![
            container(icon_text).padding(2),
            column![
                row![name_text, tags_row].spacing(6).align_y(Alignment::Center),
                services_row,
            ]
            .spacing(3)
            .width(Length::FillPortion(2)),
            network_col.width(Length::FillPortion(2)),
            container(latency_element).width(Length::FillPortion(1)),
            btn_details,
        ]
        .spacing(10)
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

    /// Inspector de detalles para el dispositivo seleccionado
    fn view_inspector<'a>(&'a self, device: &'a Device) -> Element<'a, Message> {
        let mut insp = Column::new().spacing(12);

        // Barra superior del inspector (Título + Cerrar)
        let top_bar = row![
            text(format!("{} {}", device.category.icon(), Translations::device_details(self.language))).size(15).color(CustomStyles::accent_cyan()),
            space::horizontal(),
            button(text("✕").size(13))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([2, 6]))
                .on_press(Message::CloseInspector),
        ]
        .align_y(Alignment::Center);

        insp = insp.push(top_bar);

        // Nombre y alias
        insp = insp.push(
            column![
                text(device.display_name()).size(16).color(CustomStyles::text_primary()),
                text(device.subtitle()).size(11).color(CustomStyles::text_muted()),
            ]
            .spacing(1),
        );

        // Selector para indicar/cambiar el tipo de dispositivo
        let category_selector = row![
            text(Translations::device_type_label(self.language))
                .size(11)
                .color(CustomStyles::text_muted())
                .width(110),
            pick_list(
                DeviceCategory::all(),
                Some(device.category),
                move |cat| Message::ChangeDeviceCategory(device.ip, cat),
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

        // Ficha técnica
        let mut specs_col = Column::new().spacing(5);

        specs_col = specs_col.push(Self::spec_line("Dirección IP:", &device.ip.to_string()));
        specs_col = specs_col.push(Self::spec_line("Dirección MAC:", &device.mac));
        specs_col = specs_col.push(Self::spec_line("Fabricante:", &device.vendor));
        specs_col = specs_col.push(Self::spec_line(
            "Hostname DNS:",
            device.hostname.as_deref().unwrap_or("No asignado"),
        ));
        specs_col = specs_col.push(Self::spec_line(
            "Tipo de MAC:",
            if device.is_randomized_mac {
                "🛡️ Privada / Aleatoria (LAA)"
            } else {
                "🌐 OUI Global Registrada"
            },
        ));

        // Calidad estimada de enlace Wi-Fi / Red
        let quality_desc = if let Some(lat) = device.latency_ms {
            if lat < 1.0 {
                "🟢 Cable Directo (<1 ms) - Máxima velocidad"
            } else if lat < 20.0 {
                "🟢 Wi-Fi Excelente (~5-15 ms) - Señal óptima"
            } else if lat < 70.0 {
                "🟡 Wi-Fi Bueno (~30-60 ms) - Conexión estable"
            } else if lat < 140.0 {
                "🟠 Wi-Fi Moderado (~80-120 ms) - Reposo / Lejano"
            } else {
                "🔴 Señal Débil (>150 ms) - Posible pérdida"
            }
        } else {
            "⚪ Sin respuesta a paquetes ICMP (Ping)"
        };

        specs_col = specs_col.push(Self::spec_line("Calidad Enlace:", quality_desc));
        specs_col = specs_col.push(Self::spec_line("Interfaz de Red:", &device.interface));
        specs_col = specs_col.push(Self::spec_line("Estado:", device.status.label()));
        specs_col = specs_col.push(Self::spec_line(
            "Última respuesta:",
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

        // Botón Abrir Web si tiene puertos web
        let has_web = device.open_ports.iter().any(|p| p.port == 80 || p.port == 443 || p.port == 8080);
        if has_web || device.is_gateway {
            quick_actions = quick_actions.push(
                button(text("🌐 Abrir Web").size(11))
                    .style(|_, _| CustomStyles::primary_button())
                    .padding(Padding::from([4, 7]))
                    .on_press(Message::OpenInBrowser(device.ip)),
            );
        }

        // Botón Wake-on-LAN
        if !device.is_current_host {
            quick_actions = quick_actions.push(
                button(text("⏰ WoL").size(11))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([4, 7]))
                    .on_press(Message::SendWol(device.mac.clone())),
            );
        }

        insp = insp.push(quick_actions);

        // Herramienta 1: Test de Ping en tiempo real
        let ping_btn_text = if self.is_pinging {
            "⏳ Midiendo..."
        } else {
            "⚡ Test de Ping"
        };

        let ping_btn = button(text(ping_btn_text).size(11))
            .style(|_, _| CustomStyles::secondary_button())
            .padding(Padding::from([5, 8]))
            .on_press(Message::StartPingTest(device.ip));

        let mut ping_section = column![
            row![
                text("LATENCIA & CONECTIVIDAD").size(11).color(CustomStyles::text_muted()),
                space::horizontal(),
                ping_btn,
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(5);

        if let Some(stats) = &self.inspector_ping_stats {
            ping_section = ping_section.push(
                column![
                    row![
                        text(format!("Enviados: {}", stats.transmitted)).size(11).color(CustomStyles::text_secondary()),
                        text(format!("Recibidos: {}", stats.received)).size(11).color(CustomStyles::accent_emerald()),
                        text(format!("Pérdida: {:.0}%", stats.loss_percent)).size(11).color(if stats.loss_percent > 0.0 { CustomStyles::accent_amber() } else { CustomStyles::accent_emerald() }),
                    ]
                    .spacing(8),
                    row![
                        text(format!("Mín: {:.2} ms", stats.min_ms)).size(11).color(CustomStyles::text_secondary()),
                        text(format!("Promedio: {:.2} ms", stats.avg_ms)).size(11).color(CustomStyles::accent_cyan()),
                        text(format!("Máx: {:.2} ms", stats.max_ms)).size(11).color(CustomStyles::text_secondary()),
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

        // Herramienta 2: Escáner de Puertos Abiertos
        let port_btn_text = if self.is_deep_scanning {
            "⏳ Escaneando 30..."
        } else {
            "🔍 Escanear Puertos"
        };

        let port_btn = button(text(port_btn_text).size(11))
            .style(|_, _| CustomStyles::secondary_button())
            .padding(Padding::from([5, 8]))
            .on_press(Message::StartDeepScan(device.ip));

        let mut ports_section = column![
            row![
                text("SERVICIOS Y PUERTOS").size(11).color(CustomStyles::text_muted()),
                space::horizontal(),
                port_btn,
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(5);

        if self.deep_ports.is_empty() {
            ports_section = ports_section.push(
                text(if self.is_deep_scanning {
                    "Comprobando puertos comunes en el host..."
                } else {
                    "Ningún puerto común detectado como abierto."
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
                        text(format!("({})", p.description)).size(10).color(CustomStyles::text_muted()),
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

        scrollable(insp).height(Length::Fill).into()
    }

    /// Línea de especificación técnica clave-valor
    fn spec_line(key: &'static str, val: &str) -> Element<'static, Message> {
        row![
            text(key).size(11).color(CustomStyles::text_muted()).width(110),
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
                // Filtro por categoría
                if let Some(cat) = self.selected_category {
                    if d.category != cat {
                        return false;
                    }
                }

                // Filtro por búsqueda textual
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

        // Ordenar
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
                list.sort_by_key(|d| d.category.display_name());
            }
        }

        list
    }
}
