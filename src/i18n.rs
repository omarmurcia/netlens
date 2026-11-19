use crate::history::NetworkEventType;
use crate::model::{Device, DeviceCategory, DeviceStatus};
use crate::router::storage::StorageFileType;
use crate::security_audit::SecurityCheckId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The 10 most spoken languages in the world (by total speaker count)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Language {
    Spanish,
    #[default]
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

            (DeviceCategory::Tablet, Language::Spanish) => "📲 Tablet",
            (DeviceCategory::Tablet, Language::English) => "📲 Tablet",
            (DeviceCategory::Tablet, Language::Chinese) => "📲 平板",
            (DeviceCategory::Tablet, Language::Hindi) => "📲 टैबलेट",
            (DeviceCategory::Tablet, Language::Arabic) => "📲 جهاز لوحي",
            (DeviceCategory::Tablet, Language::French) => "📲 Tablette",
            (DeviceCategory::Tablet, Language::Bengali) => "📲 ট্যাবলেট",
            (DeviceCategory::Tablet, Language::Portuguese) => "📲 Tablet",
            (DeviceCategory::Tablet, Language::Russian) => "📲 Планшет",
            (DeviceCategory::Tablet, Language::Urdu) => "📲 ٹیبلٹ",

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

            (DeviceCategory::StreamingDevice, Language::Spanish) => "🎬 Streaming",
            (DeviceCategory::StreamingDevice, Language::English) => "🎬 Streaming",
            (DeviceCategory::StreamingDevice, Language::Chinese) => "🎬 流媒体",
            (DeviceCategory::StreamingDevice, Language::Hindi) => "🎬 स्ट्रीमिंग",
            (DeviceCategory::StreamingDevice, Language::Arabic) => "🎬 بث وسائط",
            (DeviceCategory::StreamingDevice, Language::French) => "🎬 Streaming",
            (DeviceCategory::StreamingDevice, Language::Bengali) => "🎬 স্ট্রিমিং",
            (DeviceCategory::StreamingDevice, Language::Portuguese) => "🎬 Streaming",
            (DeviceCategory::StreamingDevice, Language::Russian) => "🎬 Стриминг",
            (DeviceCategory::StreamingDevice, Language::Urdu) => "🎬 اسٹریمنگ",

            (DeviceCategory::GamingConsole, Language::Spanish) => "🎮 Consola",
            (DeviceCategory::GamingConsole, Language::English) => "🎮 Console",
            (DeviceCategory::GamingConsole, Language::Chinese) => "🎮 游戏机",
            (DeviceCategory::GamingConsole, Language::Hindi) => "🎮 कंसोल",
            (DeviceCategory::GamingConsole, Language::Arabic) => "🎮 منصة ألعاب",
            (DeviceCategory::GamingConsole, Language::French) => "🎮 Console",
            (DeviceCategory::GamingConsole, Language::Bengali) => "🎮 কনসোল",
            (DeviceCategory::GamingConsole, Language::Portuguese) => "🎮 Console",
            (DeviceCategory::GamingConsole, Language::Russian) => "🎮 Консоль",
            (DeviceCategory::GamingConsole, Language::Urdu) => "🎮 کنسول",

            (DeviceCategory::SmartSpeaker, Language::Spanish) => "🔊 Altavoz",
            (DeviceCategory::SmartSpeaker, Language::English) => "🔊 Speaker",
            (DeviceCategory::SmartSpeaker, Language::Chinese) => "🔊 音箱",
            (DeviceCategory::SmartSpeaker, Language::Hindi) => "🔊 स्पीकर",
            (DeviceCategory::SmartSpeaker, Language::Arabic) => "🔊 مكبّر صوت",
            (DeviceCategory::SmartSpeaker, Language::French) => "🔊 Enceinte",
            (DeviceCategory::SmartSpeaker, Language::Bengali) => "🔊 স্পিকার",
            (DeviceCategory::SmartSpeaker, Language::Portuguese) => "🔊 Caixa de Som",
            (DeviceCategory::SmartSpeaker, Language::Russian) => "🔊 Колонка",
            (DeviceCategory::SmartSpeaker, Language::Urdu) => "🔊 اسپیکر",

            (DeviceCategory::SecurityCamera, Language::Spanish) => "📹 Cámara",
            (DeviceCategory::SecurityCamera, Language::English) => "📹 Camera",
            (DeviceCategory::SecurityCamera, Language::Chinese) => "📹 摄像机",
            (DeviceCategory::SecurityCamera, Language::Hindi) => "📹 कैमरा",
            (DeviceCategory::SecurityCamera, Language::Arabic) => "📹 كاميرا",
            (DeviceCategory::SecurityCamera, Language::French) => "📹 Caméra",
            (DeviceCategory::SecurityCamera, Language::Bengali) => "📹 ক্যামেরা",
            (DeviceCategory::SecurityCamera, Language::Portuguese) => "📹 Câmera",
            (DeviceCategory::SecurityCamera, Language::Russian) => "📹 Камера",
            (DeviceCategory::SecurityCamera, Language::Urdu) => "📹 کیمرہ",

            (DeviceCategory::Iot, Language::Spanish) => "🔌 Domótica",
            (DeviceCategory::Iot, Language::English) => "🔌 Smart Home",
            (DeviceCategory::Iot, Language::Chinese) => "🔌 物联网",
            (DeviceCategory::Iot, Language::Hindi) => "🔌 आईओटी",
            (DeviceCategory::Iot, Language::Arabic) => "🔌 أجهزة ذكية",
            (DeviceCategory::Iot, Language::French) => "🔌 Domotique",
            (DeviceCategory::Iot, Language::Bengali) => "🔌 আইওটি",
            (DeviceCategory::Iot, Language::Portuguese) => "🔌 Automação",
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

            (DeviceCategory::StorageNas, Language::Spanish) => "💾 NAS",
            (DeviceCategory::StorageNas, Language::English) => "💾 NAS",
            (DeviceCategory::StorageNas, Language::Chinese) => "💾 NAS",
            (DeviceCategory::StorageNas, Language::Hindi) => "💾 एनएएस",
            (DeviceCategory::StorageNas, Language::Arabic) => "💾 تخزين شبكي",
            (DeviceCategory::StorageNas, Language::French) => "💾 NAS",
            (DeviceCategory::StorageNas, Language::Bengali) => "💾 ন্যাস",
            (DeviceCategory::StorageNas, Language::Portuguese) => "💾 NAS",
            (DeviceCategory::StorageNas, Language::Russian) => "💾 NAS",
            (DeviceCategory::StorageNas, Language::Urdu) => "💾 این اے ایس",

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

            (DeviceCategory::Wearable, Language::Spanish) => "⌚ Wearable",
            (DeviceCategory::Wearable, Language::English) => "⌚ Wearable",
            (DeviceCategory::Wearable, Language::Chinese) => "⌚ 穿戴设备",
            (DeviceCategory::Wearable, Language::Hindi) => "⌚ वियरेबल",
            (DeviceCategory::Wearable, Language::Arabic) => "⌚ جهاز ملبوس",
            (DeviceCategory::Wearable, Language::French) => "⌚ Montre",
            (DeviceCategory::Wearable, Language::Bengali) => "⌚ পরিধেয়",
            (DeviceCategory::Wearable, Language::Portuguese) => "⌚ Wearable",
            (DeviceCategory::Wearable, Language::Russian) => "⌚ Часы",
            (DeviceCategory::Wearable, Language::Urdu) => "⌚ پہننے والا آلہ",

            (DeviceCategory::NetworkBridge, Language::Spanish) => "🔀 Switch/AP",
            (DeviceCategory::NetworkBridge, Language::English) => "🔀 Switch/AP",
            (DeviceCategory::NetworkBridge, Language::Chinese) => "🔀 交换机/AP",
            (DeviceCategory::NetworkBridge, Language::Hindi) => "🔀 स्विच/एपी",
            (DeviceCategory::NetworkBridge, Language::Arabic) => "🔀 محوّل/موزع",
            (DeviceCategory::NetworkBridge, Language::French) => "🔀 Switch/AP",
            (DeviceCategory::NetworkBridge, Language::Bengali) => "🔀 সুইচ/এপি",
            (DeviceCategory::NetworkBridge, Language::Portuguese) => "🔀 Switch/AP",
            (DeviceCategory::NetworkBridge, Language::Russian) => "🔀 Коммутатор/AP",
            (DeviceCategory::NetworkBridge, Language::Urdu) => "🔀 سوئچ/اے پی",

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
            (DeviceCategory::Smartphone, Language::English) => "Smartphone / Phone",
            (DeviceCategory::Smartphone, Language::Chinese) => "智能手机",
            (DeviceCategory::Smartphone, Language::Hindi) => "स्मार्टफोन / मोबाइल",
            (DeviceCategory::Smartphone, Language::Arabic) => "هاتف ذكي",
            (DeviceCategory::Smartphone, Language::French) => "Smartphone / Téléphone",
            (DeviceCategory::Smartphone, Language::Bengali) => "স্মার্টফোন",
            (DeviceCategory::Smartphone, Language::Portuguese) => "Smartphone / Celular",
            (DeviceCategory::Smartphone, Language::Russian) => "Смартфон / Мобильный",
            (DeviceCategory::Smartphone, Language::Urdu) => "اسمارٹ فون",

            (DeviceCategory::Tablet, Language::Spanish) => "Tablet / iPad",
            (DeviceCategory::Tablet, Language::English) => "Tablet / iPad",
            (DeviceCategory::Tablet, Language::Chinese) => "平板电脑 / iPad",
            (DeviceCategory::Tablet, Language::Hindi) => "टैबलेट / आईपैड",
            (DeviceCategory::Tablet, Language::Arabic) => "جهاز لوحي / آيباد",
            (DeviceCategory::Tablet, Language::French) => "Tablette / iPad",
            (DeviceCategory::Tablet, Language::Bengali) => "ট্যাবলেট / আইপ্যাড",
            (DeviceCategory::Tablet, Language::Portuguese) => "Tablet / iPad",
            (DeviceCategory::Tablet, Language::Russian) => "Планшет / iPad",
            (DeviceCategory::Tablet, Language::Urdu) => "ٹیبلٹ / آئی پیڈ",

            (DeviceCategory::Computer, Language::Spanish) => "Ordenador / PC / Mac",
            (DeviceCategory::Computer, Language::English) => "Computer / PC / Mac",
            (DeviceCategory::Computer, Language::Chinese) => "计算机 / 电脑 / Mac",
            (DeviceCategory::Computer, Language::Hindi) => "कंप्यूटर / पीसी / मैक",
            (DeviceCategory::Computer, Language::Arabic) => "حاسوب / كمبيوتر / ماك",
            (DeviceCategory::Computer, Language::French) => "Ordinateur / PC / Mac",
            (DeviceCategory::Computer, Language::Bengali) => "কম্পিউটার / পিসি / ম্যাক",
            (DeviceCategory::Computer, Language::Portuguese) => "Computador / PC / Mac",
            (DeviceCategory::Computer, Language::Russian) => "Компьютер / ПК / Mac",
            (DeviceCategory::Computer, Language::Urdu) => "کمپیوٹر / پی سی / میک",

            (DeviceCategory::SmartTv, Language::Spanish) => "Smart TV / Pantalla",
            (DeviceCategory::SmartTv, Language::English) => "Smart TV / Display",
            (DeviceCategory::SmartTv, Language::Chinese) => "智能电视 / 显示屏",
            (DeviceCategory::SmartTv, Language::Hindi) => "स्मार्ट टीवी / डिस्प्ले",
            (DeviceCategory::SmartTv, Language::Arabic) => "تلفاز ذكي / شاشة",
            (DeviceCategory::SmartTv, Language::French) => "Smart TV / Écran Connecté",
            (DeviceCategory::SmartTv, Language::Bengali) => "স্মার্ট টিভি / ডিসপ্লে",
            (DeviceCategory::SmartTv, Language::Portuguese) => "Smart TV / Tela",
            (DeviceCategory::SmartTv, Language::Russian) => "Smart TV / Дисплей",
            (DeviceCategory::SmartTv, Language::Urdu) => "اسمارٹ ٹی وی / ڈسپلے",

            (DeviceCategory::StreamingDevice, Language::Spanish) => "Dispositivo de Streaming",
            (DeviceCategory::StreamingDevice, Language::English) => "Streaming Media Player",
            (DeviceCategory::StreamingDevice, Language::Chinese) => "流媒体播放设备",
            (DeviceCategory::StreamingDevice, Language::Hindi) => "मीडिया स्ट्रीमिंग प्लेयर",
            (DeviceCategory::StreamingDevice, Language::Arabic) => "مشغّل وسائط متدفقة",
            (DeviceCategory::StreamingDevice, Language::French) => "Lecteur Streaming Multimédia",
            (DeviceCategory::StreamingDevice, Language::Bengali) => "মিডিয়া স্ট্রিমিং প্লেয়ার",
            (DeviceCategory::StreamingDevice, Language::Portuguese) => "Aparelho de Streaming",
            (DeviceCategory::StreamingDevice, Language::Russian) => "Стриминговый медиаплеер",
            (DeviceCategory::StreamingDevice, Language::Urdu) => "اسٹریمنگ میڈیا پلیئر",

            (DeviceCategory::GamingConsole, Language::Spanish) => "Consola de Videojuegos",
            (DeviceCategory::GamingConsole, Language::English) => "Video Game Console",
            (DeviceCategory::GamingConsole, Language::Chinese) => "电子游戏机",
            (DeviceCategory::GamingConsole, Language::Hindi) => "वीडियो गेम कंसोल",
            (DeviceCategory::GamingConsole, Language::Arabic) => "منصة ألعاب فيديو",
            (DeviceCategory::GamingConsole, Language::French) => "Console de Jeux",
            (DeviceCategory::GamingConsole, Language::Bengali) => "ভিডিও গেম কনসোল",
            (DeviceCategory::GamingConsole, Language::Portuguese) => "Console de Videogame",
            (DeviceCategory::GamingConsole, Language::Russian) => "Игровая приставка",
            (DeviceCategory::GamingConsole, Language::Urdu) => "ویڈیو گیم کنسول",

            (DeviceCategory::SmartSpeaker, Language::Spanish) => "Altavoz Inteligente / Audio",
            (DeviceCategory::SmartSpeaker, Language::English) => "Smart Speaker / Audio",
            (DeviceCategory::SmartSpeaker, Language::Chinese) => "智能音箱 / 音频设备",
            (DeviceCategory::SmartSpeaker, Language::Hindi) => "स्मार्ट स्पीकर / ऑडियो",
            (DeviceCategory::SmartSpeaker, Language::Arabic) => "مكبّر صوت ذكي / صوتيات",
            (DeviceCategory::SmartSpeaker, Language::French) => "Enceinte Connectée / Audio",
            (DeviceCategory::SmartSpeaker, Language::Bengali) => "স্মার্ট স্পিকার / অডিও",
            (DeviceCategory::SmartSpeaker, Language::Portuguese) => "Caixa de Som Inteligente",
            (DeviceCategory::SmartSpeaker, Language::Russian) => "Умная колонка / Аудио",
            (DeviceCategory::SmartSpeaker, Language::Urdu) => "اسمارٹ اسپیکر / آڈیو",

            (DeviceCategory::SecurityCamera, Language::Spanish) => "Cámara de Seguridad / Vigilancia",
            (DeviceCategory::SecurityCamera, Language::English) => "Security Camera / Surveillance",
            (DeviceCategory::SecurityCamera, Language::Chinese) => "安防监控摄像头 / 门铃",
            (DeviceCategory::SecurityCamera, Language::Hindi) => "सुरक्षा कैमरा / निगरानी",
            (DeviceCategory::SecurityCamera, Language::Arabic) => "كاميرا أمان / مراقبة فيديو",
            (DeviceCategory::SecurityCamera, Language::French) => "Caméra de Sécurité / Surveillance",
            (DeviceCategory::SecurityCamera, Language::Bengali) => "নিরাপত্তা ক্যামেরা / নজরদারি",
            (DeviceCategory::SecurityCamera, Language::Portuguese) => "Câmera de Segurança / Vigilância",
            (DeviceCategory::SecurityCamera, Language::Russian) => "Камера наблюдения / Видеодомофон",
            (DeviceCategory::SecurityCamera, Language::Urdu) => "سیکیورٹی کیمرہ / نگرانی",

            (DeviceCategory::Iot, Language::Spanish) => "Domótica / Smart Home / IoT",
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

            (DeviceCategory::StorageNas, Language::Spanish) => "Almacenamiento en Red / NAS",
            (DeviceCategory::StorageNas, Language::English) => "Network Attached Storage / NAS",
            (DeviceCategory::StorageNas, Language::Chinese) => "网络附加存储 / NAS",
            (DeviceCategory::StorageNas, Language::Hindi) => "नेटवर्क अटैच्ड स्टोरेज / एनएएस",
            (DeviceCategory::StorageNas, Language::Arabic) => "وحدة تخزين شبكية / NAS",
            (DeviceCategory::StorageNas, Language::French) => "Serveur de Stockage Réseau / NAS",
            (DeviceCategory::StorageNas, Language::Bengali) => "নেটওয়ার্ক স্টোরেজ / ন্যাস",
            (DeviceCategory::StorageNas, Language::Portuguese) => "Armazenamento em Rede / NAS",
            (DeviceCategory::StorageNas, Language::Russian) => "Сетевое хранилище / NAS",
            (DeviceCategory::StorageNas, Language::Urdu) => "نیٹ ورک اسٹوریج / این اے ایس",

            (DeviceCategory::Server, Language::Spanish) => "Servidor / Host Virtual",
            (DeviceCategory::Server, Language::English) => "Server / Virtual Host",
            (DeviceCategory::Server, Language::Chinese) => "服务器 / 虚拟主机",
            (DeviceCategory::Server, Language::Hindi) => "सर्वर / वर्चुअल होस्ट",
            (DeviceCategory::Server, Language::Arabic) => "خادم / مضيف افتراضي",
            (DeviceCategory::Server, Language::French) => "Serveur Dédié / Hôte Virtuel",
            (DeviceCategory::Server, Language::Bengali) => "ডেডিকেটেড সার্ভার",
            (DeviceCategory::Server, Language::Portuguese) => "Servidor / Host Virtual",
            (DeviceCategory::Server, Language::Russian) => "Сервер / Виртуальный хост",
            (DeviceCategory::Server, Language::Urdu) => "سرور / ورچوئل ہوسٹ",

            (DeviceCategory::Wearable, Language::Spanish) => "Smartwatch / Wearable",
            (DeviceCategory::Wearable, Language::English) => "Smartwatch / Wearable",
            (DeviceCategory::Wearable, Language::Chinese) => "智能手表 / 穿戴设备",
            (DeviceCategory::Wearable, Language::Hindi) => "स्मार्टवॉच / वियरेबल",
            (DeviceCategory::Wearable, Language::Arabic) => "ساعة ذكية / جهاز ملبوس",
            (DeviceCategory::Wearable, Language::French) => "Montre Connectée / Wearable",
            (DeviceCategory::Wearable, Language::Bengali) => "স্মার্টওয়াচ / পরিধেয় গ্যাজেট",
            (DeviceCategory::Wearable, Language::Portuguese) => "Smartwatch / Dispositivo Vestível",
            (DeviceCategory::Wearable, Language::Russian) => "Умные часы / Носимое устройство",
            (DeviceCategory::Wearable, Language::Urdu) => "اسمارٹ واچ / پہننے والا آلہ",

            (DeviceCategory::NetworkBridge, Language::Spanish) => "Switch / Repetidor de Red",
            (DeviceCategory::NetworkBridge, Language::English) => "Network Switch / Extender",
            (DeviceCategory::NetworkBridge, Language::Chinese) => "网络交换机 / 中继器",
            (DeviceCategory::NetworkBridge, Language::Hindi) => "नेटवर्क स्विच / एक्सटेंडर",
            (DeviceCategory::NetworkBridge, Language::Arabic) => "محوّل شبكة / مقوي إشارة",
            (DeviceCategory::NetworkBridge, Language::French) => "Commutateur / Répéteur Réseau",
            (DeviceCategory::NetworkBridge, Language::Bengali) => "নেটওয়ার্ক সুইচ / এক্সটেন্ডার",
            (DeviceCategory::NetworkBridge, Language::Portuguese) => "Switch / Repetidor de Rede",
            (DeviceCategory::NetworkBridge, Language::Russian) => "Сетевой коммутатор / Репитер",
            (DeviceCategory::NetworkBridge, Language::Urdu) => "نیٹ ورک سوئچ / ایکسٹینڈر",

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

    pub fn speed_stage_idle(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Listo para comenzar",
            Language::English => "Ready to test",
            Language::Chinese => "准备测试",
            Language::Hindi => "परीक्षण के लिए तैयार",
            Language::Arabic => "جاهز للاختبار",
            Language::French => "Prêt à tester",
            Language::Bengali => "পরীক্ষার জন্য প্রস্তুত",
            Language::Portuguese => "Pronto para testar",
            Language::Russian => "Готов к тестированию",
            Language::Urdu => "ٹیسٹ کے لیے تیار",
        }
    }

    pub fn speed_stage_failed(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Error durante la prueba de velocidad",
            Language::English => "Error during speed test",
            Language::Chinese => "测速过程中发生错误",
            Language::Hindi => "स्पीड टेस्ट के दौरान त्रुटि",
            Language::Arabic => "حدث خطأ أثناء اختبار السرعة",
            Language::French => "Erreur lors du test de débit",
            Language::Bengali => "স্পিড টেস্ট চলাকালীন ত্রুটি",
            Language::Portuguese => "Erro durante o teste de velocidade",
            Language::Russian => "Ошибка во время теста скорости",
            Language::Urdu => "اسپیڈ ٹیسٹ کے دوران خرابی",
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

    pub fn speed_test_col_time_server(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Hora / Servidor",
            Language::English => "Time / Server",
            Language::Chinese => "时间 / 服务器",
            Language::Hindi => "समय / सर्वर",
            Language::Arabic => "الوقت / الخادم",
            Language::French => "Heure / Serveur",
            Language::Bengali => "সময় / সার্ভার",
            Language::Portuguese => "Hora / Servidor",
            Language::Russian => "Время / Сервер",
            Language::Urdu => "وقت / سرور",
        }
    }

    pub fn speed_test_col_download(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⬇️ Bajada",
            Language::English => "⬇️ Download",
            Language::Chinese => "⬇️ 下载",
            Language::Hindi => "⬇️ डाउनलोड",
            Language::Arabic => "⬇️ تنزيل",
            Language::French => "⬇️ Téléchargement",
            Language::Bengali => "⬇️ ডাউনলোড",
            Language::Portuguese => "⬇️ Download",
            Language::Russian => "⬇️ Скачивание",
            Language::Urdu => "⬇️ ڈاؤن لوڈ",
        }
    }

    pub fn speed_test_col_upload(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⬆️ Subida",
            Language::English => "⬆️ Upload",
            Language::Chinese => "⬆️ 上传",
            Language::Hindi => "⬆️ अपलोड",
            Language::Arabic => "⬆️ رفع",
            Language::French => "⬆️ Téléversement",
            Language::Bengali => "⬆️ আপলোড",
            Language::Portuguese => "⬆️ Upload",
            Language::Russian => "⬆️ Загрузка",
            Language::Urdu => "⬆️ اپ لوڈ",
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

    pub fn btn_applying(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⏳ Aplicando en router...",
            Language::English => "⏳ Applying on router...",
            Language::Chinese => "⏳ 正在应用到路由器...",
            Language::Hindi => "⏳ राउटर पर लागू हो रहा है...",
            Language::Arabic => "⏳ جاري التطبيق على الراوتر...",
            Language::French => "⏳ Application sur le routeur...",
            Language::Bengali => "⏳ রাউটারে প্রয়োগ হচ্ছে...",
            Language::Portuguese => "⏳ Aplicando no roteador...",
            Language::Russian => "⏳ Применение на роутере...",
            Language::Urdu => "⏳ روٹر پر لاگو ہو رہا ہے...",
        }
    }

    pub fn router_require_password(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Para bloquear o readmitir dispositivos en el router, primero ingresa la contraseña de administración en Ajustes (⚙ Ajustes).",
            Language::English => "To block or unblock devices on the router, first enter the router admin password in Settings (⚙ Settings).",
            Language::Chinese => "要在路由器上阻止或解锁设备，请先在设置 (⚙ 设置) 中输入路由器管理员密码。",
            Language::Hindi => "राउटर पर उपकरणों को ब्लॉक या अनब्लॉक करने के लिए, पहले सेटिंग्स (⚙ सेटिंग्स) में व्यवस्थापक पासवर्ड दर्ज करें।",
            Language::Arabic => "لحظر الأجهزة أو إلغاء حظرها على الراوتر، أدخل أولاً كلمة مرور مسؤول الراوتر في الإعدادات (⚙ الإعدادات).",
            Language::French => "Pour bloquer ou débloquer des appareils sur le routeur, entrez d'abord le mot de passe admin dans Paramètres (⚙ Paramètres).",
            Language::Bengali => "রাউটারে ডিভাইস ব্লক বা আনব্লক করতে, প্রথমে সেটিংসে (⚙ সেটিংস) রাউটার অ্যাডমিন পাসওয়ার্ড লিখুন।",
            Language::Portuguese => "Para bloquear ou readmitir dispositivos no roteador, primeiro insira a senha de administrador em Configurações (⚙ Configurações).",
            Language::Russian => "Для блокировки или разблокировки устройств сначала введите пароль администратора роутера в Настройках (⚙ Настройки).",
            Language::Urdu => "روٹر پر ڈیوائسز کو بلاک یا ان بلاک کرنے کے لیے، پہلے ترتیبات (⚙ ترتیبات) میں روٹر ایڈمن پاس ورڈ درج کریں۔",
        }
    }

    pub fn router_block_success(lang: Language, ip: &str, mac: &str) -> String {
        match lang {
            Language::Spanish => format!("✓ Dispositivo {} ({}) bloqueado 24/7 en el router.", ip, mac),
            Language::English => format!("✓ Device {} ({}) blocked 24/7 on router.", ip, mac),
            Language::Chinese => format!("✓ 设备 {} ({}) 已在路由器上永久阻止 24/7。", ip, mac),
            Language::Hindi => format!("✓ डिवाइस {} ({}) राउटर पर 24/7 ब्लॉक कर दिया गया।", ip, mac),
            Language::Arabic => format!("✓ تم حظر الجهاز {} ({}) على مدار 24/7 في الراوتر.", ip, mac),
            Language::French => format!("✓ Appareil {} ({}) bloqué 24h/24 sur le routeur.", ip, mac),
            Language::Bengali => format!("✓ ডিভাইস {} ({}) রাউটারে ২৪/৭ ব্লক করা হয়েছে।", ip, mac),
            Language::Portuguese => format!("✓ Dispositivo {} ({}) bloqueado 24/7 no roteador.", ip, mac),
            Language::Russian => format!("✓ Устройство {} ({}) заблокировано 24/7 на роутере.", ip, mac),
            Language::Urdu => format!("✓ ڈیوائس {} ({}) روٹر پر 24/7 بلاک کر دی گئی۔", ip, mac),
        }
    }

    pub fn router_unblock_success(lang: Language, ip: &str, mac: &str) -> String {
        match lang {
            Language::Spanish => format!("✓ Dispositivo {} ({}) readmitido exitosamente en el router.", ip, mac),
            Language::English => format!("✓ Device {} ({}) successfully unblocked on router.", ip, mac),
            Language::Chinese => format!("✓ 设备 {} ({}) 已在路由器上恢复访问许可。", ip, mac),
            Language::Hindi => format!("✓ डिवाइस {} ({}) को राउटर पर सफलतापूर्वक पुनः अनुमति दी गई।", ip, mac),
            Language::Arabic => format!("✓ تمت إعادة قبول الجهاز {} ({}) بنجاح في الراوتر.", ip, mac),
            Language::French => format!("✓ Appareil {} ({}) réadmis avec succès sur le routeur.", ip, mac),
            Language::Bengali => format!("✓ ডিভাইস {} ({}) সফলভাবে রাউটারে পুনঃঅনুমোদিত হয়েছে।", ip, mac),
            Language::Portuguese => format!("✓ Dispositivo {} ({}) readmitido com sucesso no roteador.", ip, mac),
            Language::Russian => format!("✓ Устройство {} ({}) успешно разблокировано на роутере.", ip, mac),
            Language::Urdu => format!("✓ ڈیوائس {} ({}) روٹر پر کامیابی سے بحال کر دی گئی۔", ip, mac),
        }
    }

    pub fn router_operation_failed(lang: Language, err: &str) -> String {
        match lang {
            Language::Spanish => format!("✗ Error en el router: {}. No se modificó el estado del dispositivo.", err),
            Language::English => format!("✗ Router error: {}. Device state was not modified.", err),
            Language::Chinese => format!("✗ 路由器错误：{}。设备状态未更改。", err),
            Language::Hindi => format!("✗ राउटर त्रुटि: {}। डिवाइस की स्थिति नहीं बदली गई।", err),
            Language::Arabic => format!("✗ خطأ في الراوتر: {}. لم يتم تعديل حالة الجهاز.", err),
            Language::French => format!("✗ Erreur du routeur : {}. L'état de l'appareil n'a pas été modifié.", err),
            Language::Bengali => format!("✗ রাউটার ত্রুটি: {}। ডিভাইসের অবস্থা পরিবর্তন করা হয়নি।", err),
            Language::Portuguese => format!("✗ Erro no roteador: {}. O estado do dispositivo não foi alterado.", err),
            Language::Russian => format!("✗ Ошибка роутера: {}. Состояние устройства не изменено.", err),
            Language::Urdu => format!("✗ روٹر کی خرابی: {}۔ ڈیوائس کی حالت تبدیل نہیں ہوئی۔", err),
        }
    }

    pub fn intruder_alert_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⚠️ NUEVO DISPOSITIVO NO RECONOCIDO EN LA RED",
            Language::English => "⚠️ NEW UNRECOGNIZED DEVICE ON NETWORK",
            Language::Chinese => "⚠️ 网络上发现新的未识别设备",
            Language::Hindi => "⚠️ नेटवर्क पर नया अपरिचित उपकरण पाया गया",
            Language::Arabic => "⚠️ جهاز جديد غير معروف تم اكتشافه على الشبكة",
            Language::French => "⚠️ NOUVEL APPAREIL NON RECONNU DÉTECTÉ",
            Language::Bengali => "⚠️ নেটওয়ার্কে নতুন অপরিচিত ডিভাইস সনাক্ত হয়েছে",
            Language::Portuguese => "⚠️ NOVO DISPOSITIVO NÃO RECONHECIDO NA REDE",
            Language::Russian => "⚠️ ОБНАРУЖЕНО НОВОЕ НЕИЗВЕСТНОЕ УСТРОЙСТВО",
            Language::Urdu => "⚠️ نیٹ ورک پر نیا نامعلوم آلہ دیکھا گیا",
        }
    }

    pub fn intruder_btn_trust(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🛡️ Confiar (Lista Blanca)",
            Language::English => "🛡️ Trust (Whitelist)",
            Language::Chinese => "🛡️ 信任并加入白名单",
            Language::Hindi => "🛡️ विश्वास करें (श्वेतसूची)",
            Language::Arabic => "🛡️ الوثوق (القائمة البيضاء)",
            Language::French => "🛡️ Faire confiance (Liste blanche)",
            Language::Bengali => "🛡️ বিশ্বাস করুন (হোয়াইটলিস্ট)",
            Language::Portuguese => "🛡️ Confiar (Lista Branca)",
            Language::Russian => "🛡️ Доверять (Белый список)",
            Language::Urdu => "🛡️ اعتماد کریں (وائٹ لسٹ)",
        }
    }

    pub fn intruder_btn_block(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🚫 Bloquear en Router 24/7",
            Language::English => "🚫 Block on Router 24/7",
            Language::Chinese => "🚫 在路由器上永久阻止 24/7",
            Language::Hindi => "🚫 राउटर पर 24/7 ब्लॉक करें",
            Language::Arabic => "🚫 حظر على الراوتر 24/7",
            Language::French => "🚫 Bloquer sur le routeur 24/7",
            Language::Bengali => "🚫 রাউটারে ২৪/৭ ব্লক করুন",
            Language::Portuguese => "🚫 Bloquear no Roteador 24/7",
            Language::Russian => "🚫 Блокировать на роутере 24/7",
            Language::Urdu => "🚫 روٹر پر 24/7 بلاک کریں",
        }
    }

    pub fn intruder_btn_dismiss(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "✕ Descartar",
            Language::English => "✕ Dismiss",
            Language::Chinese => "✕ 忽略",
            Language::Hindi => "✕ खारिज करें",
            Language::Arabic => "✕ تجاهل",
            Language::French => "✕ Ignorer",
            Language::Bengali => "✕ বাতিল",
            Language::Portuguese => "✕ Descartar",
            Language::Russian => "✕ Скрыть",
            Language::Urdu => "✕ مسترد کریں",
        }
    }

    pub fn auto_scan_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Escaneo Automático de Red (Auto-Discovery)",
            Language::English => "Automatic Network Scan (Auto-Discovery)",
            Language::Chinese => "自动网络扫描 (自动发现)",
            Language::Hindi => "स्वचालित नेटवर्क स्कैन (स्वतः खोज)",
            Language::Arabic => "الفحص التلقائي للشبكة (الاكتشاف التلقائي)",
            Language::French => "Scan réseau automatique (Auto-Discovery)",
            Language::Bengali => "স্বয়ংক্রিয় নেটওয়ার্ক স্ক্যান (অটো-ডিসকভারি)",
            Language::Portuguese => "Varredura Automática de Rede (Auto-Discovery)",
            Language::Russian => "Автоматическое сканирование сети (Автообнаружение)",
            Language::Urdu => "خودکار نیٹ ورک اسکین (آٹو ڈسکوری)",
        }
    }

    pub fn auto_scan_desc(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Mantiene el mapa de red actualizado en segundo plano para detectar intrusos y desconexiones.",
            Language::English => "Keeps the network map fresh in the background to detect intruders and disconnections.",
            Language::Chinese => "在后台自动保持网络拓扑最新，以发现新设备和断开连接。",
            Language::Hindi => "नए उपकरणों और डिस्कनेक्शन का पता लगाने के लिए पृष्ठभूमि में नेटवर्क मानचित्र ताज़ा रखता है।",
            Language::Arabic => "يحافظ على خريطة الشبكة محدثة في الخلفية لاكتشاف المتسللين وقطع الاتصال.",
            Language::French => "Garde la carte réseau à jour en arrière-plan pour détecter les intrus et déconnexions.",
            Language::Bengali => "অনুপ্রবেশকারী এবং সংযোগ বিচ্ছিন্নতা সনাক্ত করতে পটভূমিতে নেটওয়ার্ক মানচিত্র সতেজ রাখে।",
            Language::Portuguese => "Mantém o mapa de rede atualizado em segundo plano para detectar intrusos e desconexões.",
            Language::Russian => "Фоновое обновление карты сети для обнаружения посторонних и отключений.",
            Language::Urdu => "گھس بیٹھ اور منقطع ہونے کا پتہ لگانے کے لیے پس منظر میں نیٹ ورک میپ اپ ڈیٹ رکھتا ہے۔",
        }
    }

    pub fn auto_scan_disabled(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Desactivado",
            Language::English => "Disabled",
            Language::Chinese => "已关闭",
            Language::Hindi => "अक्षम",
            Language::Arabic => "معطل",
            Language::French => "Désactivé",
            Language::Bengali => "নিষ্ক্রিয়",
            Language::Portuguese => "Desativado",
            Language::Russian => "Отключено",
            Language::Urdu => "غیر فعال",
        }
    }

    pub fn auto_scan_30s(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Cada 30 s",
            Language::English => "Every 30 s",
            Language::Chinese => "每 30 秒",
            Language::Hindi => "हर 30 सेकंड",
            Language::Arabic => "كل 30 ثانية",
            Language::French => "Toutes les 30 s",
            Language::Bengali => "প্রতি ৩০ সেকেন্ড",
            Language::Portuguese => "A cada 30 s",
            Language::Russian => "Каждые 30 сек",
            Language::Urdu => "ہر 30 سیکنڈ",
        }
    }

    pub fn auto_scan_1m(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Cada 1 min",
            Language::English => "Every 1 min",
            Language::Chinese => "每 1 分钟",
            Language::Hindi => "हर 1 मिनट",
            Language::Arabic => "كل دقيقة",
            Language::French => "Toutes les 1 min",
            Language::Bengali => "প্রতি ১ মিনিট",
            Language::Portuguese => "A cada 1 min",
            Language::Russian => "Каждую 1 мин",
            Language::Urdu => "ہر 1 منٹ",
        }
    }

    pub fn auto_scan_2m(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Cada 2 min",
            Language::English => "Every 2 min",
            Language::Chinese => "每 2 分钟",
            Language::Hindi => "हर 2 मिनट",
            Language::Arabic => "كل دقيقتين",
            Language::French => "Toutes les 2 min",
            Language::Bengali => "প্রতি ২ মিনিট",
            Language::Portuguese => "A cada 2 min",
            Language::Russian => "Каждые 2 мин",
            Language::Urdu => "ہر 2 منٹ",
        }
    }

    pub fn auto_scan_5m(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Cada 5 min",
            Language::English => "Every 5 min",
            Language::Chinese => "每 5 分钟",
            Language::Hindi => "हर 5 मिनट",
            Language::Arabic => "كل 5 دقائق",
            Language::French => "Toutes les 5 min",
            Language::Bengali => "প্রতি ৫ মিনিট",
            Language::Portuguese => "A cada 5 min",
            Language::Russian => "Каждые 5 мин",
            Language::Urdu => "ہر 5 منٹ",
        }
    }

    pub fn traceroute_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Diagnóstico de Ruta (Traceroute)",
            Language::English => "Route Diagnosis (Traceroute)",
            Language::Chinese => "路由路径诊断 (Traceroute)",
            Language::Hindi => "मार्ग निदान (Traceroute)",
            Language::Arabic => "تشخيص المسار (Traceroute)",
            Language::French => "Diagnostic d'itinéraire (Traceroute)",
            Language::Bengali => "রুট রোগ নির্ণয় (Traceroute)",
            Language::Portuguese => "Diagnóstico de Rota (Traceroute)",
            Language::Russian => "Диагностика маршрута (Traceroute)",
            Language::Urdu => "روٹ تشخیص (Traceroute)",
        }
    }

    pub fn traceroute_btn_start(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🗺️ Trazar Ruta",
            Language::English => "🗺️ Trace Route",
            Language::Chinese => "🗺️ 跟踪路径",
            Language::Hindi => "🗺️ मार्ग ट्रैक करें",
            Language::Arabic => "🗺️ تتبع المسار",
            Language::French => "🗺️ Tracer l'itinéraire",
            Language::Bengali => "🗺️ রুট ট্রেস করুন",
            Language::Portuguese => "🗺️ Trazar Rota",
            Language::Russian => "🗺️ Трассировать",
            Language::Urdu => "🗺️ راستہ ٹریس کریں",
        }
    }

    pub fn traceroute_btn_tracing(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⏳ Trazando saltos...",
            Language::English => "⏳ Tracing hops...",
            Language::Chinese => "⏳ 正在探测跳数...",
            Language::Hindi => "⏳ हॉप्स ट्रैक हो रहे हैं...",
            Language::Arabic => "⏳ جاري تتبع القفزات...",
            Language::French => "⏳ Traçage des sauts...",
            Language::Bengali => "⏳ হপস ট্রেস হচ্ছে...",
            Language::Portuguese => "⏳ Rastreando saltos...",
            Language::Russian => "⏳ Трассировка узлов...",
            Language::Urdu => "⏳ ہاپس ٹریس ہو رہے ہیں...",
        }
    }

    pub fn traceroute_hop_col(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Salto",
            Language::English => "Hop",
            Language::Chinese => "跳数",
            Language::Hindi => "हॉप",
            Language::Arabic => "قفزة",
            Language::French => "Saut",
            Language::Bengali => "হপ",
            Language::Portuguese => "Salto",
            Language::Russian => "Узел",
            Language::Urdu => "ہاپ",
        }
    }

    pub fn traceroute_ip_col(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Dirección IP / Host",
            Language::English => "IP Address / Host",
            Language::Chinese => "IP 地址 / 主机",
            Language::Hindi => "आईपी पता / होस्ट",
            Language::Arabic => "عنوان IP / المضيف",
            Language::French => "Adresse IP / Hôte",
            Language::Bengali => "আইপি ঠিকানা / হোস্ট",
            Language::Portuguese => "Endereço IP / Host",
            Language::Russian => "IP-адрес / Хост",
            Language::Urdu => "آئی پی ایڈریس / ہوسٹ",
        }
    }

    pub fn traceroute_latency_col(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Latencia",
            Language::English => "Latency",
            Language::Chinese => "延迟",
            Language::Hindi => "विलंबता",
            Language::Arabic => "زمن الوصول",
            Language::French => "Latence",
            Language::Bengali => "লেটেন্সি",
            Language::Portuguese => "Latência",
            Language::Russian => "Задержка",
            Language::Urdu => "تاخیر",
        }
    }

    pub fn traceroute_timeout(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Sin respuesta (* * *)",
            Language::English => "No response (* * *)",
            Language::Chinese => "无响应 (* * *)",
            Language::Hindi => "कोई प्रतिक्रिया नहीं (* * *)",
            Language::Arabic => "لا يوجد رد (* * *)",
            Language::French => "Pas de réponse (* * *)",
            Language::Bengali => "কোনও প্রতিক্রিয়া নেই (* * *)",
            Language::Portuguese => "Sem resposta (* * *)",
            Language::Russian => "Нет ответа (* * *)",
            Language::Urdu => "کوئی جواب نہیں (* * *)",
        }
    }

    pub fn topology_wifi_5g(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Wi-Fi 5 GHz",
            Language::English => "Wi-Fi 5 GHz",
            Language::Chinese => "Wi-Fi 5 GHz",
            Language::Hindi => "वाई-फ़ाई 5 GHz",
            Language::Arabic => "واي فاي 5 جيجاهرتز",
            Language::French => "Wi-Fi 5 GHz",
            Language::Bengali => "ওয়াই-ফাই ৫ গিগাহার্টজ",
            Language::Portuguese => "Wi-Fi 5 GHz",
            Language::Russian => "Wi-Fi 5 ГГц",
            Language::Urdu => "وائی فائی 5 گیگا ہرٹز",
        }
    }

    pub fn topology_wifi_24g(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Wi-Fi 2.4 GHz",
            Language::English => "Wi-Fi 2.4 GHz",
            Language::Chinese => "Wi-Fi 2.4 GHz",
            Language::Hindi => "वाई-फ़ाई 2.4 GHz",
            Language::Arabic => "واي فاي 2.4 جيجاهرتز",
            Language::French => "Wi-Fi 2.4 GHz",
            Language::Bengali => "ওয়াই-ফাই ২.৪ গিগাহার্টজ",
            Language::Portuguese => "Wi-Fi 2.4 GHz",
            Language::Russian => "Wi-Fi 2.4 ГГц",
            Language::Urdu => "وائی فائی 2.4 گیگا ہرٹز",
        }
    }

    pub fn topology_ethernet(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Cable Ethernet",
            Language::English => "Ethernet Cable",
            Language::Chinese => "以太网有线",
            Language::Hindi => "ईथरनेट केबल",
            Language::Arabic => "كابل إيثرنت",
            Language::French => "Câble Ethernet",
            Language::Bengali => "ইথারনেট কেবল",
            Language::Portuguese => "Cabo Ethernet",
            Language::Russian => "Кабель Ethernet",
            Language::Urdu => "ایتھرنیٹ کیبل",
        }
    }

    pub fn autostart_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Iniciar Automáticamente con el Sistema (Segundo Plano)",
            Language::English => "Launch Automatically on System Startup (Background)",
            Language::Chinese => "随系统自动启动 (后台静默运行)",
            Language::Hindi => "सिस्टम स्टार्टअप पर स्वचालित रूप से प्रारंभ करें (पृष्ठभूमि)",
            Language::Arabic => "التشغيل التلقائي مع بدء تشغيل النظام (في الخلفية)",
            Language::French => "Démarrer automatiquement avec le système (Arrière-plan)",
            Language::Bengali => "সিস্টেম স্টার্টআপে স্বয়ংক্রিয়ভাবে চালু করুন (পটভূমি)",
            Language::Portuguese => "Iniciar Automaticamente com o Sistema (Segundo Plano)",
            Language::Russian => "Автозапуск при старте системы (В фоне)",
            Language::Urdu => "سسٹم اسٹارٹ اپ پر خودکار آغاز (پس منظر)",
        }
    }

    pub fn autostart_desc(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Ejecuta NetLens minimizado al encender el equipo para vigilar la red continuamente y avisar de intrusos.",
            Language::English => "Runs NetLens minimized on system boot to continuously monitor network activity and alert on intruders.",
            Language::Chinese => "开机时在后台静默运行 NetLens，以持续监视局域网安全并在发现新设备时发出警报。",
            Language::Hindi => "नेटवर्क की निरंतर निगरानी करने और घुसपैठियों के बारे में सचेत करने के लिए स्टार्टअप पर NetLens चलाता है।",
            Language::Arabic => "تشغيل NetLens مصغراً عند بدء التشغيل لمراقبة نشاط الشبكة باستمرار والتنبيه عند وجود متسللين.",
            Language::French => "Lance NetLens minimisé au démarrage pour surveiller le réseau en continu et alerter des intrus.",
            Language::Bengali => "নেটওয়ার্ক কার্যকলাপ ক্রমাগত নিরীক্ষণ করতে এবং অনুপ্রবেশকারীদের সম্পর্কে সতর্ক করতে স্টার্টআপে NetLens চালায়।",
            Language::Portuguese => "Executa o NetLens minimizado ao iniciar o sistema para monitorar a rede continuamente e alertar sobre intrusos.",
            Language::Russian => "Запуск NetLens в свернутом виде при старте для непрерывного мониторинга сети и оповещения о посторонних.",
            Language::Urdu => "نیٹ ورک کی نگرانی اور گھس بیٹھ کی اطلاع کے لیے اسٹارٹ اپ پر NetLens منیمائز چلاتا ہے۔",
        }
    }

    pub fn minimize_on_close_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Minimizar a Segundo Plano al Cerrar la Ventana",
            Language::English => "Minimize to Background on Window Close",
            Language::Chinese => "关闭窗口时最小化到后台",
            Language::Hindi => "विंडो बंद करने पर पृष्ठभूमि में छोटा करें",
            Language::Arabic => "التصغير إلى الخلفية عند إغلاق النافذة",
            Language::French => "Réduire en arrière-plan à la fermeture de la fenêtre",
            Language::Bengali => "উইন্ডো বন্ধ করার সময় পটভূমিতে ছোট করুন",
            Language::Portuguese => "Minimizar para Segundo Plano ao Fechar a Janela",
            Language::Russian => "Сворачивать в фон при закрытии окна",
            Language::Urdu => "ونڈو بند کرنے پر پس منظر میں منیمائز کریں",
        }
    }

    pub fn minimize_on_close_desc(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Al hacer clic en la '✕', la ventana se oculta/minimiza en lugar de cerrar el proceso, manteniendo el escaneo activo.",
            Language::English => "Clicking '✕' minimizes the window instead of exiting, keeping background scan and alerts active.",
            Language::Chinese => "点击 '✕' 时隐藏/最小化窗口而不是退出程序，以保持后台网络扫描和报警持续有效。",
            Language::Hindi => "'✕' पर क्लिक करने पर विंडो बंद होने के बजाय छिप जाती है, जिससे स्कैन सक्रिय रहता है।",
            Language::Arabic => "عند النقر فوق '✕'، يتم تصغير النافذة بدلاً من إنهاء البرنامج لمواصلة الفحص والتنبيهات.",
            Language::French => "Cliquer sur '✕' réduit la fenêtre au lieu de quitter, maintenant le scan et les alertes actifs.",
            Language::Bengali => "'✕' ক্লিক করলে উইন্ডো বন্ধ না হয়ে পটভূমিতে স্ক্যান সক্রিয় রাখে।",
            Language::Portuguese => "Clicar no '✕' minimiza a janela em vez de encerrar o programa, mantendo o escaneamento ativo.",
            Language::Russian => "Нажатие '✕' сворачивает окно вместо завершения программы, сохраняя сканирование активным.",
            Language::Urdu => "'✕' پر کلک کرنے پر پروگرام بند ہونے کے بجائے پس منظر میں اسکین فعال رکھتا ہے۔",
        }
    }

    pub fn notif_trust_success_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "✓ Dispositivo Confiable",
            Language::English => "✓ Trusted Device",
            Language::Chinese => "✓ 信任设备",
            Language::Hindi => "✓ विश्वसनीय उपकरण",
            Language::Arabic => "✓ جهاز موثوق",
            Language::French => "✓ Appareil de confiance",
            Language::Bengali => "✓ বিশ্বস্ত ডিভাইস",
            Language::Portuguese => "✓ Dispositivo Confiável",
            Language::Russian => "✓ Доверенное устройство",
            Language::Urdu => "✓ قابل اعتماد ڈیوائس",
        }
    }

    pub fn notif_trust_success_body(lang: Language, mac: &str) -> String {
        match lang {
            Language::Spanish => format!("La dirección MAC {} ha sido añadida a la lista blanca.", mac),
            Language::English => format!("MAC address {} has been added to the whitelist.", mac),
            Language::Chinese => format!("MAC 地址 {} 已成功加入白名单。", mac),
            Language::Hindi => format!("MAC पता {} श्वेतसूची में जोड़ दिया गया है।", mac),
            Language::Arabic => format!("تمت إضافة عنوان MAC {} إلى القائمة البيضاء بنجاح.", mac),
            Language::French => format!("L'adresse MAC {} a été ajoutée à la liste blanche.", mac),
            Language::Bengali => format!("MAC ঠিকানা {} সফলভাবে হোয়াইটলিস্টে যুক্ত হয়েছে।", mac),
            Language::Portuguese => format!("O endereço MAC {} foi adicionado à lista branca.", mac),
            Language::Russian => format!("MAC-адрес {} успешно добавлен в белый список.", mac),
            Language::Urdu => format!("MAC ایڈریس {} وائٹ لسٹ میں شامل کر دیا گیا ہے۔", mac),
        }
    }

    pub fn notif_block_success_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🚫 Dispositivo Bloqueado en Router",
            Language::English => "🚫 Device Blocked on Router",
            Language::Chinese => "🚫 设备已在路由器上阻止",
            Language::Hindi => "🚫 डिवाइस राउटर पर ब्लॉक किया गया",
            Language::Arabic => "🚫 تم حظر الجهاز على الراوتر",
            Language::French => "🚫 Appareil bloqué sur le routeur",
            Language::Bengali => "🚫 রাউটারে ডিভাইস ব্লক করা হয়েছে",
            Language::Portuguese => "🚫 Dispositivo Bloqueado no Roteador",
            Language::Russian => "🚫 Устройство заблокировано на роутере",
            Language::Urdu => "🚫 ڈیوائس روٹر پر بلاک کر دی گئی",
        }
    }

    pub fn notif_block_success_body(lang: Language, mac: &str) -> String {
        match lang {
            Language::Spanish => format!("La MAC {} ha sido bloqueada permanentemente 24/7 en el router.", mac),
            Language::English => format!("MAC {} has been permanently blocked 24/7 on the router.", mac),
            Language::Chinese => format!("MAC 地址 {} 已在路由器上永久 24/7 阻止。", mac),
            Language::Hindi => format!("MAC {} को राउटर पर 24/7 स्थायी रूप से ब्लॉक कर दिया गया है।", mac),
            Language::Arabic => format!("تم حظر MAC {} نهائياً 24/7 على الراوتر.", mac),
            Language::French => format!("La MAC {} a été bloquée de façon permanente 24/7 sur le routeur.", mac),
            Language::Bengali => format!("MAC {} রাউটারে ২৪/৭ স্থায়ীভাবে ব্লক করা হয়েছে।", mac),
            Language::Portuguese => format!("O MAC {} foi bloqueado permanentemente 24/7 no roteador.", mac),
            Language::Russian => format!("MAC-адрес {} навсегда заблокирован на роутере 24/7.", mac),
            Language::Urdu => format!("MAC {} کو روٹر پر 24/7 مستقل طور پر بلاک کر دیا گیا ہے۔", mac),
        }
    }

    pub fn notif_require_pass_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⚠️ Contraseña de Router Requerida",
            Language::English => "⚠️ Router Password Required",
            Language::Chinese => "⚠️ 需要路由器密码",
            Language::Hindi => "⚠️ राउटर पासवर्ड आवश्यक है",
            Language::Arabic => "⚠️ كلمة مرور الراوتر مطلوبة",
            Language::French => "⚠️ Mot de passe du routeur requis",
            Language::Bengali => "⚠️ রাউটার পাসওয়ার্ড প্রয়োজন",
            Language::Portuguese => "⚠️ Senha do Roteador Necessária",
            Language::Russian => "⚠️ Требуется пароль роутера",
            Language::Urdu => "⚠️ روٹر پاس ورڈ درکار ہے",
        }
    }

    pub fn notif_require_pass_body(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Para bloquear dispositivos en el router 24/7, introduce la contraseña del router en Ajustes de NetLens.",
            Language::English => "To block devices on the router 24/7, enter your router password in NetLens Settings.",
            Language::Chinese => "要在路由器上 24/7 阻止设备，请先在 NetLens 设置中输入路由器管理密码。",
            Language::Hindi => "राउटर पर 24/7 ब्लॉक करने के लिए, NetLens सेटिंग्स में राउटर पासवर्ड दर्ज करें।",
            Language::Arabic => "لحظر الأجهزة على الراوتر 24/7، يرجى إدخال كلمة مرور الراوتر في إعدادات NetLens.",
            Language::French => "Pour bloquer sur le routeur 24/7, entrez le mot de passe du routeur dans les paramètres NetLens.",
            Language::Bengali => "রাউটারে ২৪/৭ ব্লক করতে, NetLens সেটিংসে রাউটার পাসওয়ার্ড লিখুন।",
            Language::Portuguese => "Para bloquear dispositivos no roteador 24/7, insira a senha do roteador nas Configurações do NetLens.",
            Language::Russian => "Для блокировки на роутере 24/7 введите пароль роутера в настройках NetLens.",
            Language::Urdu => "روٹر پر 24/7 بلاک کرنے کے لیے، براہ کرم NetLens سیٹنگز میں روٹر پاس ورڈ درج کریں۔",
        }
    }

    // Audit, History, Wi-Fi, and UPnP Features
    pub fn security_audit_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🛡️ Auditoría",
            Language::English => "🛡️ Audit",
            Language::Chinese => "🛡️ 安全审计",
            Language::Hindi => "🛡️ सुरक्षा ऑडिट",
            Language::Arabic => "🛡️ تدقيق الأمان",
            Language::French => "🛡️ Audit Sécurité",
            Language::Bengali => "🛡️ সিকিউরিটি অডিট",
            Language::Portuguese => "🛡️ Auditoria",
            Language::Russian => "🛡️ Аудит",
            Language::Urdu => "🛡️ سیکیورٹی آڈٹ",
        }
    }

    pub fn timeline_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "📋 Historial",
            Language::English => "📋 Timeline",
            Language::Chinese => "📋 事件历史",
            Language::Hindi => "📋 इतिहास",
            Language::Arabic => "📋 السجل",
            Language::French => "📋 Historique",
            Language::Bengali => "📋 টাইমলাইন",
            Language::Portuguese => "📋 Histórico",
            Language::Russian => "📋 История",
            Language::Urdu => "📋 ٹائم لائن",
        }
    }

    pub fn wifi_analyzer_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "📶 Wi-Fi",
            Language::English => "📶 Wi-Fi",
            Language::Chinese => "📶 Wi-Fi",
            Language::Hindi => "📶 Wi-Fi",
            Language::Arabic => "📶 Wi-Fi",
            Language::French => "📶 Wi-Fi",
            Language::Bengali => "📶 Wi-Fi",
            Language::Portuguese => "📶 Wi-Fi",
            Language::Russian => "📶 Wi-Fi",
            Language::Urdu => "📶 Wi-Fi",
        }
    }

    pub fn upnp_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🌐 Router UPnP",
            Language::English => "🌐 Router UPnP",
            Language::Chinese => "🌐 路由器 UPnP",
            Language::Hindi => "🌐 राउटर UPnP",
            Language::Arabic => "🌐 راوتر UPnP",
            Language::French => "🌐 Routeur UPnP",
            Language::Bengali => "🌐 রাউটার UPnP",
            Language::Portuguese => "🌐 Roteador UPnP",
            Language::Russian => "🌐 Роутер UPnP",
            Language::Urdu => "🌐 روٹر UPnP",
        }
    }

    // --- System Notifications and Parameterizable Settings ---

    pub fn notif_settings_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🔔 Notificaciones del Sistema y Alertas",
            Language::English => "🔔 System Notifications & Alerts",
            Language::Chinese => "🔔 系统通知与警报",
            Language::Hindi => "🔔 सिस्टम सूचनाएं और अलर्ट",
            Language::Arabic => "🔔 إشعارات وتنبيهات النظام",
            Language::French => "🔔 Notifications et alertes système",
            Language::Bengali => "🔔 সিস্টেম বিজ্ঞপ্তি ও সতর্কতা",
            Language::Portuguese => "🔔 Notificações do Sistema e Alertas",
            Language::Russian => "🔔 Системные уведомления и оповещения",
            Language::Urdu => "🔔 سسٹم کی اطلاعات اور انتباہات",
        }
    }

    pub fn notif_settings_desc(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Configura el comportamiento, avisos sonoros y nivel de detalle para alertas de red.",
            Language::English => "Configure behavior, sound alerts, and detail level for network notifications.",
            Language::Chinese => "配置网络警报的行为、声音提示和详细级别。",
            Language::Hindi => "नेटवर्क अलर्ट के लिए व्यवहार, ध्वनि और विवरण स्तर कॉन्फ़िगर करें।",
            Language::Arabic => "تكوين السلوك والتنبيهات الصوتية ومستوى التفاصيل لتنبيهات الشبكة.",
            Language::French => "Configurez le comportement, les alertes sonores et le niveau de détail du réseau.",
            Language::Bengali => "নেটওয়ার্ক সতর্কতার জন্য আচরণ, শব্দ এবং বিস্তারিত স্তর কনফিগার করুন।",
            Language::Portuguese => "Configure o comportamento, avisos sonoros e nível de detalhe para alertas de rede.",
            Language::Russian => "Настройка поведения, звуковых сигналов и уровня детализации сетевых оповещений.",
            Language::Urdu => "نیٹ ورک انتباہات کے لیے رویہ، صوتی انتباہات اور تفصیل کی سطح وضع کریں۔",
        }
    }

    pub fn notif_master_enabled(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "✓ Notificaciones Activadas",
            Language::English => "✓ Notifications Enabled",
            Language::Chinese => "✓ 已启用通知",
            Language::Hindi => "✓ सूचनाएं सक्षम हैं",
            Language::Arabic => "✓ تم تفعيل الإشعارات",
            Language::French => "✓ Notifications activées",
            Language::Bengali => "✓ বিজ্ঞপ্তি সক্রিয়",
            Language::Portuguese => "✓ Notificações Ativadas",
            Language::Russian => "✓ Уведомления включены",
            Language::Urdu => "✓ اطلاعات فعال ہیں",
        }
    }

    pub fn notif_master_disabled(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "✕ Notificaciones Desactivadas",
            Language::English => "✕ Notifications Disabled",
            Language::Chinese => "✕ 已禁用通知",
            Language::Hindi => "✕ सूचनाएं अक्षम हैं",
            Language::Arabic => "✕ تم تعطيل الإشعارات",
            Language::French => "✕ Notifications désactivées",
            Language::Bengali => "✕ বিজ্ঞপ্তি নিষ্ক্রিয়",
            Language::Portuguese => "✕ Notificações Desativadas",
            Language::Russian => "✕ Уведомления отключены",
            Language::Urdu => "✕ اطلاعات غیر فعال ہیں",
        }
    }

    pub fn notif_trigger_unknown(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Nuevos dispositivos desconocidos (Intrusos)",
            Language::English => "New unknown devices (Intruders)",
            Language::Chinese => "新未知设备（入侵者）",
            Language::Hindi => "नए अज्ञात उपकरण (घुसपैठिए)",
            Language::Arabic => "أجهزة جديدة غير معروفة (دخلاء)",
            Language::French => "Nouveaux appareils inconnus (Intrus)",
            Language::Bengali => "নতুন অজানা ডিভাইস (অনুপ্রবেশকারী)",
            Language::Portuguese => "Novos dispositivos desconhecidos (Intrusos)",
            Language::Russian => "Новые неизвестные устройства (Злоумышленники)",
            Language::Urdu => "نئی نامعلوم ڈیوائسز (حملہ آور)",
        }
    }

    pub fn notif_trigger_unwhitelisted_reconnect(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Reconexión de no autorizados (Fuera de lista blanca)",
            Language::English => "Unauthorized reconnections (Not in whitelist)",
            Language::Chinese => "非白名单设备重新连接",
            Language::Hindi => "गैर-श्वेतसूचीबद्ध उपकरणों का पुनः कनेक्शन",
            Language::Arabic => "إعادة اتصال أجهزة غير مصرح بها (خارج القائمة البيضاء)",
            Language::French => "Reconnexion d'appareils non autorisés (Hors liste blanche)",
            Language::Bengali => "অননুমোদিত ডিভাইসের পুনঃসংযোগ (হোয়াইটলিস্ট বহির্ভূত)",
            Language::Portuguese => "Reconexão de não autorizados (Fora da lista branca)",
            Language::Russian => "Повторное подключение неавторизованных устройств",
            Language::Urdu => "غیر مجاز ڈیوائسز کا دوبارہ رابطہ (وائٹ لسٹ سے باہر)",
        }
    }

    pub fn notif_trigger_whitelisted_reconnect(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Reconexión de autorizados (Lista blanca)",
            Language::English => "Authorized reconnections (Whitelist)",
            Language::Chinese => "白名单设备重新连接",
            Language::Hindi => "श्वेतसूचीबद्ध उपकरणों का पुनः कनेक्शन",
            Language::Arabic => "إعادة اتصال الأجهزة الموثوقة (القائمة البيضاء)",
            Language::French => "Reconnexion d'appareils autorisés (Liste blanche)",
            Language::Bengali => "অনুমোদিত ডিভাইসের পুনঃসংযোগ (হোয়াইটলিস্ট)",
            Language::Portuguese => "Reconexão de autorizados (Lista branca)",
            Language::Russian => "Повторное подключение доверенных устройств",
            Language::Urdu => "مجاز ڈیوائسز کا دوبارہ رابطہ (وائٹ لسٹ)",
        }
    }

    pub fn notif_trigger_disconnected(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Desconexión de dispositivos",
            Language::English => "Device disconnections",
            Language::Chinese => "设备断开连接",
            Language::Hindi => "उपकरण डिस्कनेक्शन",
            Language::Arabic => "انقطاع اتصال الأجهزة",
            Language::French => "Déconnexion d'appareils",
            Language::Bengali => "ডিভাইস সংযোগ বিচ্ছিন্নতা",
            Language::Portuguese => "Desconexão de dispositivos",
            Language::Russian => "Отключение устройств от сети",
            Language::Urdu => "ڈیوائس کا منقطع ہونا",
        }
    }

    pub fn notif_trigger_outages(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Caídas y restablecimiento de Internet",
            Language::English => "Internet outages & recovery",
            Language::Chinese => "互联网中断与恢复",
            Language::Hindi => "इंटरनेट आउटेज और रिकवरी",
            Language::Arabic => "انقطاع الإنترنت واستعادته",
            Language::French => "Coupures et rétablissement d'Internet",
            Language::Bengali => "ইন্টারনেট বিভ্রাট ও পুনরুদ্ধার",
            Language::Portuguese => "Quedas e restabelecimento de Internet",
            Language::Russian => "Сбои и восстановление интернета",
            Language::Urdu => "انٹرنیٹ کی بندش اور بحالی",
        }
    }

    pub fn notif_trigger_security(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Alertas críticas de auditoría de seguridad",
            Language::English => "Critical security audit alerts",
            Language::Chinese => "严重安全审计警报",
            Language::Hindi => "गंभीर सुरक्षा ऑडिट अलर्ट",
            Language::Arabic => "تنبيهات أمنية حرجة من التدقيق",
            Language::French => "Alertes critiques d'audit de sécurité",
            Language::Bengali => "গুরুত্বপূর্ণ নিরাপত্তা অডিট সতর্কতা",
            Language::Portuguese => "Alertas críticos de auditoria de segurança",
            Language::Russian => "Критические оповещения аудита безопасности",
            Language::Urdu => "سیکیورٹی آڈٹ کے اہم انتباہات",
        }
    }

    pub fn notif_urgency_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Urgencia:",
            Language::English => "Urgency:",
            Language::Chinese => "紧急程度:",
            Language::Hindi => "तीव्रता:",
            Language::Arabic => "الأهمية:",
            Language::French => "Urgence :",
            Language::Bengali => "জরুরিতা:",
            Language::Portuguese => "Urgência:",
            Language::Russian => "Срочность:",
            Language::Urdu => "ضرورت:",
        }
    }

    pub fn notif_timeout_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Duración:",
            Language::English => "Duration:",
            Language::Chinese => "持续时间:",
            Language::Hindi => "अवधि:",
            Language::Arabic => "المدة:",
            Language::French => "Durée :",
            Language::Bengali => "সময়সীমা:",
            Language::Portuguese => "Duração:",
            Language::Russian => "Длительность:",
            Language::Urdu => "دورانیہ:",
        }
    }

    pub fn notif_sound_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Alerta sonora del sistema",
            Language::English => "System sound alert",
            Language::Chinese => "播放系统提示音",
            Language::Hindi => "सिस्टम ध्वनि चेतावनी",
            Language::Arabic => "تنبيه صوتي من النظام",
            Language::French => "Alerte sonore système",
            Language::Bengali => "সিস্টেম শব্দ সতর্কতা",
            Language::Portuguese => "Alerta sonoro do sistema",
            Language::Russian => "Звуковой сигнал системы",
            Language::Urdu => "سسٹم صوتی انتباہ",
        }
    }

    pub fn notif_detail_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Nivel de detalle:",
            Language::English => "Detail level:",
            Language::Chinese => "详细级别:",
            Language::Hindi => "विवरण स्तर:",
            Language::Arabic => "مستوى التفاصيل:",
            Language::French => "Niveau de détail :",
            Language::Bengali => "বিস্তারিত স্তর:",
            Language::Portuguese => "Nível de detalhe:",
            Language::Russian => "Уровень подробностей:",
            Language::Urdu => "تفصیل کی سطح:",
        }
    }

    pub fn notif_show_mac_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Mostrar dirección MAC",
            Language::English => "Show MAC address",
            Language::Chinese => "显示 MAC 地址",
            Language::Hindi => "MAC पता दिखाएं",
            Language::Arabic => "عرض عنوان MAC",
            Language::French => "Afficher l'adresse MAC",
            Language::Bengali => "MAC ঠিকানা দেখান",
            Language::Portuguese => "Mostrar endereço MAC",
            Language::Russian => "Показывать MAC-адрес",
            Language::Urdu => "MAC ایڈریس دکھائیں",
        }
    }

    pub fn notif_show_vendor_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Mostrar fabricante del equipo",
            Language::English => "Show device vendor",
            Language::Chinese => "显示设备制造商",
            Language::Hindi => "निर्माता दिखाएं",
            Language::Arabic => "عرض الشركة المصنعة",
            Language::French => "Afficher le fabricant",
            Language::Bengali => "ডিভাইস প্রস্তুতকারক দেখান",
            Language::Portuguese => "Mostrar fabricante do dispositivo",
            Language::Russian => "Показывать производителя",
            Language::Urdu => "ڈیوائس کا مینوفیکچرر دکھائیں",
        }
    }

    pub fn notif_prefix_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Prefijo de notificación:",
            Language::English => "Notification prefix:",
            Language::Chinese => "通知前缀:",
            Language::Hindi => "सूचना उपसर्ग:",
            Language::Arabic => "بادئة الإشعار:",
            Language::French => "Préfixe de notification :",
            Language::Bengali => "বিজ্ঞপ্তি প্রিফিক্স:",
            Language::Portuguese => "Prefixo da notificação:",
            Language::Russian => "Префикс уведомлений:",
            Language::Urdu => "اطلاع کا سابقہ:",
        }
    }

    pub fn notif_test_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🔔 Probar Notificación",
            Language::English => "🔔 Test Notification",
            Language::Chinese => "🔔 测试系统通知",
            Language::Hindi => "🔔 परीक्षण अधिसूचना",
            Language::Arabic => "🔔 اختبار الإشعار",
            Language::French => "🔔 Tester la notification",
            Language::Bengali => "🔔 বিজ্ঞপ্তি পরীক্ষা",
            Language::Portuguese => "🔔 Testar Notificação",
            Language::Russian => "🔔 Тестовое уведомление",
            Language::Urdu => "🔔 نوٹیفکیشن ٹیسٹ کریں",
        }
    }

    pub fn notif_reconnect_unwhitelisted_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⚠️ Dispositivo No Autorizado Reconectado",
            Language::English => "⚠️ Unauthorized Device Reconnected",
            Language::Chinese => "⚠️ 未授权设备重新连线",
            Language::Hindi => "⚠️ अनधिकृत उपकरण पुनः कनेक्ट हुआ",
            Language::Arabic => "⚠️ جهاز غير مصرح به أعاد الاتصال",
            Language::French => "⚠️ Appareil non autorisé reconnecté",
            Language::Bengali => "⚠️ অননুমোদিত ডিভাইস পুনরায় সংযুক্ত হয়েছে",
            Language::Portuguese => "⚠️ Dispositivo Não Autorizado Reconectado",
            Language::Russian => "⚠️ Неавторизованное устройство подключилось",
            Language::Urdu => "⚠️ غیر مجاز ڈیوائس دوبارہ منسلک ہو گئی",
        }
    }

    pub fn notif_reconnect_whitelisted_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🟢 Dispositivo Autorizado Conectado",
            Language::English => "🟢 Trusted Device Connected",
            Language::Chinese => "🟢 已信任设备已连接",
            Language::Hindi => "🟢 विश्वसनीय उपकरण कनेक्ट हुआ",
            Language::Arabic => "🟢 جهاز موثوق متصل الآن",
            Language::French => "🟢 Appareil de confiance connecté",
            Language::Bengali => "🟢 বিশ্বস্ত ডিভাইস সংযুক্ত হয়েছে",
            Language::Portuguese => "🟢 Dispositivo Confiável Conectado",
            Language::Russian => "🟢 Доверенное устройство подключено",
            Language::Urdu => "🟢 قابل اعتماد ڈیوائس منسلک ہے",
        }
    }

    pub fn notif_device_disconnected_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⚪ Dispositivo Desconectado",
            Language::English => "⚪ Device Disconnected",
            Language::Chinese => "⚪ 设备已断开",
            Language::Hindi => "⚪ उपकरण डिस्कनेक्ट हुआ",
            Language::Arabic => "⚪ تم فصل الجهاز",
            Language::French => "⚪ Appareil déconnecté",
            Language::Bengali => "⚪ ডিভাইস সংযোগ বিচ্ছিন্ন",
            Language::Portuguese => "⚪ Dispositivo Desconectado",
            Language::Russian => "⚪ Устройство отключено",
            Language::Urdu => "⚪ ڈیوائس منقطع ہو گئی",
        }
    }

    pub fn notif_realtime_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⚡ Detección Instantánea (Kernel Netlink)",
            Language::English => "⚡ Instant Detection (Kernel Netlink)",
            Language::Chinese => "⚡ 实时即时侦测 (Kernel Netlink)",
            Language::Hindi => "⚡ त्वरित पहचान (Kernel Netlink)",
            Language::Arabic => "⚡ اكتشاف فوري (Kernel Netlink)",
            Language::French => "⚡ Détection Instantanée (Kernel Netlink)",
            Language::Bengali => "⚡ তাৎক্ষণিক সনাক্তকরণ (Kernel Netlink)",
            Language::Portuguese => "⚡ Detecção Instantânea (Kernel Netlink)",
            Language::Russian => "⚡ Мгновенное обнаружение (Kernel Netlink)",
            Language::Urdu => "⚡ فوری شناخت (Kernel Netlink)",
        }
    }

    pub fn notif_realtime_desc(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Detecta conexiones en milisegundos escuchando eventos de vecindad del kernel Linux.",
            Language::English => "Detects connections in milliseconds listening to Linux kernel neighbor events.",
            Language::Chinese => "通过监听 Linux 内核邻居事件在毫秒内侦测新连接。",
            Language::Hindi => "Linux कर्नेल इवेंट्स सुनकर मिलीसेकंड में कनेक्शन का पता लगाएं।",
            Language::Arabic => "يكتشف الاتصالات في أجزاء من الثانية بالاستماع لأحداث نواة لينكس.",
            Language::French => "Détecte les connexions en millisecondes via les événements du noyau Linux.",
            Language::Bengali => "লিনাক্স কার্নেল ইভেন্ট শুনে মিলি সেকেন্ডে সংযোগ সনাক্ত করে।",
            Language::Portuguese => "Detecta conexões em milissegundos ouvindo eventos do kernel Linux.",
            Language::Russian => "Обнаруживает подключения за миллисекунды через события ядра Linux.",
            Language::Urdu => "لینکس کرنل ایونٹس کو سن کر ملی سیکنڈز میں کنکشنز کا پتہ لگاتا ہے۔",
        }
    }

    pub fn btn_enabled(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "✓ Activado",
            Language::English => "✓ Enabled",
            Language::Chinese => "✓ 已启用",
            Language::Hindi => "✓ सक्षम",
            Language::Arabic => "✓ مفعل",
            Language::French => "✓ Activé",
            Language::Bengali => "✓ সক্রিয়",
            Language::Portuguese => "✓ Ativado",
            Language::Russian => "✓ Включено",
            Language::Urdu => "✓ فعال",
        }
    }

    pub fn btn_disabled(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Desactivado",
            Language::English => "Disabled",
            Language::Chinese => "已禁用",
            Language::Hindi => "अक्षम",
            Language::Arabic => "معطل",
            Language::French => "Désactivé",
            Language::Bengali => "নিষ্ক্রিয়",
            Language::Portuguese => "Desativado",
            Language::Russian => "Отключено",
            Language::Urdu => "غیر فعال",
        }
    }

    pub fn badge_priority(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Prioritario",
            Language::English => "Priority",
            Language::Chinese => "优先",
            Language::Hindi => "प्राथमिकता",
            Language::Arabic => "أولوية",
            Language::French => "Prioritaire",
            Language::Bengali => "অগ্রাধিকার",
            Language::Portuguese => "Prioritário",
            Language::Russian => "Приоритет",
            Language::Urdu => "ترجیحی",
        }
    }

    pub fn notif_section_triggers(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Eventos y Disparadores:",
            Language::English => "Events & Triggers:",
            Language::Chinese => "事件与触发条件:",
            Language::Hindi => "इवेंट्स और ट्रिगर्स:",
            Language::Arabic => "الأحداث والمشغلات:",
            Language::French => "Événements et déclencheurs :",
            Language::Bengali => "ইভেন্ট এবং ট্রিগার:",
            Language::Portuguese => "Eventos e Disparadores:",
            Language::Russian => "События и триггеры:",
            Language::Urdu => "واقعات اور محرکات:",
        }
    }

    pub fn notif_section_behavior(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Comportamiento y Entrega:",
            Language::English => "Behavior & Delivery:",
            Language::Chinese => "行为与递送设置:",
            Language::Hindi => "व्यवहार और वितरण:",
            Language::Arabic => "السلوك والتسليم:",
            Language::French => "Comportement et diffusion :",
            Language::Bengali => "আচরণ এবং বিতরণ:",
            Language::Portuguese => "Comportamento e Entrega:",
            Language::Russian => "Поведение и доставка:",
            Language::Urdu => "رویہ اور ترسیل:",
        }
    }

    pub fn notif_section_content(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Contenido y Formato:",
            Language::English => "Content & Formatting:",
            Language::Chinese => "内容与格式:",
            Language::Hindi => "सामग्री और प्रारूप:",
            Language::Arabic => "المحتوى والتنسيق:",
            Language::French => "Contenu et formatage :",
            Language::Bengali => "বিষয়বস্তু এবং বিন্যাস:",
            Language::Portuguese => "Conteúdo e Formatação:",
            Language::Russian => "Содержимое и формат:",
            Language::Urdu => "مواد اور فارمیٹنگ:",
        }
    }

    pub fn notif_heartbeat_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⚡ Desconexión Rápida (Heartbeat)",
            Language::English => "⚡ Fast Disconnection (Heartbeat)",
            Language::Chinese => "⚡ 快速断开侦测 (Heartbeat)",
            Language::Hindi => "⚡ तेज़ डिस्कनेक्शन (Heartbeat)",
            Language::Arabic => "⚡ اكتشاف سريع لانقطاع الاتصال (Heartbeat)",
            Language::French => "⚡ Déconnexion rapide (Heartbeat)",
            Language::Bengali => "⚡ দ্রুত সংযোগ বিচ্ছিন্নতা (Heartbeat)",
            Language::Portuguese => "⚡ Desconexão Rápida (Heartbeat)",
            Language::Russian => "⚡ Быстрое отключение (Heartbeat)",
            Language::Urdu => "⚡ تیز رفتار منقطع (Heartbeat)",
        }
    }

    pub fn notif_heartbeat_desc(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Sondea equipos en línea cada 4s y alerta desconexiones en segundos en vez de minutos.",
            Language::English => "Probes online devices every 4s and alerts disconnections in seconds instead of minutes.",
            Language::Chinese => "每 4 秒探测在线设备，并在数秒内警报断开连接。",
            Language::Hindi => "हर 4 सेकंड में ऑनलाइन उपकरणों की जांच करता है और मिनटों के बजाय सेकंड में अलर्ट देता है।",
            Language::Arabic => "يفحص الأجهزة المتصلة كل 4 ثوانٍ وينبه بانقطاعها في ثوانٍ بدلاً من دقائق.",
            Language::French => "Sonde les appareils en ligne toutes les 4s et alerte en quelques secondes.",
            Language::Bengali => "প্রতি ৪ সেকেন্ডে অনলাইন ডিভাইস পরীক্ষা করে মিনিটের পরিবর্তে সেকেন্ডে সতর্কতা দেয়।",
            Language::Portuguese => "Verifica dispositivos online a cada 4s e alerta desconexões em segundos.",
            Language::Russian => "Опрашивает устройства каждые 4с и сообщает об отключении за секунды, а не минуты.",
            Language::Urdu => "ہر 4 سیکنڈ میں آن لائن ڈیوائسز کی جانچ کرتا ہے اور منٹوں کے بجائے سیکنڈز میں الرٹ دیتا ہے۔",
        }
    }

    // --- Router USB Storage and NAS ---

    pub fn storage_btn_header(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "💾 Almacenamiento Router",
            Language::English => "💾 Router Storage",
            Language::Chinese => "💾 路由器存储",
            Language::Hindi => "💾 राउटर स्टोरेज",
            Language::Arabic => "💾 مساحة تخزين الراوتر",
            Language::French => "💾 Stockage Routeur",
            Language::Bengali => "💾 রাউটার স্টোরেজ",
            Language::Portuguese => "💾 Armazenamento do Roteador",
            Language::Russian => "💾 Накопитель роутера",
            Language::Urdu => "💾 راؤٹر اسٹوریج",
        }
    }

    pub fn storage_modal_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Explorador de Almacenamiento del Router (NAS USB)",
            Language::English => "Router Storage Explorer (USB NAS)",
            Language::Chinese => "路由器存储资源管理器 (USB NAS)",
            Language::Hindi => "राउटर स्टोरेज एक्सप्लोरर (USB NAS)",
            Language::Arabic => "مستكشف تخزين الراوتر (USB NAS)",
            Language::French => "Explorateur de stockage du routeur (NAS USB)",
            Language::Bengali => "রাউটার স্টোরেজ এক্সপ্লোরার (USB NAS)",
            Language::Portuguese => "Explorador de Armazenamento do Roteador (NAS USB)",
            Language::Russian => "Проводник накопителя роутера (USB NAS)",
            Language::Urdu => "راؤٹر اسٹوریج ایکسپلورر (USB NAS)",
        }
    }

    pub fn storage_modal_desc(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Explora unidades USB conectadas al router, administra archivos, descarga y sube contenido en tu red local.",
            Language::English => "Browse USB drives connected to your router, manage files, download and upload content on your LAN.",
            Language::Chinese => "浏览连接到路由器的 USB 驱动器，管理文件，在局域网内下载和上传内容。",
            Language::Hindi => "अपने राउटर से जुड़े USB ड्राइव ब्राउज़ करें, फ़ाइलें प्रबंधित करें और स्थानीय नेटवर्क पर डाउनलोड/अपलोड करें।",
            Language::Arabic => "تصفح وحدات تخزين USB المتصلة بالراوتر، وأدر الملفات، وقم بالتنزيل والرفع عبر الشبكة المحلية.",
            Language::French => "Parcourez les clés USB connectées au routeur, gérez les fichiers, téléchargez et envoyez du contenu sur votre réseau local.",
            Language::Bengali => "আপনার রাউটারে সংযুক্ত USB ড্রাইভ ব্রাউজ করুন, ফাইল পরিচালনা করুন এবং স্থানীয় নেটওয়ার্কে ডাউনলোড/আপলোড করুন।",
            Language::Portuguese => "Navegue pelas unidades USB conectadas ao roteador, gerencie arquivos, baixe e envie conteúdo em sua rede local.",
            Language::Russian => "Просматривайте USB-накопители, подключенные к роутеру, управляйте файлами, скачивайте и загружайте данные.",
            Language::Urdu => "اپنے راؤٹر سے منسلک USB ڈرائیوز کو براؤز کریں، فائلیں منظم کریں اور نیٹ ورک پر ڈاؤن لوڈ اور اپ لوڈ کریں۔",
        }
    }

    pub fn storage_target_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "IP del Router / Servidor SMB:",
            Language::English => "Router IP / SMB Server:",
            Language::Chinese => "路由器 IP / SMB 服务器:",
            Language::Hindi => "राउटर आईपी / एसएमबी सर्वर:",
            Language::Arabic => "عنوان IP للراوتر / خادم SMB:",
            Language::French => "IP du routeur / Serveur SMB :",
            Language::Bengali => "রাউটার আইপি / SMB সার্ভার:",
            Language::Portuguese => "IP do Roteador / Servidor SMB:",
            Language::Russian => "IP роутера / SMB сервер:",
            Language::Urdu => "راؤٹر کا IP / SMB سرور:",
        }
    }

    pub fn storage_share_select(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Unidad / Recurso:",
            Language::English => "Drive / Share:",
            Language::Chinese => "驱动器 / 共享:",
            Language::Hindi => "ड्राइव / साझा:",
            Language::Arabic => "محرك الأقراص / المشاركة:",
            Language::French => "Disque / Partage :",
            Language::Bengali => "ড্রাইভ / শেয়ার:",
            Language::Portuguese => "Unidade / Compartilhamento:",
            Language::Russian => "Диск / Общий ресурс:",
            Language::Urdu => "ڈرائیو / شیئر:",
        }
    }

    pub fn storage_up_level(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⬆ Subir Nivel",
            Language::English => "⬆ Up Level",
            Language::Chinese => "⬆ 返回上一级",
            Language::Hindi => "⬆ ऊपर जाएं",
            Language::Arabic => "⬆ المستوى الأعلى",
            Language::French => "⬆ Dossier parent",
            Language::Bengali => "⬆ এক স্তর উপরে",
            Language::Portuguese => "⬆ Subir Nível",
            Language::Russian => "⬆ На уровень вверх",
            Language::Urdu => "⬆ اوپر کی سطح",
        }
    }

    pub fn storage_new_folder(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "📁 Nueva Carpeta",
            Language::English => "📁 New Folder",
            Language::Chinese => "📁 新建文件夹",
            Language::Hindi => "📁 नया फ़ोल्डर",
            Language::Arabic => "📁 مجلد جديد",
            Language::French => "📁 Nouveau dossier",
            Language::Bengali => "📁 নতুন ফোল্ডার",
            Language::Portuguese => "📁 Nova Pasta",
            Language::Russian => "📁 Новая папка",
            Language::Urdu => "📁 نیا فولڈر",
        }
    }

    pub fn storage_upload_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "📄 Subir Archivo",
            Language::English => "📄 Upload File",
            Language::Chinese => "📄 上传文件",
            Language::Hindi => "📄 फ़ाइल अपलोड करें",
            Language::Arabic => "📄 رفع ملف",
            Language::French => "📄 Envoyer fichier",
            Language::Bengali => "📄 ফাইল আপলোড",
            Language::Portuguese => "📄 Enviar Arquivo",
            Language::Russian => "📄 Загрузить файл",
            Language::Urdu => "📄 فائل اپ لوڈ کریں",
        }
    }

    pub fn storage_upload_folder_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "📁 Subir Carpeta",
            Language::English => "📁 Upload Folder",
            Language::Chinese => "📁 上传文件夹",
            Language::Hindi => "📁 फ़ोल्डर अपलोड करें",
            Language::Arabic => "📁 رفع مجلد",
            Language::French => "📁 Envoyer dossier",
            Language::Bengali => "📁 ফোল্ডার আপলোড",
            Language::Portuguese => "📁 Enviar Pasta",
            Language::Russian => "📁 Загрузить папку",
            Language::Urdu => "📁 فولڈر اپ لوڈ کریں",
        }
    }

    pub fn storage_refresh(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🔄 Actualizar",
            Language::English => "🔄 Refresh",
            Language::Chinese => "🔄 刷新",
            Language::Hindi => "🔄 रीफ़्रेश",
            Language::Arabic => "🔄 تحديث",
            Language::French => "🔄 Actualiser",
            Language::Bengali => "🔄 রিফ্রেশ",
            Language::Portuguese => "🔄 Atualizar",
            Language::Russian => "🔄 Обновить",
            Language::Urdu => "🔄 تازہ کریں",
        }
    }

    pub fn storage_demo_mode_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🎭 Modo Demo",
            Language::English => "🎭 Demo Mode",
            Language::Chinese => "🎭 演示模式",
            Language::Hindi => "🎭 डेमो मोड",
            Language::Arabic => "🎭 وضع العرض التجريبي",
            Language::French => "🎭 Mode Démo",
            Language::Bengali => "🎭 ডেমো মোড",
            Language::Portuguese => "🎭 Modo Demo",
            Language::Russian => "🎭 Демо-режим",
            Language::Urdu => "🎭 ڈیمو موڈ",
        }
    }

    pub fn storage_filter_placeholder(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🔍 Filtrar archivos por nombre...",
            Language::English => "🔍 Filter files by name...",
            Language::Chinese => "🔍 按名称筛选文件...",
            Language::Hindi => "🔍 नाम से फ़ाइलें फ़िल्टर करें...",
            Language::Arabic => "🔍 تصفية الملفات حسب الاسم...",
            Language::French => "🔍 Filtrer les fichiers par nom...",
            Language::Bengali => "🔍 নাম অনুসারে ফাইল ফিল্টার করুন...",
            Language::Portuguese => "🔍 Filtrar arquivos por nome...",
            Language::Russian => "🔍 Фильтр файлов по имени...",
            Language::Urdu => "🔍 نام کے لحاظ سے فائلیں فلٹر کریں...",
        }
    }

    pub fn storage_col_type(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Tipo",
            Language::English => "Type",
            Language::Chinese => "类型",
            Language::Hindi => "प्रकार",
            Language::Arabic => "النوع",
            Language::French => "Type",
            Language::Bengali => "ধরন",
            Language::Portuguese => "Tipo",
            Language::Russian => "Тип",
            Language::Urdu => "قسم",
        }
    }

    pub fn storage_col_name(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Nombre",
            Language::English => "Name",
            Language::Chinese => "名称",
            Language::Hindi => "नाम",
            Language::Arabic => "الاسم",
            Language::French => "Nom",
            Language::Bengali => "নাম",
            Language::Portuguese => "Nome",
            Language::Russian => "Имя",
            Language::Urdu => "نام",
        }
    }

    pub fn storage_col_size(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Tamaño",
            Language::English => "Size",
            Language::Chinese => "大小",
            Language::Hindi => "आकार",
            Language::Arabic => "الحجم",
            Language::French => "Taille",
            Language::Bengali => "আকার",
            Language::Portuguese => "Tamanho",
            Language::Russian => "Размер",
            Language::Urdu => "سائز",
        }
    }

    pub fn storage_col_modified(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Modificado",
            Language::English => "Modified",
            Language::Chinese => "修改时间",
            Language::Hindi => "संशोधित",
            Language::Arabic => "تاريخ التعديل",
            Language::French => "Modifié",
            Language::Bengali => "সংশোধিত",
            Language::Portuguese => "Modificado",
            Language::Russian => "Изменен",
            Language::Urdu => "ترمیم شدہ",
        }
    }

    pub fn storage_col_actions(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Acciones",
            Language::English => "Actions",
            Language::Chinese => "操作",
            Language::Hindi => "कार्रवाई",
            Language::Arabic => "الإجراءات",
            Language::French => "Actions",
            Language::Bengali => "অ্যাকশন",
            Language::Portuguese => "Ações",
            Language::Russian => "Действия",
            Language::Urdu => "اقدامات",
        }
    }

    pub fn storage_action_download(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Descargar",
            Language::English => "Download",
            Language::Chinese => "下载",
            Language::Hindi => "डाउनलोड",
            Language::Arabic => "تنزيل",
            Language::French => "Télécharger",
            Language::Bengali => "ডাউনলোড",
            Language::Portuguese => "Baixar",
            Language::Russian => "Скачать",
            Language::Urdu => "ڈاؤن لوڈ",
        }
    }

    pub fn storage_action_delete(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Eliminar",
            Language::English => "Delete",
            Language::Chinese => "删除",
            Language::Hindi => "हटाएं",
            Language::Arabic => "حذف",
            Language::French => "Supprimer",
            Language::Bengali => "মুছুন",
            Language::Portuguese => "Excluir",
            Language::Russian => "Удалить",
            Language::Urdu => "حذف کریں",
        }
    }

    pub fn storage_action_details(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Detalles",
            Language::English => "Details",
            Language::Chinese => "详情",
            Language::Hindi => "विवरण",
            Language::Arabic => "التفاصيل",
            Language::French => "Détails",
            Language::Bengali => "বিবরণ",
            Language::Portuguese => "Detalhes",
            Language::Russian => "Свойства",
            Language::Urdu => "تفصیلات",
        }
    }

    pub fn storage_action_open(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Abrir",
            Language::English => "Open",
            Language::Chinese => "打开",
            Language::Hindi => "खोलें",
            Language::Arabic => "فتح",
            Language::French => "Ouvrir",
            Language::Bengali => "খুলুন",
            Language::Portuguese => "Abrir",
            Language::Russian => "Открыть",
            Language::Urdu => "کھولیں",
        }
    }

    pub fn storage_empty_dir(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Esta carpeta está vacía.",
            Language::English => "This folder is empty.",
            Language::Chinese => "此文件夹为空。",
            Language::Hindi => "यह फ़ोल्डर खाली है।",
            Language::Arabic => "هذا المجلد فارغ.",
            Language::French => "Ce dossier est vide.",
            Language::Bengali => "এই ফোল্ডারটি খালি।",
            Language::Portuguese => "Esta pasta está vazia.",
            Language::Russian => "Эта папка пуста.",
            Language::Urdu => "یہ فولڈر خالی ہے۔",
        }
    }

    pub fn storage_confirm_delete_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "¿Confirmar eliminación?",
            Language::English => "Confirm Deletion?",
            Language::Chinese => "确认删除？",
            Language::Hindi => "हटाने की पुष्टि करें?",
            Language::Arabic => "تأكيد الحذف؟",
            Language::French => "Confirmer la suppression ?",
            Language::Bengali => "মুছে ফেলার বিষয়টি নিশ্চিত করবেন?",
            Language::Portuguese => "Confirmar Exclusão?",
            Language::Russian => "Подтвердить удаление?",
            Language::Urdu => "حذف کرنے کی تصدیق کریں؟",
        }
    }

    pub fn storage_confirm_delete_msg(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "¿Estás seguro de que deseas eliminar permanentemente este elemento del almacenamiento del router?",
            Language::English => "Are you sure you want to permanently delete this item from router storage?",
            Language::Chinese => "您确定要从路由器存储中永久删除此项目吗？",
            Language::Hindi => "क्या आप वाकई इस आइटम को राउटर स्टोरेज से स्थायी रूप से हटाना चाहते हैं?",
            Language::Arabic => "هل أنت متأكد من رغبتك في حذف هذا العنصر نهائياً من وحدة تخزين الراوتر؟",
            Language::French => "Voulez-vous vraiment supprimer définitivement cet élément du stockage du routeur ?",
            Language::Bengali => "আপনি কি নিশ্চিত যে আপনি রাউটার স্টোরেজ থেকে এই আইটেমটি স্থায়ীভাবে মুছে ফেলতে চান?",
            Language::Portuguese => "Tem certeza de que deseja excluir permanentemente este item do armazenamento do roteador?",
            Language::Russian => "Вы уверены, что хотите навсегда удалить этот элемент с накопителя роутера?",
            Language::Urdu => "کیا آپ واقعی اس آئٹم کو راؤٹر اسٹوریج سے مستقل طور پر حذف کرنا چاہتے ہیں؟",
        }
    }

    pub fn storage_creds_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Credenciales de Acceso SMB / NAS",
            Language::English => "SMB / NAS Access Credentials",
            Language::Chinese => "SMB / NAS 访问凭据",
            Language::Hindi => "एसएमबी / एनएएस एक्सेस क्रेडेंशियल्स",
            Language::Arabic => "بيانات اعتماد الوصول إلى SMB / NAS",
            Language::French => "Identifiants d'accès SMB / NAS",
            Language::Bengali => "SMB / NAS অ্যাক্সেস শংসাপত্র",
            Language::Portuguese => "Credenciais de Acesso SMB / NAS",
            Language::Russian => "Учетные данные SMB / NAS",
            Language::Urdu => "SMB / NAS رسائی کی اسناد",
        }
    }

    pub fn storage_creds_desc(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Si tu router requiere usuario y contraseña para acceder al disco USB, ingrésalos aquí (déjalos vacíos para acceso anónimo/invitado).",
            Language::English => "If your router requires credentials to access the USB drive, enter them here (leave empty for anonymous/guest access).",
            Language::Chinese => "如果您的路由器需要凭据才能访问 USB 驱动器，请在此处输入（留空以匿名/来宾访问）。",
            Language::Hindi => "यदि आपके राउटर को USB ड्राइव तक पहुंचने के लिए क्रेडेंशियल्स की आवश्यकता है, तो उन्हें यहां दर्ज करें (अतिथि पहुंच के लिए खाली छोड़ें)।",
            Language::Arabic => "إذا كان الراوتر يطلب بيانات اعتماد للوصول إلى محرك أقراص USB، فأدخلها هنا (اتركها فارغة للوصول المجهول/الضيف).",
            Language::French => "Si votre routeur nécessite des identifiants pour accéder à la clé USB, saisissez-les ici (laissez vide pour l'accès invité).",
            Language::Bengali => "আপনার রাউটারের USB ড্রাইভে অ্যাক্সেসের জন্য শংসাপত্রের প্রয়োজন হলে এখানে লিখুন (অতিথির জন্য খালি রাখুন)।",
            Language::Portuguese => "Se o seu roteador exigir credenciais para acessar a unidade USB, insira-as aqui (deixe em branco para acesso de convidado).",
            Language::Russian => "Если ваш роутер требует учетные данные для доступа к USB, введите их здесь (оставьте пустыми для гостя).",
            Language::Urdu => "اگر آپ کے راؤٹر کو USB ڈرائیو تک رسائی کے لیے اسناد کی ضرورت ہے تو انہیں یہاں درج کریں (مہمان کے لیے خالی چھوڑیں)۔",
        }
    }

    pub fn storage_badge_available(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "💾 Almacenamiento USB",
            Language::English => "💾 USB Storage",
            Language::Chinese => "💾 USB 存储",
            Language::Hindi => "💾 USB स्टोरेज",
            Language::Arabic => "💾 تخزين USB",
            Language::French => "💾 Stockage USB",
            Language::Bengali => "💾 USB স্টোরেজ",
            Language::Portuguese => "💾 Armazenamento USB",
            Language::Russian => "💾 USB накопитель",
            Language::Urdu => "💾 USB اسٹوریج",
        }
    }

    pub fn storage_upload_progress_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Progreso de Subida al Router",
            Language::English => "Router Upload Progress",
            Language::Chinese => "路由器上传进度",
            Language::Hindi => "राउटर अपलोड प्रगति",
            Language::Arabic => "تقدم الرفع إلى الراوتر",
            Language::French => "Progression du téléversement",
            Language::Bengali => "রাউটার আপলোড অগ্রগতি",
            Language::Portuguese => "Progresso de Envio ao Roteador",
            Language::Russian => "Прогресс загрузки на роутер",
            Language::Urdu => "راؤٹر اپ لوڈ کی پیش رفت",
        }
    }

    pub fn storage_upload_progress_speed(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Velocidad",
            Language::English => "Speed",
            Language::Chinese => "速度",
            Language::Hindi => "गति",
            Language::Arabic => "السرعة",
            Language::French => "Vitesse",
            Language::Bengali => "গতি",
            Language::Portuguese => "Velocidade",
            Language::Russian => "Скорость",
            Language::Urdu => "رفتار",
        }
    }

    pub fn storage_upload_progress_target(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Destino:",
            Language::English => "Destination:",
            Language::Chinese => "目标:",
            Language::Hindi => "गंतव्य:",
            Language::Arabic => "الوجهة:",
            Language::French => "Destination :",
            Language::Bengali => "গন্তব্য:",
            Language::Portuguese => "Destino:",
            Language::Russian => "Назначение:",
            Language::Urdu => "منزل:",
        }
    }

    pub fn storage_upload_dismiss_btn(lang: Language) -> &'static str {
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

    // --- Tooltips across 10 Official Languages ---

    pub fn tip_settings(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Configurar idioma, tema visual y parámetros del sistema",
            Language::English => "Configure language, visual theme, and system settings",
            Language::Chinese => "配置语言、视觉主题及系统设置",
            Language::Hindi => "भाषा, दृश्य थीम और सिस्टम सेटिंग्स कॉन्फ़िगर करें",
            Language::Arabic => "تكوين اللغة والمظهر وإعدادات النظام",
            Language::French => "Configurer la langue, le thème visuel et les paramètres",
            Language::Bengali => "ভাষা, ভিজ্যুয়াল থিম এবং সিস্টেম সেটিংস কনফিগার করুন",
            Language::Portuguese => "Configurar idioma, tema visual e parâmetros do sistema",
            Language::Russian => "Настроить язык, тему оформления и параметры системы",
            Language::Urdu => "زبان، بصری تھیم اور سسٹم کی ترتیبات ترتیب دیں",
        }
    }

    pub fn tip_speed_test(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Medir velocidad de bajada, subida y latencia a Internet",
            Language::English => "Measure download speed, upload speed, and Internet latency",
            Language::Chinese => "测试互联网下载速度、上传速度和延迟",
            Language::Hindi => "डाउनलोड गति, अपलोड गति और इंटरनेट विलंबता मापें",
            Language::Arabic => "قياس سرعة التنزيل والرفع وزمن الوصول للإنترنت",
            Language::French => "Mesurer le débit descendant, montant et la latence Internet",
            Language::Bengali => "ডাউনলোড গতি, আপলোড গতি এবং ইন্টারনেট লেটেন্সি পরিমাপ করুন",
            Language::Portuguese => "Medir velocidade de download, upload e latência da Internet",
            Language::Russian => "Измерить скорость загрузки, отдачи и задержку интернета",
            Language::Urdu => "انٹرنیٹ ڈاؤن لوڈ، اپ لوڈ کی رفتار اور لیٹنسی کی پیمائش کریں",
        }
    }

    pub fn tip_security_audit(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Auditar puertos abiertos, contraseñas y riesgos de la red",
            Language::English => "Audit open ports, passwords, and local network risks",
            Language::Chinese => "审计开放端口、弱密码及局域网安全风险",
            Language::Hindi => "खुले पोर्ट, पासवर्ड और नेटवर्क सुरक्षा जोखिमों का ऑडिट करें",
            Language::Arabic => "تدقيق المنافذ المفتوحة وكلمات المرور ومخاطر الشبكة",
            Language::French => "Auditer les ports ouverts, mots de passe et risques réseau",
            Language::Bengali => "খোলা পোর্ট, পাসওয়ার্ড এবং নেটওয়ার্ক ঝুঁকি অডিট করুন",
            Language::Portuguese => "Auditar portas abertas, senhas e riscos de segurança da rede",
            Language::Russian => "Проверить открытые порты, пароли и риски безопасности сети",
            Language::Urdu => "کھلے پورٹس، پاس ورڈز اور نیٹ ورک سیکیورٹی خطرات کا آڈٹ کریں",
        }
    }

    pub fn tip_timeline(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Ver registro cronológico de eventos y conexiones de red",
            Language::English => "View chronological log of network events and connections",
            Language::Chinese => "查看网络事件与设备连接的历史时序记录",
            Language::Hindi => "नेटवर्क घटनाओं और कनेक्शनों का कालानुक्रमिक इतिहास देखें",
            Language::Arabic => "عرض السجل الزمني لأحداث الشبكة والاتصالات",
            Language::French => "Voir l'historique chronologique des événements et connexions",
            Language::Bengali => "নেটওয়ার্ক ঘটনা এবং সংযোগের ক্রমানুসারে ইতিহাস দেখুন",
            Language::Portuguese => "Ver histórico cronológico de eventos e conexões de rede",
            Language::Russian => "Просмотреть хронологический журнал событий и подключений",
            Language::Urdu => "نیٹ ورک کے واقعات اور رابطوں کا تاریخی ریکارڈ دیکھیں",
        }
    }

    pub fn tip_wifi_analyzer(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Analizar canales, interferencias y cobertura Wi-Fi",
            Language::English => "Analyze Wi-Fi channels, interference, and signal coverage",
            Language::Chinese => "分析 Wi-Fi 信道、干扰状况与信号覆盖",
            Language::Hindi => "वाई-फ़ाई चैनल, हस्तक्षेप और सिग्नल कवरेज का विश्लेषण करें",
            Language::Arabic => "تحليل قنوات Wi-Fi والتداخلات وقوة الإشارة",
            Language::French => "Analyser les canaux Wi-Fi, les interférences et le signal",
            Language::Bengali => "ওয়াই-ফাই চ্যানেল, হস্তক্ষেপ এবং সংকেত কভারেজ বিশ্লেষণ করুন",
            Language::Portuguese => "Analisar canais Wi-Fi, interferências e cobertura de sinal",
            Language::Russian => "Анализировать каналы Wi-Fi, помехи и покрытие сигнала",
            Language::Urdu => "وائی فائی چینلز، مداخلت اور سگنل کوریج کا تجزیہ کریں",
        }
    }

    pub fn tip_router_storage(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Explorar y administrar archivos en unidades USB del router",
            Language::English => "Explore and manage files on router USB storage drives",
            Language::Chinese => "浏览与管理路由器挂载的 USB 存储驱动器文件",
            Language::Hindi => "राउटर के USB स्टोरेज में फ़ाइलें एक्सप्लोर और प्रबंधित करें",
            Language::Arabic => "استكشاف وإدارة الملفات في وحدات تخزين USB بالراوتر",
            Language::French => "Explorer et gérer les fichiers sur les supports USB du routeur",
            Language::Bengali => "রাউটারের USB ড্রাইভে ফাইল এক্সপ্লোর ও পরিচালনা করুন",
            Language::Portuguese => "Explorar e gerenciar arquivos nas unidades USB do roteador",
            Language::Russian => "Просматривать и управлять файлами на USB-накопителях роутера",
            Language::Urdu => "روٹر کی USB ڈرائیوز پر فائلیں دیکھیں اور ان کا انتظام کریں",
        }
    }

    pub fn tip_export_csv(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Exportar listado de dispositivos a hoja de cálculo CSV",
            Language::English => "Export list of detected devices to CSV spreadsheet",
            Language::Chinese => "将检测到的设备列表导出为 CSV 表格",
            Language::Hindi => "पहचाने गए उपकरणों की सूची को CSV स्प्रेडशीट में निर्यात करें",
            Language::Arabic => "تصدير قائمة الأجهزة المكتشفة إلى جدول CSV",
            Language::French => "Exporter la liste des appareils vers une feuille CSV",
            Language::Bengali => "শনাক্তকৃত ডিভাইসের তালিকা CSV ফাইলে রপ্তানি করুন",
            Language::Portuguese => "Exportar lista de dispositivos para planilha CSV",
            Language::Russian => "Экспортировать список устройств в таблицу CSV",
            Language::Urdu => "شناخت شدہ آلات کی فہرست CSV اسپریڈشیٹ میں برآمد کریں",
        }
    }

    pub fn tip_export_json(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Exportar topología y datos completos de red a formato JSON",
            Language::English => "Export full network topology and device data to JSON",
            Language::Chinese => "将完整的网络拓扑与设备详细数据导出为 JSON",
            Language::Hindi => "पूर्ण नेटवर्क टोपोलॉजी और डिवाइस डेटा को JSON में निर्यात करें",
            Language::Arabic => "تصدير طبولوجيا الشبكة وبيانات الأجهزة الكاملة إلى JSON",
            Language::French => "Exporter la topologie et les données réseau en JSON",
            Language::Bengali => "সম্পূর্ণ নেটওয়ার্ক টপোলজি এবং ডেটা JSON ফরম্যাটে রপ্তানি করুন",
            Language::Portuguese => "Exportar topologia e dados completos da rede para JSON",
            Language::Russian => "Экспортировать полную топологию и данные сети в JSON",
            Language::Urdu => "مکمل نیٹ ورک ٹوپولوجی اور ڈیٹا JSON فارمیٹ میں برآمد کریں",
        }
    }

    pub fn tip_scan_network(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Escanear subred local para descubrir dispositivos conectados",
            Language::English => "Scan local subnet to discover connected devices",
            Language::Chinese => "扫描本地子网以发现连接的设备",
            Language::Hindi => "कनेक्टेड उपकरणों की खोज के लिए स्थानीय सबनेट स्कैन करें",
            Language::Arabic => "فحص الشبكة الفرعية المحلية لاكتشاف الأجهزة المتصلة",
            Language::French => "Scanner le sous-réseau local pour découvrir les appareils",
            Language::Bengali => "সংযুক্ত ডিভাইস আবিষ্কার করতে লোকাল সাবনেট স্ক্যান করুন",
            Language::Portuguese => "Escanear sub-rede local para descobrir dispositivos conectados",
            Language::Russian => "Сканировать локальную подсеть для обнаружения устройств",
            Language::Urdu => "منسلک آلات دریافت کرنے کے لیے مقامی سب نیٹ اسکین کریں",
        }
    }

    pub fn tip_copy_ip(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Copiar dirección IP al portapapeles",
            Language::English => "Copy IP address to clipboard",
            Language::Chinese => "复制 IP 地址到剪贴板",
            Language::Hindi => "IP पता क्लिपबोर्ड पर कॉपी करें",
            Language::Arabic => "نسخ عنوان IP إلى الحافظة",
            Language::French => "Copier l'adresse IP dans le presse-papiers",
            Language::Bengali => "আইপি ঠিকানা ক্লিপবোর্ডে কপি করুন",
            Language::Portuguese => "Copiar endereço IP para a área de transferência",
            Language::Russian => "Скопировать IP-адрес в буфер обмена",
            Language::Urdu => "IP ایڈریس کلپ بورڈ پر کاپی کریں",
        }
    }

    pub fn tip_copy_mac(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Copiar dirección MAC física al portapapeles",
            Language::English => "Copy physical MAC address to clipboard",
            Language::Chinese => "复制物理 MAC 地址到剪贴板",
            Language::Hindi => "भौतिक MAC पता क्लिपबोर्ड पर कॉपी करें",
            Language::Arabic => "نسخ عنوان MAC الفعلي إلى الحافظة",
            Language::French => "Copier l'adresse MAC physique dans le presse-papiers",
            Language::Bengali => "MAC ঠিকানা ক্লিপবোর্ডে কপি করুন",
            Language::Portuguese => "Copiar endereço MAC físico para a área de transferência",
            Language::Russian => "Скопировать физический MAC-адрес в буфер обмена",
            Language::Urdu => "MAC ایڈریس کلپ بورڈ پر کاپی کریں",
        }
    }

    pub fn tip_device_details(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Ver ficha técnica, puertos y herramientas de diagnóstico",
            Language::English => "View device specifications, open ports, and diagnostics",
            Language::Chinese => "查看设备详细信息、开放端口与诊断工具",
            Language::Hindi => "डिवाइस विनिर्देश, खुले पोर्ट और डायग्नोस्टिक्स देखें",
            Language::Arabic => "عرض مواصفات الجهاز والمنافذ المفتوحة وأدوات التشخيص",
            Language::French => "Voir les spécifications, ports ouverts et outils de diagnostic",
            Language::Bengali => "ডিভাইসের বিবরণ, খোলা পোর্ট এবং ডায়াগনস্টিক টুলস দেখুন",
            Language::Portuguese => "Ver especificações técnicas, portas e ferramentas de diagnóstico",
            Language::Russian => "Просмотреть параметры устройства, открытые порты и диагностику",
            Language::Urdu => "ڈیوائس کی تفصیلات، کھلے پورٹس اور تشخیصی ٹولز دیکھیں",
        }
    }

    pub fn tip_ping(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Realizar prueba de latencia ICMP en vivo hacia este equipo",
            Language::English => "Perform live ICMP ping latency test to this device",
            Language::Chinese => "向此设备执行实时 ICMP Ping 延迟测试",
            Language::Hindi => "इस डिवाइस पर लाइव ICMP पिंग विलंबता परीक्षण चलाएं",
            Language::Arabic => "إجراء اختبار زمن وصول ICMP المباشر إلى هذا الجهاز",
            Language::French => "Effectuer un test de latence Ping ICMP en direct",
            Language::Bengali => "এই ডিভাইসে লাইভ ICMP পিং লেটেন্সি পরীক্ষা চালান",
            Language::Portuguese => "Executar teste de latência Ping ICMP ao vivo para este dispositivo",
            Language::Russian => "Выполнить тест задержки ICMP Ping к этому устройству",
            Language::Urdu => "اس ڈیوائس پر لائیو ICMP پنگ ٹیسٹ چلائیں",
        }
    }

    pub fn tip_traceroute(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Rastrear ruta de paquetes y saltos de red hacia este host",
            Language::English => "Trace network packet route and hops to this host",
            Language::Chinese => "追踪到达此主机的网络数据包跳数与路由",
            Language::Hindi => "इस होस्ट तक नेटवर्क पैकेट रूट और हॉप्स ट्रेस करें",
            Language::Arabic => "تتبع مسار حزم الشبكة والقفزات إلى هذا المضيف",
            Language::French => "Tracer la route des paquets et les sauts vers cet hôte",
            Language::Bengali => "এই হোস্টের নেটওয়ার্ক প্যাকেট রুট ও হপ ট্রেস করুন",
            Language::Portuguese => "Rastrear rota de pacotes e saltos de rede para este host",
            Language::Russian => "Трассировать маршрут сетевых пакетов и узлы до этого хоста",
            Language::Urdu => "اس ہوسٹ تک نیٹ ورک پیکٹ کا راستہ اور ہاپس ٹریس کریں",
        }
    }

    pub fn tip_wol(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Enviar paquete mágico Wake-on-LAN para encender el equipo",
            Language::English => "Send Wake-on-LAN magic packet to power on device",
            Language::Chinese => "发送 Wake-on-LAN 魔术包以远程唤醒启动设备",
            Language::Hindi => "डिवाइस चालू करने के लिए Wake-on-LAN मैजिक पैकेट भेजें",
            Language::Arabic => "إرسال حزمة Wake-on-LAN السحرية لتشغيل الجهاز عن بُعد",
            Language::French => "Envoyer un paquet magique Wake-on-LAN pour allumer l'appareil",
            Language::Bengali => "ডিভাইস চালু করতে Wake-on-LAN ম্যাজিক প্যাকেট পাঠান",
            Language::Portuguese => "Enviar pacote mágico Wake-on-LAN para ligar o dispositivo",
            Language::Russian => "Отправить магический пакет Wake-on-LAN для включения устройства",
            Language::Urdu => "ڈیوائس آن کرنے کے لیے Wake-on-LAN میجک پیکٹ بھیجیں",
        }
    }

    pub fn tip_whitelist(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Marcar o desmarcar como dispositivo de confianza autorizado",
            Language::English => "Mark or unmark as trusted authorized device",
            Language::Chinese => "标记或取消标记为受信任的授权白名单设备",
            Language::Hindi => "विश्वसनीय अधिकृत डिवाइस के रूप में चिह्नित या अचिह्नित करें",
            Language::Arabic => "تحديد أو إلغاء تحديد كجهاز موثوق ومصرح به",
            Language::French => "Marquer ou démarquer comme appareil de confiance",
            Language::Bengali => "বিশ্বস্ত অনুমোদিত ডিভাইস হিসেবে চিহ্নিত বা বাতিল করুন",
            Language::Portuguese => "Marcar ou desmarcar como dispositivo confiável na lista branca",
            Language::Russian => "Добавить или удалить из списка доверенных устройств",
            Language::Urdu => "قابل اعتماد مجاز ڈیوائس کے طور پر نشان زد یا غیر نشان زد کریں",
        }
    }

    pub fn tip_block_router(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Bloquear o permitir acceso a la red en el cortafuegos del router",
            Language::English => "Block or allow network access in the router firewall",
            Language::Chinese => "在路由器防火墙中阻止或允许该设备访问网络",
            Language::Hindi => "राउटर फ़ायरवॉल में नेटवर्क एक्सेस ब्लॉक या अनुमति दें",
            Language::Arabic => "حظر أو السماح بالوصول إلى الشبكة في جدار حماية الراوتر",
            Language::French => "Bloquer ou autoriser l'accès réseau dans le pare-feu du routeur",
            Language::Bengali => "রাউটার ফায়ারওয়ালে নেটওয়ার্ক অ্যাক্সেস ব্লক বা অনুমোদন করুন",
            Language::Portuguese => "Bloquear ou permitir acesso à rede no firewall do roteador",
            Language::Russian => "Заблокировать или разрешить доступ к сети в брандмауэре роутера",
            Language::Urdu => "روٹر فائر وال میں نیٹ ورک تک رسائی کو بلاک یا اجازت دیں",
        }
    }

    pub fn tip_save_alias(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Guardar nombre personalizado para identificar este dispositivo",
            Language::English => "Save custom alias to identify this device",
            Language::Chinese => "保存自定义别名以便于识别此设备",
            Language::Hindi => "इस उपकरण की पहचान के लिए कस्टम नाम सहेजें",
            Language::Arabic => "حفظ الاسم المخصص لتمييز هذا الجهاز",
            Language::French => "Enregistrer un nom personnalisé pour identifier cet appareil",
            Language::Bengali => "এই ডিভাইস শনাক্ত করতে কাস্টম নাম সংরক্ষণ করুন",
            Language::Portuguese => "Salvar nome personalizado para identificar este dispositivo",
            Language::Russian => "Сохранить пользовательское имя для идентификации устройства",
            Language::Urdu => "اس ڈیوائس کی شناخت کے لیے اپنی مرضی کا نام محفوظ کریں",
        }
    }

    pub fn tip_close(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Cerrar esta ventana o panel",
            Language::English => "Close this window or panel",
            Language::Chinese => "关闭此窗口或面板",
            Language::Hindi => "यह विंडो या पैनल बंद करें",
            Language::Arabic => "إغلاق هذه النافذة أو اللوحة",
            Language::French => "Fermer cette fenêtre ou ce panneau",
            Language::Bengali => "এই উইন্ডো বা প্যানেলটি বন্ধ করুন",
            Language::Portuguese => "Fechar esta janela ou painel",
            Language::Russian => "Закрыть это окно или панель",
            Language::Urdu => "یہ ونڈو یا پینل بند کریں",
        }
    }

    pub fn tip_storage_refresh(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Actualizar listado de archivos y espacio libre del disco",
            Language::English => "Refresh file listing and free disk space",
            Language::Chinese => "刷新文件列表与磁盘可用空间",
            Language::Hindi => "फ़ाइल सूची और खाली डिस्क स्थान रीफ़्रेश करें",
            Language::Arabic => "تحديث قائمة الملفات ومساحة القرص المتوفرة",
            Language::French => "Actualiser la liste des fichiers et l'espace disque disponible",
            Language::Bengali => "ফাইল তালিকা এবং ডিস্কের ফাঁকা স্থান রিফ্রেশ করুন",
            Language::Portuguese => "Atualizar listagem de arquivos e espaço livre em disco",
            Language::Russian => "Обновить список файлов и свободное место на диске",
            Language::Urdu => "فائلوں کی فہرست اور خالی ڈسک کی جگہ کو ریفریش کریں",
        }
    }

    pub fn tip_storage_creds(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Configurar credenciales de acceso SMB / Samba del router",
            Language::English => "Configure router SMB / Samba access credentials",
            Language::Chinese => "配置路由器 SMB / Samba 访问凭证",
            Language::Hindi => "राउटर SMB / Samba एक्सेस क्रेडेंशियल कॉन्फ़िगर करें",
            Language::Arabic => "تكوين بيانات اعتماد وصول SMB / Samba للراوتر",
            Language::French => "Configurer les identifiants d'accès SMB / Samba du routeur",
            Language::Bengali => "রাউটার SMB / Samba অ্যাক্সেস শংসাপত্র কনফিগার করুন",
            Language::Portuguese => "Configurar credenciais de acesso SMB / Samba do roteador",
            Language::Russian => "Настроить учетные данные SMB / Samba роутера",
            Language::Urdu => "روٹر SMB / Samba رسائی کی اسناد ترتیب دیں",
        }
    }

    pub fn tip_storage_demo(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Alternar entre modo demostración interactivo y router real",
            Language::English => "Toggle between interactive demo mode and real router",
            Language::Chinese => "在交互式演示模式与实际路由器连接之间切换",
            Language::Hindi => "डेमो मोड और वास्तविक राउटर के बीच स्विच करें",
            Language::Arabic => "التبديل بين الوضع التجريبي والاتصال الحقيقي بالراوتر",
            Language::French => "Basculer entre le mode démo interactif et le routeur réel",
            Language::Bengali => "ইন্টারেক্টিভ ডেমো মোড এবং আসল রাউটারের মধ্যে টগল করুন",
            Language::Portuguese => "Alternar entre modo de demonstração interativo e roteador real",
            Language::Russian => "Переключить между демо-режимом и реальным роутером",
            Language::Urdu => "ڈیمو موڈ اور اصلی روٹر کنکشن کے درمیان سوئچ کریں",
        }
    }

    pub fn tip_storage_up(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Subir al directorio superior",
            Language::English => "Go up to parent directory",
            Language::Chinese => "返回上一级父目录",
            Language::Hindi => "ऊपरी डायरेक्टरी पर जाएं",
            Language::Arabic => "الانتقال إلى المجلد الأعلى",
            Language::French => "Monter au répertoire parent",
            Language::Bengali => "উপরের ডিরেক্টরিতে যান",
            Language::Portuguese => "Subir para o diretório pai",
            Language::Russian => "Перейти в родительский каталог",
            Language::Urdu => "اوپری ڈائریکٹری پر جائیں",
        }
    }

    pub fn tip_storage_root(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Ir al directorio raíz de la unidad",
            Language::English => "Go to storage root directory",
            Language::Chinese => "前往存储驱动器的根目录",
            Language::Hindi => "स्टोरेज की रूट डायरेक्टरी पर जाएं",
            Language::Arabic => "الانتقال إلى المجلد الجذر",
            Language::French => "Aller au répertoire racine",
            Language::Bengali => "রুট ডিরেক্টরিতে যান",
            Language::Portuguese => "Ir para o diretório raiz",
            Language::Russian => "Перейти в корневой каталог",
            Language::Urdu => "روٹ ڈائریکٹری پر جائیں",
        }
    }

    pub fn tip_storage_new_folder(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Crear una nueva carpeta en la ubicación actual",
            Language::English => "Create a new folder at current location",
            Language::Chinese => "在当前位置创建新文件夹",
            Language::Hindi => "वर्तमान स्थान पर एक नया फ़ोल्डर बनाएं",
            Language::Arabic => "إنشاء مجلد جديد في الموقع الحالي",
            Language::French => "Créer un nouveau dossier à l'emplacement actuel",
            Language::Bengali => "বর্তমান অবস্থানে একটি নতুন ফোল্ডার তৈরি করুন",
            Language::Portuguese => "Criar uma nova pasta no local atual",
            Language::Russian => "Создать новую папку в текущем каталоге",
            Language::Urdu => "موجودہ مقام پر ایک نیا فولڈر بنائیں",
        }
    }

    pub fn tip_storage_upload_file(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Seleccionar y subir un archivo desde tu equipo al router",
            Language::English => "Select and upload a file from your computer to the router",
            Language::Chinese => "选择并从计算机上传文件到路由器",
            Language::Hindi => "अपने कंप्यूटर से राउटर पर फ़ाइल चुनें और अपलोड करें",
            Language::Arabic => "تحديد وتحميل ملف من جهازك إلى الراوتر",
            Language::French => "Sélectionner et téléverser un fichier vers le routeur",
            Language::Bengali => "কম্পিউটার থেকে রাউটারে একটি ফাইল আপলোড করুন",
            Language::Portuguese => "Selecionar e enviar um arquivo do seu computador para o roteador",
            Language::Russian => "Выбрать и загрузить файл с компьютера на роутер",
            Language::Urdu => "اپنے کمپیوٹر سے روٹر پر فائل منتخب اور اپ لوڈ کریں",
        }
    }

    pub fn tip_storage_upload_folder(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Seleccionar y subir una carpeta completa con su contenido al router",
            Language::English => "Select and upload an entire folder with its contents to the router",
            Language::Chinese => "选择并上传整个文件夹及其全部内容到路由器",
            Language::Hindi => "पूरी सामग्री सहित फ़ोल्डर चुनें और राउटर पर अपलोड करें",
            Language::Arabic => "تحديد وتحميل مجلد كامل بجميع محتوياته إلى الراوتر",
            Language::French => "Sélectionner et téléverser un dossier complet vers le routeur",
            Language::Bengali => "পুরো ফোল্ডার সমস্ত ফাইলসহ রাউটারে আপলোড করুন",
            Language::Portuguese => "Selecionar e enviar uma pasta completa com seu conteúdo para o roteador",
            Language::Russian => "Выбрать и загрузить папку целиком со всем содержимым на роутер",
            Language::Urdu => "پورا فولڈر مواد سمیت روٹر پر اپ لوڈ کریں",
        }
    }

    pub fn tip_storage_open_item(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Abrir archivo con la aplicación predeterminada o explorar carpeta",
            Language::English => "Open file with default application or explore folder",
            Language::Chinese => "使用默认应用程序打开文件或浏览文件夹",
            Language::Hindi => "डिफ़ॉल्ट एप्लिकेशन के साथ फ़ाइल खोलें या फ़ोल्डर एक्सप्लोर करें",
            Language::Arabic => "فتح الملف بالتطبيق الافتراضي أو استكشاف المجلد",
            Language::French => "Ouvrir le fichier avec l'application par défaut ou explorer le dossier",
            Language::Bengali => "ডিফল্ট অ্যাপ দিয়ে ফাইল খুলুন বা ফোল্ডার দেখুন",
            Language::Portuguese => "Abrir arquivo com aplicativo padrão ou explorar pasta",
            Language::Russian => "Открыть файл в приложении по умолчанию или открыть папку",
            Language::Urdu => "پہلے سے طے شدہ ایپ سے فائل کھولیں یا فولڈر دیکھیں",
        }
    }

    pub fn tip_storage_download_item(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Descargar una copia de este archivo a tu carpeta de Descargas",
            Language::English => "Download a copy of this file to your local Downloads folder",
            Language::Chinese => "将此文件的副本下载到本地下载文件夹",
            Language::Hindi => "इस फ़ाइल की एक प्रति अपने डाउनलोड फ़ोल्डर में डाउनलोड करें",
            Language::Arabic => "تنزيل نسخة من هذا الملف إلى مجلد التنزيلات المحلي",
            Language::French => "Télécharger une copie de ce fichier dans votre dossier Téléchargements",
            Language::Bengali => "এই ফাইলের একটি কপি আপনার ডাউনলোড ফোল্ডারে ডাউনলোড করুন",
            Language::Portuguese => "Baixar uma cópia deste arquivo para sua pasta de Downloads",
            Language::Russian => "Скачать копию этого файла в локальную папку загрузок",
            Language::Urdu => "اس فائل کی کاپی اپنے ڈاؤن لوڈز فولڈر میں ڈاؤن لوڈ کریں",
        }
    }

    pub fn tip_storage_details_item(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Ver metadatos, tamaño exacto, atributos y fecha de modificación",
            Language::English => "View metadata, exact size, attributes, and modification date",
            Language::Chinese => "查看元数据、确切大小、属性和修改时间",
            Language::Hindi => "मेटाडेटा, सटीक आकार, विशेषताएँ और संशोधन तिथि देखें",
            Language::Arabic => "عرض البيانات الوصفية والحجم الدقيق والسمات وتاريخ التعديل",
            Language::French => "Voir les métadonnées, la taille exacte, les attributs et la date",
            Language::Bengali => "মেটাডেটা, সঠিক আকার, বৈশিষ্ট্য এবং পরিবর্তনের তারিখ দেখুন",
            Language::Portuguese => "Ver metadados, tamanho exato, atributos e data de modificação",
            Language::Russian => "Просмотреть метаданные, точный размер, атрибуты и дату изменения",
            Language::Urdu => "میٹا ڈیٹا، درست سائز، اوصاف اور ترمیم کی تاریخ دیکھیں",
        }
    }

    pub fn tip_storage_delete_item(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Eliminar este archivo o carpeta permanentemente del almacenamiento",
            Language::English => "Permanently delete this file or folder from storage",
            Language::Chinese => "从存储中永久删除此文件或文件夹",
            Language::Hindi => "इस फ़ाइल या फ़ोल्डर को स्टोरेज से स्थायी रूप से हटाएं",
            Language::Arabic => "حذف هذا الملف أو المجلد نهائياً من وحدة التخزين",
            Language::French => "Supprimer définitivement ce fichier ou dossier du stockage",
            Language::Bengali => "এই ফাইল বা ফোল্ডার স্টোরেজ থেকে স্থায়ীভাবে মুছুন",
            Language::Portuguese => "Excluir este arquivo ou pasta permanentemente do armazenamento",
            Language::Russian => "Безвозвратно удалить этот файл или папку с накопителя",
            Language::Urdu => "اس فائل یا فولڈر کو اسٹوریج سے مستقل طور پر حذف کریں",
        }
    }

    pub fn tip_filter_all(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Mostrar todos los dispositivos detectados en la red",
            Language::English => "Show all detected devices on the network",
            Language::Chinese => "显示网络上检测到的所有设备",
            Language::Hindi => "नेटवर्क पर पहचाने गए सभी उपकरण दिखाएं",
            Language::Arabic => "إظهار جميع الأجهزة المكتشفة على الشبكة",
            Language::French => "Afficher tous les appareils détectés sur le réseau",
            Language::Bengali => "নেটওয়ার্কে পাওয়া সমস্ত ডিভাইস দেখান",
            Language::Portuguese => "Exibir todos os dispositivos detectados na rede",
            Language::Russian => "Показать все обнаруженные в сети устройства",
            Language::Urdu => "نیٹ ورک پر پائے جانے والے تمام آلات دکھائیں",
        }
    }

    pub fn tip_filter_whitelist(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Filtrar únicamente dispositivos de confianza en la lista blanca",
            Language::English => "Filter only trusted devices in the whitelist",
            Language::Chinese => "仅筛选白名单中的受信任设备",
            Language::Hindi => "केवल श्वेतसूची में शामिल विश्वसनीय उपकरणों को फ़िल्टर करें",
            Language::Arabic => "تصفية الأجهزة الموثوقة فقط في القائمة البيضاء",
            Language::French => "Filtrer uniquement les appareils de confiance de la liste blanche",
            Language::Bengali => "শুধুমাত্র হোয়াইটলিস্টের বিশ্বস্ত ডিভাইসগুলো ফিল্টার করুন",
            Language::Portuguese => "Filtrar apenas dispositivos confiáveis da lista branca",
            Language::Russian => "Фильтровать только доверенные устройства из белого списка",
            Language::Urdu => "صرف وائٹ لسٹ میں شامل قابل اعتماد آلات کو فلٹر کریں",
        }
    }

    pub fn tip_filter_blocked(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Filtrar únicamente dispositivos bloqueados en el router",
            Language::English => "Filter only devices blocked on the router",
            Language::Chinese => "仅筛选在路由器中已被阻止的设备",
            Language::Hindi => "केवल राउटर पर अवरुद्ध उपकरणों को फ़िल्टर करें",
            Language::Arabic => "تصفية الأجهزة المحظورة على الراوتر فقط",
            Language::French => "Filtrer uniquement les appareils bloqués sur le routeur",
            Language::Bengali => "শুধুমাত্র রাউটারে ব্লক করা ডিভাইসগুলো ফিল্টার করুন",
            Language::Portuguese => "Filtrar apenas dispositivos bloqueados no roteador",
            Language::Russian => "Фильтровать только заблокированные на роутере устройства",
            Language::Urdu => "صرف وہ آلات فلٹر کریں جنہیں روٹر پر بلاک کیا گیا ہے",
        }
    }

    pub fn tip_port_scan(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Escanear puertos TCP abiertos en este dispositivo",
            Language::English => "Scan open TCP ports on this device",
            Language::Chinese => "扫描此设备上开放的 TCP 端口",
            Language::Hindi => "इस उपकरण पर खुले टीसीपी पोर्ट स्कैन करें",
            Language::Arabic => "فحص منافذ TCP المفتوحة على هذا الجهاز",
            Language::French => "Analyser les ports TCP ouverts sur cet appareil",
            Language::Bengali => "এই ডিভাইসে উন্মুক্ত টিসিپی পোর্ট স্ক্যান করুন",
            Language::Portuguese => "Escanear portas TCP abertas neste dispositivo",
            Language::Russian => "Сканировать открытые TCP-порты на этом устройстве",
            Language::Urdu => "اس ڈیوائس پر کھلی ہوئی TCP پورٹس اسکین کریں",
        }
    }

    pub fn tip_filter_category(cat: DeviceCategory, lang: Language) -> &'static str {
        match cat {
            DeviceCategory::Router => match lang {
                Language::Spanish => "Filtrar por routers y puertas de enlace",
                Language::English => "Filter by routers and gateways",
                Language::Chinese => "按路由器和网关筛选",
                Language::Hindi => "राउटर और गेटवे द्वारा फ़िल्टर करें",
                Language::Arabic => "التصفية حسب أجهزة التوجيه والبوابات",
                Language::French => "Filtrer par routeurs et passerelles",
                Language::Bengali => "রাউটার এবং গেটওয়ে দ্বারা ফিল্টার করুন",
                Language::Portuguese => "Filtrar por roteadores e gateways",
                Language::Russian => "Фильтровать по роутерам и шлюзам",
                Language::Urdu => "روٹرز اور گیٹ ویز کے مطابق فلٹر کریں",
            },
            DeviceCategory::Smartphone => match lang {
                Language::Spanish => "Filtrar por teléfonos inteligentes",
                Language::English => "Filter by smartphones",
                Language::Chinese => "按智能手机筛选",
                Language::Hindi => "स्मार्टफ़ोन द्वारा फ़िल्टर करें",
                Language::Arabic => "التصفية حسب الهواتف الذكية",
                Language::French => "Filtrer par smartphones",
                Language::Bengali => "স্মার্টফোন দ্বারা ফিল্টার করুন",
                Language::Portuguese => "Filtrar por smartphones",
                Language::Russian => "Фильтровать по смартфонам",
                Language::Urdu => "اسمارٹ فونز کے مطابق فلٹر کریں",
            },
            DeviceCategory::Tablet => match lang {
                Language::Spanish => "Filtrar por tabletas y lectores digitales",
                Language::English => "Filter by tablets and e-readers",
                Language::Chinese => "按平板电脑和电子书筛选",
                Language::Hindi => "टैबलेट और ई-रीडर द्वारा फ़िल्टर करें",
                Language::Arabic => "التصفية حسب الأجهزة اللوحية والقارئات الإلكترونية",
                Language::French => "Filtrer par tablettes et liseuses",
                Language::Bengali => "ট্যাবলেট এবং ই-রিডার দ্বারা ফিল্টার করুন",
                Language::Portuguese => "Filtrar por tablets e leitores digitais",
                Language::Russian => "Фильтровать по планшетам и электронным книгам",
                Language::Urdu => "ٹیبلٹس اور ای ریڈرز کے مطابق فلٹر کریں",
            },
            DeviceCategory::Computer => match lang {
                Language::Spanish => "Filtrar por computadoras de escritorio y portátiles",
                Language::English => "Filter by desktop and laptop computers",
                Language::Chinese => "按台式机和笔记本电脑筛选",
                Language::Hindi => "डेस्कटॉप और लैपटॉप कंप्यूटर द्वारा फ़िल्टर करें",
                Language::Arabic => "التصفية حسب أجهزة الكمبيوتر المكتبية والمحمولة",
                Language::French => "Filtrer par ordinateurs fixes et portables",
                Language::Bengali => "ডেস্কটপ এবং ল্যাপটপ কম্পিউটার দ্বারা ফিল্টার করুন",
                Language::Portuguese => "Filtrar por computadores desktop e notebooks",
                Language::Russian => "Фильтровать по настольным ПК и ноутбукам",
                Language::Urdu => "ڈیسک ٹاپ اور لیپ ٹاپ کمپیوٹرز کے مطابق فلٹر کریں",
            },
            DeviceCategory::SmartTv => match lang {
                Language::Spanish => "Filtrar por televisores inteligentes",
                Language::English => "Filter by smart TVs",
                Language::Chinese => "按智能电视筛选",
                Language::Hindi => "स्मार्ट टीवी द्वारा फ़िल्टर करें",
                Language::Arabic => "التصفية حسب أجهزة التلفزيون الذكية",
                Language::French => "Filtrer par téléviseurs intelligents",
                Language::Bengali => "স্মার্ট টিভি দ্বারা ফিল্টার করুন",
                Language::Portuguese => "Filtrar por smart TVs",
                Language::Russian => "Фильтровать по смарт-телевизорам",
                Language::Urdu => "اسمارٹ ٹی وی کے مطابق فلٹر کریں",
            },
            DeviceCategory::StreamingDevice => match lang {
                Language::Spanish => "Filtrar por dispositivos de streaming y reproductores",
                Language::English => "Filter by streaming devices and media players",
                Language::Chinese => "按流媒体和播放器设备筛选",
                Language::Hindi => "स्ट्रीमिंग डिवाइस और मीडिया प्लेयर द्वारा फ़िल्टर करें",
                Language::Arabic => "التصفية حسب أجهزة البث ومشغلات الوسائط",
                Language::French => "Filtrer par appareils de streaming et lecteurs multimédias",
                Language::Bengali => "স্ট্রিমিং ডিভাইস এবং মিডিয়া প্লেয়ার দ্বারা ফিল্টার করুন",
                Language::Portuguese => "Filtrar por dispositivos de streaming e reprodutores multimídia",
                Language::Russian => "Фильтровать по стриминговым устройствам и медиаплеерам",
                Language::Urdu => "اسٹریمنگ ڈیوائسز اور میڈیا پلیئرز کے مطابق فلٹر کریں",
            },
            DeviceCategory::GamingConsole => match lang {
                Language::Spanish => "Filtrar por consolas de videojuegos",
                Language::English => "Filter by gaming consoles",
                Language::Chinese => "按游戏主机筛选",
                Language::Hindi => "गेमिंग कंसोल द्वारा फ़िल्टर करें",
                Language::Arabic => "التصفية حسب منصات الألعاب",
                Language::French => "Filtrer par consoles de jeux",
                Language::Bengali => "গেমিং কনসোল দ্বারা ফিল্টার করুন",
                Language::Portuguese => "Filtrar por consoles de videogame",
                Language::Russian => "Фильтровать по игровым консолям",
                Language::Urdu => "گیمنگ کنسولز کے مطابق فلٹر کریں",
            },
            DeviceCategory::SmartSpeaker => match lang {
                Language::Spanish => "Filtrar por altavoces inteligentes y sistemas de audio",
                Language::English => "Filter by smart speakers and audio systems",
                Language::Chinese => "按智能音箱和音频系统筛选",
                Language::Hindi => "स्मार्ट स्पीकर और ऑडियो सिस्टम द्वारा फ़िल्टर करें",
                Language::Arabic => "التصفية حسب مكبرات الصوت الذكية وأنظمة الصوت",
                Language::French => "Filtrer par enceintes connectées et systèmes audio",
                Language::Bengali => "স্মার্ট স্পিকার এবং অডিও সিস্টেম দ্বারা ফিল্টার করুন",
                Language::Portuguese => "Filtrar por caixas de som inteligentes e sistemas de áudio",
                Language::Russian => "Фильтровать по умным колонкам и аудиосистемам",
                Language::Urdu => "اسمارٹ اسپیکرز اور آڈیو سسٹمز کے مطابق فلٹر کریں",
            },
            DeviceCategory::SecurityCamera => match lang {
                Language::Spanish => "Filtrar por cámaras de seguridad y videovigilancia",
                Language::English => "Filter by security cameras and surveillance devices",
                Language::Chinese => "按安防监控摄像头和门铃筛选",
                Language::Hindi => "सुरक्षा कैमरों और निगरानी उपकरणों द्वारा फ़िल्टर करें",
                Language::Arabic => "التصفية حسب كاميرات الأمان والمراقبة",
                Language::French => "Filtrer par caméras de sécurité et surveillance",
                Language::Bengali => "নিরাপত্তা ক্যামেরা এবং নজরদারি ডিভাইস দ্বারা ফিল্টার করুন",
                Language::Portuguese => "Filtrar por câmeras de segurança e vigilância",
                Language::Russian => "Фильтровать по камерам наблюдения и видеодомофонам",
                Language::Urdu => "سیکیورٹی کیمروں اور نگرانی کے آلات کے مطابق فلٹر کریں",
            },
            DeviceCategory::Iot => match lang {
                Language::Spanish => "Filtrar por dispositivos IoT y domótica",
                Language::English => "Filter by IoT and smart home devices",
                Language::Chinese => "按物联网与智能家居设备筛选",
                Language::Hindi => "आईओटी और स्मार्ट होम उपकरणों द्वारा फ़िल्टर करें",
                Language::Arabic => "التصفية حسب أجهزة إنترنت الأشياء والمنازل الذكية",
                Language::French => "Filtrer par appareils IoT et domotique",
                Language::Bengali => "আইওটি এবং স্মার্ট হোম ডিভাইস দ্বারা ফিল্টার করুন",
                Language::Portuguese => "Filtrar por dispositivos IoT e casa inteligente",
                Language::Russian => "Фильтровать по устройствам умного дома и IoT",
                Language::Urdu => "آئی او ٹی اور سمارٹ ہوم ڈیوائسز کے مطابق فلٹر کریں",
            },
            DeviceCategory::Printer => match lang {
                Language::Spanish => "Filtrar por impresoras y escáneres de red",
                Language::English => "Filter by network printers and scanners",
                Language::Chinese => "按网络打印机和扫描仪筛选",
                Language::Hindi => "नेटवर्क प्रिंटर और स्कैनर द्वारा फ़िल्टर करें",
                Language::Arabic => "التصفية حسب الطابعات والماسحات الضوئية الشبكية",
                Language::French => "Filtrer par imprimantes et scanners réseau",
                Language::Bengali => "নেটওয়ার্ক প্রিন্টার এবং স্ক্যানার দ্বারা ফিল্টার করুন",
                Language::Portuguese => "Filtrar por impressoras e scanners de rede",
                Language::Russian => "Фильтровать по сетевым принтерам и сканерам",
                Language::Urdu => "نیٹ ورک پرنٹرز اور اسکینرز کے مطابق فلٹر کریں",
            },
            DeviceCategory::StorageNas => match lang {
                Language::Spanish => "Filtrar por servidores de almacenamiento en red (NAS)",
                Language::English => "Filter by network attached storage (NAS) servers",
                Language::Chinese => "按网络附加存储（NAS）服务器筛选",
                Language::Hindi => "नेटवर्क अटैच्ड स्टोरेज (NAS) सर्वर द्वारा फ़िल्टर करें",
                Language::Arabic => "التصفية حسب خوادم التخزين الشبكي (NAS)",
                Language::French => "Filtrer par serveurs de stockage réseau (NAS)",
                Language::Bengali => "নেটওয়ার্ক স্টোরেজ (ন্যাস) সার্ভার দ্বারা ফিল্টার করুন",
                Language::Portuguese => "Filtrar por servidores de armazenamento em rede (NAS)",
                Language::Russian => "Фильтровать по сетевым хранилищам (NAS)",
                Language::Urdu => "نیٹ ورک اسٹوریج (این اے ایس) سرورز کے مطابق فلٹر کریں",
            },
            DeviceCategory::Server => match lang {
                Language::Spanish => "Filtrar por servidores dedicados y máquinas virtuales",
                Language::English => "Filter by dedicated servers and virtual machines",
                Language::Chinese => "按专用服务器和虚拟机筛选",
                Language::Hindi => "समर्पित सर्वर और वर्चुअल मशीनों द्वारा फ़िल्टर करें",
                Language::Arabic => "التصفية حسب الخوادم المخصصة والأجهزة الافتراضية",
                Language::French => "Filtrer par serveurs dédiés et machines virtuelles",
                Language::Bengali => "ডেডিকেটেড সার্ভার এবং ভার্চুয়াল মেশিন দ্বারা ফিল্টার করুন",
                Language::Portuguese => "Filtrar por servidores dedicados e máquinas virtuais",
                Language::Russian => "Фильтровать по серверам и виртуальным машинам",
                Language::Urdu => "وقف سرورز اور ورچوئل مشینوں کے مطابق فلٹر کریں",
            },
            DeviceCategory::Wearable => match lang {
                Language::Spanish => "Filtrar por relojes inteligentes y pulseras de actividad",
                Language::English => "Filter by smartwatches and fitness trackers",
                Language::Chinese => "按智能手表和运动手环筛选",
                Language::Hindi => "स्मार्टवॉच और फिटनेस ट्रैकर द्वारा फ़िल्टर करें",
                Language::Arabic => "التصفية حسب الساعات الذكية وأجهزة التتبع الرياضي",
                Language::French => "Filtrer par montres connectées et bracelets de fitness",
                Language::Bengali => "স্মার্টওয়াচ এবং ফিটনেস ট্র্যাকার দ্বারা ফিল্টার করুন",
                Language::Portuguese => "Filtrar por smartwatches e pulseiras de atividade",
                Language::Russian => "Фильтровать по умным часам и фитнес-трекерам",
                Language::Urdu => "اسمارٹ واچز اور فٹنس ٹریکرز کے مطابق فلٹر کریں",
            },
            DeviceCategory::NetworkBridge => match lang {
                Language::Spanish => "Filtrar por switches de red, repetidores y puntos de acceso",
                Language::English => "Filter by network switches, repeaters, and access points",
                Language::Chinese => "按网络交换机、中继器和接入点筛选",
                Language::Hindi => "नेटवर्क स्विच, रिपीटर और एक्सेस पॉइंट द्वारा फ़िल्टर करें",
                Language::Arabic => "التصفية حسب محولات الشبكة والمقويات ونقاط الوصول",
                Language::French => "Filtrer par commutateurs, répéteurs et points d'accès",
                Language::Bengali => "নেটওয়ার্ক সুইচ, রিপিটার এবং অ্যাক্সেস পয়েন্ট দ্বারা ফিল্টার করুন",
                Language::Portuguese => "Filtrar por switches de rede, repetidores e pontos de acesso",
                Language::Russian => "Фильтровать по коммутаторам, репитерам и точкам доступа",
                Language::Urdu => "نیٹ ورک سوئچز، ریپیٹرز اور ایکسیس پوائنٹس کے مطابق فلٹر کریں",
            },
            _ => match lang {
                Language::Spanish => "Filtrar dispositivos por categoría",
                Language::English => "Filter devices by category",
                Language::Chinese => "按分类筛选设备",
                Language::Hindi => "श्रेणी के अनुसार उपकरण फ़िल्टर करें",
                Language::Arabic => "تصفية الأجهزة حسب الفئة",
                Language::French => "Filtrer les appareils par catégorie",
                Language::Bengali => "বিভাগ অনুসারে ডিভাইস ফিল্টার করুন",
                Language::Portuguese => "Filtrar dispositivos por categoria",
                Language::Russian => "Фильтровать устройства по категории",
                Language::Urdu => "زمرہ کے لحاظ سے آلات فلٹر کریں",
            },
        }
    }

    // ==========================================
    // Security Audit
    // ==========================================

    pub fn audit_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🛡️ Auditoría de Seguridad de Red",
            Language::English => "🛡️ Network Security Audit",
            Language::Chinese => "🛡️ 网络安全审计",
            Language::Hindi => "🛡️ नेटवर्क सुरक्षा ऑडिट",
            Language::Arabic => "🛡️ تدقيق أمان الشبكة",
            Language::French => "🛡️ Audit de Sécurité Réseau",
            Language::Bengali => "🛡️ নেটওয়ার্ক সুরক্ষা নিরীক্ষা",
            Language::Portuguese => "🛡️ Auditoria de Segurança de Rede",
            Language::Russian => "🛡️ Аудит безопасности сети",
            Language::Urdu => "🛡️ نیٹ ورک سیکیورٹی آڈٹ",
        }
    }

    pub fn audit_subtitle(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Evaluación proactiva de riesgos, vulnerabilidades y exposición de la red local.",
            Language::English => "Proactive assessment of risks, vulnerabilities, and local network exposure.",
            Language::Chinese => "主动评估本地网络风险、漏洞和暴露面。",
            Language::Hindi => "स्थानीय नेटवर्क जोखिमों, कमजोरियों और अनावरण का सक्रिय मूल्यांकन।",
            Language::Arabic => "تقييم استباقي للمخاطر ونقاط الضعف وتعرض الشبكة المحلية.",
            Language::French => "Évaluation proactive des risques, vulnérabilités et exposition du réseau local.",
            Language::Bengali => "স্থানীয় নেটওয়ার্ক ঝুঁকি, দুর্বলতা এবং উন্মুক্ততার সক্রিয় মূল্যায়ন।",
            Language::Portuguese => "Avaliação proativa de riscos, vulnerabilidades e exposição da rede local.",
            Language::Russian => "Проактивная оценка рисков, уязвимостей и защищенности локальной сети.",
            Language::Urdu => "مقامی نیٹ ورک کے خطرات، کمزوریوں اور نمائش کا فعال جائزہ۔",
        }
    }

    pub fn audit_not_run(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "No ejecutada",
            Language::English => "Not executed",
            Language::Chinese => "未执行",
            Language::Hindi => "निष्पादित नहीं",
            Language::Arabic => "لم يتم التشغيل",
            Language::French => "Non exécuté",
            Language::Bengali => "চালানো হয়নি",
            Language::Portuguese => "Não executado",
            Language::Russian => "Не выполнялся",
            Language::Urdu => "نہیں چلایا گیا",
        }
    }

    pub fn audit_score_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Puntuación:",
            Language::English => "Score:",
            Language::Chinese => "评分:",
            Language::Hindi => "स्कोर:",
            Language::Arabic => "النتيجة:",
            Language::French => "Score :",
            Language::Bengali => "স্কোর:",
            Language::Portuguese => "Pontuação:",
            Language::Russian => "Оценка:",
            Language::Urdu => "اسکور:",
        }
    }

    pub fn audit_grade_label(score: u32, lang: Language) -> &'static str {
        match score {
            90..=100 => match lang {
                Language::Spanish => "Excelente",
                Language::English => "Excellent",
                Language::Chinese => "极佳",
                Language::Hindi => "उत्कृष्ट",
                Language::Arabic => "ممتاز",
                Language::French => "Excellent",
                Language::Bengali => "চমৎকার",
                Language::Portuguese => "Excelente",
                Language::Russian => "Отлично",
                Language::Urdu => "بہترین",
            },
            75..=89 => match lang {
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
            },
            50..=74 => match lang {
                Language::Spanish => "Advertencia",
                Language::English => "Warning",
                Language::Chinese => "警告",
                Language::Hindi => "चेतावनी",
                Language::Arabic => "تحذير",
                Language::French => "Avertissement",
                Language::Bengali => "সতর্কতা",
                Language::Portuguese => "Aviso",
                Language::Russian => "Предупреждение",
                Language::Urdu => "انتباہ",
            },
            _ => match lang {
                Language::Spanish => "En Riesgo",
                Language::English => "At Risk",
                Language::Chinese => "存在风险",
                Language::Hindi => "जोखिम में",
                Language::Arabic => "في خطر",
                Language::French => "À risque",
                Language::Bengali => "ঝুঁকিতে",
                Language::Portuguese => "Em Risco",
                Language::Russian => "Под угрозой",
                Language::Urdu => "خطرے میں",
            },
        }
    }

    pub fn audit_devices_audited(count: usize, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Dispositivos auditados: {}", count),
            Language::English => format!("Devices audited: {}", count),
            Language::Chinese => format!("已审计设备: {}", count),
            Language::Hindi => format!("ऑडिट किए गए उपकरण: {}", count),
            Language::Arabic => format!("الأجهزة المدققة: {}", count),
            Language::French => format!("Appareils audités : {}", count),
            Language::Bengali => format!("অডিট করা ডিভাইস: {}", count),
            Language::Portuguese => format!("Dispositivos auditados: {}", count),
            Language::Russian => format!("Проверено устройств: {}", count),
            Language::Urdu => format!("آڈٹ شدہ آلات: {}", count),
        }
    }

    pub fn audit_observations(count: usize, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Observaciones: {}", count),
            Language::English => format!("Observations: {}", count),
            Language::Chinese => format!("问题发现: {}", count),
            Language::Hindi => format!("अवलोकन: {}", count),
            Language::Arabic => format!("الملاحظات: {}", count),
            Language::French => format!("Observations : {}", count),
            Language::Bengali => format!("পর্যবেক্ষণ: {}", count),
            Language::Portuguese => format!("Observações: {}", count),
            Language::Russian => format!("Замечания: {}", count),
            Language::Urdu => format!("مشاہدات: {}", count),
        }
    }

    pub fn audit_critical_risks(count: usize, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Riesgos críticos: {}", count),
            Language::English => format!("Critical risks: {}", count),
            Language::Chinese => format!("严重风险: {}", count),
            Language::Hindi => format!("गंभीर जोखिम: {}", count),
            Language::Arabic => format!("المخاطر الحرجة: {}", count),
            Language::French => format!("Risques critiques : {}", count),
            Language::Bengali => format!("গুরুতর ঝুঁকি: {}", count),
            Language::Portuguese => format!("Riscos críticos: {}", count),
            Language::Russian => format!("Критические риски: {}", count),
            Language::Urdu => format!("اہم خطرات: {}", count),
        }
    }

    pub fn audit_affected_devices(count: usize, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Dispositivos Afectados ({}):", count),
            Language::English => format!("Affected Devices ({}):", count),
            Language::Chinese => format!("受影响设备 ({}):", count),
            Language::Hindi => format!("प्रभावित उपकरण ({}):", count),
            Language::Arabic => format!("الأجهزة المتأثرة ({}):", count),
            Language::French => format!("Appareils affectés ({}) :", count),
            Language::Bengali => format!("ক্ষতিগ্রস্ত ডিভাইস ({}):", count),
            Language::Portuguese => format!("Dispositivos Afetados ({}):", count),
            Language::Russian => format!("Затронутые устройства ({}):", count),
            Language::Urdu => format!("متاثرہ آلات ({}):", count),
        }
    }

    pub fn audit_rerun_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🔄 Reejecutar Auditoría",
            Language::English => "🔄 Re-run Audit",
            Language::Chinese => "🔄 重新运行审计",
            Language::Hindi => "🔄 ऑडिट पुनः चलाएं",
            Language::Arabic => "🔄 إعادة تشغيل التدقيق",
            Language::French => "🔄 Réexécuter l'audit",
            Language::Bengali => "🔄 পুনরায় অডিট চালান",
            Language::Portuguese => "🔄 Executar Novamente",
            Language::Russian => "🔄 Повторить аудит",
            Language::Urdu => "🔄 دوبارہ آڈٹ چلائیں",
        }
    }

    pub fn audit_check_title(id: SecurityCheckId, lang: Language) -> &'static str {
        match id {
            SecurityCheckId::TelnetDetected => match lang {
                Language::Spanish => "Protocolo Telnet Detectado (Puerto 23)",
                Language::English => "Telnet Protocol Detected (Port 23)",
                Language::Chinese => "检测到 Telnet 协议 (端口 23)",
                Language::Hindi => "टेलनेट प्रोटोकॉल का पता चला (पोर्ट 23)",
                Language::Arabic => "تم رصد بروتوكول تلنت (المنفذ 23)",
                Language::French => "Protocole Telnet détecté (Port 23)",
                Language::Bengali => "টেলনেট প্রোটোকল সনাক্ত হয়েছে (পোর্ট ২৩)",
                Language::Portuguese => "Protocolo Telnet Detectado (Porta 23)",
                Language::Russian => "Обнаружен протокол Telnet (порт 23)",
                Language::Urdu => "ٹیل نیٹ پروٹوکول کا پتہ چلا (پورٹ 23)",
            },
            SecurityCheckId::FtpDetected => match lang {
                Language::Spanish => "Servicio FTP Inseguro (Puerto 21)",
                Language::English => "Insecure FTP Service (Port 21)",
                Language::Chinese => "不安全的 FTP 服务 (端口 21)",
                Language::Hindi => "असुरक्षित एफ़टीपी सेवा (पोर्ट 21)",
                Language::Arabic => "خدمة FTP غير آمنة (المنفذ 21)",
                Language::French => "Service FTP non sécurisé (Port 21)",
                Language::Bengali => "অসুরক্ষিত এফটিপি পরিষেবা (পোর্ট ২১)",
                Language::Portuguese => "Serviço FTP Inseguro (Porta 21)",
                Language::Russian => "Небезопасная служба FTP (порт 21)",
                Language::Urdu => "غیر محفوظ FTP سروس (پورٹ 21)",
            },
            SecurityCheckId::EncryptedServices => match lang {
                Language::Spanish => "Servicios de Red Cifrados",
                Language::English => "Encrypted Network Services",
                Language::Chinese => "加密的网络服务",
                Language::Hindi => "एन्क्रिप्टेड नेटवर्क सेवाएं",
                Language::Arabic => "خدمات شبكة مشفرة",
                Language::French => "Services réseau chiffrés",
                Language::Bengali => "এনক্রিপ্ট করা নেটওয়ার্ক পরিষেবা",
                Language::Portuguese => "Serviços de Rede Criptografados",
                Language::Russian => "Зашифрованные сетевые службы",
                Language::Urdu => "خفیہ کردہ نیٹ ورک سروسز",
            },
            SecurityCheckId::UnwhitelistedDevices => match lang {
                Language::Spanish => "Dispositivos no incluidos en Lista Blanca",
                Language::English => "Devices Not in Whitelist",
                Language::Chinese => "未加入白名单的设备",
                Language::Hindi => "श्वेतसूची में शामिल नहीं किए गए उपकरण",
                Language::Arabic => "أجهزة غير مدرجة في القائمة البيضاء",
                Language::French => "Appareils non inclus dans la liste blanche",
                Language::Bengali => "হোয়াইটলিস্টে অন্তর্ভুক্ত নয় এমন ডিভাইস",
                Language::Portuguese => "Dispositivos fora da Lista Branca",
                Language::Russian => "Устройства не из белого списка",
                Language::Urdu => "وائٹ لسٹ میں شامل نہ ہونے والے آلات",
            },
            SecurityCheckId::AccessControlWhitelisted => match lang {
                Language::Spanish => "Control de Acceso y Lista Blanca",
                Language::English => "Access Control & Whitelist",
                Language::Chinese => "访问控制与白名单",
                Language::Hindi => "पहुंच नियंत्रण और श्वेतसूची",
                Language::Arabic => "التحكم في الوصول والقائمة البيضاء",
                Language::French => "Contrôle d'accès et liste blanche",
                Language::Bengali => "অ্যাক্সেস নিয়ন্ত্রণ ও হোয়াইটলিস্ট",
                Language::Portuguese => "Controle de Acesso e Lista Branca",
                Language::Russian => "Контроль доступа и белый список",
                Language::Urdu => "رسائی کنٹرول اور وائٹ لسٹ",
            },
            SecurityCheckId::RandomizedMacs => match lang {
                Language::Spanish => "MACs Aleatorias sin Identificar",
                Language::English => "Unidentified Randomized MACs",
                Language::Chinese => "未识别的随机 MAC 地址",
                Language::Hindi => "अपरिचित यादृच्छिक मैक पते",
                Language::Arabic => "عناوين MAC عشوائية غير معرّفة",
                Language::French => "Adresses MAC aléatoires non identifiées",
                Language::Bengali => "অচিহ্নিত এলোমেলো ম্যাক ঠিকানা",
                Language::Portuguese => "MACs Aleatórios não Identificados",
                Language::Russian => "Неопознанные случайные MAC-адреса",
                Language::Urdu => "غیر شناخت شدہ بے ترتیب میک ایڈریس",
            },
            SecurityCheckId::DangerousPortsForwarded => match lang {
                Language::Spanish => "Puertos Críticos Expuestos a Internet",
                Language::English => "Critical Ports Exposed to Internet",
                Language::Chinese => "暴露于互联网的关键端口",
                Language::Hindi => "इंटरनेट पर खुले संवेदनशील पोर्ट",
                Language::Arabic => "منافذ حساسة مكشوفة للإنترنت",
                Language::French => "Ports critiques exposés à Internet",
                Language::Bengali => "ইন্টারনেটে উন্মুক্ত গুরুত্বপূর্ণ পোর্ট",
                Language::Portuguese => "Portas Críticas Expostas à Internet",
                Language::Russian => "Критические порты, открытые в интернет",
                Language::Urdu => "انٹرنیٹ کے سامنے کھلی اہم پورٹس",
            },
            SecurityCheckId::ControlledPortMapping => match lang {
                Language::Spanish => "Mapeo de Puertos Controlado",
                Language::English => "Controlled Port Mapping",
                Language::Chinese => "可控的端口映射",
                Language::Hindi => "नियंत्रित पोर्ट मैपिंग",
                Language::Arabic => "تعيين منافذ خاضع للتحكم",
                Language::French => "Redirection de ports maîtrisée",
                Language::Bengali => "নিয়ন্ত্রিত পোর্ট ম্যাপিং",
                Language::Portuguese => "Mapeamento de Portas Controlado",
                Language::Russian => "Контролируемое перенаправление портов",
                Language::Urdu => "کنٹرول شدہ پورٹ میپنگ",
            },
            SecurityCheckId::RouterPasswordMissing => match lang {
                Language::Spanish => "Contraseña de Router no Configurada en NetLens",
                Language::English => "Router Password Not Configured in NetLens",
                Language::Chinese => "NetLens 中未配置路由器密码",
                Language::Hindi => "NetLens में राउटर पासवर्ड कॉन्फ़िगर नहीं है",
                Language::Arabic => "كلمة مرور الراوتر غير مهيأة في NetLens",
                Language::French => "Mot de passe du routeur non configuré dans NetLens",
                Language::Bengali => "NetLens-এ রাউটার পাসওয়ার্ড কনফিগার করা নেই",
                Language::Portuguese => "Senha do Roteador não Configurada no NetLens",
                Language::Russian => "Пароль роутера не настроен в NetLens",
                Language::Urdu => "NetLens میں روٹر پاس ورڈ ترتیب نہیں دیا گیا",
            },
            SecurityCheckId::RouterIntegrationActive => match lang {
                Language::Spanish => "Integración con Router Activa",
                Language::English => "Active Router Integration",
                Language::Chinese => "路由器集成处于活跃状态",
                Language::Hindi => "सक्रिय राउटर एकीकरण",
                Language::Arabic => "التكامل مع الراوتر نشط",
                Language::French => "Intégration du routeur active",
                Language::Bengali => "সক্রিয় রাউটার ইন্টিগ্রেশন",
                Language::Portuguese => "Integração com Roteador Ativa",
                Language::Russian => "Активная интеграция с роутером",
                Language::Urdu => "روٹر انضمام فعال ہے",
            },
        }
    }

    pub fn audit_check_description(id: SecurityCheckId, count: usize, lang: Language) -> String {
        match id {
            SecurityCheckId::TelnetDetected => match lang {
                Language::Spanish => "Se detectaron dispositivos con Telnet abierto. Telnet transmite credenciales y comandos en texto plano sin cifrar.".to_string(),
                Language::English => "Devices with open Telnet were detected. Telnet transmits credentials and commands in unencrypted plain text.".to_string(),
                Language::Chinese => "检测到开放 Telnet 的设备。Telnet 以未加密的明文传输凭据和命令。".to_string(),
                Language::Hindi => "खुले टेलनेट वाले उपकरण पाए गए। टेलनेट बिना एन्क्रिप्शन के सादे पाठ में क्रेडेंशियल और कमांड भेजता है।".to_string(),
                Language::Arabic => "تم اكتشاف أجهزة بها Telnet مفتوح. ينقل Telnet بيانات الاعتماد والأوامر بنص صريح غير مشفر.".to_string(),
                Language::French => "Des appareils avec Telnet ouvert ont été détectés. Telnet transmet les identifiants et les commandes en texte clair non chiffré.".to_string(),
                Language::Bengali => "খোলা টেলনেট সহ ডিভাইস সনাক্ত করা হয়েছে। টেলনেট অ-এনক্রিপ্ট করা প্লেইন টেক্সটে প্রমাণপত্র এবং কমান্ড প্রেরণ করে।".to_string(),
                Language::Portuguese => "Dispositivos com Telnet aberto foram detectados. Telnet transmite credenciais e comandos em texto não criptografado.".to_string(),
                Language::Russian => "Обнаружены устройства с открытым Telnet. Telnet передает учетные данные и команды в незашифрованном виде.".to_string(),
                Language::Urdu => "کھلی ٹیل نیٹ والے آلات کا پتہ چلا۔ ٹیل نیٹ اسناد اور کمانڈز بغیر خفیہ کاری کے سادہ متن میں منتقل کرتا ہے۔".to_string(),
            },
            SecurityCheckId::FtpDetected => match lang {
                Language::Spanish => "FTP tradicional no cifra contraseñas ni archivos durante la transferencia local.".to_string(),
                Language::English => "Traditional FTP does not encrypt passwords or files during local transfer.".to_string(),
                Language::Chinese => "传统的 FTP 在传输过程中不会对密码或文件进行加密。".to_string(),
                Language::Hindi => "पारंपरिक एफ़टीपी स्थानीय स्थानांतरण के दौरान पासवर्ड या फ़ाइलों को एन्क्रिप्ट नहीं करता है।".to_string(),
                Language::Arabic => "لا يقوم FTP التقليدي بتشفير كلمات المرور أو الملفات أثناء النقل المحلي.".to_string(),
                Language::French => "Le protocole FTP classique ne chiffre ni les mots de passe ni les fichiers lors des transferts locaux.".to_string(),
                Language::Bengali => "ঐতিহ্যবাহী এফটিপি স্থানীয় স্থানান্তরের সময় পাসওয়ার্ড বা ফাইল এনক্রিপ্ট করে না।".to_string(),
                Language::Portuguese => "O FTP tradicional não criptografa senhas ou arquivos durante a transferência local.".to_string(),
                Language::Russian => "Традиционный FTP не шифрует пароли и файлы при локальной передаче.".to_string(),
                Language::Urdu => "روایتی FTP مقامی منتقلی کے دوران پاس ورڈز یا فائلوں کو خفیہ نہیں کرتا ہے۔".to_string(),
            },
            SecurityCheckId::EncryptedServices => match lang {
                Language::Spanish => "No se detectaron puertos vulnerables obsoletos (Telnet o FTP plano) en los dispositivos activos.".to_string(),
                Language::English => "No obsolete vulnerable ports (plain Telnet or FTP) were detected on active devices.".to_string(),
                Language::Chinese => "在活跃设备上未检测到过时且不安全的端口 (明文 Telnet 或 FTP)。".to_string(),
                Language::Hindi => "सक्रिय उपकरणों पर कोई पुराना असुरक्षित पोर्ट (सादा टेलनेट या एफ़टीपी) नहीं पाया गया।".to_string(),
                Language::Arabic => "لم يتم رصد منافذ قديمة غير آمنة (مثل Telnet أو FTP الصريح) على الأجهزة النشطة.".to_string(),
                Language::French => "Aucun port obsolète vulnérable (Telnet ou FTP en clair) n'a été détecté sur les appareils actifs.".to_string(),
                Language::Bengali => "সক্রিয় ডিভাইসগুলোতে কোনো পুরানো অসুরক্ষিত পোর্ট সনাক্ত করা যায়নি।".to_string(),
                Language::Portuguese => "Nenhuma porta vulnerável obsoleta (Telnet ou FTP sem criptografia) foi detectada nos dispositivos ativos.".to_string(),
                Language::Russian => "На активных устройствах не обнаружено устаревших уязвимых портов (незащищенный Telnet или FTP).".to_string(),
                Language::Urdu => "فعال آلات پر کوئی پرانی کمزور پورٹس نہیں پائی گئیں۔".to_string(),
            },
            SecurityCheckId::UnwhitelistedDevices => match lang {
                Language::Spanish => format!("Existen {} dispositivos conectados que no han sido validados como conocidos.", count),
                Language::English => format!("There are {} connected devices that have not been validated as known.", count),
                Language::Chinese => format!("有 {} 台连接的设备尚未确认为已知设备。", count),
                Language::Hindi => format!("ऐसे {} कनेक्टेड उपकरण हैं जिन्हें ज्ञात के रूप में मान्य नहीं किया गया है।", count),
                Language::Arabic => format!("يوجد {} أجهزة متصلة لم يتم التحقق منها كأجهزة موثوقة.", count),
                Language::French => format!("Il y a {} appareils connectés qui n'ont pas été validés comme connus.", count),
                Language::Bengali => format!("{}টি সংযুক্ত ডিভাইস রয়েছে যা পরিচিত হিসেবে যাচাই করা হয়নি।", count),
                Language::Portuguese => format!("Existem {} dispositivos conectados que não foram validados como conhecidos.", count),
                Language::Russian => format!("Обнаружено {} подключенных устройств, не подтвержденных как доверенные.", count),
                Language::Urdu => format!("{} منسلک آلات موجود ہیں جنہیں معلوم کے طور پر تصدیق نہیں کیا گیا ہے۔", count),
            },
            SecurityCheckId::AccessControlWhitelisted => match lang {
                Language::Spanish => "Todos los dispositivos conectados están identificados o no se han marcado intrusos.".to_string(),
                Language::English => "All connected devices are identified or no intruders were flagged.".to_string(),
                Language::Chinese => "所有连接的设备均已识别，未发现入侵者。".to_string(),
                Language::Hindi => "सभी कनेक्टेड उपकरणों की पहचान की गई है या कोई घुसपैठिया चिह्नित नहीं किया गया है।".to_string(),
                Language::Arabic => "تم تحديد جميع الأجهزة المتصلة أو لم يتم رصد متسللين.".to_string(),
                Language::French => "Tous les appareils connectés sont identifiés ou aucun intrus n'a été signalé.".to_string(),
                Language::Bengali => "সমস্ত সংযুক্ত ডিভাইস চিহ্নিত বা কোনো অনুপ্রবেশকারী চিহ্নিত করা হয়নি।".to_string(),
                Language::Portuguese => "Todos os dispositivos conectados estão identificados ou nenhum intruso foi detectado.".to_string(),
                Language::Russian => "Все подключенные устройства опознаны, нарушителей не обнаружено.".to_string(),
                Language::Urdu => "تمام منسلک آلات کی شناخت ہو چکی ہے یا کوئی گھس بیٹھیا نہیں پایا گیا۔".to_string(),
            },
            SecurityCheckId::RandomizedMacs => match lang {
                Language::Spanish => "Dispositivos modernos (iOS/Android) usan MAC privada aleatoria, dificultando saber a quién pertenece el equipo.".to_string(),
                Language::English => "Modern devices (iOS/Android) use randomized private MACs, making ownership identification harder.".to_string(),
                Language::Chinese => "现代设备 (iOS/Android) 使用随机私有 MAC，增加了识别设备所有者的难度。".to_string(),
                Language::Hindi => "आधुनिक उपकरण (iOS/Android) यादृच्छिक निजी मैक का उपयोग करते हैं, जिससे स्वामित्व की पहचान मुश्किल हो जाती है।".to_string(),
                Language::Arabic => "تستخدم الأجهزة الحديثة (iOS/Android) عناوين MAC عشوائية خاصة، مما يصعّب معرفة مالك الجهاز.".to_string(),
                Language::French => "Les appareils modernes (iOS/Android) utilisent des adresses MAC aléatoires, rendant leur identification plus difficile.".to_string(),
                Language::Bengali => "আধুনিক ডিভাইসগুলো (iOS/Android) এলোমেলো ব্যক্তিগত ম্যাক ব্যবহার করে, যা মালিকানা সনাক্ত করা কঠিন করে তোলে।".to_string(),
                Language::Portuguese => "Dispositivos modernos (iOS/Android) usam MAC privado aleatório, dificultando saber quem é o dono do aparelho.".to_string(),
                Language::Russian => "Современные устройства (iOS/Android) используют случайные MAC-адреса, затрудняя идентификацию владельца.".to_string(),
                Language::Urdu => "جدید آلات (iOS/Android) بے ترتیب نجی میک استعمال کرتے ہیں، جس سے مالک کی شناخت مشکل ہو جاتی ہے۔".to_string(),
            },
            SecurityCheckId::DangerousPortsForwarded => match lang {
                Language::Spanish => "Se detectaron reglas NAT en el router que exponen servicios sensibles a ataques directos desde Internet.".to_string(),
                Language::English => "NAT rules detected on the router exposing sensitive services to direct Internet attacks.".to_string(),
                Language::Chinese => "在路由器上检测到 NAT 转发规则，将敏感服务直接暴露在互联网攻击之下。".to_string(),
                Language::Hindi => "राउटर पर एनएटी नियम पाए गए जो इंटरनेट से सीधे हमलों के लिए संवेदनशील सेवाओं को उजागर करते हैं।".to_string(),
                Language::Arabic => "تم رصد قواعد NAT على الراوتر تعرض خدمات حساسة لهجمات مباشرة من الإنترنت.".to_string(),
                Language::French => "Des règles NAT exposant des services sensibles à des attaques directes depuis Internet ont été détectées.".to_string(),
                Language::Bengali => "রাউটারে NAT নিয়ম সনাক্ত করা হয়েছে যা সংবেদনশীল পরিষেবাগুলোকে ইন্টারনেটের সরাসরি আক্রমণের মুখে ফেলে।".to_string(),
                Language::Portuguese => "Foram detectadas regras NAT no roteador expondo serviços confidenciais a ataques diretos da Internet.".to_string(),
                Language::Russian => "Обнаружены правила NAT на роутере, открывающие доступ к уязвимым службам из интернета.".to_string(),
                Language::Urdu => "روٹر پر NAT قواعد پائے گئے جو حساس سروسز کو براہ راست انٹرنیٹ حملوں کے خطرے میں ڈالتے ہیں۔".to_string(),
            },
            SecurityCheckId::ControlledPortMapping => match lang {
                Language::Spanish => format!("Hay {} puertos abiertos en el router, ninguno considerado de alto riesgo crítico.", count),
                Language::English => format!("There are {} open ports on the router, none considered high-risk critical.", count),
                Language::Chinese => format!("路由器上有 {} 个开放端口，均不属于高危关键风险端口。", count),
                Language::Hindi => format!("राउटर पर {} खुले पोर्ट हैं, जिनमें से कोई भी उच्च जोखिम वाला नहीं माना जाता है।", count),
                Language::Arabic => format!("هناك {} منافذ مفتوحة على الراوتر، ولا يُعتبر أي منها عالي الخطورة.", count),
                Language::French => format!("Il y a {} ports ouverts sur le routeur, aucun n'étant considéré comme à risque critique.", count),
                Language::Bengali => format!("রাউটারে {}টি পোর্ট খোলা আছে, যার কোনটিই উচ্চ ঝুঁকিপূর্ণ নয়।", count),
                Language::Portuguese => format!("Há {} portas abertas no roteador, nenhuma considerada de alto risco crítico.", count),
                Language::Russian => format!("На роутере открыто {} портов, критически опасных среди них нет.", count),
                Language::Urdu => format!("روٹر پر {} کھلی ہوئی پورٹس ہیں، جن میں سے کوئی بھی زیادہ خطرناک نہیں ہے۔", count),
            },
            SecurityCheckId::RouterPasswordMissing => match lang {
                Language::Spanish => "NetLens no tiene guardada la contraseña administrativa para aplicar bloqueo 24/7 en el router.".to_string(),
                Language::English => "NetLens does not have the admin password stored to apply 24/7 blocking on the router.".to_string(),
                Language::Chinese => "NetLens 未保存管理密码，无法在路由器上应用全天候阻断。".to_string(),
                Language::Hindi => "राउटर पर 24/7 ब्लॉकिंग लागू करने के लिए NetLens में एडमिन पासवर्ड सहेजा नहीं गया है।".to_string(),
                Language::Arabic => "لم يتم حفظ كلمة مرور المسؤول في NetLens لتطبيق الحظر على الراوتر على مدار الساعة.".to_string(),
                Language::French => "NetLens n'a pas enregistré le mot de passe administrateur pour bloquer les appareils sur le routeur 24/7.".to_string(),
                Language::Bengali => "রাউটারে ২৪/৭ ব্লকিং প্রয়োগ করার জন্য NetLens-এ অ্যাডমিন পাসওয়ার্ড সংরক্ষিত নেই।".to_string(),
                Language::Portuguese => "O NetLens não possui a senha de administrador salva para aplicar bloqueio 24/7 no roteador.".to_string(),
                Language::Russian => "В NetLens не сохранен пароль администратора для круглосуточной блокировки на роутере.".to_string(),
                Language::Urdu => "روٹر پر چوبیس گھنٹے بلاکنگ لاگو کرنے کے لیے NetLens میں ایڈمن پاس ورڈ محفوظ نہیں ہے۔".to_string(),
            },
            SecurityCheckId::RouterIntegrationActive => match lang {
                Language::Spanish => "La contraseña de administración del router está configurada, permitiendo aplicar listas de bloqueo directamente por hardware.".to_string(),
                Language::English => "Router admin password is configured, allowing hardware-level blocklist enforcement.".to_string(),
                Language::Chinese => "路由器管理员密码已配置，允许直接在硬件层面应用黑名单。".to_string(),
                Language::Hindi => "राउटर एडमिन पासवर्ड कॉन्फ़िगर है, जिससे हार्डवेयर स्तर पर ब्लॉकलिस्ट लागू की जा सकती है।".to_string(),
                Language::Arabic => "تمت تهيئة كلمة مرور مسؤول الراوتر، مما يتيح تطبيق قوائم الحظر عبر العتاد مباشرة.".to_string(),
                Language::French => "Le mot de passe administrateur du routeur est configuré, permettant le blocage matériel des appareils.".to_string(),
                Language::Bengali => "রাউটার অ্যাডমিন পাসওয়ার্ড কনফিগার করা আছে, যা হার্ডওয়্যার স্তরে ব্লক প্রয়োগের অনুমতি দেয়।".to_string(),
                Language::Portuguese => "A senha de administrador do roteador está configurada, permitindo aplicar listas de bloqueio no hardware.".to_string(),
                Language::Russian => "Пароль администратора роутера настроен, что позволяет блокировать устройства на уровне железа.".to_string(),
                Language::Urdu => "روٹر ایڈمن پاس ورڈ کنفیگر ہے، جو ہارڈ ویئر کی سطح پر بلیک لسٹ نافذ کرنے کی اجازت دیتا ہے۔".to_string(),
            },
        }
    }

    pub fn audit_check_recommendation(id: SecurityCheckId, lang: Language) -> &'static str {
        match id {
            SecurityCheckId::TelnetDetected => match lang {
                Language::Spanish => "Deshabilita Telnet y utiliza SSH (puerto 22) para administración segura cifrada.",
                Language::English => "Disable Telnet and use SSH (port 22) for secure encrypted management.",
                Language::Chinese => "禁用 Telnet 并使用 SSH (端口 22) 进行安全加密管理。",
                Language::Hindi => "टेलनेट को अक्षम करें और सुरक्षित एन्क्रिप्टेड प्रबंधन के लिए एसएसएच (पोर्ट 22) का उपयोग करें।",
                Language::Arabic => "عطّل Telnet واستخدم SSH (المنفذ 22) للإدارة المشفرة الآمنة.",
                Language::French => "Désactivez Telnet et utilisez SSH (port 22) pour une gestion chiffrée sécurisée.",
                Language::Bengali => "টেলনেট নিষ্ক্রিয় করুন এবং সুরক্ষিত এনক্রিপ্ট করা পরিচালনার জন্য SSH (পোর্ট ২২) ব্যবহার করুন।",
                Language::Portuguese => "Desative o Telnet e utilize SSH (porta 22) para gerenciamento criptografado seguro.",
                Language::Russian => "Отключите Telnet и используйте SSH (порт 22) для безопасного зашифрованного управления.",
                Language::Urdu => "ٹیل نیٹ کو غیر فعال کریں اور محفوظ انکرپٹڈ انتظام کے لیے SSH (پورٹ 22) استعمال کریں۔",
            },
            SecurityCheckId::FtpDetected => match lang {
                Language::Spanish => "Migra hacia SFTP (sobre SSH) o FTPS con certificados TLS modernos.",
                Language::English => "Migrate to SFTP (over SSH) or FTPS with modern TLS certificates.",
                Language::Chinese => "迁移到 SFTP (基于 SSH) 或具有现代 TLS 证书的 FTPS。",
                Language::Hindi => "एसएफटीपी (एसएसएच पर) या आधुनिक टीएलएस प्रमाणपत्रों के साथ एफटीपीएस पर माइग्रेट करें।",
                Language::Arabic => "انتقل إلى SFTP (عبر SSH) أو FTPS بشهادات TLS حديثة.",
                Language::French => "Migrez vers SFTP (via SSH) ou FTPS avec des certificats TLS modernes.",
                Language::Bengali => "SFTP (SSH-এর মাধ্যমে) বা আধুনিক TLS সার্টিফিকেট সহ FTPS-এ স্থানান্তরিত হন।",
                Language::Portuguese => "Migre para SFTP (sobre SSH) ou FTPS com certificados TLS modernos.",
                Language::Russian => "Перейдите на SFTP (через SSH) или FTPS с современными сертификатами TLS.",
                Language::Urdu => "SFTP (بذریعہ SSH) یا جدید TLS سرٹیفکیٹس کے ساتھ FTPS پر منتقل ہوں۔",
            },
            SecurityCheckId::EncryptedServices => match lang {
                Language::Spanish => "Continúa utilizando servicios cifrados (SSH, HTTPS, SFTP).",
                Language::English => "Continue using encrypted services (SSH, HTTPS, SFTP).",
                Language::Chinese => "继续使用加密服务 (SSH, HTTPS, SFTP)。",
                Language::Hindi => "एन्क्रिप्टेड सेवाओं (SSH, HTTPS, SFTP) का उपयोग जारी रखें।",
                Language::Arabic => "استمر في استخدام الخدمات المشفرة (SSH, HTTPS, SFTP).",
                Language::French => "Continuez à utiliser des services chiffrés (SSH, HTTPS, SFTP).",
                Language::Bengali => "এনক্রিপ্ট করা পরিষেবাগুলো (SSH, HTTPS, SFTP) ব্যবহার করা চালিয়ে যান।",
                Language::Portuguese => "Continue utilizando serviços criptografados (SSH, HTTPS, SFTP).",
                Language::Russian => "Продолжайте использовать зашифрованные службы (SSH, HTTPS, SFTP).",
                Language::Urdu => "خفیہ کردہ سروسز (SSH، HTTPS، SFTP) کا استعمال جاری رکھیں۔",
            },
            SecurityCheckId::UnwhitelistedDevices => match lang {
                Language::Spanish => "Revisa la lista de dispositivos y añade los legítimos a la Lista Blanca o bloquéalos en el router si son desconocidos.",
                Language::English => "Review the device list and add legitimate ones to Whitelist or block them on the router if unknown.",
                Language::Chinese => "检查设备列表并将合法设备加入白名单，如为未知设备可在路由器中将其阻止。",
                Language::Hindi => "उपकरण सूची की समीक्षा करें और वैध उपकरणों को श्वेतसूची में जोड़ें या अज्ञात होने पर उन्हें ब्लॉक करें।",
                Language::Arabic => "راجع قائمة الأجهزة وأضف الأجهزة الموثوقة إلى القائمة البيضاء أو احظرها إذا كانت غير معروفة.",
                Language::French => "Passez en revue la liste et ajoutez les appareils légitimes à la liste blanche ou bloquez-les sur le routeur.",
                Language::Bengali => "ডিভাইস তালিকা পর্যালোচনা করুন এবং বৈধ ডিভাইসগুলোকে হোয়াইটলিস্টে যোগ করুন বা অজানা হলে ব্লক করুন।",
                Language::Portuguese => "Revise a lista de dispositivos e adicione os legítimos à Lista Branca ou bloqueie-os no roteador.",
                Language::Russian => "Проверьте список устройств и добавьте известные в белый список или заблокируйте незнакомые.",
                Language::Urdu => "ڈیوائس کی فہرست کا جائزہ لیں اور جائز آلات کو وائٹ لسٹ میں شامل کریں یا نامعلوم ہونے پر روٹر پر بلاک کریں۔",
            },
            SecurityCheckId::AccessControlWhitelisted => match lang {
                Language::Spanish => "Mantén actualizado el inventario de dispositivos del hogar.",
                Language::English => "Keep your home device inventory up to date.",
                Language::Chinese => "保持家庭设备清单处于最新状态。",
                Language::Hindi => "अपने घरेलू उपकरणों की सूची हमेशा अद्यतित रखें।",
                Language::Arabic => "حافظ على تحديث قائمة أجهزة منزلك بانتظام.",
                Language::French => "Maintenez à jour l'inventaire des appareils de votre réseau.",
                Language::Bengali => "আপনার বাড়ির ডিভাইসের তালিকা আপডেট রাখুন।",
                Language::Portuguese => "Mantenha atualizado o inventário de dispositivos da rede.",
                Language::Russian => "Поддерживайте список домашних устройств в актуальном состоянии.",
                Language::Urdu => "اپنے گھریلو آلات کی فہرست کو ہمیشہ اپ ڈیٹ رکھیں۔",
            },
            SecurityCheckId::RandomizedMacs => match lang {
                Language::Spanish => "Asigna un alias personalizado a cada dispositivo para reconocerlo fácilmente en futuras conexiones.",
                Language::English => "Assign a custom alias to each device to easily recognize it on future connections.",
                Language::Chinese => "为每个设备分配自定义别名，以便在将来的连接中轻松识别。",
                Language::Hindi => "भविष्य के कनेक्शनों में आसानी से पहचानने के लिए प्रत्येक उपकरण को एक कस्टम उपनाम दें।",
                Language::Arabic => "عيّن اسماً مخصصاً لكل جهاز لتسهيل التعرف عليه في الاتصالات المستقبلية.",
                Language::French => "Attribuez un alias personnalisé à chaque appareil pour le reconnaître facilement.",
                Language::Bengali => "ভবিষ্যতের সংযোগে সহজে চিনতে প্রতিটি ডিভাইসে একটি কাস্টম নাম বরাদ্দ করুন।",
                Language::Portuguese => "Atribua um apelido personalizado a cada dispositivo para reconhecê-lo facilmente.",
                Language::Russian => "Назначьте понятные псевдонимы устройствам для их удобного распознавания.",
                Language::Urdu => "مستقبل کے کنکشنز میں آسانی سے پہچاننے کے لیے ہر ڈیوائس کو ایک عرفی نام تفویض کریں۔",
            },
            SecurityCheckId::DangerousPortsForwarded => match lang {
                Language::Spanish => "Elimina las reglas de reenvío de puertos para SSH, RDP, SMB o Telnet hacia Internet. Utiliza una VPN doméstica (WireGuard/OpenVPN) para acceder de forma remota.",
                Language::English => "Remove port forwarding rules for SSH, RDP, SMB or Telnet to the Internet. Use a home VPN (WireGuard/OpenVPN) for remote access.",
                Language::Chinese => "移除针对互联网的 SSH、RDP、SMB 或 Telnet 端口转发规则。使用家用 VPN (WireGuard/OpenVPN) 进行远程访问。",
                Language::Hindi => "इंटरनेट पर एसएसएच, आरडीपी, एसएमबी या टेलनेट के लिए पोर्ट फ़ॉरवर्डिंग नियम हटाएं। रिमोट एक्सेस के लिए घरेलू वीपीएन का उपयोग करें।",
                Language::Arabic => "احذف قواعد إعادة توجيه المنافذ لـ SSH أو RDP أو SMB أو Telnet للإنترنت. استخدم VPN منزلي للوصول عن بُعد.",
                Language::French => "Supprimez les redirections de ports pour SSH, RDP, SMB ou Telnet vers Internet. Utilisez un VPN domestique pour l'accès distant.",
                Language::Bengali => "ইন্টারনেটের জন্য SSH, RDP, SMB বা Telnet পোর্ট ফরওয়ার্ডিং নিয়ম সরান। দূরবর্তী অ্যাক্সেসের জন্য একটি হোম ভিপিএন ব্যবহার করুন।",
                Language::Portuguese => "Remova regras de encaminhamento de portas SSH, RDP, SMB ou Telnet para a Internet. Use uma VPN doméstica para acesso remoto.",
                Language::Russian => "Удалите правила перенаправления портов SSH, RDP, SMB или Telnet наружу. Используйте домашний VPN для удаленного доступа.",
                Language::Urdu => "انٹرنیٹ پر SSH، RDP، SMB یا Telnet کے لیے پورٹ فارورڈنگ کے قواعد کو ہٹا دیں۔ ریموٹ رسائی کے لیے گھریلو VPN استعمال کریں۔",
            },
            SecurityCheckId::ControlledPortMapping => match lang {
                Language::Spanish => "Revisa periódicamente las aplicaciones que abren puertos mediante UPnP.",
                Language::English => "Periodically review applications that open ports via UPnP.",
                Language::Chinese => "定期检查通过 UPnP 打开端口的应用程序。",
                Language::Hindi => "UPnP के माध्यम से पोर्ट खोलने वाले अनुप्रयोगों की समय-समय पर समीक्षा करें।",
                Language::Arabic => "راجع بانتظام التطبيقات التي تفتح منافذ عبر UPnP.",
                Language::French => "Vérifiez régulièrement les applications qui ouvrent des ports via UPnP.",
                Language::Bengali => "UPnP এর মাধ্যমে পোর্ট খোলা অ্যাপ্লিকেশনগুলো নিয়মিত পর্যালোচনা করুন।",
                Language::Portuguese => "Revise periodicamente os aplicativos que abrem portas via UPnP.",
                Language::Russian => "Периодически проверяйте приложения, открывающие порты через UPnP.",
                Language::Urdu => "وقفے وقفے سے ان ایپلیکیشنز کا جائزہ لیں جو UPnP کے ذریعے پورٹس کھولتی ہیں۔",
            },
            SecurityCheckId::RouterPasswordMissing => match lang {
                Language::Spanish => "Ingresa la contraseña en Ajustes para habilitar el control por hardware y aislamiento de intrusos.",
                Language::English => "Enter the password in Settings to enable hardware control and intruder isolation.",
                Language::Chinese => "在“设置”中输入密码以启用硬件级控制和入侵者隔离。",
                Language::Hindi => "हार्डवेयर नियंत्रण और घुसपैठियों के अलगाव को सक्षम करने के लिए सेटिंग्स में पासवर्ड दर्ज करें।",
                Language::Arabic => "أدخل كلمة المرور في الإعدادات لتمكين التحكم بالعتاد وعزل المتطفلين.",
                Language::French => "Entrez le mot de passe dans Paramètres pour activer le contrôle matériel et l'isolation des intrus.",
                Language::Bengali => "হার্ডওয়্যার নিয়ন্ত্রণ এবং অনুপ্রবেশকারী বিচ্ছিন্নকরণ সক্ষম করতে সেটিংসে পাসওয়ার্ড লিখুন।",
                Language::Portuguese => "Insira a senha nas Configurações para habilitar o controle por hardware e isolamento de intrusos.",
                Language::Russian => "Введите пароль в Настройках, чтобы включить аппаратный контроль и изоляцию нарушителей.",
                Language::Urdu => "ہارڈ ویئر کنٹرول اور گھس بیٹھنے والوں کی تنہائی کو فعال کرنے کے لیے ترتیبات میں پاس ورڈ درج کریں۔",
            },
            SecurityCheckId::RouterIntegrationActive => match lang {
                Language::Spanish => "Excelente, la red está protegida activamente.",
                Language::English => "Excellent, the network is actively protected.",
                Language::Chinese => "太棒了，网络受到主动保护。",
                Language::Hindi => "शानदार, नेटवर्क सक्रिय रूप से सुरक्षित है।",
                Language::Arabic => "رائع، الشبكة محمية بشكل فعّال.",
                Language::French => "Parfait, le réseau est activement protégé.",
                Language::Bengali => "চমৎকার, নেটওয়ার্ক সক্রিয়ভাবে সুরক্ষিত।",
                Language::Portuguese => "Excelente, a rede está ativamente protegida.",
                Language::Russian => "Отлично, сеть находится под активной защитой.",
                Language::Urdu => "بہترین، نیٹ ورک فعال طور پر محفوظ ہے۔",
            },
        }
    }

    // ==========================================
    // Event History and Timeline
    // ==========================================

    pub fn timeline_modal_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "📋 Historial de Eventos de Red",
            Language::English => "📋 Network Event History",
            Language::Chinese => "📋 网络事件历史",
            Language::Hindi => "📋 नेटवर्क ईवेंट इतिहास",
            Language::Arabic => "📋 سجل أحداث الشبكة",
            Language::French => "📋 Historique des Événements Réseau",
            Language::Bengali => "📋 নেটওয়ার্ক ইভেন্ট ইতিহাস",
            Language::Portuguese => "📋 Histórico de Eventos de Rede",
            Language::Russian => "📋 Журнал сетевых событий",
            Language::Urdu => "📋 نیٹ ورک واقعات کی تاریخ",
        }
    }

    pub fn timeline_modal_desc(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Registro en tiempo real de conexiones, desconexiones, intrusos y cortes de Internet.",
            Language::English => "Real-time log of connections, disconnections, intruders, and Internet outages.",
            Language::Chinese => "连接、断开、入侵者和网络中断的实时日志。",
            Language::Hindi => "कनेक्शन, वियोग, घुसपैठियों और इंटरनेट रुकावटों का वास्तविक समय लॉग।",
            Language::Arabic => "سجل فوري لعمليات الاتصال وانقطاعها والمتسللين وانقطاع الإنترنت.",
            Language::French => "Journal en temps réel des connexions, déconnexions, intrus et pannes d'Internet.",
            Language::Bengali => "সংযোগ, সংযোগ বিচ্ছিন্ন, অনুপ্রবেশকারী এবং ইন্টারনেট বিভ্রাটের রিয়েল-টাইম লগ।",
            Language::Portuguese => "Registro em tempo real de conexões, desconexões, intrusos e quedas de Internet.",
            Language::Russian => "Журнал подключений, отключений, нарушителей и сбоев интернета в реальном времени.",
            Language::Urdu => "کنیکشنز، منقطع ہونے، گھس بیٹھنے والوں اور انٹرنیٹ بند ہونے کا ریئل ٹائم لاگ۔",
        }
    }

    pub fn timeline_online(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🟢 En Línea",
            Language::English => "🟢 Online",
            Language::Chinese => "🟢 在线",
            Language::Hindi => "🟢 ऑनलाइन",
            Language::Arabic => "🟢 متصل",
            Language::French => "🟢 En ligne",
            Language::Bengali => "🟢 অনলাইন",
            Language::Portuguese => "🟢 Online",
            Language::Russian => "🟢 В сети",
            Language::Urdu => "🟢 آن لائن",
        }
    }

    pub fn timeline_outage(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🔴 Corte Activo",
            Language::English => "🔴 Outage Active",
            Language::Chinese => "🔴 网络中断",
            Language::Hindi => "🔴 रुकावट सक्रिय",
            Language::Arabic => "🔴 انقطاع نشط",
            Language::French => "🔴 Panne active",
            Language::Bengali => "🔴 বিভ্রাট সক্রিয়",
            Language::Portuguese => "🔴 Queda Ativa",
            Language::Russian => "🔴 Сбой сети",
            Language::Urdu => "🔴 بندش فعال",
        }
    }

    pub fn timeline_total_events(count: usize, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Total Eventos: {}", count),
            Language::English => format!("Total Events: {}", count),
            Language::Chinese => format!("总事件数: {}", count),
            Language::Hindi => format!("कुल घटनाएं: {}", count),
            Language::Arabic => format!("إجمالي الأحداث: {}", count),
            Language::French => format!("Total des événements : {}", count),
            Language::Bengali => format!("মোট ইভেন্ট: {}", count),
            Language::Portuguese => format!("Total de Eventos: {}", count),
            Language::Russian => format!("Всего событий: {}", count),
            Language::Urdu => format!("کل واقعات: {}", count),
        }
    }

    pub fn timeline_uptime_pct(pct: f64, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Disponibilidad Internet: {:.1}%", pct),
            Language::English => format!("Internet Availability: {:.1}%", pct),
            Language::Chinese => format!("网络可用性: {:.1}%", pct),
            Language::Hindi => format!("इंटरनेट उपलब्धता: {:.1}%", pct),
            Language::Arabic => format!("جاهزية الإنترنت: {:.1}%", pct),
            Language::French => format!("Disponibilité Internet : {:.1}%", pct),
            Language::Bengali => format!("ইন্টারনেট প্রাপ্যতা: {:.1}%", pct),
            Language::Portuguese => format!("Disponibilidade da Internet: {:.1}%", pct),
            Language::Russian => format!("Доступность интернета: {:.1}%", pct),
            Language::Urdu => format!("انٹرنیٹ کی دستیابی: {:.1}%", pct),
        }
    }

    pub fn timeline_clear_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🗑️ Limpiar",
            Language::English => "🗑️ Clear",
            Language::Chinese => "🗑️ 清空",
            Language::Hindi => "🗑️ साफ़ करें",
            Language::Arabic => "🗑️ مسح",
            Language::French => "🗑️ Effacer",
            Language::Bengali => "🗑️ মুছুন",
            Language::Portuguese => "🗑️ Limpar",
            Language::Russian => "🗑️ Очистить",
            Language::Urdu => "🗑️ صاف کریں",
        }
    }

    pub fn timeline_empty(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "No hay eventos registrados recientemente.",
            Language::English => "No recent events recorded.",
            Language::Chinese => "最近无记录的事件。",
            Language::Hindi => "हाल ही में कोई घटना दर्ज नहीं की गई।",
            Language::Arabic => "لا توجد أحداث مسجلة مؤخراً.",
            Language::French => "Aucun événement enregistré récemment.",
            Language::Bengali => "সম্প্রতি কোনো ইভেন্ট রেকর্ড করা হয়নি।",
            Language::Portuguese => "Nenhum evento registrado recentemente.",
            Language::Russian => "Недавних событий не зафиксировано.",
            Language::Urdu => "حال ہی میں کوئی واقعہ ریکارڈ نہیں ہوا۔",
        }
    }

    pub fn event_type_name(event_type: NetworkEventType, lang: Language) -> &'static str {
        match event_type {
            NetworkEventType::DeviceConnected => match lang {
                Language::Spanish => "Conexión",
                Language::English => "Connected",
                Language::Chinese => "已连接",
                Language::Hindi => "कनेक्टेड",
                Language::Arabic => "اتصال",
                Language::French => "Connexion",
                Language::Bengali => "সংযুক্ত",
                Language::Portuguese => "Conexão",
                Language::Russian => "Подключение",
                Language::Urdu => "منسلک",
            },
            NetworkEventType::DeviceDisconnected => match lang {
                Language::Spanish => "Desconexión",
                Language::English => "Disconnected",
                Language::Chinese => "已断开",
                Language::Hindi => "डिस्कनेक्टेड",
                Language::Arabic => "قطع الاتصال",
                Language::French => "Déconnexion",
                Language::Bengali => "বিচ্ছিন্ন",
                Language::Portuguese => "Desconexão",
                Language::Russian => "Отключение",
                Language::Urdu => "منقطع",
            },
            NetworkEventType::NewDeviceDetected => match lang {
                Language::Spanish => "Nuevo Dispositivo",
                Language::English => "New Device",
                Language::Chinese => "新设备",
                Language::Hindi => "नया उपकरण",
                Language::Arabic => "جهاز جديد",
                Language::French => "Nouvel appareil",
                Language::Bengali => "নতুন ডিভাইস",
                Language::Portuguese => "Novo Dispositivo",
                Language::Russian => "Новое устройство",
                Language::Urdu => "نیا آلہ",
            },
            NetworkEventType::SecurityAlert => match lang {
                Language::Spanish => "Alerta de Seguridad",
                Language::English => "Security Alert",
                Language::Chinese => "安全警报",
                Language::Hindi => "सुरक्षा चेतावनी",
                Language::Arabic => "تنبيه أمني",
                Language::French => "Alerte de sécurité",
                Language::Bengali => "সুরক্ষা সতর্কতা",
                Language::Portuguese => "Alerta de Segurança",
                Language::Russian => "Предупреждение безопасности",
                Language::Urdu => "سیکیورٹی الرٹ",
            },
            NetworkEventType::InternetOutage => match lang {
                Language::Spanish => "Corte de Internet",
                Language::English => "Internet Outage",
                Language::Chinese => "网络中断",
                Language::Hindi => "इंटरनेट आउटेज",
                Language::Arabic => "انقطاع الإنترنت",
                Language::French => "Panne Internet",
                Language::Bengali => "ইন্টারনেট বিভ্রাট",
                Language::Portuguese => "Queda de Internet",
                Language::Russian => "Сбой интернета",
                Language::Urdu => "انٹرنیٹ بندش",
            },
            NetworkEventType::InternetRestored => match lang {
                Language::Spanish => "Internet Restablecido",
                Language::English => "Internet Restored",
                Language::Chinese => "网络已恢复",
                Language::Hindi => "इंटरनेट बहाल",
                Language::Arabic => "استعادة الإنترنت",
                Language::French => "Internet rétabli",
                Language::Bengali => "ইন্টারনেট পুনরুদ্ধার",
                Language::Portuguese => "Internet Restaurada",
                Language::Russian => "Интернет восстановлен",
                Language::Urdu => "انٹرنیٹ بحال",
            },
        }
    }

    pub fn event_title(event_type: NetworkEventType, original: &str, lang: Language) -> String {
        if lang == Language::Spanish {
            return original.to_string();
        }
        match event_type {
            NetworkEventType::DeviceConnected => {
                if original.contains("Tiempo Real") || original.contains("Real-Time") {
                    match lang {
                        Language::English => "Device Connected (Real-Time)",
                        Language::Chinese => "设备已连接 (实时)",
                        Language::Hindi => "उपकरण कनेक्ट हुआ (रीयल-टाइम)",
                        Language::Arabic => "تم اتصال الجهاز (الوقت الفعلي)",
                        Language::French => "Appareil connecté (Temps réel)",
                        Language::Bengali => "ডিভাইস সংযুক্ত হয়েছে (রিয়েল-টাইম)",
                        Language::Portuguese => "Dispositivo Conectado (Tempo Real)",
                        Language::Russian => "Устройство подключено (Реальное время)",
                        Language::Urdu => "آلہ منسلک ہو گیا (ریئل ٹائم)",
                        Language::Spanish => unreachable!(),
                    }.to_string()
                } else {
                    match lang {
                        Language::English => "Device Connected",
                        Language::Chinese => "设备已连接",
                        Language::Hindi => "उपकरण कनेक्ट हुआ",
                        Language::Arabic => "تم اتصال الجهاز",
                        Language::French => "Appareil connecté",
                        Language::Bengali => "ডিভাইস সংযুক্ত হয়েছে",
                        Language::Portuguese => "Dispositivo Conectado",
                        Language::Russian => "Устройство подключено",
                        Language::Urdu => "آلہ منسلک ہو گیا",
                        Language::Spanish => unreachable!(),
                    }.to_string()
                }
            }
            NetworkEventType::DeviceDisconnected => {
                if original.contains("Heartbeat") {
                    match lang {
                        Language::English => "Device Disconnected (Heartbeat)",
                        Language::Chinese => "设备已断开 (心跳检测)",
                        Language::Hindi => "उपकरण डिस्कनेक्ट हुआ (हार्टबीट)",
                        Language::Arabic => "تم قطع اتصال الجهاز (نبض القلب)",
                        Language::French => "Appareil déconnecté (Heartbeat)",
                        Language::Bengali => "ডিভাইস সংযোগ বিচ্ছিন্ন হয়েছে (হার্টবিট)",
                        Language::Portuguese => "Dispositivo Desconectado (Heartbeat)",
                        Language::Russian => "Устройство отключено (Heartbeat)",
                        Language::Urdu => "آلہ منقطع ہو گیا (ہارٹ بیٹ)",
                        Language::Spanish => unreachable!(),
                    }.to_string()
                } else if original.contains("Tiempo Real") || original.contains("Real-Time") {
                    match lang {
                        Language::English => "Device Disconnected (Real-Time)",
                        Language::Chinese => "设备已断开 (实时)",
                        Language::Hindi => "उपकरण डिस्कनेक्ट हुआ (रीयल-टाइम)",
                        Language::Arabic => "تم قطع اتصال الجهاز (الوقت الفعلي)",
                        Language::French => "Appareil déconnecté (Temps réel)",
                        Language::Bengali => "ডিভাইস সংযোগ বিচ্ছিন্ন হয়েছে (রিয়েল-টাইম)",
                        Language::Portuguese => "Dispositivo Desconectado (Tempo Real)",
                        Language::Russian => "Устройство отключено (Реальное время)",
                        Language::Urdu => "آلہ منقطع ہو گیا (ریئل ٹائم)",
                        Language::Spanish => unreachable!(),
                    }.to_string()
                } else {
                    match lang {
                        Language::English => "Device Disconnected",
                        Language::Chinese => "设备已断开",
                        Language::Hindi => "उपकरण डिस्कनेक्ट हुआ",
                        Language::Arabic => "تم قطع اتصال الجهاز",
                        Language::French => "Appareil déconnecté",
                        Language::Bengali => "ডিভাইস সংযোগ বিচ্ছিন্ন হয়েছে",
                        Language::Portuguese => "Dispositivo Desconectado",
                        Language::Russian => "Устройство отключено",
                        Language::Urdu => "آلہ منقطع ہو گیا",
                        Language::Spanish => unreachable!(),
                    }.to_string()
                }
            }
            NetworkEventType::NewDeviceDetected => {
                if original.contains("Tiempo Real") || original.contains("Real-Time") {
                    match lang {
                        Language::English => "New Device (Real-Time)",
                        Language::Chinese => "新设备 (实时)",
                        Language::Hindi => "नया उपकरण (रीयल-टाइम)",
                        Language::Arabic => "جهاز جديد (الوقت الفعلي)",
                        Language::French => "Nouvel appareil (Temps réel)",
                        Language::Bengali => "নতুন ডিভাইস (রিয়েল-টাইম)",
                        Language::Portuguese => "Novo Dispositivo (Tempo Real)",
                        Language::Russian => "Новое устройство (Реальное время)",
                        Language::Urdu => "نیا آلہ (ریئل ٹائم)",
                        Language::Spanish => unreachable!(),
                    }.to_string()
                } else {
                    match lang {
                        Language::English => "New Device Detected",
                        Language::Chinese => "检测到新设备",
                        Language::Hindi => "नया उपकरण पहचाना गया",
                        Language::Arabic => "تم اكتشاف جهاز جديد",
                        Language::French => "Nouvel appareil détecté",
                        Language::Bengali => "নতুন ডিভাইস সনাক্ত হয়েছে",
                        Language::Portuguese => "Novo Dispositivo Detectado",
                        Language::Russian => "Обнаружено новое устройство",
                        Language::Urdu => "نیا آلہ پایا گیا",
                        Language::Spanish => unreachable!(),
                    }.to_string()
                }
            }
            NetworkEventType::SecurityAlert => {
                if original.contains("Tiempo Real") || original.contains("Real-Time") {
                    match lang {
                        Language::English => "Unauthorized Reconnection (Real-Time)",
                        Language::Chinese => "未授权重新连接 (实时)",
                        Language::Hindi => "अनधिकृत पुनः कनेक्शन (रीयल-टाइम)",
                        Language::Arabic => "إعادة اتصال غير مصرح بها (الوقت الفعلي)",
                        Language::French => "Reconnexion non autorisée (Temps réel)",
                        Language::Bengali => "অননুমোদিত পুনঃসংযোগ (রিয়েল-টাইম)",
                        Language::Portuguese => "Reconexão Não Autorizada (Tempo Real)",
                        Language::Russian => "Несанкционированное переподключение (Реальное время)",
                        Language::Urdu => "غیر مجاز دوبارہ رابطہ (ریئل ٹائم)",
                        Language::Spanish => unreachable!(),
                    }.to_string()
                } else if original.contains("Reconexión") || original.contains("Reconnection") {
                    match lang {
                        Language::English => "Unauthorized Reconnection",
                        Language::Chinese => "未授权重新连接",
                        Language::Hindi => "अनधिकृत पुनः कनेक्शन",
                        Language::Arabic => "إعادة اتصال غير مصرح بها",
                        Language::French => "Reconnexion non autorisée",
                        Language::Bengali => "অননুমোদিত পুনঃসংযোগ",
                        Language::Portuguese => "Reconexão Não Autorizada",
                        Language::Russian => "Несанкционированное переподключение",
                        Language::Urdu => "غیر مجاز دوبارہ رابطہ",
                        Language::Spanish => unreachable!(),
                    }.to_string()
                } else {
                    match lang {
                        Language::English => "Security Alert",
                        Language::Chinese => "安全警报",
                        Language::Hindi => "सुरक्षा चेतावनी",
                        Language::Arabic => "تنبيه أمني",
                        Language::French => "Alerte de sécurité",
                        Language::Bengali => "সুরক্ষা সতর্কতা",
                        Language::Portuguese => "Alerta de Segurança",
                        Language::Russian => "Предупреждение безопасности",
                        Language::Urdu => "سیکیورٹی الرٹ",
                        Language::Spanish => unreachable!(),
                    }.to_string()
                }
            }
            NetworkEventType::InternetOutage => {
                if original.contains("ISP") || original.contains("Proveedor") {
                    match lang {
                        Language::English => "Alert: ISP / Internet Provider Outage",
                        Language::Chinese => "警告: ISP / 互联网提供商中断",
                        Language::Hindi => "चेतावनी: आईएसपी / इंटरनेट प्रदाता आउटेज",
                        Language::Arabic => "تنبيه: انقطاع مزود خدمة الإنترنت (ISP)",
                        Language::French => "Alerte : Panne FAI / Fournisseur d'accès",
                        Language::Bengali => "সতর্কতা: আইএসপি / ইন্টারনেট সরবরাহকারী বিভ্রাট",
                        Language::Portuguese => "Alerta: Queda do ISP / Provedor de Internet",
                        Language::Russian => "Внимание: Сбой интернет-провайдера (ISP)",
                        Language::Urdu => "انتباہ: آئی ایس پی / انٹرنیٹ فراہم کنندہ کا تعطل",
                        Language::Spanish => unreachable!(),
                    }.to_string()
                } else if original.contains("Local") || original.contains("Wi-Fi") || original.contains("LAN") {
                    match lang {
                        Language::English => "Alert: Local Network Disconnection (Wi-Fi / LAN)",
                        Language::Chinese => "警告: 本地网络断开 (Wi-Fi / LAN)",
                        Language::Hindi => "चेतावनी: स्थानीय नेटवर्क डिस्कनेक्शन (Wi-Fi / LAN)",
                        Language::Arabic => "تنبيه: انقطاع الشبكة المحلية (Wi-Fi / LAN)",
                        Language::French => "Alerte : Déconnexion réseau local (Wi-Fi / LAN)",
                        Language::Bengali => "সতর্কতা: স্থানীয় নেটওয়ার্ক সংযোগ বিচ্ছিন্ন (Wi-Fi / LAN)",
                        Language::Portuguese => "Alerta: Desconexão da Rede Local (Wi-Fi / LAN)",
                        Language::Russian => "Внимание: Отключение локальной сети (Wi-Fi / LAN)",
                        Language::Urdu => "انتباہ: مقامی نیٹ ورک منقطع (Wi-Fi / LAN)",
                        Language::Spanish => unreachable!(),
                    }.to_string()
                } else {
                    match lang {
                        Language::English => "Internet Outage Detected",
                        Language::Chinese => "检测到互联网中断",
                        Language::Hindi => "इंटरनेट आउटेज का पता चला",
                        Language::Arabic => "تم رصد انقطاع للإنترنت",
                        Language::French => "Coupure d'Internet détectée",
                        Language::Bengali => "ইন্টারনেট বিভ্রাট সনাক্ত হয়েছে",
                        Language::Portuguese => "Queda de Conexão à Internet",
                        Language::Russian => "Обнаружен сбой подключения к интернету",
                        Language::Urdu => "انٹرنیٹ بندش پائی گئی",
                        Language::Spanish => unreachable!(),
                    }.to_string()
                }
            }
            NetworkEventType::InternetRestored => match lang {
                Language::English => "Internet Connection Restored",
                Language::Chinese => "互联网连接已恢复",
                Language::Hindi => "इंटरनेट कनेक्शन बहाल हुआ",
                Language::Arabic => "تم استعادة اتصال الإنترنت",
                Language::French => "Connexion Internet rétablie",
                Language::Bengali => "ইন্টারনেট সংযোগ পুনরুদ্ধার হয়েছে",
                Language::Portuguese => "Conexão à Internet Restaurada",
                Language::Russian => "Подключение к интернету восстановлено",
                Language::Urdu => "انٹرنیٹ کنکشن بحال ہو گیا",
                Language::Spanish => unreachable!(),
            }.to_string(),
        }
    }

    pub fn event_details(event_type: NetworkEventType, original: &str, lang: Language) -> String {
        if lang == Language::Spanish {
            return original.to_string();
        }

        match event_type {
            NetworkEventType::InternetOutage => match lang {
                Language::English => "Internet access has been interrupted on the local network.",
                Language::Chinese => "本地网络的互联网访问已中断。",
                Language::Hindi => "स्थानीय नेटवर्क पर इंटरनेट का उपयोग बाधित हो गया है।",
                Language::Arabic => "تم انقطاع الوصول إلى الإنترنت على الشبكة المحلية.",
                Language::French => "L'accès à Internet a été interrompu sur le réseau local.",
                Language::Bengali => "স্থানীয় নেটওয়ার্কে ইন্টারনেট অ্যাক্সেস ব্যাহত হয়েছে।",
                Language::Portuguese => "O acesso à Internet foi interrompido na rede local.",
                Language::Russian => "Доступ в интернет прерван в локальной сети.",
                Language::Urdu => "مقامی نیٹ ورک پر انٹرنیٹ تک رسائی منقطع ہو گئی ہے۔",
                Language::Spanish => unreachable!(),
            }.to_string(),

            NetworkEventType::InternetRestored => {
                let dur = if let Some(idx) = original.find("de ") {
                    original[idx + 3..].trim_end_matches('.').trim()
                } else if let Some(idx) = original.find("of ") {
                    original[idx + 3..].trim_end_matches('.').trim()
                } else {
                    ""
                };

                if !dur.is_empty() {
                    match lang {
                        Language::English => format!("Service restored after an outage of {}.", dur),
                        Language::Chinese => format!("网络在中断 {} 后已恢复服务。", dur),
                        Language::Hindi => format!("{} के व्यवधान के बाद सेवा बहाल हो गई।", dur),
                        Language::Arabic => format!("تمت استعادة الخدمة بعد انقطاع دام {}.", dur),
                        Language::French => format!("Service rétabli après une interruption de {}.", dur),
                        Language::Bengali => format!("{} বাধা থাকার পর পরিষেবা পুনরুদ্ধার করা হয়েছে।", dur),
                        Language::Portuguese => format!("Serviço restabelecido após uma interrupção de {}.", dur),
                        Language::Russian => format!("Связь восстановлена после сбоя длительностью {}.", dur),
                        Language::Urdu => format!("{} کے تعطل کے بعد سروس بحال ہو گئی۔", dur),
                        Language::Spanish => unreachable!(),
                    }
                } else {
                    match lang {
                        Language::English => "Internet access restored on the local network.".to_string(),
                        Language::Chinese => "本地网络互联网访问已恢复。".to_string(),
                        Language::Hindi => "स्थानीय नेटवर्क पर इंटरनेट का उपयोग बहाल हो गया।".to_string(),
                        Language::Arabic => "تمت استعادة الوصول إلى الإنترنت على الشبكة المحلية.".to_string(),
                        Language::French => "Accès à Internet rétabli sur le réseau local.".to_string(),
                        Language::Bengali => "স্থানীয় নেটওয়ার্কে ইন্টারনেট অ্যাক্সেস পুনরুদ্ধার হয়েছে।".to_string(),
                        Language::Portuguese => "Acesso à Internet restabelecido na rede local.".to_string(),
                        Language::Russian => "Доступ в интернет в локальной сети восстановлен.".to_string(),
                        Language::Urdu => "مقامی نیٹ ورک پر انٹرنیٹ تک رسائی بحال ہو گئی۔".to_string(),
                        Language::Spanish => unreachable!(),
                    }
                }
            }

            NetworkEventType::DeviceDisconnected => {
                let target = extract_target_prefix(original, &[
                    " ya no responde",
                    " is no longer",
                    " no longer",
                    " disconnected",
                ]);
                match lang {
                    Language::English => format!("{} is no longer responding on the local network.", target),
                    Language::Chinese => format!("{} 不再响应本地网络。", target),
                    Language::Hindi => format!("{} अब स्थानीय नेटवर्क पर प्रतिक्रिया नहीं दे रहा है।", target),
                    Language::Arabic => format!("{} لم يعد يستجيب على الشبكة المحلية.", target),
                    Language::French => format!("{} ne répond plus sur le réseau local.", target),
                    Language::Bengali => format!("{} স্থানীয় নেটওয়ার্কে আর সাড়া দিচ্ছে না।", target),
                    Language::Portuguese => format!("{} não está mais respondendo na rede local.", target),
                    Language::Russian => format!("{} больше не отвечает в локальной сети.", target),
                    Language::Urdu => format!("{} اب مقامی نیٹ ورک پر جواب نہیں دے رہا ہے۔", target),
                    Language::Spanish => unreachable!(),
                }
            }

            NetworkEventType::NewDeviceDetected => {
                let target = extract_target_prefix(original, &[
                    " detectado instantáneamente",
                    " visto por primera vez",
                    " detected instantaneously",
                    " seen for the first time",
                    " detectado",
                    " detected",
                ]);
                let is_instant = original.contains("instantáneamente") || original.contains("instantaneously");
                if is_instant {
                    match lang {
                        Language::English => format!("{} detected instantaneously.", target),
                        Language::Chinese => format!("{} 已被即时检测到。", target),
                        Language::Hindi => format!("{} तुरंत पहचाना गया।", target),
                        Language::Arabic => format!("{} تم رصده فورياً.", target),
                        Language::French => format!("{} détecté instantanément.", target),
                        Language::Bengali => format!("{} তাৎক্ষণিকভাবে সনাক্ত করা হয়েছে।", target),
                        Language::Portuguese => format!("{} detectado instantaneamente.", target),
                        Language::Russian => format!("{} мгновенно обнаружен.", target),
                        Language::Urdu => format!("{} فوری طور پر پایا گیا۔", target),
                        Language::Spanish => unreachable!(),
                    }
                } else {
                    match lang {
                        Language::English => format!("{} seen for the first time on the network.", target),
                        Language::Chinese => format!("{} 首次在网络中被检测到。", target),
                        Language::Hindi => format!("{} पहली बार नेटवर्क पर देखा गया।", target),
                        Language::Arabic => format!("{} تم رصده لأول مرة على الشبكة.", target),
                        Language::French => format!("{} vu pour la première fois sur le réseau.", target),
                        Language::Bengali => format!("{} নেটওয়ার্কে প্রথমবারের মতো সনাক্ত করা হয়েছে।", target),
                        Language::Portuguese => format!("{} visto pela primeira vez na rede.", target),
                        Language::Russian => format!("{} впервые замечен в сети.", target),
                        Language::Urdu => format!("{} پہلی بار نیٹ ورک پر دیکھا گیا۔", target),
                        Language::Spanish => unreachable!(),
                    }
                }
            }

            NetworkEventType::SecurityAlert => {
                let target = extract_target_prefix(original, &[
                    " no está en la Lista Blanca",
                    " is not whitelisted",
                    " not on whitelist",
                ]);
                match lang {
                    Language::English => format!("{} is not whitelisted and has reconnected to the network.", target),
                    Language::Chinese => format!("{} 不在白名单中，已重新连接到网络。", target),
                    Language::Hindi => format!("{} श्वेतसूची में नहीं है और नेटवर्क से फिर से कनेक्ट हो गया है।", target),
                    Language::Arabic => format!("{} ليس في القائمة البيضاء وأعاد الاتصال بالشبكة.", target),
                    Language::French => format!("{} n'est pas sur la liste blanche et s'est reconnecté au réseau.", target),
                    Language::Bengali => format!("{} হোয়াইটলিস্টে নেই এবং নেটওয়ার্কে পুনরায় সংযুক্ত হয়েছে।", target),
                    Language::Portuguese => format!("{} não está na Lista Branca e se reconectou à rede.", target),
                    Language::Russian => format!("{} не в белом списке и снова подключился к сети.", target),
                    Language::Urdu => format!("{} وائٹ لسٹ میں نہیں ہے اور دوبارہ نیٹ ورک سے منسلک ہو گیا ہے۔", target),
                    Language::Spanish => unreachable!(),
                }
            }

            NetworkEventType::DeviceConnected => {
                let target = extract_target_prefix(original, &[
                    " conectado a la red",
                    " activo en la red local",
                    " connected to the network",
                    " active on the local network",
                ]);
                match lang {
                    Language::English => format!("{} active on the local network.", target),
                    Language::Chinese => format!("{} 在本地网络中活跃。", target),
                    Language::Hindi => format!("{} स्थानीय नेटवर्क पर सक्रिय है।", target),
                    Language::Arabic => format!("{} نشط على الشبكة المحلية.", target),
                    Language::French => format!("{} actif sur le réseau local.", target),
                    Language::Bengali => format!("{} স্থানীয় নেটওয়ার্কে সক্রিয়।", target),
                    Language::Portuguese => format!("{} ativo na rede local.", target),
                    Language::Russian => format!("{} активен в локальной сети.", target),
                    Language::Urdu => format!("{} مقامی نیٹ ورک پر فعال ہے۔", target),
                    Language::Spanish => unreachable!(),
                }
            }
        }
    }

    pub fn timeline_cleared_notif(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Historial de eventos de red limpiado.",
            Language::English => "Network event history cleared.",
            Language::Chinese => "网络事件历史记录已清除。",
            Language::Hindi => "नेटवर्क ईवेंट इतिहास साफ़ कर दिया गया।",
            Language::Arabic => "تم مسح سجل أحداث الشبكة.",
            Language::French => "Historique des événements réseau effacé.",
            Language::Bengali => "নেটওয়ার্ক ইভেন্ট ইতিহাস সাফ করা হয়েছে।",
            Language::Portuguese => "Histórico de eventos de rede limpo.",
            Language::Russian => "История событий сети очищена.",
            Language::Urdu => "نیٹ ورک کے واقعات کی تاریخ صاف کر دی گئی۔",
        }
    }

    pub fn device_requests_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🌐 Peticiones y Búsquedas",
            Language::English => "🌐 Requests & Queries",
            Language::Chinese => "🌐 请求与查询",
            Language::Hindi => "🌐 अनुरोध और खोज",
            Language::Arabic => "🌐 الطلبات وعمليات البحث",
            Language::French => "🌐 Requêtes et Recherches",
            Language::Bengali => "🌐 অনুরোধ ও অনুসন্ধান",
            Language::Portuguese => "🌐 Requisições e Buscas",
            Language::Russian => "🌐 Запросы и поиск",
            Language::Urdu => "🌐 درخواستیں اور تلاش",
        }
    }

    pub fn device_requests_simulate_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "➕ Simular",
            Language::English => "➕ Simulate",
            Language::Chinese => "➕ 模拟",
            Language::Hindi => "➕ सिमुलेट करें",
            Language::Arabic => "➕ محاكاة",
            Language::French => "➕ Simuler",
            Language::Bengali => "➕ সিমুলেট",
            Language::Portuguese => "➕ Simular",
            Language::Russian => "➕ Симуляция",
            Language::Urdu => "➕ نمونہ",
        }
    }

    pub fn device_requests_empty(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "No hay peticiones registradas aún para este equipo.",
            Language::English => "No requests recorded yet for this device.",
            Language::Chinese => "该设备尚未记录任何请求。",
            Language::Hindi => "इस उपकरण के लिए अभी तक कोई अनुरोध दर्ज नहीं किया गया है।",
            Language::Arabic => "لم يتم تسجيل أي طلبات لهذا الجهاز بعد.",
            Language::French => "Aucune requête enregistrée pour cet appareil pour le moment.",
            Language::Bengali => "এই ডিভাইসের জন্য এখনও কোনো অনুরোধ রেকর্ড করা হয়নি।",
            Language::Portuguese => "Nenhuma requisição registrada ainda para este dispositivo.",
            Language::Russian => "Для этого устройства еще нет зарегистрированных запросов.",
            Language::Urdu => "اس آلہ کے لیے ابھی تک کوئی درخواست ریکارڈ نہیں کی گئی ہے۔",
        }
    }

    pub fn device_requests_cleared_notif(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Historial de peticiones limpiado.",
            Language::English => "Request history cleared.",
            Language::Chinese => "请求历史记录已清除。",
            Language::Hindi => "अनुरोध इतिहास साफ़ कर दिया गया।",
            Language::Arabic => "تم مسح سجل الطلبات.",
            Language::French => "Historique des requêtes effacé.",
            Language::Bengali => "অনুরোধের ইতিহাস সাফ করা হয়েছে।",
            Language::Portuguese => "Histórico de requisições limpo.",
            Language::Russian => "История запросов очищена.",
            Language::Urdu => "درخواست کی تاریخ صاف کر دی گئی۔",
        }
    }

    // ==========================================
    // Wi-Fi Spectrum Analyzer
    // ==========================================

    pub fn wifi_modal_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "📶 Analizador de Canales Wi-Fi y Espectro",
            Language::English => "📶 Wi-Fi Channel & Spectrum Analyzer",
            Language::Chinese => "📶 Wi-Fi 信道与频谱分析器",
            Language::Hindi => "📶 वाई-फ़ाई चैनल और स्पेक्ट्रम विश्लेषक",
            Language::Arabic => "📶 محلل قنوات وطيف الواي فاي",
            Language::French => "📶 Analyseur de Spectre et Canaux Wi-Fi",
            Language::Bengali => "📶 ওয়াই-ফাই চ্যানেল ও স্পেকট্রাম বিশ্লেষক",
            Language::Portuguese => "📶 Analisador de Canais Wi-Fi e Espectro",
            Language::Russian => "📶 Анализатор каналов и спектра Wi-Fi",
            Language::Urdu => "📶 وائی فائی چینل اور سپیکٹرم تجزیہ کار",
        }
    }

    pub fn wifi_modal_desc(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Inspección de saturación en frecuencias de 2.4 GHz y 5 GHz con recomendación de canales óptimos.",
            Language::English => "Saturation inspection on 2.4 GHz and 5 GHz frequencies with optimal channel recommendations.",
            Language::Chinese => "检测 2.4 GHz 和 5 GHz 频段拥挤度并提供最佳信道推荐。",
            Language::Hindi => "2.4 GHz और 5 GHz आवृत्तियों पर भीड़ की जांच और सर्वोत्तम चैनल अनुशंसा।",
            Language::Arabic => "فحص تشبع ترددات 2.4 جيجاهرتز و5 جيجاهرتز مع توصيات بالقنوات المثلى.",
            Language::French => "Contrôle de saturation sur 2,4 GHz et 5 GHz avec recommandations de canaux optimaux.",
            Language::Bengali => "২.৪ গিগাহার্টজ এবং ৫ গিগাহার্টজে চ্যানেল স্যাচুরেশন পরীক্ষা ও সর্বোত্তম চ্যানেলের পরামর্শ।",
            Language::Portuguese => "Inspeção de saturação nas frequências de 2,4 GHz e 5 GHz com recomendação de canais ideais.",
            Language::Russian => "Анализ загруженности частот 2.4 ГГц и 5 ГГц с рекомендацией оптимальных каналов.",
            Language::Urdu => "2.4 GHz اور 5 GHz فریکوئنسیوں پر بھیڑ کی جانچ اور بہترین چینلز کی سفارش۔",
        }
    }

    pub fn wifi_scan_btn(is_scanning: bool, lang: Language) -> &'static str {
        if is_scanning {
            match lang {
                Language::Spanish => "Escaneando Wi-Fi...",
                Language::English => "Scanning Wi-Fi...",
                Language::Chinese => "正在扫描 Wi-Fi...",
                Language::Hindi => "वाई-फ़ाई स्कैन हो रहा है...",
                Language::Arabic => "جارٍ فحص الواي فاي...",
                Language::French => "Analyse Wi-Fi en cours...",
                Language::Bengali => "ওয়াই-ফাই স্ক্যান হচ্ছে...",
                Language::Portuguese => "Escaneando Wi-Fi...",
                Language::Russian => "Сканирование Wi-Fi...",
                Language::Urdu => "وائی فائی اسکین ہو رہا ہے...",
            }
        } else {
            match lang {
                Language::Spanish => "🔄 Escanear Espectro",
                Language::English => "🔄 Scan Spectrum",
                Language::Chinese => "🔄 扫描频谱",
                Language::Hindi => "🔄 स्पेक्ट्रम स्कैन करें",
                Language::Arabic => "🔄 فحص الطيف",
                Language::French => "🔄 Scanner le spectre",
                Language::Bengali => "🔄 স্পেকট্রাম স্ক্যান করুন",
                Language::Portuguese => "🔄 Escanear Espectro",
                Language::Russian => "🔄 Сканировать спектр",
                Language::Urdu => "🔄 سپیکٹرم اسکین کریں",
            }
        }
    }

    pub fn wifi_current_network(ssid: Option<&str>, channel: Option<u32>, lang: Language) -> String {
        let ch_str = channel.map(|c| c.to_string()).unwrap_or_else(|| "-".to_string());
        match lang {
            Language::Spanish => format!("Red Actual: {} (Canal {})", ssid.unwrap_or("No asociado"), ch_str),
            Language::English => format!("Current Network: {} (Channel {})", ssid.unwrap_or("Not associated"), ch_str),
            Language::Chinese => format!("当前网络: {} (信道 {})", ssid.unwrap_or("未关联"), ch_str),
            Language::Hindi => format!("वर्तमान नेटवर्क: {} (चैनल {})", ssid.unwrap_or("संबद्ध नहीं"), ch_str),
            Language::Arabic => format!("الشبكة الحالية: {} (القناة {})", ssid.unwrap_or("غير متصل"), ch_str),
            Language::French => format!("Réseau actuel : {} (Canal {})", ssid.unwrap_or("Non associé"), ch_str),
            Language::Bengali => format!("বর্তমান নেটওয়ার্ক: {} (চ্যানেল {})", ssid.unwrap_or("যুক্ত নয়"), ch_str),
            Language::Portuguese => format!("Rede Atual: {} (Canal {})", ssid.unwrap_or("Não associado"), ch_str),
            Language::Russian => format!("Текущая сеть: {} (Канал {})", ssid.unwrap_or("Не подключено"), ch_str),
            Language::Urdu => format!("موجودہ نیٹ ورک: {} (چینل {})", ssid.unwrap_or("غیر منسلک"), ch_str),
        }
    }

    pub fn wifi_current_network_none(lang: Language) -> String {
        match lang {
            Language::Spanish => "Red Actual: No disponible".to_string(),
            Language::English => "Current Network: Not available".to_string(),
            Language::Chinese => "当前网络: 不可用".to_string(),
            Language::Hindi => "वर्तमान नेटवर्क: उपलब्ध नहीं".to_string(),
            Language::Arabic => "الشبكة الحالية: غير متوفرة".to_string(),
            Language::French => "Réseau actuel : Non disponible".to_string(),
            Language::Bengali => "বর্তমান নেটওয়ার্ক: উপলব্ধ নেই".to_string(),
            Language::Portuguese => "Rede Atual: Não disponível".to_string(),
            Language::Russian => "Текущая сеть: Недоступна".to_string(),
            Language::Urdu => "موجودہ نیٹ ورک: دستیاب نہیں".to_string(),
        }
    }

    pub fn wifi_optimal_channels_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "💡 Canales Óptimos Recomendados",
            Language::English => "💡 Recommended Optimal Channels",
            Language::Chinese => "💡 推荐的最佳信道",
            Language::Hindi => "💡 अनुशंसित सर्वोत्तम चैनल",
            Language::Arabic => "💡 القنوات المثلى الموصى بها",
            Language::French => "💡 Canaux optimaux recommandés",
            Language::Bengali => "💡 প্রস্তাবিত সর্বোত্তম চ্যানেল",
            Language::Portuguese => "💡 Canais Ideais Recomendados",
            Language::Russian => "💡 Рекомендуемые оптимальные каналы",
            Language::Urdu => "💡 تجویز کردہ بہترین چینلز",
        }
    }

    pub fn wifi_channel_24_label(ch: u32, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Canal 2.4 GHz: #{}", ch),
            Language::English => format!("2.4 GHz Channel: #{}", ch),
            Language::Chinese => format!("2.4 GHz 信道: #{}", ch),
            Language::Hindi => format!("2.4 GHz चैनल: #{}", ch),
            Language::Arabic => format!("قناة 2.4 جيجاهرتز: #{}", ch),
            Language::French => format!("Canal 2,4 GHz : #{}", ch),
            Language::Bengali => format!("২.৪ GHz চ্যানেল: #{}", ch),
            Language::Portuguese => format!("Canal 2.4 GHz: #{}", ch),
            Language::Russian => format!("Канал 2.4 ГГц: #{}", ch),
            Language::Urdu => format!("2.4 GHz چینل: #{}", ch),
        }
    }

    pub fn wifi_channel_5_label(ch: u32, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Canal 5 GHz: #{}", ch),
            Language::English => format!("5 GHz Channel: #{}", ch),
            Language::Chinese => format!("5 GHz 信道: #{}", ch),
            Language::Hindi => format!("5 GHz चैनल: #{}", ch),
            Language::Arabic => format!("قناة 5 جيجاهرتز: #{}", ch),
            Language::French => format!("Canal 5 GHz : #{}", ch),
            Language::Bengali => format!("৫ GHz চ্যানেল: #{}", ch),
            Language::Portuguese => format!("Canal 5 GHz: #{}", ch),
            Language::Russian => format!("Канал 5 ГГц: #{}", ch),
            Language::Urdu => format!("5 GHz چینل: #{}", ch),
        }
    }

    pub fn wifi_recommendation_text(curr_ch: Option<u32>, best_24: u32, best_5: u32, lang: Language) -> String {
        match curr_ch {
            Some(curr) if curr <= 14 => {
                if curr == best_24 {
                    match lang {
                        Language::Spanish => format!("Tu red está en el canal {} (2.4 GHz), el cual presenta la menor interferencia en tu entorno.", curr),
                        Language::English => format!("Your network is on channel {} (2.4 GHz), which has the lowest interference in your surroundings.", curr),
                        Language::Chinese => format!("您的网络位于信道 {} (2.4 GHz)，这是当前环境下干扰最小的信道。", curr),
                        Language::Hindi => format!("आपका नेटवर्क चैनल {} (2.4 GHz) पर है, जिसमें आपके परिवेश में सबसे कम व्यवधान है।", curr),
                        Language::Arabic => format!("شبكتك على القناة {} (2.4 جيجاهرتز)، وهي الأقل تداخلاً في بيئتك المحيطة.", curr),
                        Language::French => format!("Votre réseau est sur le canal {} (2,4 GHz), qui présente le moins d'interférences dans votre environnement.", curr),
                        Language::Bengali => format!("আপনার নেটওয়ার্ক চ্যানেল {} (২.৪ GHz)-এ রয়েছে, যাতে সর্বনিম্ন হস্তক্ষেপ রয়েছে।", curr),
                        Language::Portuguese => format!("Sua rede está no canal {} (2.4 GHz), que apresenta a menor interferência no seu ambiente.", curr),
                        Language::Russian => format!("Ваша сеть работает на канале {} (2.4 ГГц), на котором меньше всего помех в вашем окружении.", curr),
                        Language::Urdu => format!("آپ کا نیٹ ورک چینل {} (2.4 GHz) پر ہے، جس میں کم سے کم مداخلت ہے۔", curr),
                    }
                } else {
                    match lang {
                        Language::Spanish => format!("Tu red está en el canal {} (2.4 GHz). Se recomienda cambiar al canal {} para evitar interferencias de redes vecinas.", curr, best_24),
                        Language::English => format!("Your network is on channel {} (2.4 GHz). It is recommended to switch to channel {} to avoid neighbor network interference.", curr, best_24),
                        Language::Chinese => format!("您的网络在信道 {} (2.4 GHz)。建议切换到信道 {} 以避免邻近网络的干扰。", curr, best_24),
                        Language::Hindi => format!("आपका नेटवर्क चैनल {} (2.4 GHz) पर है। पड़ोसी नेटवर्क के हस्तक्षेप से बचने के लिए चैनल {} पर जाने की सलाह दी जाती है।", curr, best_24),
                        Language::Arabic => format!("شبكتك على القناة {} (2.4 جيجاهرتز). يوصى بالتبديل إلى القناة {} لتجنب التداخل من الشبكات المجاورة.", curr, best_24),
                        Language::French => format!("Votre réseau est sur le canal {} (2,4 GHz). Il est recommandé de passer au canal {} pour éviter les interférences des réseaux voisins.", curr, best_24),
                        Language::Bengali => format!("আপনার নেটওয়ার্ক চ্যানেল {} (২.৪ GHz)-এ আছে। পার্শ্ববর্তী হস্তক্ষেপ এড়াতে চ্যানেল {} এ যাওয়ার পরামর্শ দেওয়া হচ্ছে।", curr, best_24),
                        Language::Portuguese => format!("Sua rede está no canal {} (2.4 GHz). Recomenda-se mudar para o canal {} para evitar interferências de redes vizinhas.", curr, best_24),
                        Language::Russian => format!("Ваша сеть работает на канале {} (2.4 ГГц). Рекомендуется перейти на канал {}, чтобы избежать помех от соседних сетей.", curr, best_24),
                        Language::Urdu => format!("آپ کا نیٹ ورک چینل {} (2.4 GHz) پر ہے۔ قریبی نیٹ ورکس کی مداخلت سے بچنے کے لیے چینل {} پر جانے کی سفارش کی جاتی ہے۔", curr, best_24),
                    }
                }
            }
            Some(curr) => match lang {
                Language::Spanish => format!("Tu red opera en el canal {} (5 GHz). El espectro de 5 GHz está optimizado.", curr),
                Language::English => format!("Your network operates on channel {} (5 GHz). The 5 GHz spectrum is optimized.", curr),
                Language::Chinese => format!("您的网络运行在信道 {} (5 GHz)。5 GHz 频段已优化。", curr),
                Language::Hindi => format!("आपका नेटवर्क चैनल {} (5 GHz) पर काम करता है। 5 GHz स्पेक्ट्रम अनुकूलित है।", curr),
                Language::Arabic => format!("تعمل شبكتك على القناة {} (5 جيجاهرتز). طيف 5 جيجاهرتز مُحسّن.", curr),
                Language::French => format!("Votre réseau fonctionne sur le canal {} (5 GHz). Le spectre 5 GHz est optimisé.", curr),
                Language::Bengali => format!("আপনার নেটওয়ার্ক চ্যানেল {} (৫ GHz)-এ পরিচালিত হচ্ছে। ৫ GHz স্পেকট্রাম অপ্টিমাইজ করা হয়েছে।", curr),
                Language::Portuguese => format!("Sua rede opera no canal {} (5 GHz). O espectro de 5 GHz está otimizado.", curr),
                Language::Russian => format!("Ваша сеть работает на канале {} (5 ГГц). Диапазон 5 ГГц оптимизирован.", curr),
                Language::Urdu => format!("آپ کا نیٹ ورک چینل {} (5 GHz) پر چلتا ہے۔ 5 GHz سپیکٹرم بہترین ہے۔", curr),
            },
            None => match lang {
                Language::Spanish => format!("Canal 2.4 GHz recomendado: Canal {} (menos congestionado). Canal 5 GHz recomendado: Canal {}.", best_24, best_5),
                Language::English => format!("Recommended 2.4 GHz: Channel {} (least crowded). Recommended 5 GHz: Channel {}.", best_24, best_5),
                Language::Chinese => format!("推荐 2.4 GHz: 信道 {} (最少拥挤)。推荐 5 GHz: 信道 {}。", best_24, best_5),
                Language::Hindi => format!("अनुशंसित 2.4 GHz: चैनल {} (कम भीड़)। अनुशंसित 5 GHz: चैनल {}।", best_24, best_5),
                Language::Arabic => format!("القناة الموصى بها لـ 2.4 جيجاهرتز: قناة {} (الأقل ازدحاماً). القناة الموصى بها لـ 5 جيجاهرتز: قناة {}.", best_24, best_5),
                Language::French => format!("Recommandé 2,4 GHz : Canal {} (le moins encombré). Recommandé 5 GHz : Canal {}.", best_24, best_5),
                Language::Bengali => format!("প্রস্তাবিত ২.৪ GHz: চ্যানেল {} (কম ভিড়)। প্রস্তাবিত ৫ GHz: চ্যানেল {}।", best_24, best_5),
                Language::Portuguese => format!("Canal 2.4 GHz recomendado: Canal {} (menos congestionado). Canal 5 GHz recomendado: Canal {}.", best_24, best_5),
                Language::Russian => format!("Рекомендуемый 2.4 ГГц: Канал {} (наименее загружен). Рекомендуемый 5 ГГц: Канал {}.", best_24, best_5),
                Language::Urdu => format!("تجویز کردہ 2.4 GHz: چینل {} (کم ترین ہجوم)۔ تجویز کردہ 5 GHz: چینل {}۔", best_24, best_5),
            },
        }
    }

    pub fn wifi_occupancy_24_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Ocupación Espectro 2.4 GHz (Canales 1, 6, 11 no solapados)",
            Language::English => "2.4 GHz Spectrum Occupancy (Non-overlapping channels 1, 6, 11)",
            Language::Chinese => "2.4 GHz 频谱占用 (非重叠信道 1, 6, 11)",
            Language::Hindi => "2.4 GHz स्पेक्ट्रम व्यस्तता (गैर-ओवरलैपिंग चैनल 1, 6, 11)",
            Language::Arabic => "إشغال طيف 2.4 جيجاهرتز (القنوات غير المتداخلة 1، 6، 11)",
            Language::French => "Occupation du spectre 2,4 GHz (canaux non chevauchants 1, 6, 11)",
            Language::Bengali => "২.৪ GHz স্পেকট্রাম দখল (নন-ওভারল্যাপিং চ্যানেল ১, ৬, ১১)",
            Language::Portuguese => "Ocupação do Espectro 2.4 GHz (Canais 1, 6, 11 não sobrepostos)",
            Language::Russian => "Занятость спектра 2.4 ГГц (непересекающиеся каналы 1, 6, 11)",
            Language::Urdu => "2.4 GHz سپیکٹرم کا قبضہ (غیر اوورلیپنگ چینلز 1، 6، 11)",
        }
    }

    pub fn wifi_channel_box_title(ch: u32, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Canal {}", ch),
            Language::English => format!("Channel {}", ch),
            Language::Chinese => format!("信道 {}", ch),
            Language::Hindi => format!("चैनल {}", ch),
            Language::Arabic => format!("قناة {}", ch),
            Language::French => format!("Canal {}", ch),
            Language::Bengali => format!("চ্যানেল {}", ch),
            Language::Portuguese => format!("Canal {}", ch),
            Language::Russian => format!("Канал {}", ch),
            Language::Urdu => format!("چینل {}", ch),
        }
    }

    pub fn wifi_networks_count(count: usize, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("{} Redes", count),
            Language::English => format!("{} Networks", count),
            Language::Chinese => format!("{} 个网络", count),
            Language::Hindi => format!("{} नेटवर्क", count),
            Language::Arabic => format!("{} شبكات", count),
            Language::French => format!("{} réseaux", count),
            Language::Bengali => format!("{}টি নেটওয়ার্ক", count),
            Language::Portuguese => format!("{} Redes", count),
            Language::Russian => format!("{} сетей", count),
            Language::Urdu => format!("{} نیٹ ورکس", count),
        }
    }

    pub fn wifi_max_signal(sig: u8, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Máx: {}%", sig),
            Language::English => format!("Max: {}%", sig),
            Language::Chinese => format!("最高: {}%", sig),
            Language::Hindi => format!("अधिकतम: {}%", sig),
            Language::Arabic => format!("الحد الأقصى: {}%", sig),
            Language::French => format!("Max : {}%", sig),
            Language::Bengali => format!("সর্বোচ্চ: {}%", sig),
            Language::Portuguese => format!("Máx: {}%", sig),
            Language::Russian => format!("Макс: {}%", sig),
            Language::Urdu => format!("زیادہ سے زیادہ: {}%", sig),
        }
    }

    pub fn wifi_detected_networks_header(count: usize, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Redes Wi-Fi Detectadas ({})", count),
            Language::English => format!("Detected Wi-Fi Networks ({})", count),
            Language::Chinese => format!("检测到的 Wi-Fi 网络 ({})", count),
            Language::Hindi => format!("पहचाने गए वाई-फ़ाई नेटवर्क ({})", count),
            Language::Arabic => format!("شبكات الواي فاي المكتشفة ({})", count),
            Language::French => format!("Réseaux Wi-Fi détectés ({})", count),
            Language::Bengali => format!("সনাক্ত করা ওয়াই-ফাই নেটওয়ার্ক ({})", count),
            Language::Portuguese => format!("Redes Wi-Fi Detectadas ({})", count),
            Language::Russian => format!("Обнаруженные сети Wi-Fi ({})", count),
            Language::Urdu => format!("پائے گئے وائی فائی نیٹ ورکس ({})", count),
        }
    }

    pub fn wifi_badge_current(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "ACTUAL",
            Language::English => "CURRENT",
            Language::Chinese => "当前",
            Language::Hindi => "वर्तमान",
            Language::Arabic => "الحالي",
            Language::French => "ACTUEL",
            Language::Bengali => "বর্তমান",
            Language::Portuguese => "ATUAL",
            Language::Russian => "ТЕКУЩАЯ",
            Language::Urdu => "موجودہ",
        }
    }

    pub fn wifi_prompt_scan(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Presiona 'Escanear Espectro' para analizar las redes Wi-Fi del entorno.",
            Language::English => "Press 'Scan Spectrum' to analyze surrounding Wi-Fi networks.",
            Language::Chinese => "点击“扫描频谱”以分析周围的 Wi-Fi 网络。",
            Language::Hindi => "आस-पास के वाई-फ़ाई नेटवर्क का विश्लेषण करने के लिए 'स्पेक्ट्रम स्कैन करें' दबाएं।",
            Language::Arabic => "اضغط على 'فحص الطيف' لتحليل شبكات الواي فاي المحيطة.",
            Language::French => "Appuyez sur « Scanner le spectre » pour analyser les réseaux Wi-Fi environnants.",
            Language::Bengali => "আশেপাশের ওয়াই-ফাই নেটওয়ার্ক বিশ্লেষণ করতে 'স্পেকট্রাম স্ক্যান করুন' টিপুন।",
            Language::Portuguese => "Pressione 'Escanear Espectro' para analisar as redes Wi-Fi ao redor.",
            Language::Russian => "Нажмите «Сканировать спектр», чтобы проанализировать окружающие сети Wi-Fi.",
            Language::Urdu => "آس پاس کے وائی فائی نیٹ ورکس کا تجزیہ کرنے کے لیے 'سپیکٹرم اسکین کریں' دبائیں۔",
        }
    }

    // ==========================================
    // Router Storage / USB NAS
    // ==========================================

    pub fn storage_btn_credentials(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🔑 Credenciales",
            Language::English => "🔑 Credentials",
            Language::Chinese => "🔑 凭据",
            Language::Hindi => "🔑 क्रेडेंशियल्स",
            Language::Arabic => "🔑 بيانات الاعتماد",
            Language::French => "🔑 Identifiants",
            Language::Bengali => "🔑 প্রমাণপত্র",
            Language::Portuguese => "🔑 Credenciais",
            Language::Russian => "🔑 Учетные данные",
            Language::Urdu => "🔑 اسناد",
        }
    }

    pub fn storage_creds_user_placeholder(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Usuario (ej. admin o guest)",
            Language::English => "Username (e.g. admin or guest)",
            Language::Chinese => "用户名 (例如 admin 或 guest)",
            Language::Hindi => "उपयोगकर्ता नाम (उदा. admin या guest)",
            Language::Arabic => "اسم المستخدم (مثل admin أو guest)",
            Language::French => "Nom d'utilisateur (ex. admin ou guest)",
            Language::Bengali => "ব্যবহারকারীর নাম (যেমন admin বা guest)",
            Language::Portuguese => "Usuário (ex. admin ou guest)",
            Language::Russian => "Имя пользователя (напр. admin или guest)",
            Language::Urdu => "صارف نام (مثلاً admin یا guest)",
        }
    }

    pub fn storage_creds_pass_placeholder(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Contraseña",
            Language::English => "Password",
            Language::Chinese => "密码",
            Language::Hindi => "पासवर्ड",
            Language::Arabic => "كلمة المرور",
            Language::French => "Mot de passe",
            Language::Bengali => "পাসওয়ার্ড",
            Language::Portuguese => "Senha",
            Language::Russian => "Пароль",
            Language::Urdu => "پاس ورڈ",
        }
    }

    pub fn storage_creds_user_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Usuario:",
            Language::English => "Username:",
            Language::Chinese => "用户名:",
            Language::Hindi => "उपयोगकर्ता:",
            Language::Arabic => "اسم المستخدم:",
            Language::French => "Nom d'utilisateur :",
            Language::Bengali => "ব্যবহারকারী:",
            Language::Portuguese => "Usuário:",
            Language::Russian => "Пользователь:",
            Language::Urdu => "صارف نام:",
        }
    }

    pub fn storage_creds_pass_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Contraseña:",
            Language::English => "Password:",
            Language::Chinese => "密码:",
            Language::Hindi => "पासवर्ड:",
            Language::Arabic => "كلمة المرور:",
            Language::French => "Mot de passe :",
            Language::Bengali => "পাসওয়ার্ড:",
            Language::Portuguese => "Senha:",
            Language::Russian => "Пароль:",
            Language::Urdu => "پاس ورڈ:",
        }
    }

    pub fn storage_creds_group_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Grupo:",
            Language::English => "Workgroup:",
            Language::Chinese => "工作组:",
            Language::Hindi => "कार्यसमूह:",
            Language::Arabic => "مجموعة العمل:",
            Language::French => "Groupe :",
            Language::Bengali => "ওয়ার্কগ্রুপ:",
            Language::Portuguese => "Grupo:",
            Language::Russian => "Рабочая группа:",
            Language::Urdu => "ورک گروپ:",
        }
    }

    pub fn storage_creds_save_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Guardar y Conectar",
            Language::English => "Save and Connect",
            Language::Chinese => "保存并连接",
            Language::Hindi => "सहेजें और कनेक्ट करें",
            Language::Arabic => "حفظ والاتصال",
            Language::French => "Enregistrer et connecter",
            Language::Bengali => "সংরক্ষণ করুন এবং সংযোগ করুন",
            Language::Portuguese => "Salvar e Conectar",
            Language::Russian => "Сохранить и подключить",
            Language::Urdu => "محفوظ کریں اور منسلک کریں",
        }
    }

    pub fn cancel(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Cancelar",
            Language::English => "Cancel",
            Language::Chinese => "取消",
            Language::Hindi => "रद्द करें",
            Language::Arabic => "إلغاء",
            Language::French => "Annuler",
            Language::Bengali => "বাতিল করুন",
            Language::Portuguese => "Cancelar",
            Language::Russian => "Отмена",
            Language::Urdu => "منسوخ کریں",
        }
    }

    pub fn storage_space_info(free_str: &str, total_str: &str, percent: f64, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("💾 Espacio: {} libres de {} ({:.2}% en uso)", free_str, total_str, percent),
            Language::English => format!("💾 Space: {} free of {} ({:.2}% in use)", free_str, total_str, percent),
            Language::Chinese => format!("💾 空间: {} 可用 / 总计 {} (已用 {:.2}%)", free_str, total_str, percent),
            Language::Hindi => format!("💾 स्थान: {} में से {} खाली ({:.2}% उपयोग में)", total_str, free_str, percent),
            Language::Arabic => format!("💾 المساحة: {} متوفرة من أصل {} (مستخدم {:.2}%)", free_str, total_str, percent),
            Language::French => format!("💾 Espace : {} libres sur {} ({:.2}% utilisé)", free_str, total_str, percent),
            Language::Bengali => format!("💾 স্থান: {} এর মধ্যে {} খালি ({:.2}% ব্যবহৃত)", total_str, free_str, percent),
            Language::Portuguese => format!("💾 Espaço: {} livres de {} ({:.2}% em uso)", free_str, total_str, percent),
            Language::Russian => format!("💾 Память: {} свободно из {} ({:.2}% занято)", free_str, total_str, percent),
            Language::Urdu => format!("💾 جگہ: {} میں سے {} خالی ({:.2}% زیر استعمال)", total_str, free_str, percent),
        }
    }

    pub fn storage_btn_root(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🏠 Raíz",
            Language::English => "🏠 Root",
            Language::Chinese => "🏠 根目录",
            Language::Hindi => "🏠 रूट",
            Language::Arabic => "🏠 الجذر",
            Language::French => "🏠 Racine",
            Language::Bengali => "🏠 রুট",
            Language::Portuguese => "🏠 Raiz",
            Language::Russian => "🏠 Корень",
            Language::Urdu => "🏠 روٹ",
        }
    }

    pub fn storage_root_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Raíz",
            Language::English => "Root",
            Language::Chinese => "根目录",
            Language::Hindi => "रूट",
            Language::Arabic => "الجذر",
            Language::French => "Racine",
            Language::Bengali => "রুট",
            Language::Portuguese => "Raiz",
            Language::Russian => "Корень",
            Language::Urdu => "روٹ",
        }
    }

    pub fn storage_upload_completed(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "✓ Completado",
            Language::English => "✓ Completed",
            Language::Chinese => "✓ 已完成",
            Language::Hindi => "✓ पूरा हुआ",
            Language::Arabic => "✓ مكتمل",
            Language::French => "✓ Terminé",
            Language::Bengali => "✓ সম্পন্ন",
            Language::Portuguese => "✓ Concluído",
            Language::Russian => "✓ Завершено",
            Language::Urdu => "✓ مکمل ہوا",
        }
    }

    pub fn storage_upload_error(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "✕ Error",
            Language::English => "✕ Error",
            Language::Chinese => "✕ 错误",
            Language::Hindi => "✕ त्रुटि",
            Language::Arabic => "✕ خطأ",
            Language::French => "✕ Erreur",
            Language::Bengali => "✕ ত্রুটি",
            Language::Portuguese => "✕ Erro",
            Language::Russian => "✕ Ошибка",
            Language::Urdu => "✕ خرابی",
        }
    }

    pub fn storage_upload_file_progress(curr: &str, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Archivo: {}", curr),
            Language::English => format!("File: {}", curr),
            Language::Chinese => format!("文件: {}", curr),
            Language::Hindi => format!("फ़ाइल: {}", curr),
            Language::Arabic => format!("الملف: {}", curr),
            Language::French => format!("Fichier : {}", curr),
            Language::Bengali => format!("ফাইল: {}", curr),
            Language::Portuguese => format!("Arquivo: {}", curr),
            Language::Russian => format!("Файл: {}", curr),
            Language::Urdu => format!("فائل: {}", curr),
        }
    }

    pub fn storage_upload_folder_progress(idx: usize, total: usize, curr: &str, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Archivo {} de {}: {}", idx, total, curr),
            Language::English => format!("File {} of {}: {}", idx, total, curr),
            Language::Chinese => format!("文件 {} / {}: {}", idx, total, curr),
            Language::Hindi => format!("फ़ाइल {} / {}: {}", idx, total, curr),
            Language::Arabic => format!("ملف {} من {}: {}", idx, total, curr),
            Language::French => format!("Fichier {} sur {} : {}", idx, total, curr),
            Language::Bengali => format!("ফাইল {} / {}: {}", idx, total, curr),
            Language::Portuguese => format!("Arquivo {} de {}: {}", idx, total, curr),
            Language::Russian => format!("Файл {} из {}: {}", idx, total, curr),
            Language::Urdu => format!("فائل {} از {}: {}", idx, total, curr),
        }
    }

    pub fn storage_create_folder_prompt(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Crear carpeta en esta ubicación:",
            Language::English => "Create folder in this location:",
            Language::Chinese => "在此位置创建文件夹:",
            Language::Hindi => "इस स्थान पर फ़ोल्डर बनाएं:",
            Language::Arabic => "إنشاء مجلد في هذا المسار:",
            Language::French => "Créer un dossier à cet emplacement :",
            Language::Bengali => "এই অবস্থানে ফোল্ডার তৈরি করুন:",
            Language::Portuguese => "Criar pasta neste local:",
            Language::Russian => "Создать папку в этом расположении:",
            Language::Urdu => "اس مقام پر فولڈر بنائیں:",
        }
    }

    pub fn storage_new_folder_placeholder(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Nombre de la carpeta...",
            Language::English => "Folder name...",
            Language::Chinese => "文件夹名称...",
            Language::Hindi => "फ़ोल्डर का नाम...",
            Language::Arabic => "اسم المجلد...",
            Language::French => "Nom du dossier...",
            Language::Bengali => "ফোল্ডারের নাম...",
            Language::Portuguese => "Nome da pasta...",
            Language::Russian => "Имя папки...",
            Language::Urdu => "فولڈر کا نام...",
        }
    }

    pub fn storage_create_confirm(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "✓ Crear",
            Language::English => "✓ Create",
            Language::Chinese => "✓ 创建",
            Language::Hindi => "✓ बनाएं",
            Language::Arabic => "✓ إنشاء",
            Language::French => "✓ Créer",
            Language::Bengali => "✓ তৈরি করুন",
            Language::Portuguese => "✓ Criar",
            Language::Russian => "✓ Создать",
            Language::Urdu => "✓ بنائیں",
        }
    }

    pub fn storage_props_title(name: &str, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Propiedades de '{}'", name),
            Language::English => format!("Properties of '{}'", name),
            Language::Chinese => format!("'{}' 的属性", name),
            Language::Hindi => format!("'{}' के गुण", name),
            Language::Arabic => format!("خصائص '{}'", name),
            Language::French => format!("Propriétés de « {} »", name),
            Language::Bengali => format!("'{}' এর বৈশিষ্ট্য", name),
            Language::Portuguese => format!("Propriedades de '{}'", name),
            Language::Russian => format!("Свойства «{}»", name),
            Language::Urdu => format!("'{}' کی خصوصیات", name),
        }
    }

    pub fn storage_prop_type(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Tipo:",
            Language::English => "Type:",
            Language::Chinese => "类型:",
            Language::Hindi => "प्रकार:",
            Language::Arabic => "النوع:",
            Language::French => "Type :",
            Language::Bengali => "ধরন:",
            Language::Portuguese => "Tipo:",
            Language::Russian => "Тип:",
            Language::Urdu => "قسم:",
        }
    }

    pub fn storage_prop_full_path(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Ruta Completa:",
            Language::English => "Full Path:",
            Language::Chinese => "完整路径:",
            Language::Hindi => "पूर्ण पथ:",
            Language::Arabic => "المسار الكامل:",
            Language::French => "Chemin complet :",
            Language::Bengali => "সম্পূর্ণ পাথ:",
            Language::Portuguese => "Caminho Completo:",
            Language::Russian => "Полный путь:",
            Language::Urdu => "مکمل راستہ:",
        }
    }

    pub fn storage_prop_size(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Tamaño:",
            Language::English => "Size:",
            Language::Chinese => "大小:",
            Language::Hindi => "आकार:",
            Language::Arabic => "الحجم:",
            Language::French => "Taille :",
            Language::Bengali => "আকার:",
            Language::Portuguese => "Tamanho:",
            Language::Russian => "Размер:",
            Language::Urdu => "سائز:",
        }
    }

    pub fn storage_prop_modified(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Modificado:",
            Language::English => "Modified:",
            Language::Chinese => "修改时间:",
            Language::Hindi => "संशोधित:",
            Language::Arabic => "تاريخ التعديل:",
            Language::French => "Modifié le :",
            Language::Bengali => "সংশোধিত:",
            Language::Portuguese => "Modificado:",
            Language::Russian => "Изменено:",
            Language::Urdu => "ترمیم شدہ:",
        }
    }

    pub fn storage_prop_attributes(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Atributos:",
            Language::English => "Attributes:",
            Language::Chinese => "属性:",
            Language::Hindi => "विशेषताएं:",
            Language::Arabic => "السمات:",
            Language::French => "Attributs :",
            Language::Bengali => "বৈশিষ্ট্য:",
            Language::Portuguese => "Atributos:",
            Language::Russian => "Атрибуты:",
            Language::Urdu => "خصوصیات:",
        }
    }

    pub fn storage_file_type_name(ft: StorageFileType, lang: Language) -> &'static str {
        match ft {
            StorageFileType::Folder => match lang {
                Language::Spanish => "Carpeta",
                Language::English => "Folder",
                Language::Chinese => "文件夹",
                Language::Hindi => "फ़ोल्डर",
                Language::Arabic => "مجلد",
                Language::French => "Dossier",
                Language::Bengali => "ফোল্ডার",
                Language::Portuguese => "Pasta",
                Language::Russian => "Папка",
                Language::Urdu => "فولڈر",
            },
            StorageFileType::Image => match lang {
                Language::Spanish => "Imagen",
                Language::English => "Image",
                Language::Chinese => "图像",
                Language::Hindi => "छवि",
                Language::Arabic => "صورة",
                Language::French => "Image",
                Language::Bengali => "ছবি",
                Language::Portuguese => "Imagem",
                Language::Russian => "Изображение",
                Language::Urdu => "تصویر",
            },
            StorageFileType::Video => match lang {
                Language::Spanish => "Video",
                Language::English => "Video",
                Language::Chinese => "视频",
                Language::Hindi => "वीडियो",
                Language::Arabic => "فيديو",
                Language::French => "Vidéo",
                Language::Bengali => "ভিডিও",
                Language::Portuguese => "Vídeo",
                Language::Russian => "Видео",
                Language::Urdu => "ویڈیو",
            },
            StorageFileType::Audio => match lang {
                Language::Spanish => "Audio",
                Language::English => "Audio",
                Language::Chinese => "音频",
                Language::Hindi => "ऑडियो",
                Language::Arabic => "صوت",
                Language::French => "Audio",
                Language::Bengali => "অডিও",
                Language::Portuguese => "Áudio",
                Language::Russian => "Аудио",
                Language::Urdu => "آڈیو",
            },
            StorageFileType::Document => match lang {
                Language::Spanish => "Documento",
                Language::English => "Document",
                Language::Chinese => "文档",
                Language::Hindi => "दस्तावेज़",
                Language::Arabic => "مستند",
                Language::French => "Document",
                Language::Bengali => "নথি",
                Language::Portuguese => "Documento",
                Language::Russian => "Документ",
                Language::Urdu => "دستاویز",
            },
            StorageFileType::Archive => match lang {
                Language::Spanish => "Archivo Comprimido",
                Language::English => "Archive / Zip",
                Language::Chinese => "压缩文件",
                Language::Hindi => "अभिलेखागार",
                Language::Arabic => "ملف مضغوط",
                Language::French => "Archive compressée",
                Language::Bengali => "সংকুচিত ফাইল",
                Language::Portuguese => "Arquivo Compactado",
                Language::Russian => "Архив",
                Language::Urdu => "آرکائیو",
            },
            StorageFileType::Executable => match lang {
                Language::Spanish => "Ejecutable / Paquete",
                Language::English => "Executable / Package",
                Language::Chinese => "可执行程序/软件包",
                Language::Hindi => "निष्पादन योग्य / पैकेज",
                Language::Arabic => "ملف تنفيذي / حزمة",
                Language::French => "Exécutable / Paquet",
                Language::Bengali => "এক্সিকিউটেবল / প্যাকেজ",
                Language::Portuguese => "Executável / Pacote",
                Language::Russian => "Исполняемый файл / пакет",
                Language::Urdu => "قابل عمل / پیکیج",
            },
            StorageFileType::Code => match lang {
                Language::Spanish => "Código Fuente",
                Language::English => "Source Code",
                Language::Chinese => "源代码",
                Language::Hindi => "स्रोत कोड",
                Language::Arabic => "كود المصدر",
                Language::French => "Code source",
                Language::Bengali => "উৎস কোড",
                Language::Portuguese => "Código-Fonte",
                Language::Russian => "Исходный код",
                Language::Urdu => "سورس کوڈ",
            },
            StorageFileType::Unknown => match lang {
                Language::Spanish => "Archivo",
                Language::English => "File",
                Language::Chinese => "文件",
                Language::Hindi => "फ़ाइल",
                Language::Arabic => "ملف",
                Language::French => "Fichier",
                Language::Bengali => "ফাইল",
                Language::Portuguese => "Arquivo",
                Language::Russian => "Файл",
                Language::Urdu => "فائل",
            },
        }
    }

    pub fn storage_loading_files(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Cargando archivos del router...",
            Language::English => "Loading files from router...",
            Language::Chinese => "正在从路由器加载文件...",
            Language::Hindi => "राउटर से फ़ाइलें लोड हो रही हैं...",
            Language::Arabic => "جارٍ تحميل الملفات من الراوتر...",
            Language::French => "Chargement des fichiers du routeur...",
            Language::Bengali => "রাউটার থেকে ফাইল লোড হচ্ছে...",
            Language::Portuguese => "Carregando arquivos do roteador...",
            Language::Russian => "Загрузка файлов с роутера...",
            Language::Urdu => "روٹر سے فائلیں لوڈ ہو رہی ہیں...",
        }
    }

    pub fn storage_no_filter_match(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Ningún archivo coincide con el filtro.",
            Language::English => "No files match the filter.",
            Language::Chinese => "没有文件匹配筛选条件。",
            Language::Hindi => "कोई फ़ाइल फ़िल्टर से मेल नहीं खाती।",
            Language::Arabic => "لا توجد ملفات تطابق معيار التصفية.",
            Language::French => "Aucun fichier ne correspond au filtre.",
            Language::Bengali => "কোনো ফাইল ফিল্টারের সাথে মেলে না।",
            Language::Portuguese => "Nenhum arquivo corresponde ao filtro.",
            Language::Russian => "Нет файлов, соответствующих фильтру.",
            Language::Urdu => "کوئی فائل فلٹر سے مماثل نہیں ہے۔",
        }
    }

    pub fn storage_summary_counts(total: usize, dirs: usize, files: usize, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("{} elementos ({} carpetas, {} archivos)", total, dirs, files),
            Language::English => format!("{} items ({} folders, {} files)", total, dirs, files),
            Language::Chinese => format!("{} 个项目 ({} 个文件夹, {} 个文件)", total, dirs, files),
            Language::Hindi => format!("{} आइटम ({} फ़ोल्डर, {} फ़ाइलें)", total, dirs, files),
            Language::Arabic => format!("{} عناصر ({} مجلدات، {} ملفات)", total, dirs, files),
            Language::French => format!("{} éléments ({} dossiers, {} fichiers)", total, dirs, files),
            Language::Bengali => format!("{}টি আইটেম ({}টি ফোল্ডার, {}টি ফাইল)", total, dirs, files),
            Language::Portuguese => format!("{} itens ({} pastas, {} arquivos)", total, dirs, files),
            Language::Russian => format!("{} элементов ({} папок, {} файлов)", total, dirs, files),
            Language::Urdu => format!("{} آئٹمز ({} فولڈرز، {} فائلیں)", total, dirs, files),
        }
    }

    // ==========================================
    // Universal Router UPnP / IGD
    // ==========================================

    pub fn upnp_modal_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🌐 Router Universal UPnP / IGD",
            Language::English => "🌐 Universal Router UPnP / IGD",
            Language::Chinese => "🌐 通用路由器 UPnP / IGD",
            Language::Hindi => "🌐 यूनिवर्सल राउटर UPnP / IGD",
            Language::Arabic => "🌐 موجه UPnP / IGD العالمي",
            Language::French => "🌐 Routeur Universel UPnP / IGD",
            Language::Bengali => "🌐 সার্বজনীন রাউটার UPnP / IGD",
            Language::Portuguese => "🌐 Roteador Universal UPnP / IGD",
            Language::Russian => "🌐 Универсальный роутер UPnP / IGD",
            Language::Urdu => "🌐 یونیورسل روٹر UPnP / IGD",
        }
    }

    pub fn upnp_modal_desc(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Consulta universal de puerta de enlace, IP pública externa y tabla de reenvío de puertos NAT (UPnP).",
            Language::English => "Universal query of gateway, public external IP, and NAT port forwarding table (UPnP).",
            Language::Chinese => "通用网关、公网外部 IP 和 NAT 端口转发规则查询 (UPnP)。",
            Language::Hindi => "गेटवे, सार्वजनिक बाहरी आईपी और एनएटी पोर्ट अग्रेषण तालिका की सार्वभौमिक क्वेरी (UPnP)।",
            Language::Arabic => "استعلام شامل عن البوابة وعنوان IP الخارجي وجدول إعادة توجيه منافذ NAT (UPnP).",
            Language::French => "Interrogation universelle de passerelle, IP publique externe et table de redirection NAT (UPnP).",
            Language::Bengali => "গেটওয়ে, সর্বজনীন বহিরাগত আইপি এবং NAT পোর্ট ফরওয়ার্ডিং টেবিলের সার্বজনীন অনুসন্ধান (UPnP)।",
            Language::Portuguese => "Consulta universal de gateway, IP público externo e tabela de encaminhamento NAT (UPnP).",
            Language::Russian => "Запрос шлюза, внешнего публичного IP и таблицы переадресации портов NAT (UPnP).",
            Language::Urdu => "گیٹ وے، پبلک بیرونی آئی پی اور NAT پورٹ فارورڈنگ ٹیبل کا یونیورسل استفسار (UPnP)۔",
        }
    }

    pub fn upnp_query_btn(is_loading: bool, lang: Language) -> &'static str {
        if is_loading {
            match lang {
                Language::Spanish => "Consultando...",
                Language::English => "Querying...",
                Language::Chinese => "正在查询...",
                Language::Hindi => "पूछताछ जारी...",
                Language::Arabic => "جارٍ الاستعلام...",
                Language::French => "Interrogation...",
                Language::Bengali => "অনুসন্ধান চলছে...",
                Language::Portuguese => "Consultando...",
                Language::Russian => "Запрос...",
                Language::Urdu => "استفسار جاری ہے...",
            }
        } else {
            match lang {
                Language::Spanish => "🔍 Consultar Router",
                Language::English => "🔍 Query Router",
                Language::Chinese => "🔍 查询路由器",
                Language::Hindi => "🔍 राउटर से पूछें",
                Language::Arabic => "🔍 استعلام الراوتر",
                Language::French => "🔍 Interroger le routeur",
                Language::Bengali => "🔍 রাউটার অনুসন্ধান করুন",
                Language::Portuguese => "🔍 Consultar Roteador",
                Language::Russian => "🔍 Опросить роутер",
                Language::Urdu => "🔍 روٹر سے پوچھیں",
            }
        }
    }

    pub fn upnp_prompt(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Haz clic en 'Consultar Router' para interrogar la interfaz UPnP IGD.",
            Language::English => "Click 'Query Router' to interrogate the UPnP IGD interface.",
            Language::Chinese => "点击“查询路由器”以查询 UPnP IGD 接口。",
            Language::Hindi => "UPnP IGD इंटरफ़ेस से पूछताछ करने के लिए 'राउटर से पूछें' पर क्लिक करें।",
            Language::Arabic => "انقر فوق 'استعلام الراوتر' لفحص واجهة UPnP IGD.",
            Language::French => "Cliquez sur « Interroger le routeur » pour interroger l'interface UPnP IGD.",
            Language::Bengali => "UPnP IGD ইন্টারফেস অনুসন্ধান করতে 'রাউটার অনুসন্ধান করুন' এ ক্লিক করুন।",
            Language::Portuguese => "Clique em 'Consultar Roteador' para interrogar a interface UPnP IGD.",
            Language::Russian => "Нажмите «Опросить роутер» для опроса интерфейса UPnP IGD.",
            Language::Urdu => "UPnP IGD انٹرفیس سے استفسار کرنے کے لیے 'روٹر سے پوچھیں' پر کلک کریں۔",
        }
    }

    pub fn upnp_badge_wan_risk(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "RIESGO WAN",
            Language::English => "WAN RISK",
            Language::Chinese => "WAN 风险",
            Language::Hindi => "WAN जोखिम",
            Language::Arabic => "خطر WAN",
            Language::French => "RISQUE WAN",
            Language::Bengali => "WAN ঝুঁকি",
            Language::Portuguese => "RISCO WAN",
            Language::Russian => "РИСК WAN",
            Language::Urdu => "WAN خطرہ",
        }
    }

    pub fn upnp_badge_active(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "ACTIVO",
            Language::English => "ACTIVE",
            Language::Chinese => "活跃",
            Language::Hindi => "सक्रिय",
            Language::Arabic => "نشط",
            Language::French => "ACTIF",
            Language::Bengali => "সক্রিয়",
            Language::Portuguese => "ATIVO",
            Language::Russian => "АКТИВЕН",
            Language::Urdu => "فعال",
        }
    }

    pub fn upnp_badge_inactive(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "INACTIVO",
            Language::English => "INACTIVE",
            Language::Chinese => "未激活",
            Language::Hindi => "निष्क्रिय",
            Language::Arabic => "غير نشط",
            Language::French => "INACTIF",
            Language::Bengali => "নিষ্ক্রিয়",
            Language::Portuguese => "INATIVO",
            Language::Russian => "НЕАКТИВЕН",
            Language::Urdu => "غیر فعال",
        }
    }

    pub fn upnp_public_ip_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "IP PÚBLICA (WAN)",
            Language::English => "PUBLIC IP (WAN)",
            Language::Chinese => "公网 IP (WAN)",
            Language::Hindi => "सार्वजनिक आईपी (WAN)",
            Language::Arabic => "IP العام (WAN)",
            Language::French => "IP PUBLIQUE (WAN)",
            Language::Bengali => "পাবলিক আইপি (WAN)",
            Language::Portuguese => "IP PÚBLICO (WAN)",
            Language::Russian => "ПУБЛИЧНЫЙ IP (WAN)",
            Language::Urdu => "پبلک آئی پی (WAN)",
        }
    }

    pub fn upnp_not_assigned(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "No asignada",
            Language::English => "Not assigned",
            Language::Chinese => "未分配",
            Language::Hindi => "अनिर्दिष्ट",
            Language::Arabic => "غير معين",
            Language::French => "Non assignée",
            Language::Bengali => "বরাদ্দ করা হয়নি",
            Language::Portuguese => "Não atribuído",
            Language::Russian => "Не назначен",
            Language::Urdu => "مقرر نہیں کیا گیا",
        }
    }

    pub fn upnp_status_connected(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Conectado",
            Language::English => "Connected",
            Language::Chinese => "已连接",
            Language::Hindi => "कनेक्टेड",
            Language::Arabic => "متصل",
            Language::French => "Connecté",
            Language::Bengali => "সংযুক্ত",
            Language::Portuguese => "Conectado",
            Language::Russian => "Подключено",
            Language::Urdu => "منسلک",
        }
    }

    pub fn upnp_status_label(status: &str, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Estado: {}", status),
            Language::English => format!("Status: {}", status),
            Language::Chinese => format!("状态: {}", status),
            Language::Hindi => format!("स्थिति: {}", status),
            Language::Arabic => format!("الحالة: {}", status),
            Language::French => format!("État: {}", status),
            Language::Bengali => format!("স্থিতি: {}", status),
            Language::Portuguese => format!("Status: {}", status),
            Language::Russian => format!("Статус: {}", status),
            Language::Urdu => format!("حالت: {}", status),
        }
    }

    pub fn upnp_uptime_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "TIEMPO ACTIVIDAD",
            Language::English => "UPTIME",
            Language::Chinese => "运行时间",
            Language::Hindi => "सक्रियता समय",
            Language::Arabic => "وقت التشغيل",
            Language::French => "TEMPS DE FONCTIONNEMENT",
            Language::Bengali => "সক্রিয় সময়",
            Language::Portuguese => "TEMPO DE ATIVIDADE",
            Language::Russian => "ВРЕМЯ РАБОТЫ",
            Language::Urdu => "چلنے کا وقت",
        }
    }

    pub fn upnp_uptime_sub(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Uptime Router",
            Language::English => "Router Uptime",
            Language::Chinese => "路由器运行时间",
            Language::Hindi => "राउटर अपटाइम",
            Language::Arabic => "مدة تشغيل الراوتر",
            Language::French => "Disponibilité Routeur",
            Language::Bengali => "রাউটার আপটাইম",
            Language::Portuguese => "Uptime do Roteador",
            Language::Russian => "Время работы роутера",
            Language::Urdu => "روٹر اپ ٹائم",
        }
    }

    pub fn upnp_traffic_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "TRÁFICO TOTAL WAN",
            Language::English => "TOTAL WAN TRAFFIC",
            Language::Chinese => "总广域网流量",
            Language::Hindi => "कुल वैन ट्रैफ़िक",
            Language::Arabic => "إجمالي حركة مرور WAN",
            Language::French => "TRAFIC TOTAL WAN",
            Language::Bengali => "মোট WAN ট্র্যাফিক",
            Language::Portuguese => "TRÁFEGO TOTAL WAN",
            Language::Russian => "ОБЩИЙ ТРАФИК WAN",
            Language::Urdu => "کل WAN ٹریفک",
        }
    }

    pub fn upnp_traffic_sub(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Acumulado interfaz",
            Language::English => "Interface cumulative",
            Language::Chinese => "接口累计",
            Language::Hindi => "इंटरफ़ेस संचयी",
            Language::Arabic => "التراكمي للواجهة",
            Language::French => "Cumul d'interface",
            Language::Bengali => "ইন্টারফেস সঞ্চিত",
            Language::Portuguese => "Acumulado da interface",
            Language::Russian => "Суммарно на интерфейсе",
            Language::Urdu => "انٹرفیس کا مجموعہ",
        }
    }

    pub fn upnp_port_mappings_header(count: usize, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Reglas de Reenvío de Puertos Activas ({})", count),
            Language::English => format!("Active Port Forwarding Rules ({})", count),
            Language::Chinese => format!("活动端口转发规则 ({})", count),
            Language::Hindi => format!("सक्रिय पोर्ट फ़ॉरवर्डिंग नियम ({})", count),
            Language::Arabic => format!("قواعد إعادة توجيه المنافذ النشطة ({})", count),
            Language::French => format!("Règles actives de redirection de ports ({})", count),
            Language::Bengali => format!("সক্রিয় পোর্ট ফরওয়ার্ডিং নিয়ম ({})", count),
            Language::Portuguese => format!("Regras ativas de encaminhamento de portas ({})", count),
            Language::Russian => format!("Активные правила проброса портов ({})", count),
            Language::Urdu => format!("فعال پورٹ فارورڈنگ کے قواعد ({})", count),
        }
    }

    pub fn upnp_no_mappings(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "No se detectaron puertos redirigidos mediante UPnP en el router.",
            Language::English => "No port forwarding rules detected via UPnP on the router.",
            Language::Chinese => "在路由器上未检测到通过 UPnP 转发的端口。",
            Language::Hindi => "राउटर पर UPnP के माध्यम से कोई अग्रेषित पोर्ट नहीं मिले।",
            Language::Arabic => "لم يتم رصد منافذ موجهة عبر UPnP على الراوتر.",
            Language::French => "Aucun port redirigé via UPnP détecté sur le routeur.",
            Language::Bengali => "রাউটারে UPnP এর মাধ্যমে কোনো ফরোয়ার্ড করা পোর্ট সনাক্ত করা যায়নি।",
            Language::Portuguese => "Nenhuma regra de portas encaminhadas por UPnP foi detectada no roteador.",
            Language::Russian => "На роутере не обнаружено перенаправленных портов через UPnP.",
            Language::Urdu => "روٹر پر UPnP کے ذریعے ری ڈائریکٹ شدہ پورٹس نہیں ملیں۔",
        }
    }

    pub fn upnp_port_label(port: u16, proto: &str, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("Puerto {} ({})", port, proto),
            Language::English => format!("Port {} ({})", port, proto),
            Language::Chinese => format!("端口 {} ({})", port, proto),
            Language::Hindi => format!("पोर्ट {} ({})", port, proto),
            Language::Arabic => format!("المنفذ {} ({})", port, proto),
            Language::French => format!("Port {} ({})", port, proto),
            Language::Bengali => format!("পোর্ট {} ({})", port, proto),
            Language::Portuguese => format!("Porta {} ({})", port, proto),
            Language::Russian => format!("Порт {} ({})", port, proto),
            Language::Urdu => format!("پورٹ {} ({})", port, proto),
        }
    }

    pub fn export_json_dialog_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Guardar reporte JSON de dispositivos",
            Language::English => "Save Device JSON Report",
            Language::Chinese => "保存设备 JSON 报告",
            Language::Hindi => "डिवाइस JSON रिपोर्ट सहेजें",
            Language::Arabic => "حفظ تقرير الأجهزة بصيغة JSON",
            Language::French => "Enregistrer le rapport JSON des appareils",
            Language::Bengali => "ডিভাইস JSON রিপোর্ট সংরক্ষণ করুন",
            Language::Portuguese => "Salvar relatório JSON de dispositivos",
            Language::Russian => "Сохранить отчет об устройствах в формате JSON",
            Language::Urdu => "ڈیوائس JSON رپورٹ محفوظ کریں",
        }
    }

    pub fn export_csv_dialog_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Guardar listado de dispositivos en CSV",
            Language::English => "Save Device List as CSV",
            Language::Chinese => "将设备列表保存为 CSV",
            Language::Hindi => "डिवाइस सूची को CSV के रूप में सहेजें",
            Language::Arabic => "حفظ قائمة الأجهزة بصيغة CSV",
            Language::French => "Enregistrer la liste des appareils en CSV",
            Language::Bengali => "ডিভাইসের তালিকা CSV হিসেবে সংরক্ষণ করুন",
            Language::Portuguese => "Salvar lista de dispositivos em CSV",
            Language::Russian => "Сохранить список устройств в формате CSV",
            Language::Urdu => "ڈیوائس کی فہرست CSV کے طور पर محفوظ کریں",
        }
    }

    pub fn export_success_msg(fname: &str, lang: Language) -> String {
        match lang {
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
        }
    }

    pub fn export_csv_success_msg(fname: &str, lang: Language) -> String {
        match lang {
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
        }
    }

    pub fn about_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "ℹ️ Acerca de",
            Language::English => "ℹ️ About",
            Language::Chinese => "ℹ️ 关于",
            Language::Hindi => "ℹ️ के बारे में",
            Language::Arabic => "ℹ️ حول",
            Language::French => "ℹ️ À propos",
            Language::Bengali => "ℹ️ সম্পর্কে",
            Language::Portuguese => "ℹ️ Sobre",
            Language::Russian => "ℹ️ О программе",
            Language::Urdu => "ℹ️ کے بارے میں",
        }
    }

    pub fn tip_about(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Acerca de NetLens y créditos del autor",
            Language::English => "About NetLens and author credits",
            Language::Chinese => "关于 NetLens 及作者信息",
            Language::Hindi => "NetLens और लेखक क्रेडिट के बारे में",
            Language::Arabic => "حول NetLens ومعلومات المطور",
            Language::French => "À propos de NetLens et crédits de l'auteur",
            Language::Bengali => "NetLens এবং লেখক কৃতিত্ব সম্পর্কে",
            Language::Portuguese => "Sobre o NetLens e créditos do autor",
            Language::Russian => "О NetLens и об авторе",
            Language::Urdu => "NetLens اور مصنف کے کریڈٹ کے بارے میں",
        }
    }

    pub fn about_modal_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Acerca de NetLens",
            Language::English => "About NetLens",
            Language::Chinese => "关于 NetLens",
            Language::Hindi => "NetLens के बारे में",
            Language::Arabic => "حول NetLens",
            Language::French => "À propos de NetLens",
            Language::Bengali => "NetLens সম্পর্কে",
            Language::Portuguese => "Sobre o NetLens",
            Language::Russian => "О программе NetLens",
            Language::Urdu => "NetLens کے بارے میں",
        }
    }

    pub fn about_app_description(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Monitor y explorador visual de red local de alto rendimiento, con análisis de espectro Wi-Fi, gestión de almacenamiento USB en router y auditoría de seguridad.",
            Language::English => "High-performance visual local network monitor and explorer, featuring Wi-Fi spectrum analysis, router USB storage management, and security auditing.",
            Language::Chinese => "高性能可视化局域网监控与探测器，配备 Wi-Fi 频谱分析、路由器 USB 存储管理与网络安全审计。",
            Language::Hindi => "उच्च-प्रदर्शन दृश्य स्थानीय नेटवर्क मॉनिटर और एक्सप्लोरर, वाई-फाई स्पेक्ट्रम विश्लेषण, राउटर यूएसबी स्टोरेज प्रबंधन और सुरक्षा ऑडिटिंग के साथ।",
            Language::Arabic => "مراقب واستكشاف مرئي عالي الأداء للشبكة المحلية، يتميز بتحليل طيف الواي فاي وإدارة وحدة تخزين راوتر USB والتدقيق الأمني.",
            Language::French => "Moniteur et explorateur visuel de réseau local haute performance, avec analyse spectrale Wi-Fi, gestion du stockage USB du routeur et audit de sécurité.",
            Language::Bengali => "উচ্চ-কার্যক্ষমতাসম্পন্ন ভিজ্যুয়াল লোকাল নেটওয়ার্ক মনিটর এবং এক্সপ্লোরার, সাথে ওয়াই-ফাই স্পেকট্রাম বিশ্লেষণ, রাউটার ইউএসবি স্টোরেজ ব্যবস্থাপনা এবং নিরাপত্তা অডিট।",
            Language::Portuguese => "Monitor e explorador visual de rede local de alto desempenho, com análise de espectro Wi-Fi, gerenciamento de armazenamento USB do roteador e auditoria de segurança.",
            Language::Russian => "Высокопроизводительный визуальный монитор и сканер локальной сети с анализом спектра Wi-Fi, управлением USB-накопителем роутера и аудитом безопасности.",
            Language::Urdu => "اعلی کارکردگی کا حامل بصری لوکل نیٹ ورک مانیٹر اور ایکسپلورر، وائی فائی اسپیکٹرم تجزیہ، راؤٹر یو ایس بی اسٹوریج مینجمنٹ اور سیکیورٹی آڈٹ کے ساتھ۔",
        }
    }

    pub fn about_author_section(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Autor y Desarrollador",
            Language::English => "Author & Lead Developer",
            Language::Chinese => "作者与主开发者",
            Language::Hindi => "लेखक और मुख्य डेवलपर",
            Language::Arabic => "المؤلف والمطور الرئيسي",
            Language::French => "Auteur et Développeur Principal",
            Language::Bengali => "লেখক এবং প্রধান বিকাশকারী",
            Language::Portuguese => "Autor e Desenvolvedor Principal",
            Language::Russian => "Автор и главный разработчик",
            Language::Urdu => "مصنف اور اہم ڈویلپر",
        }
    }

    pub fn about_role(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Ingeniero de Software y Arquitectura de Sistemas",
            Language::English => "Software Engineer & Systems Architect",
            Language::Chinese => "软件工程师与系统架构师",
            Language::Hindi => "सॉफ्टवेयर इंजीनियर और सिस्टम्स आर्किटेक्ट",
            Language::Arabic => "مهندس برمجيات ومهندس أنظمة",
            Language::French => "Ingénieur Logiciel et Architecte Systèmes",
            Language::Bengali => "সফটওয়্যার ইঞ্জিনিয়ার এবং সিস্টেম আর্কিটেক্ট",
            Language::Portuguese => "Engenheiro de Software e Arquiteto de Sistemas",
            Language::Russian => "Инженер-программист и системный архитектор",
            Language::Urdu => "سافٹ ویئر انجینئر اور سسٹمز آرکیٹیکٹ",
        }
    }

    pub fn about_email_label(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Correo de Contacto",
            Language::English => "Contact Email",
            Language::Chinese => "联系邮箱",
            Language::Hindi => "संपर्क ईमेल",
            Language::Arabic => "البريد الإلكتروني للتواصل",
            Language::French => "E-mail de Contact",
            Language::Bengali => "যোগাযোগের ইমেল",
            Language::Portuguese => "E-mail de Contato",
            Language::Russian => "Контактный e-mail",
            Language::Urdu => "رابطہ ای میل",
        }
    }

    pub fn about_copy_email(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Copiar Correo",
            Language::English => "Copy Email",
            Language::Chinese => "复制邮箱",
            Language::Hindi => "ईमेल कॉपी करें",
            Language::Arabic => "نسخ البريد",
            Language::French => "Copier l'e-mail",
            Language::Bengali => "ইমেল কপি করুন",
            Language::Portuguese => "Copiar E-mail",
            Language::Russian => "Скопировать email",
            Language::Urdu => "ای میل کاپی کریں",
        }
    }

    pub fn about_send_email(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Enviar Correo",
            Language::English => "Send Email",
            Language::Chinese => "发送邮件",
            Language::Hindi => "ईमेल भेजें",
            Language::Arabic => "إرسال بريد",
            Language::French => "Envoyer un e-mail",
            Language::Bengali => "ইমেল পাঠান",
            Language::Portuguese => "Enviar E-mail",
            Language::Russian => "Написать письмо",
            Language::Urdu => "ای میل بھیجیں",
        }
    }

    pub fn about_repositories_section(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Repositorios Oficiales",
            Language::English => "Official Repositories",
            Language::Chinese => "官方代码仓库",
            Language::Hindi => "आधिकारिक रिपॉजिटरी",
            Language::Arabic => "المستودعات الرسمية",
            Language::French => "Dépôts Officiels",
            Language::Bengali => "অফিসিয়াল রিপোজিটরি",
            Language::Portuguese => "Repositórios Oficiais",
            Language::Russian => "Официальные репозитории",
            Language::Urdu => "سرکاری ریپوزٹریز",
        }
    }

    pub fn about_source_code(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Código Fuente (NetLens)",
            Language::English => "Source Code (NetLens)",
            Language::Chinese => "源代码 (NetLens)",
            Language::Hindi => "स्रोत कोड (NetLens)",
            Language::Arabic => "شفرة المصدر (NetLens)",
            Language::French => "Code Source (NetLens)",
            Language::Bengali => "উৎস কোড (NetLens)",
            Language::Portuguese => "Código-Fonte (NetLens)",
            Language::Russian => "Исходный код (NetLens)",
            Language::Urdu => "ماخذ کوڈ (NetLens)",
        }
    }

    pub fn about_pacman_repo(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Repositorio Pacman (CachyOS / Arch)",
            Language::English => "Pacman Repository (CachyOS / Arch)",
            Language::Chinese => "Pacman 软件源 (CachyOS / Arch)",
            Language::Hindi => "पैकमैन रिपॉजिटरी (CachyOS / Arch)",
            Language::Arabic => "مستودع Pacman (CachyOS / Arch)",
            Language::French => "Dépôt Pacman (CachyOS / Arch)",
            Language::Bengali => "প্যাকম্যান রিপোজিটরি (CachyOS / Arch)",
            Language::Portuguese => "Repositório Pacman (CachyOS / Arch)",
            Language::Russian => "Репозиторий Pacman (CachyOS / Arch)",
            Language::Urdu => "پیک مین ریپوزٹری (CachyOS / Arch)",
        }
    }

    pub fn about_web_portal(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Portal Web e Instalador",
            Language::English => "Web Portal & Installer",
            Language::Chinese => "网页门户与安装器",
            Language::Hindi => "वेब पोर्टल और इंस्टॉलर",
            Language::Arabic => "بوابة الويب والمثبت",
            Language::French => "Portail Web & Installateur",
            Language::Bengali => "ওয়েব পোর্টাল এবং ইনস্টলার",
            Language::Portuguese => "Portal Web e Instalador",
            Language::Russian => "Веб-портал и установщик",
            Language::Urdu => "ویب پورٹل اور انسٹالر",
        }
    }

    pub fn about_tech_specs(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Especificaciones Técnicas",
            Language::English => "Technical Specifications",
            Language::Chinese => "技术规格",
            Language::Hindi => "तकनीकी विवरण",
            Language::Arabic => "المواصفات التقنية",
            Language::French => "Spécifications Techniques",
            Language::Bengali => "প্রযুক্তিগত বৈশিষ্ট্য",
            Language::Portuguese => "Especificações Técnicas",
            Language::Russian => "Технические спецификации",
            Language::Urdu => "تکنیکی وضاحتیں",
        }
    }

    pub fn about_license(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Licencia MIT de Código Abierto",
            Language::English => "MIT Open Source License",
            Language::Chinese => "MIT 开源许可证",
            Language::Hindi => "एमआईटी ओपन सोर्स लाइसेंस",
            Language::Arabic => "ترخيص MIT مفتوح المصدر",
            Language::French => "Licence Open Source MIT",
            Language::Bengali => "এমআইটি ওপেন সোর্স লাইসেন্স",
            Language::Portuguese => "Licença de Código Aberto MIT",
            Language::Russian => "Открытая лицензия MIT",
            Language::Urdu => "MIT اوپن سورس لائسنس",
        }
    }

    pub fn about_open_in_browser(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Abrir",
            Language::English => "Open",
            Language::Chinese => "打开",
            Language::Hindi => "खोलें",
            Language::Arabic => "فتح",
            Language::French => "Ouvrir",
            Language::Bengali => "খুলুন",
            Language::Portuguese => "Abrir",
            Language::Russian => "Открыть",
            Language::Urdu => "کھولیں",
        }
    }

    pub fn about_copy_url(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Copiar",
            Language::English => "Copy",
            Language::Chinese => "复制",
            Language::Hindi => "कॉपी",
            Language::Arabic => "نسخ",
            Language::French => "Copier",
            Language::Bengali => "কপি",
            Language::Portuguese => "Copiar",
            Language::Russian => "Копировать",
            Language::Urdu => "کاپی",
        }
    }

    // System Tray & Context Menu
    pub fn tray_tooltip_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "NetLens - Monitor de Red Local",
            Language::English => "NetLens - Local Network Monitor",
            Language::Chinese => "NetLens - 本地局域网监控器",
            Language::Hindi => "NetLens - स्थानीय नेटवर्क मॉनिटर",
            Language::Arabic => "NetLens - مراقب الشبكة المحلية",
            Language::French => "NetLens - Moniteur de Réseau Local",
            Language::Bengali => "NetLens - লোকাল নেটওয়ার্ক মনিটর",
            Language::Portuguese => "NetLens - Monitor de Rede Local",
            Language::Russian => "NetLens - Монитор локальной сети",
            Language::Urdu => "NetLens - مقامی نیٹ ورک مانیٹر",
        }
    }

    pub fn tray_tooltip_scanning(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Escaneando red local...",
            Language::English => "Scanning local network...",
            Language::Chinese => "正在扫描局域网...",
            Language::Hindi => "स्थानीय नेटवर्क स्कैन हो रहा है...",
            Language::Arabic => "جاري فحص الشبكة المحلية...",
            Language::French => "Scan du réseau local en cours...",
            Language::Bengali => "লোকাল নেটওয়ার্ক স্ক্যান হচ্ছে...",
            Language::Portuguese => "Escaneando rede local...",
            Language::Russian => "Сканирование локальной сети...",
            Language::Urdu => "مقامی نیٹ ورک اسکین ہو رہا ہے...",
        }
    }

    pub fn tray_tooltip_with_gw(count: usize, gw: &str, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("{} dispositivos activos\nPuerta de enlace: {}", count, gw),
            Language::English => format!("{} active devices\nGateway: {}", count, gw),
            Language::Chinese => format!("{} 个在线设备\n网关: {}", count, gw),
            Language::Hindi => format!("{} सक्रिय डिवाइस\nगेटवे: {}", count, gw),
            Language::Arabic => format!("{} جهاز نشط\nالبوابة: {}", count, gw),
            Language::French => format!("{} appareils actifs\nPasserelle : {}", count, gw),
            Language::Bengali => format!("{}টি সক্রিয় ডিভাইস\nগেটওয়ে: {}", count, gw),
            Language::Portuguese => format!("{} dispositivos ativos\nGateway: {}", count, gw),
            Language::Russian => format!("{} активных устройств\nШлюз: {}", count, gw),
            Language::Urdu => format!("{} فعال آلات\nگیٹ وے: {}", count, gw),
        }
    }

    pub fn tray_tooltip_devices(count: usize, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("{} dispositivos activos", count),
            Language::English => format!("{} active devices", count),
            Language::Chinese => format!("{} 个在线设备", count),
            Language::Hindi => format!("{} सक्रिय डिवाइस", count),
            Language::Arabic => format!("{} جهاز نشط", count),
            Language::French => format!("{} appareils actifs", count),
            Language::Bengali => format!("{}টি সক্রিয় ডিভাইস", count),
            Language::Portuguese => format!("{} dispositivos ativos", count),
            Language::Russian => format!("{} активных устройств", count),
            Language::Urdu => format!("{} فعال آلات", count),
        }
    }

    pub fn tray_menu_open(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "👁️ Abrir NetLens",
            Language::English => "👁️ Open NetLens",
            Language::Chinese => "👁️ 打开 NetLens",
            Language::Hindi => "👁️ NetLens खोलें",
            Language::Arabic => "👁️ فتح NetLens",
            Language::French => "👁️ Ouvrir NetLens",
            Language::Bengali => "👁️ NetLens খুলুন",
            Language::Portuguese => "👁️ Abrir NetLens",
            Language::Russian => "👁️ Открыть NetLens",
            Language::Urdu => "👁️ NetLens کھولیں",
        }
    }

    pub fn tray_menu_scan_now(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⚡ Escanear Red Ahora",
            Language::English => "⚡ Scan Network Now",
            Language::Chinese => "⚡ 立即扫描网络",
            Language::Hindi => "⚡ अभी नेटवर्क स्कैन करें",
            Language::Arabic => "⚡ فحص الشبكة الآن",
            Language::French => "⚡ Scanner le réseau maintenant",
            Language::Bengali => "⚡ এখনই নেটওয়ার্ক স্ক্যান করুন",
            Language::Portuguese => "⚡ Escanear Rede Agora",
            Language::Russian => "⚡ Сканировать сеть сейчас",
            Language::Urdu => "⚡ ابھی نیٹ ورک اسکین کریں",
        }
    }

    pub fn tray_menu_scanning(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "⏳ Escaneando red...",
            Language::English => "⏳ Scanning network...",
            Language::Chinese => "⏳ 正在扫描网络...",
            Language::Hindi => "⏳ नेटवर्क स्कैनिंग जारी है...",
            Language::Arabic => "⏳ جاري فحص الشبكة...",
            Language::French => "⏳ Scan du réseau en cours...",
            Language::Bengali => "⏳ নেটওয়ার্ক স্ক্যান হচ্ছে...",
            Language::Portuguese => "⏳ Escaneando rede...",
            Language::Russian => "⏳ Сканирование сети...",
            Language::Urdu => "⏳ نیٹ ورک اسکیننگ جاری ہے...",
        }
    }

    pub fn tray_menu_devices(count: usize, lang: Language) -> String {
        match lang {
            Language::Spanish => format!("📊 Dispositivos activos: {}", count),
            Language::English => format!("📊 Active devices: {}", count),
            Language::Chinese => format!("📊 在线设备数: {}", count),
            Language::Hindi => format!("📊 सक्रिय डिवाइस: {}", count),
            Language::Arabic => format!("📊 الأجهزة النشطة: {}", count),
            Language::French => format!("📊 Appareils actifs : {}", count),
            Language::Bengali => format!("📊 সক্রিয় ডিভাইস: {}", count),
            Language::Portuguese => format!("📊 Dispositivos ativos: {}", count),
            Language::Russian => format!("📊 Активных устройств: {}", count),
            Language::Urdu => format!("📊 فعال آلات: {}", count),
        }
    }

    pub fn tray_menu_exit(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🚪 Salir de NetLens",
            Language::English => "🚪 Quit NetLens",
            Language::Chinese => "🚪 退出 NetLens",
            Language::Hindi => "🚪 NetLens से बाहर निकलें",
            Language::Arabic => "🚪 خروج من NetLens",
            Language::French => "🚪 Quitter NetLens",
            Language::Bengali => "🚪 NetLens প্রস্থান করুন",
            Language::Portuguese => "🚪 Sair do NetLens",
            Language::Russian => "🚪 Выйти из NetLens",
            Language::Urdu => "🚪 NetLens بند کریں",
        }
    }

    // Version Check & Updates
    pub fn update_latest_version(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Estás usando la última versión",
            Language::English => "You are using the latest version",
            Language::Chinese => "您正在使用最新版本",
            Language::Hindi => "आप नवीनतम संस्करण का उपयोग कर रहे हैं",
            Language::Arabic => "أنت تستخدم أحدث إصدار",
            Language::French => "Vous utilisez la dernière version",
            Language::Bengali => "আপনি সর্বশেষ সংস্করণটি ব্যবহার করছেন",
            Language::Portuguese => "Você está usando a versão mais recente",
            Language::Russian => "Вы используете последнюю версию",
            Language::Urdu => "آپ تازہ ترین ورژن استعمال کر رہے ہیں",
        }
    }

    pub fn update_available(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Actualización disponible",
            Language::English => "Update available",
            Language::Chinese => "发现新版本",
            Language::Hindi => "अपडेट उपलब्ध है",
            Language::Arabic => "تحديث متاح",
            Language::French => "Mise à jour disponible",
            Language::Bengali => "আপডেট উপলব্ধ",
            Language::Portuguese => "Atualização disponível",
            Language::Russian => "Доступно обновление",
            Language::Urdu => "اپ ڈیٹ دستیاب ہے",
        }
    }

    pub fn update_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Actualizar",
            Language::English => "Update",
            Language::Chinese => "更新",
            Language::Hindi => "अपडेट करें",
            Language::Arabic => "تحديث",
            Language::French => "Mettre à jour",
            Language::Bengali => "আপডেট করুন",
            Language::Portuguese => "Atualizar",
            Language::Russian => "Обновить",
            Language::Urdu => "اپ ڈیٹ کریں",
        }
    }

    pub fn check_for_updates(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Comprobar actualizaciones",
            Language::English => "Check for updates",
            Language::Chinese => "检查更新",
            Language::Hindi => "अपडेट की जाँच करें",
            Language::Arabic => "التحقق من التحديثات",
            Language::French => "Vérifier les mises à jour",
            Language::Bengali => "আপডেট পরীক্ষা করুন",
            Language::Portuguese => "Verificar atualizações",
            Language::Russian => "Проверить обновления",
            Language::Urdu => "اپ ڈیٹس چیک کریں",
        }
    }

    pub fn checking_updates(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Comprobando actualizaciones...",
            Language::English => "Checking for updates...",
            Language::Chinese => "正在检查更新...",
            Language::Hindi => "अपडेट जाँचे जा रहे हैं...",
            Language::Arabic => "جاري التحقق من التحديثات...",
            Language::French => "Vérification des mises à jour...",
            Language::Bengali => "আপডেট পরীক্ষা করা হচ্ছে...",
            Language::Portuguese => "Verificando atualizações...",
            Language::Russian => "Проверка обновлений...",
            Language::Urdu => "اپ ڈیٹس چیک ہو رہی ہیں...",
        }
    }

    pub fn update_modal_title(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Nueva Actualización Disponible",
            Language::English => "New Update Available",
            Language::Chinese => "发现新版本可用",
            Language::Hindi => "नया अपडेट उपलब्ध है",
            Language::Arabic => "يتوفر إصدار جديد",
            Language::French => "Nouvelle version disponible",
            Language::Bengali => "নতুন আপডেট উপলব্ধ",
            Language::Portuguese => "Nova Versão Disponível",
            Language::Russian => "Доступна новая версия",
            Language::Urdu => "نیا اپ ڈیٹ دستیاب ہے",
        }
    }

    pub fn update_instructions(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Para actualizar NetLens a la última versión, ejecuta el siguiente comando en tu terminal:",
            Language::English => "To update NetLens to the latest version, run the following command in your terminal:",
            Language::Chinese => "要将 NetLens 更新至最新版本，请在终端中运行以下命令：",
            Language::Hindi => "NetLens को नवीनतम संस्करण में अपडेट करने के लिए, अपने टर्मिनल में यह कमांड चलाएं:",
            Language::Arabic => "لتحديث NetLens إلى أحدث إصدار، قم بتشغيل الأمر التالي في سطر الأوامر:",
            Language::French => "Pour mettre à jour NetLens vers la dernière version, exécutez la commande suivante dans votre terminal :",
            Language::Bengali => "NetLens সর্বশেষ সংস্করণে আপডেট করতে, আপনার টার্মিনালে নিম্নলিখিত কমান্ডটি চালান:",
            Language::Portuguese => "Para atualizar o NetLens para a versão mais recente, execute o seguinte comando no terminal:",
            Language::Russian => "Чтобы обновить NetLens до последней версии, выполните следующую команду в терминале:",
            Language::Urdu => "NetLens کو تازہ ترین ورژن میں اپ ڈیٹ کرنے کے لیے، اپنے ٹرمینل میں درج ذیل کمانڈ چلائیں:",
        }
    }

    pub fn update_copy_command(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Copiar Comando",
            Language::English => "Copy Command",
            Language::Chinese => "复制命令",
            Language::Hindi => "कमांड कॉपी करें",
            Language::Arabic => "نسخ الأمر",
            Language::French => "Copier la commande",
            Language::Bengali => "কমান্ড কপি করুন",
            Language::Portuguese => "Copiar Comando",
            Language::Russian => "Скопировать команду",
            Language::Urdu => "کمانڈ کاپی کریں",
        }
    }

    pub fn update_copied_notif(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Comando de actualización copiado al portapapeles.",
            Language::English => "Update command copied to clipboard.",
            Language::Chinese => "更新命令已复制到剪贴板。",
            Language::Hindi => "अपडेट कमांड क्लिपबोर्ड पर कॉपी किया गया।",
            Language::Arabic => "تم نسخ أمر التحديث إلى الحافظة.",
            Language::French => "Commande de mise à jour copiée dans le presse-papiers.",
            Language::Bengali => "আপডেট কমান্ড ক্লিপবোর্ডে কপি করা হয়েছে।",
            Language::Portuguese => "Comando de actualización copiado para a área de transferência.",
            Language::Russian => "Команда обновления скопирована в буфер обмена.",
            Language::Urdu => "اپ ڈیٹ کمانڈ کلپ بورڈ پر کاپی ہو گئی۔",
        }
    }

    pub fn update_now_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🚀 Actualizar Ahora (Automático)",
            Language::English => "🚀 Update Now (Automatic)",
            Language::Chinese => "🚀 立即更新 (自动)",
            Language::Hindi => "🚀 अभी अपडेट करें (स्वचालित)",
            Language::Arabic => "🚀 التحديث الآن (تلقائي)",
            Language::French => "🚀 Mettre à jour maintenant (Automatique)",
            Language::Bengali => "🚀 এখনই আপডেট করুন (স্বয়ংক্রিয়)",
            Language::Portuguese => "🚀 Atualizar Agora (Automático)",
            Language::Russian => "🚀 Обновить сейчас (Автоматически)",
            Language::Urdu => "🚀 ابھی اپ ڈیٹ کریں (خودکار)",
        }
    }

    pub fn update_in_terminal_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "💻 Abrir en Terminal",
            Language::English => "💻 Open in Terminal",
            Language::Chinese => "💻 在终端中打开",
            Language::Hindi => "💻 टर्मिनल में खोलें",
            Language::Arabic => "💻 فتح في الطرفية",
            Language::French => "💻 Ouvrir dans le terminal",
            Language::Bengali => "💻 টার্মিনালে খুলুন",
            Language::Portuguese => "💻 Abrir no Terminal",
            Language::Russian => "💻 Открыть в терминале",
            Language::Urdu => "💻 ٹرمینل میں کھولیں",
        }
    }

    pub fn update_authorizing_and_installing(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Solicitando autorización e instalando actualización en segundo plano...",
            Language::English => "Requesting authorization and installing update in background...",
            Language::Chinese => "正在请求授权并在后台安装更新...",
            Language::Hindi => "प्राधिकरण का अनुरोध किया जा रहा है और पृष्ठभूमि में अपडेट इंस्टॉल हो रहा है...",
            Language::Arabic => "جاري طلب المصادقة وتثبيت التحديث في الخلفية...",
            Language::French => "Demande d'autorisation et installation de la mise à jour en arrière-plan...",
            Language::Bengali => "অনুমোদন অনুরোধ করা হচ্ছে এবং পটভূমিতে আপডেট ইনস্টল করা হচ্ছে...",
            Language::Portuguese => "Solicitando autorização e instalando atualização em segundo plano...",
            Language::Russian => "Запрос авторизации и установка обновления в фоновом режиме...",
            Language::Urdu => "اجازت کی درخواست کی جا رہی ہے اور پس منظر میں اپ ڈیٹ انسٹال ہو رہی ہے...",
        }
    }

    pub fn update_success_banner(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "✅ ¡Actualización completada con éxito!",
            Language::English => "✅ Update completed successfully!",
            Language::Chinese => "✅ 更新已成功完成！",
            Language::Hindi => "✅ अपडेट सफलतापूर्वक पूरा हुआ!",
            Language::Arabic => "✅ اكتمل التحديث بنجاح!",
            Language::French => "✅ Mise à jour terminée avec succès !",
            Language::Bengali => "✅ আপডেট সফলভাবে সম্পন্ন হয়েছে!",
            Language::Portuguese => "✅ Atualização concluída com sucesso!",
            Language::Russian => "✅ Обновление успешно завершено!",
            Language::Urdu => "✅ اپ ڈیٹ کامیابی سے مکمل ہو گئی!",
        }
    }

    pub fn update_restart_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "🔄 Reiniciar NetLens Ahora",
            Language::English => "🔄 Restart NetLens Now",
            Language::Chinese => "🔄 立即重启 NetLens",
            Language::Hindi => "🔄 NetLens को अभी पुनः प्रारंभ करें",
            Language::Arabic => "🔄 إعادة تشغيل NetLens الآن",
            Language::French => "🔄 Redémarrer NetLens maintenant",
            Language::Bengali => "🔄 NetLens এখনই রিস্টার্ট করুন",
            Language::Portuguese => "🔄 Reiniciar NetLens Agora",
            Language::Russian => "🔄 Перезапустить NetLens сейчас",
            Language::Urdu => "🔄 NetLens ابھی دوبارہ شروع کریں",
        }
    }

    pub fn update_failed_banner(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Error en la actualización:",
            Language::English => "Update failed:",
            Language::Chinese => "更新失败：",
            Language::Hindi => "अपडेट विफल रहा:",
            Language::Arabic => "فشل التحديث:",
            Language::French => "Échec de la mise à jour :",
            Language::Bengali => "আপডেট ব্যর্থ হয়েছে:",
            Language::Portuguese => "Falha na atualização:",
            Language::Russian => "Ошибка обновления:",
            Language::Urdu => "اپ ڈیٹ ناکام ہو گئی:",
        }
    }

    pub fn update_retry_btn(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Reintentar",
            Language::English => "Retry",
            Language::Chinese => "重试",
            Language::Hindi => "पुनः प्रयास करें",
            Language::Arabic => "إعادة المحاولة",
            Language::French => "Réessayer",
            Language::Bengali => "পুনরায় চেষ্টা করুন",
            Language::Portuguese => "Tentar novamente",
            Language::Russian => "Повторить",
            Language::Urdu => "دوبارہ کوشش کریں",
        }
    }
}

fn extract_target_prefix<'a>(original: &'a str, patterns: &[&str]) -> &'a str {
    for pat in patterns {
        if let Some(idx) = original.find(pat) {
            return original[..idx].trim();
        }
    }
    original.trim_end_matches('.').trim()
}
