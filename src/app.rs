use crate::export::Exporter;
use crate::model::{Device, DeviceCategory, NetworkContext, PortInfo};
use crate::scanner::{PingStats, ScanEvent, Scanner};
use crate::theme::CustomStyles;
use iced::futures::channel::mpsc;
use iced::widget::{
    button, column, container, pick_list, progress_bar, row, scrollable, space, text,
    text_input, Column,
};
use iced::{Alignment, Color, Element, Length, Padding, Task, Theme};
use mac_oui::Oui;
use std::fmt;
use std::net::Ipv4Addr;
use std::sync::Arc;

pub const AVAILABLE_THEMES: &[Theme] = &[
    Theme::TokyoNight,
    Theme::CatppuccinMocha,
    Theme::Nord,
    Theme::Dracula,
    Theme::Dark,
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
            SortOption::Ip => write!(f, "Ordenar: Dirección IP"),
            SortOption::Latency => write!(f, "Ordenar: Latencia (Ping)"),
            SortOption::Name => write!(f, "Ordenar: Nombre de Equipo"),
            SortOption::Vendor => write!(f, "Ordenar: Fabricante"),
            SortOption::Category => write!(f, "Ordenar: Categoría"),
        }
    }
}

pub struct App {
    pub network: NetworkContext,
    pub devices: Vec<Device>,
    pub oui_db: Option<Arc<Oui>>,
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
}

#[derive(Debug, Clone)]
pub enum Message {
    // Escaneo
    StartScan,
    ScanEventReceived(ScanEvent),
    RefreshNetwork,

    // Filtros y Búsqueda
    SearchChanged(String),
    CategorySelected(Option<DeviceCategory>),
    SortChanged(SortOption),
    ThemeChanged(Theme),

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
        };

        // Iniciar un escaneo automático al arrancar
        (app, Task::done(Message::StartScan))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::RefreshNetwork => {
                self.network = Scanner::detect_network_context();
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
                    // Si ya existe la IP, actualizar datos conservando alias previo
                    if let Some(existing) = self.devices.iter_mut().find(|d| d.ip == device.ip) {
                        let prev_alias = existing.custom_alias.clone();
                        *existing = device;
                        if existing.custom_alias.is_none() {
                            existing.custom_alias = prev_alias;
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

            Message::ThemeChanged(theme) => {
                self.current_theme = theme;
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
                Task::perform(
                    async move { Scanner::ping_test_detailed(ip).await },
                    Message::PingTestFinished,
                )
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
                Task::perform(
                    async move { Scanner::deep_port_scan(ip).await },
                    Message::DeepScanFinished,
                )
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
        let mut content = Column::new().spacing(16).padding(20);

        // 1. Cabecera principal (Título, red, botones de escaneo y exportación)
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

        // 2. Tarjetas de métricas y resumen de red
        content = content.push(self.view_metric_cards());

        // 3. Barra de filtros, categorías, buscador y ordenación
        content = content.push(self.view_controls());

        // 4. Área de contenido principal: lista de dispositivos + inspector lateral (si está abierto)
        let filtered_devices = self.get_filtered_devices();

        let device_list = self.view_device_list(&filtered_devices);

        let main_area: Element<'_, Message> = if let Some(selected_ip) = self.selected_device_ip {
            if let Some(selected_device) = self.devices.iter().find(|d| d.ip == selected_ip) {
                row![
                    container(device_list).width(Length::FillPortion(3)),
                    space::horizontal().width(16),
                    container(self.view_inspector(selected_device))
                        .width(Length::FillPortion(2))
                        .style(|_| CustomStyles::inspector_panel())
                        .padding(16),
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
                text("⚡ NetLens").size(26).color(CustomStyles::accent_cyan()),
                text(" | Monitor de Red").size(18).color(CustomStyles::text_secondary()),
            ]
            .align_y(Alignment::Center)
            .spacing(8),
            row![
                text(&self.status_text)
                    .size(13)
                    .color(CustomStyles::text_muted()),
                button(text("🔄").size(11))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([1, 5]))
                    .on_press(Message::RefreshNetwork),
            ]
            .align_y(Alignment::Center)
            .spacing(6),
        ]
        .spacing(4);

        let mut actions = row![].spacing(10).align_y(Alignment::Center);

        // Selector de tema
        actions = actions.push(
            row![
                text("Tema:").size(13).color(CustomStyles::text_muted()),
                pick_list(
                    AVAILABLE_THEMES,
                    Some(&self.current_theme),
                    |t| Message::ThemeChanged(t.clone())
                ),
            ]
            .align_y(Alignment::Center)
            .spacing(6),
        );

        // Botón Exportar CSV
        actions = actions.push(
            button(
                text("📊 Exportar CSV").size(13)
            )
            .style(|_, _| CustomStyles::secondary_button())
            .padding(Padding::from([7, 12]))
            .on_press(Message::ExportCsv),
        );

        // Botón Exportar JSON
        actions = actions.push(
            button(
                text("📥 Exportar JSON").size(13)
            )
            .style(|_, _| CustomStyles::secondary_button())
            .padding(Padding::from([7, 12]))
            .on_press(Message::ExportJson),
        );

        // Botón Escanear Red
        let scan_btn = if self.is_scanning {
            button(text("⏳ Escaneando...").size(14))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([8, 16]))
        } else {
            button(text("⚡ Escanear Red").size(14))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([8, 16]))
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

    /// Tarjetas de métricas superiores
    fn view_metric_cards(&self) -> Element<'_, Message> {
        let total_devices = self.devices.len();
        let total_online = self
            .devices
            .iter()
            .filter(|d| d.status == crate::model::DeviceStatus::Online)
            .count();

        // Card 1: Total de dispositivos
        let card_total = container(
            column![
                text("DISPOSITIVOS ACTIVOS").size(11).color(CustomStyles::text_muted()),
                row![
                    text(format!("{}", total_devices)).size(24).color(CustomStyles::text_primary()),
                    text(format!("({} en línea)", total_online)).size(13).color(CustomStyles::accent_emerald()),
                ]
                .align_y(Alignment::Center)
                .spacing(8),
            ]
            .spacing(4),
        )
        .padding(12)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_container());

        // Card 2: Router / Puerta de Enlace
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
                text("PUERTA DE ENLACE / ROUTER").size(11).color(CustomStyles::text_muted()),
                text(gw_name).size(15).color(CustomStyles::accent_cyan()),
                text(format!("IP: {}", gw_ip_str)).size(12).color(CustomStyles::text_secondary()),
            ]
            .spacing(4),
        )
        .padding(12)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_highlighted());

        // Card 3: Este Equipo
        let local_ip_str = self
            .network
            .local_ip
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| "No detectado".to_string());

        let card_local = container(
            column![
                text("ESTE EQUIPO (HOST)").size(11).color(CustomStyles::text_muted()),
                text(format!("Interfaz: {}", self.network.interface)).size(15).color(CustomStyles::text_primary()),
                text(format!("IP: {}", local_ip_str)).size(12).color(CustomStyles::text_secondary()),
            ]
            .spacing(4),
        )
        .padding(12)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_container());

        // Card 4: Desglose por categorías principales
        let count_phones = self.devices.iter().filter(|d| d.category == DeviceCategory::Smartphone).count();
        let count_computers = self.devices.iter().filter(|d| d.category == DeviceCategory::Computer).count();
        let count_iot = self.devices.iter().filter(|d| d.category == DeviceCategory::Iot).count();
        let count_tv = self.devices.iter().filter(|d| d.category == DeviceCategory::SmartTv).count();

        let card_breakdown = container(
            column![
                text("TIPOS DE DISPOSITIVOS").size(11).color(CustomStyles::text_muted()),
                row![
                    text(format!("📱 {}", count_phones)).size(13).color(CustomStyles::text_primary()),
                    text(format!("💻 {}", count_computers)).size(13).color(CustomStyles::text_primary()),
                    text(format!("📺 {}", count_tv)).size(13).color(CustomStyles::text_primary()),
                    text(format!("🔌 {}", count_iot)).size(13).color(CustomStyles::text_primary()),
                ]
                .spacing(12),
            ]
            .spacing(6),
        )
        .padding(12)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_container());

        row![card_total, card_gateway, card_local, card_breakdown]
            .spacing(12)
            .into()
    }

    /// Controles de filtrado, búsqueda y ordenación
    fn view_controls(&self) -> Element<'_, Message> {
        let search_bar = text_input(
            "🔍 Buscar por IP, nombre, fabricante o MAC...",
            &self.search_query,
        )
        .on_input(Message::SearchChanged)
        .padding(Padding::from([8, 12]))
        .width(Length::FillPortion(2));

        // Botones de categoría tipo "pills"
        let mut category_pills = row![].spacing(6).align_y(Alignment::Center);

        // Botón "Todos"
        let is_all_active = self.selected_category.is_none();
        category_pills = category_pills.push(
            button(text("Todos").size(12))
                .style(move |_, _| {
                    if is_all_active {
                        CustomStyles::filter_pill_active()
                    } else {
                        CustomStyles::filter_pill_inactive()
                    }
                })
                .padding(Padding::from([5, 10]))
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
            let label = format!("{} {}", cat.icon(), cat.display_name().split('/').next().unwrap_or(""));
            category_pills = category_pills.push(
                button(text(label).size(12))
                    .style(move |_, _| {
                        if is_active {
                            CustomStyles::filter_pill_active()
                        } else {
                            CustomStyles::filter_pill_inactive()
                        }
                    })
                    .padding(Padding::from([5, 10]))
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
        .spacing(12)
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

        let mut list_col = Column::new().spacing(10).padding(Padding {
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
        let icon_text = text(device.category.icon()).size(28);

        // Nombre y subtítulo
        let name_text = text(device.display_name())
            .size(16)
            .color(if device.is_gateway || device.is_current_host {
                CustomStyles::accent_cyan()
            } else {
                CustomStyles::text_primary()
            });

        let mut tags_row = row![].spacing(6).align_y(Alignment::Center);

        // Badge de Puerta de enlace o Host Local
        if device.is_gateway {
            tags_row = tags_row.push(
                container(text("ROUTER").size(10).color(Color::WHITE))
                    .padding(Padding::from([2, 6]))
                    .style(|_| CustomStyles::badge_pill(CustomStyles::accent_cyan(), Color::WHITE)),
            );
        }
        if device.is_current_host {
            tags_row = tags_row.push(
                container(text("ESTE EQUIPO").size(10).color(Color::WHITE))
                    .padding(Padding::from([2, 6]))
                    .style(|_| CustomStyles::badge_pill(CustomStyles::accent_purple(), Color::WHITE)),
            );
        }

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
                .padding(Padding::from([2, 6]))
                .style(|_| CustomStyles::badge_pill(Color::from_rgb8(51, 65, 85), CustomStyles::text_secondary())),
        );

        // Badge de MAC aleatoria
        if device.is_randomized_mac {
            tags_row = tags_row.push(
                container(text("🛡️ Privacidad").size(10).color(CustomStyles::accent_amber()))
                    .padding(Padding::from([2, 6]))
                    .style(|_| CustomStyles::badge_pill(Color::from_rgb8(69, 45, 12), CustomStyles::accent_amber())),
            );
        }

        // Columna de información de red (IP + MAC)
        let network_col = column![
            row![
                text("IP: ").size(12).color(CustomStyles::text_muted()),
                text(device.ip.to_string()).size(13).color(CustomStyles::text_primary()),
                button(text("📋").size(11))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([1, 4]))
                    .on_press(Message::CopyToClipboard(device.ip.to_string(), "Dirección IP".to_string())),
            ]
            .align_y(Alignment::Center)
            .spacing(4),
            row![
                text("MAC: ").size(12).color(CustomStyles::text_muted()),
                text(&device.mac).size(12).color(CustomStyles::text_secondary()),
                button(text("📋").size(11))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([1, 4]))
                    .on_press(Message::CopyToClipboard(device.mac.clone(), "Dirección MAC".to_string())),
            ]
            .align_y(Alignment::Center)
            .spacing(4),
        ]
        .spacing(2);

        // Latencia / RTT
        let latency_element: Element<'a, Message> = if let Some(lat) = device.latency_ms {
            let lat_color = if lat < 15.0 {
                CustomStyles::accent_emerald()
            } else if lat < 80.0 {
                CustomStyles::accent_amber()
            } else {
                Color::from_rgb8(239, 68, 68)
            };
            row![
                text("🟢").size(10),
                text(format!("{:.1} ms", lat)).size(13).color(lat_color),
            ]
            .spacing(4)
            .align_y(Alignment::Center)
            .into()
        } else {
            row![
                text("⚪").size(10),
                text("Sin ping").size(12).color(CustomStyles::text_muted()),
            ]
            .spacing(4)
            .align_y(Alignment::Center)
            .into()
        };

        // Servicios / Puertos detectados
        let mut services_row = row![].spacing(4).align_y(Alignment::Center);
        for p in device.open_ports.iter().take(3) {
            services_row = services_row.push(
                container(text(format!("{}: {}", p.port, p.service)).size(10).color(CustomStyles::accent_cyan()))
                    .padding(Padding::from([1, 5]))
                    .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 40, 60), CustomStyles::accent_cyan())),
            );
        }
        if device.open_ports.len() > 3 {
            services_row = services_row.push(
                text(format!("+{}", device.open_ports.len() - 3))
                    .size(10)
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
            .padding(Padding::from([6, 12]))
            .on_press(Message::SelectDevice(device.ip));

        let card_content = row![
            container(icon_text).padding(4),
            column![
                row![name_text, tags_row].spacing(8).align_y(Alignment::Center),
                services_row,
            ]
            .spacing(4)
            .width(Length::FillPortion(2)),
            network_col.width(Length::FillPortion(2)),
            container(latency_element).width(Length::FillPortion(1)),
            btn_details,
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        container(card_content)
            .padding(12)
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
        let mut insp = Column::new().spacing(14);

        // Barra superior del inspector (Título + Cerrar)
        let top_bar = row![
            text(format!("{} Detalles del Dispositivo", device.category.icon())).size(16).color(CustomStyles::accent_cyan()),
            space::horizontal(),
            button(text("✕").size(14))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([2, 8]))
                .on_press(Message::CloseInspector),
        ]
        .align_y(Alignment::Center);

        insp = insp.push(top_bar);

        // Nombre y alias
        insp = insp.push(
            column![
                text(device.display_name()).size(18).color(CustomStyles::text_primary()),
                text(device.subtitle()).size(12).color(CustomStyles::text_muted()),
            ]
            .spacing(2),
        );

        // Editor de alias personalizado
        let alias_row = row![
            text_input("Apodo / Nota personalizada...", &self.alias_input)
                .on_input(Message::AliasInputChanged)
                .padding(Padding::from([5, 8]))
                .width(Length::Fill),
            button(text("Guardar").size(12))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([6, 10]))
                .on_press(Message::SaveAlias(device.ip)),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        insp = insp.push(alias_row);

        // Ficha técnica
        let mut specs_col = Column::new().spacing(6);

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
        specs_col = specs_col.push(Self::spec_line("Interfaz de Red:", &device.interface));
        specs_col = specs_col.push(Self::spec_line("Estado:", device.status.label()));
        specs_col = specs_col.push(Self::spec_line(
            "Última respuesta:",
            &device.last_seen.format("%H:%M:%S").to_string(),
        ));

        insp = insp.push(
            container(specs_col)
                .padding(10)
                .style(|_| CustomStyles::card_container()),
        );

        // Acciones rápidas (Copiar, Abrir en navegador, WoL)
        let mut quick_actions = row![].spacing(6).align_y(Alignment::Center);

        quick_actions = quick_actions.push(
            button(text("📋 Copiar IP").size(11))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([5, 8]))
                .on_press(Message::CopyToClipboard(device.ip.to_string(), "IP".to_string())),
        );

        quick_actions = quick_actions.push(
            button(text("📋 Copiar MAC").size(11))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([5, 8]))
                .on_press(Message::CopyToClipboard(device.mac.clone(), "MAC".to_string())),
        );

        // Botón Abrir Web si tiene puertos web
        let has_web = device.open_ports.iter().any(|p| p.port == 80 || p.port == 443 || p.port == 8080);
        if has_web || device.is_gateway {
            quick_actions = quick_actions.push(
                button(text("🌐 Abrir Web").size(11))
                    .style(|_, _| CustomStyles::primary_button())
                    .padding(Padding::from([5, 8]))
                    .on_press(Message::OpenInBrowser(device.ip)),
            );
        }

        // Botón Wake-on-LAN
        if !device.is_current_host {
            quick_actions = quick_actions.push(
                button(text("⏰ WoL").size(11))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([5, 8]))
                    .on_press(Message::SendWol(device.mac.clone())),
            );
        }

        insp = insp.push(quick_actions);

        // Herramienta 1: Test de Ping en tiempo real
        let ping_btn_text = if self.is_pinging {
            "⏳ Midiendo latencia..."
        } else {
            "⚡ Test de Ping (4 paquetes)"
        };

        let ping_btn = button(text(ping_btn_text).size(12))
            .style(|_, _| CustomStyles::secondary_button())
            .padding(Padding::from([6, 10]))
            .on_press(Message::StartPingTest(device.ip));

        let mut ping_section = column![
            row![
                text("LATENCIA & CONECTIVIDAD").size(11).color(CustomStyles::text_muted()),
                space::horizontal(),
                ping_btn,
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(6);

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
                .spacing(4),
            );
        }

        insp = insp.push(
            container(ping_section)
                .padding(10)
                .style(|_| CustomStyles::card_container()),
        );

        // Herramienta 2: Escáner de Puertos Abiertos
        let port_btn_text = if self.is_deep_scanning {
            "⏳ Escaneando 30 puertos..."
        } else {
            "🔍 Escanear Puertos"
        };

        let port_btn = button(text(port_btn_text).size(12))
            .style(|_, _| CustomStyles::secondary_button())
            .padding(Padding::from([6, 10]))
            .on_press(Message::StartDeepScan(device.ip));

        let mut ports_section = column![
            row![
                text("SERVICIOS Y PUERTOS").size(11).color(CustomStyles::text_muted()),
                space::horizontal(),
                port_btn,
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(6);

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
                        container(text(format!("TCP {}", p.port)).size(10).color(CustomStyles::accent_cyan()))
                            .padding(Padding::from([1, 4]))
                            .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 30, 50), CustomStyles::accent_cyan())),
                        text(&p.service).size(11).color(CustomStyles::text_primary()),
                        text(format!("({})", p.description)).size(10).color(CustomStyles::text_muted()),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                );
            }
            ports_section = ports_section.push(scrollable(port_list).height(120));
        }

        insp = insp.push(
            container(ports_section)
                .padding(10)
                .style(|_| CustomStyles::card_container()),
        );

        scrollable(insp).height(Length::Fill).into()
    }

    /// Línea de especificación técnica clave-valor
    fn spec_line(key: &'static str, val: &str) -> Element<'static, Message> {
        row![
            text(key).size(12).color(CustomStyles::text_muted()).width(110),
            text(val.to_string()).size(12).color(CustomStyles::text_primary()),
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
            text(if is_success { "✓" } else { "⚠" }).size(14),
            text(message).size(13),
            space::horizontal(),
            button(text("✕").size(12))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([2, 6]))
                .on_press(Message::DismissNotification),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        container(notif_row)
            .padding(8)
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
