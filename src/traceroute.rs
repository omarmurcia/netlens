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
    /// Ejecuta un trazado de ruta asíncrono utilizando tracepath sin requerir privilegios de superusuario (root)
    pub async fn trace(target: &str) -> Result<Vec<HopInfo>, String> {
        let output = Command::new("tracepath")
            .arg("-n")
            .arg("-m")
            .arg("10")
            .arg(target)
            .output()
            .await
            .map_err(|e| format!("Error ejecutando tracepath: {}", e))?;

        let text = String::from_utf8_lossy(&output.stdout);
        let hops = Self::parse_output(&text, target);
        if hops.is_empty() {
            return Err("No se obtuvieron saltos del trazado de ruta".to_string());
        }
        Ok(hops)
    }

    /// Parsea la salida textual de `tracepath -n`
    pub fn parse_output(text: &str, target: &str) -> Vec<HopInfo> {
        let mut hops: Vec<HopInfo> = Vec::new();

        for line in text.lines() {
            let line = line.trim();
            // Formato de tracepath: " 1:  192.168.1.1  1.293ms" o " 6:  no reply"
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
                                Some("Router Local (Gateway)".to_string())
                            } else if hop_num == 2 && (ip.starts_with("192.168.") || ip.starts_with("10.")) {
                                Some("Modem / ISP Gateway".to_string())
                            } else if hop_num == 3 {
                                Some("Nodo Operador ISP".to_string())
                            } else if ip == target {
                                Some("Destino Final".to_string())
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
