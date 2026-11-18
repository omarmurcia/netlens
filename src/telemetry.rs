use std::fs;
use std::process::Command;
use std::time::Instant;

/// Information about local Wi-Fi connection status
#[derive(Debug, Clone, Default)]
pub struct WifiStatus {
    #[allow(dead_code)]
    pub interface: String,
    pub ssid: Option<String>,
    pub signal_dbm: Option<i32>,
    pub signal_percent: u8,
    pub frequency_mhz: Option<f32>,
    pub bitrate_mbps: Option<f32>,
    pub is_connected: bool,
}

impl WifiStatus {
    #[allow(dead_code)]
    pub fn signal_label(&self) -> &'static str {
        if self.signal_percent >= 80 {
            "Excellent"
        } else if self.signal_percent >= 60 {
            "Good"
        } else if self.signal_percent >= 40 {
            "Fair"
        } else if self.signal_percent >= 20 {
            "Weak"
        } else {
            "Very Weak"
        }
    }

    pub fn band_label(&self) -> String {
        if let Some(freq) = self.frequency_mhz {
            if freq >= 5000.0 {
                "5 GHz".to_string()
            } else if freq >= 2400.0 {
                "2.4 GHz".to_string()
            } else if freq >= 6000.0 {
                "6 GHz (Wi-Fi 6E)".to_string()
            } else {
                format!("{:.0} MHz", freq)
            }
        } else {
            "Wi-Fi".to_string()
        }
    }
}

/// Information about wired Ethernet connection
#[derive(Debug, Clone, Default)]
pub struct EthernetStatus {
    pub interface: String,
    pub speed_mbps: Option<u32>,
    #[allow(dead_code)]
    pub is_full_duplex: bool,
    #[allow(dead_code)]
    pub is_up: bool,
}

impl EthernetStatus {
    pub fn display_summary(&self) -> String {
        if let Some(speed) = self.speed_mbps {
            if speed >= 1000 {
                format!("{} Gbps Full Duplex", speed / 1000)
            } else {
                format!("{} Mbps", speed)
            }
        } else {
            "Connected Ethernet Cable".to_string()
        }
    }
}

/// Real-time network telemetry (traffic and signal)
#[derive(Debug, Clone)]
pub struct Telemetry {
    pub current_rx_bytes: u64,
    pub current_tx_bytes: u64,
    pub prev_rx_bytes: u64,
    pub prev_tx_bytes: u64,
    pub download_speed_bps: f64,
    pub upload_speed_bps: f64,
    pub initial_rx_bytes: u64,
    pub initial_tx_bytes: u64,
    pub last_sample: Instant,
    pub wifi: Option<WifiStatus>,
    pub ethernet: Option<EthernetStatus>,
    pub active_interface: String,
}

impl Telemetry {
    pub fn new(active_interface: &str) -> Self {
        let (rx, tx) = Self::sample_total_traffic();
        let wifi = Self::detect_wifi();
        let ethernet = Self::detect_ethernet(active_interface);

        Self {
            current_rx_bytes: rx,
            current_tx_bytes: tx,
            prev_rx_bytes: rx,
            prev_tx_bytes: tx,
            download_speed_bps: 0.0,
            upload_speed_bps: 0.0,
            initial_rx_bytes: rx,
            initial_tx_bytes: tx,
            last_sample: Instant::now(),
            wifi,
            ethernet,
            active_interface: active_interface.to_string(),
        }
    }

    /// Updates speed sampling and network status
    pub fn update_sample(&mut self, active_interface: &str) {
        self.active_interface = active_interface.to_string();
        let (rx, tx) = Self::sample_total_traffic();
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_sample).as_secs_f64();

        if elapsed >= 0.5 {
            let rx_diff = rx.saturating_sub(self.prev_rx_bytes);
            let tx_diff = tx.saturating_sub(self.prev_tx_bytes);

            self.download_speed_bps = (rx_diff as f64) / elapsed;
            self.upload_speed_bps = (tx_diff as f64) / elapsed;

            self.prev_rx_bytes = rx;
            self.prev_tx_bytes = tx;
            self.current_rx_bytes = rx;
            self.current_tx_bytes = tx;
            self.last_sample = now;
        }

        // Refresh Wi-Fi and Ethernet status
        self.wifi = Self::detect_wifi();
        self.ethernet = Self::detect_ethernet(active_interface);
    }

    /// Reads total transmitted and received bytes in /proc/net/dev
    pub fn sample_total_traffic() -> (u64, u64) {
        let mut total_rx = 0u64;
        let mut total_tx = 0u64;

        if let Ok(content) = fs::read_to_string("/proc/net/dev") {
            for line in content.lines().skip(2) {
                if let Some((iface, data)) = line.split_once(':') {
                    let iface_clean = iface.trim();
                    // Ignore local loopback interface
                    if iface_clean != "lo" && !iface_clean.starts_with("docker") && !iface_clean.starts_with("veth") {
                        let fields: Vec<&str> = data.split_whitespace().collect();
                        if fields.len() >= 9 {
                            let rx = fields[0].parse::<u64>().unwrap_or(0);
                            let tx = fields[8].parse::<u64>().unwrap_or(0);
                            total_rx += rx;
                            total_tx += tx;
                        }
                    }
                }
            }
        }

        (total_rx, total_tx)
    }

    /// Detects and parses Wi-Fi signal information
    pub fn detect_wifi() -> Option<WifiStatus> {
        let mut wifi_iface = None;

        // Search for wireless interfaces in /sys/class/net/*/wireless or starting with wl
        if let Ok(entries) = fs::read_dir("/sys/class/net") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let wireless_dir = entry.path().join("wireless");
                let phy_dir = entry.path().join("phy80211");
                if wireless_dir.exists() || phy_dir.exists() || name.starts_with("wl") {
                    wifi_iface = Some(name);
                    break;
                }
            }
        }

        let iface = wifi_iface?;
        let mut status = WifiStatus {
            interface: iface.clone(),
            is_connected: false,
            ..Default::default()
        };

        // 1. Attempt with `iw dev <iface> link` command
        if let Ok(output) = Command::new("iw").args(["dev", &iface, "link"]).output() {
            let text = String::from_utf8_lossy(&output.stdout);
            if text.contains("Connected to") {
                status.is_connected = true;

                for line in text.lines() {
                    let trimmed = line.trim();
                    if let Some(rest) = trimmed.strip_prefix("SSID:") {
                        status.ssid = Some(rest.trim().to_string());
                    } else if let Some(rest) = trimmed.strip_prefix("freq:") {
                        if let Ok(freq) = rest.trim().parse::<f32>() {
                            status.frequency_mhz = Some(freq);
                        }
                    } else if let Some(rest) = trimmed.strip_prefix("signal:") {
                        let parts: Vec<&str> = rest.split_whitespace().collect();
                        if let Some(dbm_str) = parts.first() {
                            if let Ok(dbm) = dbm_str.parse::<i32>() {
                                status.signal_dbm = Some(dbm);
                                status.signal_percent = Self::dbm_to_percent(dbm);
                            }
                        }
                    } else if let Some(rest) = trimmed.strip_prefix("tx bitrate:") {
                        let parts: Vec<&str> = rest.split_whitespace().collect();
                        if let Some(rate_str) = parts.first() {
                            if let Ok(rate) = rate_str.parse::<f32>() {
                                status.bitrate_mbps = Some(rate);
                            }
                        }
                    }
                }
            }
        }

        // 2. Fallback: Parse /proc/net/wireless if no signal obtained from `iw`
        if status.signal_dbm.is_none() {
            if let Ok(wireless_content) = fs::read_to_string("/proc/net/wireless") {
                for line in wireless_content.lines().skip(2) {
                    if line.contains(&iface) {
                        let fields: Vec<&str> = line.split_whitespace().collect();
                        if fields.len() >= 4 {
                            // Column 3 is signal level
                            let level_str = fields[3].trim_end_matches('.');
                            if let Ok(level) = level_str.parse::<i32>() {
                                status.signal_dbm = Some(level);
                                status.signal_percent = Self::dbm_to_percent(level);
                                status.is_connected = true;
                            }
                        }
                    }
                }
            }
        }

        if status.is_connected {
            Some(status)
        } else {
            None
        }
    }

    /// Detects and parses Ethernet wired interface information
    pub fn detect_ethernet(iface: &str) -> Option<EthernetStatus> {
        let path = format!("/sys/class/net/{}", iface);
        let speed_path = format!("{}/speed", path);
        let duplex_path = format!("{}/duplex", path);
        let operstate_path = format!("{}/operstate", path);

        let speed = fs::read_to_string(&speed_path)
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok());

        let duplex = fs::read_to_string(&duplex_path)
            .ok()
            .map(|s| s.trim() == "full")
            .unwrap_or(false);

        let is_up = fs::read_to_string(&operstate_path)
            .ok()
            .map(|s| s.trim() == "up")
            .unwrap_or(false);

        // If it is a wired interface (not wireless) and active
        if !iface.starts_with("wl") && (speed.is_some() || is_up) {
            Some(EthernetStatus {
                interface: iface.to_string(),
                speed_mbps: speed,
                is_full_duplex: duplex,
                is_up,
            })
        } else {
            None
        }
    }

    /// Converts dBm (-100 to -30) to readable percentage (0% to 100%)
    pub fn dbm_to_percent(dbm: i32) -> u8 {
        if dbm >= -50 {
            100
        } else if dbm <= -100 {
            0
        } else {
            // Between -100 and -50 there is 50 dBm range -> each dBm equals 2%
            (2 * (dbm + 100)).clamp(0, 100) as u8
        }
    }

    /// Formats bytes per second to readable text (B/s, KB/s, MB/s, GB/s)
    pub fn format_speed(bps: f64) -> String {
        if bps < 1024.0 {
            format!("{:.0} B/s", bps)
        } else if bps < 1024.0 * 1024.0 {
            format!("{:.1} KB/s", bps / 1024.0)
        } else if bps < 1024.0 * 1024.0 * 1024.0 {
            format!("{:.2} MB/s", bps / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB/s", bps / (1024.0 * 1024.0 * 1024.0))
        }
    }

    /// Formats cumulative bytes to readable text (KB, MB, GB)
    pub fn format_bytes(bytes: u64) -> String {
        let b = bytes as f64;
        if b < 1024.0 {
            format!("{} B", bytes)
        } else if b < 1024.0 * 1024.0 {
            format!("{:.1} KB", b / 1024.0)
        } else if b < 1024.0 * 1024.0 * 1024.0 {
            format!("{:.1} MB", b / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", b / (1024.0 * 1024.0 * 1024.0))
        }
    }

    /// Total bytes downloaded during this session
    pub fn session_download_bytes(&self) -> u64 {
        self.current_rx_bytes.saturating_sub(self.initial_rx_bytes)
    }

    /// Total bytes uploaded during this session
    pub fn session_upload_bytes(&self) -> u64 {
        self.current_tx_bytes.saturating_sub(self.initial_tx_bytes)
    }
}
