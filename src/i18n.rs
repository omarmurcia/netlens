use crate::model::{Device, DeviceCategory, DeviceStatus};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Los 10 idiomas más hablados del mundo (por número total de hablantes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Language {
    #[default]
    Spanish,
    English,
    Chinese,
    Hindi,
    Arabic,
    French,
    Bengali,
    Portuguese,
    Russian,
    Urdu,
}

impl Language {
    pub const ALL: &[Language] = &[
        Language::Spanish,
        Language::English,
        Language::Chinese,
        Language::Hindi,
        Language::Arabic,
        Language::French,
        Language::Bengali,
        Language::Portuguese,
        Language::Russian,
        Language::Urdu,
    ];

    pub fn native_name(&self) -> &'static str {
        match self {
            Language::Spanish => "Español",
            Language::English => "English",
            Language::Chinese => "中文 (Chino)",
            Language::Hindi => "हिन्दी (Hindi)",
            Language::Arabic => "العربية (Árabe)",
            Language::French => "Français (Francés)",
            Language::Bengali => "বাংলা (Bengalí)",
            Language::Portuguese => "Português",
            Language::Russian => "Русский (Ruso)",
            Language::Urdu => "اردو (Urdu)",
        }
    }

    #[allow(dead_code)]
    pub fn code(&self) -> &'static str {
        match self {
            Language::Spanish => "es",
            Language::English => "en",
            Language::Chinese => "zh",
            Language::Hindi => "hi",
            Language::Arabic => "ar",
            Language::French => "fr",
            Language::Bengali => "bn",
            Language::Portuguese => "pt",
            Language::Russian => "ru",
            Language::Urdu => "ur",
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.native_name())
    }
}

pub struct Translations;

impl Translations {
    pub fn window_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "NetLens - Explorador y Monitor de Red Local",
            Language::English => "NetLens - Local Network Device Explorer & Monitor",
            Language::Chinese => "NetLens - 局域网设备探测与实时监控",
            Language::Hindi => "NetLens - स्थानीय नेटवर्क एक्सप्लोरर और मॉनिटर",
            Language::Arabic => "NetLens - مستكشف ومراقب أجهزة الشبكة المحلية",
            Language::French => "NetLens - Explorateur et Moniteur de Réseau Local",
            Language::Bengali => "NetLens - স্থানীয় নেটওয়ার্ক ডিভাইস এক্সপ্লোরার ও মনিটর",
            Language::Portuguese => "NetLens - Explorador e Monitor de Rede Local",
            Language::Russian => "NetLens - Сканер и монитор локальной сети",
            Language::Urdu => "NetLens - مقامی نیٹ ورک ڈیوائس ایکسپلورر اور مانیٹر",
        }
    }

    pub fn app_subtitle(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Monitor y Explorador de Red Local",
            Language::English => "Local Network Device Monitor",
            Language::Chinese => "局域网设备监控与资源探测",
            Language::Hindi => "स्थानीय नेटवर्क डिवाइस मॉनिटर",
            Language::Arabic => "مراقب واستكشاف أجهزة الشبكة المحلية",
            Language::French => "Moniteur de Réseau Local et Périphériques",
            Language::Bengali => "স্থানীয় নেটওয়ার্ক ডিভাইস মনিটর",
            Language::Portuguese => "Monitor e Explorador de Rede Local",
            Language::Russian => "Монитор устройств локальной сети",
            Language::Urdu => "مقامی نیٹ ورک ڈیوائس مانیٹر",
        }
    }

    pub fn scan_network(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⚡ Escanear Red",
            Language::English => "⚡ Scan Network",
            Language::Chinese => "⚡ 扫描网络",
            Language::Hindi => "⚡ नेटवर्क स्कैन करें",
            Language::Arabic => "⚡ فحص الشبكة",
            Language::French => "⚡ Scanner le Réseau",
            Language::Bengali => "⚡ নেটওয়ার্ক স্ক্যান",
            Language::Portuguese => "⚡ Escanear Rede",
            Language::Russian => "⚡ Сканировать сеть",
            Language::Urdu => "⚡ نیٹ ورک اسکین کریں",
        }
    }

    pub fn scanning(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⏳ Escaneando...",
            Language::English => "⏳ Scanning...",
            Language::Chinese => "⏳ 正在扫描...",
            Language::Hindi => "⏳ स्कैनिंग जारी है...",
            Language::Arabic => "⏳ جاري الفحص...",
            Language::French => "⏳ Scan en cours...",
            Language::Bengali => "⏳ স্ক্যান হচ্ছে...",
            Language::Portuguese => "⏳ Escaneando...",
            Language::Russian => "⏳ Сканирование...",
            Language::Urdu => "⏳ اسکیننگ جاری ہے...",
        }
    }

    pub fn settings(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⚙️ Ajustes",
            Language::English => "⚙️ Settings",
            Language::Chinese => "⚙️ 设置",
            Language::Hindi => "⚙️ सेटिंग्स",
            Language::Arabic => "⚙️ الإعدادات",
            Language::French => "⚙️ Paramètres",
            Language::Bengali => "⚙️ সেটিংস",
            Language::Portuguese => "⚙️ Configurações",
            Language::Russian => "⚙️ Настройки",
            Language::Urdu => "⚙️ ترتیبات",
        }
    }

    pub fn download(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⬇️ DESCARGA",
            Language::English => "⬇️ DOWNLOAD",
            Language::Chinese => "⬇️ 下载速率",
            Language::Hindi => "⬇️ डाउनलोड",
            Language::Arabic => "⬇️ تنزيل",
            Language::French => "⬇️ TÉLÉCHARGEMENT",
            Language::Bengali => "⬇️ ডাউনলোড",
            Language::Portuguese => "⬇️ DOWNLOAD",
            Language::Russian => "⬇️ СКАЧИВАНИЕ",
            Language::Urdu => "⬇️ ڈاؤن لوڈ",
        }
    }

    pub fn upload(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⬆️ SUBIDA",
            Language::English => "⬆️ UPLOAD",
            Language::Chinese => "⬆️ 上传速率",
            Language::Hindi => "⬆️ अपलोड",
            Language::Arabic => "⬆️ رفع",
            Language::French => "⬆️ TÉLÉVERSEMENT",
            Language::Bengali => "⬆️ আপলোড",
            Language::Portuguese => "⬆️ UPLOAD",
            Language::Russian => "⬆️ ЗАГРУЗКА",
            Language::Urdu => "⬆️ اپ لوڈ",
        }
    }

    pub fn signal_wifi(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "📶 SEÑAL WI-FI",
            Language::English => "📶 WI-FI SIGNAL",
            Language::Chinese => "📶 WI-FI 信号强度",
            Language::Hindi => "📶 वाई-फाई सिग्नल",
            Language::Arabic => "📶 إشارة الواي فاي",
            Language::French => "📶 SIGNAL WI-FI",
            Language::Bengali => "📶 ওয়াই-ফাই সিগন্যাল",
            Language::Portuguese => "📶 SINAL WI-FI",
            Language::Russian => "📶 СИГНАЛ WI-FI",
            Language::Urdu => "📶 وائی فائی سگنل",
        }
    }

    pub fn ethernet_cable(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🔌 CABLE ETHERNET",
            Language::English => "🔌 ETHERNET CABLE",
            Language::Chinese => "🔌 以太网有线连接",
            Language::Hindi => "🔌 ईथरनेट केबल",
            Language::Arabic => "🔌 كابل إيثرنت",
            Language::French => "🔌 CÂBLE ÉTHERNET",
            Language::Bengali => "🔌 ইথারনেট কেবল",
            Language::Portuguese => "🔌 CABO ETHERNET",
            Language::Russian => "🔌 КАБЕЛЬ ETHERNET",
            Language::Urdu => "🔌 ایتھرنیٹ کیبل",
        }
    }

    pub fn live(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "EN VIVO",
            Language::English => "LIVE",
            Language::Chinese => "实时",
            Language::Hindi => "लाइव",
            Language::Arabic => "مباشر",
            Language::French => "EN DIRECT",
            Language::Bengali => "লাইভ",
            Language::Portuguese => "AO VIVO",
            Language::Russian => "В ЭФИРЕ",
            Language::Urdu => "لائیو",
        }
    }

    pub fn session(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Sesión",
            Language::English => "Session",
            Language::Chinese => "本次会话",
            Language::Hindi => "सत्र",
            Language::Arabic => "الجلسة",
            Language::French => "Session",
            Language::Bengali => "সেশন",
            Language::Portuguese => "Sessão",
            Language::Russian => "Сессия",
            Language::Urdu => "سیشن",
        }
    }

    pub fn network_and_router(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🌐 RED Y ROUTER",
            Language::English => "🌐 NETWORK & ROUTER",
            Language::Chinese => "🌐 网络与路由器",
            Language::Hindi => "🌐 नेटवर्क और राउटर",
            Language::Arabic => "🌐 الشبكة والموجّه",
            Language::French => "🌐 RÉSEAU & ROUTEUR",
            Language::Bengali => "🌐 নেটওয়ার্ক ও রাউটার",
            Language::Portuguese => "🌐 REDE E ROTEADOR",
            Language::Russian => "🌐 СЕТЬ И РОУТЕР",
            Language::Urdu => "🌐 نیٹ ورک اور راؤٹر",
        }
    }

    pub fn gateway_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Puerta de Enlace",
            Language::English => "Gateway",
            Language::Chinese => "网关",
            Language::Hindi => "गेटवे",
            Language::Arabic => "البوابة",
            Language::French => "Passerelle",
            Language::Bengali => "গেটওয়ে",
            Language::Portuguese => "Gateway",
            Language::Russian => "Шлюз",
            Language::Urdu => "گیٹ وے",
        }
    }

    pub fn devices_count(count: usize, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("{} Dispositivos", count),
            Language::English => format!("{} Devices", count),
            Language::Chinese => format!("{} 台设备", count),
            Language::Hindi => format!("{} उपकरण", count),
            Language::Arabic => format!("{} أجهزة", count),
            Language::French => format!("{} Appareils", count),
            Language::Bengali => format!("{} টি ডিভাইস", count),
            Language::Portuguese => format!("{} Dispositivos", count),
            Language::Russian => format!("{} Устройств", count),
            Language::Urdu => format!("{} آلات", count),
        }
    }

    pub fn not_detected(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "No detectado",
            Language::English => "Not detected",
            Language::Chinese => "未检测到",
            Language::Hindi => "पहचाना नहीं गया",
            Language::Arabic => "غير مكتشف",
            Language::French => "Non détecté",
            Language::Bengali => "শনাক্ত হয়নি",
            Language::Portuguese => "Não detectado",
            Language::Russian => "Не обнаружено",
            Language::Urdu => "معلوم نہیں ہوا",
        }
    }

    pub fn search_placeholder(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🔍 Buscar por IP, nombre, fabricante o MAC...",
            Language::English => "🔍 Search by IP, name, vendor, or MAC...",
            Language::Chinese => "🔍 按 IP、名称、厂商或 MAC 搜索...",
            Language::Hindi => "🔍 आईपी, नाम, निर्माता या मैक से खोजें...",
            Language::Arabic => "🔍 ابحث حسب IP، الاسم، الشركة أو MAC...",
            Language::French => "🔍 Rechercher par IP, nom, fabricant ou MAC...",
            Language::Bengali => "🔍 আইপি, নাম, ভেন্ডর বা ম্যাক দিয়ে খুঁজুন...",
            Language::Portuguese => "🔍 Buscar por IP, nome, fabricante ou MAC...",
            Language::Russian => "🔍 Поиск по IP, имени, вендору или MAC...",
            Language::Urdu => "🔍 تلاش کریں بذریعہ IP، نام، مینوفیکچرر یا MAC...",
        }
    }

    pub fn all_filter(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Todos",
            Language::English => "All",
            Language::Chinese => "全部",
            Language::Hindi => "सभी",
            Language::Arabic => "الكل",
            Language::French => "Tous",
            Language::Bengali => "সব",
            Language::Portuguese => "Todos",
            Language::Russian => "Все",
            Language::Urdu => "سبھی",
        }
    }

    pub fn category_short_name(cat: DeviceCategory, lang: Language) -> &'static str {
        match (cat, lang) {
            (DeviceCategory::Router, Language::Spanish) => "🌐 Router",
            (DeviceCategory::Router, Language::Chinese) => "🌐 路由器",
            (DeviceCategory::Router, Language::Hindi) => "🌐 राउटर",
            (DeviceCategory::Router, Language::Arabic) => "🌐 موجّه",
            (DeviceCategory::Router, Language::French) => "🌐 Routeur",
            (DeviceCategory::Router, Language::Bengali) => "🌐 রাউটার",
            (DeviceCategory::Router, Language::Portuguese) => "🌐 Roteador",
            (DeviceCategory::Router, Language::Russian) => "🌐 Роутер",
            (DeviceCategory::Router, Language::Urdu) => "🌐 راؤٹر",
            (DeviceCategory::Router, _) => "🌐 Router",

            (DeviceCategory::Smartphone, Language::Spanish) => "📱 Móvil",
            (DeviceCategory::Smartphone, Language::English) => "📱 Phone",
            (DeviceCategory::Smartphone, Language::Chinese) => "📱 手机",
            (DeviceCategory::Smartphone, Language::Hindi) => "📱 मोबाइल",
            (DeviceCategory::Smartphone, Language::Arabic) => "📱 هاتف",
            (DeviceCategory::Smartphone, Language::French) => "📱 Mobile",
            (DeviceCategory::Smartphone, Language::Bengali) => "📱 ফোন",
            (DeviceCategory::Smartphone, Language::Portuguese) => "📱 Celular",
            (DeviceCategory::Smartphone, Language::Russian) => "📱 Смартфон",
            (DeviceCategory::Smartphone, Language::Urdu) => "📱 موبائل",

            (DeviceCategory::Computer, Language::Spanish) => "💻 Ordenador",
            (DeviceCategory::Computer, Language::English) => "💻 PC",
            (DeviceCategory::Computer, Language::Chinese) => "💻 电脑",
            (DeviceCategory::Computer, Language::Hindi) => "💻 पीसी",
            (DeviceCategory::Computer, Language::Arabic) => "💻 كمبيوتر",
            (DeviceCategory::Computer, Language::French) => "💻 PC",
            (DeviceCategory::Computer, Language::Bengali) => "💻 পিসি",
            (DeviceCategory::Computer, Language::Portuguese) => "💻 PC",
            (DeviceCategory::Computer, Language::Russian) => "💻 ПК",
            (DeviceCategory::Computer, Language::Urdu) => "💻 پی سی",

            (DeviceCategory::SmartTv, Language::Spanish) => "📺 Smart TV",
            (DeviceCategory::SmartTv, Language::English) => "📺 Smart TV",
            (DeviceCategory::SmartTv, Language::Chinese) => "📺 电视",
            (DeviceCategory::SmartTv, Language::Hindi) => "📺 टीवी",
            (DeviceCategory::SmartTv, Language::Arabic) => "📺 تلفاز",
            (DeviceCategory::SmartTv, Language::French) => "📺 Télé",
            (DeviceCategory::SmartTv, Language::Bengali) => "📺 টিভি",
            (DeviceCategory::SmartTv, Language::Portuguese) => "📺 Smart TV",
            (DeviceCategory::SmartTv, Language::Russian) => "📺 ТВ",
            (DeviceCategory::SmartTv, Language::Urdu) => "📺 ٹی وی",

            (DeviceCategory::Iot, Language::Spanish) => "🔌 Domótica",
            (DeviceCategory::Iot, Language::English) => "🔌 Smart Home",
            (DeviceCategory::Iot, Language::Chinese) => "🔌 物联网",
            (DeviceCategory::Iot, Language::Hindi) => "🔌 आईओटी",
            (DeviceCategory::Iot, Language::Arabic) => "🔌 إنترنت الأشياء",
            (DeviceCategory::Iot, Language::French) => "🔌 Domotique",
            (DeviceCategory::Iot, Language::Bengali) => "🔌 আইওটি",
            (DeviceCategory::Iot, Language::Portuguese) => "🔌 Casa Inteligente",
            (DeviceCategory::Iot, Language::Russian) => "🔌 Умный дом",
            (DeviceCategory::Iot, Language::Urdu) => "🔌 آئی او ٹی",

            (DeviceCategory::Printer, Language::Spanish) => "🖨️ Impresora",
            (DeviceCategory::Printer, Language::English) => "🖨️ Printer",
            (DeviceCategory::Printer, Language::Chinese) => "🖨️ 打印机",
            (DeviceCategory::Printer, Language::Hindi) => "🖨️ प्रिंटर",
            (DeviceCategory::Printer, Language::Arabic) => "🖨️ طابعة",
            (DeviceCategory::Printer, Language::French) => "🖨️ Imprimante",
            (DeviceCategory::Printer, Language::Bengali) => "🖨️ প্রিন্টার",
            (DeviceCategory::Printer, Language::Portuguese) => "🖨️ Impressora",
            (DeviceCategory::Printer, Language::Russian) => "🖨️ Принтер",
            (DeviceCategory::Printer, Language::Urdu) => "🖨️ پرنٹر",

            (DeviceCategory::Server, Language::Spanish) => "🖧 Servidor",
            (DeviceCategory::Server, Language::English) => "🖧 Server",
            (DeviceCategory::Server, Language::Chinese) => "🖧 服务器",
            (DeviceCategory::Server, Language::Hindi) => "🖧 सर्वर",
            (DeviceCategory::Server, Language::Arabic) => "🖧 خادم",
            (DeviceCategory::Server, Language::French) => "🖧 Serveur",
            (DeviceCategory::Server, Language::Bengali) => "🖧 সার্ভার",
            (DeviceCategory::Server, Language::Portuguese) => "🖧 Servidor",
            (DeviceCategory::Server, Language::Russian) => "🖧 Сервер",
            (DeviceCategory::Server, Language::Urdu) => "🖧 سرور",

            (DeviceCategory::Unknown, Language::Spanish) => "❓ Genérico",
            (DeviceCategory::Unknown, Language::English) => "❓ Generic",
            (DeviceCategory::Unknown, Language::Chinese) => "❓ 通用",
            (DeviceCategory::Unknown, Language::Hindi) => "❓ सामान्य",
            (DeviceCategory::Unknown, Language::Arabic) => "❓ عام",
            (DeviceCategory::Unknown, Language::French) => "❓ Générique",
            (DeviceCategory::Unknown, Language::Bengali) => "❓ সাধারণ",
            (DeviceCategory::Unknown, Language::Portuguese) => "❓ Genérico",
            (DeviceCategory::Unknown, Language::Russian) => "❓ Другое",
            (DeviceCategory::Unknown, Language::Urdu) => "❓ عام",
        }
    }

    pub fn category_name(cat: DeviceCategory, lang: Language) -> &'static str {
        match (cat, lang) {
            (DeviceCategory::Router, Language::Spanish) => "Router / Puerta de Enlace",
            (DeviceCategory::Router, Language::English) => "Router / Gateway",
            (DeviceCategory::Router, Language::Chinese) => "路由器 / 网关",
            (DeviceCategory::Router, Language::Hindi) => "राउटर / गेटवे",
            (DeviceCategory::Router, Language::Arabic) => "الموجّه / البوابة",
            (DeviceCategory::Router, Language::French) => "Routeur / Passerelle",
            (DeviceCategory::Router, Language::Bengali) => "রাউটার / গেটওয়ে",
            (DeviceCategory::Router, Language::Portuguese) => "Roteador / Gateway",
            (DeviceCategory::Router, Language::Russian) => "Маршрутизатор / Шлюз",
            (DeviceCategory::Router, Language::Urdu) => "راؤٹر / گیٹ وے",

            (DeviceCategory::Smartphone, Language::Spanish) => "Smartphone / Móvil",
            (DeviceCategory::Smartphone, Language::English) => "Smartphone / Tablet",
            (DeviceCategory::Smartphone, Language::Chinese) => "智能手机 / 平板",
            (DeviceCategory::Smartphone, Language::Hindi) => "स्मार्टफोन / मोबाइल",
            (DeviceCategory::Smartphone, Language::Arabic) => "هاتف ذكي / لوحي",
            (DeviceCategory::Smartphone, Language::French) => "Smartphone / Tablette",
            (DeviceCategory::Smartphone, Language::Bengali) => "স্মার্টফোন / মোবাইল",
            (DeviceCategory::Smartphone, Language::Portuguese) => "Smartphone / Tablet",
            (DeviceCategory::Smartphone, Language::Russian) => "Смартфон / Планшет",
            (DeviceCategory::Smartphone, Language::Urdu) => "اسمارٹ فون / موبائل",

            (DeviceCategory::Computer, Language::Spanish) => "Ordenador / PC",
            (DeviceCategory::Computer, Language::English) => "Computer / PC",
            (DeviceCategory::Computer, Language::Chinese) => "计算机 / 电脑",
            (DeviceCategory::Computer, Language::Hindi) => "कंप्यूटर / पीसी",
            (DeviceCategory::Computer, Language::Arabic) => "حاسوب / كمبيوتر",
            (DeviceCategory::Computer, Language::French) => "Ordinateur / PC",
            (DeviceCategory::Computer, Language::Bengali) => "কম্পিউটার / পিসি",
            (DeviceCategory::Computer, Language::Portuguese) => "Computador / PC",
            (DeviceCategory::Computer, Language::Russian) => "Компьютер / ПК",
            (DeviceCategory::Computer, Language::Urdu) => "کمپیوٹر / پی سی",

            (DeviceCategory::SmartTv, Language::Spanish) => "Smart TV / Multimedia",
            (DeviceCategory::SmartTv, Language::English) => "Smart TV / Streaming",
            (DeviceCategory::SmartTv, Language::Chinese) => "智能电视 / 流媒体",
            (DeviceCategory::SmartTv, Language::Hindi) => "स्मार्ट टीवी / मीडिया",
            (DeviceCategory::SmartTv, Language::Arabic) => "تلفاز ذكي / وسائط",
            (DeviceCategory::SmartTv, Language::French) => "Smart TV / Streaming",
            (DeviceCategory::SmartTv, Language::Bengali) => "স্মার্ট টিভি / মিডিয়া",
            (DeviceCategory::SmartTv, Language::Portuguese) => "Smart TV / Streaming",
            (DeviceCategory::SmartTv, Language::Russian) => "Smart TV / Медиа",
            (DeviceCategory::SmartTv, Language::Urdu) => "اسمارٹ ٹی وی / میڈیا",

            (DeviceCategory::Iot, Language::Spanish) => "Domótica / IoT",
            (DeviceCategory::Iot, Language::English) => "Smart Home / IoT",
            (DeviceCategory::Iot, Language::Chinese) => "智能家居 / 物联网",
            (DeviceCategory::Iot, Language::Hindi) => "स्मार्ट होम / आईओटी",
            (DeviceCategory::Iot, Language::Arabic) => "المنزل الذكي / إنترنت الأشياء",
            (DeviceCategory::Iot, Language::French) => "Maison Connectée / IoT",
            (DeviceCategory::Iot, Language::Bengali) => "স্মার্ট হোম / আইওটি",
            (DeviceCategory::Iot, Language::Portuguese) => "Casa Inteligente / IoT",
            (DeviceCategory::Iot, Language::Russian) => "Умный дом / IoT",
            (DeviceCategory::Iot, Language::Urdu) => "اسمارٹ ہوم / آئی او ٹی",

            (DeviceCategory::Printer, Language::Spanish) => "Impresora de Red",
            (DeviceCategory::Printer, Language::English) => "Network Printer",
            (DeviceCategory::Printer, Language::Chinese) => "网络打印机",
            (DeviceCategory::Printer, Language::Hindi) => "नेटवर्क प्रिंटर",
            (DeviceCategory::Printer, Language::Arabic) => "طابعة شبكية",
            (DeviceCategory::Printer, Language::French) => "Imprimante Réseau",
            (DeviceCategory::Printer, Language::Bengali) => "নেটওয়ার্ক প্রিন্টার",
            (DeviceCategory::Printer, Language::Portuguese) => "Impressora de Rede",
            (DeviceCategory::Printer, Language::Russian) => "Сетевой принтер",
            (DeviceCategory::Printer, Language::Urdu) => "نیٹ ورک پرنٹر",

            (DeviceCategory::Server, Language::Spanish) => "Servidor / NAS",
            (DeviceCategory::Server, Language::English) => "Server / NAS",
            (DeviceCategory::Server, Language::Chinese) => "服务器 / NAS",
            (DeviceCategory::Server, Language::Hindi) => "सर्वर / एनएएस",
            (DeviceCategory::Server, Language::Arabic) => "خادم / وحدة تخزين شبكية",
            (DeviceCategory::Server, Language::French) => "Serveur / NAS",
            (DeviceCategory::Server, Language::Bengali) => "সার্ভার / ন্যাস",
            (DeviceCategory::Server, Language::Portuguese) => "Servidor / NAS",
            (DeviceCategory::Server, Language::Russian) => "Сервер / NAS",
            (DeviceCategory::Server, Language::Urdu) => "سرور / این اے ایس",

            (DeviceCategory::Unknown, Language::Spanish) => "Dispositivo Genérico",
            (DeviceCategory::Unknown, Language::English) => "Generic Device",
            (DeviceCategory::Unknown, Language::Chinese) => "通用设备",
            (DeviceCategory::Unknown, Language::Hindi) => "सामान्य उपकरण",
            (DeviceCategory::Unknown, Language::Arabic) => "جهاز عام",
            (DeviceCategory::Unknown, Language::French) => "Périphérique Générique",
            (DeviceCategory::Unknown, Language::Bengali) => "সাধারণ ডিভাইস",
            (DeviceCategory::Unknown, Language::Portuguese) => "Dispositivo Genérico",
            (DeviceCategory::Unknown, Language::Russian) => "Другое устройство",
            (DeviceCategory::Unknown, Language::Urdu) => "عام ڈیوائس",
        }
    }

    pub fn sort_label(sort: crate::app::SortOption, lang: Language) -> &'static str {
        use crate::app::SortOption;
        match (sort, lang) {
            (SortOption::Ip, _) => "🌐 IP",
            (SortOption::Latency, _) => "⚡ Ping",

            (SortOption::Name, Language::Spanish) => "📝 Nombre",
            (SortOption::Name, Language::English) => "📝 Name",
            (SortOption::Name, Language::Chinese) => "📝 名称",
            (SortOption::Name, Language::Hindi) => "📝 नाम",
            (SortOption::Name, Language::Arabic) => "📝 الاسم",
            (SortOption::Name, Language::French) => "📝 Nom",
            (SortOption::Name, Language::Bengali) => "📝 নাম",
            (SortOption::Name, Language::Portuguese) => "📝 Nome",
            (SortOption::Name, Language::Russian) => "📝 Имя",
            (SortOption::Name, Language::Urdu) => "📝 نام",

            (SortOption::Vendor, Language::Spanish) => "🏭 Fabricante",
            (SortOption::Vendor, Language::English) => "🏭 Vendor",
            (SortOption::Vendor, Language::Chinese) => "🏭 厂商",
            (SortOption::Vendor, Language::Hindi) => "🏭 निर्माता",
            (SortOption::Vendor, Language::Arabic) => "🏭 الشركة",
            (SortOption::Vendor, Language::French) => "🏭 Fabricant",
            (SortOption::Vendor, Language::Bengali) => "🏭 প্রস্তুতকারক",
            (SortOption::Vendor, Language::Portuguese) => "🏭 Fabricante",
            (SortOption::Vendor, Language::Russian) => "🏭 Производитель",
            (SortOption::Vendor, Language::Urdu) => "🏭 مینوفیکچرر",

            (SortOption::Category, Language::Spanish) => "🏷️ Tipo",
            (SortOption::Category, Language::English) => "🏷️ Type",
            (SortOption::Category, Language::Chinese) => "🏷️ 类型",
            (SortOption::Category, Language::Hindi) => "🏷️ प्रकार",
            (SortOption::Category, Language::Arabic) => "🏷️ النوع",
            (SortOption::Category, Language::French) => "🏷️ Type",
            (SortOption::Category, Language::Bengali) => "🏷️ ধরন",
            (SortOption::Category, Language::Portuguese) => "🏷️ Tipo",
            (SortOption::Category, Language::Russian) => "🏷️ Тип",
            (SortOption::Category, Language::Urdu) => "🏷️ قسم",
        }
    }

    pub fn badge_router(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "ROUTER",
            Language::English => "ROUTER",
            Language::Chinese => "路由器",
            Language::Hindi => "राउटर",
            Language::Arabic => "موجّه",
            Language::French => "ROUTEUR",
            Language::Bengali => "রাউটার",
            Language::Portuguese => "ROTEADOR",
            Language::Russian => "РОУТЕР",
            Language::Urdu => "راؤٹر",
        }
    }

    pub fn badge_local_host(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "ESTE EQUIPO",
            Language::English => "THIS DEVICE",
            Language::Chinese => "本机",
            Language::Hindi => "यह उपकरण",
            Language::Arabic => "هذا الجهاز",
            Language::French => "CET APPAREIL",
            Language::Bengali => "এই ডিভাইস",
            Language::Portuguese => "ESTE DISPOSITIVO",
            Language::Russian => "ЭТО УСТРОЙСТВО",
            Language::Urdu => "یہ ڈیوائس",
        }
    }

    pub fn badge_privacy(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🛡️ Privacidad",
            Language::English => "🛡️ Privacy",
            Language::Chinese => "🛡️ 隐私保护",
            Language::Hindi => "🛡️ गोपनीयता",
            Language::Arabic => "🛡️ الخصوصية",
            Language::French => "🛡️ Confidentialité",
            Language::Bengali => "🛡️ গোপনীয়তা",
            Language::Portuguese => "🛡️ Privacidade",
            Language::Russian => "🛡️ Приватность",
            Language::Urdu => "🛡️ پرائیویسی",
        }
    }

    pub fn vendor_private(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "MAC Privada",
            Language::English => "Private MAC",
            Language::Chinese => "私有 MAC",
            Language::Hindi => "निजी मैक",
            Language::Arabic => "MAC خاص",
            Language::French => "MAC Privée",
            Language::Bengali => "ব্যক্তিগত ম্যাক",
            Language::Portuguese => "MAC Privado",
            Language::Russian => "Приватный MAC",
            Language::Urdu => "پرائیویٹ میک",
        }
    }

    pub fn vendor_generic(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Genérico",
            Language::English => "Generic",
            Language::Chinese => "通用设备",
            Language::Hindi => "सामान्य",
            Language::Arabic => "عام",
            Language::French => "Générique",
            Language::Bengali => "সাধারণ",
            Language::Portuguese => "Genérico",
            Language::Russian => "Обычное",
            Language::Urdu => "عام",
        }
    }

    pub fn latency_cable(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Cable (<1ms)",
            Language::English => "Cable (<1ms)",
            Language::Chinese => "有线连接 (<1ms)",
            Language::Hindi => "केबल (<1ms)",
            Language::Arabic => "كابل (<1ms)",
            Language::French => "Câble (<1ms)",
            Language::Bengali => "কেবল (<1ms)",
            Language::Portuguese => "Cabo (<1ms)",
            Language::Russian => "Кабель (<1ms)",
            Language::Urdu => "کیبل (<1ms)",
        }
    }

    pub fn latency_excellent(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Wi-Fi Excelente",
            Language::English => "Wi-Fi Excellent",
            Language::Chinese => "Wi-Fi 极佳",
            Language::Hindi => "वाई-फाई उत्कृष्ट",
            Language::Arabic => "واي فاي ممتاز",
            Language::French => "Wi-Fi Excellent",
            Language::Bengali => "ওয়াই-ফাই চমৎকার",
            Language::Portuguese => "Wi-Fi Excelente",
            Language::Russian => "Wi-Fi Отлично",
            Language::Urdu => "وائی فائی بہترین",
        }
    }

    pub fn latency_good(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Wi-Fi Bueno",
            Language::English => "Wi-Fi Good",
            Language::Chinese => "Wi-Fi 良好",
            Language::Hindi => "वाई-फाई अच्छा",
            Language::Arabic => "واي فاي جيد",
            Language::French => "Wi-Fi Bon",
            Language::Bengali => "ওয়াই-ফাই ভাল",
            Language::Portuguese => "Wi-Fi Bom",
            Language::Russian => "Wi-Fi Хорошо",
            Language::Urdu => "وائی فائی اچھا",
        }
    }

    pub fn latency_fair(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Wi-Fi Moderado",
            Language::English => "Wi-Fi Moderate",
            Language::Chinese => "Wi-Fi 一般",
            Language::Hindi => "वाई-फाई मध्यम",
            Language::Arabic => "واي فاي متوسط",
            Language::French => "Wi-Fi Moyen",
            Language::Bengali => "ওয়াই-ফাই মাঝারি",
            Language::Portuguese => "Wi-Fi Médio",
            Language::Russian => "Wi-Fi Средне",
            Language::Urdu => "وائی فائی درمیانہ",
        }
    }

    pub fn latency_weak(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Señal Débil",
            Language::English => "Weak Signal",
            Language::Chinese => "信号较弱",
            Language::Hindi => "कमजोर सिग्नल",
            Language::Arabic => "إشارة ضعيفة",
            Language::French => "Signal Faible",
            Language::Bengali => "দুর্বল সিগন্যাল",
            Language::Portuguese => "Sinal Fraco",
            Language::Russian => "Слабый сигнал",
            Language::Urdu => "کمزور سگنل",
        }
    }

    pub fn latency_no_response(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Sin respuesta ICMP",
            Language::English => "No ICMP response",
            Language::Chinese => "无 ICMP 响应",
            Language::Hindi => "कोई आईसीएमपी उत्तर नहीं",
            Language::Arabic => "لا استجابة ICMP",
            Language::French => "Aucune réponse ICMP",
            Language::Bengali => "কোন ICMP উত্তর নেই",
            Language::Portuguese => "Sem resposta ICMP",
            Language::Russian => "Нет ответа ICMP",
            Language::Urdu => "کوئی ICMP جواب نہیں",
        }
    }

    pub fn details_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Detalles →",
            Language::English => "Details →",
            Language::Chinese => "详情 →",
            Language::Hindi => "विवरण →",
            Language::Arabic => "التفاصيل →",
            Language::French => "Détails →",
            Language::Bengali => "বিবরণ →",
            Language::Portuguese => "Detalhes →",
            Language::Russian => "Детали →",
            Language::Urdu => "تفصیلات →",
        }
    }

    pub fn device_details(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Detalles del Dispositivo",
            Language::English => "Device Details",
            Language::Chinese => "设备详细信息",
            Language::Hindi => "डिवाइस विवरण",
            Language::Arabic => "تفاصيل الجهاز",
            Language::French => "Détails de l'appareil",
            Language::Bengali => "ডিভাইসের বিবরণ",
            Language::Portuguese => "Detalhes do Dispositivo",
            Language::Russian => "Сведения об устройстве",
            Language::Urdu => "ڈیوائس کی تفصیلات",
        }
    }

    pub fn device_type_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Tipo de Dispositivo:",
            Language::English => "Device Type:",
            Language::Chinese => "设备类型：",
            Language::Hindi => "डिवाइस का प्रकार:",
            Language::Arabic => "نوع الجهاز:",
            Language::French => "Type d'appareil :",
            Language::Bengali => "ডিভাইসের ধরন:",
            Language::Portuguese => "Tipo de Dispositivo:",
            Language::Russian => "Тип устройства:",
            Language::Urdu => "ڈیوائس کی قسم:",
        }
    }

    pub fn alias_placeholder(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Apodo / Nota personalizada...",
            Language::English => "Custom nickname / note...",
            Language::Chinese => "自定义昵称/备注...",
            Language::Hindi => "कस्टम उपनाम / नोट...",
            Language::Arabic => "اللقب المخصص / ملاحظة...",
            Language::French => "Surnom personnalisé / note...",
            Language::Bengali => "কাস্টম ডাকনাম / নোট...",
            Language::Portuguese => "Apelido / Nota personalizada...",
            Language::Russian => "Псевдоним / заметка...",
            Language::Urdu => "عرفیت / ذاتی نوٹ...",
        }
    }

    pub fn save(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Guardar",
            Language::English => "Save",
            Language::Chinese => "保存",
            Language::Hindi => "सहेजें",
            Language::Arabic => "حفظ",
            Language::French => "Enregistrer",
            Language::Bengali => "সংরক্ষণ",
            Language::Portuguese => "Salvar",
            Language::Russian => "Сохранить",
            Language::Urdu => "محفوظ کریں",
        }
    }

    pub fn close(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Cerrar",
            Language::English => "Close",
            Language::Chinese => "关闭",
            Language::Hindi => "बंद करें",
            Language::Arabic => "إغلاق",
            Language::French => "Fermer",
            Language::Bengali => "বন্ধ করুন",
            Language::Portuguese => "Fechar",
            Language::Russian => "Закрыть",
            Language::Urdu => "بند کریں",
        }
    }

    pub fn theme_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Tema del UI:",
            Language::English => "UI Theme:",
            Language::Chinese => "界面主题：",
            Language::Hindi => "यूआई थीम:",
            Language::Arabic => "سمة الواجهة:",
            Language::French => "Thème de l'interface :",
            Language::Bengali => "ইউজার ইন্টারফেস থিম:",
            Language::Portuguese => "Tema da Interface:",
            Language::Russian => "Тема оформления:",
            Language::Urdu => "انٹرفیس تھیم:",
        }
    }

    pub fn theme_desc(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Personaliza los colores y el estilo visual de NetLens",
            Language::English => "Customize NetLens color palette and visual styling",
            Language::Chinese => "自定义 NetLens 的色彩风格与界面视觉主题",
            Language::Hindi => "NetLens के रंग और विज़ुअल थीम को कस्टमाइज़ करें",
            Language::Arabic => "تخصيص لوحة الألوان والسمة المرئية لـ NetLens",
            Language::French => "Personnalisez la palette de couleurs et le thème visuel",
            Language::Bengali => "NetLens-এর রঙের প্যালেট ও ভিজ্যুয়াল স্টাইল পরিবর্তন করুন",
            Language::Portuguese => "Personalize as cores e o estilo visual do NetLens",
            Language::Russian => "Настройте цветовую схему и визуальный стиль NetLens",
            Language::Urdu => "NetLens کے رنگ اور بصری تھیم کو اپنی مرضی کے مطابق بنائیں",
        }
    }

    pub fn language_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Idioma de la Aplicación:",
            Language::English => "App Language:",
            Language::Chinese => "应用语言：",
            Language::Hindi => "ऐप की भाषा:",
            Language::Arabic => "لغة التطبيق:",
            Language::French => "Langue de l'application :",
            Language::Bengali => "অ্যাপের ভাষা:",
            Language::Portuguese => "Idioma do Aplicativo:",
            Language::Russian => "Язык приложения:",
            Language::Urdu => "ایپ کی زبان:",
        }
    }

    pub fn language_desc(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Soporte para los 10 idiomas con mayor número de hablantes del mundo",
            Language::English => "Support for the 10 most spoken languages in the world",
            Language::Chinese => "全面支持全球使用人数最多的前 10 大语言",
            Language::Hindi => "विश्व की 10 सबसे अधिक बोली जाने वाली भाषाओं का समर्थन",
            Language::Arabic => "دعم كامل لأكثر 10 لغات انتشاراً وتحدثاً في العالم",
            Language::French => "Prise en charge des 10 langues les plus parlées au monde",
            Language::Bengali => "বিশ্বের শীর্ষ ১০টি সর্বাধিক ব্যবহৃত ভাষার সম্পূর্ণ সমর্থন",
            Language::Portuguese => "Suporte para os 10 idiomas mais falados do mundo",
            Language::Russian => "Поддержка 10 самых распространенных языков мира",
            Language::Urdu => "دنیا کی 10 سب سے زیادہ بولی جانے والی زبانوں کی معاونت",
        }
    }

    pub fn sys_info_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Información de Conectividad del Sistema",
            Language::English => "System Connectivity Information",
            Language::Chinese => "系统网络连接与网关信息",
            Language::Hindi => "सिस्टम कनेक्टिविटी जानकारी",
            Language::Arabic => "معلومات اتصال النظام",
            Language::French => "Informations sur la connectivité du système",
            Language::Bengali => "সিস্টেম সংযোগের তথ্য",
            Language::Portuguese => "Informações de Conectividade do Sistema",
            Language::Russian => "Информация о подключении системы",
            Language::Urdu => "سسٹم کنیکٹیویٹی معلومات",
        }
    }

    pub fn sys_active_iface(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Interfaz Activa:",
            Language::English => "Active Interface:",
            Language::Chinese => "活跃网卡接口：",
            Language::Hindi => "सक्रिय इंटरफ़ेस:",
            Language::Arabic => "الواجهة النشطة:",
            Language::French => "Interface Active :",
            Language::Bengali => "সক্রিয় ইন্টারফেস:",
            Language::Portuguese => "Interface Ativa:",
            Language::Russian => "Активный интерфейс:",
            Language::Urdu => "فعال انٹرفیس:",
        }
    }

    pub fn sys_subnet(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Subred Local:",
            Language::English => "Local Subnet:",
            Language::Chinese => "本地网段：",
            Language::Hindi => "स्थानीय सबनेट:",
            Language::Arabic => "الشبكة الفرعية:",
            Language::French => "Sous-réseau Local :",
            Language::Bengali => "স্থানীয় সাবনেট:",
            Language::Portuguese => "Sub-rede Local:",
            Language::Russian => "Локальная подсеть:",
            Language::Urdu => "مقامی سب نیٹ:",
        }
    }

    pub fn sys_gateway(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Puerta de Enlace:",
            Language::English => "Gateway:",
            Language::Chinese => "网关地址：",
            Language::Hindi => "गेटवे:",
            Language::Arabic => "البوابة:",
            Language::French => "Passerelle :",
            Language::Bengali => "গেটওয়ে:",
            Language::Portuguese => "Gateway:",
            Language::Russian => "Шлюз:",
            Language::Urdu => "گیٹ وے:",
        }
    }

    pub fn spec_ip(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Dirección IP:",
            Language::English => "IP Address:",
            Language::Chinese => "IP 地址：",
            Language::Hindi => "आईपी पता:",
            Language::Arabic => "عنوان IP:",
            Language::French => "Adresse IP :",
            Language::Bengali => "আইপি ঠিকানা:",
            Language::Portuguese => "Endereço IP:",
            Language::Russian => "IP-адрес:",
            Language::Urdu => "IP ایڈریس:",
        }
    }

    pub fn spec_mac(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Dirección MAC:",
            Language::English => "MAC Address:",
            Language::Chinese => "MAC 地址：",
            Language::Hindi => "मैक पता:",
            Language::Arabic => "عنوان MAC:",
            Language::French => "Adresse MAC :",
            Language::Bengali => "ম্যাক ঠিকানা:",
            Language::Portuguese => "Endereço MAC:",
            Language::Russian => "MAC-адрес:",
            Language::Urdu => "میک ایڈریس:",
        }
    }

    pub fn spec_vendor(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Fabricante:",
            Language::English => "Manufacturer:",
            Language::Chinese => "设备厂商：",
            Language::Hindi => "निर्माता:",
            Language::Arabic => "الشركة المصنعة:",
            Language::French => "Fabricant :",
            Language::Bengali => "প্রস্তুতকারক:",
            Language::Portuguese => "Fabricante:",
            Language::Russian => "Производитель:",
            Language::Urdu => "مینوفیکچرر:",
        }
    }

    pub fn spec_hostname(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Hostname DNS:",
            Language::English => "DNS Hostname:",
            Language::Chinese => "DNS 主机名：",
            Language::Hindi => "डीएनएस होस्टनेम:",
            Language::Arabic => "اسم المضيف DNS:",
            Language::French => "Nom d'hôte DNS :",
            Language::Bengali => "ডিএনএস হোস্টনাম:",
            Language::Portuguese => "Hostname DNS:",
            Language::Russian => "Имя узла DNS:",
            Language::Urdu => "DNS ہوسٹ نیم:",
        }
    }

    pub fn spec_not_assigned(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "No asignado",
            Language::English => "Not assigned",
            Language::Chinese => "未分配",
            Language::Hindi => "असाइन नहीं",
            Language::Arabic => "غير معين",
            Language::French => "Non assigné",
            Language::Bengali => "নির্ধারিত নয়",
            Language::Portuguese => "Não atribuído",
            Language::Russian => "Не назначено",
            Language::Urdu => "تعین نہیں کیا گیا",
        }
    }

    pub fn spec_mac_type(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Tipo de MAC:",
            Language::English => "MAC Type:",
            Language::Chinese => "MAC 类型：",
            Language::Hindi => "मैक प्रकार:",
            Language::Arabic => "نوع MAC:",
            Language::French => "Type de MAC :",
            Language::Bengali => "ম্যাকের ধরন:",
            Language::Portuguese => "Tipo de MAC:",
            Language::Russian => "Тип MAC:",
            Language::Urdu => "میک کی قسم:",
        }
    }

    pub fn mac_type_private(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🛡️ Privada / Aleatoria (LAA)",
            Language::English => "🛡️ Private / Randomized (LAA)",
            Language::Chinese => "🛡️ 随机/私有地址 (LAA)",
            Language::Hindi => "🛡️ निजी / रैंडम (LAA)",
            Language::Arabic => "🛡️ خاص / عشوائي (LAA)",
            Language::French => "🛡️ Privée / Aléatoire (LAA)",
            Language::Bengali => "🛡️ ব্যক্তিগত / র‍্যান্ডম (LAA)",
            Language::Portuguese => "🛡️ Privado / Aleatório (LAA)",
            Language::Russian => "🛡️ Приватный / Случайный (LAA)",
            Language::Urdu => "🛡️ پرائیویٹ / رینڈم (LAA)",
        }
    }

    pub fn mac_type_global(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🌐 OUI Global Registrada",
            Language::English => "🌐 Registered Global OUI",
            Language::Chinese => "🌐 全球注册 OUI",
            Language::Hindi => "🌐 पंजीकृत ग्लोबल OUI",
            Language::Arabic => "🌐 OUI مسجل عالمياً",
            Language::French => "🌐 OUI Global Enregistré",
            Language::Bengali => "🌐 নিবন্ধিত গ্লোবাল OUI",
            Language::Portuguese => "🌐 OUI Global Registrado",
            Language::Russian => "🌐 Зарегистрированный OUI",
            Language::Urdu => "🌐 رجسٹرڈ گلوبل OUI",
        }
    }

    pub fn spec_link_quality(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Calidad Enlace:",
            Language::English => "Link Quality:",
            Language::Chinese => "链路质量：",
            Language::Hindi => "लिंक गुणवत्ता:",
            Language::Arabic => "جودة الاتصال:",
            Language::French => "Qualité du lien :",
            Language::Bengali => "সংযোগের মান:",
            Language::Portuguese => "Qualidade do Link:",
            Language::Russian => "Качество связи:",
            Language::Urdu => "ربط کا معیار:",
        }
    }

    pub fn link_quality_desc(latency: Option<f32>, lang: Language) -> &'static str {
        if let Some(lat) = latency {
            if lat < 1.0 {
                match lang {
                    Language::Spanish => "🟢 Cable Directo (<1 ms) - Máxima velocidad",
                    Language::English => "🟢 Direct Cable (<1 ms) - Max Speed",
                    Language::Chinese => "🟢 直连有线 (<1 ms) - 最高速率",
                    Language::Hindi => "🟢 डायरेक्ट केबल (<1 ms) - अधिकतम गति",
                    Language::Arabic => "🟢 كابل مباشر (<1 ms) - أقصى سرعة",
                    Language::French => "🟢 Câble Direct (<1 ms) - Vitesse max",
                    Language::Bengali => "🟢 সরাসরি কেবল (<1 ms) - সর্বোচ্চ গতি",
                    Language::Portuguese => "🟢 Cabo Direto (<1 ms) - Velocidade máxima",
                    Language::Russian => "🟢 Прямой кабель (<1 ms) - Макс. скорость",
                    Language::Urdu => "🟢 براہ راست کیبل (<1 ms) - زیادہ سے زیادہ رفتار",
                }
            } else if lat < 20.0 {
                match lang {
                    Language::Spanish => "🟢 Wi-Fi Excelente (~5-15 ms) - Señal óptima",
                    Language::English => "🟢 Wi-Fi Excellent (~5-15 ms) - Optimal Signal",
                    Language::Chinese => "🟢 Wi-Fi 极佳 (~5-15 ms) - 信号优异",
                    Language::Hindi => "🟢 वाई-फाई उत्कृष्ट (~5-15 ms) - इष्टतम सिग्नल",
                    Language::Arabic => "🟢 واي فاي ممتاز (~5-15 ms) - إشارة مثالية",
                    Language::French => "🟢 Wi-Fi Excellent (~5-15 ms) - Signal optimal",
                    Language::Bengali => "🟢 ওয়াই-ফাই চমৎকার (~5-15 ms) - সর্বোত্তম সিগন্যাল",
                    Language::Portuguese => "🟢 Wi-Fi Excelente (~5-15 ms) - Sinal ideal",
                    Language::Russian => "🟢 Wi-Fi Отлично (~5-15 ms) - Оптимальный сигнал",
                    Language::Urdu => "🟢 وائی فائی بہترین (~5-15 ms) - بہترین سگنل",
                }
            } else if lat < 70.0 {
                match lang {
                    Language::Spanish => "🟡 Wi-Fi Bueno (~30-60 ms) - Conexión estable",
                    Language::English => "🟡 Wi-Fi Good (~30-60 ms) - Stable Connection",
                    Language::Chinese => "🟡 Wi-Fi 良好 (~30-60 ms) - 连接稳定",
                    Language::Hindi => "🟡 वाई-फाई अच्छा (~30-60 ms) - स्थिर कनेक्शन",
                    Language::Arabic => "🟡 واي فاي جيد (~30-60 ms) - اتصال مستقر",
                    Language::French => "🟡 Wi-Fi Bon (~30-60 ms) - Connexion stable",
                    Language::Bengali => "🟡 ওয়াই-ফাই ভাল (~30-60 ms) - স্থিতিশীল সংযোগ",
                    Language::Portuguese => "🟡 Wi-Fi Bom (~30-60 ms) - Conexão estável",
                    Language::Russian => "🟡 Wi-Fi Хорошо (~30-60 ms) - Стабильная связь",
                    Language::Urdu => "🟡 وائی فائی اچھا (~30-60 ms) - مستحکم کنکشن",
                }
            } else if lat < 140.0 {
                match lang {
                    Language::Spanish => "🟠 Wi-Fi Moderado (~80-120 ms) - Reposo / Lejano",
                    Language::English => "🟠 Wi-Fi Moderate (~80-120 ms) - Idle / Distant",
                    Language::Chinese => "🟠 Wi-Fi 一般 (~80-120 ms) - 待机/距离远",
                    Language::Hindi => "🟠 वाई-फाई मध्यम (~80-120 ms) - दूरस्थ",
                    Language::Arabic => "🟠 واي فاي متوسط (~80-120 ms) - بعيد / خامل",
                    Language::French => "🟠 Wi-Fi Moyen (~80-120 ms) - Éloigné / Veille",
                    Language::Bengali => "🟠 ওয়াই-ফাই মাঝারি (~80-120 ms) - নিষ্ক্রিয় / দূরবর্তী",
                    Language::Portuguese => "🟠 Wi-Fi Moderado (~80-120 ms) - Distante",
                    Language::Russian => "🟠 Wi-Fi Средне (~80-120 ms) - Далеко / Спящий",
                    Language::Urdu => "🟠 وائی فائی درمیانہ (~80-120 ms) - دور",
                }
            } else {
                match lang {
                    Language::Spanish => "🔴 Señal Débil (>150 ms) - Posible pérdida",
                    Language::English => "🔴 Weak Signal (>150 ms) - Possible Loss",
                    Language::Chinese => "🔴 信号较弱 (>150 ms) - 可能丢包",
                    Language::Hindi => "🔴 कमजोर सिग्नल (>150 ms) - पैकेट हानि संभव",
                    Language::Arabic => "🔴 إشارة ضعيفة (>150 ms) - احتمال فقدان حزم",
                    Language::French => "🔴 Signal Faible (>150 ms) - Perte possible",
                    Language::Bengali => "🔴 দুর্বল সিগন্যাল (>150 ms) - প্যাকেট হারানোর সম্ভাবনা",
                    Language::Portuguese => "🔴 Sinal Fraco (>150 ms) - Possível perda",
                    Language::Russian => "🔴 Слабый сигнал (>150 ms) - Возможна потеря",
                    Language::Urdu => "🔴 کمزور سگنل (>150 ms) - پیکٹ نقصان کا امکان",
                }
            }
        } else {
            match lang {
                Language::Spanish => "⚪ Sin respuesta a paquetes ICMP (Ping)",
                Language::English => "⚪ No response to ICMP packets (Ping)",
                Language::Chinese => "⚪ 无 ICMP 数据包响应 (Ping)",
                Language::Hindi => "⚪ आईसीएमपी पैकेट का कोई उत्तर नहीं",
                Language::Arabic => "⚪ لا استجابة لحزم ICMP (Ping)",
                Language::French => "⚪ Aucune réponse aux paquets ICMP (Ping)",
                Language::Bengali => "⚪ কোনো ICMP উত্তর পাওয়া যায়নি",
                Language::Portuguese => "⚪ Sem resposta a pacotes ICMP (Ping)",
                Language::Russian => "⚪ Нет ответа на ICMP-пакеты (Ping)",
                Language::Urdu => "⚪ ICMP پیکٹس کا کوئی جواب نہیں",
            }
        }
    }

    pub fn spec_interface(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Interfaz de Red:",
            Language::English => "Network Interface:",
            Language::Chinese => "网络接口：",
            Language::Hindi => "नेटवर्क इंटरफ़ेस:",
            Language::Arabic => "واجهة الشبكة:",
            Language::French => "Interface Réseau :",
            Language::Bengali => "নেটওয়ার্ক ইন্টারফেস:",
            Language::Portuguese => "Interface de Rede:",
            Language::Russian => "Сетевой интерфейс:",
            Language::Urdu => "نیٹ ورک انٹرفیس:",
        }
    }

    pub fn spec_status(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Estado:",
            Language::English => "Status:",
            Language::Chinese => "在线状态：",
            Language::Hindi => "स्थिति:",
            Language::Arabic => "الحالة:",
            Language::French => "Statut :",
            Language::Bengali => "অবস্থা:",
            Language::Portuguese => "Estado:",
            Language::Russian => "Статус:",
            Language::Urdu => "حالت:",
        }
    }

    pub fn status_label(status: DeviceStatus, lang: Language) -> &'static str {
        match (status, lang) {
            (DeviceStatus::Online, Language::Spanish) => "En línea",
            (DeviceStatus::Online, Language::English) => "Online",
            (DeviceStatus::Online, Language::Chinese) => "在线",
            (DeviceStatus::Online, Language::Hindi) => "ऑनलाइन",
            (DeviceStatus::Online, Language::Arabic) => "متصل",
            (DeviceStatus::Online, Language::French) => "En ligne",
            (DeviceStatus::Online, Language::Bengali) => "অনলাইন",
            (DeviceStatus::Online, Language::Portuguese) => "Online",
            (DeviceStatus::Online, Language::Russian) => "В сети",
            (DeviceStatus::Online, Language::Urdu) => "آن لائن",

            (DeviceStatus::Degraded, Language::Spanish) => "Inestable",
            (DeviceStatus::Degraded, Language::English) => "Degraded",
            (DeviceStatus::Degraded, Language::Chinese) => "不稳定",
            (DeviceStatus::Degraded, Language::Hindi) => "अस्थिर",
            (DeviceStatus::Degraded, Language::Arabic) => "غير مستقر",
            (DeviceStatus::Degraded, Language::French) => "Instable",
            (DeviceStatus::Degraded, Language::Bengali) => "অস্থির",
            (DeviceStatus::Degraded, Language::Portuguese) => "Instável",
            (DeviceStatus::Degraded, Language::Russian) => "Нестабильно",
            (DeviceStatus::Degraded, Language::Urdu) => "غیر مستحکم",

            (DeviceStatus::Offline, Language::Spanish) => "Desconectado",
            (DeviceStatus::Offline, Language::English) => "Offline",
            (DeviceStatus::Offline, Language::Chinese) => "离线",
            (DeviceStatus::Offline, Language::Hindi) => "ऑफ़लाइन",
            (DeviceStatus::Offline, Language::Arabic) => "غير متصل",
            (DeviceStatus::Offline, Language::French) => "Déconnecté",
            (DeviceStatus::Offline, Language::Bengali) => "অফলাইন",
            (DeviceStatus::Offline, Language::Portuguese) => "Desconectado",
            (DeviceStatus::Offline, Language::Russian) => "Не в сети",
            (DeviceStatus::Offline, Language::Urdu) => "آف لائن",
        }
    }

    pub fn spec_last_seen(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Última respuesta:",
            Language::English => "Last Response:",
            Language::Chinese => "最后响应时间：",
            Language::Hindi => "अंतिम प्रतिक्रिया:",
            Language::Arabic => "آخر استجابة:",
            Language::French => "Dernière réponse :",
            Language::Bengali => "সর্বশেষ প্রতিক্রিয়া:",
            Language::Portuguese => "Última resposta:",
            Language::Russian => "Последний ответ:",
            Language::Urdu => "آخری جواب:",
        }
    }

    pub fn action_open_web(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🌐 Abrir Web",
            Language::English => "🌐 Open Web",
            Language::Chinese => "🌐 打开网页",
            Language::Hindi => "🌐 वेब खोलें",
            Language::Arabic => "🌐 فتح الويب",
            Language::French => "🌐 Ouvrir Web",
            Language::Bengali => "🌐 ওয়েব খুলুন",
            Language::Portuguese => "🌐 Abrir Web",
            Language::Russian => "🌐 Открыть веб",
            Language::Urdu => "🌐 ویب کھولیں",
        }
    }

    pub fn section_latency(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "LATENCIA & CONECTIVIDAD",
            Language::English => "LATENCY & CONNECTIVITY",
            Language::Chinese => "延迟与连通性",
            Language::Hindi => "विलंबता और कनेक्टिविटी",
            Language::Arabic => "زمن الاستجابة والاتصال",
            Language::French => "LATENCE & CONNECTIVITÉ",
            Language::Bengali => "লেটেন্সি ও সংযোগ",
            Language::Portuguese => "LATÊNCIA E CONECTIVIDADE",
            Language::Russian => "ЗАДЕРЖКА И СВЯЗЬ",
            Language::Urdu => "تاخیر اور کنیکٹیویٹی",
        }
    }

    pub fn btn_ping_test(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⚡ Test de Ping",
            Language::English => "⚡ Ping Test",
            Language::Chinese => "⚡ Ping 测试",
            Language::Hindi => "⚡ पिंग परीक्षण",
            Language::Arabic => "⚡ اختبار Ping",
            Language::French => "⚡ Test de Ping",
            Language::Bengali => "⚡ পিং টেস্ট",
            Language::Portuguese => "⚡ Teste de Ping",
            Language::Russian => "⚡ Тест Ping",
            Language::Urdu => "⚡ پنگ ٹیسٹ",
        }
    }

    pub fn btn_measuring(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⏳ Midiendo...",
            Language::English => "⏳ Measuring...",
            Language::Chinese => "⏳ 正在测量...",
            Language::Hindi => "⏳ मापन जारी...",
            Language::Arabic => "⏳ جاري القياس...",
            Language::French => "⏳ Mesure en cours...",
            Language::Bengali => "⏳ পরিমাপ হচ্ছে...",
            Language::Portuguese => "⏳ Medindo...",
            Language::Russian => "⏳ Измерение...",
            Language::Urdu => "⏳ پیمائش جاری...",
        }
    }

    pub fn ping_sent(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Enviados:",
            Language::English => "Sent:",
            Language::Chinese => "已发送：",
            Language::Hindi => "भेजे गए:",
            Language::Arabic => "تم الإرسال:",
            Language::French => "Envoyés :",
            Language::Bengali => "প্রেরিত:",
            Language::Portuguese => "Enviados:",
            Language::Russian => "Отправлено:",
            Language::Urdu => "بھیجے گئے:",
        }
    }

    pub fn ping_received(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Recibidos:",
            Language::English => "Received:",
            Language::Chinese => "已接收：",
            Language::Hindi => "प्राप्त:",
            Language::Arabic => "تم الاستلام:",
            Language::French => "Reçus :",
            Language::Bengali => "গৃহীত:",
            Language::Portuguese => "Recebidos:",
            Language::Russian => "Получено:",
            Language::Urdu => "وصول:",
        }
    }

    pub fn ping_loss(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Pérdida:",
            Language::English => "Loss:",
            Language::Chinese => "丢包率：",
            Language::Hindi => "हानि:",
            Language::Arabic => "الفقدان:",
            Language::French => "Perte :",
            Language::Bengali => "ক্ষতি:",
            Language::Portuguese => "Perda:",
            Language::Russian => "Потери:",
            Language::Urdu => "نقصان:",
        }
    }

    pub fn ping_min(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Mín:",
            Language::English => "Min:",
            Language::Chinese => "最小：",
            Language::Hindi => "न्यूनतम:",
            Language::Arabic => "الأدنى:",
            Language::French => "Min :",
            Language::Bengali => "সর্বনিম্ন:",
            Language::Portuguese => "Mín:",
            Language::Russian => "Мин:",
            Language::Urdu => "کم از کم:",
        }
    }

    pub fn ping_avg(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Promedio:",
            Language::English => "Avg:",
            Language::Chinese => "平均：",
            Language::Hindi => "औसत:",
            Language::Arabic => "المتوسط:",
            Language::French => "Moy :",
            Language::Bengali => "গড়:",
            Language::Portuguese => "Média:",
            Language::Russian => "Среднее:",
            Language::Urdu => "اوسط:",
        }
    }

    pub fn ping_max(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Máx:",
            Language::English => "Max:",
            Language::Chinese => "最大：",
            Language::Hindi => "अधिकतम:",
            Language::Arabic => "الأقصى:",
            Language::French => "Max :",
            Language::Bengali => "সর্বোচ্চ:",
            Language::Portuguese => "Máx:",
            Language::Russian => "Макс:",
            Language::Urdu => "زیادہ سے زیادہ:",
        }
    }

    pub fn section_ports(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "SERVICIOS Y PUERTOS",
            Language::English => "SERVICES & PORTS",
            Language::Chinese => "网络服务与端口",
            Language::Hindi => "सेवाएं और पोर्ट",
            Language::Arabic => "الخدمات والمنافذ",
            Language::French => "SERVICES & PORTS",
            Language::Bengali => "পরিষেবা ও পোর্ট",
            Language::Portuguese => "SERVIÇOS E PORTAS",
            Language::Russian => "СЛУЖБЫ И ПОРТЫ",
            Language::Urdu => "خدمات اور پورٹس",
        }
    }

    pub fn btn_scan_ports(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🔍 Escanear Puertos",
            Language::English => "🔍 Scan Ports",
            Language::Chinese => "🔍 扫描端口",
            Language::Hindi => "🔍 पोर्ट स्कैन करें",
            Language::Arabic => "🔍 فحص المنافذ",
            Language::French => "🔍 Scanner les Ports",
            Language::Bengali => "🔍 পোর্ট স্ক্যান",
            Language::Portuguese => "🔍 Escanear Portas",
            Language::Russian => "🔍 Сканировать порты",
            Language::Urdu => "🔍 پورٹس اسکین کریں",
        }
    }

    pub fn btn_scanning_ports(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⏳ Escaneando 30...",
            Language::English => "⏳ Scanning 30...",
            Language::Chinese => "⏳ 正在扫描 30 个端口...",
            Language::Hindi => "⏳ स्कैनिंग 30...",
            Language::Arabic => "⏳ جاري فحص 30...",
            Language::French => "⏳ Scan de 30 ports...",
            Language::Bengali => "⏳ ৩০টি স্ক্যান হচ্ছে...",
            Language::Portuguese => "⏳ Escaneando 30...",
            Language::Russian => "⏳ Сканирование 30...",
            Language::Urdu => "⏳ ۳۰ اسکین ہو رہے ہیں...",
        }
    }

    pub fn ports_checking(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Comprobando puertos comunes en el host...",
            Language::English => "Checking common ports on host...",
            Language::Chinese => "正在探测主机常见端口...",
            Language::Hindi => "होस्ट पर सामान्य पोर्ट जांचे जा रहे हैं...",
            Language::Arabic => "جاري التحقق من المنافذ الشائعة...",
            Language::French => "Vérification des ports courants...",
            Language::Bengali => "হোস্টে সাধারণ পোর্ট চেক করা হচ্ছে...",
            Language::Portuguese => "Verificando portas comuns no host...",
            Language::Russian => "Проверка стандартных портов...",
            Language::Urdu => "ہوسٹ پر عام پورٹس کی جانچ جاری ہے...",
        }
    }

    pub fn no_open_ports(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Ningún puerto común detectado como abierto.",
            Language::English => "No common open ports detected.",
            Language::Chinese => "未检测到开放的常见端口。",
            Language::Hindi => "कोई सामान्य खुला पोर्ट नहीं मिला।",
            Language::Arabic => "لم يتم العثور على منافذ مفتوحة.",
            Language::French => "Aucun port courant détecté ouvert.",
            Language::Bengali => "কোনো উন্মুক্ত সাধারণ পোর্ট পাওয়া যায়নি।",
            Language::Portuguese => "Nenhuma porta comum aberta detectada.",
            Language::Russian => "Открытых стандартных портов не обнаружено.",
            Language::Urdu => "کوئی عام کھلا پورٹ نہیں ملا۔",
        }
    }

    pub fn empty_scanning(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Escaneando la red en segundo plano... Los dispositivos aparecerán aquí pronto.",
            Language::English => "Scanning the network in background... Devices will appear here soon.",
            Language::Chinese => "正在后台扫描网络... 发现的设备稍后将在此显示。",
            Language::Hindi => "पृष्ठभूमि में नेटवर्क स्कैन किया जा रहा है... उपकरण जल्द ही यहां दिखाई देंगे।",
            Language::Arabic => "جاري فحص الشبكة في الخلفية... ستظهر الأجهزة هنا قريباً.",
            Language::French => "Scan du réseau en arrière-plan... Les appareils apparaîtront bientôt.",
            Language::Bengali => "পটভূমিতে নেটওয়ার্ক স্ক্যান করা হচ্ছে... শীঘ্রই ডিভাইসগুলি প্রদর্শিত হবে।",
            Language::Portuguese => "Escaneando a rede em segundo plano... Os dispositivos aparecerão em breve.",
            Language::Russian => "Сканирование сети в фоновом режиме... Устройства скоро появятся.",
            Language::Urdu => "پس منظر میں نیٹ ورک اسکین ہو رہا ہے... جلد آلات یہاں نظر آئیں گے۔",
        }
    }

    pub fn empty_filtered(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "No hay dispositivos que coincidan con los filtros seleccionados.",
            Language::English => "No devices match the selected filters.",
            Language::Chinese => "没有找到匹配当前筛选条件的设备。",
            Language::Hindi => "चयनित फ़िल्टर से कोई उपकरण मेल नहीं खाता।",
            Language::Arabic => "لا توجد أجهزة مطابقة لعوامل التصفية المحددة.",
            Language::French => "Aucun appareil ne correspond aux filtres sélectionnés.",
            Language::Bengali => "নির্বাচিত ফিল্টারের সাথে কোনো ডিভাইস মিলছে না।",
            Language::Portuguese => "Nenhum dispositivo corresponde aos filtros selecionados.",
            Language::Russian => "Нет устройств, соответствующих фильтрам.",
            Language::Urdu => "منتخب کردہ فلٹرز سے کوئی آلہ مطابقت نہیں رکھتا۔",
        }
    }

    pub fn empty_initial(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "No se han detectado dispositivos aún. Haz clic en 'Escanear Red'.",
            Language::English => "No devices detected yet. Click 'Scan Network'.",
            Language::Chinese => "尚未检测到设备，请点击“扫描网络”。",
            Language::Hindi => "अभी तक कोई उपकरण नहीं मिला। 'नेटवर्क स्कैन करें' पर क्लिक करें।",
            Language::Arabic => "لم يتم اكتشاف أي أجهزة بعد. انقر على 'فحص الشبكة'.",
            Language::French => "Aucun appareil détecté pour l'instant. Cliquez sur 'Scanner le Réseau'.",
            Language::Bengali => "এখনও কোনো ডিভাইস শনাক্ত হয়নি। 'নেটওয়ার্ক স্ক্যান' ক্লিক করুন।",
            Language::Portuguese => "Nenhum dispositivo detectado ainda. Clique em 'Escanear Rede'.",
            Language::Russian => "Устройства еще не обнаружены. Нажмите 'Сканировать сеть'.",
            Language::Urdu => "ابھی تک کوئی آلہ نہیں ملا۔ 'نیٹ ورک اسکین کریں' پر کلک کریں۔",
        }
    }

    pub fn wifi_quality_label(percent: u8, lang: Language) -> &'static str {
        if percent >= 80 {
            match lang {
                Language::Spanish => "Excelente",
                Language::English => "Excellent",
                Language::Chinese => "极好",
                Language::Hindi => "उत्कृष्ट",
                Language::Arabic => "ممتاز",
                Language::French => "Excellent",
                Language::Bengali => "চমৎকার",
                Language::Portuguese => "Excelente",
                Language::Russian => "Отлично",
                Language::Urdu => "بہترین",
            }
        } else if percent >= 60 {
            match lang {
                Language::Spanish => "Buena",
                Language::English => "Good",
                Language::Chinese => "良好",
                Language::Hindi => "अच्छा",
                Language::Arabic => "جيد",
                Language::French => "Bon",
                Language::Bengali => "ভাল",
                Language::Portuguese => "Boa",
                Language::Russian => "Хорошо",
                Language::Urdu => "اچھا",
            }
        } else if percent >= 40 {
            match lang {
                Language::Spanish => "Moderada",
                Language::English => "Moderate",
                Language::Chinese => "中等",
                Language::Hindi => "मध्यम",
                Language::Arabic => "متوسط",
                Language::French => "Moyen",
                Language::Bengali => "মাঝারি",
                Language::Portuguese => "Média",
                Language::Russian => "Средне",
                Language::Urdu => "درمیانہ",
            }
        } else if percent >= 20 {
            match lang {
                Language::Spanish => "Baja",
                Language::English => "Weak",
                Language::Chinese => "较弱",
                Language::Hindi => "कमजोर",
                Language::Arabic => "ضعيف",
                Language::French => "Faible",
                Language::Bengali => "দুর্বল",
                Language::Portuguese => "Fraca",
                Language::Russian => "Слабо",
                Language::Urdu => "کمزور",
            }
        } else {
            match lang {
                Language::Spanish => "Muy Débil",
                Language::English => "Very Weak",
                Language::Chinese => "极弱",
                Language::Hindi => "बहुत कमजोर",
                Language::Arabic => "ضعيف جداً",
                Language::French => "Très faible",
                Language::Bengali => "খুব দুর্বল",
                Language::Portuguese => "Muito fraca",
                Language::Russian => "Очень слабо",
                Language::Urdu => "بہت کمزور",
            }
        }
    }

    pub fn device_subtitle(device: &Device, lang: Language) -> String {
        let mut parts = Vec::new();
        if device.is_gateway {
            parts.push(match lang {
                Language::Spanish => "Puerta de Enlace Predeterminada",
                Language::English => "Default Gateway",
                Language::Chinese => "默认网关",
                Language::Hindi => "डिफ़ॉल्ट गेटवे",
                Language::Arabic => "البوابة الافتراضية",
                Language::French => "Passerelle par Défaut",
                Language::Bengali => "ডিফল্ট গেটওয়ে",
                Language::Portuguese => "Gateway Padrão",
                Language::Russian => "Шлюз по умолчанию",
                Language::Urdu => "طے شدہ گیٹ وے",
            });
        }
        if device.is_current_host {
            parts.push(match lang {
                Language::Spanish => "Este Equipo (Host Local)",
                Language::English => "This Device (Local Host)",
                Language::Chinese => "本机 (本地主机)",
                Language::Hindi => "यह उपकरण (स्थानीय होस्ट)",
                Language::Arabic => "هذا الجهاز (المضيف المحلي)",
                Language::French => "Cet Appareil (Hôte Local)",
                Language::Bengali => "এই ডিভাইস (স্থানীয় হোস্ট)",
                Language::Portuguese => "Este Dispositivo (Host Local)",
                Language::Russian => "Это устройство (локальный хост)",
                Language::Urdu => "یہ ڈیوائس (مقامی ہوسٹ)",
            });
        }
        if !device.vendor.is_empty() && device.vendor != "Desconocido" {
            parts.push(&device.vendor);
        } else if device.is_randomized_mac {
            parts.push(Self::mac_type_private(lang));
        }
        if parts.is_empty() {
            parts.push(match lang {
                Language::Spanish => "Fabricante no identificado",
                Language::English => "Unidentified Manufacturer",
                Language::Chinese => "厂商未识别",
                Language::Hindi => "अज्ञात निर्माता",
                Language::Arabic => "جهة تصنيع غير محددة",
                Language::French => "Fabricant non identifié",
                Language::Bengali => "অজ্ঞাত প্রস্তুতকারক",
                Language::Portuguese => "Fabricante não identificado",
                Language::Russian => "Неопознанный производитель",
                Language::Urdu => "نامعلوم مینوفیکچرر",
            });
        }
        parts.join(" • ")
    }
}
