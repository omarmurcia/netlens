use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;

/// Discovered identity information via local network protocols (mDNS, NetBIOS, SSDP)
#[derive(Debug, Clone, Default)]
pub struct DiscoveredIdentity {
    pub model_or_friendly_name: Option<String>,
    pub hostname: Option<String>,
    pub services: Vec<String>,
}

pub struct Discovery;

impl Discovery {
    /// Attempts to enrich device identity by querying mDNS, NetBIOS, and SSDP
    pub async fn probe(ip: Ipv4Addr) -> DiscoveredIdentity {
        let mut identity = DiscoveredIdentity::default();

        // 1. Attempt NetBIOS Name Service query (UDP 137)
        if let Ok(Ok(Some(netbios_name))) = timeout(Duration::from_millis(350), Self::probe_netbios(ip)).await {
            identity.model_or_friendly_name = Some(netbios_name.clone());
            identity.hostname = Some(netbios_name);
            identity.services.push("NetBIOS / SMB".to_string());
        }

        // 2. Attempt mDNS PTR query (UDP 5353)
        if let Ok(Ok(Some(mdns_name))) = timeout(Duration::from_millis(350), Self::probe_mdns_ptr(ip)).await {
            let clean_name = mdns_name.trim_end_matches(".local").replace(".local.", "");
            if identity.model_or_friendly_name.is_none() {
                identity.model_or_friendly_name = Some(clean_name.clone());
            }
            if identity.hostname.is_none() {
                identity.hostname = Some(clean_name);
            }
            if !identity.services.contains(&"mDNS / ZeroConf".to_string()) {
                identity.services.push("mDNS / ZeroConf".to_string());
            }
        }

        // 3. Attempt SSDP query (UDP 1900)
        if let Ok(Ok(Some((friendly_name, server_tag)))) = timeout(Duration::from_millis(350), Self::probe_ssdp(ip)).await {
            if let Some(fname) = friendly_name {
                identity.model_or_friendly_name = Some(fname);
            }
            if let Some(srv) = server_tag {
                if !identity.services.contains(&srv) {
                    identity.services.push(srv);
                }
            }
        }

        identity
    }

    /// Queries NetBIOS Node Status (NBSTAT) on UDP port 137
    async fn probe_netbios(ip: Ipv4Addr) -> Result<Option<String>, ()> {
        let socket = UdpSocket::bind("0.0.0.0:0").await.map_err(|_| ())?;
        let target = SocketAddrV4::new(ip, 137);

        // Standard NetBIOS Node Status Request packet (50 bytes)
        // Query "*" encoded in base32 half-ASCII (RFC 1002)
        #[rustfmt::skip]
        let query: [u8; 50] = [
            0x82, 0x28, // Transaction ID
            0x00, 0x00, // Flags: Query
            0x00, 0x01, // Questions: 1
            0x00, 0x00, // Answer RRs: 0
            0x00, 0x00, // Authority RRs: 0
            0x00, 0x00, // Additional RRs: 0
            // Query Name: 0x20 (32 bytes)
            0x20,
            0x43, 0x4b, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x00,       // Terminator
            0x00, 0x21, // Type: NBSTAT (0x0021)
            0x00, 0x01, // Class: IN (0x0001)
        ];

        socket.send_to(&query, target).await.map_err(|_| ())?;

        let mut buf = [0u8; 1024];
        let (len, from) = socket.recv_from(&mut buf).await.map_err(|_| ())?;
        if from.ip() != std::net::IpAddr::V4(ip) || len < 57 {
            return Ok(None);
        }

        // In NetBIOS response, number of names starts at byte 56
        let num_names = buf[56] as usize;
        let mut offset = 57;

        for _ in 0..num_names {
            if offset + 18 > len {
                break;
            }
            let name_bytes = &buf[offset..offset + 15];
            let name_type = buf[offset + 15];

            // Type 0x00 = Workstation/Machine Name, Type 0x20 = Server
            if name_type == 0x00 || name_type == 0x20 {
                if let Ok(name_str) = std::str::from_utf8(name_bytes) {
                    let trimmed = name_str.trim();
                    if !trimmed.is_empty()
                        && !trimmed.starts_with("IS~")
                        && !trimmed.starts_with("__MSBROWSE__")
                    {
                        return Ok(Some(trimmed.to_string()));
                    }
                }
            }

            offset += 18;
        }

        Ok(None)
    }

    /// Reverse mDNS query (PTR) for <ip-reversed>.in-addr.arpa on UDP 5353
    async fn probe_mdns_ptr(ip: Ipv4Addr) -> Result<Option<String>, ()> {
        let socket = UdpSocket::bind("0.0.0.0:0").await.map_err(|_| ())?;
        let target = SocketAddrV4::new(ip, 5353);

        // Build reverse name: 1.168.192.in-addr.arpa
        let octets = ip.octets();
        let labels = [
            octets[3].to_string(),
            octets[2].to_string(),
            octets[1].to_string(),
            octets[0].to_string(),
            "in-addr".to_string(),
            "arpa".to_string(),
        ];

        let mut packet = Vec::with_capacity(64);
        // Header
        packet.extend_from_slice(&[
            0x12, 0x34, // ID
            0x00, 0x00, // Flags (Standard Query)
            0x00, 0x01, // 1 Question
            0x00, 0x00, // 0 Answers
            0x00, 0x00, // 0 Auth
            0x00, 0x00, // 0 Addr
        ]);

        // Questions
        for label in &labels {
            packet.push(label.len() as u8);
            packet.extend_from_slice(label.as_bytes());
        }
        packet.push(0x00); // Terminating null byte
        packet.extend_from_slice(&[0x00, 0x0c]); // Type: PTR (12)
        packet.extend_from_slice(&[0x00, 0x01]); // Class: IN (1)

        socket.send_to(&packet, target).await.map_err(|_| ())?;

        let mut buf = [0u8; 1024];
        let (len, from) = socket.recv_from(&mut buf).await.map_err(|_| ())?;
        if from.ip() != std::net::IpAddr::V4(ip) || len < 12 {
            return Ok(None);
        }

        // Search for readable strings ending in .local
        let body = &buf[12..len];
        if let Some(name) = Self::extract_local_name(body) {
            return Ok(Some(name));
        }

        Ok(None)
    }

    /// Extracts a hostname from DNS bytes ending in .local
    fn extract_local_name(bytes: &[u8]) -> Option<String> {
        let text = String::from_utf8_lossy(bytes);
        for word in text.split(|c: char| !c.is_alphanumeric() && c != '-' && c != '_' && c != '.') {
            if word.ends_with(".local") && word.len() > 6 {
                return Some(word.to_string());
            }
        }
        None
    }

    /// SSDP M-SEARCH query (UDP 1900)
    async fn probe_ssdp(ip: Ipv4Addr) -> Result<Option<(Option<String>, Option<String>)>, ()> {
        let socket = UdpSocket::bind("0.0.0.0:0").await.map_err(|_| ())?;
        let target = SocketAddrV4::new(ip, 1900);

        let msearch = format!(
            "M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nMAN: \"ssdp:discover\"\r\nMX: 1\r\nST: ssdp:all\r\n\r\n"
        );

        socket.send_to(msearch.as_bytes(), target).await.map_err(|_| ())?;

        let mut buf = [0u8; 2048];
        let (len, _) = socket.recv_from(&mut buf).await.map_err(|_| ())?;
        let response = String::from_utf8_lossy(&buf[..len]);

        let mut friendly_name = None;
        let mut server_tag = None;

        for line in response.lines() {
            let lower = line.to_lowercase();
            if lower.starts_with("server:") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let srv = parts[1].trim();
                    server_tag = Some(srv.to_string());
                    if friendly_name.is_none() && (srv.contains("UPnP") || srv.contains("Linux") || srv.contains("Windows")) {
                        friendly_name = Some(srv.to_string());
                    }
                }
            }
        }

        Ok(Some((friendly_name, server_tag)))
    }
}
