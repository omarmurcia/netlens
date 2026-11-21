use super::{Language, Translations};
use crate::model::DeviceCategory;

impl Translations {
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

}
