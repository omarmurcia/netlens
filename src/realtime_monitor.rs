use iced::futures::channel::mpsc;
use std::net::Ipv4Addr;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NeighborAction {
    Appeared {
        ip: Ipv4Addr,
        mac: String,
        iface: String,
    },
    Disappeared {
        ip: Ipv4Addr,
        mac: Option<String>,
        iface: String,
    },
}

pub struct RealtimeMonitor;

impl RealtimeMonitor {
    /// Parsea una línea de evento emitida por el kernel a través de `ip monitor neigh`
    pub fn parse_neighbor_line(line: &str) -> Option<NeighborAction> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }

        let is_deleted = line.starts_with("Deleted ") || line.starts_with("deleted ");
        let content = if is_deleted {
            line.strip_prefix("Deleted ")
                .or_else(|| line.strip_prefix("deleted "))?
                .trim()
        } else {
            line
        };

        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        // IP es el primer token (ignorar si es IPv6 u otro formato)
        let ip: Ipv4Addr = parts[0].parse().ok()?;

        let mut iface = String::new();
        let mut mac: Option<String> = None;
        let mut state = String::new();

        let mut i = 1;
        while i < parts.len() {
            match parts[i] {
                "dev" if i + 1 < parts.len() => {
                    iface = parts[i + 1].to_string();
                    i += 2;
                }
                "lladdr" if i + 1 < parts.len() => {
                    mac = Some(parts[i + 1].to_lowercase());
                    i += 2;
                }
                s => {
                    state = s.to_uppercase();
                    i += 1;
                }
            }
        }

        if is_deleted {
            return Some(NeighborAction::Disappeared { ip, mac, iface });
        }

        // Ignorar estados de fallo o incompletos
        if state == "FAILED" || state == "INCOMPLETE" {
            return None;
        }

        if let Some(mac_str) = mac {
            if mac_str != "00:00:00:00:00:00"
                && mac_str != "ff:ff:ff:ff:ff:ff"
                && mac_str.len() >= 11
            {
                return Some(NeighborAction::Appeared {
                    ip,
                    mac: mac_str,
                    iface,
                });
            }
        }

        None
    }

    /// Inicia el listener asíncrono en segundo plano conectado a los eventos del kernel Linux
    pub fn spawn_listener(sender: mpsc::UnboundedSender<NeighborAction>) {
        tokio::spawn(async move {
            loop {
                let mut cmd = Command::new("ip");
                cmd.args(["monitor", "neigh"])
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::null());

                match cmd.spawn() {
                    Ok(mut child) => {
                        if let Some(stdout) = child.stdout.take() {
                            let mut reader = BufReader::new(stdout).lines();
                            while let Ok(Some(line)) = reader.next_line().await {
                                if let Some(action) = Self::parse_neighbor_line(&line) {
                                    if sender.unbounded_send(action).is_err() {
                                        let _ = child.kill().await;
                                        return;
                                    }
                                }
                            }
                        }
                        let _ = child.wait().await;
                    }
                    Err(err) => {
                        eprintln!(
                            "Aviso: No se pudo iniciar monitor netlink en tiempo real: {}",
                            err
                        );
                    }
                }

                // Pausa antes de reintentar si el comando termina
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        });
    }
}
