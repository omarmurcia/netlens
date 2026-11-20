use std::fs;
use std::time::Instant;

/// Snapshot returned by asynchronous background telemetry sampling
#[derive(Debug, Clone)]
pub struct TelemetrySnapshot {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub sampled_at: Instant,
    pub wifi: Option<WifiStatus>,
    pub wifi_updated: bool,
    pub wifi_iface: Option<String>,
    pub ethernet: Option<EthernetStatus>,
    pub active_interface: String,
}

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
            if freq >= 5900.0 {
                "6 GHz (Wi-Fi 6E)".to_string()
            } else if freq >= 4900.0 {
                "5 GHz".to_string()
            } else if freq >= 2400.0 {
                "2.4 GHz".to_string()
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
    pub cached_wifi_iface: Option<String>,
}

impl Telemetry {
    pub fn new(active_interface: &str) -> Self {
        let (rx, tx) = Self::sample_total_traffic();
        let cached_wifi_iface = Self::find_wifi_interface();
        let wifi = cached_wifi_iface
            .as_deref()
            .and_then(Self::detect_wifi_proc_only);
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
            cached_wifi_iface,
        }
    }

    /// Discovers the first wireless network interface name from /sys/class/net
    pub fn find_wifi_interface() -> Option<String> {
        let entries = fs::read_dir("/sys/class/net").ok()?;
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let wireless_dir = entry.path().join("wireless");
            let phy_dir = entry.path().join("phy80211");
            if wireless_dir.exists() || phy_dir.exists() || name.starts_with("wl") {
                return Some(name);
            }
        }
        None
    }

    /// Collects a full telemetry snapshot off the UI thread (non-blocking)
    pub async fn collect_snapshot(
        active_interface: String,
        cached_wifi_iface: Option<String>,
        query_iw: bool,
    ) -> TelemetrySnapshot {
        let wifi_iface = match cached_wifi_iface {
            Some(iface) if std::path::Path::new(&format!("/sys/class/net/{}", iface)).exists() => {
                Some(iface)
            }
            _ => tokio::task::spawn_blocking(Self::find_wifi_interface)
                .await
                .ok()
                .flatten(),
        };

        let active_clone = active_interface.clone();
        let wifi_iface_clone = wifi_iface.clone();
        let (rx_bytes, tx_bytes, ethernet, proc_wifi) = tokio::task::spawn_blocking(move || {
            let (rx, tx) = Self::sample_total_traffic();
            let eth = Self::detect_ethernet(&active_clone);
            let proc_w = wifi_iface_clone
                .as_deref()
                .and_then(Self::detect_wifi_proc_only);
            (rx, tx, eth, proc_w)
        })
        .await
        .unwrap_or((0, 0, None, None));

        let (wifi, wifi_updated) = if query_iw {
            if let Some(ref iface) = wifi_iface {
                (Self::detect_wifi_iw_async(iface).await.or(proc_wifi), true)
            } else {
                (None, true)
            }
        } else {
            (proc_wifi, false)
        };

        TelemetrySnapshot {
            rx_bytes,
            tx_bytes,
            sampled_at: Instant::now(),
            wifi,
            wifi_updated,
            wifi_iface,
            ethernet,
            active_interface,
        }
    }

    /// Applies an asynchronously collected telemetry snapshot on the UI thread
    pub fn apply_snapshot(&mut self, snap: TelemetrySnapshot) {
        self.active_interface = snap.active_interface;
        self.cached_wifi_iface = snap.wifi_iface;
        self.ethernet = snap.ethernet;

        let elapsed = snap.sampled_at.duration_since(self.last_sample).as_secs_f64();
        if elapsed >= 0.5 {
            let rx_diff = snap.rx_bytes.saturating_sub(self.prev_rx_bytes);
            let tx_diff = snap.tx_bytes.saturating_sub(self.prev_tx_bytes);

            self.download_speed_bps = (rx_diff as f64) / elapsed;
            self.upload_speed_bps = (tx_diff as f64) / elapsed;

            self.prev_rx_bytes = snap.rx_bytes;
            self.prev_tx_bytes = snap.tx_bytes;
            self.current_rx_bytes = snap.rx_bytes;
            self.current_tx_bytes = snap.tx_bytes;
            self.last_sample = snap.sampled_at;
        }

        if snap.wifi_updated {
            self.wifi = snap.wifi;
        } else if let Some(proc_w) = snap.wifi {
            if let Some(existing) = &mut self.wifi {
                // Keep SSID/freq/bitrate from last `iw` query while refreshing live dBm from /proc/net/wireless
                existing.signal_dbm = proc_w.signal_dbm;
                existing.signal_percent = proc_w.signal_percent;
                existing.is_connected = proc_w.is_connected;
            } else {
                self.wifi = Some(proc_w);
            }
        }
    }

    /// Updates speed sampling and network status synchronously (for CLI/tests)
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

    /// Reads total transmitted and received bytes in /proc/net/dev without heap allocations per line
    pub fn sample_total_traffic() -> (u64, u64) {
        let mut total_rx = 0u64;
        let mut total_tx = 0u64;

        if let Ok(content) = fs::read_to_string("/proc/net/dev") {
            for line in content.lines().skip(2) {
                if let Some((iface, data)) = line.split_once(':') {
                    let iface_clean = iface.trim();
                    // Ignore local loopback and virtual container interfaces
                    if iface_clean != "lo" && !iface_clean.starts_with("docker") && !iface_clean.starts_with("veth") {
                        let mut fields = data.split_whitespace();
                        if let (Some(rx_s), Some(tx_s)) = (fields.next(), fields.nth(7)) {
                            total_rx += rx_s.parse::<u64>().unwrap_or(0);
                            total_tx += tx_s.parse::<u64>().unwrap_or(0);
                        }
                    }
                }
            }
        }

        (total_rx, total_tx)
    }

    fn parse_iw_output(iface: &str, text: &str) -> Option<WifiStatus> {
        if !text.contains("Connected to") {
            return None;
        }
        let mut status = WifiStatus {
            interface: iface.to_string(),
            is_connected: true,
            ..Default::default()
        };

        for line in text.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("SSID:") {
                status.ssid = Some(rest.trim().to_string());
            } else if let Some(rest) = trimmed.strip_prefix("freq:") {
                if let Ok(freq) = rest.trim().parse::<f32>() {
                    status.frequency_mhz = Some(freq);
                }
            } else if let Some(rest) = trimmed.strip_prefix("signal:") {
                if let Some(dbm_str) = rest.split_whitespace().next() {
                    if let Ok(dbm) = dbm_str.parse::<i32>() {
                        status.signal_dbm = Some(dbm);
                        status.signal_percent = Self::dbm_to_percent(dbm);
                    }
                }
            } else if let Some(rest) = trimmed.strip_prefix("tx bitrate:") {
                if let Some(rate_str) = rest.split_whitespace().next() {
                    if let Ok(rate) = rate_str.parse::<f32>() {
                        status.bitrate_mbps = Some(rate);
                    }
                }
            }
        }

        Some(status)
    }

    /// Reads live Wi-Fi signal directly from /proc/net/wireless without spawning processes
    pub fn detect_wifi_proc_only(iface: &str) -> Option<WifiStatus> {
        let wireless_content = fs::read_to_string("/proc/net/wireless").ok()?;
        for line in wireless_content.lines().skip(2) {
            if line.contains(iface) {
                if let Some(level_str) = line.split_whitespace().nth(3) {
                    let level_clean = level_str.trim_end_matches('.');
                    if let Ok(level) = level_clean.parse::<i32>() {
                        return Some(WifiStatus {
                            interface: iface.to_string(),
                            signal_dbm: Some(level),
                            signal_percent: Self::dbm_to_percent(level),
                            is_connected: true,
                            ..Default::default()
                        });
                    }
                }
            }
        }
        None
    }

    /// Asynchronous `iw dev <iface> link` query
    async fn detect_wifi_iw_async(iface: &str) -> Option<WifiStatus> {
        let output = tokio::process::Command::new("iw")
            .args(["dev", iface, "link"])
            .output()
            .await
            .ok()?;
        let text = String::from_utf8_lossy(&output.stdout);
        Self::parse_iw_output(iface, &text)
    }

    /// Detects and parses Wi-Fi signal information synchronously
    pub fn detect_wifi() -> Option<WifiStatus> {
        let iface = Self::find_wifi_interface()?;

        if let Ok(output) = std::process::Command::new("iw")
            .args(["dev", &iface, "link"])
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout);
            if let Some(status) = Self::parse_iw_output(&iface, &text) {
                if status.signal_dbm.is_some() {
                    return Some(status);
                }
                if let Some(proc_status) = Self::detect_wifi_proc_only(&iface) {
                    return Some(WifiStatus {
                        signal_dbm: proc_status.signal_dbm,
                        signal_percent: proc_status.signal_percent,
                        ..status
                    });
                }
                return Some(status);
            }
        }

        Self::detect_wifi_proc_only(&iface)
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
