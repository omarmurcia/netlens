use super::{Language, Translations};

impl Translations {
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

}
