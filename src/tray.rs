use crate::i18n::{Language, Translations};
#[cfg(not(windows))]
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

#[cfg(not(windows))]
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

#[cfg(not(windows))]
pub struct NetLensTray {
    pub state: Arc<Mutex<TrayState>>,
    pub icon_cache: Option<Icon>,
}

#[cfg(not(windows))]
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

/// Handle used by the app to ask the tray to refresh itself after TrayState changed.
#[cfg(not(windows))]
pub struct TrayHandle(Handle<NetLensTray>);

#[cfg(not(windows))]
impl TrayHandle {
    pub fn refresh(&self) {
        let h = self.0.clone();
        tokio::spawn(async move {
            let _ = h.update(|_| ()).await;
        });
    }
}

/// Registers StatusNotifierItem in system tray (System Tray D-Bus)
#[cfg(not(windows))]
pub fn spawn_tray(
    state: Arc<Mutex<TrayState>>,
    handle_holder: Arc<Mutex<Option<TrayHandle>>>,
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
                        *lock = Some(TrayHandle(handle));
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


// ---------------------------------------------------------------------------
// Windows notification-area (system tray) implementation
// ---------------------------------------------------------------------------

/// Handle used by the app to ask the tray to refresh itself after `TrayState` changed.
#[cfg(windows)]
pub struct TrayHandle(std::sync::mpsc::Sender<()>);

#[cfg(windows)]
impl TrayHandle {
    pub fn refresh(&self) {
        let _ = self.0.send(());
    }
}

/// Registers the NetLens icon in the Windows notification area.
/// The icon lives on a dedicated thread running a Win32 message pump (required by `tray-icon`).
#[cfg(windows)]
pub fn spawn_tray(
    state: Arc<Mutex<TrayState>>,
    handle_holder: Arc<Mutex<Option<TrayHandle>>>,
) {
    use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
    use tray_icon::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        DispatchMessageW, PeekMessageW, TranslateMessage, MSG, PM_REMOVE,
    };

    let (refresh_tx, refresh_rx) = std::sync::mpsc::channel::<()>();
    if let Ok(mut lock) = handle_holder.lock() {
        *lock = Some(TrayHandle(refresh_tx));
    }

    std::thread::spawn(move || {
        let snapshot = |state: &Arc<Mutex<TrayState>>| -> TrayState {
            state.lock().map(|s| s.clone()).unwrap_or_default()
        };
        let describe = |s: &TrayState| -> (String, String) {
            let lang = s.language;
            let tooltip = if s.is_scanning {
                Translations::tray_tooltip_scanning(lang).to_string()
            } else if let Some(gw) = &s.gateway_ip {
                Translations::tray_tooltip_with_gw(s.device_count, gw, lang)
            } else {
                Translations::tray_tooltip_devices(s.device_count, lang)
            };
            let status = if s.is_scanning {
                Translations::tray_menu_scanning(lang).to_string()
            } else {
                Translations::tray_menu_devices(s.device_count, lang)
            };
            (format!("{}\n{}", Translations::tray_tooltip_title(lang), tooltip), status)
        };

        let initial = snapshot(&state);
        let lang = initial.language;
        let (tooltip, status_text) = describe(&initial);

        let open_item = MenuItem::new(Translations::tray_menu_open(lang), true, None);
        let scan_item = MenuItem::new(Translations::tray_menu_scan_now(lang), !initial.is_scanning, None);
        let status_item = MenuItem::new(status_text, false, None);
        let exit_item = MenuItem::new(Translations::tray_menu_exit(lang), true, None);

        let menu = Menu::new();
        let _ = menu.append_items(&[
            &open_item,
            &PredefinedMenuItem::separator(),
            &scan_item,
            &status_item,
            &PredefinedMenuItem::separator(),
            &exit_item,
        ]);

        let icon = {
            let png_bytes = include_bytes!("../assets/netlens.png");
            image::load_from_memory_with_format(png_bytes, image::ImageFormat::Png)
                .ok()
                .and_then(|img| {
                    let rgba = img.resize_exact(32, 32, image::imageops::FilterType::Lanczos3).into_rgba8();
                    let (w, h) = (rgba.width(), rgba.height());
                    tray_icon::Icon::from_rgba(rgba.into_raw(), w, h).ok()
                })
        };

        let mut builder = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_menu_on_left_click(false)
            .with_tooltip(tooltip);
        if let Some(icon) = icon {
            builder = builder.with_icon(icon);
        }
        let tray = match builder.build() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Warning: could not create the Windows tray icon: {}", e);
                return;
            }
        };

        let open_id = open_item.id().clone();
        let scan_id = scan_item.id().clone();
        let exit_id = exit_item.id().clone();

        loop {
            // Pump Win32 messages for this thread (needed for tray icon & popup menu)
            unsafe {
                let mut msg: MSG = std::mem::zeroed();
                while PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }

            while let Ok(event) = MenuEvent::receiver().try_recv() {
                if event.id == open_id {
                    send_tray_action(TrayAction::RestoreWindow);
                } else if event.id == scan_id {
                    send_tray_action(TrayAction::TriggerScan);
                } else if event.id == exit_id {
                    send_tray_action(TrayAction::Exit);
                }
            }

            while let Ok(event) = TrayIconEvent::receiver().try_recv() {
                if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } = event
                {
                    send_tray_action(TrayAction::RestoreWindow);
                }
            }

            let mut needs_refresh = false;
            while refresh_rx.try_recv().is_ok() {
                needs_refresh = true;
            }
            if needs_refresh {
                let s = snapshot(&state);
                let (tooltip, status_text) = describe(&s);
                let _ = tray.set_tooltip(Some(tooltip));
                open_item.set_text(Translations::tray_menu_open(s.language));
                scan_item.set_text(Translations::tray_menu_scan_now(s.language));
                scan_item.set_enabled(!s.is_scanning);
                status_item.set_text(status_text);
                exit_item.set_text(Translations::tray_menu_exit(s.language));
            }

            std::thread::sleep(std::time::Duration::from_millis(40));
        }
    });
}