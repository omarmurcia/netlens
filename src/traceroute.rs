use tokio::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub struct HopInfo {
    pub hop: u32,
    pub ip: Option<String>,
    pub hostname: Option<String>,
    pub latency_ms: Option<f32>,
    pub is_timeout: bool,
    pub label: Option<String>,
}

pub struct Traceroute;

impl Traceroute {
    /// Executes an asynchronous traceroute using tracepath without requiring root privileges
    pub async fn trace(target: &str) -> Result<Vec<HopInfo>, String> {
        #[cfg(windows)]
        let (output, tool) = (
            crate::procutil::NoWindow::no_window(Command::new("tracert"))
                .args(["-d", "-h", "10", "-w", "1000", target])
                .output()
                .await
                .map_err(|e| format!("Error executing tracert: {}", e))?,
            "tracert",
        );
        #[cfg(not(windows))]
        let (output, tool) = (
            crate::procutil::NoWindow::no_window(Command::new("tracepath"))
                .arg("-n")
                .arg("-m")
                .arg("10")
                .arg(target)
                .output()
                .await
                .map_err(|e| format!("Error executing tracepath: {}", e))?,
            "tracepath",
        );
        let _ = tool;

        let text = String::from_utf8_lossy(&output.stdout);
        #[cfg(windows)]
        let hops = Self::parse_tracert_output(&text, target);
        #[cfg(not(windows))]
        let hops = Self::parse_output(&text, target);
        if hops.is_empty() {
            return Err("No hops obtained from traceroute".to_string());
        }
        Ok(hops)
    }

    fn hop_label(hop_num: u32, ip: &str, target: &str) -> Option<String> {
        if hop_num == 1 && (ip.starts_with("192.168.") || ip.starts_with("10.")) {
            Some("Local Router (Gateway)".to_string())
        } else if hop_num == 2 && (ip.starts_with("192.168.") || ip.starts_with("10.")) {
            Some("Modem / ISP Gateway".to_string())
        } else if hop_num == 3 {
            Some("ISP Operator Node".to_string())
        } else if ip == target {
            Some("Final Destination".to_string())
        } else {
            None
        }
    }

    /// Parses textual output of Windows `tracert -d`:
    /// `  1    <1 ms    <1 ms    <1 ms  192.168.1.1` or `  2     *        *        *     Request timed out.`
    /// (locale independent: hops are detected by a leading number, probes by `*` / `N ms` tokens)
    pub fn parse_tracert_output(text: &str, target: &str) -> Vec<HopInfo> {
        let mut hops: Vec<HopInfo> = Vec::new();
        for line in text.lines() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let Some(first) = tokens.first() else { continue };
            let Ok(hop_num) = first.parse::<u32>() else { continue };

            // Last token that parses as an IPv4 address is the hop address
            let ip = tokens
                .iter()
                .rev()
                .find(|t| t.parse::<std::net::Ipv4Addr>().is_ok())
                .map(|t| t.to_string());

            // Latency: first number that is followed by an "ms" token, `<1` meaning sub-millisecond
            let mut latency_ms = None;
            for (i, t) in tokens.iter().enumerate().skip(1) {
                let next_is_ms = tokens.get(i + 1).map_or(false, |n| n.eq_ignore_ascii_case("ms"));
                let glued = t.to_lowercase().strip_suffix("ms").map(|s| s.to_string());
                let raw = if next_is_ms {
                    Some(t.to_string())
                } else {
                    glued
                };
                if let Some(raw) = raw {
                    let sub_ms = raw.starts_with('<');
                    if let Ok(v) = raw.trim_start_matches('<').parse::<f32>() {
                        latency_ms = Some(if sub_ms { 0.5 } else { v });
                        break;
                    }
                }
            }

            match ip {
                Some(ip) => {
                    let label = Self::hop_label(hop_num, &ip, target);
                    hops.push(HopInfo {
                        hop: hop_num,
                        ip: Some(ip),
                        hostname: None,
                        latency_ms,
                        is_timeout: false,
                        label,
                    });
                }
                None => hops.push(HopInfo {
                    hop: hop_num,
                    ip: None,
                    hostname: None,
                    latency_ms: None,
                    is_timeout: true,
                    label: None,
                }),
            }
        }
        hops
    }

    /// Parses textual output of `tracepath -n`
    pub fn parse_output(text: &str, target: &str) -> Vec<HopInfo> {
        let mut hops: Vec<HopInfo> = Vec::new();

        for line in text.lines() {
            let line = line.trim();
            // tracepath format: " 1:  192.168.1.1  1.293ms" or " 6:  no reply"
            if let Some(colon_pos) = line.find(':') {
                let hop_str = line[..colon_pos].trim();
                if let Ok(hop_num) = hop_str.parse::<u32>() {
                    let rest = line[colon_pos + 1..].trim();
                    if rest.contains("no reply") {
                        if !hops.iter().any(|h| h.hop == hop_num) {
                            hops.push(HopInfo {
                                hop: hop_num,
                                ip: None,
                                hostname: None,
                                latency_ms: None,
                                is_timeout: true,
                                label: None,
                            });
                        }
                    } else {
                        let parts: Vec<&str> = rest.split_whitespace().collect();
                        if !parts.is_empty() {
                            let ip = parts[0].to_string();
                            let mut latency_ms = None;
                            for p in &parts[1..] {
                                if p.ends_with("ms") {
                                    let num_str = p.trim_end_matches("ms");
                                    if let Ok(ms) = num_str.parse::<f32>() {
                                        latency_ms = Some(ms);
                                        break;
                                    }
                                }
                            }

                            let label = if hop_num == 1 && (ip.starts_with("192.168.") || ip.starts_with("10.")) {
                                Some("Local Router (Gateway)".to_string())
                            } else if hop_num == 2 && (ip.starts_with("192.168.") || ip.starts_with("10.")) {
                                Some("Modem / ISP Gateway".to_string())
                            } else if hop_num == 3 {
                                Some("ISP Operator Node".to_string())
                            } else if ip == target {
                                Some("Final Destination".to_string())
                            } else {
                                None
                            };

                            if let Some(existing) = hops.iter_mut().find(|h| h.hop == hop_num) {
                                if existing.latency_ms.is_none() && latency_ms.is_some() {
                                    existing.latency_ms = latency_ms;
                                    existing.ip = Some(ip);
                                    existing.label = label;
                                    existing.is_timeout = false;
                                }
                            } else {
                                hops.push(HopInfo {
                                    hop: hop_num,
                                    ip: Some(ip),
                                    hostname: None,
                                    latency_ms,
                                    is_timeout: false,
                                    label,
                                });
                            }
                        }
                    }
                }
            }
        }

        hops
    }
}
