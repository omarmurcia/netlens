use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiNetworkInfo {
    pub ssid: String,
    pub bssid: String,
    pub channel: u32,
    pub frequency_mhz: u32,
    pub signal_percent: u8,
    pub security: String,
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelOccupancy {
    pub channel: u32,
    pub networks_count: usize,
    pub max_signal: u8,
    pub ssids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WifiAnalyzerReport {
    pub networks: Vec<WifiNetworkInfo>,
    pub current_ssid: Option<String>,
    pub current_channel: Option<u32>,
    pub recommended_channel_24: u32,
    pub recommended_channel_5: u32,
    pub recommendation_text: String,
    pub channels_24: Vec<ChannelOccupancy>,
    pub channels_5: Vec<ChannelOccupancy>,
}

pub struct WifiAnalyzer;

impl WifiAnalyzer {
    /// Escanea las redes Wi-Fi del entorno y genera un reporte analítico de canales y saturación
    pub async fn scan() -> Result<WifiAnalyzerReport, String> {
        // Ejecutar nmcli para listar redes Wi-Fi
        let output = Command::new("nmcli")
            .args(["-t", "-f", "IN-USE,SSID,BSSID,CHAN,FREQ,SIGNAL,SECURITY", "dev", "wifi", "list"])
            .output()
            .await
            .map_err(|e| format!("Error ejecutando nmcli: {}", e))?;

        let raw = String::from_utf8_lossy(&output.stdout);
        let mut networks = Vec::new();
        let mut current_ssid = None;
        let mut current_channel = None;

        for line in raw.lines() {
            let parsed = Self::parse_nmcli_line(line);
            if let Some(net) = parsed {
                if net.is_current {
                    current_ssid = Some(net.ssid.clone());
                    current_channel = Some(net.channel);
                }
                networks.push(net);
            }
        }

        // Si nmcli falló o está vacío, intentar sin IN-USE
        if networks.is_empty() {
            let output_fallback = Command::new("nmcli")
                .args(["-t", "-f", "SSID,BSSID,CHAN,FREQ,SIGNAL,SECURITY", "dev", "wifi", "list"])
                .output()
                .await
                .map_err(|e| format!("Error en nmcli fallback: {}", e))?;

            let raw_fallback = String::from_utf8_lossy(&output_fallback.stdout);
            for line in raw_fallback.lines() {
                if let Some(net) = Self::parse_nmcli_line_simple(line) {
                    networks.push(net);
                }
            }
        }

        let report = Self::analyze_networks(networks, current_ssid, current_channel);
        Ok(report)
    }

    /// Analiza la ocupación de canales y calcula las recomendaciones de espectro
    fn analyze_networks(
        networks: Vec<WifiNetworkInfo>,
        current_ssid: Option<String>,
        current_channel: Option<u32>,
    ) -> WifiAnalyzerReport {
        let mut map_24: HashMap<u32, (usize, u8, Vec<String>)> = HashMap::new();
        let mut map_5: HashMap<u32, (usize, u8, Vec<String>)> = HashMap::new();

        // Inicializar canales no solapados estándar de 2.4 GHz
        for &ch in &[1, 6, 11] {
            map_24.insert(ch, (0, 0, Vec::new()));
        }

        for net in &networks {
            let is_5ghz = net.frequency_mhz >= 5000;
            let target_map = if is_5ghz { &mut map_5 } else { &mut map_24 };

            let entry = target_map.entry(net.channel).or_insert((0, 0, Vec::new()));
            entry.0 += 1;
            entry.1 = entry.1.max(net.signal_percent);
            let display_ssid = if net.ssid.is_empty() {
                format!("(Red Oculta {})", &net.bssid[..8.min(net.bssid.len())])
            } else {
                net.ssid.clone()
            };
            if !entry.2.contains(&display_ssid) {
                entry.2.push(display_ssid);
            }
        }

        // Convertir a listas ordenadas
        let mut channels_24: Vec<ChannelOccupancy> = map_24
            .into_iter()
            .map(|(channel, (networks_count, max_signal, ssids))| ChannelOccupancy {
                channel,
                networks_count,
                max_signal,
                ssids,
            })
            .collect();
        channels_24.sort_by_key(|c| c.channel);

        let mut channels_5: Vec<ChannelOccupancy> = map_5
            .into_iter()
            .map(|(channel, (networks_count, max_signal, ssids))| ChannelOccupancy {
                channel,
                networks_count,
                max_signal,
                ssids,
            })
            .collect();
        channels_5.sort_by_key(|c| c.channel);

        // Elegir mejor canal en 2.4 GHz entre los no solapados (1, 6, 11)
        let mut best_ch_24 = 1;
        let mut min_interference_24 = usize::MAX;
        for &ch in &[1, 6, 11] {
            let count = channels_24.iter().find(|c| c.channel == ch).map(|c| c.networks_count).unwrap_or(0);
            if count < min_interference_24 {
                min_interference_24 = count;
                best_ch_24 = ch;
            }
        }

        // Elegir mejor canal en 5 GHz (canales típicos 36, 40, 44, 48, 149, 153, 157, 161)
        let candidates_5 = [36, 44, 48, 149, 157];
        let mut best_ch_5 = 36;
        let mut min_interference_5 = usize::MAX;
        for &ch in &candidates_5 {
            let count = channels_5.iter().find(|c| c.channel == ch).map(|c| c.networks_count).unwrap_or(0);
            if count < min_interference_5 {
                min_interference_5 = count;
                best_ch_5 = ch;
            }
        }

        // Redactar recomendación
        let recommendation_text = match current_channel {
            Some(curr) if curr <= 14 => {
                if curr == best_ch_24 {
                    format!("Tu red está en el canal {} (2.4 GHz), el cual presenta la menor interferencia en tu entorno.", curr)
                } else {
                    format!(
                        "Tu red está en el canal {} (2.4 GHz). Se recomienda cambiar al canal {} para evitar interferencias de {} redes vecinas.",
                        curr, best_ch_24, min_interference_24
                    )
                }
            }
            Some(curr) => {
                format!("Tu red opera en el canal {} (5 GHz). El espectro de 5 GHz está optimizado.", curr)
            }
            None => {
                format!(
                    "Canal 2.4 GHz recomendado: Canal {} (menos congestionado). Canal 5 GHz recomendado: Canal {}.",
                    best_ch_24, best_ch_5
                )
            }
        };

        WifiAnalyzerReport {
            networks,
            current_ssid,
            current_channel,
            recommended_channel_24: best_ch_24,
            recommended_channel_5: best_ch_5,
            recommendation_text,
            channels_24,
            channels_5,
        }
    }

    /// Parsea una línea de salida de nmcli con formato con IN-USE
    fn parse_nmcli_line(line: &str) -> Option<WifiNetworkInfo> {
        let clean = line.replace(r"\:", "__COLON__");
        let parts: Vec<&str> = clean.split(':').collect();
        if parts.len() < 7 {
            return None;
        }

        let is_current = parts[0].trim() == "*";
        let ssid = parts[1].replace("__COLON__", ":").trim().to_string();
        let bssid = parts[2].replace("__COLON__", ":").trim().to_string();
        let channel: u32 = parts[3].trim().parse().ok()?;
        let freq_str = parts[4].replace("MHz", "").trim().to_string();
        let frequency_mhz: u32 = freq_str.parse().unwrap_or(2412);
        let signal_percent: u8 = parts[5].trim().parse().unwrap_or(50);
        let security = parts[6].replace("__COLON__", ":").trim().to_string();

        Some(WifiNetworkInfo {
            ssid,
            bssid,
            channel,
            frequency_mhz,
            signal_percent,
            security,
            is_current,
        })
    }

    /// Parsea una línea simple sin IN-USE
    fn parse_nmcli_line_simple(line: &str) -> Option<WifiNetworkInfo> {
        let clean = line.replace(r"\:", "__COLON__");
        let parts: Vec<&str> = clean.split(':').collect();
        if parts.len() < 6 {
            return None;
        }

        let ssid = parts[0].replace("__COLON__", ":").trim().to_string();
        let bssid = parts[1].replace("__COLON__", ":").trim().to_string();
        let channel: u32 = parts[2].trim().parse().ok()?;
        let freq_str = parts[3].replace("MHz", "").trim().to_string();
        let frequency_mhz: u32 = freq_str.parse().unwrap_or(2412);
        let signal_percent: u8 = parts[4].trim().parse().unwrap_or(50);
        let security = parts[5].replace("__COLON__", ":").trim().to_string();

        Some(WifiNetworkInfo {
            ssid,
            bssid,
            channel,
            frequency_mhz,
            signal_percent,
            security,
            is_current: false,
        })
    }
}
