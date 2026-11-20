use crate::i18n::{Language, Translations};
use ksni::{menu::*, Category, Handle, Icon, MenuItem, Status, ToolTip, Tray, TrayMethods};
use std::sync::{Arc, Mutex, OnceLock};
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum TrayAction {
    RestoreWindow,
    TriggerScan,
    Exit,
}

static TRAY_BROADCAST: OnceLock<broadcast::Sender<TrayAction>> = OnceLock::new();

pub fn get_tray_sender() -> &'static broadcast::Sender<TrayAction> {
    TRAY_BROADCAST.get_or_init(|| {
        let (tx, _rx) = broadcast::channel(32);
        tx
    })
}

pub fn send_tray_action(action: TrayAction) {
    let _ = get_tray_sender().send(action);
}

/// Extracts and converts NetLens icon to ARGB32 format for System Tray (64x64 to minimize D-Bus payload)
pub fn get_icon_argb() -> Option<Icon> {
    let png_bytes = include_bytes!("../assets/netlens.png");
    let img = image::load_from_memory_with_format(png_bytes, image::ImageFormat::Png).ok()?;
    let resized = img.resize_exact(64, 64, image::imageops::FilterType::Lanczos3);
    let rgba = resized.into_rgba8();
    let width = rgba.width() as i32;
    let height = rgba.height() as i32;
    let mut data = rgba.into_raw();
    for pixel in data.chunks_exact_mut(4) {
        pixel.rotate_right(1); // RGBA to ARGB
    }
    Some(Icon { width, height, data })
}

/// Extracts NetLens icon in standard RGBA format for window and taskbar
pub fn get_window_icon() -> Option<iced::window::Icon> {
    let png_bytes = include_bytes!("../assets/netlens.png");
    let img = image::load_from_memory_with_format(png_bytes, image::ImageFormat::Png).ok()?;
    let rgba = img.into_rgba8();
    let (width, height) = (rgba.width(), rgba.height());
    iced::window::icon::from_rgba(rgba.into_raw(), width, height).ok()
}

#[derive(Debug, Clone, Default)]
pub struct TrayState {
    pub device_count: usize,
    pub is_scanning: bool,
    pub gateway_ip: Option<String>,
    pub language: Language,
}

pub struct NetLensTray {
    pub state: Arc<Mutex<TrayState>>,
    pub icon_cache: Option<Icon>,
}

impl Tray for NetLensTray {
    fn id(&self) -> String {
        "netlens".to_string()
    }

    fn category(&self) -> Category {
        Category::ApplicationStatus
    }

    fn title(&self) -> String {
        "NetLens".to_string()
    }

    fn status(&self) -> Status {
        Status::Active
    }

    fn icon_name(&self) -> String {
        "netlens".to_string()
    }

    fn icon_pixmap(&self) -> Vec<Icon> {
        if let Some(icon) = &self.icon_cache {
            vec![icon.clone()]
        } else {
            Vec::new()
        }
    }

    fn tool_tip(&self) -> ToolTip {
        let (count, scanning, gw, lang) = {
            let s = self.state.lock().unwrap();
            (s.device_count, s.is_scanning, s.gateway_ip.clone(), s.language)
        };
        let desc = if scanning {
            Translations::tray_tooltip_scanning(lang).to_string()
        } else if let Some(gw_ip) = gw {
            Translations::tray_tooltip_with_gw(count, &gw_ip, lang)
        } else {
            Translations::tray_tooltip_devices(count, lang)
        };

        ToolTip {
            icon_name: "netlens".to_string(),
            icon_pixmap: self.icon_pixmap(),
            title: Translations::tray_tooltip_title(lang).to_string(),
            description: desc,
        }
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        send_tray_action(TrayAction::RestoreWindow);
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        let (count, is_scanning, lang) = {
            let s = self.state.lock().unwrap();
            (s.device_count, s.is_scanning, s.language)
        };

        let status_text = if is_scanning {
            Translations::tray_menu_scanning(lang).to_string()
        } else {
            Translations::tray_menu_devices(count, lang)
        };

        vec![
            StandardItem {
                label: Translations::tray_menu_open(lang).into(),
                activate: Box::new(|_| {
                    send_tray_action(TrayAction::RestoreWindow);
                }),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: Translations::tray_menu_scan_now(lang).into(),
                enabled: !is_scanning,
                activate: Box::new(|_| {
                    send_tray_action(TrayAction::TriggerScan);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: status_text,
                enabled: false,
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: Translations::tray_menu_exit(lang).into(),
                activate: Box::new(|_| {
                    send_tray_action(TrayAction::Exit);
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}

/// Registers StatusNotifierItem in system tray (System Tray D-Bus)
pub fn spawn_tray(
    state: Arc<Mutex<TrayState>>,
    handle_holder: Arc<Mutex<Option<Handle<NetLensTray>>>>,
) {
    let icon_cache = get_icon_argb();

    tokio::spawn(async move {
        // At system startup, StatusNotifierWatcher may take a few seconds to initialize.
        // `assume_sni_available(true)` automatically waits for the panel/watcher to become available on D-Bus.
        // In addition, progressive retries are performed in case of session bus delays upon login.
        for attempt in 1..=5 {
            let tray = NetLensTray {
                state: Arc::clone(&state),
                icon_cache: icon_cache.clone(),
            };

            match tray.assume_sni_available(true).spawn().await {
                Ok(handle) => {
                    if let Ok(mut lock) = handle_holder.lock() {
                        *lock = Some(handle);
                    }
                    return;
                }
                Err(e) => {
                    if attempt < 5 {
                        tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                    } else {
                        eprintln!(
                            "Aviso: No se pudo registrar StatusNotifierItem en la bandeja del sistema tras 5 intentos: {}",
                            e
                        );
                    }
                }
            }
        }
    });
}
