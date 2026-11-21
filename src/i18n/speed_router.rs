use super::{Language, Translations};

impl Translations {
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

}
