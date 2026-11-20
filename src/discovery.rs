use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};
use tokio::time::timeout;

/// Discovered identity information via local network protocols (mDNS, NetBIOS, SSDP, HTTP/DIAL)
#[derive(Debug, Clone, Default)]
pub struct DiscoveredIdentity {
    pub model_or_friendly_name: Option<String>,
    pub hostname: Option<String>,
    pub services: Vec<String>,
}

pub struct Discovery;

impl Discovery {
    /// Attempts to enrich device identity by querying NetBIOS, mDNS, SSDP/UPnP, and HTTP/DIAL concurrently
    pub async fn probe(ip: Ipv4Addr) -> DiscoveredIdentity {
        let mut identity = DiscoveredIdentity::default();

        let (nb_res, mdns_res, ssdp_res, http_res) = tokio::join!(
            timeout(Duration::from_millis(300), Self::probe_netbios(ip)),
            timeout(Duration::from_millis(300), Self::probe_mdns_ptr(ip)),
            timeout(Duration::from_millis(300), Self::probe_ssdp(ip)),
            timeout(Duration::from_millis(250), Self::probe_http_banner(ip)),
        );

        // 1. NetBIOS Name Service query (UDP 137)
        if let Ok(Ok(Some(netbios_name))) = nb_res {
            identity.model_or_friendly_name = Some(netbios_name.clone());
            identity.hostname = Some(netbios_name);
            identity.services.push("NetBIOS / SMB".to_string());
        }

        // 2. mDNS PTR query & service detection (UDP 5353)
        if let Ok(Ok(Some((mdns_name, mdns_services)))) = mdns_res {
            if let Some(name) = mdns_name {
                let clean_name = name.trim_end_matches(".local").replace(".local.", "");
                if identity.model_or_friendly_name.is_none() {
                    identity.model_or_friendly_name = Some(clean_name.clone());
                }
                if identity.hostname.is_none() {
                    identity.hostname = Some(clean_name);
                }
            }
            for svc in mdns_services {
                if !identity.services.contains(&svc) {
                    identity.services.push(svc);
                }
            }
            if !identity.services.contains(&"mDNS / ZeroConf".to_string()) {
                identity.services.push("mDNS / ZeroConf".to_string());
            }
        }

        // 3. SSDP / UPnP M-SEARCH query (UDP 1900)
        if let Ok(Ok(Some((friendly_name, ssdp_services)))) = ssdp_res {
            if let Some(fname) = friendly_name {
                if identity.model_or_friendly_name.is_none() {
                    identity.model_or_friendly_name = Some(fname);
                }
            }
            for srv in ssdp_services {
                if !identity.services.contains(&srv) {
                    identity.services.push(srv);
                }
            }
        }

        // 4. Quick HTTP/DIAL service identification (Port 8008 Google Cast & Port 80 Web titles)
        if let Ok(Ok(Some((web_name, web_service)))) = http_res {
            if let Some(wname) = web_name {
                if identity.model_or_friendly_name.is_none() {
                    identity.model_or_friendly_name = Some(wname);
                }
            }
            if let Some(wsvc) = web_service {
                if !identity.services.contains(&wsvc) {
                    identity.services.push(wsvc);
                }
            }
        }

        identity
    }

    /// Queries NetBIOS Node Status (NBSTAT) on UDP port 137
    async fn probe_netbios(ip: Ipv4Addr) -> Result<Option<String>, ()> {
        let socket = UdpSocket::bind("0.0.0.0:0").await.map_err(|_| ())?;
        let target = SocketAddrV4::new(ip, 137);

        #[rustfmt::skip]
        let query: [u8; 50] = [
            0x82, 0x28, // Transaction ID
            0x00, 0x00, // Flags: Query
            0x00, 0x01, // Questions: 1
            0x00, 0x00, // Answer RRs: 0
            0x00, 0x00, // Authority RRs: 0
            0x00, 0x00, // Additional RRs: 0
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

    /// Reverse mDNS query (PTR) on UDP 5353 and service detection
    async fn probe_mdns_ptr(ip: Ipv4Addr) -> Result<Option<(Option<String>, Vec<String>)>, ()> {
        let socket = UdpSocket::bind("0.0.0.0:0").await.map_err(|_| ())?;
        let target = SocketAddrV4::new(ip, 5353);

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
        packet.extend_from_slice(&[
            0x12, 0x34, // ID
            0x00, 0x00, // Flags (Standard Query)
            0x00, 0x01, // 1 Question
            0x00, 0x00, // 0 Answers
            0x00, 0x00, // 0 Auth
            0x00, 0x00, // 0 Addr
        ]);

        for label in &labels {
            packet.push(label.len() as u8);
            packet.extend_from_slice(label.as_bytes());
        }
        packet.push(0x00);
        packet.extend_from_slice(&[0x00, 0x0c]); // Type: PTR
        packet.extend_from_slice(&[0x00, 0x01]); // Class: IN

        socket.send_to(&packet, target).await.map_err(|_| ())?;

        let mut buf = [0u8; 1500];
        let (len, from) = socket.recv_from(&mut buf).await.map_err(|_| ())?;
        if from.ip() != std::net::IpAddr::V4(ip) || len < 12 {
            return Ok(None);
        }

        let body = &buf[12..len];
        let raw_text = String::from_utf8_lossy(body).to_lowercase();
        let mut detected_services = Vec::new();

        if raw_text.contains("googlecast") || raw_text.contains("_googlecast") {
            detected_services.push("Streaming / Google Cast".to_string());
        }
        if raw_text.contains("airplay") || raw_text.contains("_airplay") {
            detected_services.push("Streaming / Apple AirPlay".to_string());
        }
        if raw_text.contains("raop") {
            detected_services.push("Audio / AirPlay (RAOP)".to_string());
        }
        if raw_text.contains("ipp") || raw_text.contains("_printer") {
            detected_services.push("Network Printing (IPP)".to_string());
        }
        if raw_text.contains("sonos") {
            detected_services.push("Smart Audio / Sonos".to_string());
        }
        if raw_text.contains("spotify") {
            detected_services.push("Audio / Spotify Connect".to_string());
        }
        if raw_text.contains("hap") || raw_text.contains("homekit") {
            detected_services.push("Smart Home / HomeKit (HAP)".to_string());
        }
        if raw_text.contains("axis-video") || raw_text.contains("onvif") || raw_text.contains("rtsp") {
            detected_services.push("Security / IP Camera Video".to_string());
        }

        let extracted_name = Self::extract_local_name(body);
        Ok(Some((extracted_name, detected_services)))
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
    async fn probe_ssdp(ip: Ipv4Addr) -> Result<Option<(Option<String>, Vec<String>)>, ()> {
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
        let mut services = Vec::new();

        for line in response.lines() {
            let lower = line.to_lowercase();
            if lower.starts_with("server:") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let srv = parts[1].trim();
                    if srv.contains("Sonos") {
                        services.push("Smart Audio / Sonos".to_string());
                        if friendly_name.is_none() {
                            friendly_name = Some("Sonos Speaker".to_string());
                        }
                    } else if srv.contains("Roku") {
                        services.push("Streaming / Roku".to_string());
                        if friendly_name.is_none() {
                            friendly_name = Some("Roku Streaming Device".to_string());
                        }
                    } else if srv.contains("Tizen") || srv.contains("WebOS") {
                        services.push("Smart TV Display".to_string());
                    }
                }
            } else if lower.starts_with("st:") {
                if lower.contains("dial-multiscreen") {
                    services.push("Streaming / DIAL (Chromecast/FireTV)".to_string());
                } else if lower.contains("mediarenderer") {
                    services.push("UPnP MediaRenderer (Smart TV/Audio)".to_string());
                } else if lower.contains("zoneplayer") {
                    services.push("Smart Audio / Sonos ZonePlayer".to_string());
                } else if lower.contains("digitalsecuritycamera") {
                    services.push("Security / UPnP Security Camera".to_string());
                } else if lower.contains("printer") {
                    services.push("Network Printing / UPnP".to_string());
                }
            }
        }

        Ok(Some((friendly_name, services)))
    }

    /// Quick HTTP probe for DIAL eureka_info (port 8008) and web titles (port 80) concurrently
    async fn probe_http_banner(ip: Ipv4Addr) -> Result<Option<(Option<String>, Option<String>)>, ()> {
        let dial_fut = async {
            let dial_addr = SocketAddrV4::new(ip, 8008);
            if let Ok(Ok(mut stream)) = timeout(Duration::from_millis(120), TcpStream::connect(dial_addr)).await {
                let req = b"GET /setup/eureka_info HTTP/1.0\r\nHost: 127.0.0.1:8008\r\nConnection: close\r\n\r\n";
                if stream.write_all(req).await.is_ok() {
                    let mut buf = [0u8; 1024];
                    if let Ok(Ok(n)) = timeout(Duration::from_millis(120), stream.read(&mut buf)).await {
                        let body = String::from_utf8_lossy(&buf[..n]);
                        if let Some(pos) = body.find("\"name\":\"") {
                            let rem = &body[pos + 8..];
                            if let Some(end) = rem.find('"') {
                                let name = &rem[..end];
                                if !name.is_empty() {
                                    return Some((Some(name.to_string()), Some("Streaming / Google Cast".to_string())));
                                }
                            }
                        }
                        return Some((Some("Google Cast Device".to_string()), Some("Streaming / Google Cast".to_string())));
                    }
                }
            }
            None
        };

        let http_fut = async {
            let http_addr = SocketAddrV4::new(ip, 80);
            if let Ok(Ok(mut stream)) = timeout(Duration::from_millis(120), TcpStream::connect(http_addr)).await {
                let req = format!("GET / HTTP/1.0\r\nHost: {}\r\nUser-Agent: NetLens/0.2\r\nConnection: close\r\n\r\n", ip);
                if stream.write_all(req.as_bytes()).await.is_ok() {
                    let mut buf = [0u8; 1500];
                    if let Ok(Ok(n)) = timeout(Duration::from_millis(120), stream.read(&mut buf)).await {
                        let resp = String::from_utf8_lossy(&buf[..n]);
                        let resp_lower = resp.to_lowercase();

                        if let Some(start_tag) = resp_lower.find("<title>") {
                            let title_part = &resp[start_tag + 7..];
                            if let Some(end_tag) = title_part.to_lowercase().find("</title>") {
                                let title = title_part[..end_tag].trim();
                                if !title.is_empty() && title.len() < 50 {
                                    let service = if resp_lower.contains("camera") || resp_lower.contains("hikvision") || resp_lower.contains("reolink") || resp_lower.contains("dahua") {
                                        Some("Security / IP Camera Web".to_string())
                                    } else if resp_lower.contains("synology") || resp_lower.contains("diskstation") || resp_lower.contains("qnap") || resp_lower.contains("truenas") {
                                        Some("Storage / NAS Web UI".to_string())
                                    } else if resp_lower.contains("tasmota") || resp_lower.contains("wled") || resp_lower.contains("shelly") {
                                        Some("Smart Home / IoT Web UI".to_string())
                                    } else if resp_lower.contains("printer") || resp_lower.contains("laserjet") || resp_lower.contains("epson") {
                                        Some("Printer / Web Admin".to_string())
                                    } else {
                                        Some("HTTP Web Panel".to_string())
                                    };
                                    return Some((Some(title.to_string()), service));
                                }
                            }
                        }
                    }
                }
            }
            None
        };

        let (dial_res, http_res) = tokio::join!(dial_fut, http_fut);
        Ok(dial_res.or(http_res))
    }
}
