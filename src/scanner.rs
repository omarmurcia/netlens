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

/// Events emitted during network scanning
#[derive(Debug, Clone)]
pub enum ScanEvent {
    Started { total_hosts: usize, subnet: String },
    Progress { scanned: usize, total: usize },
    HostFound(Device),
    Completed { total_found: usize, duration_ms: u64 },
    #[allow(dead_code)]
    Error(String),
}

/// Detailed ping test result
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
    /// Detects current network context by inspecting interfaces and system routing table
    pub fn detect_network_context() -> NetworkContext {
        let mut ctx = NetworkContext::default();

        // 1. Read default route from /proc/net/route
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

        // If no default interface was found, search /sys/class/net
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

        // 2. Obtain IP address and netmask for the interface
        if !ctx.interface.is_empty() {
            // Read local MAC address
            let mac_path = format!("/sys/class/net/{}/address", ctx.interface);
            if let Ok(mac_str) = fs::read_to_string(&mac_path) {
                ctx.local_mac = Some(mac_str.trim().to_lowercase());
            }

            // Query assigned IP address using 'ip' command
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

        // If subnet could not be determined, fallback to 192.168.1.0/24
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

    /// Checks if a MAC address is private / locally administered (IEEE LAA)
    pub fn is_randomized_mac(mac: &str) -> bool {
        let clean = mac.replace([':', '-'], "");
        if clean.len() >= 2 {
            if let Ok(first_byte) = u8::from_str_radix(&clean[0..2], 16) {
                // Bit 1 of byte 0 indicates Locally Administered Address (LAA)
                return (first_byte & 0x02) != 0;
            }
        }
        false
    }

    /// Looks up the manufacturer in the IEEE OUI database
    pub fn lookup_vendor(oui_db: Option<&Oui>, mac: &str) -> String {
        if let Some(db) = oui_db {
            if let Ok(Some(entry)) = db.lookup_by_mac(mac) {
                if !entry.company_name.trim().is_empty() {
                    return entry.company_name.clone();
                }
            }
        }

        // Additional lookups for well-known popular vendor prefixes
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
            "Private / Randomized Address".to_string()
        } else {
            "Unknown".to_string()
        }
    }

    /// Categorizes device type based on hostname, open ports, and manufacturer
    pub fn categorize_device(
        is_gateway: bool,
        is_current_host: bool,
        hostname: Option<&str>,
        vendor: &str,
        open_ports: &[PortInfo],
    ) -> DeviceCategory {
        Self::categorize_device_advanced(
            is_gateway,
            is_current_host,
            hostname,
            vendor,
            open_ports,
            &[],
            None,
        )
    }

    /// Advanced multi-layered device categorization engine
    pub fn categorize_device_advanced(
        is_gateway: bool,
        is_current_host: bool,
        hostname: Option<&str>,
        vendor: &str,
        open_ports: &[PortInfo],
        discovered_services: &[String],
        model_name: Option<&str>,
    ) -> DeviceCategory {
        if is_gateway {
            return DeviceCategory::Router;
        }

        let host_lower = hostname.unwrap_or("").to_lowercase();
        let vendor_lower = vendor.to_lowercase();
        let model_lower = model_name.unwrap_or("").to_lowercase();
        let open_port_nums: Vec<u16> = open_ports.iter().map(|p| p.port).collect();
        let svc_str = discovered_services.join(" ").to_lowercase();

        // 1. Security Cameras / Surveillance / NVR
        if open_port_nums.contains(&554)
            || open_port_nums.contains(&8554)
            || svc_str.contains("camera")
            || svc_str.contains("onvif")
            || svc_str.contains("rtsp")
            || svc_str.contains("axis-video")
            || host_lower.contains("camera")
            || host_lower.contains("cam")
            || host_lower.contains("camara")
            || host_lower.contains("cctv")
            || host_lower.contains("ipcam")
            || host_lower.contains("webcam")
            || host_lower.contains("doorbell")
            || host_lower.contains("timbre")
            || host_lower.contains("nvr")
            || host_lower.contains("dvr")
            || host_lower.contains("ring")
            || host_lower.contains("reolink")
            || host_lower.contains("hikvision")
            || host_lower.contains("dahua")
            || host_lower.contains("wyze")
            || host_lower.contains("eufy")
            || host_lower.contains("amcrest")
            || host_lower.contains("foscam")
            || host_lower.contains("uniview")
            || host_lower.contains("arlo")
            || model_lower.contains("camera")
            || model_lower.contains("doorbell")
            || model_lower.contains("cam")
            || vendor_lower.contains("hikvision")
            || vendor_lower.contains("dahua")
            || vendor_lower.contains("reolink")
            || vendor_lower.contains("ring")
            || vendor_lower.contains("nest labs")
            || vendor_lower.contains("arlo")
            || vendor_lower.contains("wyze")
            || vendor_lower.contains("axis communications")
            || vendor_lower.contains("foscam")
            || vendor_lower.contains("amcrest")
            || vendor_lower.contains("uniview")
            || vendor_lower.contains("hanwha")
            || vendor_lower.contains("vivotek")
            || vendor_lower.contains("swann")
            || vendor_lower.contains("lorex")
        {
            return DeviceCategory::SecurityCamera;
        }

        // 2. Smart Speakers & Audio Systems (Sonos, Echo, HomePod, Nest Audio)
        if open_port_nums.contains(&1400)
            || svc_str.contains("sonos")
            || svc_str.contains("spotify connect")
            || svc_str.contains("raop")
            || host_lower.contains("sonos")
            || host_lower.contains("soundbar")
            || host_lower.contains("speaker")
            || host_lower.contains("altavoz")
            || host_lower.contains("bocina")
            || host_lower.contains("echo")
            || host_lower.contains("alexa")
            || host_lower.contains("homepod")
            || host_lower.contains("nest-audio")
            || host_lower.contains("nest-mini")
            || host_lower.contains("google-home")
            || host_lower.contains("jbl-link")
            || model_lower.contains("sonos")
            || model_lower.contains("echo")
            || model_lower.contains("speaker")
            || model_lower.contains("homepod")
            || vendor_lower.contains("sonos")
            || vendor_lower.contains("bose")
            || vendor_lower.contains("harman")
            || vendor_lower.contains("jbl")
            || vendor_lower.contains("bang & olufsen")
            || vendor_lower.contains("denon")
            || vendor_lower.contains("marantz")
            || vendor_lower.contains("marshall")
            || vendor_lower.contains("klipsch")
            || (vendor_lower.contains("amazon") && (host_lower.contains("echo") || host_lower.contains("dot") || host_lower.contains("alexa")) && !host_lower.contains("fire"))
        {
            return DeviceCategory::SmartSpeaker;
        }

        // 3. Network Printers (IPP 631, JetDirect 9100)
        if open_port_nums.contains(&631)
            || open_port_nums.contains(&9100)
            || svc_str.contains("ipp")
            || svc_str.contains("printer")
            || svc_str.contains("cups")
            || host_lower.contains("printer")
            || host_lower.contains("print")
            || host_lower.contains("impresora")
            || host_lower.contains("laserjet")
            || host_lower.contains("deskjet")
            || host_lower.contains("officejet")
            || host_lower.contains("inkjet")
            || model_lower.contains("printer")
            || model_lower.contains("laserjet")
            || vendor_lower.contains("epson")
            || vendor_lower.contains("brother")
            || vendor_lower.contains("canon")
            || vendor_lower.contains("hp inc")
            || vendor_lower.contains("hewlett packard")
            || vendor_lower.contains("xerox")
            || vendor_lower.contains("kyocera")
            || vendor_lower.contains("lexmark")
            || vendor_lower.contains("ricoh")
            || vendor_lower.contains("konica minolta")
        {
            return DeviceCategory::Printer;
        }

        // 4. Video Game Consoles (PlayStation, Xbox, Nintendo, Steam Deck)
        if host_lower.contains("playstation")
            || host_lower.contains("ps5")
            || host_lower.contains("ps4")
            || host_lower.contains("ps3")
            || host_lower.contains("ps-")
            || host_lower.contains("xbox")
            || host_lower.contains("xboxone")
            || host_lower.contains("xboxseries")
            || host_lower.contains("nintendo")
            || host_lower.contains("nintendo-switch")
            || host_lower.contains("switch-oled")
            || (host_lower.contains("switch") && (vendor_lower.contains("nintendo") || host_lower.contains("oled") || host_lower.contains("lite")))
            || host_lower.contains("steamdeck")
            || host_lower.contains("steam-deck")
            || model_lower.contains("playstation")
            || model_lower.contains("xbox")
            || model_lower.contains("nintendo")
            || model_lower.contains("steam deck")
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

        // 5. Streaming Devices (Roku, Chromecast, Fire TV, Apple TV, Android TV Box, Nvidia Shield)
        if host_lower.contains("chromecast")
            || host_lower.contains("roku")
            || host_lower.contains("firetv")
            || host_lower.contains("firestick")
            || host_lower.contains("fire-tv")
            || host_lower.contains("appletv")
            || host_lower.contains("apple-tv")
            || host_lower.contains("shield")
            || host_lower.contains("mibox")
            || host_lower.contains("googletv")
            || host_lower.contains("streamer")
            || host_lower.contains("streaming")
            || host_lower.contains("anycast")
            || host_lower.contains("tvbox")
            || model_lower.contains("chromecast")
            || model_lower.contains("roku")
            || model_lower.contains("apple tv")
            || model_lower.contains("fire tv")
            || (vendor_lower.contains("roku"))
            || (vendor_lower.contains("amazon") && (host_lower.contains("firetv") || host_lower.contains("firestick") || host_lower.contains("fire-tv") || host_lower.contains("stick") || host_lower.contains("aft")) && !host_lower.contains("kindle") && !host_lower.contains("tab"))
            || (vendor_lower.contains("google") && (host_lower.contains("chromecast") || host_lower.contains("cast")))
            || (vendor_lower.contains("apple") && host_lower.contains("tv"))
            || (vendor_lower.contains("nvidia") && host_lower.contains("shield"))
            || open_port_nums.contains(&8008)
            || open_port_nums.contains(&8009)
            || open_port_nums.contains(&7000)
            || svc_str.contains("google cast")
            || svc_str.contains("dial")
        {
            return DeviceCategory::StreamingDevice;
        }

        // 6. Smart TVs
        if host_lower.contains("tv")
            || host_lower.contains("bravia")
            || host_lower.contains("webos")
            || host_lower.contains("tizen")
            || host_lower.contains("hisense")
            || host_lower.contains("tcl")
            || host_lower.contains("vizio")
            || host_lower.contains("skyworth")
            || model_lower.contains("tv")
            || model_lower.contains("smart tv")
            || (vendor_lower.contains("samsung") && host_lower.contains("samsung") && !host_lower.contains("galaxy") && !host_lower.contains("tab"))
            || (vendor_lower.contains("lg electronics") && (host_lower.contains("tv") || host_lower.contains("webos")))
            || (vendor_lower.contains("sony") && host_lower.contains("tv"))
            || vendor_lower.contains("tcl")
            || vendor_lower.contains("hisense")
            || vendor_lower.contains("vizio")
            || svc_str.contains("smart tv")
            || svc_str.contains("mediarenderer")
        {
            return DeviceCategory::SmartTv;
        }

        // 7. Tablets & E-Readers (Separate from Smartphones)
        if host_lower.contains("ipad")
            || host_lower.contains("tablet")
            || host_lower.contains("tab-")
            || host_lower.contains("tab_")
            || host_lower.contains("galaxy-tab")
            || host_lower.contains("galaxytab")
            || host_lower.contains("kindle")
            || host_lower.contains("fire-tablet")
            || host_lower.contains("fire-hd")
            || host_lower.contains("firehd")
            || host_lower.contains("kobo")
            || host_lower.contains("remarkable")
            || host_lower.contains("lenovotab")
            || host_lower.contains("lenovo-tab")
            || host_lower.contains("matepad")
            || host_lower.contains("mediapad")
            || model_lower.contains("ipad")
            || model_lower.contains("tablet")
            || model_lower.contains("kindle")
        {
            return DeviceCategory::Tablet;
        }

        // 8. Network Storage / NAS (Synology, QNAP, TrueNAS, Unraid)
        if open_port_nums.contains(&5000)
            || open_port_nums.contains(&5001)
            || svc_str.contains("nas")
            || host_lower.contains("nas")
            || host_lower.contains("synology")
            || host_lower.contains("qnap")
            || host_lower.contains("truenas")
            || host_lower.contains("freenas")
            || host_lower.contains("unraid")
            || host_lower.contains("diskstation")
            || host_lower.contains("mycloud")
            || host_lower.contains("omv")
            || host_lower.contains("terramaster")
            || host_lower.contains("asustor")
            || model_lower.contains("diskstation")
            || model_lower.contains("nas")
            || vendor_lower.contains("synology")
            || vendor_lower.contains("qnap")
            || vendor_lower.contains("asustor")
            || vendor_lower.contains("drobo")
            || vendor_lower.contains("terramaster")
        {
            return DeviceCategory::StorageNas;
        }

        // 9. Network Bridges, Wi-Fi Extenders, Repeaters, Switches
        if host_lower.contains("extender")
            || host_lower.contains("repeater")
            || host_lower.contains("repetidor")
            || host_lower.contains("ap-")
            || host_lower.contains("accesspoint")
            || host_lower.contains("powerline")
            || host_lower.contains("satellite")
            || host_lower.contains("mesh-node")
            || host_lower.contains("unifi-ap")
            || (host_lower.contains("switch") && !host_lower.contains("nintendo"))
            || model_lower.contains("extender")
            || model_lower.contains("repeater")
            || model_lower.contains("access point")
            || vendor_lower.contains("devolo")
            || (vendor_lower.contains("tp-link") && (host_lower.contains("re") || host_lower.contains("extender") || host_lower.contains("range")))
            || (vendor_lower.contains("ubiquiti") && (host_lower.contains("ap") || host_lower.contains("switch")))
        {
            return DeviceCategory::NetworkBridge;
        }

        // 10. Routers / Gateways
        if open_port_nums.contains(&53)
            || host_lower.contains("router")
            || host_lower.contains("gateway")
            || host_lower.contains("linksys")
            || host_lower.contains("openwrt")
            || host_lower.contains("dd-wrt")
            || host_lower.contains("pfsense")
            || host_lower.contains("opnsense")
            || vendor_lower.contains("linksys")
            || vendor_lower.contains("cisco")
            || vendor_lower.contains("netgear")
            || vendor_lower.contains("mikrotik")
            || vendor_lower.contains("ubiquiti")
            || vendor_lower.contains("fritz!box")
            || vendor_lower.contains("d-link")
            || vendor_lower.contains("zyxel")
            || (vendor_lower.contains("tp-link") && !host_lower.contains("re") && !host_lower.contains("extender") && !host_lower.contains("plug") && !host_lower.contains("bulb"))
        {
            return DeviceCategory::Router;
        }

        // 11. Smartphones
        if host_lower.contains("phone")
            || host_lower.contains("iphone")
            || host_lower.contains("galaxy")
            || host_lower.contains("pixel")
            || host_lower.contains("android")
            || host_lower.contains("a25")
            || host_lower.contains("a54")
            || host_lower.contains("s22")
            || host_lower.contains("s23")
            || host_lower.contains("s24")
            || host_lower.contains("s25")
            || host_lower.contains("xiaomi")
            || host_lower.contains("redmi")
            || host_lower.contains("huawei")
            || host_lower.contains("motorola")
            || host_lower.contains("oneplus")
            || host_lower.contains("oppo")
            || host_lower.contains("vivo")
            || host_lower.contains("realme")
            || host_lower.contains("honor")
            || host_lower.contains("poco")
            || model_lower.contains("iphone")
            || model_lower.contains("phone")
            || model_lower.contains("galaxy")
            || model_lower.contains("pixel")
        {
            return DeviceCategory::Smartphone;
        }

        // 12. Smartwatches & Wearables
        if host_lower.contains("watch")
            || host_lower.contains("reloj")
            || host_lower.contains("band")
            || host_lower.contains("garmin")
            || host_lower.contains("fitbit")
            || host_lower.contains("apple-watch")
            || host_lower.contains("galaxy-watch")
            || host_lower.contains("wearos")
            || host_lower.contains("amazfit")
            || host_lower.contains("whoop")
            || model_lower.contains("watch")
            || model_lower.contains("band")
            || vendor_lower.contains("garmin")
            || vendor_lower.contains("fitbit")
            || vendor_lower.contains("polar electro")
            || vendor_lower.contains("suunto")
            || vendor_lower.contains("whoop")
            || vendor_lower.contains("fossil")
        {
            return DeviceCategory::Wearable;
        }

        // 13. Smart Home / IoT / Domótica
        if open_port_nums.contains(&1883)
            || svc_str.contains("hap")
            || svc_str.contains("homekit")
            || svc_str.contains("mqtt")
            || svc_str.contains("tasmota")
            || svc_str.contains("wled")
            || svc_str.contains("esphome")
            || vendor_lower.contains("tuya")
            || vendor_lower.contains("espressif")
            || vendor_lower.contains("sonoff")
            || vendor_lower.contains("itead")
            || vendor_lower.contains("shelly")
            || vendor_lower.contains("alterco")
            || vendor_lower.contains("philips lighting")
            || vendor_lower.contains("signify")
            || vendor_lower.contains("hue")
            || vendor_lower.contains("tasmota")
            || vendor_lower.contains("wled")
            || vendor_lower.contains("broadlink")
            || vendor_lower.contains("lutron")
            || vendor_lower.contains("legrand")
            || vendor_lower.contains("lifx")
            || vendor_lower.contains("yeelight")
            || vendor_lower.contains("aqara")
            || vendor_lower.contains("switchbot")
            || vendor_lower.contains("ecobee")
            || vendor_lower.contains("honeywell")
            || host_lower.contains("plug")
            || host_lower.contains("bulb")
            || host_lower.contains("sensor")
            || host_lower.contains("iot")
            || host_lower.contains("relay")
            || host_lower.contains("termostato")
            || host_lower.contains("thermostat")
            || host_lower.contains("led")
            || host_lower.contains("strip")
            || host_lower.contains("wled")
            || host_lower.contains("tasmota")
            || host_lower.contains("esphome")
            || host_lower.contains("smart-plug")
            || host_lower.contains("smart-bulb")
            || model_lower.contains("plug")
            || model_lower.contains("bulb")
            || model_lower.contains("sensor")
        {
            return DeviceCategory::Iot;
        }

        // 14. Dedicated Servers / Virtualization Hosts / Homelab
        if host_lower.contains("server")
            || host_lower.contains("srv")
            || host_lower.contains("proxmox")
            || host_lower.contains("docker")
            || host_lower.contains("k8s")
            || host_lower.contains("kubernetes")
            || host_lower.contains("pihole")
            || host_lower.contains("pi-hole")
            || host_lower.contains("raspberry")
            || host_lower.contains("rpi")
            || host_lower.contains("nextcloud")
            || host_lower.contains("homelab")
            || open_port_nums.contains(&9000)
            || model_lower.contains("server")
            || model_lower.contains("raspberry pi")
            || (vendor_lower.contains("raspberry pi") && !host_lower.contains("pc") && !host_lower.contains("desktop"))
        {
            return DeviceCategory::Server;
        }

        // 15. Personal Computers (Desktop, Laptop, PC, Mac)
        if is_current_host
            || open_port_nums.contains(&22)
            || open_port_nums.contains(&445)
            || open_port_nums.contains(&3389)
            || open_port_nums.contains(&5900)
            || host_lower.contains("pc")
            || host_lower.contains("laptop")
            || host_lower.contains("desktop")
            || host_lower.contains("cachyos")
            || host_lower.contains("arch")
            || host_lower.contains("ubuntu")
            || host_lower.contains("fedora")
            || host_lower.contains("debian")
            || host_lower.contains("windows")
            || host_lower.contains("macbook")
            || host_lower.contains("imac")
            || host_lower.contains("mac-mini")
            || host_lower.contains("mac-pro")
            || host_lower.contains("thinkpad")
            || host_lower.contains("dell")
            || host_lower.contains("lenovo")
            || host_lower.contains("asus")
            || host_lower.contains("acer")
            || host_lower.contains("msi")
            || host_lower.contains("hp-laptop")
            || host_lower.contains("hp-desktop")
            || model_lower.contains("macbook")
            || model_lower.contains("thinkpad")
            || model_lower.contains("laptop")
            || model_lower.contains("desktop")
        {
            return DeviceCategory::Computer;
        }

        DeviceCategory::Unknown
    }

    /// Performs a full asynchronous scan and emits real-time events
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

        // 1. Fast sweep with ICMP/TCP packets to populate kernel ARP cache
        let semaphore = Arc::new(Semaphore::new(45));
        let mut tasks = Vec::with_capacity(total_hosts);

        for host_num in 1..=254 {
            let target_ip = Ipv4Addr::new(octets[0], octets[1], octets[2], host_num);
            let sem_clone = Arc::clone(&semaphore);

            tasks.push(tokio::spawn(async move {
                let _permit = sem_clone.acquire().await;
                // Send single-packet ping with strict 500ms timeout
                let _ = Command::new("ping")
                    .args(["-c", "1", "-W", "1", &target_ip.to_string()])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    .await;
            }));
        }

        // Wait for basic connectivity probes to finish
        for (idx, task) in tasks.into_iter().enumerate() {
            let _ = task.await;
            if (idx + 1) % 15 == 0 || idx + 1 == total_hosts {
                let _ = sender.unbounded_send(ScanEvent::Progress {
                    scanned: idx + 1,
                    total: total_hosts,
                });
            }
        }

        // Brief delay allowing the kernel to commit ARP responses
        tokio::time::sleep(Duration::from_millis(250)).await;

        // 2. Read kernel ARP table (/proc/net/arp)
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

                    // Flags 0x2 means completed/reachable
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

        // Ensure gateway is present
        if let Some(gw_ip) = ctx.gateway_ip {
            if !seen_ips.contains(&gw_ip) {
                let mac = ctx.gateway_mac.clone().unwrap_or_else(|| "d8:ec:5e:9c:77:1b".to_string());
                discovered_ips.push((gw_ip, mac, ctx.interface.clone()));
                seen_ips.insert(gw_ip);
            }
        }

        // Ensure current host is present
        if let Some(local_ip) = ctx.local_ip {
            if !seen_ips.contains(&local_ip) {
                let mac = ctx.local_mac.clone().unwrap_or_else(|| "00:00:00:00:00:00".to_string());
                discovered_ips.push((local_ip, mac, ctx.interface.clone()));
                seen_ips.insert(local_ip);
            }
        }

        // Sort IPs in ascending order
        discovered_ips.sort_by_key(|&(ip, _, _)| ip);

        // 3. Enrich data for each discovered host (reverse DNS, ports, latency)
        let total_found = discovered_ips.len();
        let mut enrich_tasks = Vec::new();

        for (ip, mac, iface) in discovered_ips {
            let is_gateway = ctx.gateway_ip == Some(ip);
            let is_current = ctx.local_ip == Some(ip);
            let oui_ref = oui_db.clone();

            enrich_tasks.push(tokio::spawn(async move {
                // Latency
                let latency_ms = if is_current {
                    Some(0.05)
                } else {
                    Self::measure_ping_latency(ip).await
                };

                // Hostname via reverse DNS
                let hostname = tokio::task::spawn_blocking(move || {
                    let addr = IpAddr::V4(ip);
                    dns_lookup::lookup_addr(&addr).ok()
                })
                .await
                .ok()
                .flatten();

                // Advanced local identity discovery (mDNS, NetBIOS, SSDP)
                let discovery = crate::discovery::Discovery::probe(ip).await;
                let final_hostname = discovery.hostname.or(hostname);
                let model_name = discovery.model_or_friendly_name;
                let mut discovered_services = discovery.services;

                // Vendor OUI
                let is_randomized = Self::is_randomized_mac(&mac);
                let vendor = Self::lookup_vendor(oui_ref.as_deref(), &mac);

                // Common quick ports
                let open_ports = Self::quick_probe_ports(ip).await;
                for p in &open_ports {
                    if !discovered_services.iter().any(|s| s.contains(&p.service)) {
                        discovered_services.push(p.service.clone());
                    }
                }

                // Advanced Multi-Layer Categorization
                let category = Self::categorize_device_advanced(
                    is_gateway,
                    is_current,
                    final_hostname.as_deref(),
                    &vendor,
                    &open_ports,
                    &discovered_services,
                    model_name.as_deref(),
                );

                let now = Local::now();
                let mut initial_requests = Vec::new();

                // 1. Requests from discovered services (mDNS / SSDP)
                for svc in &discovered_services {
                    let (cat, proto) = if svc.contains("AirPlay") {
                        ("Streaming / Apple AirPlay", "mDNS")
                    } else if svc.contains("Cast") {
                        ("Streaming / Google Cast", "mDNS")
                    } else if svc.contains("Sonos") {
                        ("Sonos Controller / Grouping", "mDNS / SSDP")
                    } else if svc.contains("Samba") || svc.contains("SMB") {
                        ("Files / SMB", "NetBIOS / mDNS")
                    } else if svc.contains("Printer") || svc.contains("Impresora") || svc.contains("IPP") {
                        ("Print Service", "mDNS / IPP")
                    } else if svc.contains("Camera") || svc.contains("ONVIF") || svc.contains("RTSP") {
                        ("Camera Stream / ONVIF", "mDNS / RTSP")
                    } else {
                        ("Local Service", "mDNS / SSDP")
                    };
                    initial_requests.push(crate::model::DeviceRequestEntry {
                        timestamp: now,
                        query: format!("_{}._tcp.local", svc.to_lowercase().replace(' ', "")),
                        category: cat.to_string(),
                        protocol: proto.to_string(),
                    });
                }

                // 2. DNS query for resolved hostname
                if let Some(host) = &final_hostname {
                    initial_requests.push(crate::model::DeviceRequestEntry {
                        timestamp: now,
                        query: format!("{}.lan", host),
                        category: "Local DNS Resolution".to_string(),
                        protocol: "DNS PTR".to_string(),
                    });
                }

                // 3. Typical requests based on device type
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
                            category: "Connectivity Check".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                    }
                    DeviceCategory::Tablet => {
                        let domain = if vendor.to_lowercase().contains("apple") || final_hostname.as_deref().unwrap_or("").to_lowercase().contains("ipad") {
                            "mesu.apple.com"
                        } else {
                            "connectivitycheck.gstatic.com"
                        };
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: domain.to_string(),
                            category: "Tablet App & System Sync".to_string(),
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
                            category: "Streaming Device".to_string(),
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
                            category: "Online Gaming / Console".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                    }
                    DeviceCategory::SmartSpeaker => {
                        let query = if vendor.to_lowercase().contains("sonos") || final_hostname.as_deref().unwrap_or("").to_lowercase().contains("sonos") {
                            "msmetrics.ws.sonos.com"
                        } else {
                            "device-metrics-us.amazon.com"
                        };
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: query.to_string(),
                            category: "Audio Streaming & Voice Telemetry".to_string(),
                            protocol: "HTTPS / TLS".to_string(),
                        });
                    }
                    DeviceCategory::SecurityCamera => {
                        let query = if vendor.to_lowercase().contains("ring") {
                            "api.ring.com"
                        } else if vendor.to_lowercase().contains("wyze") {
                            "api.wyzecam.com"
                        } else {
                            "pool.ntp.org"
                        };
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: query.to_string(),
                            category: "Video Cloud / Telemetry".to_string(),
                            protocol: "DNS A / RTSP".to_string(),
                        });
                    }
                    DeviceCategory::StorageNas => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "checkip.synology.com".to_string(),
                            category: "NAS DDNS / Cloud Sync".to_string(),
                            protocol: "HTTPS".to_string(),
                        });
                    }
                    DeviceCategory::Server => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "registry-1.docker.io".to_string(),
                            category: "Container Registry / Updates".to_string(),
                            protocol: "HTTPS".to_string(),
                        });
                    }
                    DeviceCategory::Printer => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "epsonconnect.com".to_string(),
                            category: "Cloud Print / Telemetry".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                    }
                    DeviceCategory::Wearable => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "gateway.icloud.com".to_string(),
                            category: "Health & Cloud Sync".to_string(),
                            protocol: "HTTPS".to_string(),
                        });
                    }
                    DeviceCategory::NetworkBridge => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "tplinkwifi.net".to_string(),
                            category: "Extender / AP Management".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                    }
                    DeviceCategory::Iot => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "mqtt.eclipseprojects.io".to_string(),
                            category: "IoT Telemetry / MQTT".to_string(),
                            protocol: "TCP 1883".to_string(),
                        });
                    }
                    DeviceCategory::Computer => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "www.google.com".to_string(),
                            category: "Web Search / HTTP".to_string(),
                            protocol: "DNS A".to_string(),
                        });
                    }
                    DeviceCategory::Router => {
                        initial_requests.push(crate::model::DeviceRequestEntry {
                            timestamp: now,
                            query: "router.linksys.com".to_string(),
                            category: "Gateway / Admin".to_string(),
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

        // Collect devices and send in real time
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

    /// Measures RTT latency by executing an individual ping
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

    /// Fast probe on key ports for basic fingerprinting
    pub async fn quick_probe_ports(ip: Ipv4Addr) -> Vec<PortInfo> {
        let candidate_ports = [22, 53, 80, 443, 445, 554, 631, 1400, 1883, 3074, 5000, 8008, 8080, 9100];
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

    /// Deep port scan for a specific target device
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

    /// Executes a 4-packet ping test and extracts complete statistics
    pub async fn ping_test_detailed(ip: Ipv4Addr) -> Result<PingStats, String> {
        let output = Command::new("ping")
            .args(["-c", "4", "-W", "1", &ip.to_string()])
            .output()
            .await
            .map_err(|e| format!("Error executing ping: {}", e))?;

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

    /// Sends a Wake-on-LAN (WoL) magic packet to remotely wake a computer
    pub fn send_wake_on_lan(mac_str: &str) -> Result<(), String> {
        let clean = mac_str.replace([':', '-'], "");
        if clean.len() != 12 {
            return Err("Invalid MAC address (must be 12 hexadecimal characters)".to_string());
        }

        let mut mac_bytes = [0u8; 6];
        for i in 0..6 {
            mac_bytes[i] = u8::from_str_radix(&clean[i * 2..i * 2 + 2], 16)
                .map_err(|_| "Invalid hexadecimal character in MAC address")?;
        }

        // Construct magic packet: 6 bytes of 0xFF followed by 16 repetitions of MAC
        let mut packet = Vec::with_capacity(102);
        packet.extend_from_slice(&[0xFF; 6]);
        for _ in 0..16 {
            packet.extend_from_slice(&mac_bytes);
        }

        let socket = std::net::UdpSocket::bind("0.0.0.0:0")
            .map_err(|e| format!("Error binding UDP socket: {}", e))?;
        socket
            .set_broadcast(true)
            .map_err(|e| format!("Error enabling broadcast: {}", e))?;

        // Send to universal broadcast as well as standard ports 9 and 7
        socket
            .send_to(&packet, "255.255.255.255:9")
            .map_err(|e| format!("Error sending WoL packet: {}", e))?;
        let _ = socket.send_to(&packet, "255.255.255.255:7");

        Ok(())
    }
}
