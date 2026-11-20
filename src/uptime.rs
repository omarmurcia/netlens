use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutageType {
    IspInternetDown,   // Gateway responds, but Internet (public DNS) is down
    LocalNetworkDown,  // Neither local Gateway nor Internet responds
}

impl OutageType {
    pub fn label(&self) -> &'static str {
        match self {
            OutageType::IspInternetDown => "ISP / Internet Service Provider Outage",
            OutageType::LocalNetworkDown => "Local Network Disconnection (Wi-Fi / LAN)",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutageRecord {
    pub start_time: DateTime<Local>,
    pub end_time: Option<DateTime<Local>>,
    pub duration_secs: u64,
    pub outage_type: OutageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UptimeTracker {
    pub is_online: bool,
    pub consecutive_failures: u32,
    pub current_outage_start: Option<(DateTime<Local>, OutageType)>,
    pub outages: Vec<OutageRecord>,
    pub total_checks: u64,
    pub successful_checks: u64,
}

impl Default for UptimeTracker {
    fn default() -> Self {
        Self {
            is_online: true,
            consecutive_failures: 0,
            current_outage_start: None,
            outages: Vec::new(),
            total_checks: 0,
            successful_checks: 0,
        }
    }
}

impl UptimeTracker {
    /// Checks connectivity to the gateway and the Internet (Cloudflare/Google DNS) concurrently
    pub async fn probe_connectivity(gateway_ip: Option<Ipv4Addr>) -> (bool, bool) {
        let gw_fut = async {
            if let Some(gw) = gateway_ip {
                let (p80, p53) = tokio::join!(
                    Self::probe_tcp_quick(gw, 80),
                    Self::probe_tcp_quick(gw, 53),
                );
                if p80 || p53 {
                    true
                } else {
                    Self::probe_ping(gw).await
                }
            } else {
                false
            }
        };

        let internet_fut = async {
            let (cf, gg) = tokio::join!(
                Self::probe_tcp_quick(Ipv4Addr::new(1, 1, 1, 1), 53),
                Self::probe_tcp_quick(Ipv4Addr::new(8, 8, 8, 8), 53),
            );
            cf || gg
        };

        tokio::join!(gw_fut, internet_fut)
    }

    /// Quick TCP socket test with 350ms timeout
    async fn probe_tcp_quick(ip: Ipv4Addr, port: u16) -> bool {
        let addr = SocketAddrV4::new(ip, port);
        timeout(Duration::from_millis(350), TcpStream::connect(addr))
            .await
            .map(|res| res.is_ok())
            .unwrap_or(false)
    }

    /// Quick ping test
    async fn probe_ping(ip: Ipv4Addr) -> bool {
        tokio::process::Command::new("ping")
            .args(["-c", "1", "-W", "1", &ip.to_string()])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await
            .map(|s| s.success())
            .unwrap_or(false)
    }

    /// Records connectivity probe result and returns an event message on state change
    pub fn record_probe(&mut self, gw_ok: bool, internet_ok: bool) -> Option<(crate::history::NetworkEventType, String, String)> {
        self.total_checks += 1;
        let now = Local::now();

        if internet_ok {
            self.successful_checks += 1;
            self.consecutive_failures = 0;

            if !self.is_online {
                // Connection recovery after an outage
                self.is_online = true;
                if let Some((start, o_type)) = self.current_outage_start.take() {
                    let duration = (now - start).num_seconds().max(1) as u64;
                    let record = OutageRecord {
                        start_time: start,
                        end_time: Some(now),
                        duration_secs: duration,
                        outage_type: o_type,
                    };
                    self.outages.push(record);
                    if self.outages.len() > 50 {
                        self.outages.remove(0);
                    }

                    let dur_str = if duration < 60 {
                        format!("{}s", duration)
                    } else {
                        format!("{}m {}s", duration / 60, duration % 60)
                    };

                    return Some((
                        crate::history::NetworkEventType::InternetRestored,
                        "Internet Connection Restored".to_string(),
                        format!("Service recovered after an interruption of {}.", dur_str),
                    ));
                }
            }
        } else {
            self.consecutive_failures += 1;

            // At least 2 consecutive failures are required to confirm real outage and avoid false positives
            if self.consecutive_failures >= 2 && self.is_online {
                self.is_online = false;
                let o_type = if gw_ok {
                    OutageType::IspInternetDown
                } else {
                    OutageType::LocalNetworkDown
                };

                self.current_outage_start = Some((now, o_type));

                return Some((
                    crate::history::NetworkEventType::InternetOutage,
                    format!("Alert: {}", o_type.label()),
                    "Internet access has been interrupted on the local network.".to_string(),
                ));
            }
        }

        None
    }

    /// Estimated uptime percentage
    pub fn uptime_percentage(&self) -> f32 {
        if self.total_checks == 0 {
            100.0
        } else {
            (self.successful_checks as f32 / self.total_checks as f32 * 100.0).clamp(0.0, 100.0)
        }
    }
}
