use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// System boot and desktop autostart manager (XDG Autostart standard)
pub struct AutostartManager;

impl AutostartManager {
    /// Gets the path to the autostart desktop entry in user's home (~/.config/autostart/netlens.desktop)
    pub fn autostart_file_path() -> Option<PathBuf> {
        let base_dir = if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
            let p = PathBuf::from(xdg_config);
            if !p.as_os_str().is_empty() {
                p
            } else if let Ok(home) = env::var("HOME") {
                PathBuf::from(home).join(".config")
            } else {
                return None;
            }
        } else if let Ok(home) = env::var("HOME") {
            PathBuf::from(home).join(".config")
        } else {
            return None;
        };
        Some(base_dir.join("autostart").join("netlens.desktop"))
    }

    /// Ensures the official 512x512 application icon is present in ~/.local/share/icons/...
    pub fn ensure_icon_installed() -> Option<PathBuf> {
        let system_icon = PathBuf::from("/usr/share/icons/hicolor/512x512/apps/netlens.png");
        if system_icon.exists() {
            return Some(system_icon);
        }
        let home = env::var("HOME").ok()?;
        let icon_dir = PathBuf::from(&home).join(".local/share/icons/hicolor/512x512/apps");
        let icon_path = icon_dir.join("netlens.png");
        if !icon_path.exists() {
            let _ = fs::create_dir_all(&icon_dir);
            let _ = fs::write(&icon_path, include_bytes!("../assets/netlens.png"));
        }
        Some(icon_path)
    }

    /// Ensures application menu integration (~/.local/share/applications/netlens.desktop)
    pub fn ensure_applications_desktop(exec_path: &Path) {
        if let Ok(home) = env::var("HOME") {
            let app_dir = PathBuf::from(&home).join(".local/share/applications");
            let app_desktop = app_dir.join("netlens.desktop");

            // If NetLens is installed as a system package (under /usr) or /usr/share/applications/netlens.desktop exists,
            // we must NEVER duplicate it in ~/.local/share/applications/ because package managers (pacman/shelly)
            // manage /usr/share/applications/ and do not touch user's $HOME upon uninstall.
            if exec_path.starts_with("/usr/") || Path::new("/usr/share/applications/netlens.desktop").exists() {
                if app_desktop.exists() {
                    let _ = fs::remove_file(&app_desktop);
                }
                return;
            }

            let _ = fs::create_dir_all(&app_dir);

            let icon_path_str = Self::ensure_icon_installed()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "netlens".to_string());

            let content = format!(
                "[Desktop Entry]\n\
                Type=Application\n\
                Name=NetLens\n\
                GenericName=Network Monitor\n\
                Comment=Local Network Device Explorer & Monitor\n\
                Exec=\"{}\"\n\
                Icon={}\n\
                Terminal=false\n\
                Categories=Network;Monitor;System;\n\
                StartupWMClass=netlens\n",
                exec_path.to_string_lossy(),
                icon_path_str
            );
            let _ = fs::write(app_desktop, content);
        }
    }

    /// Determines the permanent, valid executable path on the system.
    /// Resolves the AppImage issue: avoids using ephemeral /tmp/.mount_XXXXXX mount points
    /// which are destroyed when the system reboots.
    pub fn get_persistent_executable_path() -> Result<PathBuf, String> {
        // 1. Check AppImage runtime environment variables
        for var in &["APPIMAGE", "TARGET_APPIMAGE", "ARGV0"] {
            if let Ok(val) = env::var(var) {
                let trimmed = val.trim();
                if !trimmed.is_empty() {
                    let p = PathBuf::from(trimmed);
                    if p.exists() {
                        return Ok(fs::canonicalize(&p).unwrap_or(p));
                    }
                }
            }
        }

        // 2. Prioritize official system package installation (/usr/bin/netlens)
        let system_bin = PathBuf::from("/usr/bin/netlens");
        if system_bin.exists() {
            return Ok(system_bin);
        }

        // 3. Check current process executable path
        let current_exe = env::current_exe()
            .map_err(|e| format!("Could not determine executable path: {}", e))?;

        let exe_str = current_exe.to_string_lossy();

        // 4. If running from a temporary mount in /tmp without AppImage env vars
        if exe_str.starts_with("/tmp/.mount_") {
            if let Ok(home) = env::var("HOME") {
                let candidates = [
                    PathBuf::from(&home).join("Documents/homenet/netlens/dist/NetLens-x86_64.AppImage"),
                    PathBuf::from(&home).join(".local/bin/NetLens-x86_64.AppImage"),
                    PathBuf::from(&home).join("Applications/NetLens-x86_64.AppImage"),
                    PathBuf::from(&home).join("Desktop/NetLens-x86_64.AppImage"),
                ];
                for candidate in &candidates {
                    if candidate.exists() {
                        return Ok(fs::canonicalize(candidate).unwrap_or_else(|_| candidate.clone()));
                    }
                }
            }
            return Err("NetLens is running from a temporary mount point and permanent AppImage path was not found.".to_string());
        }

        // 5. If running inside target/ (cargo run, tests) or test runners, NEVER use the target/test binary!
        if exe_str.contains("/target/") || exe_str.contains("integration_tests") {
            if let Ok(which_out) = std::process::Command::new("which").arg("netlens").output() {
                if which_out.status.success() {
                    let path_str = String::from_utf8_lossy(&which_out.stdout).trim().to_string();
                    if !path_str.is_empty() {
                        let p = PathBuf::from(path_str);
                        if p.exists() {
                            return Ok(p);
                        }
                    }
                }
            }
            // Safest fallback for Linux system installation
            return Ok(PathBuf::from("/usr/bin/netlens"));
        }

        Ok(current_exe)
    }

    /// Checks whether autostart on system boot is enabled and valid
    pub fn is_enabled() -> bool {
        let Some(path) = Self::autostart_file_path() else {
            return false;
        };

        if !path.exists() {
            return false;
        }

        if let Ok(content) = fs::read_to_string(&path) {
            // If pointing to a dead /tmp temporary path, it is not a valid autostart
            if content.contains("/tmp/.mount_") {
                return false;
            }
            // If pointing to a test runner or build target, it is not a valid autostart
            if content.contains("/target/") || content.contains("integration_tests") {
                return false;
            }
            // If explicitly disabled by desktop manager
            if content.contains("X-GNOME-Autostart-enabled=false") {
                return false;
            }
            // Check that Exec entry is present
            content.contains("Exec=")
        } else {
            false
        }
    }

    /// Generates autostart desktop entry content pointing to binary with `--minimized` argument
    pub fn generate_desktop_entry(exec_path: &str) -> String {
        let formatted_exec = if exec_path.contains(' ') && !exec_path.starts_with('"') {
            format!("\"{}\"", exec_path)
        } else {
            exec_path.to_string()
        };

        let icon_str = Self::ensure_icon_installed()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "netlens".to_string());

        format!(
            "[Desktop Entry]\n\
            Type=Application\n\
            Name=NetLens\n\
            GenericName=Network Monitor\n\
            Comment=Local Network Device Explorer & Monitor\n\
            Exec={} --minimized\n\
            Icon={}\n\
            Terminal=false\n\
            Categories=Network;Monitor;System;\n\
            X-GNOME-Autostart-enabled=true\n\
            X-KDE-autostart-after=panel\n\
            X-MATE-Autostart-enabled=true\n\
            StartupNotify=false\n",
            formatted_exec,
            icon_str
        )
    }

    /// Enables or disables automatic startup with the operating system
    pub fn set_enabled(enable: bool) -> Result<(), String> {
        let path = Self::autostart_file_path()
            .ok_or_else(|| "Could not determine user HOME directory".to_string())?;

        if enable {
            let persistent_exe = Self::get_persistent_executable_path()?;
            let exe_str = persistent_exe.to_string_lossy();

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Error creating autostart directory {:?}: {}", parent, e))?;
            }

            // Ensure icon and desktop integration are generated
            let _ = Self::ensure_icon_installed();
            Self::ensure_applications_desktop(&persistent_exe);

            let content = Self::generate_desktop_entry(&exe_str);
            fs::write(&path, content)
                .map_err(|e| format!("Error writing autostart file {:?}: {}", path, e))?;
        } else {
            if path.exists() {
                fs::remove_file(&path)
                    .map_err(|e| format!("Error removing autostart file {:?}: {}", path, e))?;
            }
            if let Ok(home) = env::var("HOME") {
                let local_desktop = PathBuf::from(home).join(".local/share/applications/netlens.desktop");
                if local_desktop.exists() {
                    let _ = fs::remove_file(local_desktop);
                }
            }
        }

        Ok(())
    }

    /// Checks if autostart is marked as active but has a stale/broken path, and auto-repairs it
    pub fn sync_if_needed(autostart_enabled: bool) {
        if !autostart_enabled {
            if let Some(path) = Self::autostart_file_path() {
                if path.exists() {
                    let _ = fs::remove_file(path);
                }
            }
            if let Ok(home) = env::var("HOME") {
                let local_desktop = PathBuf::from(home).join(".local/share/applications/netlens.desktop");
                if local_desktop.exists() {
                    let _ = fs::remove_file(local_desktop);
                }
            }
            return;
        }

        let needs_repair = match Self::autostart_file_path() {
            Some(path) => {
                if !path.exists() {
                    true
                } else if let Ok(content) = fs::read_to_string(&path) {
                    content.contains("/tmp/.mount_")
                        || content.contains("/target/")
                        || content.contains("integration_tests")
                        || !Self::is_enabled()
                } else {
                    true
                }
            }
            None => false,
        };

        if needs_repair {
            if let Err(e) = Self::set_enabled(true) {
                eprintln!("Warning: Could not auto-repair autostart entry: {}", e);
            }
        }
    }
}
