use std::env;
use std::path::PathBuf;
use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, KEY_SET_VALUE};
use winreg::RegKey;

const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const VALUE_NAME: &str = "NetLens";

/// Windows startup manager (HKCU\Software\Microsoft\Windows\CurrentVersion\Run)
pub struct AutostartManager;

impl AutostartManager {
    /// Windows uses the registry instead of an autostart file.
    pub fn autostart_file_path() -> Option<PathBuf> {
        None
    }

    /// Resolves the executable that should be launched at logon.
    pub fn get_persistent_executable_path() -> Result<PathBuf, String> {
        let exe = env::current_exe().map_err(|e| format!("Could not determine executable path: {}", e))?;
        Ok(std::fs::canonicalize(&exe)
            .map(|p| PathBuf::from(p.to_string_lossy().trim_start_matches(r"\\?\")))
            .unwrap_or(exe))
    }

    fn command_line(exe: &str) -> String {
        format!("\"{}\" --minimized", exe)
    }

    fn registered_value() -> Option<String> {
        let key = RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey_with_flags(RUN_KEY, KEY_READ)
            .ok()?;
        key.get_value::<String, _>(VALUE_NAME).ok()
    }

    /// True if the Run entry exists and points to a valid executable.
    pub fn is_enabled() -> bool {
        match Self::registered_value() {
            Some(cmd) => {
                if cmd.contains("\\target\\") || cmd.contains("integration_tests") {
                    return false;
                }
                let exe = cmd.trim_start_matches('"').split('"').next().unwrap_or("").to_string();
                PathBuf::from(exe).exists()
            }
            None => false,
        }
    }

    /// Enables or disables automatic startup with Windows
    pub fn set_enabled(enable: bool) -> Result<(), String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if enable {
            let exe = Self::get_persistent_executable_path()?;
            let (key, _) = hkcu
                .create_subkey(RUN_KEY)
                .map_err(|e| format!("Error opening registry Run key: {}", e))?;
            key.set_value(VALUE_NAME, &Self::command_line(&exe.to_string_lossy()))
                .map_err(|e| format!("Error writing registry Run value: {}", e))?;
        } else if let Ok(key) = hkcu.open_subkey_with_flags(RUN_KEY, KEY_SET_VALUE) {
            // Ignore "not found": already disabled
            let _ = key.delete_value(VALUE_NAME);
        }
        Ok(())
    }

    /// Removes/repairs the Run entry so it matches the user's saved preference
    pub fn sync_if_needed(autostart_enabled: bool) {
        if !autostart_enabled {
            let _ = Self::set_enabled(false);
            return;
        }
        let current_exe = Self::get_persistent_executable_path()
            .map(|p| Self::command_line(&p.to_string_lossy()))
            .ok();
        let needs_repair = !Self::is_enabled() || Self::registered_value() != current_exe;
        if needs_repair {
            if let Err(e) = Self::set_enabled(true) {
                eprintln!("Warning: Could not auto-repair autostart entry: {}", e);
            }
        }
    }
}
