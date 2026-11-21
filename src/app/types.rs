use crate::i18n::{Language, Translations};
use crate::model::DeviceCategory;
use iced::Theme;
use std::fmt;
use std::net::Ipv4Addr;

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
