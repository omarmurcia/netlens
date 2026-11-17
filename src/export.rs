use crate::model::{Device, NetworkContext};
use chrono::Local;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

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

    /// Exporta los dispositivos a formato JSON con sangría legible
    pub fn export_json(devices: &[Device], ctx: &NetworkContext) -> Result<PathBuf, String> {
        let now_str = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("netlens_scan_{}.json", now_str);
        let path = Self::get_export_path(&filename);

        let report = JsonExportReport {
            app: "NetLens - Explorador de Red",
            timestamp: Local::now().to_rfc3339(),
            total_devices: devices.len(),
            network: ctx,
            devices,
        };

        let json_str = serde_json::to_string_pretty(&report)
            .map_err(|e| format!("Error serializando JSON: {}", e))?;

        fs::write(&path, json_str)
            .map_err(|e| format!("Error guardando archivo JSON en {:?}: {}", path, e))?;

        Ok(path)
    }

    /// Exporta los dispositivos a formato CSV estándar
    pub fn export_csv(devices: &[Device]) -> Result<PathBuf, String> {
        let now_str = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("netlens_scan_{}.csv", now_str);
        let path = Self::get_export_path(&filename);

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

        fs::write(&path, csv)
            .map_err(|e| format!("Error guardando archivo CSV en {:?}: {}", path, e))?;

        Ok(path)
    }
}
