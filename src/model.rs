use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::Ipv4Addr;

/// Inferred device category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceCategory {
    Router,
    Smartphone,
    Tablet,
    Computer,
    SmartTv,
    StreamingDevice,
    GamingConsole,
    SmartSpeaker,
    SecurityCamera,
    #[serde(alias = "SmartHomeIot", alias = "SmartHome")]
    Iot,
    Printer,
    #[serde(alias = "Nas", alias = "Storage")]
    StorageNas,
    Server,
    Wearable,
    NetworkBridge,
    Unknown,
}

impl DeviceCategory {
    pub fn icon(&self) -> &'static str {
        match self {
            DeviceCategory::Router => "🌐",
            DeviceCategory::Smartphone => "📱",
            DeviceCategory::Tablet => "📲",
            DeviceCategory::Computer => "💻",
            DeviceCategory::SmartTv => "📺",
            DeviceCategory::StreamingDevice => "🎬",
            DeviceCategory::GamingConsole => "🎮",
            DeviceCategory::SmartSpeaker => "🔊",
            DeviceCategory::SecurityCamera => "📹",
            DeviceCategory::Iot => "🔌",
            DeviceCategory::Printer => "🖨️",
            DeviceCategory::StorageNas => "💾",
            DeviceCategory::Server => "🖧",
            DeviceCategory::Wearable => "⌚",
            DeviceCategory::NetworkBridge => "🔀",
            DeviceCategory::Unknown => "❓",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            DeviceCategory::Router => "Router / Puerta de Enlace",
            DeviceCategory::Smartphone => "Smartphone / Móvil",
            DeviceCategory::Tablet => "Tablet / iPad",
            DeviceCategory::Computer => "Ordenador / PC / Mac",
            DeviceCategory::SmartTv => "Smart TV / Pantalla",
            DeviceCategory::StreamingDevice => "Dispositivo de Streaming",
            DeviceCategory::GamingConsole => "Consola de Videojuegos",
            DeviceCategory::SmartSpeaker => "Altavoz Inteligente / Audio",
            DeviceCategory::SecurityCamera => "Cámara de Seguridad / Vigilancia",
            DeviceCategory::Iot => "Domótica / Smart Home / IoT",
            DeviceCategory::Printer => "Impresora de Red",
            DeviceCategory::StorageNas => "Almacenamiento / Servidor NAS",
            DeviceCategory::Server => "Servidor / Host Virtual",
            DeviceCategory::Wearable => "Smartwatch / Wearable",
            DeviceCategory::NetworkBridge => "Switch / Repetidor de Red",
            DeviceCategory::Unknown => "Dispositivo Genérico",
        }
    }

    #[allow(dead_code)]
    pub fn all() -> &'static [DeviceCategory] {
        &[
            DeviceCategory::Router,
            DeviceCategory::Smartphone,
            DeviceCategory::Tablet,
            DeviceCategory::Computer,
            DeviceCategory::SmartTv,
            DeviceCategory::StreamingDevice,
            DeviceCategory::GamingConsole,
            DeviceCategory::SmartSpeaker,
            DeviceCategory::SecurityCamera,
            DeviceCategory::Iot,
            DeviceCategory::Printer,
            DeviceCategory::StorageNas,
            DeviceCategory::Server,
            DeviceCategory::Wearable,
            DeviceCategory::NetworkBridge,
            DeviceCategory::Unknown,
        ]
    }
}

impl fmt::Display for DeviceCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.icon(), self.display_name())
    }
}

/// Device connectivity status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceStatus {
    Online,
    Degraded,
    Offline,
}

impl DeviceStatus {
    #[allow(dead_code)]
    pub fn label(&self) -> &'static str {
        match self {
            DeviceStatus::Online => "Online",
            DeviceStatus::Degraded => "Degraded",
            DeviceStatus::Offline => "Offline",
        }
    }
}

/// Information about an open network port
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortInfo {
    pub port: u16,
    pub service: String,
    pub description: String,
}

/// Network request, DNS query, or web activity record per device
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceRequestEntry {
    pub timestamp: DateTime<Local>,
    pub query: String,
    pub category: String,
    pub protocol: String,
}

/// Discovered device on the local network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub ip: Ipv4Addr,
    pub mac: String,
    pub vendor: String,
    pub hostname: Option<String>,
    pub category: DeviceCategory,
    pub is_randomized_mac: bool,
    pub is_gateway: bool,
    pub is_current_host: bool,
    pub interface: String,
    pub latency_ms: Option<f32>,
    pub open_ports: Vec<PortInfo>,
    pub status: DeviceStatus,
    pub first_seen: DateTime<Local>,
    pub last_seen: DateTime<Local>,
    pub custom_alias: Option<String>,
    #[serde(default)]
    pub is_whitelisted: bool,
    #[serde(default)]
    pub is_blocked: bool,
    #[serde(default)]
    pub speed_limit_kbps: Option<u32>,
    #[serde(default)]
    pub connection_type: Option<String>,
    #[serde(default)]
    pub wifi_signal_dbm: Option<i32>,
    #[serde(default)]
    pub negotiated_mbps: Option<u32>,
    #[serde(default)]
    pub model_name: Option<String>,
    #[serde(default)]
    pub discovered_services: Vec<String>,
    #[serde(default)]
    pub request_history: Vec<DeviceRequestEntry>,
}

impl Device {
    /// Primary display name to show in the user interface
    pub fn display_name(&self) -> String {
        if let Some(alias) = &self.custom_alias {
            if !alias.trim().is_empty() {
                return alias.clone();
            }
        }
        if let Some(model) = &self.model_name {
            if !model.trim().is_empty() {
                return model.clone();
            }
        }
        if let Some(host) = &self.hostname {
            if !host.trim().is_empty() {
                return host.clone();
            }
        }
        if !self.vendor.trim().is_empty() && self.vendor != "Desconocido" && self.vendor != "Unknown" {
            return format!("{} ({})", self.vendor, self.ip);
        }
        if self.is_gateway {
            return format!("Gateway ({})", self.ip);
        }
        if self.is_current_host {
            return format!("This Device ({})", self.ip);
        }
        format!("Device {}", self.ip)
    }

    /// Technical subtitle with hardware and connection details
    #[allow(dead_code)]
    pub fn subtitle(&self) -> String {
        let mut parts = Vec::new();
        if self.is_gateway {
            parts.push("Default Gateway");
        }
        if self.is_current_host {
            parts.push("This Device (Local Host)");
        }
        if !self.vendor.is_empty() && self.vendor != "Desconocido" && self.vendor != "Unknown" {
            parts.push(&self.vendor);
        } else if self.is_randomized_mac {
            parts.push("Private / Randomized MAC Address");
        }
        if parts.is_empty() {
            parts.push("Unidentified Manufacturer");
        }
        parts.join(" • ")
    }

    /// Records a new request or search query in this device's history
    pub fn add_request(&mut self, query: String, category: String, protocol: String) {
        let entry = DeviceRequestEntry {
            timestamp: Local::now(),
            query,
            category,
            protocol,
        };
        self.request_history.insert(0, entry);
        if self.request_history.len() > 60 {
            self.request_history.truncate(60);
        }
    }
}

/// Local network context information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkContext {
    pub interface: String,
    pub local_ip: Option<Ipv4Addr>,
    pub local_mac: Option<String>,
    pub gateway_ip: Option<Ipv4Addr>,
    pub gateway_mac: Option<String>,
    pub subnet_cidr: String,
}

/// Standard well-known ports definition
pub struct WellKnownPort;

impl WellKnownPort {
    pub fn describe(port: u16) -> (&'static str, &'static str) {
        match port {
            21 => ("FTP", "Unencrypted file transfer"),
            22 => ("SSH", "Secure remote terminal (Linux/Unix/Mac)"),
            23 => ("Telnet", "Unencrypted legacy remote terminal"),
            25 => ("SMTP", "Outgoing mail server"),
            53 => ("DNS", "Domain name resolution"),
            80 => ("HTTP", "Web server / Admin panel"),
            110 => ("POP3", "Incoming mail retrieval"),
            135 => ("MS-RPC", "Windows Remote Procedure Call"),
            139 => ("NetBIOS", "NetBIOS Session Service"),
            143 => ("IMAP", "Mailbox access protocol"),
            443 => ("HTTPS", "Encrypted secure web server"),
            445 => ("SMB", "Windows / Samba file sharing"),
            548 => ("AFP", "Apple Filing Protocol for macOS"),
            554 => ("RTSP", "Real-Time Streaming Protocol / IP Cameras"),
            631 => ("IPP/CUPS", "Internet Printing Protocol / CUPS"),
            993 => ("IMAPS", "Secure IMAP mail"),
            995 => ("POP3S", "Secure POP3 mail"),
            1400 => ("Sonos", "Sonos Smart Speaker Control"),
            1433 => ("MSSQL", "Microsoft SQL Server database"),
            1723 => ("PPTP", "Point-to-Point VPN tunnel"),
            1883 => ("MQTT", "Smart Home / IoT Message Broker"),
            1900 => ("SSDP/UPnP", "Smart TV & media discovery"),
            3074 => ("XboxLive/PSN", "Gaming Console Network Service"),
            3306 => ("MySQL", "MySQL / MariaDB database"),
            3389 => ("RDP", "Windows Remote Desktop"),
            5000 => ("UPnP / DSM", "Synology NAS panel or UPnP server"),
            5001 => ("Synology HTTPS", "Synology DSM Encrypted Management"),
            5353 => ("mDNS", "Multicast DNS ZeroConf"),
            5432 => ("PostgreSQL", "PostgreSQL database"),
            8008 => ("Google Cast", "Chromecast / Google Home device"),
            8009 => ("Google Cast V2", "Secure Chromecast media streaming"),
            8080 => ("HTTP-Alt", "Alternative web server / Proxy"),
            8443 => ("HTTPS-Alt", "Alternative secure web server"),
            8554 => ("RTSP-Alt", "Alternative IP Camera Video Stream"),
            9000 => ("Portainer", "Portainer Docker management web UI"),
            9100 => ("RAW Print", "JetDirect raw network printing"),
            _ => ("Unknown", "Non-standard service"),
        }
    }
}
