use super::{Language, Translations};
use crate::model::{Device, DeviceStatus};

impl Translations {
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

}
