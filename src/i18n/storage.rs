use super::{Language, Translations};
use crate::router::storage::StorageFileType;

impl Translations {
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

    pub fn storage_drag_drop_prompt(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "📥 Suelta los archivos o carpetas aquí para subirlos",
            Language::English => "📥 Drop files or folders here to upload",
            Language::Chinese => "📥 将文件或文件夹拖放到此处即可上传",
            Language::Hindi => "📥 अपलोड करने के लिए फ़ाइलें या फ़ोल्डर यहाँ छोड़ें",
            Language::Arabic => "📥 أسقط الملفات أو المجلدات هنا لتحميلها",
            Language::French => "📥 Déposez les fichiers ou dossiers ici pour téléverser",
            Language::Bengali => "📥 আপলোড করতে ফাইল বা ফোল্ডার এখানে ড্রপ করুন",
            Language::Portuguese => "📥 Solte os arquivos ou pastas aqui para enviar",
            Language::Russian => "📥 Перетащите файлы или папки сюда для загрузки",
            Language::Urdu => "📥 اپ لوڈ کرنے کے لیے فائلیں یا فولڈرز یہاں ڈراپ کریں",
        }
    }

    pub fn storage_drag_drop_hint(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Arrastra archivos o carpetas desde tu explorador directamente sobre esta área",
            Language::English => "Drag files or folders directly from your file manager onto this area",
            Language::Chinese => "直接从文件管理器拖动文件或文件夹到此区域",
            Language::Hindi => "फ़ाइल प्रबंधक से फ़ाइलें या फ़ोल्डर सीधे इस क्षेत्र में खींचें",
            Language::Arabic => "اسحب الملفات أو المجلدات من مدير الملفات مباشرة إلى هذه المنطقة",
            Language::French => "Glissez des fichiers ou dossiers depuis votre gestionnaire directement sur cette zone",
            Language::Bengali => "আপনার ফাইল ম্যানেজার থেকে ফাইল বা ফোল্ডার সরাসরি এই অংশে টেনে আনুন",
            Language::Portuguese => "Arraste arquivos ou pastas do gerenciador de arquivos diretamente para esta área",
            Language::Russian => "Перетаскивайте файлы или папки из файлового менеджера прямо в эту область",
            Language::Urdu => "اپنے فائل مینیجر سے فائلیں یا فولڈرز براہ راست اس علاقے پر گھسیٹیں",
        }
    }

    pub fn storage_drag_drop_select_share(lang: Language) -> &'static str {
        match lang {
            Language::Spanish => "Selecciona un recurso compartido antes de subir archivos mediante arrastrar y soltar",
            Language::English => "Select a shared resource before uploading files via drag and drop",
            Language::Chinese => "在通过拖放上传文件之前，请先选择共享资源",
            Language::Hindi => "ड्रैग और ड्रॉप द्वारा फ़ाइलें अपलोड करने से पहले एक साझा संसाधन चुनें",
            Language::Arabic => "حدد مورداً مشتركاً قبل تحميل الملفات عبر السحب والإفلات",
            Language::French => "Sélectionnez un partage réseau avant de téléverser des fichiers par glisser-déposer",
            Language::Bengali => "ড্র্যাগ এবং ড্রপের মাধ্যমে ফাইল আপলোড করার আগে একটি শেয়ার নির্বাচন করুন",
            Language::Portuguese => "Selecione um compartilhamento antes de enviar arquivos via arrastar e soltar",
            Language::Russian => "Выберите общий ресурс перед загрузкой файлов методом перетаскивания",
            Language::Urdu => "ڈریگ اور ڈراپ کے ذریعے فائلیں اپ لوڈ کرنے سے پہلے ایک شیئر منتخب کریں",
        }
    }

}
