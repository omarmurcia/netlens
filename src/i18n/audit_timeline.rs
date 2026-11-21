use super::{extract_target_prefix, Language, Translations};
use crate::history::NetworkEventType;
use crate::security_audit::SecurityCheckId;

impl Translations {
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

}
