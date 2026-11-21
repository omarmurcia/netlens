use super::{Language, Translations};

impl Translations {
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
