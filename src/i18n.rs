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

    pub fn port_description(port: u16, lang: Language) -> &'static str {
        match (port, lang) {
            (21, Language::Spanish) => "Transferencia de archivos sin cifrar",
            (21, Language::English) => "Unencrypted file transfer protocol",
            (21, Language::Chinese) => "未加密文件传输协议",
            (21, Language::Hindi) => "असुरक्षित फ़ाइल स्थानांतरण",
            (21, Language::Arabic) => "بروتوكول نقل الملفات غير المشفر",
            (21, Language::French) => "Transfert de fichiers non chiffré",
            (21, Language::Bengali) => "এনক্রিপ্টহীন ফাইল ট্রান্সফার",
            (21, Language::Portuguese) => "Transferência de arquivos sem criptografia",
            (21, Language::Russian) => "Передача файлов без шифрования",
            (21, Language::Urdu) => "غیر مرموز فائل ٹرانسفر پروٹوکول",

            (22, Language::Spanish) => "Terminal remota segura (Linux/Unix/Mac)",
            (22, Language::English) => "Secure remote shell access (Linux/Unix/Mac)",
            (22, Language::Chinese) => "安全远程终端访问 (Linux/Unix/Mac)",
            (22, Language::Hindi) => "सुरक्षित रिमोट शेल (Linux/Unix/Mac)",
            (22, Language::Arabic) => "وصول بعيد آمن عبر الطرفية (Linux/Unix/Mac)",
            (22, Language::French) => "Accès distant sécurisé (Linux/Unix/Mac)",
            (22, Language::Bengali) => "নিরাপদ রিমোট শেল (Linux/Unix/Mac)",
            (22, Language::Portuguese) => "Acesso remoto seguro por terminal",
            (22, Language::Russian) => "Безопасный удаленный терминал (Linux/Unix/Mac)",
            (22, Language::Urdu) => "محفوظ ریموٹ شیل (Linux/Unix/Mac)",

            (23, Language::Spanish) => "Acceso por terminal sin cifrar",
            (23, Language::English) => "Unencrypted remote terminal access",
            (23, Language::Chinese) => "未加密远程终端访问",
            (23, Language::Hindi) => "असुरक्षित टर्मिनल एक्सेस",
            (23, Language::Arabic) => "وصول عبر الطرفية غير مشفر",
            (23, Language::French) => "Terminal distant non chiffré",
            (23, Language::Bengali) => "এনক্রিপ্টহীন রিমোট টার্মিনাল",
            (23, Language::Portuguese) => "Terminal remoto sem criptografia",
            (23, Language::Russian) => "Нешифрованный удаленный терминал",
            (23, Language::Urdu) => "غیر مرموز ریموٹ ٹرمینل",

            (25, Language::Spanish) => "Servidor de correo saliente (SMTP)",
            (25, Language::English) => "Outgoing email server (SMTP)",
            (25, Language::Chinese) => "外发邮件服务器 (SMTP)",
            (25, Language::Hindi) => "आउटगोइंग ईमेल सर्वर (SMTP)",
            (25, Language::Arabic) => "خادم البريد الصادر (SMTP)",
            (25, Language::French) => "Serveur de messagerie sortant (SMTP)",
            (25, Language::Bengali) => "আউটগোয়িং ইমেইল সার্ভার (SMTP)",
            (25, Language::Portuguese) => "Servidor de e-mail de saída (SMTP)",
            (25, Language::Russian) => "Сервер исходящей почты (SMTP)",
            (25, Language::Urdu) => "آؤٹ گوئنگ ای میل سرور (SMTP)",

            (53, Language::Spanish) => "Resolución de nombres de dominio (DNS)",
            (53, Language::English) => "Domain Name System resolution (DNS)",
            (53, Language::Chinese) => "域名解析系统服务 (DNS)",
            (53, Language::Hindi) => "डोमेन नाम प्रणाली रिज़ॉल्यूशन (DNS)",
            (53, Language::Arabic) => "دليل نظام أسماء النطاقات (DNS)",
            (53, Language::French) => "Résolution des noms de domaine (DNS)",
            (53, Language::Bengali) => "ডোমেন নেম সিস্টেম রেজোলিউশন (DNS)",
            (53, Language::Portuguese) => "Resolução de nomes de domínio (DNS)",
            (53, Language::Russian) => "Разрешение доменных имен (DNS)",
            (53, Language::Urdu) => "ڈومین نیم سسٹم ریزولوشن (DNS)",

            (80, Language::Spanish) => "Servidor web / Panel de administración (HTTP)",
            (80, Language::English) => "Web server / Admin dashboard (HTTP)",
            (80, Language::Chinese) => "Web 服务器 / 管理后台面板 (HTTP)",
            (80, Language::Hindi) => "वेब सर्वर / एडमिन डैशबोर्ड (HTTP)",
            (80, Language::Arabic) => "خادم ويب / لوحة الإدارة (HTTP)",
            (80, Language::French) => "Serveur Web / Panneau d'administration (HTTP)",
            (80, Language::Bengali) => "ওয়েব সার্ভার / অ্যাডমিন ড্যাশবোর্ড (HTTP)",
            (80, Language::Portuguese) => "Servidor Web / Painel de administração (HTTP)",
            (80, Language::Russian) => "Веб-сервер / Панель управления (HTTP)",
            (80, Language::Urdu) => "ویب سرور / ایڈمن ڈیش بورڈ (HTTP)",

            (110, Language::Spanish) => "Recepción de correo electrónico (POP3)",
            (110, Language::English) => "Incoming POP3 email retrieval",
            (110, Language::Chinese) => "POP3 接收邮件服务",
            (110, Language::Hindi) => "पीओपी3 ईमेल प्राप्ति",
            (110, Language::Arabic) => "استلام البريد الإلكتروني POP3",
            (110, Language::French) => "Réception de courrier POP3",
            (110, Language::Bengali) => "পিওপি৩ ইমেইল গ্রহণ",
            (110, Language::Portuguese) => "Recebimento de e-mail POP3",
            (110, Language::Russian) => "Получение почты по протоколу POP3",
            (110, Language::Urdu) => "POP3 ای میل وصولی",

            (135, Language::Spanish) => "Llamada a procedimiento remoto de Windows (MS-RPC)",
            (135, Language::English) => "Microsoft Windows RPC service",
            (135, Language::Chinese) => "Windows 远程过程调用服务 (RPC)",
            (135, Language::Hindi) => "विंडोज़ आरपीसी सेवा",
            (135, Language::Arabic) => "خدمة استدعاء الإجراء البعيد لـ Windows",
            (135, Language::French) => "Appel de procédure distante Windows",
            (135, Language::Bengali) => "উইন্ডোজ আরপিসি সার্ভিস",
            (135, Language::Portuguese) => "Chamada de procedimento remoto Windows",
            (135, Language::Russian) => "Служба удаленного вызова процедур Windows",
            (135, Language::Urdu) => "ونڈوز ریموٹ پروسیجر کال سروس",

            (139, Language::Spanish) => "Servicio de sesión NetBIOS",
            (139, Language::English) => "NetBIOS Session Service",
            (139, Language::Chinese) => "NetBIOS 会话服务",
            (139, Language::Hindi) => "नेटबायोस सत्र सेवा",
            (139, Language::Arabic) => "خدمة جلسة NetBIOS",
            (139, Language::French) => "Service de session NetBIOS",
            (139, Language::Bengali) => "নেটবায়োস সেশন সার্ভিস",
            (139, Language::Portuguese) => "Serviço de sessão NetBIOS",
            (139, Language::Russian) => "Служба сеансов NetBIOS",
            (139, Language::Urdu) => "نیٹ بائیوس سیشن سروس",

            (143, Language::Spanish) => "Acceso a buzón de correo IMAP",
            (143, Language::English) => "IMAP email mailbox access",
            (143, Language::Chinese) => "IMAP 邮箱访问服务",
            (143, Language::Hindi) => "आईमैप ईमेल एक्सेस",
            (143, Language::Arabic) => "الوصول إلى صندوق بريد IMAP",
            (143, Language::French) => "Accès à la boîte aux lettres IMAP",
            (143, Language::Bengali) => "আইম্যাপ ইমেইল অ্যাক্সেস",
            (143, Language::Portuguese) => "Acesso à caixa de correio IMAP",
            (143, Language::Russian) => "Доступ к почтовому ящику IMAP",
            (143, Language::Urdu) => "IMAP ای میل باکس تک رسائی",

            (443, Language::Spanish) => "Servidor web seguro cifrado (HTTPS)",
            (443, Language::English) => "Secure encrypted web server (HTTPS)",
            (443, Language::Chinese) => "安全加密 Web 服务器 (HTTPS)",
            (443, Language::Hindi) => "सुरक्षित एन्क्रिप्टेड वेब सर्वर (HTTPS)",
            (443, Language::Arabic) => "خادم ويب مشفر وآمن (HTTPS)",
            (443, Language::French) => "Serveur Web sécurisé et chiffré (HTTPS)",
            (443, Language::Bengali) => "নিরাপদ এনক্রিপ্ট করা ওয়েব সার্ভার (HTTPS)",
            (443, Language::Portuguese) => "Servidor Web seguro criptografado (HTTPS)",
            (443, Language::Russian) => "Защищенный веб-сервер (HTTPS)",
            (443, Language::Urdu) => "محفوظ انکرپٹڈ ویب سرور (HTTPS)",

            (445, Language::Spanish) => "Compartición de archivos Windows / Samba (SMB)",
            (445, Language::English) => "Windows / Samba network file sharing (SMB)",
            (445, Language::Chinese) => "Windows / Samba 网络共享服务 (SMB)",
            (445, Language::Hindi) => "विंडोज़ / सांबा फ़ाइल शेयरिंग (SMB)",
            (445, Language::Arabic) => "مشاركة ملفات Windows / Samba (SMB)",
            (445, Language::French) => "Partage de fichiers Windows / Samba (SMB)",
            (445, Language::Bengali) => "উইন্ডোজ / সাম্বা ফাইল শেয়ারিং (SMB)",
            (445, Language::Portuguese) => "Compartilhamento de arquivos Windows / Samba (SMB)",
            (445, Language::Russian) => "Общий доступ к файлам Windows / Samba (SMB)",
            (445, Language::Urdu) => "ونڈوز / سامبا فائل شیئرنگ (SMB)",

            (548, Language::Spanish) => "Apple Filing Protocol para macOS (AFP)",
            (548, Language::English) => "Apple Filing Protocol (macOS file share)",
            (548, Language::Chinese) => "苹果文件共享协议 (Apple AFP)",
            (548, Language::Hindi) => "एप्पल फाइलिंग प्रोटोकॉल (macOS)",
            (548, Language::Arabic) => "بروتوكول مشاركة ملفات Apple (macOS)",
            (548, Language::French) => "Protocole de partage de fichiers Apple (macOS)",
            (548, Language::Bengali) => "অ্যাপল ফাইল শেয়ারিং প্রোটোকল",
            (548, Language::Portuguese) => "Compartilhamento de arquivos Apple (macOS)",
            (548, Language::Russian) => "Файловый протокол Apple для macOS (AFP)",
            (548, Language::Urdu) => "ایپل فائل شیئرنگ پروٹوکول (macOS)",

            (554, Language::Spanish) => "Streaming de video / Cámaras IP (RTSP)",
            (554, Language::English) => "RTSP real-time video stream / IP camera",
            (554, Language::Chinese) => "RTSP 实时流媒体 / IP 监控摄像头",
            (554, Language::Hindi) => "आरटीएसपी वीडियो स्ट्रीमिंग / आईपी कैमरा",
            (554, Language::Arabic) => "بث فيديو RTSP / كاميرات IP",
            (554, Language::French) => "Flux vidéo en direct RTSP / Caméra IP",
            (554, Language::Bengali) => "আরটিএসপি ভিডিও স্ট্রিমিং / আইপি ক্যামেরা",
            (554, Language::Portuguese) => "Streaming de vídeo RTSP / Câmera IP",
            (554, Language::Russian) => "Потоковое видео RTSP / IP-камеры",
            (554, Language::Urdu) => "RTSP ریئل ٹائم ویڈیو اسٹریم / IP کیمرہ",

            (631, Language::Spanish) => "Servidor de impresión de red (CUPS/IPP)",
            (631, Language::English) => "Network print service (CUPS/IPP)",
            (631, Language::Chinese) => "网络打印服务 (CUPS/IPP)",
            (631, Language::Hindi) => "नेटवर्क प्रिंट सेवा (CUPS/IPP)",
            (631, Language::Arabic) => "خدمة الطباعة عبر الشبكة (CUPS/IPP)",
            (631, Language::French) => "Service d'impression réseau (CUPS/IPP)",
            (631, Language::Bengali) => "নেটওয়ার্ক প্রিন্ট সার্ভিস (CUPS/IPP)",
            (631, Language::Portuguese) => "Serviço de impressão em rede (CUPS/IPP)",
            (631, Language::Russian) => "Сетевая служба печати (CUPS/IPP)",
            (631, Language::Urdu) => "نیٹ ورک پرنٹ سروس (CUPS/IPP)",

            (993, Language::Spanish) => "Correo IMAP seguro cifrado (IMAPS)",
            (993, Language::English) => "Secure encrypted IMAP email service",
            (993, Language::Chinese) => "安全加密 IMAPS 邮件服务",
            (993, Language::Hindi) => "सुरक्षित एन्क्रिप्टेड आईमैप ईमेल",
            (993, Language::Arabic) => "خدمة بريد IMAP المشفرة والآمنة",
            (993, Language::French) => "Courrier IMAP sécurisé et chiffré",
            (993, Language::Bengali) => "নিরাপদ এনক্রিপ্ট করা আইম্যাপ ইমেইল",
            (993, Language::Portuguese) => "Serviço de e-mail IMAP seguro criptografado",
            (993, Language::Russian) => "Защищенная почтовая служба IMAPS",
            (993, Language::Urdu) => "محفوظ انکرپٹڈ IMAP سروس",

            (995, Language::Spanish) => "Correo POP3 seguro cifrado (POP3S)",
            (995, Language::English) => "Secure encrypted POP3 email service",
            (995, Language::Chinese) => "安全加密 POP3S 邮件服务",
            (995, Language::Hindi) => "सुरक्षित एन्क्रिप्टेड पीओपी3 ईमेल",
            (995, Language::Arabic) => "خدمة بريد POP3 المشفرة والآمنة",
            (995, Language::French) => "Courrier POP3 sécurisé et chiffré",
            (995, Language::Bengali) => "নিরাপদ এনক্রিপ্ট করা পিওপি৩ ইমেইল",
            (995, Language::Portuguese) => "Serviço de e-mail POP3 seguro criptografado",
            (995, Language::Russian) => "Защищенная почтовая служба POP3S",
            (995, Language::Urdu) => "محفوظ انکرپٹڈ POP3 سروس",

            (1433, Language::Spanish) => "Base de datos Microsoft SQL Server",
            (1433, Language::English) => "Microsoft SQL Server database engine",
            (1433, Language::Chinese) => "Microsoft SQL Server 数据库引擎",
            (1433, Language::Hindi) => "माइक्रोसॉफ्ट एसक्यूएल सर्वर डेटाबेस",
            (1433, Language::Arabic) => "قاعدة بيانات Microsoft SQL Server",
            (1433, Language::French) => "Base de données Microsoft SQL Server",
            (1433, Language::Bengali) => "মাইক্রোসফট এসকিউএল সার্ভার ডেটাবেস",
            (1433, Language::Portuguese) => "Banco de dados Microsoft SQL Server",
            (1433, Language::Russian) => "База данных Microsoft SQL Server",
            (1433, Language::Urdu) => "مائیکروسافٹ ایس کیو ایل سرور ڈیٹا بیس",

            (1723, Language::Spanish) => "Túnel VPN punto a punto (PPTP)",
            (1723, Language::English) => "Point-to-point tunneling VPN (PPTP)",
            (1723, Language::Chinese) => "点对点隧道 VPN (PPTP)",
            (1723, Language::Hindi) => "प्वाइंट-टू-प्वाइंट वीपीएन टनल (PPTP)",
            (1723, Language::Arabic) => "نفق VPN من نقطة إلى نقطة (PPTP)",
            (1723, Language::French) => "Tunnel VPN point-à-point (PPTP)",
            (1723, Language::Bengali) => "পয়েন্ট-টু-পয়েন্ট ভিপিএন টানেল (PPTP)",
            (1723, Language::Portuguese) => "Túnel VPN ponto a ponto (PPTP)",
            (1723, Language::Russian) => "Туннель VPN точка-точка (PPTP)",
            (1723, Language::Urdu) => "پوائنٹ ٹو پوائنٹ وی پی این ٹنل (PPTP)",

            (1900, Language::Spanish) => "Descubrimiento de Smart TVs y multimedia (UPnP)",
            (1900, Language::English) => "UPnP / SSDP media and Smart TV discovery",
            (1900, Language::Chinese) => "UPnP / SSDP 智能电视与多媒体发现",
            (1900, Language::Hindi) => "स्मार्ट टीवी और मीडिया खोज (UPnP)",
            (1900, Language::Arabic) => "اكتشاف أجهزة الوسائط والتلفاز الذكي (UPnP)",
            (1900, Language::French) => "Découverte multimédia et Smart TV (UPnP)",
            (1900, Language::Bengali) => "স্মার্ট টিভি ও মিডিয়া আবিষ্কার (UPnP)",
            (1900, Language::Portuguese) => "Descoberta de mídia e Smart TVs (UPnP)",
            (1900, Language::Russian) => "Обнаружение мультимедиа и Smart TV (UPnP)",
            (1900, Language::Urdu) => "اسمارٹ ٹی وی اور میڈیا ڈسکوری (UPnP)",

            (3306, Language::Spanish) => "Base de datos MySQL / MariaDB",
            (3306, Language::English) => "MySQL / MariaDB database server",
            (3306, Language::Chinese) => "MySQL / MariaDB 数据库服务",
            (3306, Language::Hindi) => "माईएसक्यूएल / मारियाडीबी डेटाबेस",
            (3306, Language::Arabic) => "خادم قاعدة بيانات MySQL / MariaDB",
            (3306, Language::French) => "Serveur de base de données MySQL / MariaDB",
            (3306, Language::Bengali) => "মাইএসকিউএল / মারিয়াডিবি ডেটাবেস",
            (3306, Language::Portuguese) => "Servidor de banco de dados MySQL / MariaDB",
            (3306, Language::Russian) => "Сервер баз данных MySQL / MariaDB",
            (3306, Language::Urdu) => "مائی ایس کیو ایل / ماریا ڈی بی ڈیٹا بیس",

            (3389, Language::Spanish) => "Escritorio Remoto de Windows (RDP)",
            (3389, Language::English) => "Microsoft Windows Remote Desktop (RDP)",
            (3389, Language::Chinese) => "Windows 远程桌面连接 (RDP)",
            (3389, Language::Hindi) => "विंडोज़ रिमोट डेस्कटॉप (RDP)",
            (3389, Language::Arabic) => "سطح المكتب البعيد لـ Windows (RDP)",
            (3389, Language::French) => "Bureau à distance Windows (RDP)",
            (3389, Language::Bengali) => "উইন্ডোজ রিমোট ডেস্কটপ (RDP)",
            (3389, Language::Portuguese) => "Área de Trabalho Remota do Windows (RDP)",
            (3389, Language::Russian) => "Удаленный рабочий стол Windows (RDP)",
            (3389, Language::Urdu) => "ونڈوز ریموٹ ڈیسک ٹاپ (RDP)",

            (5000, Language::Spanish) => "Panel Synology NAS o servidor UPnP",
            (5000, Language::English) => "Synology NAS DSM or UPnP media server",
            (5000, Language::Chinese) => "群晖 Synology NAS 控制面板或 UPnP",
            (5000, Language::Hindi) => "सिनोलॉजी एनएएस या यूपीएनपी सर्वर",
            (5000, Language::Arabic) => "لوحة Synology NAS أو خادم UPnP",
            (5000, Language::French) => "Panneau Synology NAS ou serveur UPnP",
            (5000, Language::Bengali) => "সিনোলজি ন্যাস বা ইউপিএনপি সার্ভার",
            (5000, Language::Portuguese) => "Painel Synology NAS ou servidor UPnP",
            (5000, Language::Russian) => "Панель Synology NAS или UPnP-сервер",
            (5000, Language::Urdu) => "سائنولوجی NAS یا UPnP سرور",

            (5353, Language::Spanish) => "Multicast DNS ZeroConf (Bonjour/Avahi)",
            (5353, Language::English) => "Multicast DNS ZeroConf (Bonjour/Avahi)",
            (5353, Language::Chinese) => "多播 DNS 零配置发现 (Bonjour/Avahi)",
            (5353, Language::Hindi) => "मल्टीकास्ट डीएनएस जीरोकॉन्फ़ (Bonjour/Avahi)",
            (5353, Language::Arabic) => "استكشاف ZeroConf عبر Multicast DNS",
            (5353, Language::French) => "DNS multidiffusion ZeroConf (Bonjour/Avahi)",
            (5353, Language::Bengali) => "মাল্টিকাস্ট ডিএনএস জিরোকনফ",
            (5353, Language::Portuguese) => "Multicast DNS ZeroConf (Bonjour/Avahi)",
            (5353, Language::Russian) => "Многоадресный DNS ZeroConf (Bonjour/Avahi)",
            (5353, Language::Urdu) => "ملٹی کاسٹ DNS زیرو کنفگریشن",

            (5432, Language::Spanish) => "Base de datos relacional PostgreSQL",
            (5432, Language::English) => "PostgreSQL relational database server",
            (5432, Language::Chinese) => "PostgreSQL 关系型数据库服务",
            (5432, Language::Hindi) => "पोस्टग्रेएसक्यूएल डेटाबेस सर्वर",
            (5432, Language::Arabic) => "خادم قاعدة بيانات PostgreSQL",
            (5432, Language::French) => "Serveur de base de données PostgreSQL",
            (5432, Language::Bengali) => "পোস্টগ্রিসকিউএল ডেটাবেস সার্ভার",
            (5432, Language::Portuguese) => "Servidor de banco de dados PostgreSQL",
            (5432, Language::Russian) => "Сервер баз данных PostgreSQL",
            (5432, Language::Urdu) => "پوسٹگری ایس کیو ایل ڈیٹا بیس",

            (8000, Language::Spanish) => "Servidor web de desarrollo o prueba",
            (8000, Language::English) => "Development / test web server",
            (8000, Language::Chinese) => "本地开发与测试 Web 服务器",
            (8000, Language::Hindi) => "डेवलपमेंट / टेस्ट वेब सर्वर",
            (8000, Language::Arabic) => "خادم ويب للتطوير والاختبار",
            (8000, Language::French) => "Serveur Web de développement / test",
            (8000, Language::Bengali) => "ডেভেলপমেন্ট / টেস্ট ওয়েব সার্ভার",
            (8000, Language::Portuguese) => "Servidor Web de desenvolvimento ou teste",
            (8000, Language::Russian) => "Сервер разработки или тестирования",
            (8000, Language::Urdu) => "ڈیولپمنٹ / ٹیسٹ ویب سرور",

            (8080, Language::Spanish) => "Servidor web alternativo / Proxy",
            (8080, Language::English) => "Alternative web server / Proxy service",
            (8080, Language::Chinese) => "备用 Web 服务器 / 代理服务",
            (8080, Language::Hindi) => "वैकल्पिक वेब सर्वर / प्रॉक्सी",
            (8080, Language::Arabic) => "خادم ويب بديل / بروكسي",
            (8080, Language::French) => "Serveur Web alternatif / Proxy",
            (8080, Language::Bengali) => "বিকল্প ওয়েব সার্ভার / প্রক্সি",
            (8080, Language::Portuguese) => "Servidor Web alternativo / Proxy",
            (8080, Language::Russian) => "Альтернативный веб-сервер / Прокси",
            (8080, Language::Urdu) => "متبادل ویب سرور / پراکسی",

            (8443, Language::Spanish) => "Servidor web seguro alternativo (SSL)",
            (8443, Language::English) => "Alternative secure web server (SSL)",
            (8443, Language::Chinese) => "备用安全加密 Web 服务器 (SSL)",
            (8443, Language::Hindi) => "वैकल्पिक सुरक्षित वेब सर्वर (SSL)",
            (8443, Language::Arabic) => "خادم ويب آمن بديل (SSL)",
            (8443, Language::French) => "Serveur Web sécurisé alternatif (SSL)",
            (8443, Language::Bengali) => "বিকল্প নিরাপদ ওয়েব সার্ভার (SSL)",
            (8443, Language::Portuguese) => "Servidor Web seguro alternativo (SSL)",
            (8443, Language::Russian) => "Альтернативный защищенный веб-сервер (SSL)",
            (8443, Language::Urdu) => "متبادل محفوظ ویب سرور (SSL)",

            (9000, Language::Spanish) => "Panel de gestión Docker (Portainer)",
            (9000, Language::English) => "Docker container management panel (Portainer)",
            (9000, Language::Chinese) => "Docker 容器集群管理面板 (Portainer)",
            (9000, Language::Hindi) => "डॉकर कंटेनर प्रबंधन (Portainer)",
            (9000, Language::Arabic) => "لوحة إدارة حاويات Docker (Portainer)",
            (9000, Language::French) => "Panneau de gestion Docker (Portainer)",
            (9000, Language::Bengali) => "ডকার কন্টেইনার ম্যানেজমেন্ট (Portainer)",
            (9000, Language::Portuguese) => "Painel de gerenciamento Docker (Portainer)",
            (9000, Language::Russian) => "Панель управления контейнерами Docker (Portainer)",
            (9000, Language::Urdu) => "ڈاکر کنٹینر مینجمنٹ پینل (Portainer)",

            (9090, Language::Spanish) => "Panel de administración del sistema (Cockpit)",
            (9090, Language::English) => "System administration / metrics panel (Cockpit)",
            (9090, Language::Chinese) => "系统管理与监控面板 (Cockpit / Prometheus)",
            (9090, Language::Hindi) => "सिस्टम प्रबंधन और मेट्रिक्स (Cockpit)",
            (9090, Language::Arabic) => "لوحة إدارة ومراقبة النظام (Cockpit)",
            (9090, Language::French) => "Panneau d'administration système (Cockpit)",
            (9090, Language::Bengali) => "সিস্টেম ম্যানেজমেন্ট প্যানেল (Cockpit)",
            (9090, Language::Portuguese) => "Painel de administração do sistema (Cockpit)",
            (9090, Language::Russian) => "Панель администрирования системы (Cockpit)",
            (9090, Language::Urdu) => "سسٹم ایڈمنسٹریشن پینل (Cockpit)",

            (9100, Language::Spanish) => "Puerto de impresión directa en red (RAW)",
            (9100, Language::English) => "Direct network printing port (RAW)",
            (9100, Language::Chinese) => "网络打印机直连端口 (RAW)",
            (9100, Language::Hindi) => "डायरेक्ट नेटवर्क प्रिंटिंग (RAW)",
            (9100, Language::Arabic) => "منفذ الطباعة الشبكية المباشرة (RAW)",
            (9100, Language::French) => "Port d'impression réseau directe (RAW)",
            (9100, Language::Bengali) => "সরাসরি নেটওয়ার্ক প্রিন্টিং পোর্ট (RAW)",
            (9100, Language::Portuguese) => "Porta de impressão direta em rede (RAW)",
            (9100, Language::Russian) => "Порт прямой сетевой печати (RAW)",
            (9100, Language::Urdu) => "براہ راست نیٹ ورک پرنٹنگ پورٹ (RAW)",

            (_, Language::Spanish) => "Servicio de red estándar o propietario",
            (_, Language::English) => "Standard or custom network service",
            (_, Language::Chinese) => "标准或专用网络服务",
            (_, Language::Hindi) => "मानक या कस्टम नेटवर्क सेवा",
            (_, Language::Arabic) => "خدمة شبكة مخصصة أو قياسية",
            (_, Language::French) => "Service réseau standard ou personnalisé",
            (_, Language::Bengali) => "স্ট্যান্ডার্ড বা কাস্টম নেটওয়ার্ক সার্ভিস",
            (_, Language::Portuguese) => "Serviço de rede padrão ou personalizado",
            (_, Language::Russian) => "Сетевая служба или пользовательский сервис",
            (_, Language::Urdu) => "معیاری یا کسٹم نیٹ ورک سروس",
        }
    }

    pub fn speed_test_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🚀 Test de Velocidad",
            Language::English => "🚀 Speed Test",
            Language::Chinese => "🚀 测速",
            Language::Hindi => "🚀 स्पीड टेस्ट",
            Language::Arabic => "🚀 اختبار السرعة",
            Language::French => "🚀 Test de Débit",
            Language::Bengali => "🚀 স্পিড টেস্ট",
            Language::Portuguese => "🚀 Teste de Velocidade",
            Language::Russian => "🚀 Тест скорости",
            Language::Urdu => "🚀 اسپیڈ ٹیسٹ",
        }
    }

    pub fn speed_test_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🚀 Test de Velocidad de Internet",
            Language::English => "🚀 Internet Speed Test",
            Language::Chinese => "🚀 互联网网速测试",
            Language::Hindi => "🚀 इंटरनेट स्पीड टेस्ट",
            Language::Arabic => "🚀 اختبار سرعة الإنترنت",
            Language::French => "🚀 Test de Débit Internet",
            Language::Bengali => "🚀 ইন্টারনেট স্পিড টেস্ট",
            Language::Portuguese => "🚀 Teste de Velocidade da Internet",
            Language::Russian => "🚀 Тест скорости интернета",
            Language::Urdu => "🚀 انٹرنیٹ اسپیڈ ٹیسٹ",
        }
    }

    pub fn speed_test_subtitle(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Mide tu latencia (Ping), velocidad de descarga y velocidad de subida en tiempo real",
            Language::English => "Measure real-time ping, download speed, and upload speed",
            Language::Chinese => "实时精准测量网络延迟 (Ping)、下载速率与上传速率",
            Language::Hindi => "वास्तविक समय में पिंग, डाउनलोड गति और अपलोड गति मापें",
            Language::Arabic => "قياس زمن الاستجابة وسرعة التنزيل والرفع بدقة وبشكل فوري",
            Language::French => "Mesurez votre latence (Ping), débit descendant et débit montant en temps réel",
            Language::Bengali => "রিয়েল-টাইমে পিং, ডাউনলোড স্পিড এবং আপলোড স্পিড পরিমাপ করুন",
            Language::Portuguese => "Meça seu ping, velocidade de download e upload em tempo real",
            Language::Russian => "Точное измерение пинга, скорости скачивания и загрузки в реальном времени",
            Language::Urdu => "ریئل ٹائم میں پنگ، ڈاؤن لوڈ اسپیڈ اور اپ لوڈ اسپیڈ کی پیمائش کریں",
        }
    }

    pub fn speed_test_ping(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🏓 Latencia / Ping",
            Language::English => "🏓 Latency / Ping",
            Language::Chinese => "🏓 网络延迟 / Ping",
            Language::Hindi => "🏓 विलंबता / पिंग",
            Language::Arabic => "🏓 زمن الاستجابة / Ping",
            Language::French => "🏓 Latence / Ping",
            Language::Bengali => "🏓 লেটেন্সি / পিং",
            Language::Portuguese => "🏓 Latência / Ping",
            Language::Russian => "🏓 Задержка / Пинг",
            Language::Urdu => "🏓 تاخیر / پنگ",
        }
    }

    pub fn speed_test_download(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⬇️ Descarga",
            Language::English => "⬇️ Download",
            Language::Chinese => "⬇️ 下载速率",
            Language::Hindi => "⬇️ डाउनलोड",
            Language::Arabic => "⬇️ سرعة التنزيل",
            Language::French => "⬇️ Téléchargement",
            Language::Bengali => "⬇️ ডাউনলোড",
            Language::Portuguese => "⬇️ Download",
            Language::Russian => "⬇️ Скачивание",
            Language::Urdu => "⬇️ ڈاؤن لوڈ",
        }
    }

    pub fn speed_test_upload(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⬆️ Subida",
            Language::English => "⬆️ Upload",
            Language::Chinese => "⬆️ 上传速率",
            Language::Hindi => "⬆️ अपलोड",
            Language::Arabic => "⬆️ سرعة الرفع",
            Language::French => "⬆️ Téléversement",
            Language::Bengali => "⬆️ আপলোড",
            Language::Portuguese => "⬆️ Upload",
            Language::Russian => "⬆️ Загрузка",
            Language::Urdu => "⬆️ اپ لوڈ",
        }
    }

    pub fn speed_test_server(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Servidor / Red:",
            Language::English => "Server / Network:",
            Language::Chinese => "测速节点与服务器：",
            Language::Hindi => "सर्वर / नेटवर्क:",
            Language::Arabic => "الخادم / الشبكة:",
            Language::French => "Serveur / Réseau :",
            Language::Bengali => "সার্ভার / নেটওয়ার্ক:",
            Language::Portuguese => "Servidor / Rede:",
            Language::Russian => "Сервер / Сеть:",
            Language::Urdu => "سرور / نیٹ ورک:",
        }
    }

    pub fn speed_test_start(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "▶ Iniciar Test de Velocidad",
            Language::English => "▶ Start Speed Test",
            Language::Chinese => "▶ 开始测速",
            Language::Hindi => "▶ टेस्ट शुरू करें",
            Language::Arabic => "▶ بدء اختبار السرعة",
            Language::French => "▶ Démarrer le Test",
            Language::Bengali => "▶ টেস্ট শুরু করুন",
            Language::Portuguese => "▶ Iniciar Teste",
            Language::Russian => "▶ Запустить тест скорости",
            Language::Urdu => "▶ ٹیسٹ شروع کریں",
        }
    }

    pub fn speed_test_running(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⏳ Midiendo velocidad...",
            Language::English => "⏳ Running speed test...",
            Language::Chinese => "⏳ 正在测速中...",
            Language::Hindi => "⏳ गति मापी जा रही है...",
            Language::Arabic => "⏳ جاري اختبار السرعة...",
            Language::French => "⏳ Test de débit en cours...",
            Language::Bengali => "⏳ স্পিড টেস্ট চলছে...",
            Language::Portuguese => "⏳ Testando velocidade...",
            Language::Russian => "⏳ Выполняется тест скорости...",
            Language::Urdu => "⏳ رفتار ماپی جا رہی ہے...",
        }
    }

    pub fn speed_stage_ping(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Midiendo latencia (Ping)...",
            Language::English => "Measuring latency (Ping)...",
            Language::Chinese => "正在测量网络延迟 (Ping)...",
            Language::Hindi => "विलंबता (पिंग) मापी जा रही है...",
            Language::Arabic => "جاري قياس زمن الاستجابة (Ping)...",
            Language::French => "Mesure de la latence (Ping)...",
            Language::Bengali => "লেটেন্সি (পিং) পরিমাপ করা হচ্ছে...",
            Language::Portuguese => "Medindo latência (Ping)...",
            Language::Russian => "Измерение задержки (Ping)...",
            Language::Urdu => "تاخیر (پنگ) کی پیمائش جاری ہے...",
        }
    }

    pub fn speed_stage_download(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Midiendo velocidad de descarga...",
            Language::English => "Measuring download speed...",
            Language::Chinese => "正在测试下载速率...",
            Language::Hindi => "डाउनलोड गति मापी जा रही है...",
            Language::Arabic => "جاري قياس سرعة التنزيل...",
            Language::French => "Mesure de la vitesse de téléchargement...",
            Language::Bengali => "ডাউনলোড গতি পরিমাপ করা হচ্ছে...",
            Language::Portuguese => "Medindo velocidade de download...",
            Language::Russian => "Тестирование скорости скачивания...",
            Language::Urdu => "ڈاؤن لوڈ اسپیڈ کی جانچ جاری ہے...",
        }
    }

    pub fn speed_stage_upload(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Midiendo velocidad de subida...",
            Language::English => "Measuring upload speed...",
            Language::Chinese => "正在测试上传速率...",
            Language::Hindi => "अपलोड गति मापी जा रही है...",
            Language::Arabic => "جاري قياس سرعة الرفع...",
            Language::French => "Mesure de la vitesse de téléversement...",
            Language::Bengali => "আপলোড গতি পরিমাপ করা হচ্ছে...",
            Language::Portuguese => "Medindo velocidade de upload...",
            Language::Russian => "Тестирование скорости загрузки...",
            Language::Urdu => "اپ لوڈ اسپیڈ کی جانچ جاری ہے...",
        }
    }

    pub fn speed_stage_complete(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Prueba de velocidad completada con éxito.",
            Language::English => "Speed test completed successfully.",
            Language::Chinese => "网络测速已圆满完成。",
            Language::Hindi => "स्पीड टेस्ट सफलतापूर्वक पूरा हुआ।",
            Language::Arabic => "اكتمل اختبار السرعة بنجاح.",
            Language::French => "Test de débit terminé avec succès.",
            Language::Bengali => "স্পিড টেস্ট সফলভাবে সম্পন্ন হয়েছে।",
            Language::Portuguese => "Teste de velocidade concluído com sucesso.",
            Language::Russian => "Тест скорости успешно завершен.",
            Language::Urdu => "اسپیڈ ٹیسٹ کامیابی سے مکمل ہو گیا۔",
        }
    }

    #[allow(dead_code)]
    pub fn filter_all(lang: Language) -> &'static str {
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
            Language::Urdu => "تمام",
        }
    }

    pub fn filter_whitelist(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Lista Blanca",
            Language::English => "Whitelist",
            Language::Chinese => "白名单",
            Language::Hindi => "श्वेतसूची",
            Language::Arabic => "القائمة البيضاء",
            Language::French => "Liste blanche",
            Language::Bengali => "হোয়াইটলিস্ট",
            Language::Portuguese => "Lista Branca",
            Language::Russian => "Белый список",
            Language::Urdu => "وائٹ لسٹ",
        }
    }

    pub fn filter_blocked(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Bloqueados",
            Language::English => "Blocked",
            Language::Chinese => "已阻止",
            Language::Hindi => "अवरुद्ध",
            Language::Arabic => "المحظورة",
            Language::French => "Bloqués",
            Language::Bengali => "ব্লক করা",
            Language::Portuguese => "Bloqueados",
            Language::Russian => "Заблокированные",
            Language::Urdu => "بلاک شدہ",
        }
    }

    pub fn badge_blocked(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "BLOQUEADO",
            Language::English => "BLOCKED",
            Language::Chinese => "已阻止",
            Language::Hindi => "अवरुद्ध",
            Language::Arabic => "محظور",
            Language::French => "BLOQUÉ",
            Language::Bengali => "ব্লকড",
            Language::Portuguese => "BLOQUEADO",
            Language::Russian => "ЗАБЛОКИРОВАН",
            Language::Urdu => "بلاک",
        }
    }

    pub fn badge_whitelisted(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "CONFIANZA",
            Language::English => "TRUSTED",
            Language::Chinese => "信任设备",
            Language::Hindi => "विश्वसनीय",
            Language::Arabic => "موثوق",
            Language::French => "CONFIANCE",
            Language::Bengali => "বিশ্বস্ত",
            Language::Portuguese => "CONFIÁVEL",
            Language::Russian => "ДОВЕРЕННЫЙ",
            Language::Urdu => "قابل اعتماد",
        }
    }

    pub fn btn_block_router(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🚫 Bloquear en Router",
            Language::English => "🚫 Block on Router",
            Language::Chinese => "🚫 在路由器上阻止",
            Language::Hindi => "🚫 राउटर पर ब्लॉक करें",
            Language::Arabic => "🚫 حظر على الراوتر",
            Language::French => "🚫 Bloquer sur le routeur",
            Language::Bengali => "🚫 রাউটারে ব্লক করুন",
            Language::Portuguese => "🚫 Bloquear no Roteador",
            Language::Russian => "🚫 Блокировать на роутере",
            Language::Urdu => "🚫 روٹر پر بلاک کریں",
        }
    }

    pub fn btn_unblock_router(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "✅ Desbloquear / Readmitir",
            Language::English => "✅ Unblock / Readmit",
            Language::Chinese => "✅ 解锁 / 重新允许",
            Language::Hindi => "✅ अनब्लॉक / पुनः अनुमति",
            Language::Arabic => "✅ إلغاء الحظر / إعادة القبول",
            Language::French => "✅ Débloquer / Réadmettre",
            Language::Bengali => "✅ আনব্লক / পুনঃঅনুমতি",
            Language::Portuguese => "✅ Desbloquear / Readmitir",
            Language::Russian => "✅ Разблокировать",
            Language::Urdu => "✅ ان بلاک / دوبارہ بحال",
        }
    }

    pub fn btn_add_whitelist(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🛡️ Añadir a Lista Blanca",
            Language::English => "🛡️ Add to Whitelist",
            Language::Chinese => "🛡️ 加入白名单",
            Language::Hindi => "🛡️ श्वेतसूची में जोड़ें",
            Language::Arabic => "🛡️ إضافة للقائمة البيضاء",
            Language::French => "🛡️ Ajouter à la liste blanche",
            Language::Bengali => "🛡️ হোয়াইটলিস্টে যোগ করুন",
            Language::Portuguese => "🛡️ Adicionar à Lista Branca",
            Language::Russian => "🛡️ Добавить в белый список",
            Language::Urdu => "🛡️ وائٹ لسٹ میں شامل کریں",
        }
    }

    pub fn btn_remove_whitelist(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🛡️ Quitar de Lista Blanca",
            Language::English => "🛡️ Remove from Whitelist",
            Language::Chinese => "🛡️ 移出白名单",
            Language::Hindi => "🛡️ श्वेतसूची से हटाएं",
            Language::Arabic => "🛡️ إزالة من القائمة البيضاء",
            Language::French => "🛡️ Retirer de la liste blanche",
            Language::Bengali => "🛡️ হোয়াইটলিস্ট থেকে মুছুন",
            Language::Portuguese => "🛡️ Remover da Lista Branca",
            Language::Russian => "🛡️ Убрать из белого списка",
            Language::Urdu => "🛡️ وائٹ لسٹ سے نکالیں",
        }
    }

    pub fn cannot_block_self(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Protección de seguridad: No puedes bloquear tu propia PC.",
            Language::English => "Security protection: You cannot block your own PC.",
            Language::Chinese => "安全保护：无法阻止本机电脑。",
            Language::Hindi => "सुरक्षा सुरक्षा: आप अपने पीसी को ब्लॉक नहीं कर सकते।",
            Language::Arabic => "حماية الأمان: لا يمكنك حظر جهازك المحلي.",
            Language::French => "Protection de sécurité : Impossible de bloquer votre propre PC.",
            Language::Bengali => "নিরাপত্তা সুরক্ষা: আপনি নিজের পিসি ব্লক করতে পারবেন না।",
            Language::Portuguese => "Proteção de segurança: Você não pode bloquear seu próprio PC.",
            Language::Russian => "Защита безопасности: Нельзя заблокировать собственный ПК.",
            Language::Urdu => "سیکیورٹی تحفظ: آپ اپنا پی سی بلاک نہیں کر سکتے۔",
        }
    }

    pub fn cannot_block_gateway(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Protección de seguridad: No puedes bloquear el Router.",
            Language::English => "Security protection: You cannot block the Router.",
            Language::Chinese => "安全保护：无法阻止网关/路由器。",
            Language::Hindi => "सुरक्षा सुरक्षा: आप गेटवे/राउटर को ब्लॉक नहीं कर सकते।",
            Language::Arabic => "حماية الأمان: لا يمكنك حظر البوابة / الراوتر.",
            Language::French => "Protection de sécurité : Impossible de bloquer le routeur.",
            Language::Bengali => "নিরাপত্তা সুরক্ষা: আপনি গেটওয়ে / রাউটার ব্লক করতে পারবেন না।",
            Language::Portuguese => "Proteção de segurança: Você não pode bloquear o roteador.",
            Language::Russian => "Защита безопасности: Нельзя заблокировать шлюз / роутер.",
            Language::Urdu => "سیکیورٹی تحفظ: آپ گیٹ وے / روٹر کو بلاک نہیں کر سکتے۔",
        }
    }

    pub fn speed_test_history_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Historial de Pruebas de Velocidad",
            Language::English => "Speed Test History",
            Language::Chinese => "测速历史记录",
            Language::Hindi => "स्पीड टेस्ट इतिहास",
            Language::Arabic => "سجل اختبارات السرعة",
            Language::French => "Historique des tests de débit",
            Language::Bengali => "স্পিড টেস্ট ইতিহাস",
            Language::Portuguese => "Histórico de Testes de Velocidade",
            Language::Russian => "История тестов скорости",
            Language::Urdu => "اسپیڈ ٹیسٹ کی تاریخ",
        }
    }

    pub fn speed_test_history_empty(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Aún no se han registrado pruebas de velocidad.",
            Language::English => "No speed tests recorded yet.",
            Language::Chinese => "尚未记录任何测速结果。",
            Language::Hindi => "अभी तक कोई स्पीड टेस्ट दर्ज नहीं हुआ है।",
            Language::Arabic => "لم يتم تسجيل أي اختبارات سرعة بعد.",
            Language::French => "Aucun test de débit enregistré pour le moment.",
            Language::Bengali => "এখনও কোনও স্পিড টেস্ট রেকর্ড করা হয়নি।",
            Language::Portuguese => "Nenhum teste de velocidade gravado ainda.",
            Language::Russian => "Тесты скорости пока не проводились.",
            Language::Urdu => "ابھی تک کوئی اسپیڈ ٹیسٹ ریکارڈ نہیں ہوا۔",
        }
    }

    pub fn speed_test_history_clear(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Limpiar Historial",
            Language::English => "Clear History",
            Language::Chinese => "清除历史",
            Language::Hindi => "इतिहास साफ़ करें",
            Language::Arabic => "مسح السجل",
            Language::French => "Effacer l'historique",
            Language::Bengali => "ইতিহাস মুছুন",
            Language::Portuguese => "Limpar Histórico",
            Language::Russian => "Очистить историю",
            Language::Urdu => "تاریخ صاف کریں",
        }
    }

    pub fn router_settings_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Control de Router (Linksys MR73 Mesh)",
            Language::English => "Router Control (Linksys MR73 Mesh)",
            Language::Chinese => "路由器控制 (Linksys MR73 Mesh)",
            Language::Hindi => "राउटर नियंत्रण (Linksys MR73 Mesh)",
            Language::Arabic => "التحكم في الراوتر (Linksys MR73 Mesh)",
            Language::French => "Contrôle du routeur (Linksys MR73 Mesh)",
            Language::Bengali => "রাউটার নিয়ন্ত্রণ (Linksys MR73 Mesh)",
            Language::Portuguese => "Controle do Roteador (Linksys MR73 Mesh)",
            Language::Russian => "Управление роутером (Linksys MR73 Mesh)",
            Language::Urdu => "روٹر کنٹرول (Linksys MR73 Mesh)",
        }
    }

    pub fn router_ip_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "IP del Router",
            Language::English => "Router IP",
            Language::Chinese => "路由器 IP",
            Language::Hindi => "राउटर आईपी",
            Language::Arabic => "عنوان IP للراوتر",
            Language::French => "IP du routeur",
            Language::Bengali => "রাউটার আইপি",
            Language::Portuguese => "IP do Roteador",
            Language::Russian => "IP роутера",
            Language::Urdu => "روٹر آئی پی",
        }
    }

    pub fn router_pass_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Contraseña de Admin del Router",
            Language::English => "Router Admin Password",
            Language::Chinese => "路由器管理员密码",
            Language::Hindi => "राउटर व्यवस्थापक पासवर्ड",
            Language::Arabic => "كلمة مرور مسؤول الراوتر",
            Language::French => "Mot de passe admin du routeur",
            Language::Bengali => "রাউটার অ্যাডমিন পাসওয়ার্ড",
            Language::Portuguese => "Senha de Admin do Roteador",
            Language::Russian => "Пароль администратора роутера",
            Language::Urdu => "روٹر ایڈمن پاس ورڈ",
        }
    }

    pub fn router_test_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Probar Conexión",
            Language::English => "Test Connection",
            Language::Chinese => "测试连接",
            Language::Hindi => "कनेक्शन जांचें",
            Language::Arabic => "اختبار الاتصال",
            Language::French => "Tester la connexion",
            Language::Bengali => "সংযোগ পরীক্ষা করুন",
            Language::Portuguese => "Testar Conexão",
            Language::Russian => "Проверить подключение",
            Language::Urdu => "کنکشن ٹیسٹ کریں",
        }
    }

    pub fn router_connected_ok(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🟢 Conectado con éxito al router Linksys MR73",
            Language::English => "🟢 Successfully connected to Linksys MR73 router",
            Language::Chinese => "🟢 成功连接到 Linksys MR73 路由器",
            Language::Hindi => "🟢 Linksys MR73 राउटर से सफलतापूर्वक जुड़ा",
            Language::Arabic => "🟢 تم الاتصال بنجاح براوتر Linksys MR73",
            Language::French => "🟢 Connecté avec succès au routeur Linksys MR73",
            Language::Bengali => "🟢 Linksys MR73 রাউটারে সফলভাবে সংযুক্ত হয়েছে",
            Language::Portuguese => "🟢 Conectado com sucesso ao roteador Linksys MR73",
            Language::Russian => "🟢 Успешное подключение к роутеру Linksys MR73",
            Language::Urdu => "🟢 Linksys MR73 روٹر سے کامیابی سے منسلک ہو گیا",
        }
    }

    pub fn router_connected_err(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🔴 Error: Contraseña inválida o router inalcanzable",
            Language::English => "🔴 Error: Invalid password or router unreachable",
            Language::Chinese => "🔴 错误：密码无效或无法访问路由器",
            Language::Hindi => "🔴 त्रुटि: अमान्य पासवर्ड या राउटर अनुपलब्ध",
            Language::Arabic => "🔴 خطأ: كلمة المرور غير صالحة أو تعذر الوصول إلى الراوتر",
            Language::French => "🔴 Erreur : Mot de passe invalide ou routeur inaccessible",
            Language::Bengali => "🔴 ত্রুটি: অবৈধ পাসওয়ার্ড বা রাউটার অপ্রাপ্য",
            Language::Portuguese => "🔴 Erro: Senha inválida ou roteador inacessível",
            Language::Russian => "🔴 Ошибка: Неверный пароль или роутер недоступен",
            Language::Urdu => "🔴 خرابی: غلط پاس ورڈ یا روٹر تک رسائی ممکن نہیں",
        }
    }
}
