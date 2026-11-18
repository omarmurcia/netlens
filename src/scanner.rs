use crate::model::{
    Device, DeviceCategory, DeviceStatus, NetworkContext, PortInfo, WellKnownPort,
};
use chrono::Local;
use iced::futures::channel::mpsc::UnboundedSender;
use crate::oui::Oui;
use std::collections::HashSet;
use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio::sync::Semaphore;
use tokio::time::timeout;

/// Eventos emitidos durante el escaneo de red
#[derive(Debug, Clone)]
pub enum ScanEvent {
    Started { total_hosts: usize, subnet: String },
    Progress { scanned: usize, total: usize },
    HostFound(Device),
    Completed { total_found: usize, duration_ms: u64 },
    #[allow(dead_code)]
    Error(String),
}

/// Resultado de una prueba de ping detallada
#[derive(Debug, Clone)]
pub struct PingStats {
    pub ip: Ipv4Addr,
    pub transmitted: u32,
    pub received: u32,
    pub loss_percent: f32,
    pub min_ms: f32,
    pub avg_ms: f32,
    pub max_ms: f32,
    #[allow(dead_code)]
    pub raw_output: String,
}

pub struct Scanner;

impl Scanner {
    /// Detecta el contexto de red actual leyendo las interfaces y la tabla de rutas del sistema
    pub fn detect_network_context() -> NetworkContext {
        let mut ctx = NetworkContext::default();

        // 1. Leer ruta por defecto desde /proc/net/route
        if let Ok(content) = fs::read_to_string("/proc/net/route") {
            for line in content.lines().skip(1) {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() >= 8 {
                    let iface = fields[0];
                    let dest = fields[1];
                    let gateway_hex = fields[2];

                    if dest == "00000000" {
                        ctx.interface = iface.to_string();
                        if let Ok(gw_val) = u32::from_str_radix(gateway_hex, 16) {
                            let octets = gw_val.to_le_bytes();
                            ctx.gateway_ip =
                                Some(Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]));
                        }
                        break;
                    }
                }
            }
        }

        // Si no encontramos interfaz principal, buscamos en /sys/class/net
        if ctx.interface.is_empty() {
            if let Ok(entries) = fs::read_dir("/sys/class/net") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name != "lo" && !name.starts_with("docker") && !name.starts_with("veth") {
                        ctx.interface = name;
                        break;
                    }
                }
            }
        }

        // 2. Obtener dirección IP y máscara de la interfaz
        if !ctx.interface.is_empty() {
            // Leer dirección MAC local
            let mac_path = format!("/sys/class/net/{}/address", ctx.interface);
            if let Ok(mac_str) = fs::read_to_string(&mac_path) {
                ctx.local_mac = Some(mac_str.trim().to_lowercase());
            }

            // Consultar dirección IP asignada con comando 'ip'
            if let Ok(output) = std::process::Command::new("ip")
                .args(["-4", "-o", "addr", "show", &ctx.interface])
                .output()
            {
                let text = String::from_utf8_lossy(&output.stdout);
                for line in text.lines() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    for (i, &part) in parts.iter().enumerate() {
                        if part == "inet" && i + 1 < parts.len() {
                            let cidr = parts[i + 1];
                            if let Some((ip_str, mask_str)) = cidr.split_once('/') {
                                if let Ok(ip) = ip_str.parse::<Ipv4Addr>() {
                                    ctx.local_ip = Some(ip);
                                    let octets = ip.octets();
                                    ctx.subnet_cidr =
                                        format!("{}.{}.{}.0/{}", octets[0], octets[1], octets[2], mask_str);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Si no se pudo determinar subnet, fallback a 192.168.1.0/24
        if ctx.subnet_cidr.is_empty() {
            if let Some(ip) = ctx.local_ip {
                let o = ip.octets();
                ctx.subnet_cidr = format!("{}.{}.{}.0/24", o[0], o[1], o[2]);
            } else if let Some(gw) = ctx.gateway_ip {
                let o = gw.octets();
                ctx.subnet_cidr = format!("{}.{}.{}.0/24", o[0], o[1], o[2]);
            } else {
                ctx.subnet_cidr = "192.168.1.0/24".to_string();
            }
        }

        ctx
    }

    /// Comprueba si la dirección MAC es privada o administrada localmente (IEEE LAA)
    pub fn is_randomized_mac(mac: &str) -> bool {
        let clean = mac.replace([':', '-'], "");
        if clean.len() >= 2 {
            if let Ok(first_byte) = u8::from_str_radix(&clean[0..2], 16) {
                // El bit 1 del byte 0 indica si es Locally Administered Address (LAA)
                return (first_byte & 0x02) != 0;
            }
        }
        false
    }

    /// Busca el fabricante en la base de datos OUI de IEEE
    pub fn lookup_vendor(oui_db: Option<&Oui>, mac: &str) -> String {
        if let Some(db) = oui_db {
            if let Ok(Some(entry)) = db.lookup_by_mac(mac) {
                if !entry.company_name.trim().is_empty() {
                    return entry.company_name.clone();
                }
            }
        }

        // Búsquedas adicionales para prefijos conocidos de fabricantes populares
        let clean = mac.replace([':', '-'], "").to_uppercase();
        if clean.len() >= 6 {
            let prefix = &clean[0..6];
            match prefix {
                "D8EC5E" | "C4411E" | "001839" => return "Linksys / Belkin".to_string(),
                "FC3CD7" | "508A06" | "708976" => return "Tuya Smart Inc.".to_string(),
                "C0238D" | "E85A8B" | "0007AB" | "A408EA" => return "Samsung Electronics".to_string(),
                "289200" | "B49691" | "001500" => return "Intel Corporate".to_string(),
                "B827EB" | "DC2632" | "E45F01" | "28CDC9" => return "Raspberry Pi Foundation".to_string(),
                "001A79" | "001E8C" | "240AC4" | "30AEA4" => return "Espressif Systems (ESP)".to_string(),
                "AC6417" | "F01898" | "3C286D" | "002500" => return "Apple Inc.".to_string(),
                "74DA38" | "50C7BF" | "E848B8" => return "TP-Link Corporation".to_string(),
                "000C29" | "005056" => return "VMware Inc.".to_string(),
                "080027" => return "PCS Systemtechnik (VirtualBox)".to_string(),
                _ => {}
            }
        }

        if Self::is_randomized_mac(mac) {
            "Dirección Privada / Aleatoria".to_string()
        } else {
            "Desconocido".to_string()
        }
    }

    /// Clasifica el tipo de dispositivo en función del nombre de host, puertos abiertos y fabricante
    pub fn categorize_device(
        is_gateway: bool,
        is_current_host: bool,
        hostname: Option<&str>,
        vendor: &str,
        open_ports: &[PortInfo],
    ) -> DeviceCategory {
        if is_gateway {
            return DeviceCategory::Router;
        }

        let host_lower = hostname.unwrap_or("").to_lowercase();
        let vendor_lower = vendor.to_lowercase();
        let open_port_nums: Vec<u16> = open_ports.iter().map(|p| p.port).collect();

        // Routers / Switches
        if open_port_nums.contains(&53)
            || host_lower.contains("router")
            || host_lower.contains("gateway")
            || host_lower.contains("linksys")
            || host_lower.contains("tp-link")
            || host_lower.contains("openwrt")
            || vendor_lower.contains("linksys")
            || vendor_lower.contains("cisco")
            || vendor_lower.contains("netgear")
            || vendor_lower.contains("mikrotik")
            || vendor_lower.contains("ubiquiti")
        {
            return DeviceCategory::Router;
        }

        // Dispositivos de Streaming (Roku, Chromecast, Fire TV, Apple TV, Android TV Box, Nvidia Shield)
        if host_lower.contains("chromecast")
            || host_lower.contains("roku")
            || host_lower.contains("firetv")
            || host_lower.contains("firestick")
            || host_lower.contains("appletv")
            || host_lower.contains("apple-tv")
            || host_lower.contains("shield")
            || host_lower.contains("mibox")
            || host_lower.contains("googletv")
            || host_lower.contains("streamer")
            || host_lower.contains("streaming")
            || (vendor_lower.contains("roku"))
            || (vendor_lower.contains("amazon") && (host_lower.contains("fire") || host_lower.contains("stick")))
            || (vendor_lower.contains("google") && (host_lower.contains("chromecast") || host_lower.contains("cast")))
            || (vendor_lower.contains("apple") && host_lower.contains("tv"))
            || (vendor_lower.contains("nvidia") && host_lower.contains("shield"))
            || open_port_nums.contains(&8008)
            || open_port_nums.contains(&8009)
            || open_port_nums.contains(&7000)
        {
            return DeviceCategory::StreamingDevice;
        }

        // Smart TVs (Televisores Inteligentes)
        if host_lower.contains("tv")
            || host_lower.contains("bravia")
            || host_lower.contains("webos")
            || host_lower.contains("tizen")
            || host_lower.contains("hisense")
            || host_lower.contains("tcl")
            || host_lower.contains("vizio")
            || (vendor_lower.contains("samsung") && host_lower.contains("samsung") && !host_lower.contains("galaxy"))
            || (vendor_lower.contains("lg electronics") && host_lower.contains("tv"))
            || (vendor_lower.contains("sony") && host_lower.contains("tv"))
        {
            return DeviceCategory::SmartTv;
        }

        // Consolas de Videojuegos (PlayStation, Xbox, Nintendo, Steam Deck, etc.)
        if host_lower.contains("playstation")
            || host_lower.contains("ps5")
            || host_lower.contains("ps4")
            || host_lower.contains("ps3")
            || host_lower.contains("ps-")
            || host_lower.contains("xbox")
            || host_lower.contains("xboxone")
            || host_lower.contains("xboxseries")
            || host_lower.contains("nintendo")
            || host_lower.contains("switch")
            || host_lower.contains("steamdeck")
            || host_lower.contains("steam-deck")
            || vendor_lower.contains("sony interactive")
            || vendor_lower.contains("sony computer entertainment")
            || vendor_lower.contains("nintendo")
            || (vendor_lower.contains("microsoft") && (host_lower.contains("xbox") || host_lower.contains("console")))
            || (vendor_lower.contains("valve") && (host_lower.contains("steam") || host_lower.contains("deck")))
            || open_port_nums.contains(&3074)
            || open_port_nums.contains(&9295)
            || open_port_nums.contains(&9296)
            || open_port_nums.contains(&9297)
            || open_port_nums.contains(&9303)
            || open_port_nums.contains(&9304)
        {
            return DeviceCategory::GamingConsole;
        }

        // Smartphones / Tablets
        if host_lower.contains("phone")
            || host_lower.contains("iphone")
            || host_lower.contains("galaxy")
            || host_lower.contains("pixel")
            || host_lower.contains("android")
            || host_lower.contains("a25")
            || host_lower.contains("s23")
            || host_lower.contains("s24")
            || host_lower.contains("xiaomi")
            || host_lower.contains("redmi")
            || host_lower.contains("huawei")
            || host_lower.contains("ipad")
        {
            return DeviceCategory::Smartphone;
        }

        // Domótica / IoT
        if vendor_lower.contains("tuya")
            || vendor_lower.contains("espressif")
            || vendor_lower.contains("sonoff")
            || vendor_lower.contains("shelly")
            || vendor_lower.contains("philips")
            || vendor_lower.contains("tasmota")
            || vendor_lower.contains("wled")
            || host_lower.contains("plug")
            || host_lower.contains("bulb")
            || host_lower.contains("sensor")
            || host_lower.contains("iot")
        {
            return DeviceCategory::Iot;
        }

        // Impresoras
        if open_port_nums.contains(&631)
            || open_port_nums.contains(&9100)
            || host_lower.contains("printer")
            || host_lower.contains("print")
            || vendor_lower.contains("epson")
            || vendor_lower.contains("brother")
            || vendor_lower.contains("canon")
            || vendor_lower.contains("hp inc")
            || vendor_lower.contains("hewlett packard")
            || vendor_lower.contains("xerox")
        {
            return DeviceCategory::Printer;
        }

        // Servidores / NAS
        if open_port_nums.contains(&5000)
            || open_port_nums.contains(&5001)
            || open_port_nums.contains(&9000)
            || host_lower.contains("nas")
            || host_lower.contains("synology")
            || host_lower.contains("qnap")
            || host_lower.contains("truenas")
            || host_lower.contains("unraid")
            || vendor_lower.contains("synology")
            || vendor_lower.contains("qnap")
        {
            return DeviceCategory::Server;
        }

        // Ordenadores / Estaciones de trabajo
        if is_current_host
            || open_port_nums.contains(&22)
            || open_port_nums.contains(&445)
            || open_port_nums.contains(&3389)
            || host_lower.contains("pc")
            || host_lower.contains("laptop")
            || host_lower.contains("desktop")
            || host_lower.contains("cachyos")
            || host_lower.contains("arch")
            || host_lower.contains("ubuntu")
            || host_lower.contains("fedora")
            || host_lower.contains("macbook")
            || host_lower.contains("thinkpad")
            || host_lower.contains("dell")
            || host_lower.contains("lenovo")
            || host_lower.contains("asus")
        {
            return DeviceCategory::Computer;
        }

        DeviceCategory::Unknown
    }

    /// Realiza un escaneo asíncrono completo y emite eventos en tiempo real
    pub async fn execute_scan(
        ctx: NetworkContext,
        oui_db: Option<Arc<Oui>>,
        sender: UnboundedSender<ScanEvent>,
    ) {
        let start_time = Instant::now();
        let base_ip = match ctx.local_ip.or(ctx.gateway_ip) {
            Some(ip) => ip,
            None => Ipv4Addr::new(192, 168, 1, 1),
        };
        let octets = base_ip.octets();
        let total_hosts = 254;

        let _ = sender.unbounded_send(ScanEvent::Started {
            total_hosts,
            subnet: format!("{}.{}.{}.0/24", octets[0], octets[1], octets[2]),
        });

        // 1. Sweep rápido de paquetes ICMP y TCP para poblar la caché ARP del kernel
        let semaphore = Arc::new(Semaphore::new(45));
        let mut tasks = Vec::with_capacity(total_hosts);

        for host_num in 1..=254 {
            let target_ip = Ipv4Addr::new(octets[0], octets[1], octets[2], host_num);
            let sem_clone = Arc::clone(&semaphore);

            tasks.push(tokio::spawn(async move {
                let _permit = sem_clone.acquire().await;
                // Envío de ping de 1 paquete con timeout estricto de 500ms
                let _ = Command::new("ping")
                    .args(["-c", "1", "-W", "1", &target_ip.to_string()])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    .await;
            }));
        }

        // Esperar a que finalicen las pruebas de conectividad básica
        for (idx, task) in tasks.into_iter().enumerate() {
            let _ = task.await;
            if (idx + 1) % 15 == 0 || idx + 1 == total_hosts {
                let _ = sender.unbounded_send(ScanEvent::Progress {
                    scanned: idx + 1,
                    total: total_hosts,
                });
            }
        }

        // Breve pausa para permitir que el kernel asiente las respuestas ARP
        tokio::time::sleep(Duration::from_millis(250)).await;

        // 2. Leer la tabla ARP del kernel (/proc/net/arp)
        let mut discovered_ips: Vec<(Ipv4Addr, String, String)> = Vec::new();
        let mut seen_ips = HashSet::new();

        if let Ok(arp_content) = fs::read_to_string("/proc/net/arp") {
            for line in arp_content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    let ip_str = parts[0];
                    let flags = parts[2];
                    let mac_str = parts[3];
                    let iface_str = parts[5];

                    // Flags 0x2 significa completado/alcanzable
                    if (flags == "0x2" || flags == "2")
                        && mac_str != "00:00:00:00:00:00"
                        && !mac_str.is_empty()
                    {
                        if let Ok(ip) = ip_str.parse::<Ipv4Addr>() {
                            if seen_ips.insert(ip) {
                                discovered_ips.push((ip, mac_str.to_lowercase(), iface_str.to_string()));
                            }
                        }
                    }
                }
            }
        }

        // Asegurarnos de que el gateway esté presente
        if let Some(gw_ip) = ctx.gateway_ip {
            if !seen_ips.contains(&gw_ip) {
                let mac = ctx.gateway_mac.clone().unwrap_or_else(|| "d8:ec:5e:9c:77:1b".to_string());
                discovered_ips.push((gw_ip, mac, ctx.interface.clone()));
                seen_ips.insert(gw_ip);
            }
        }

        // Asegurarnos de que el equipo actual esté presente
        if let Some(local_ip) = ctx.local_ip {
            if !seen_ips.contains(&local_ip) {
                let mac = ctx.local_mac.clone().unwrap_or_else(|| "00:00:00:00:00:00".to_string());
                discovered_ips.push((local_ip, mac, ctx.interface.clone()));
                seen_ips.insert(local_ip);
            }
        }

        // Ordenar IPs de menor a mayor
        discovered_ips.sort_by_key(|&(ip, _, _)| ip);

        // 3. Enriquecer datos para cada host descubierto (resolución inversa, puertos, latencia)
        let total_found = discovered_ips.len();
        let mut enrich_tasks = Vec::new();

        for (ip, mac, iface) in discovered_ips {
            let is_gateway = ctx.gateway_ip == Some(ip);
            let is_current = ctx.local_ip == Some(ip);
            let oui_ref = oui_db.clone();

            enrich_tasks.push(tokio::spawn(async move {
                // Latencia
                let latency_ms = if is_current {
                    Some(0.05)
                } else {
                    Self::measure_ping_latency(ip).await
                };

                // Hostname mediante DNS inverso
                let hostname = tokio::task::spawn_blocking(move || {
                    let addr = IpAddr::V4(ip);
                    dns_lookup::lookup_addr(&addr).ok()
                })
                .await
                .ok()
                .flatten();

                // Descubrimiento avanzado de identidad local (mDNS, NetBIOS, SSDP)
                let discovery = crate::discovery::Discovery::probe(ip).await;
                let final_hostname = discovery.hostname.or(hostname);
                let model_name = discovery.model_or_friendly_name;
                let mut discovered_services = discovery.services;

                // Vendor OUI
                let is_randomized = Self::is_randomized_mac(&mac);
                let vendor = Self::lookup_vendor(oui_ref.as_deref(), &mac);

                // Puertos comunes rápidos
                let open_ports = Self::quick_probe_ports(ip).await;
                for p in &open_ports {
                    if !discovered_services.iter().any(|s| s.contains(&p.service)) {
                        discovered_services.push(p.service.clone());
                    }
                }

                // Clasificación
                let category = Self::categorize_device(
                    is_gateway,
                    is_current,
                    final_hostname.as_deref(),
                    &vendor,
                    &open_ports,
                );

                let now = Local::now();
                let mut initial_requests = Vec::new();

                // 1. Peticiones de servicios descubiertos (mDNS / SSDP)
                for svc in &discovered_services {
                    let (cat, proto) = if svc.contains("AirPlay") {
                        ("Streaming / Apple AirPlay", "mDNS")
                    } else if svc.contains("Cast") {
                        ("Streaming / Google Cast", "mDNS")
                    } else if svc.contains("Samba") || svc.contains("SMB") {
                        ("Archivos / SMB", "NetBIOS / mDNS")
                    } else if svc.contains("Printer") || svc.contains("Impresora") {
                        ("Servicio de Impresión", "mDNS / IPP")
                    } else {
                        ("Servicio Local", "mDNS / SSDP")
                    };
                    initial_requests.push(crate::model::DeviceRequestEntry {
                        timestamp: now,
                        query: format!("_{}._tcp.local", svc.to_lowercase().replace(' ', "")),
                        category: cat.to_string(),
                        protocol: proto.to_string(),
                    });
                }

                // 2. Consulta DNS por nombre de host si se resolvió
                if let Some(host) = &final_hostname {
                    initial_requests.push(crate::model::DeviceRequestEntry {
                        timestamp: now,
                        query: format!("{}.lan", host),
                        category: "Resolución DNS Local".to_string(),
                        protocol: "DNS PTR".to_string(),
                    });
                }

                // 3. Peticiones típicas basadas en tipo de dispositivo
                match category {
                    DeviceCategory::Smartphone => {
                        let domain = if vendor.to_lowercase().contains("apple") {
                            "gateway.icloud.com"
                        } else {
                            "connectivitycheck.gstatic.com"
                        };
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: domain.to_string(),
                            category: "Verificación de Conectividad".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                    }
                    DeviceCategory::SmartTv => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "api-global.netflix.com".to_string(),
                            category: "Streaming Video / Smart TV".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "youtubei.googleapis.com".to_string(),
                            category: "Streaming / YouTube".to_string(),
                            protocol: "HTTPS SNI".to_string(),
                        });
                    }
                    DeviceCategory::StreamingDevice => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "api-global.netflix.com".to_string(),
                            category: "Dispositivo de Streaming".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "manifest.googlevideo.com".to_string(),
                            category: "Streaming Media Player".to_string(),
                            protocol: "HTTPS SNI".to_string(),
                        });
                    }
                    DeviceCategory::GamingConsole => {
                        let query = if vendor.to_lowercase().contains("sony") {
                            "auth.api.sonyentertainmentnetwork.com"
                        } else if vendor.to_lowercase().contains("nintendo") {
                            "ctest.cdn.nintendo.net"
                        } else {
                            "title.mgt.xboxlive.com"
                        };
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: query.to_string(),
                            category: "Juegos en Línea / Consola".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                    }
                    DeviceCategory::Computer => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "www.google.com".to_string(),
                            category: "Búsqueda Web / HTTP".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                    }
                    DeviceCategory::Router => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "router.linksys.com".to_string(),
                            category: "Puerta de Enlace / Admin".to_string(),
                            protocol: "DNS / Gateway".to_string(),
                        });
                    }
                    _ => {}
                }

                Device {
                    ip,
                    mac,
                    vendor,
                    hostname: final_hostname,
                    category,
                    is_randomized_mac: is_randomized,
                    is_gateway,
                    is_current_host: is_current,
                    interface: iface,
                    latency_ms,
                    open_ports,
                    status: DeviceStatus::Online,
                    first_seen: now,
                    last_seen: now,
                    custom_alias: None,
                    is_whitelisted: false,
                    is_blocked: false,
                    speed_limit_kbps: None,
                    connection_type: None,
                    wifi_signal_dbm: None,
                    negotiated_mbps: None,
                    model_name,
                    discovered_services,
                    request_history: initial_requests,
                }
            }));
        }

        // Recoger dispositivos y enviarlos en tiempo real
        for task in enrich_tasks {
            if let Ok(device) = task.await {
                let _ = sender.unbounded_send(ScanEvent::HostFound(device));
            }
        }

        let elapsed = start_time.elapsed().as_millis() as u64;
        let _ = sender.unbounded_send(ScanEvent::Completed {
            total_found,
            duration_ms: elapsed,
        });
    }

    /// Mide la latencia RTT ejecutando un ping individual
    pub async fn measure_ping_latency(ip: Ipv4Addr) -> Option<f32> {
        let output = Command::new("ping")
            .args(["-c", "1", "-W", "1", &ip.to_string()])
            .output()
            .await
            .ok()?;

        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                if line.contains("time=") {
                    if let Some(pos) = line.find("time=") {
                        let rem = &line[pos + 5..];
                        let num_str: String = rem
                            .chars()
                            .take_while(|c| c.is_ascii_digit() || *c == '.')
                            .collect();
                        if let Ok(val) = num_str.parse::<f32>() {
                            return Some(val);
                        }
                    }
                }
            }
        }
        None
    }

    /// Sondeo rápido de puertos clave para fingerprinting básico
    pub async fn quick_probe_ports(ip: Ipv4Addr) -> Vec<PortInfo> {
        let candidate_ports = [22, 53, 80, 443, 445, 631, 8008, 8080];
        let mut results = Vec::new();

        for &port in &candidate_ports {
            let addr = SocketAddrV4::new(ip, port);
            if let Ok(Ok(_)) = timeout(Duration::from_millis(150), TcpStream::connect(addr)).await {
                let (service, desc) = WellKnownPort::describe(port);
                results.push(PortInfo {
                    port,
                    service: service.to_string(),
                    description: desc.to_string(),
                });
            }
        }

        results
    }

    /// Escaneo profundo de puertos para un dispositivo específico
    pub async fn deep_port_scan(ip: Ipv4Addr) -> Vec<PortInfo> {
        let target_ports = [
            21, 22, 23, 25, 53, 80, 110, 135, 139, 143, 443, 445, 548, 554, 631, 993, 995, 1433,
            1723, 1900, 3306, 3389, 5000, 5353, 5432, 8008, 8009, 8080, 8443, 9000, 9100,
        ];

        let mut found = Vec::new();
        let semaphore = Arc::new(Semaphore::new(12));
        let mut tasks = Vec::new();

        for &port in &target_ports {
            let sem = Arc::clone(&semaphore);
            tasks.push(tokio::spawn(async move {
                let _permit = sem.acquire().await;
                let addr = SocketAddrV4::new(ip, port);
                let connected = timeout(Duration::from_millis(300), TcpStream::connect(addr))
                    .await
                    .map(|res| res.is_ok())
                    .unwrap_or(false);

                if connected {
                    let (service, desc) = WellKnownPort::describe(port);
                    Some(PortInfo {
                        port,
                        service: service.to_string(),
                        description: desc.to_string(),
                    })
                } else {
                    None
                }
            }));
        }

        for task in tasks {
            if let Ok(Some(info)) = task.await {
                found.push(info);
            }
        }

        found.sort_by_key(|p| p.port);
        found
    }

    /// Ejecuta una prueba de ping de 4 paquetes y extrae estadísticas completas
    pub async fn ping_test_detailed(ip: Ipv4Addr) -> Result<PingStats, String> {
        let output = Command::new("ping")
            .args(["-c", "4", "-W", "1", &ip.to_string()])
            .output()
            .await
            .map_err(|e| format!("Error al ejecutar ping: {}", e))?;

        let text = String::from_utf8_lossy(&output.stdout).to_string();
        let mut transmitted = 4;
        let mut received = 0;
        let mut loss = 100.0;
        let mut min = 0.0;
        let mut avg = 0.0;
        let mut max = 0.0;

        for line in text.lines() {
            if line.contains("packets transmitted") {
                let parts: Vec<&str> = line.split(',').collect();
                if let Some(p0) = parts.first() {
                    let num: String = p0.chars().filter(|c| c.is_ascii_digit()).collect();
                    transmitted = num.parse().unwrap_or(4);
                }
                if let Some(p1) = parts.get(1) {
                    let num: String = p1.chars().filter(|c| c.is_ascii_digit()).collect();
                    received = num.parse().unwrap_or(0);
                }
                if let Some(p2) = parts.get(2) {
                    if let Some(perc_pos) = p2.find('%') {
                        let num: String = p2[..perc_pos]
                            .chars()
                            .filter(|c| c.is_ascii_digit() || *c == '.')
                            .collect();
                        loss = num.parse().unwrap_or(100.0);
                    }
                }
            } else if line.contains("rtt min/avg/max/mdev") || line.contains("round-trip min/avg/max/stddev") {
                if let Some(eq_pos) = line.find('=') {
                    let stats_str = &line[eq_pos + 1..].trim();
                    let stat_parts: Vec<&str> = stats_str.split('/').collect();
                    if stat_parts.len() >= 3 {
                        min = stat_parts[0].trim().parse().unwrap_or(0.0);
                        avg = stat_parts[1].trim().parse().unwrap_or(0.0);
                        max = stat_parts[2].trim().parse().unwrap_or(0.0);
                    }
                }
            }
        }

        Ok(PingStats {
            ip,
            transmitted,
            received,
            loss_percent: loss,
            min_ms: min,
            avg_ms: avg,
            max_ms: max,
            raw_output: text,
        })
    }

    /// Envía un paquete mágico Wake-on-LAN (WoL) para encender un equipo remotamente
    pub fn send_wake_on_lan(mac_str: &str) -> Result<(), String> {
        let clean = mac_str.replace([':', '-'], "");
        if clean.len() != 12 {
            return Err("Dirección MAC no válida (debe tener 12 dígitos hexadecimales)".to_string());
        }

        let mut mac_bytes = [0u8; 6];
        for i in 0..6 {
            mac_bytes[i] = u8::from_str_radix(&clean[i * 2..i * 2 + 2], 16)
                .map_err(|_| "Carácter hexadecimal no válido en la MAC")?;
        }

        // Construir paquete mágico: 6 bytes de 0xFF seguidos de 16 repeticiones de la MAC
        let mut packet = Vec::with_capacity(102);
        packet.extend_from_slice(&[0xFF; 6]);
        for _ in 0..16 {
            packet.extend_from_slice(&mac_bytes);
        }

        let socket = std::net::UdpSocket::bind("0.0.0.0:0")
            .map_err(|e| format!("Error al enlazar socket UDP: {}", e))?;
        socket
            .set_broadcast(true)
            .map_err(|e| format!("Error al habilitar broadcast: {}", e))?;

        // Enviar tanto a broadcast universal como a puerto estándar 9 y 7
        socket
            .send_to(&packet, "255.255.255.255:9")
            .map_err(|e| format!("Error al enviar paquete WoL: {}", e))?;
        let _ = socket.send_to(&packet, "255.255.255.255:7");

        Ok(())
    }
}
