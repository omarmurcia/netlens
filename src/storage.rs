use crate::i18n::Language;
use crate::model::{Device, DeviceCategory};
use crate::notification::NotificationSettings;
use crate::speedtest::SpeedTestResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceMetadata {
    pub custom_alias: Option<String>,
    pub category: Option<DeviceCategory>,
    #[serde(default)]
    pub is_whitelisted: bool,
    #[serde(default)]
    pub is_blocked: bool,
    #[serde(default)]
    pub speed_limit_kbps: Option<u32>,
    #[serde(default)]
    pub request_history: Vec<crate::model::DeviceRequestEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub language: Language,
    pub theme: String,
    pub devices: Vec<Device>,
    pub device_metadata: HashMap<String, DeviceMetadata>,
    pub last_speed_test: Option<SpeedTestResult>,
    #[serde(default)]
    pub speed_test_history: Vec<SpeedTestResult>,
    #[serde(default)]
    pub router_ip: Option<String>,
    #[serde(default)]
    pub router_password: Option<String>,
    #[serde(default)]
    pub auto_scan_interval_secs: u32,
    #[serde(default)]
    pub known_macs: Vec<String>,
    #[serde(default)]
    pub autostart: bool,
    #[serde(default)]
    pub minimize_to_tray_on_close: bool,
    #[serde(default)]
    pub network_events: Vec<crate::history::NetworkEvent>,
    #[serde(default)]
    pub notification_settings: NotificationSettings,
    #[serde(default)]
    pub storage_credentials: Option<crate::router::StorageCredentials>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            language: Language::Spanish,
            theme: "TokyoNight".to_string(),
            devices: Vec::new(),
            device_metadata: HashMap::new(),
            last_speed_test: None,
            speed_test_history: Vec::new(),
            router_ip: None,
            router_password: None,
            auto_scan_interval_secs: 0,
            known_macs: Vec::new(),
            autostart: false,
            minimize_to_tray_on_close: false,
            network_events: Vec::new(),
            notification_settings: NotificationSettings::default(),
            storage_credentials: None,
        }
    }
}

pub struct Storage;

impl Storage {
    /// Ruta del archivo de configuración persistente según XDG (~/.config/netlens/state.json)
    pub fn get_storage_path() -> PathBuf {
        let base_dir = if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg_config)
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config")
        } else {
            PathBuf::from(".")
        };

        base_dir.join("netlens").join("state.json")
    }

    /// Carga el estado guardado desde el disco. Si no existe o falla, retorna valores por defecto.
    pub fn load() -> AppState {
        let path = Self::get_storage_path();
        if !path.exists() {
            return AppState::default();
        }

        match fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<AppState>(&content) {
                Ok(mut state) => {
                    // Reaplicar metadatos (alias, categorías, listas blancas/negras) a los dispositivos
                    Self::apply_metadata(&mut state.devices, &state.device_metadata);
                    state
                }
                Err(err) => {
                    eprintln!("Aviso: No se pudo deserializar el estado guardado: {}", err);
                    AppState::default()
                }
            },
            Err(err) => {
                eprintln!("Aviso: No se pudo leer el archivo de estado {:?}: {}", path, err);
                AppState::default()
            }
        }
    }

    /// Guarda el estado actual en el disco de manera segura
    pub fn save(state: &AppState) -> Result<(), String> {
        let path = Self::get_storage_path();
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                let _ = fs::create_dir_all(parent);
            }
        }

        let json = serde_json::to_string_pretty(state)
            .map_err(|e| format!("Error serializando estado a JSON: {}", e))?;

        fs::write(&path, json)
            .map_err(|e| format!("Error guardando estado en {:?}: {}", path, e))?;

        Ok(())
    }

    /// Aplica alias, categorías y políticas de seguridad (whitelist/blacklist) a dispositivos por su MAC
    pub fn apply_metadata(devices: &mut [Device], metadata: &HashMap<String, DeviceMetadata>) {
        for dev in devices.iter_mut() {
            let mac_key = dev.mac.trim().to_lowercase();
            if let Some(meta) = metadata.get(&mac_key) {
                if let Some(alias) = &meta.custom_alias {
                    if !alias.trim().is_empty() {
                        dev.custom_alias = Some(alias.clone());
                    }
                }
                if let Some(cat) = meta.category {
                    dev.category = cat;
                }
                dev.is_whitelisted = meta.is_whitelisted;
                dev.is_blocked = meta.is_blocked;
                dev.speed_limit_kbps = meta.speed_limit_kbps;
                if !meta.request_history.is_empty() {
                    dev.request_history = meta.request_history.clone();
                }
            }
        }
    }

    /// Actualiza o inserta la personalización de un dispositivo por su MAC
    pub fn update_device_meta(
        metadata: &mut HashMap<String, DeviceMetadata>,
        mac: &str,
        alias: Option<String>,
        category: Option<DeviceCategory>,
    ) {
        let key = mac.trim().to_lowercase();
        let entry = metadata.entry(key).or_default();
        if alias.is_some() {
            entry.custom_alias = alias;
        }
        if category.is_some() {
            entry.category = category;
        }
    }

    /// Marca o desmarca un dispositivo en la lista blanca
    pub fn set_device_whitelist(
        metadata: &mut HashMap<String, DeviceMetadata>,
        mac: &str,
        whitelisted: bool,
    ) {
        let key = mac.trim().to_lowercase();
        let entry = metadata.entry(key).or_default();
        entry.is_whitelisted = whitelisted;
    }

    /// Marca o desmarca un dispositivo en la lista negra (bloqueado)
    pub fn set_device_blocked(
        metadata: &mut HashMap<String, DeviceMetadata>,
        mac: &str,
        blocked: bool,
    ) {
        let key = mac.trim().to_lowercase();
        let entry = metadata.entry(key).or_default();
        entry.is_blocked = blocked;
    }

    /// Define o quita el límite de velocidad asignado a un dispositivo
    #[allow(dead_code)]
    pub fn set_device_speed_limit(
        metadata: &mut HashMap<String, DeviceMetadata>,
        mac: &str,
        limit_kbps: Option<u32>,
    ) {
        let key = mac.trim().to_lowercase();
        let entry = metadata.entry(key).or_default();
        entry.speed_limit_kbps = limit_kbps;
    }

    /// Actualiza el historial de peticiones del dispositivo por su MAC
    pub fn update_device_requests(
        metadata: &mut HashMap<String, DeviceMetadata>,
        mac: &str,
        requests: &[crate::model::DeviceRequestEntry],
    ) {
        let key = mac.trim().to_lowercase();
        let entry = metadata.entry(key).or_default();
        entry.request_history = requests.to_vec();
    }
}
