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
    /// Ejecuta una auditoría integral de seguridad de la red local
    pub fn audit(
        devices: &[Device],
        port_mappings: &[UpnpPortMapping],
        has_router_password: bool,
    ) -> SecurityAuditReport {
        let mut score: i32 = 100;
        let mut checks = Vec::new();
        let total_devices = devices.len();

        // 1. Verificación: Puertos inseguros sin cifrar (Telnet / FTP)
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
                title: "Protocolo Telnet Detectado (Puerto 23)".to_string(),
                description: "Se detectaron dispositivos con Telnet abierto. Telnet transmite credenciales y comandos en texto plano sin cifrar.".to_string(),
                severity: AuditSeverity::Critical,
                recommendation: "Deshabilita Telnet y utiliza SSH (puerto 22) para administración segura cifrada.".to_string(),
                affected_devices: telnet_devices,
                check_id: Some(SecurityCheckId::TelnetDetected),
            });
        }

        if has_ftp {
            score -= 10;
            checks.push(SecurityCheckItem {
                title: "Servicio FTP Inseguro (Puerto 21)".to_string(),
                description: "FTP tradicional no cifra contraseñas ni archivos durante la transferencia local.".to_string(),
                severity: AuditSeverity::Warning,
                recommendation: "Migra hacia SFTP (sobre SSH) o FTPS con certificados TLS modernos.".to_string(),
                affected_devices: ftp_devices,
                check_id: Some(SecurityCheckId::FtpDetected),
            });
        }

        if !has_telnet && !has_ftp {
            checks.push(SecurityCheckItem {
                title: "Servicios de Red Cifrados".to_string(),
                description: "No se detectaron puertos vulnerables obsoletos (Telnet o FTP plano) en los dispositivos activos.".to_string(),
                severity: AuditSeverity::Pass,
                recommendation: "Continúa utilizando servicios cifrados (SSH, HTTPS, SFTP).".to_string(),
                affected_devices: Vec::new(),
                check_id: Some(SecurityCheckId::EncryptedServices),
            });
        }

        // 2. Verificación: Dispositivos no autorizados / Fuera de lista blanca
        let unwhitelisted: Vec<String> = devices
            .iter()
            .filter(|d| !d.is_gateway && !d.is_current_host && !d.is_whitelisted)
            .map(|d| format!("{} ({})", d.display_name(), d.ip))
            .collect();

        if !unwhitelisted.is_empty() && devices.iter().any(|d| d.is_whitelisted) {
            score -= (unwhitelisted.len() as i32 * 5).min(20);
            checks.push(SecurityCheckItem {
                title: "Dispositivos no incluidos en Lista Blanca".to_string(),
                description: format!("Existen {} dispositivos conectados que no han sido validados como conocidos.", unwhitelisted.len()),
                severity: AuditSeverity::Warning,
                recommendation: "Revisa la lista de dispositivos y añade los legítimos a la Lista Blanca o bloquéalos en el router si son desconocidos.".to_string(),
                affected_devices: unwhitelisted,
                check_id: Some(SecurityCheckId::UnwhitelistedDevices),
            });
        } else {
            checks.push(SecurityCheckItem {
                title: "Control de Acceso y Lista Blanca".to_string(),
                description: "Todos los dispositivos conectados están identificados o no se han marcado intrusos.".to_string(),
                severity: AuditSeverity::Pass,
                recommendation: "Mantén actualizado el inventario de dispositivos del hogar.".to_string(),
                affected_devices: Vec::new(),
                check_id: Some(SecurityCheckId::AccessControlWhitelisted),
            });
        }

        // 3. Verificación: Direcciones MAC privadas/aleatorias no identificadas
        let unaliased_random_macs: Vec<String> = devices
            .iter()
            .filter(|d| d.is_randomized_mac && d.custom_alias.is_none())
            .map(|d| format!("IP {} (MAC {})", d.ip, d.mac))
            .collect();

        if !unaliased_random_macs.is_empty() {
            score -= (unaliased_random_macs.len() as i32 * 4).min(15);
            checks.push(SecurityCheckItem {
                title: "MACs Aleatorias sin Identificar".to_string(),
                description: "Dispositivos modernos (iOS/Android) usan MAC privada aleatoria, dificultando saber a quién pertenece el equipo.".to_string(),
                severity: AuditSeverity::Info,
                recommendation: "Asigna un alias personalizado a cada dispositivo para reconocerlo fácilmente en futuras conexiones.".to_string(),
                affected_devices: unaliased_random_macs,
                check_id: Some(SecurityCheckId::RandomizedMacs),
            });
        }

        // 4. Verificación: Reenvío de puertos hacia Internet (UPnP Port Mapping)
        let mut dangerous_ports_forwarded = Vec::new();
        for m in port_mappings {
            if m.enabled {
                if [22, 23, 80, 445, 3389, 5900].contains(&m.external_port) {
                    dangerous_ports_forwarded.push(format!(
                        "Puerto {} ({}) -> {}",
                        m.external_port, m.protocol, m.internal_client
                    ));
                }
            }
        }

        if !dangerous_ports_forwarded.is_empty() {
            score -= 25;
            checks.push(SecurityCheckItem {
                title: "Puertos Críticos Expuestos a Internet".to_string(),
                description: "Se detectaron reglas NAT en el router que exponen servicios sensibles a ataques directos desde Internet.".to_string(),
                severity: AuditSeverity::Critical,
                recommendation: "Elimina las reglas de reenvío de puertos para SSH, RDP, SMB o Telnet hacia Internet. Utiliza una VPN doméstica (WireGuard/OpenVPN) para acceder de forma remota.".to_string(),
                affected_devices: dangerous_ports_forwarded,
                check_id: Some(SecurityCheckId::DangerousPortsForwarded),
            });
        } else if !port_mappings.is_empty() {
            checks.push(SecurityCheckItem {
                title: "Mapeo de Puertos Controlado".to_string(),
                description: format!("Hay {} puertos abiertos en el router, ninguno considerado de alto riesgo crítico.", port_mappings.len()),
                severity: AuditSeverity::Pass,
                recommendation: "Revisa periódicamente las aplicaciones que abren puertos mediante UPnP.".to_string(),
                affected_devices: Vec::new(),
                check_id: Some(SecurityCheckId::ControlledPortMapping),
            });
        }

        // 5. Verificación: Seguridad de la Puerta de Enlace (Router)
        if !has_router_password {
            score -= 10;
            checks.push(SecurityCheckItem {
                title: "Contraseña de Router no Configurada en NetLens".to_string(),
                description: "NetLens no tiene guardada la contraseña administrativa para aplicar bloqueo 24/7 en el router.".to_string(),
                severity: AuditSeverity::Info,
                recommendation: "Ingresa la contraseña en Ajustes para habilitar el control por hardware y aislamiento de intrusos.".to_string(),
                affected_devices: Vec::new(),
                check_id: Some(SecurityCheckId::RouterPasswordMissing),
            });
        } else {
            checks.push(SecurityCheckItem {
                title: "Integración con Router Activa".to_string(),
                description: "La contraseña de administración del router está configurada, permitiendo aplicar listas de bloqueo directamente por hardware.".to_string(),
                severity: AuditSeverity::Pass,
                recommendation: "Excelente, la red está protegida activamente.".to_string(),
                affected_devices: Vec::new(),
                check_id: Some(SecurityCheckId::RouterIntegrationActive),
            });
        }

        let final_score = score.clamp(0, 100) as u32;
        let grade_label = match final_score {
            90..=100 => "Excelente".to_string(),
            75..=89 => "Buena".to_string(),
            50..=74 => "Advertencia".to_string(),
            _ => "En Riesgo".to_string(),
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
