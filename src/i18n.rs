use crate::model::DeviceCategory;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Los 10 idiomas más hablados del mundo
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
}
