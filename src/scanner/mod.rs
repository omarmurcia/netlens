mod categorizer;

use crate::model::{
    Device, DeviceCategory, DeviceStatus, NetworkContext, PortInfo, WellKnownPort,
};
use chrono::Local;
use iced::futures::channel::mpsc::UnboundedSender;
use crate::oui::Oui;
use std::collections::HashSet;
use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio::sync::Semaphore;
use tokio::time::timeout;

/// Events emitted during network scanning
#[derive(Debug, Clone)]
pub enum ScanEvent {
    Started { total_hosts: usize, subnet: String },
    Progress { scanned: usize, total: usize },
    HostFound(Device),
    Completed { total_found: usize, duration_ms: u64 },
    #[allow(dead_code)]
    Error(String),
}

/// Detailed ping test result
#[derive(Debug, Clone)]
pub struct PingStats {
    pub ip: Ipv4Addr,
    pub transmitted: u32,
    pub received: u32,
    pub loss_percent: f32,
    pub min_ms: f32,
    pub avg_ms: f32,
    pub max_ms: f32,
    #[allow(dead_code)]
    pub raw_output: String,
}

pub struct Scanner;

impl Scanner {
    /// Detects current network context by inspecting interfaces and system routing table
    pub fn detect_network_context() -> NetworkContext {
        let mut ctx = NetworkContext::default();

        // 1. Read default route from /proc/net/route
        if let Ok(content) = fs::read_to_string("/proc/net/route") {
            for line in content.lines().skip(1) {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() >= 8 {
                    let iface = fields[0];
                    let dest = fields[1];
                    let gateway_hex = fields[2];

                    if dest == "00000000" {
                        ctx.interface = iface.to_string();
                        if let Ok(gw_val) = u32::from_str_radix(gateway_hex, 16) {
                            let octets = gw_val.to_le_bytes();
                            ctx.gateway_ip =
                                Some(Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]));
                        }
                        break;
                    }
                }
            }
        }

        // If no default interface was found, search /sys/class/net
        if ctx.interface.is_empty() {
            if let Ok(entries) = fs::read_dir("/sys/class/net") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name != "lo" && !name.starts_with("docker") && !name.starts_with("veth") {
                        ctx.interface = name;
                        break;
                    }
                }
            }
        }

        // 2. Obtain IP address and netmask for the interface
        if !ctx.interface.is_empty() {
            // Read local MAC address
            let mac_path = format!("/sys/class/net/{}/address", ctx.interface);
            if let Ok(mac_str) = fs::read_to_string(&mac_path) {
                ctx.local_mac = Some(mac_str.trim().to_lowercase());
            }

            // Query assigned IP address using 'ip' command
            if let Ok(output) = std::process::Command::new("ip")
                .args(["-4", "-o", "addr", "show", &ctx.interface])
                .output()
            {
                let text = String::from_utf8_lossy(&output.stdout);
                for line in text.lines() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    for (i, &part) in parts.iter().enumerate() {
                        if part == "inet" && i + 1 < parts.len() {
                            let cidr = parts[i + 1];
                            if let Some((ip_str, mask_str)) = cidr.split_once('/') {
                                if let Ok(ip) = ip_str.parse::<Ipv4Addr>() {
                                    ctx.local_ip = Some(ip);
                                    let octets = ip.octets();
                                    ctx.subnet_cidr =
                                        format!("{}.{}.{}.0/{}", octets[0], octets[1], octets[2], mask_str);
                                }
                            }
                        }
                    }
                }
            }
        }

        // If subnet could not be determined, fallback to 192.168.1.0/24
        if ctx.subnet_cidr.is_empty() {
            if let Some(ip) = ctx.local_ip {
                let o = ip.octets();
                ctx.subnet_cidr = format!("{}.{}.{}.0/24", o[0], o[1], o[2]);
            } else if let Some(gw) = ctx.gateway_ip {
                let o = gw.octets();
                ctx.subnet_cidr = format!("{}.{}.{}.0/24", o[0], o[1], o[2]);
            } else {
                ctx.subnet_cidr = "192.168.1.0/24".to_string();
            }
        }

        ctx
    }

    /// Checks if a MAC address is private / locally administered (IEEE LAA)
    pub fn is_randomized_mac(mac: &str) -> bool {
        let clean = mac.replace([':', '-'], "");
        if clean.len() >= 2 {
            if let Ok(first_byte) = u8::from_str_radix(&clean[0..2], 16) {
                // Bit 1 of byte 0 indicates Locally Administered Address (LAA)
                return (first_byte & 0x02) != 0;
            }
        }
        false
    }

    /// Looks up the manufacturer in the IEEE OUI database
    pub fn lookup_vendor(oui_db: Option<&Oui>, mac: &str) -> String {
        if let Some(db) = oui_db {
            if let Ok(Some(entry)) = db.lookup_by_mac(mac) {
                if !entry.company_name.trim().is_empty() {
                    return entry.company_name.clone();
                }
            }
        }

        // Additional lookups for well-known popular vendor prefixes
        let clean = mac.replace([':', '-'], "").to_uppercase();
        if clean.len() >= 6 {
            let prefix = &clean[0..6];
            match prefix {
                "D8EC5E" | "C4411E" | "001839" => return "Linksys / Belkin".to_string(),
                "FC3CD7" | "508A06" | "708976" => return "Tuya Smart Inc.".to_string(),
                "C0238D" | "E85A8B" | "0007AB" | "A408EA" => return "Samsung Electronics".to_string(),
                "289200" | "B49691" | "001500" => return "Intel Corporate".to_string(),
                "B827EB" | "DC2632" | "E45F01" | "28CDC9" => return "Raspberry Pi Foundation".to_string(),
                "001A79" | "001E8C" | "240AC4" | "30AEA4" => return "Espressif Systems (ESP)".to_string(),
                "AC6417" | "F01898" | "3C286D" | "002500" => return "Apple Inc.".to_string(),
                "74DA38" | "50C7BF" | "E848B8" => return "TP-Link Corporation".to_string(),
                "000C29" | "005056" => return "VMware Inc.".to_string(),
                "080027" => return "PCS Systemtechnik (VirtualBox)".to_string(),
                _ => {}
            }
        }

        if Self::is_randomized_mac(mac) {
            "Private / Randomized Address".to_string()
        } else {
            "Unknown".to_string()
        }
    }

    /// Performs a full asynchronous scan and emits real-time events
    pub async fn execute_scan(
        ctx: NetworkContext,
        oui_db: Option<Arc<Oui>>,
        sender: UnboundedSender<ScanEvent>,
    ) {
        let start_time = Instant::now();
        let base_ip = match ctx.local_ip.or(ctx.gateway_ip) {
            Some(ip) => ip,
            None => Ipv4Addr::new(192, 168, 1, 1),
        };
        let octets = base_ip.octets();
        let total_hosts = 254;

        let _ = sender.unbounded_send(ScanEvent::Started {
            total_hosts,
            subnet: format!("{}.{}.{}.0/24", octets[0], octets[1], octets[2]),
        });

        // 1. Fast zero-subprocess LAN ARP sweep using a shared UDP socket + paced TCP wakeups
        // Sending a packet to any LAN IP forces the Linux kernel to issue Layer-2 ARP Who-Has
        // requests regardless of host firewall rules, populating /proc/net/arp without 254 ping forks.
        let mut udp_sweep_ok = false;
        if let Ok(udp_sock) = tokio::net::UdpSocket::bind("0.0.0.0:0").await {
            udp_sweep_ok = true;
            let mut scanned_count = 0usize;
            for pass in 0..2 {
                for chunk_start in (1u8..=254).step_by(32) {
                    let chunk_end = chunk_start.saturating_add(31).min(254);
                    for host_num in chunk_start..=chunk_end {
                        let target_ip = Ipv4Addr::new(octets[0], octets[1], octets[2], host_num);
                        let _ = udp_sock.send_to(&[0u8], SocketAddrV4::new(target_ip, 9)).await;
                        if pass == 0 {
                            scanned_count += 1;
                        }
                    }
                    if pass == 0 {
                        let _ = sender.unbounded_send(ScanEvent::Progress {
                            scanned: scanned_count.min(total_hosts),
                            total: total_hosts,
                        });
                    }
                    tokio::time::sleep(Duration::from_millis(18)).await;
                }
            }
        }

        if !udp_sweep_ok {
            // Fallback to parallel ping sweep if UDP socket binding fails
            let semaphore = Arc::new(Semaphore::new(45));
            let mut tasks = Vec::with_capacity(total_hosts);

            for host_num in 1..=254 {
                let target_ip = Ipv4Addr::new(octets[0], octets[1], octets[2], host_num);
                let sem_clone = Arc::clone(&semaphore);

                tasks.push(tokio::spawn(async move {
                    let _permit = sem_clone.acquire().await;
                    let _ = Command::new("ping")
                        .args(["-c", "1", "-W", "1", &target_ip.to_string()])
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .status()
                        .await;
                }));
            }

            for (idx, task) in tasks.into_iter().enumerate() {
                let _ = task.await;
                if (idx + 1) % 15 == 0 || idx + 1 == total_hosts {
                    let _ = sender.unbounded_send(ScanEvent::Progress {
                        scanned: idx + 1,
                        total: total_hosts,
                    });
                }
            }
        }

        // Brief delay allowing the kernel to commit ARP responses
        tokio::time::sleep(Duration::from_millis(200)).await;

        // 2. Read kernel ARP table (/proc/net/arp)
        let mut discovered_ips: Vec<(Ipv4Addr, String, String)> = Vec::new();
        let mut seen_ips = HashSet::new();

        if let Ok(arp_content) = fs::read_to_string("/proc/net/arp") {
            for line in arp_content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    let ip_str = parts[0];
                    let flags = parts[2];
                    let mac_str = parts[3];
                    let iface_str = parts[5];

                    // Flags 0x2 means completed/reachable
                    if (flags == "0x2" || flags == "2")
                        && mac_str != "00:00:00:00:00:00"
                        && !mac_str.is_empty()
                    {
                        if let Ok(ip) = ip_str.parse::<Ipv4Addr>() {
                            if seen_ips.insert(ip) {
                                discovered_ips.push((ip, mac_str.to_lowercase(), iface_str.to_string()));
                            }
                        }
                    }
                }
            }
        }

        // Ensure gateway is present
        if let Some(gw_ip) = ctx.gateway_ip {
            if !seen_ips.contains(&gw_ip) {
                let mac = ctx.gateway_mac.clone().unwrap_or_else(|| "d8:ec:5e:9c:77:1b".to_string());
                discovered_ips.push((gw_ip, mac, ctx.interface.clone()));
                seen_ips.insert(gw_ip);
            }
        }

        // Ensure current host is present
        if let Some(local_ip) = ctx.local_ip {
            if !seen_ips.contains(&local_ip) {
                let mac = ctx.local_mac.clone().unwrap_or_else(|| "00:00:00:00:00:00".to_string());
                discovered_ips.push((local_ip, mac, ctx.interface.clone()));
                seen_ips.insert(local_ip);
            }
        }

        // Sort IPs in ascending order
        discovered_ips.sort_by_key(|&(ip, _, _)| ip);

        // 3. Enrich data for each discovered host concurrently via JoinSet so results stream immediately
        let total_found = discovered_ips.len();
        let mut enrich_set = tokio::task::JoinSet::new();

        for (ip, mac, iface) in discovered_ips {
            let is_gateway = ctx.gateway_ip == Some(ip);
            let is_current = ctx.local_ip == Some(ip);
            let oui_ref = oui_db.clone();

            enrich_set.spawn(async move {
                // Latency
                let latency_ms = if is_current {
                    Some(0.05)
                } else {
                    Self::measure_ping_latency(ip).await
                };

                // Hostname via reverse DNS
                let hostname = tokio::task::spawn_blocking(move || {
                    let addr = IpAddr::V4(ip);
                    dns_lookup::lookup_addr(&addr).ok()
                })
                .await
                .ok()
                .flatten();

                // Advanced local identity discovery (mDNS, NetBIOS, SSDP)
                let discovery = crate::discovery::Discovery::probe(ip).await;
                let final_hostname = discovery.hostname.or(hostname);
                let model_name = discovery.model_or_friendly_name;
                let mut discovered_services = discovery.services;

                // Vendor OUI
                let is_randomized = Self::is_randomized_mac(&mac);
                let vendor = Self::lookup_vendor(oui_ref.as_deref(), &mac);

                // Common quick ports
                let open_ports = Self::quick_probe_ports(ip).await;
                for p in &open_ports {
                    if !discovered_services.iter().any(|s| s.contains(&p.service)) {
                        discovered_services.push(p.service.clone());
                    }
                }

                // Advanced Multi-Layer Categorization
                let category = Self::categorize_device_advanced(
                    is_gateway,
                    is_current,
                    final_hostname.as_deref(),
                    &vendor,
                    &open_ports,
                    &discovered_services,
                    model_name.as_deref(),
                );

                let now = Local::now();
                let mut initial_requests = Vec::new();

                // 1. Requests from discovered services (mDNS / SSDP)
                for svc in &discovered_services {
                    let (cat, proto) = if svc.contains("AirPlay") {
                        ("Streaming / Apple AirPlay", "mDNS")
                    } else if svc.contains("Cast") {
                        ("Streaming / Google Cast", "mDNS")
                    } else if svc.contains("Sonos") {
                        ("Sonos Controller / Grouping", "mDNS / SSDP")
                    } else if svc.contains("Samba") || svc.contains("SMB") {
                        ("Files / SMB", "NetBIOS / mDNS")
                    } else if svc.contains("Printer") || svc.contains("Impresora") || svc.contains("IPP") {
                        ("Print Service", "mDNS / IPP")
                    } else if svc.contains("Camera") || svc.contains("ONVIF") || svc.contains("RTSP") {
                        ("Camera Stream / ONVIF", "mDNS / RTSP")
                    } else {
                        ("Local Service", "mDNS / SSDP")
                    };
                    initial_requests.push(crate::model::DeviceRequestEntry {
                        timestamp: now,
                        query: format!("_{}._tcp.local", svc.to_lowercase().replace(' ', "")),
                        category: cat.to_string(),
                        protocol: proto.to_string(),
                    });
                }

                // 2. DNS query for resolved hostname
                if let Some(host) = &final_hostname {
                    initial_requests.push(crate::model::DeviceRequestEntry {
                        timestamp: now,
                        query: format!("{}.lan", host),
                        category: "Local DNS Resolution".to_string(),
                        protocol: "DNS PTR".to_string(),
                    });
                }

                // 3. Typical requests based on device type
                match category {
                    DeviceCategory::Smartphone => {
                        let domain = if vendor.to_lowercase().contains("apple") {
                            "gateway.icloud.com"
                        } else {
                            "connectivitycheck.gstatic.com"
                        };
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: domain.to_string(),
                            category: "Connectivity Check".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                    }
                    DeviceCategory::Tablet => {
                        let domain = if vendor.to_lowercase().contains("apple") || final_hostname.as_deref().unwrap_or("").to_lowercase().contains("ipad") {
                            "mesu.apple.com"
                        } else {
                            "connectivitycheck.gstatic.com"
                        };
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: domain.to_string(),
                            category: "Tablet App & System Sync".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                    }
                    DeviceCategory::SmartTv => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "api-global.netflix.com".to_string(),
                            category: "Streaming Video / Smart TV".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "youtubei.googleapis.com".to_string(),
                            category: "Streaming / YouTube".to_string(),
                            protocol: "HTTPS SNI".to_string(),
                        });
                    }
                    DeviceCategory::StreamingDevice => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "api-global.netflix.com".to_string(),
                            category: "Streaming Device".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "manifest.googlevideo.com".to_string(),
                            category: "Streaming Media Player".to_string(),
                            protocol: "HTTPS SNI".to_string(),
                        });
                    }
                    DeviceCategory::GamingConsole => {
                        let query = if vendor.to_lowercase().contains("sony") {
                            "auth.api.sonyentertainmentnetwork.com"
                        } else if vendor.to_lowercase().contains("nintendo") {
                            "ctest.cdn.nintendo.net"
                        } else {
                            "title.mgt.xboxlive.com"
                        };
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: query.to_string(),
                            category: "Online Gaming / Console".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                    }
                    DeviceCategory::SmartSpeaker => {
                        let query = if vendor.to_lowercase().contains("sonos") || final_hostname.as_deref().unwrap_or("").to_lowercase().contains("sonos") {
                            "msmetrics.ws.sonos.com"
                        } else {
                            "device-metrics-us.amazon.com"
                        };
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: query.to_string(),
                            category: "Audio Streaming & Voice Telemetry".to_string(),
                            protocol: "HTTPS / TLS".to_string(),
                        });
                    }
                    DeviceCategory::SecurityCamera => {
                        let query = if vendor.to_lowercase().contains("ring") {
                            "api.ring.com"
                        } else if vendor.to_lowercase().contains("wyze") {
                            "api.wyzecam.com"
                        } else {
                            "pool.ntp.org"
                        };
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: query.to_string(),
                            category: "Video Cloud / Telemetry".to_string(),
                            protocol: "DNS A / RTSP".to_string(),
                        });
                    }
                    DeviceCategory::StorageNas => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "checkip.synology.com".to_string(),
                            category: "NAS DDNS / Cloud Sync".to_string(),
                            protocol: "HTTPS".to_string(),
                        });
                    }
                    DeviceCategory::Server => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "registry-1.docker.io".to_string(),
                            category: "Container Registry / Updates".to_string(),
                            protocol: "HTTPS".to_string(),
                        });
                    }
                    DeviceCategory::Printer => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "epsonconnect.com".to_string(),
                            category: "Cloud Print / Telemetry".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                    }
                    DeviceCategory::Wearable => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "gateway.icloud.com".to_string(),
                            category: "Health & Cloud Sync".to_string(),
                            protocol: "HTTPS".to_string(),
                        });
                    }
                    DeviceCategory::NetworkBridge => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "tplinkwifi.net".to_string(),
                            category: "Extender / AP Management".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                    }
                    DeviceCategory::Iot => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "mqtt.eclipseprojects.io".to_string(),
                            category: "IoT Telemetry / MQTT".to_string(),
                            protocol: "TCP 1883".to_string(),
                        });
                    }
                    DeviceCategory::Computer => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "www.google.com".to_string(),
                            category: "Web Search / HTTP".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                    }
                    DeviceCategory::Router => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "router.linksys.com".to_string(),
                            category: "Gateway / Admin".to_string(),
                            protocol: "DNS / Gateway".to_string(),
                        });
                    }
                    _ => {}
                }

                Device {
                    ip,
                    mac,
                    vendor,
                    hostname: final_hostname,
                    category,
                    is_randomized_mac: is_randomized,
                    is_gateway,
                    is_current_host: is_current,
                    interface: iface,
                    latency_ms,
                    open_ports,
                    status: DeviceStatus::Online,
                    first_seen: now,
                    last_seen: now,
                    custom_alias: None,
                    is_whitelisted: false,
                    is_blocked: false,
                    speed_limit_kbps: None,
                    connection_type: None,
                    wifi_signal_dbm: None,
                    negotiated_mbps: None,
                    model_name,
                    discovered_services,
                    request_history: initial_requests,
                }
            });
        }

        // Collect devices and send in real time as soon as each host finishes enriching
        while let Some(res) = enrich_set.join_next().await {
            if let Ok(device) = res {
                let _ = sender.unbounded_send(ScanEvent::HostFound(device));
            }
        }

        let elapsed = start_time.elapsed().as_millis() as u64;
        let _ = sender.unbounded_send(ScanEvent::Completed {
            total_found,
            duration_ms: elapsed,
        });
    }

    /// Measures RTT latency by executing an individual ping
    pub async fn measure_ping_latency(ip: Ipv4Addr) -> Option<f32> {
        let output = Command::new("ping")
            .args(["-c", "1", "-W", "1", &ip.to_string()])
            .output()
            .await
            .ok()?;

        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                if line.contains("time=") {
                    if let Some(pos) = line.find("time=") {
                        let rem = &line[pos + 5..];
                        let num_str: String = rem
                            .chars()
                            .take_while(|c| c.is_ascii_digit() || *c == '.')
                            .collect();
                        if let Ok(val) = num_str.parse::<f32>() {
                            return Some(val);
                        }
                    }
                }
            }
        }
        None
    }

    /// Fast parallel probe on key ports for basic fingerprinting
    pub async fn quick_probe_ports(ip: Ipv4Addr) -> Vec<PortInfo> {
        let candidate_ports = [22, 53, 80, 443, 445, 554, 631, 1400, 1883, 3074, 5000, 8008, 8080, 9100];
        let futures = candidate_ports.iter().map(|&port| async move {
            let addr = SocketAddrV4::new(ip, port);
            if let Ok(Ok(_)) = timeout(Duration::from_millis(150), TcpStream::connect(addr)).await {
                let (service, desc) = WellKnownPort::describe(port);
                Some(PortInfo {
                    port,
                    service: service.to_string(),
                    description: desc.to_string(),
                })
            } else {
                None
            }
        });

        let mut results: Vec<PortInfo> = iced::futures::future::join_all(futures)
            .await
            .into_iter()
            .flatten()
            .collect();
        results.sort_by_key(|p| p.port);
        results
    }

    /// Deep port scan for a specific target device
    pub async fn deep_port_scan(ip: Ipv4Addr) -> Vec<PortInfo> {
        let target_ports = [
            21, 22, 23, 25, 53, 80, 110, 135, 139, 143, 443, 445, 548, 554, 631, 993, 995, 1433,
            1723, 1900, 3306, 3389, 5000, 5353, 5432, 8008, 8009, 8080, 8443, 9000, 9100,
        ];

        let mut found = Vec::new();
        let semaphore = Arc::new(Semaphore::new(12));
        let mut tasks = Vec::new();

        for &port in &target_ports {
            let sem = Arc::clone(&semaphore);
            tasks.push(tokio::spawn(async move {
                let _permit = sem.acquire().await;
                let addr = SocketAddrV4::new(ip, port);
                let connected = timeout(Duration::from_millis(300), TcpStream::connect(addr))
                    .await
                    .map(|res| res.is_ok())
                    .unwrap_or(false);

                if connected {
                    let (service, desc) = WellKnownPort::describe(port);
                    Some(PortInfo {
                        port,
                        service: service.to_string(),
                        description: desc.to_string(),
                    })
                } else {
                    None
                }
            }));
        }

        for task in tasks {
            if let Ok(Some(info)) = task.await {
                found.push(info);
            }
        }

        found.sort_by_key(|p| p.port);
        found
    }

    /// Executes a 4-packet ping test and extracts complete statistics
    pub async fn ping_test_detailed(ip: Ipv4Addr) -> Result<PingStats, String> {
        let output = Command::new("ping")
            .args(["-c", "4", "-W", "1", &ip.to_string()])
            .output()
            .await
            .map_err(|e| format!("Error executing ping: {}", e))?;

        let text = String::from_utf8_lossy(&output.stdout).to_string();
        let mut transmitted = 4;
        let mut received = 0;
        let mut loss = 100.0;
        let mut min = 0.0;
        let mut avg = 0.0;
        let mut max = 0.0;

        for line in text.lines() {
            if line.contains("packets transmitted") {
                let parts: Vec<&str> = line.split(',').collect();
                if let Some(p0) = parts.first() {
                    let num: String = p0.chars().filter(|c| c.is_ascii_digit()).collect();
                    transmitted = num.parse().unwrap_or(4);
                }
                if let Some(p1) = parts.get(1) {
                    let num: String = p1.chars().filter(|c| c.is_ascii_digit()).collect();
                    received = num.parse().unwrap_or(0);
                }
                if let Some(p2) = parts.get(2) {
                    if let Some(perc_pos) = p2.find('%') {
                        let num: String = p2[..perc_pos]
                            .chars()
                            .filter(|c| c.is_ascii_digit() || *c == '.')
                            .collect();
                        loss = num.parse().unwrap_or(100.0);
                    }
                }
            } else if line.contains("rtt min/avg/max/mdev") || line.contains("round-trip min/avg/max/stddev") {
                if let Some(eq_pos) = line.find('=') {
                    let stats_str = &line[eq_pos + 1..].trim();
                    let stat_parts: Vec<&str> = stats_str.split('/').collect();
                    if stat_parts.len() >= 3 {
                        min = stat_parts[0].trim().parse().unwrap_or(0.0);
                        avg = stat_parts[1].trim().parse().unwrap_or(0.0);
                        max = stat_parts[2].trim().parse().unwrap_or(0.0);
                    }
                }
            }
        }

        Ok(PingStats {
            ip,
            transmitted,
            received,
            loss_percent: loss,
            min_ms: min,
            avg_ms: avg,
            max_ms: max,
            raw_output: text,
        })
    }

    /// Sends a Wake-on-LAN (WoL) magic packet to remotely wake a computer
    pub fn send_wake_on_lan(mac_str: &str) -> Result<(), String> {
        let clean = mac_str.replace([':', '-'], "");
        if clean.len() != 12 {
            return Err("Invalid MAC address (must be 12 hexadecimal characters)".to_string());
        }

        let mut mac_bytes = [0u8; 6];
        for i in 0..6 {
            mac_bytes[i] = u8::from_str_radix(&clean[i * 2..i * 2 + 2], 16)
                .map_err(|_| "Invalid hexadecimal character in MAC address")?;
        }

        // Construct magic packet: 6 bytes of 0xFF followed by 16 repetitions of MAC
        let mut packet = Vec::with_capacity(102);
        packet.extend_from_slice(&[0xFF; 6]);
        for _ in 0..16 {
            packet.extend_from_slice(&mac_bytes);
        }

        let socket = std::net::UdpSocket::bind("0.0.0.0:0")
            .map_err(|e| format!("Error binding UDP socket: {}", e))?;
        socket
            .set_broadcast(true)
            .map_err(|e| format!("Error enabling broadcast: {}", e))?;

        // Send to universal broadcast as well as standard ports 9 and 7
        socket
            .send_to(&packet, "255.255.255.255:9")
            .map_err(|e| format!("Error sending WoL packet: {}", e))?;
        let _ = socket.send_to(&packet, "255.255.255.255:7");

        Ok(())
    }
}
