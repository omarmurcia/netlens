use crate::model::Device;
use crate::router::upnp::UpnpPortMapping;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditSeverity {
    Pass,
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityCheckId {
    TelnetDetected,
    FtpDetected,
    EncryptedServices,
    UnwhitelistedDevices,
    AccessControlWhitelisted,
    RandomizedMacs,
    DangerousPortsForwarded,
    ControlledPortMapping,
    RouterPasswordMissing,
    RouterIntegrationActive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCheckItem {
    pub title: String,
    pub description: String,
    pub severity: AuditSeverity,
    pub recommendation: String,
    pub affected_devices: Vec<String>,
    #[serde(default)]
    pub check_id: Option<SecurityCheckId>,
}

impl SecurityCheckItem {
    pub fn title_localized(&self, lang: crate::i18n::Language) -> &str {
        if let Some(id) = self.check_id {
            crate::i18n::Translations::audit_check_title(id, lang)
        } else {
            &self.title
        }
    }

    pub fn description_localized(&self, lang: crate::i18n::Language) -> String {
        if let Some(id) = self.check_id {
            crate::i18n::Translations::audit_check_description(id, self.affected_devices.len(), lang)
        } else {
            self.description.clone()
        }
    }

    pub fn recommendation_localized(&self, lang: crate::i18n::Language) -> &str {
        if let Some(id) = self.check_id {
            crate::i18n::Translations::audit_check_recommendation(id, lang)
        } else {
            &self.recommendation
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditReport {
    pub score: u32,
    pub grade_label: String,
    pub checks: Vec<SecurityCheckItem>,
    pub total_devices_analyzed: usize,
    pub issues_count: usize,
    pub critical_count: usize,
}

pub struct SecurityAuditor;

impl SecurityAuditor {
    /// Executes a comprehensive security audit of the local network
    pub fn audit(
        devices: &[Device],
        port_mappings: &[UpnpPortMapping],
        has_router_password: bool,
    ) -> SecurityAuditReport {
        let mut score: i32 = 100;
        let mut checks = Vec::new();
        let total_devices = devices.len();

        // 1. Verification: Insecure unencrypted ports (Telnet / FTP)
        let mut telnet_devices = Vec::new();
        let mut ftp_devices = Vec::new();
        for d in devices {
            for p in &d.open_ports {
                if p.port == 23 {
                    telnet_devices.push(format!("{} ({})", d.display_name(), d.ip));
                } else if p.port == 21 {
                    ftp_devices.push(format!("{} ({})", d.display_name(), d.ip));
                }
            }
        }

        let has_telnet = !telnet_devices.is_empty();
        let has_ftp = !ftp_devices.is_empty();

        if has_telnet {
            score -= 20;
            checks.push(SecurityCheckItem {
                title: "Telnet Protocol Detected (Port 23)".to_string(),
                description: "Devices with open Telnet detected. Telnet transmits credentials and commands in unencrypted plaintext.".to_string(),
                severity: AuditSeverity::Critical,
                recommendation: "Disable Telnet and use SSH (port 22) for secure encrypted administration.".to_string(),
                affected_devices: telnet_devices,
                check_id: Some(SecurityCheckId::TelnetDetected),
            });
        }

        if has_ftp {
            score -= 10;
            checks.push(SecurityCheckItem {
                title: "Insecure FTP Service (Port 21)".to_string(),
                description: "Traditional FTP does not encrypt passwords or files during local transfer.".to_string(),
                severity: AuditSeverity::Warning,
                recommendation: "Migrate to SFTP (over SSH) or FTPS with modern TLS certificates.".to_string(),
                affected_devices: ftp_devices,
                check_id: Some(SecurityCheckId::FtpDetected),
            });
        }

        if !has_telnet && !has_ftp {
            checks.push(SecurityCheckItem {
                title: "Encrypted Network Services".to_string(),
                description: "No obsolete vulnerable ports (Telnet or plaintext FTP) detected on active devices.".to_string(),
                severity: AuditSeverity::Pass,
                recommendation: "Continue using encrypted services (SSH, HTTPS, SFTP).".to_string(),
                affected_devices: Vec::new(),
                check_id: Some(SecurityCheckId::EncryptedServices),
            });
        }

        // 2. Verification: Unauthorized devices / Outside whitelist
        let unwhitelisted: Vec<String> = devices
            .iter()
            .filter(|d| !d.is_gateway && !d.is_current_host && !d.is_whitelisted)
            .map(|d| format!("{} ({})", d.display_name(), d.ip))
            .collect();

        if !unwhitelisted.is_empty() && devices.iter().any(|d| d.is_whitelisted) {
            score -= (unwhitelisted.len() as i32 * 5).min(20);
            checks.push(SecurityCheckItem {
                title: "Devices Not on Whitelist".to_string(),
                description: format!("There are {} connected devices that have not been validated as known.", unwhitelisted.len()),
                severity: AuditSeverity::Warning,
                recommendation: "Review the device list and add legitimate devices to the Whitelist or block them on the router if unknown.".to_string(),
                affected_devices: unwhitelisted,
                check_id: Some(SecurityCheckId::UnwhitelistedDevices),
            });
        } else {
            checks.push(SecurityCheckItem {
                title: "Access Control & Whitelist".to_string(),
                description: "All connected devices are identified or no intruders were flagged.".to_string(),
                severity: AuditSeverity::Pass,
                recommendation: "Keep your home network device inventory updated.".to_string(),
                affected_devices: Vec::new(),
                check_id: Some(SecurityCheckId::AccessControlWhitelisted),
            });
        }

        // 3. Verification: Unidentified private / randomized MAC addresses
        let unaliased_random_macs: Vec<String> = devices
            .iter()
            .filter(|d| d.is_randomized_mac && d.custom_alias.is_none())
            .map(|d| format!("IP {} (MAC {})", d.ip, d.mac))
            .collect();

        if !unaliased_random_macs.is_empty() {
            score -= (unaliased_random_macs.len() as i32 * 4).min(15);
            checks.push(SecurityCheckItem {
                title: "Unidentified Randomized MACs".to_string(),
                description: "Modern devices (iOS/Android) use randomized private MACs, making identification harder.".to_string(),
                severity: AuditSeverity::Info,
                recommendation: "Assign a custom alias to each device to easily recognize it in future connections.".to_string(),
                affected_devices: unaliased_random_macs,
                check_id: Some(SecurityCheckId::RandomizedMacs),
            });
        }

        // 4. Verification: Internet port forwarding (UPnP Port Mapping)
        let mut dangerous_ports_forwarded = Vec::new();
        for m in port_mappings {
            if m.enabled {
                if [22, 23, 80, 445, 3389, 5900].contains(&m.external_port) {
                    dangerous_ports_forwarded.push(format!(
                        "Port {} ({}) -> {}",
                        m.external_port, m.protocol, m.internal_client
                    ));
                }
            }
        }

        if !dangerous_ports_forwarded.is_empty() {
            score -= 25;
            checks.push(SecurityCheckItem {
                title: "Critical Ports Exposed to Internet".to_string(),
                description: "Router NAT rules detected that expose sensitive services to direct Internet attacks.".to_string(),
                severity: AuditSeverity::Critical,
                recommendation: "Remove port forwarding rules for SSH, RDP, SMB, or Telnet to the Internet. Use a home VPN (WireGuard/OpenVPN) for remote access.".to_string(),
                affected_devices: dangerous_ports_forwarded,
                check_id: Some(SecurityCheckId::DangerousPortsForwarded),
            });
        } else if !port_mappings.is_empty() {
            checks.push(SecurityCheckItem {
                title: "Controlled Port Mapping".to_string(),
                description: format!("There are {} open ports on the router, none considered high risk.", port_mappings.len()),
                severity: AuditSeverity::Pass,
                recommendation: "Periodically review applications that open ports via UPnP.".to_string(),
                affected_devices: Vec::new(),
                check_id: Some(SecurityCheckId::ControlledPortMapping),
            });
        }

        // 5. Verification: Gateway / Router Security
        if !has_router_password {
            score -= 10;
            checks.push(SecurityCheckItem {
                title: "Router Password Not Configured in NetLens".to_string(),
                description: "NetLens does not have the admin password saved to apply 24/7 router hardware blocking.".to_string(),
                severity: AuditSeverity::Info,
                recommendation: "Enter the password in Settings to enable hardware control and intruder isolation.".to_string(),
                affected_devices: Vec::new(),
                check_id: Some(SecurityCheckId::RouterPasswordMissing),
            });
        } else {
            checks.push(SecurityCheckItem {
                title: "Router Integration Active".to_string(),
                description: "Router administrative password is configured, enabling direct hardware-level blocklists.".to_string(),
                severity: AuditSeverity::Pass,
                recommendation: "Excellent, the network is actively protected.".to_string(),
                affected_devices: Vec::new(),
                check_id: Some(SecurityCheckId::RouterIntegrationActive),
            });
        }

        let final_score = score.clamp(0, 100) as u32;
        let grade_label = match final_score {
            90..=100 => "Excellent".to_string(),
            75..=89 => "Good".to_string(),
            50..=74 => "Warning".to_string(),
            _ => "At Risk".to_string(),
        };

        let issues_count = checks
            .iter()
            .filter(|c| c.severity == AuditSeverity::Warning || c.severity == AuditSeverity::Critical)
            .count();
        let critical_count = checks
            .iter()
            .filter(|c| c.severity == AuditSeverity::Critical)
            .count();

        SecurityAuditReport {
            score: final_score,
            grade_label,
            checks,
            total_devices_analyzed: total_devices,
            issues_count,
            critical_count,
        }
    }
}
