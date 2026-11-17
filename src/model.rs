use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::Ipv4Addr;

/// Categoría deducida del dispositivo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceCategory {
    Router,
    Smartphone,
    Computer,
    SmartTv,
    Iot,
    Printer,
    Server,
    Unknown,
}

impl DeviceCategory {
    pub fn icon(&self) -> &'static str {
        match self {
            DeviceCategory::Router => "🌐",
            DeviceCategory::Smartphone => "📱",
            DeviceCategory::Computer => "💻",
            DeviceCategory::SmartTv => "📺",
            DeviceCategory::Iot => "🔌",
            DeviceCategory::Printer => "🖨️",
            DeviceCategory::Server => "🖧",
            DeviceCategory::Unknown => "❓",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            DeviceCategory::Router => "Router / Puerta de Enlace",
            DeviceCategory::Smartphone => "Smartphone / Móvil",
            DeviceCategory::Computer => "Ordenador / PC",
            DeviceCategory::SmartTv => "Smart TV / Streaming",
            DeviceCategory::Iot => "Domótica / IoT",
            DeviceCategory::Printer => "Impresora de Red",
            DeviceCategory::Server => "Servidor / NAS",
            DeviceCategory::Unknown => "Dispositivo Genérico",
        }
    }

    #[allow(dead_code)]
    pub fn all() -> &'static [DeviceCategory] {
        &[
            DeviceCategory::Router,
            DeviceCategory::Smartphone,
            DeviceCategory::Computer,
            DeviceCategory::SmartTv,
            DeviceCategory::Iot,
            DeviceCategory::Printer,
            DeviceCategory::Server,
            DeviceCategory::Unknown,
        ]
    }
}

impl fmt::Display for DeviceCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.icon(), self.display_name())
    }
}

/// Estado de conectividad del dispositivo
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
            DeviceStatus::Online => "En línea",
            DeviceStatus::Degraded => "Inestable",
            DeviceStatus::Offline => "Desconectado",
        }
    }
}

/// Información sobre un puerto abierto
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortInfo {
    pub port: u16,
    pub service: String,
    pub description: String,
}

/// Representa un dispositivo descubierto en la red local
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
}

impl Device {
    /// Nombre principal para mostrar en la interfaz
    pub fn display_name(&self) -> String {
        if let Some(alias) = &self.custom_alias {
            if !alias.trim().is_empty() {
                return alias.clone();
            }
        }
        if let Some(host) = &self.hostname {
            if !host.trim().is_empty() {
                return host.clone();
            }
        }
        if !self.vendor.trim().is_empty() && self.vendor != "Desconocido" {
            return format!("{} ({})", self.vendor, self.ip);
        }
        if self.is_gateway {
            return format!("Puerta de Enlace ({})", self.ip);
        }
        if self.is_current_host {
            return format!("Este Equipo ({})", self.ip);
        }
        format!("Dispositivo {}", self.ip)
    }

    /// Subtítulo con información técnica
    #[allow(dead_code)]
    pub fn subtitle(&self) -> String {
        let mut parts = Vec::new();
        if self.is_gateway {
            parts.push("Puerta de Enlace Predeterminada");
        }
        if self.is_current_host {
            parts.push("Este Equipo (Host Local)");
        }
        if !self.vendor.is_empty() && self.vendor != "Desconocido" {
            parts.push(&self.vendor);
        } else if self.is_randomized_mac {
            parts.push("Dirección MAC Privada / Aleatoria");
        }
        if parts.is_empty() {
            parts.push("Fabricante no identificado");
        }
        parts.join(" • ")
    }
}

/// Información del contexto de red local
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkContext {
    pub interface: String,
    pub local_ip: Option<Ipv4Addr>,
    pub local_mac: Option<String>,
    pub gateway_ip: Option<Ipv4Addr>,
    pub gateway_mac: Option<String>,
    pub subnet_cidr: String,
}

/// Definición de puertos estándar
pub struct WellKnownPort;

impl WellKnownPort {
    pub fn describe(port: u16) -> (&'static str, &'static str) {
        match port {
            21 => ("FTP", "Transferencia de archivos sin cifrar"),
            22 => ("SSH", "Terminal remota segura (Linux/Unix/Mac)"),
            23 => ("Telnet", "Acceso por terminal sin cifrar"),
            25 => ("SMTP", "Servidor de correo saliente"),
            53 => ("DNS", "Resolución de nombres de dominio"),
            80 => ("HTTP", "Servidor web / Panel de administración"),
            110 => ("POP3", "Recepción de correo electrónico"),
            135 => ("MS-RPC", "Llamada a procedimiento remoto de Windows"),
            139 => ("NetBIOS", "Servicio de sesión NetBIOS"),
            143 => ("IMAP", "Acceso a buzón de correo"),
            443 => ("HTTPS", "Servidor web seguro cifrado"),
            445 => ("SMB", "Compartición de archivos Windows / Samba"),
            548 => ("AFP", "Apple Filing Protocol para macOS"),
            554 => ("RTSP", "Streaming de video / Cámaras IP"),
            631 => ("IPP/CUPS", "Servidor de impresión"),
            993 => ("IMAPS", "Correo IMAP seguro"),
            995 => ("POP3S", "Correo POP3 seguro"),
            1433 => ("MSSQL", "Base de datos Microsoft SQL Server"),
            1723 => ("PPTP", "Túnel VPN punto a punto"),
            1900 => ("SSDP/UPnP", "Descubrimiento de Smart TVs y media"),
            3306 => ("MySQL", "Base de datos MySQL / MariaDB"),
            3389 => ("RDP", "Escritorio Remoto de Windows"),
            5000 => ("UPnP / DSM", "Panel Synology NAS o servidor UPnP"),
            5353 => ("mDNS", "Multicast DNS ZeroConf"),
            5432 => ("PostgreSQL", "Base de datos PostgreSQL"),
            8008 => ("Google Cast", "Dispositivo Chromecast / Google Home"),
            8009 => ("Google Cast V2", "Streaming Chromecast seguro"),
            8080 => ("HTTP-Alt", "Servidor web alternativo / Proxy"),
            8443 => ("HTTPS-Alt", "Servidor web seguro alternativo"),
            9000 => ("Portainer", "Gestor web de contenedores Docker"),
            9100 => ("RAW Print", "Puerto de impresión directo JetDirect"),
            _ => ("Desconocido", "Servicio no estándar"),
        }
    }
}
