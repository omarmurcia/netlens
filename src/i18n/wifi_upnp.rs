use super::{Language, Translations};

impl Translations {
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

}
