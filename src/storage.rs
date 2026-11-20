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

fn default_true() -> bool {
    true
}

fn default_theme() -> String {
    "TokyoNight".to_string()
}

fn default_auto_scan_interval() -> u32 {
    30
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    #[serde(default)]
    pub language: Language,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default)]
    pub devices: Vec<Device>,
    #[serde(default)]
    pub device_metadata: HashMap<String, DeviceMetadata>,
    #[serde(default)]
    pub last_speed_test: Option<SpeedTestResult>,
    #[serde(default)]
    pub speed_test_history: Vec<SpeedTestResult>,
    #[serde(default)]
    pub router_ip: Option<String>,
    #[serde(default)]
    pub router_password: Option<String>,
    #[serde(default = "default_auto_scan_interval")]
    pub auto_scan_interval_secs: u32,
    #[serde(default)]
    pub known_macs: Vec<String>,
    #[serde(default)]
    pub autostart: bool,
    #[serde(default = "default_true")]
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
            language: Language::English,
            theme: "TokyoNight".to_string(),
            devices: Vec::new(),
            device_metadata: HashMap::new(),
            last_speed_test: None,
            speed_test_history: Vec::new(),
            router_ip: None,
            router_password: None,
            auto_scan_interval_secs: 30,
            known_macs: Vec::new(),
            autostart: false,
            minimize_to_tray_on_close: true,
            network_events: Vec::new(),
            notification_settings: NotificationSettings::default(),
            storage_credentials: None,
        }
    }
}

/// Borrowed view of `AppState` for zero-clone serialization
#[derive(Serialize)]
pub struct AppStateRef<'a> {
    pub language: Language,
    pub theme: &'a str,
    pub devices: &'a [Device],
    pub device_metadata: &'a HashMap<String, DeviceMetadata>,
    pub last_speed_test: Option<&'a SpeedTestResult>,
    pub speed_test_history: &'a [SpeedTestResult],
    pub router_ip: Option<&'a str>,
    pub router_password: Option<&'a str>,
    pub auto_scan_interval_secs: u32,
    pub known_macs: Vec<&'a String>,
    pub autostart: bool,
    pub minimize_to_tray_on_close: bool,
    pub network_events: &'a [crate::history::NetworkEvent],
    pub notification_settings: &'a NotificationSettings,
    pub storage_credentials: Option<&'a crate::router::StorageCredentials>,
}

pub struct Storage;

impl Storage {
    /// Persistent configuration file path according to XDG (~/.config/netlens/state.json)
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

    /// Loads saved state from disk. If absent or corrupted, returns defaults.
    pub fn load() -> AppState {
        let path = Self::get_storage_path();
        if !path.exists() {
            return AppState::default();
        }

        match fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<AppState>(&content) {
                Ok(mut state) => {
                    // Re-apply metadata (aliases, categories, whitelists/blacklists) to devices
                    Self::apply_metadata(&mut state.devices, &state.device_metadata);
                    state
                }
                Err(err) => {
                    eprintln!("Warning: Failed to deserialize saved state: {}", err);
                    AppState::default()
                }
            },
            Err(err) => {
                eprintln!("Warning: Failed to read state file {:?}: {}", path, err);
                AppState::default()
            }
        }
    }

    /// Writes serialized JSON atomically via temp file + rename to prevent corruption
    pub fn write_json_atomic(json: &str) -> Result<(), String> {
        let path = Self::get_storage_path();
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                let _ = fs::create_dir_all(parent);
            }
        }

        let tmp_path = path.with_extension("json.tmp");
        fs::write(&tmp_path, json)
            .map_err(|e| format!("Error writing temporary state {:?}: {}", tmp_path, e))?;

        fs::rename(&tmp_path, &path)
            .map_err(|e| format!("Error committing state to {:?}: {}", path, e))?;

        Ok(())
    }

    /// Serializes a borrowed state reference without cloning `App` collections
    pub fn serialize_ref(state: &AppStateRef<'_>) -> Result<String, String> {
        serde_json::to_string_pretty(state)
            .map_err(|e| format!("Error serializing state to JSON: {}", e))
    }

    /// Safely writes current state to disk
    pub fn save(state: &AppState) -> Result<(), String> {
        let json = serde_json::to_string_pretty(state)
            .map_err(|e| format!("Error serializing state to JSON: {}", e))?;
        Self::write_json_atomic(&json)
    }

    /// Applies aliases, categories, and security policies (whitelist/blacklist) to devices by MAC
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

    /// Updates or inserts device customization metadata by MAC
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

    /// Marks or unmarks a device on the whitelist
    pub fn set_device_whitelist(
        metadata: &mut HashMap<String, DeviceMetadata>,
        mac: &str,
        whitelisted: bool,
    ) {
        let key = mac.trim().to_lowercase();
        let entry = metadata.entry(key).or_default();
        entry.is_whitelisted = whitelisted;
    }

    /// Marks or unmarks a device on the blacklist (blocked)
    pub fn set_device_blocked(
        metadata: &mut HashMap<String, DeviceMetadata>,
        mac: &str,
        blocked: bool,
    ) {
        let key = mac.trim().to_lowercase();
        let entry = metadata.entry(key).or_default();
        entry.is_blocked = blocked;
    }

    /// Defines or clears assigned speed limit for a device
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

    /// Updates device DNS request history by MAC
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
