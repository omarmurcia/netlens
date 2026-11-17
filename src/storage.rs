use crate::i18n::Language;
use crate::model::{Device, DeviceCategory};
use crate::speedtest::SpeedTestResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceMetadata {
    pub custom_alias: Option<String>,
    pub category: Option<DeviceCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub language: Language,
    pub theme: String,
    pub devices: Vec<Device>,
    pub device_metadata: HashMap<String, DeviceMetadata>,
    pub last_speed_test: Option<SpeedTestResult>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            language: Language::Spanish,
            theme: "TokyoNight".to_string(),
            devices: Vec::new(),
            device_metadata: HashMap::new(),
            last_speed_test: None,
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
                    // Reaplicar metadatos (alias y categorías personalizadas) a los dispositivos cargados
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

    /// Aplica alias y categorías personalizadas a una lista de dispositivos basado en su dirección MAC
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
}
