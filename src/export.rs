use crate::model::{Device, NetworkContext};
use chrono::Local;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Serialize)]
struct JsonExportReport<'a> {
    app: &'static str,
    timestamp: String,
    total_devices: usize,
    network: &'a NetworkContext,
    devices: &'a [Device],
}

pub struct Exporter;

impl Exporter {
    /// Generates default JSON report filename with current timestamp
    pub fn generate_json_filename() -> String {
        let now_str = Local::now().format("%Y%m%d_%H%M%S").to_string();
        format!("netlens_scan_{}.json", now_str)
    }

    /// Generates default CSV report filename with current timestamp
    pub fn generate_csv_filename() -> String {
        let now_str = Local::now().format("%Y%m%d_%H%M%S").to_string();
        format!("netlens_scan_{}.csv", now_str)
    }

    /// Resolves the default export path in user's Downloads or home folder
    pub fn get_export_path(filename: &str) -> PathBuf {
        let home = crate::procutil::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let downloads = home.join("Downloads");
        if downloads.exists() {
            downloads.join(filename)
        } else {
            home.join(filename)
        }
    }

    /// Windows: native "Save as" dialog through PowerShell / WinForms
    #[cfg(windows)]
    pub fn pick_save_path_with_file_dialog(
        default_path: &Path,
        filter_name: &str,
        filter_ext: &str,
        title: &str,
    ) -> Option<PathBuf> {
        match crate::procutil::windows_save_dialog(default_path, filter_name, filter_ext, title) {
            Some(Some(mut p)) => {
                if p.extension().is_none() {
                    p.set_extension(filter_ext);
                }
                Some(p)
            }
            Some(None) => None, // user cancelled
            None => Some(default_path.to_path_buf()),
        }
    }
    /// Opens native desktop file dialog (zenity or kdialog) for user to choose save destination
    #[cfg(not(windows))]
    pub fn pick_save_path_with_file_dialog(
        default_path: &Path,
        filter_name: &str,
        filter_ext: &str,
        title: &str,
    ) -> Option<PathBuf> {
        // 1. Try zenity (standard in GNOME / Wayland / GTK environments)
        let zenity_res = crate::procutil::NoWindow::no_window(Command::new("zenity"))
            .arg("--file-selection")
            .arg("--save")
            .arg("--confirm-overwrite")
            .arg(format!("--title={}", title))
            .arg(format!("--filename={}", default_path.to_string_lossy()))
            .arg(format!(
                "--file-filter={} (*.{}) | *.{}",
                filter_name, filter_ext, filter_ext
            ))
            .output();

        if let Ok(output) = zenity_res {
            if output.status.success() {
                let chosen = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !chosen.is_empty() {
                    let mut p = PathBuf::from(chosen);
                    if p.extension().is_none() {
                        p.set_extension(filter_ext);
                    }
                    return Some(p);
                }
            } else if output.status.code() == Some(1) {
                // User explicitly clicked Cancel or closed the dialog
                return None;
            }
        }

        // 2. Try kdialog (standard in KDE Plasma desktop environments)
        let kdialog_res = crate::procutil::NoWindow::no_window(Command::new("kdialog"))
            .arg("--getsavefilename")
            .arg(default_path.to_string_lossy().to_string())
            .arg(format!("*.{}|{} (*.{})", filter_ext, filter_name, filter_ext))
            .arg("--title")
            .arg(title)
            .output();

        if let Ok(output) = kdialog_res {
            if output.status.success() {
                let chosen = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !chosen.is_empty() {
                    let mut p = PathBuf::from(chosen);
                    if p.extension().is_none() {
                        p.set_extension(filter_ext);
                    }
                    return Some(p);
                }
            } else if output.status.code() == Some(1) {
                // User cancelled in kdialog
                return None;
            }
        }

        // 3. Fallback: If no GUI dialog is available, return default path
        Some(default_path.to_path_buf())
    }

    /// Saves device and network data formatted as JSON to the specified path
    pub fn save_json_to_path(
        path: &Path,
        devices: &[Device],
        ctx: &NetworkContext,
    ) -> Result<(), String> {
        let report = JsonExportReport {
            app: "NetLens - Network Explorer",
            timestamp: Local::now().to_rfc3339(),
            total_devices: devices.len(),
            network: ctx,
            devices,
        };

        let json_str = serde_json::to_string_pretty(&report)
            .map_err(|e| format!("Error serializing JSON: {}", e))?;

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                let _ = fs::create_dir_all(parent);
            }
        }

        fs::write(path, json_str)
            .map_err(|e| format!("Error saving JSON file to {:?}: {}", path, e))?;

        Ok(())
    }

    /// Saves device data formatted as CSV to the specified path
    pub fn save_csv_to_path(path: &Path, devices: &[Device]) -> Result<(), String> {
        let mut csv = String::new();
        // CSV Header
        csv.push_str("IP,MAC,Hostname,Manufacturer,Category,Private_MAC,Latency_ms,Open_Ports,Interface,Alias\n");

        for d in devices {
            let host = d.hostname.as_deref().unwrap_or("-");
            let alias = d.custom_alias.as_deref().unwrap_or("-");
            let latency = d
                .latency_ms
                .map(|l| format!("{:.2}", l))
                .unwrap_or_else(|| "-".to_string());
            let ports: Vec<String> = d.open_ports.iter().map(|p| p.port.to_string()).collect();
            let ports_str = if ports.is_empty() {
                "-".to_string()
            } else {
                ports.join(";")
            };

            csv.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                d.ip,
                d.mac,
                host.replace('"', "\"\""),
                d.vendor.replace('"', "\"\""),
                d.category.display_name(),
                if d.is_randomized_mac { "Yes" } else { "No" },
                latency,
                ports_str,
                d.interface,
                alias.replace('"', "\"\"")
            ));
        }

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                let _ = fs::create_dir_all(parent);
            }
        }

        fs::write(path, csv)
            .map_err(|e| format!("Error saving CSV file to {:?}: {}", path, e))?;

        Ok(())
    }

    /// Exports devices to pretty-printed JSON format (default path)
    #[allow(dead_code)]
    pub fn export_json(devices: &[Device], ctx: &NetworkContext) -> Result<PathBuf, String> {
        let filename = Self::generate_json_filename();
        let path = Self::get_export_path(&filename);
        Self::save_json_to_path(&path, devices, ctx)?;
        Ok(path)
    }

    /// Exports devices to standard CSV format (default path)
    #[allow(dead_code)]
    pub fn export_csv(devices: &[Device]) -> Result<PathBuf, String> {
        let filename = Self::generate_csv_filename();
        let path = Self::get_export_path(&filename);
        Self::save_csv_to_path(&path, devices)?;
        Ok(path)
    }
}
