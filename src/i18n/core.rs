use super::{Language, Translations};
use crate::model::DeviceCategory;

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
            Language::Spanish => "Escaneando...",
            Language::English => "Scanning...",
            Language::Chinese => "正在扫描...",
            Language::Hindi => "स्कैनिंग जारी है...",
            Language::Arabic => "جاري الفحص...",
            Language::French => "Scan en cours...",
            Language::Bengali => "স্ক্যান হচ্ছে...",
            Language::Portuguese => "Escaneando...",
            Language::Russian => "Сканирование...",
            Language::Urdu => "اسکیننگ جاری ہے...",
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

}
