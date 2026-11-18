use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::process::Command;
use tokio::time::timeout;

/// Regla de reenvío de puertos activa en el router (Port Forwarding / NAT)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct UpnpPortMapping {
    pub external_port: u16,
    pub internal_port: u16,
    pub internal_client: String,
    pub protocol: String, // "TCP" o "UDP"
    pub description: String,
    pub enabled: bool,
}

/// Estado global del router obtenido mediante UPnP IGD
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
    /// Consulta el router mediante UPnP IGD para obtener IP pública, estado y puertos mapeados
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

        // 1. Obtener IP pública (GetExternalIPAddress)
        if let Ok(ip) = Self::get_external_ip(&control_url, &service_type).await {
            status.external_ip = Some(ip);
        }

        // 2. Obtener estado de conexión y tiempo de actividad (GetStatusInfo)
        if let Ok((conn_status, uptime)) = Self::get_status_info(&control_url, &service_type).await {
            status.connection_status = Some(conn_status);
            status.uptime_secs = Some(uptime);
        }

        // 3. Obtener contadores de bytes (GetTotalBytesReceived / Sent)
        if let Ok(rx) = Self::get_bytes(&control_url, &service_type, "GetTotalBytesReceived").await {
            status.total_bytes_received = Some(rx);
        }
        if let Ok(tx) = Self::get_bytes(&control_url, &service_type, "GetTotalBytesSent").await {
            status.total_bytes_sent = Some(tx);
        }

        // 4. Obtener tabla de reenvío de puertos (GetGenericPortMappingEntry)
        status.port_mappings = Self::get_port_mappings(&control_url, &service_type).await;

        Ok(status)
    }

    /// Descubre la URL de control de WANIPConnection mediante SSDP unicast
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

    /// Descarga el XML de descripción del dispositivo y extrae la URL de control
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

        // Extraer <controlURL>
        let ctl_tag = "<controlURL>";
        let ctl_end = "</controlURL>";
        if let Some(start_idx) = xml.find(ctl_tag) {
            let rest = &xml[start_idx + ctl_tag.len()..];
            if let Some(end_idx) = rest.find(ctl_end) {
                let raw_ctl = rest[..end_idx].trim();
                let full_url = if raw_ctl.starts_with("http://") || raw_ctl.starts_with("https://") {
                    raw_ctl.to_string()
                } else {
                    // Extraer base URL
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

    /// Ejecuta una llamada SOAP a la URL de control UPnP
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
            .map_err(|e| format!("Error en petición SOAP UPnP: {}", e))?;

        if !output.status.success() {
            return Err("El router rechazó la petición UPnP SOAP".to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Obtiene la dirección IP pública del router
    async fn get_external_ip(control_url: &str, service_type: &str) -> Result<String, String> {
        let resp = Self::execute_soap(control_url, service_type, "GetExternalIPAddress", "").await?;
        Self::extract_tag_value(&resp, "NewExternalIPAddress")
            .ok_or_else(|| "No se encontró NewExternalIPAddress en la respuesta".to_string())
    }

    /// Obtiene el estado y tiempo de actividad
    async fn get_status_info(control_url: &str, service_type: &str) -> Result<(String, u64), String> {
        let resp = Self::execute_soap(control_url, service_type, "GetStatusInfo", "").await?;
        let status = Self::extract_tag_value(&resp, "NewConnectionStatus").unwrap_or_else(|| "Conectado".to_string());
        let uptime: u64 = Self::extract_tag_value(&resp, "NewUptime")
            .and_then(|u| u.parse().ok())
            .unwrap_or(0);
        Ok((status, uptime))
    }

    /// Obtiene contadores de bytes recibidos o enviados
    async fn get_bytes(control_url: &str, service_type: &str, action: &str) -> Result<u64, String> {
        let resp = Self::execute_soap(control_url, service_type, action, "").await?;
        let tag = if action.contains("Received") {
            "NewTotalBytesReceived"
        } else {
            "NewTotalBytesSent"
        };
        Self::extract_tag_value(&resp, tag)
            .and_then(|val| val.parse().ok())
            .ok_or_else(|| "No se pudo leer contador de bytes".to_string())
    }

    /// Obtiene la lista de puertos reenviados (Port Forwardings)
    async fn get_port_mappings(control_url: &str, service_type: &str) -> Vec<UpnpPortMapping> {
        let mut mappings = Vec::new();
        // Iterar sobre los índices 0 a 64
        for idx in 0..64 {
            let body = format!("<NewPortMappingIndex>{}</NewPortMappingIndex>", idx);
            if let Ok(resp) = Self::execute_soap(control_url, service_type, "GetGenericPortMappingEntry", &body).await {
                if resp.contains("s:Fault") || resp.contains("errorCode>713") {
                    break; // Fin de la lista de reglas
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
            } else {
                break;
            }
        }

        mappings
    }

    /// Utilidad simple para extraer contenido entre etiquetas XML sin dependencias pesadas
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
