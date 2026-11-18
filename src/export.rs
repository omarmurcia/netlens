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
    /// Genera el nombre de archivo por defecto para JSON con marca temporal
    pub fn generate_json_filename() -> String {
        let now_str = Local::now().format("%Y%m%d_%H%M%S").to_string();
        format!("netlens_scan_{}.json", now_str)
    }

    /// Genera el nombre de archivo por defecto para CSV con marca temporal
    pub fn generate_csv_filename() -> String {
        let now_str = Local::now().format("%Y%m%d_%H%M%S").to_string();
        format!("netlens_scan_{}.csv", now_str)
    }

    /// Obtiene la ruta por defecto en la carpeta de descargas o home del usuario
    pub fn get_export_path(filename: &str) -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let downloads = PathBuf::from(&home).join("Downloads");
        if downloads.exists() {
            downloads.join(filename)
        } else {
            PathBuf::from(&home).join(filename)
        }
    }

    /// Abre un selector de archivos nativo (zenity o kdialog) para que el usuario elija la ruta de guardado
    pub fn pick_save_path_with_file_dialog(
        default_path: &Path,
        filter_name: &str,
        filter_ext: &str,
        title: &str,
    ) -> Option<PathBuf> {
        // 1. Intentar con zenity (estándar en entornos GNOME/X11/Wayland)
        let zenity_res = Command::new("zenity")
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
                // El usuario canceló explícitamente el selector
                return None;
            }
        }

        // 2. Intentar con kdialog (estándar en entornos KDE)
        let kdialog_res = Command::new("kdialog")
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
                // El usuario canceló explícitamente
                return None;
            }
        }

        // 3. Fallback: Si no hay entorno gráfico de diálogo, se devuelve la ruta por defecto
        Some(default_path.to_path_buf())
    }

    /// Guarda los datos en formato JSON en una ruta específica
    pub fn save_json_to_path(
        path: &Path,
        devices: &[Device],
        ctx: &NetworkContext,
    ) -> Result<(), String> {
        let report = JsonExportReport {
            app: "NetLens - Explorador de Red",
            timestamp: Local::now().to_rfc3339(),
            total_devices: devices.len(),
            network: ctx,
            devices,
        };

        let json_str = serde_json::to_string_pretty(&report)
            .map_err(|e| format!("Error serializando JSON: {}", e))?;

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                let _ = fs::create_dir_all(parent);
            }
        }

        fs::write(path, json_str)
            .map_err(|e| format!("Error guardando archivo JSON en {:?}: {}", path, e))?;

        Ok(())
    }

    /// Guarda los datos en formato CSV en una ruta específica
    pub fn save_csv_to_path(path: &Path, devices: &[Device]) -> Result<(), String> {
        let mut csv = String::new();
        // Cabecera CSV
        csv.push_str("IP,MAC,Nombre/Hostname,Fabricante,Categoría,MAC_Privada,Latencia_ms,Puertos_Abiertos,Interfaz,Alias\n");

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
                if d.is_randomized_mac { "Sí" } else { "No" },
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
            .map_err(|e| format!("Error guardando archivo CSV en {:?}: {}", path, e))?;

        Ok(())
    }

    /// Exporta los dispositivos a formato JSON con sangría legible (ruta por defecto)
    #[allow(dead_code)]
    pub fn export_json(devices: &[Device], ctx: &NetworkContext) -> Result<PathBuf, String> {
        let filename = Self::generate_json_filename();
        let path = Self::get_export_path(&filename);
        Self::save_json_to_path(&path, devices, ctx)?;
        Ok(path)
    }

    /// Exporta los dispositivos a formato CSV estándar (ruta por defecto)
    #[allow(dead_code)]
    pub fn export_csv(devices: &[Device]) -> Result<PathBuf, String> {
        let filename = Self::generate_csv_filename();
        let path = Self::get_export_path(&filename);
        Self::save_csv_to_path(&path, devices)?;
        Ok(path)
    }
}
