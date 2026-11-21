use crate::i18n::Language;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationUrgency {
    Low,
    Normal,
    Critical,
}

impl NotificationUrgency {
    #[allow(dead_code)]
    pub const ALL: &'static [NotificationUrgency] = &[
        NotificationUrgency::Low,
        NotificationUrgency::Normal,
        NotificationUrgency::Critical,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationUrgency::Low => "low",
            NotificationUrgency::Normal => "normal",
            NotificationUrgency::Critical => "critical",
        }
    }

    #[allow(dead_code)]
    pub fn label(&self, lang: Language) -> &'static str {
        match (self, lang) {
            (NotificationUrgency::Low, Language::Spanish) => "Baja",
            (NotificationUrgency::Low, Language::English) => "Low",
            (NotificationUrgency::Low, Language::Chinese) => "低",
            (NotificationUrgency::Low, Language::Hindi) => "कम",
            (NotificationUrgency::Low, Language::Arabic) => "منخفضة",
            (NotificationUrgency::Low, Language::French) => "Faible",
            (NotificationUrgency::Low, Language::Bengali) => "নিম্ন",
            (NotificationUrgency::Low, Language::Portuguese) => "Baixa",
            (NotificationUrgency::Low, Language::Russian) => "Низкий",
            (NotificationUrgency::Low, Language::Urdu) => "کم",

            (NotificationUrgency::Normal, Language::Spanish) => "Normal",
            (NotificationUrgency::Normal, Language::English) => "Normal",
            (NotificationUrgency::Normal, Language::Chinese) => "正常",
            (NotificationUrgency::Normal, Language::Hindi) => "सामान्य",
            (NotificationUrgency::Normal, Language::Arabic) => "عادية",
            (NotificationUrgency::Normal, Language::French) => "Normale",
            (NotificationUrgency::Normal, Language::Bengali) => "স্বাভাবিক",
            (NotificationUrgency::Normal, Language::Portuguese) => "Normal",
            (NotificationUrgency::Normal, Language::Russian) => "Обычный",
            (NotificationUrgency::Normal, Language::Urdu) => "عام",

            (NotificationUrgency::Critical, Language::Spanish) => "Crítica",
            (NotificationUrgency::Critical, Language::English) => "Critical",
            (NotificationUrgency::Critical, Language::Chinese) => "紧急",
            (NotificationUrgency::Critical, Language::Hindi) => "गंभीर",
            (NotificationUrgency::Critical, Language::Arabic) => "حرجة",
            (NotificationUrgency::Critical, Language::French) => "Critique",
            (NotificationUrgency::Critical, Language::Bengali) => "গুরুত্বপূর্ণ",
            (NotificationUrgency::Critical, Language::Portuguese) => "Crítica",
            (NotificationUrgency::Critical, Language::Russian) => "Критический",
            (NotificationUrgency::Critical, Language::Urdu) => "انتہائی اہم",
        }
    }
}

impl std::fmt::Display for NotificationUrgency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationUrgency::Low => write!(f, "Baja"),
            NotificationUrgency::Normal => write!(f, "Normal"),
            NotificationUrgency::Critical => write!(f, "Crítica"),
        }
    }
}

impl Default for NotificationUrgency {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationDetailLevel {
    Compact,
    Detailed,
    Full,
}

impl NotificationDetailLevel {
    #[allow(dead_code)]
    pub const ALL: &'static [NotificationDetailLevel] = &[
        NotificationDetailLevel::Compact,
        NotificationDetailLevel::Detailed,
        NotificationDetailLevel::Full,
    ];

    #[allow(dead_code)]
    pub fn label(&self, lang: Language) -> &'static str {
        match (self, lang) {
            (NotificationDetailLevel::Compact, Language::Spanish) => "Compacto",
            (NotificationDetailLevel::Compact, Language::English) => "Compact",
            (NotificationDetailLevel::Compact, Language::Chinese) => "紧凑",
            (NotificationDetailLevel::Compact, Language::Hindi) => "संक्षिप्त",
            (NotificationDetailLevel::Compact, Language::Arabic) => "موجز",
            (NotificationDetailLevel::Compact, Language::French) => "Compact",
            (NotificationDetailLevel::Compact, Language::Bengali) => "সংক্ষিপ্ত",
            (NotificationDetailLevel::Compact, Language::Portuguese) => "Compacto",
            (NotificationDetailLevel::Compact, Language::Russian) => "Компактный",
            (NotificationDetailLevel::Compact, Language::Urdu) => "مختصر",

            (NotificationDetailLevel::Detailed, Language::Spanish) => "Detallado",
            (NotificationDetailLevel::Detailed, Language::English) => "Detailed",
            (NotificationDetailLevel::Detailed, Language::Chinese) => "详细",
            (NotificationDetailLevel::Detailed, Language::Hindi) => "विस्तृत",
            (NotificationDetailLevel::Detailed, Language::Arabic) => "مفصل",
            (NotificationDetailLevel::Detailed, Language::French) => "Détaillé",
            (NotificationDetailLevel::Detailed, Language::Bengali) => "বিস্তারিত",
            (NotificationDetailLevel::Detailed, Language::Portuguese) => "Detalhado",
            (NotificationDetailLevel::Detailed, Language::Russian) => "Подробный",
            (NotificationDetailLevel::Detailed, Language::Urdu) => "تفصیلی",

            (NotificationDetailLevel::Full, Language::Spanish) => "Completo",
            (NotificationDetailLevel::Full, Language::English) => "Full",
            (NotificationDetailLevel::Full, Language::Chinese) => "完整",
            (NotificationDetailLevel::Full, Language::Hindi) => "पूर्ण",
            (NotificationDetailLevel::Full, Language::Arabic) => "كامل",
            (NotificationDetailLevel::Full, Language::French) => "Complet",
            (NotificationDetailLevel::Full, Language::Bengali) => "সম্পূর্ণ",
            (NotificationDetailLevel::Full, Language::Portuguese) => "Completo",
            (NotificationDetailLevel::Full, Language::Russian) => "Полный",
            (NotificationDetailLevel::Full, Language::Urdu) => "مکمل",
        }
    }
}

impl std::fmt::Display for NotificationDetailLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationDetailLevel::Compact => write!(f, "Compacto"),
            NotificationDetailLevel::Detailed => write!(f, "Detallado"),
            NotificationDetailLevel::Full => write!(f, "Completo"),
        }
    }
}

impl Default for NotificationDetailLevel {
    fn default() -> Self {
        Self::Detailed
    }
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn default_timeout() -> u32 {
    7
}

fn default_prefix() -> String {
    "NetLens".to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationSettings {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub notify_unknown_intruders: bool,
    #[serde(default = "default_false")]
    pub notify_known_unwhitelisted_reconnect: bool,
    #[serde(default = "default_false")]
    pub notify_whitelisted_reconnect: bool,
    #[serde(default = "default_false")]
    pub notify_device_disconnected: bool,
    #[serde(default = "default_false")]
    pub notify_internet_outages: bool,
    #[serde(default = "default_false")]
    pub notify_security_critical: bool,
    #[serde(default)]
    pub urgency: NotificationUrgency,
    #[serde(default = "default_timeout")]
    pub timeout_secs: u32,
    #[serde(default)]
    pub detail_level: NotificationDetailLevel,
    #[serde(default = "default_true")]
    pub show_mac: bool,
    #[serde(default = "default_true")]
    pub show_vendor: bool,
    #[serde(default = "default_prefix")]
    pub custom_prefix: String,
    #[serde(default = "default_true")]
    pub play_sound: bool,
    #[serde(default = "default_true")]
    pub realtime_netlink_monitor: bool,
    #[serde(default = "default_true")]
    pub fast_disconnection_heartbeat: bool,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            notify_unknown_intruders: true,
            notify_known_unwhitelisted_reconnect: false,
            notify_whitelisted_reconnect: false,
            notify_device_disconnected: false,
            notify_internet_outages: false,
            notify_security_critical: false,
            urgency: NotificationUrgency::Normal,
            timeout_secs: 7,
            detail_level: NotificationDetailLevel::Detailed,
            show_mac: true,
            show_vendor: true,
            custom_prefix: "NetLens".to_string(),
            play_sound: true,
            realtime_netlink_monitor: true,
            fast_disconnection_heartbeat: true,
        }
    }
}

impl NotificationSettings {
    /// Formats the title by prepending the configured prefix
    pub fn format_title(&self, title: &str) -> String {
        let prefix = self.custom_prefix.trim();
        if prefix.is_empty() {
            title.to_string()
        } else {
            format!("{}: {}", prefix, title)
        }
    }

    /// Formats notification body according to detail level and content filters
    pub fn format_device_message(
        &self,
        event_name: &str,
        dev_name: &str,
        ip: Ipv4Addr,
        mac: &str,
        vendor: &str,
        extra_info: Option<&str>,
    ) -> String {
        match self.detail_level {
            NotificationDetailLevel::Compact => {
                if let Some(extra) = extra_info {
                    format!("{} - {} ({}) [{}]", event_name, dev_name, ip, extra)
                } else {
                    format!("{} - {} ({})", event_name, dev_name, ip)
                }
            }
            NotificationDetailLevel::Detailed => {
                let mut lines = vec![format!("{} ({})", dev_name, ip)];
                if let Some(extra) = extra_info {
                    lines.push(extra.to_string());
                }
                if self.show_mac {
                    lines.push(format!("MAC: {}", mac));
                }
                lines.join("\n")
            }
            NotificationDetailLevel::Full => {
                let mut lines = vec![format!("{} ({})", dev_name, ip)];
                if let Some(extra) = extra_info {
                    lines.push(extra.to_string());
                }
                if self.show_mac {
                    lines.push(format!("MAC: {}", mac));
                }
                if self.show_vendor && !vendor.is_empty() && vendor != "Desconocido" && vendor != "Unknown" {
                    lines.push(format!("Vendor: {}", vendor));
                }
                lines.join("\n")
            }
        }
    }

    /// Dispatches standard notification respecting enablement, title, urgency, sound, and timeout
    pub fn dispatch(&self, title: &str, body: &str, icon: &str, custom_urgency: Option<NotificationUrgency>) {
        if !self.enabled {
            return;
        }

        let full_title = self.format_title(title);

        self.dispatch_impl(&full_title, body, icon, custom_urgency);
    }

    #[cfg(windows)]
    fn dispatch_impl(&self, full_title: &str, body: &str, _icon: &str, _custom_urgency: Option<NotificationUrgency>) {
        let _ = crate::procutil::windows_toast(full_title, body);
    }

    #[cfg(not(windows))]
    fn dispatch_impl(&self, full_title: &str, body: &str, icon: &str, custom_urgency: Option<NotificationUrgency>) {
        let urgency = custom_urgency.unwrap_or(self.urgency);
        let timeout_ms = (self.timeout_secs.max(1) * 1000).to_string();

        let mut cmd = crate::procutil::NoWindow::no_window(Command::new("notify-send"));
        cmd.arg("-u")
            .arg(urgency.as_str())
            .arg("-a")
            .arg("NetLens")
            .arg("-i")
            .arg(icon)
            .arg("-t")
            .arg(timeout_ms)
            .arg(full_title)
            .arg(body);

        let _ = cmd.spawn();

        if self.play_sound {
            let _ = crate::procutil::NoWindow::no_window(Command::new("canberra-gtk-play"))
                .arg("-i")
                .arg("message-new-instant")
                .spawn();
        }
    }

    /// Dispatches an asynchronous interactive alert with action buttons (Trust / Block)
    pub async fn dispatch_interactive_alert(
        &self,
        title: &str,
        body: &str,
        trust_label: &str,
        block_label: &str,
    ) -> String {
        if !self.enabled {
            return String::new();
        }

        let full_title = self.format_title(title);

        self.interactive_impl(&full_title, body, trust_label, block_label).await
    }

    /// Windows toasts launched from PowerShell cannot report button clicks back: show it as plain info
    #[cfg(windows)]
    async fn interactive_impl(&self, full_title: &str, body: &str, _trust_label: &str, _block_label: &str) -> String {
        let _ = crate::procutil::windows_toast(full_title, body);
        String::new()
    }

    #[cfg(not(windows))]
    async fn interactive_impl(&self, full_title: &str, body: &str, trust_label: &str, block_label: &str) -> String {
        let urgency = NotificationUrgency::Critical;
        let timeout_ms = (self.timeout_secs.max(15) * 1000).to_string();

        if self.play_sound {
            let _ = crate::procutil::NoWindow::no_window(tokio::process::Command::new("canberra-gtk-play"))
                .arg("-i")
                .arg("dialog-warning")
                .spawn();
        }

        let output = crate::procutil::NoWindow::no_window(tokio::process::Command::new("notify-send"))
            .arg("-u")
            .arg(urgency.as_str())
            .arg("-i")
            .arg("network-wireless")
            .arg("-a")
            .arg("NetLens")
            .arg("-t")
            .arg(timeout_ms)
            .arg("-A")
            .arg(format!("trust={}", trust_label))
            .arg("-A")
            .arg(format!("block={}", block_label))
            .arg(full_title)
            .arg(body)
            .output()
            .await;

        match output {
            Ok(out) => String::from_utf8_lossy(&out.stdout).trim().to_string(),
            Err(_) => String::new(),
        }
    }
}
