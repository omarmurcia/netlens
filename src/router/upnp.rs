use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::process::Command;
use tokio::time::timeout;

/// Active port forwarding rule on router (Port Forwarding / NAT)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct UpnpPortMapping {
    pub external_port: u16,
    pub internal_port: u16,
    pub internal_client: String,
    pub protocol: String, // "TCP" or "UDP"
    pub description: String,
    pub enabled: bool,
}

/// Global router status obtained via UPnP IGD
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpnpRouterStatus {
    pub external_ip: Option<String>,
    pub connection_status: Option<String>,
    pub uptime_secs: Option<u64>,
    pub total_bytes_received: Option<u64>,
    pub total_bytes_sent: Option<u64>,
    pub port_mappings: Vec<UpnpPortMapping>,
    pub model_name: Option<String>,
    pub manufacturer: Option<String>,
}

pub struct UpnpClient;

impl UpnpClient {
    /// Queries the router via UPnP IGD for public IP, status, and mapped ports
    pub async fn query_router(gateway_ip: Ipv4Addr) -> Result<UpnpRouterStatus, String> {
        let (control_url, service_type) = Self::discover_control_url(gateway_ip)
            .await
            .unwrap_or_else(|| {
                (
                    format!("http://{}:5000/ctl/IPConn", gateway_ip),
                    "urn:schemas-upnp-org:service:WANIPConnection:1".to_string(),
                )
            });

        let mut status = UpnpRouterStatus::default();

        // Execute independent UPnP queries concurrently
        let (ext_ip_res, status_res, rx_res, tx_res, mappings) = tokio::join!(
            Self::get_external_ip(&control_url, &service_type),
            Self::get_status_info(&control_url, &service_type),
            Self::get_bytes(&control_url, &service_type, "GetTotalBytesReceived"),
            Self::get_bytes(&control_url, &service_type, "GetTotalBytesSent"),
            Self::get_port_mappings(&control_url, &service_type),
        );

        if let Ok(ip) = ext_ip_res {
            status.external_ip = Some(ip);
        }

        if let Ok((conn_status, uptime)) = status_res {
            status.connection_status = Some(conn_status);
            status.uptime_secs = Some(uptime);
        }

        if let Ok(rx) = rx_res {
            status.total_bytes_received = Some(rx);
        }
        if let Ok(tx) = tx_res {
            status.total_bytes_sent = Some(tx);
        }

        status.port_mappings = mappings;

        Ok(status)
    }

    /// Discovers WANIPConnection control URL via unicast SSDP
    async fn discover_control_url(gateway_ip: Ipv4Addr) -> Option<(String, String)> {
        let socket = UdpSocket::bind("0.0.0.0:0").await.ok()?;
        let target = SocketAddrV4::new(gateway_ip, 1900);

        let msg = "M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nMAN: \"ssdp:discover\"\r\nMX: 1\r\nST: urn:schemas-upnp-org:service:WANIPConnection:1\r\n\r\n";
        let _ = socket.send_to(msg.as_bytes(), target).await;

        let mut buf = [0u8; 2048];
        let res = timeout(Duration::from_millis(600), socket.recv_from(&mut buf)).await;
        if let Ok(Ok((len, _))) = res {
            let response = String::from_utf8_lossy(&buf[..len]);
            for line in response.lines() {
                let lower = line.to_lowercase();
                if lower.starts_with("location:") {
                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        let location_url = parts[1].trim();
                        if let Some((ctl, st)) = Self::parse_desc_xml(location_url).await {
                            return Some((ctl, st));
                        }
                    }
                }
            }
        }

        None
    }

    /// Downloads device description XML and extracts control URL
    async fn parse_desc_xml(desc_url: &str) -> Option<(String, String)> {
        let output = Command::new("curl")
            .args(["-s", "--max-time", "2", desc_url])
            .output()
            .await
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let xml = String::from_utf8_lossy(&output.stdout);
        let service_type = if xml.contains("urn:schemas-upnp-org:service:WANIPConnection:1") {
            "urn:schemas-upnp-org:service:WANIPConnection:1"
        } else if xml.contains("urn:schemas-upnp-org:service:WANPPPConnection:1") {
            "urn:schemas-upnp-org:service:WANPPPConnection:1"
        } else {
            return None;
        };

        // Extract <controlURL>
        let ctl_tag = "<controlURL>";
        let ctl_end = "</controlURL>";
        if let Some(start_idx) = xml.find(ctl_tag) {
            let rest = &xml[start_idx + ctl_tag.len()..];
            if let Some(end_idx) = rest.find(ctl_end) {
                let raw_ctl = rest[..end_idx].trim();
                let full_url = if raw_ctl.starts_with("http://") || raw_ctl.starts_with("https://") {
                    raw_ctl.to_string()
                } else {
                    // Extract base URL
                    if let Some(slash_idx) = desc_url[7..].find('/') {
                        let base = &desc_url[..7 + slash_idx];
                        if raw_ctl.starts_with('/') {
                            format!("{}{}", base, raw_ctl)
                        } else {
                            format!("{}/{}", base, raw_ctl)
                        }
                    } else {
                        format!("{}/{}", desc_url, raw_ctl)
                    }
                };
                return Some((full_url, service_type.to_string()));
            }
        }

        None
    }

    /// Executes a SOAP call to the UPnP control URL
    async fn execute_soap(control_url: &str, service_type: &str, action: &str, body_xml: &str) -> Result<String, String> {
        let soap_envelope = format!(
            "<?xml version=\"1.0\"?>\r\n<s:Envelope xmlns:s=\"http://schemas.xmlsoap.org/soap/envelope/\" s:encodingStyle=\"http://schemas.xmlsoap.org/soap/encoding/\">\r\n<s:Body>\r\n<u:{action} xmlns:u=\"{service_type}\">\r\n{body_xml}</u:{action}>\r\n</s:Body>\r\n</s:Envelope>",
            action = action,
            service_type = service_type,
            body_xml = body_xml
        );

        let soap_action_header = format!("\"{}#{}\"", service_type, action);

        let output = Command::new("curl")
            .args([
                "-s",
                "--max-time", "3",
                "-H", "Content-Type: text/xml; charset=\"utf-8\"",
                "-H", &format!("SOAPAction: {}", soap_action_header),
                "-X", "POST",
                "--data-binary", &soap_envelope,
                control_url,
            ])
            .output()
            .await
            .map_err(|e| format!("Error in UPnP SOAP request: {}", e))?;

        if !output.status.success() {
            return Err("Router rejected UPnP SOAP request".to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Retrieves public IP address of router
    async fn get_external_ip(control_url: &str, service_type: &str) -> Result<String, String> {
        let resp = Self::execute_soap(control_url, service_type, "GetExternalIPAddress", "").await?;
        Self::extract_tag_value(&resp, "NewExternalIPAddress")
            .ok_or_else(|| "NewExternalIPAddress not found in response".to_string())
    }

    /// Retrieves connection status and uptime
    async fn get_status_info(control_url: &str, service_type: &str) -> Result<(String, u64), String> {
        let resp = Self::execute_soap(control_url, service_type, "GetStatusInfo", "").await?;
        let status = Self::extract_tag_value(&resp, "NewConnectionStatus").unwrap_or_else(|| "Connected".to_string());
        let uptime: u64 = Self::extract_tag_value(&resp, "NewUptime")
            .and_then(|u| u.parse().ok())
            .unwrap_or(0);
        Ok((status, uptime))
    }

    /// Retrieves received or sent byte counters
    async fn get_bytes(control_url: &str, service_type: &str, action: &str) -> Result<u64, String> {
        let resp = Self::execute_soap(control_url, service_type, action, "").await?;
        let tag = if action.contains("Received") {
            "NewTotalBytesReceived"
        } else {
            "NewTotalBytesSent"
        };
        Self::extract_tag_value(&resp, tag)
            .and_then(|val| val.parse().ok())
            .ok_or_else(|| "Failed to read byte counter".to_string())
    }

    /// Retrieves port forwarding list (Port Forwardings) in small concurrent batches
    async fn get_port_mappings(control_url: &str, service_type: &str) -> Vec<UpnpPortMapping> {
        let mut mappings = Vec::new();
        for chunk_start in (0..64).step_by(4) {
            let chunk_end = (chunk_start + 4).min(64);
            let futs = (chunk_start..chunk_end).map(|idx| async move {
                let body = format!("<NewPortMappingIndex>{}</NewPortMappingIndex>", idx);
                Self::execute_soap(control_url, service_type, "GetGenericPortMappingEntry", &body).await
            });
            let results = iced::futures::future::join_all(futs).await;

            let mut reached_end = false;
            for res in results {
                match res {
                    Ok(resp) => {
                        if resp.contains("s:Fault") || resp.contains("errorCode>713") {
                            reached_end = true;
                            break;
                        }

                        let ext_port: u16 = Self::extract_tag_value(&resp, "NewExternalPort")
                            .and_then(|p| p.parse().ok())
                            .unwrap_or(0);
                        let int_port: u16 = Self::extract_tag_value(&resp, "NewInternalPort")
                            .and_then(|p| p.parse().ok())
                            .unwrap_or(0);
                        let int_client = Self::extract_tag_value(&resp, "NewInternalClient").unwrap_or_default();
                        let protocol = Self::extract_tag_value(&resp, "NewProtocol").unwrap_or_else(|| "TCP".to_string());
                        let description = Self::extract_tag_value(&resp, "NewPortMappingDescription").unwrap_or_default();
                        let enabled = Self::extract_tag_value(&resp, "NewEnabled")
                            .map(|e| e == "1" || e.to_lowercase() == "true")
                            .unwrap_or(true);

                        if ext_port > 0 && !int_client.is_empty() {
                            mappings.push(UpnpPortMapping {
                                external_port: ext_port,
                                internal_port: int_port,
                                internal_client: int_client,
                                protocol,
                                description,
                                enabled,
                            });
                        }
                    }
                    Err(_) => {
                        reached_end = true;
                        break;
                    }
                }
            }
            if reached_end {
                break;
            }
        }

        mappings
    }

    /// Simple utility to extract content between XML tags without heavy dependencies
    fn extract_tag_value(xml: &str, tag: &str) -> Option<String> {
        let open_tag = format!("<{}>", tag);
        let close_tag = format!("</{}>", tag);

        if let Some(start) = xml.find(&open_tag) {
            let after_open = &xml[start + open_tag.len()..];
            if let Some(end) = after_open.find(&close_tag) {
                return Some(after_open[..end].trim().to_string());
            }
        }

        None
    }
}
