use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkEventType {
    DeviceConnected,
    DeviceDisconnected,
    NewDeviceDetected,
    SecurityAlert,
    InternetOutage,
    InternetRestored,
}

impl NetworkEventType {
    pub fn icon(&self) -> &'static str {
        match self {
            NetworkEventType::DeviceConnected => "🟢",
            NetworkEventType::DeviceDisconnected => "⚪",
            NetworkEventType::NewDeviceDetected => "✨",
            NetworkEventType::SecurityAlert => "🚨",
            NetworkEventType::InternetOutage => "🔴",
            NetworkEventType::InternetRestored => "🌐",
        }
    }

    #[allow(dead_code)]
    pub fn label(&self) -> &'static str {
        self.label_localized(crate::i18n::Language::English)
    }

    pub fn label_localized(&self, lang: crate::i18n::Language) -> &'static str {
        crate::i18n::Translations::event_type_name(*self, lang)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEvent {
    pub id: String,
    pub timestamp: DateTime<Local>,
    pub event_type: NetworkEventType,
    pub title: String,
    pub details: String,
    pub ip: Option<Ipv4Addr>,
    pub mac: Option<String>,
}

impl NetworkEvent {
    pub fn title_localized(&self, lang: crate::i18n::Language) -> String {
        crate::i18n::Translations::event_title(self.event_type, &self.title, lang)
    }

    pub fn details_localized(&self, lang: crate::i18n::Language) -> String {
        crate::i18n::Translations::event_details(self.event_type, &self.details, lang)
    }
}

pub struct HistoryManager;

impl HistoryManager {
    /// Creates a new network timeline event
    pub fn create_event(
        event_type: NetworkEventType,
        title: String,
        details: String,
        ip: Option<Ipv4Addr>,
        mac: Option<String>,
    ) -> NetworkEvent {
        let now = Local::now();
        let id = format!("{}_{}", now.timestamp_millis(), mac.as_deref().unwrap_or("net"));

        NetworkEvent {
            id,
            timestamp: now,
            event_type,
            title,
            details,
            ip,
            mac,
        }
    }

    /// Dispatches a native desktop notification on Linux (via notify-send)
    #[allow(dead_code)]
    pub fn send_desktop_notification(title: &str, body: &str) {
        #[cfg(windows)]
        {
            let _ = crate::procutil::windows_toast(title, body);
        }
        #[cfg(not(windows))]
        let _ = crate::procutil::NoWindow::no_window(Command::new("notify-send"))
            .arg("-a")
            .arg("NetLens")
            .arg("-i")
            .arg("network-workgroup")
            .arg(title)
            .arg(body)
            .spawn();
    }
}
