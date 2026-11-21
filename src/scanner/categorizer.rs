use super::Scanner;
use crate::model::{DeviceCategory, PortInfo};

impl Scanner {
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
        let has_port = |port: u16| open_ports.iter().any(|p| p.port == port);
        let svc_str = if discovered_services.is_empty() {
            String::new()
        } else {
            discovered_services.join(" ").to_lowercase()
        };

        // 1. Security Cameras / Surveillance / NVR
        if has_port(554)
            || has_port(8554)
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
        if has_port(1400)
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
        if has_port(631)
            || has_port(9100)
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
            || has_port(3074)
            || has_port(9295)
            || has_port(9296)
            || has_port(9297)
            || has_port(9303)
            || has_port(9304)
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
            || has_port(8008)
            || has_port(8009)
            || has_port(7000)
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
        if has_port(5000)
            || has_port(5001)
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
        if has_port(53)
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
        if has_port(1883)
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
            || has_port(9000)
            || model_lower.contains("server")
            || model_lower.contains("raspberry pi")
            || (vendor_lower.contains("raspberry pi") && !host_lower.contains("pc") && !host_lower.contains("desktop"))
        {
            return DeviceCategory::Server;
        }

        // 15. Personal Computers (Desktop, Laptop, PC, Mac)
        if is_current_host
            || has_port(22)
            || has_port(445)
            || has_port(3389)
            || has_port(5900)
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

}
