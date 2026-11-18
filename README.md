# ⚡ NetLens

**NetLens** is a modern, fast, and elegant local network explorer and monitor for Linux, built with **Rust** and a native graphical interface powered by **Iced 0.14**. It enables you to discover, analyze, and manage all devices connected to your local network and router, providing rich, real-time insights with zero bloat.

![Rust](https://img.shields.io/badge/Language-Rust_1.88+-orange.svg)
![GUI](https://img.shields.io/badge/GUI-Iced_0.14-blue.svg)
![Platform](https://img.shields.io/badge/Platform-Linux_(Wayland_/_X11)-green.svg)
![License](https://img.shields.io/badge/License-MIT-purple.svg)

---

## ✨ Key Features

### 1. 📊 Real-Time Network Telemetry & Live Sparklines
- **Download Speed Monitor (⬇️):** Live measurement of system bandwidth consumption in KB/s or MB/s with 30-second rolling sparklines and session totals.
- **Upload Speed Monitor (⬆️):** Continuous transmission speed telemetry and session uploaded volume tracking.
- **Wi-Fi Signal & Link Quality (📶):** Displays signal strength in dBm and percentage (e.g., `100% (-39 dBm)`), SSID, channel, frequency band (`2.4 GHz` / `5 GHz`), and negotiated PHY bitrate (`1200 Mbps`), or Gigabit Ethernet link state via `ethtool`.
- **Latency & Connection Estimation:** Automatically classifies latency for every active node on the subnet (Excellent, Good, Moderate, Weak).

### 2. 🚀 High-Performance Internet Speed Test & Persistent History
- **Latency & RTT Probing (🏓 Ping):** Precision ICMP and HTTP latency measurement against Anycast Edge nodes (`1.1.1.1` / `8.8.8.8`).
- **Full-Bandwidth Multi-Stream Download (⬇️):** Concurrent multi-stream engine connected to local ISP and CDN nodes (Netflix Open Connect CDN & Cloudflare Anycast) bypassing TCP Slow Start bottlenecks to accurately test lines from 50 to 1000+ Mbps.
- **Sustained Upload Stream (⬆️):** Delivers a sustained data stream to measure real upstream throughput and link saturation.
- **Persistent Test History:** Automatically logs past benchmark runs with timestamps, ping, download, upload, and server details in an interactive review table with clearing options.

### 3. 🔍 Smart Multi-Vector Network Discovery
- **Asynchronous ARP & ICMP/TCP Probing:** Discovers entire `/24` or custom subnets in seconds using asynchronous concurrency (`tokio`).
- **IEEE OUI Manufacturer Identification:** Embedded database with thousands of vendor prefixes (Apple, Samsung, Intel, Belkin, Tuya Smart, Raspberry Pi, TP-Link, Xiaomi, Google, Sony, and more).
- **Privacy MAC / Randomization Detection:** Automatically identifies whether a device uses a **Locally Administered Address (LAA)** or randomized private MAC address (standard on modern Android, iOS, and Windows 11).
- **Reverse DNS & mDNS Resolution:** Resolves human-friendly hostnames broadcast to the router (e.g., `Linksys02323`, `Galaxy-A25`, `printer-office`, `cachyos-desktop`).
- **Automatic Heuristic Fingerprinting:** Classifies devices into 9 distinct categories with dedicated icons:
  - 🌐 **Router / Gateway**
  - 📱 **Smartphones & Tablets**
  - 💻 **Computers & Laptops**
  - 📺 **Smart TVs & Media Players**
  - 🔌 **IoT & Smart Home Devices**
  - 🖨️ **Network Printers**
  - 🖧 **Servers & NAS**
  - 🎮 **Gaming Consoles** (PlayStation, Xbox, Nintendo Switch, Steam Deck)
  - 📡 **Streaming Devices** (Roku, Chromecast, Fire TV, Apple TV)

### 4. 🛡️ Network Security Audit & Intruder Alerts
- **Rogue Device Intrusion Alerts:** Real-time banner alert and desktop notification whenever an unrecognized device joins the local network.
- **Comprehensive Health & Security Audit:** Evaluates router isolation, open dangerous ports (Telnet, FTP, RDP, unencrypted HTTP), vulnerable services, and unwhitelisted nodes with a clear security score (0–100%).
- **Interactive Action Alerts:** Instant notification prompts allowing users to directly **Trust (Whitelist)** or **Block** newly discovered hosts.

### 5. 🕒 Event History & Timeline
- **Real-Time Transition Logging:** Tracks connections, disconnections, IP transitions, and security events.
- **Filtering & Search:** Inspect past network events by date, MAC address, device name, or event severity.

### 6. 📶 Wi-Fi Channel & RF Spectrum Analyzer
- **2.4 GHz & 5 GHz Channel Inspection:** Analyzes channel occupancy and co-channel / adjacent-channel interference from surrounding wireless networks.
- **Smart Channel Recommendations:** Dynamically scores and recommends optimal non-overlapping channels (1, 6, 11 for 2.4 GHz; clean DFS/non-DFS blocks for 5 GHz).

### 7. 🌐 Universal Router UPnP / IGD & Port Forwarding
- **IGD Discovery:** Automatic discovery of Internet Gateway Devices (IGD) across any UPnP-compliant router.
- **Active Port Mapping List:** Visualizes all active WAN-to-LAN port mappings created by apps, games, and systems.
- **Dynamic Forwarding Management:** Add or delete custom port forwards directly from NetLens.

### 8. 🔒 Linksys Mesh Router Control & 24/7 Hardware Blocking
- **Direct JNAP API Integration:** Communicates natively with Linksys MR73 Mesh and compatible routers to control MAC filtering and parental controls.
- **Hardware-Level 24/7 Blocking:** Permanently severs network access at the router hardware level, persisting even when the monitoring PC is turned off.
- **One-Click Instant Readmission:** Restore connectivity with a single click.
- **Whitelist Management:** Tag verified devices as trusted nodes. Self-protection rules prevent accidental locking of your own PC or the gateway.

### 9. 🗄️ Router USB Storage / NAS Explorer
- **SMB Share Browser:** Discover and navigate disks and partitions plugged into the router's USB ports.
- **Recursive Directory & File Upload:** Upload entire directories recursively with live progress tracking, transfer speed, and remaining file counts.
- **File Management & Cache Protection:** Create directories, inspect metadata, download files, and perform deletions with optimistic UI updates and stale cache mitigation.
- **Interactive Demo Mode:** Explore storage features immediately even without an active router NAS configured.

### 10. 🔔 System Tray & Daemon Operation
- **Background Startup by Default:** NetLens can run silently in the background from boot.
- **Minimize to System Tray on Close:** Closing the window hides it directly to the system tray (`StatusNotifierItem`), keeping network monitors, netlink kernel listeners, and keep-alive heartbeats running seamlessly.
- **Desktop Notifications:** Customizable alerts for rogue devices, reconnected hosts, internet outages, and security risks with configurable sound and urgency levels.

### 11. 🌍 Internationalization (Top 10 World Languages)
- Fully localized in the 10 most spoken languages globally, with **English** as the default:
  - 🇬🇧 **English** (Default)
  - 🇪🇸 **Español**
  - 🇨🇳 **中文** (Mandarin Chinese)
  - 🇮🇳 **हिन्दी** (Hindi)
  - 🇸🇦 **العربية** (Arabic)
  - 🇫🇷 **Français** (French)
  - 🇧🇩 **বাংলা** (Bengali)
  - 🇧🇷 **Português** (Portuguese)
  - 🇷🇺 **Русский** (Russian)
  - 🇵🇰 **اردو** (Urdu)

---

## 🚀 Installation & Running

### Option 1: Arch Linux / CachyOS (Pacman Repository)
Add the official NetLens Pacman repository to `/etc/pacman.conf`:

```ini
[netlens]
SigLevel = Optional TrustAll
Server = https://omarmurcia.github.io/netlens-repo/x86_64
```

Then synchronize and install via Pacman or Shelly:
```bash
sudo pacman -Sy netlens
# or via shelly:
# shelly install netlens
```

### Option 2: Portable AppImage
Download `NetLens-x86_64.AppImage` from GitHub Releases:
```bash
chmod +x NetLens-x86_64.AppImage
./NetLens-x86_64.AppImage
```

### Option 3: Building from Source
Ensure Rust 1.88+ and system build tools are installed:
```bash
git clone https://github.com/omarmurcia/netlens.git
cd netlens
cargo build --release
./target/release/netlens
```

#### CLI Quick Scan Mode
To run an instant terminal network scan without launching the graphical interface:
```bash
./target/release/netlens --scan
```

---

## 🛠️ Project Architecture

```
netlens/
├── Cargo.toml               # Project manifest and dependencies (Iced 0.14, Tokio, etc.)
├── netlens.desktop          # Linux XDG desktop entry
├── README.md                # Comprehensive documentation
├── src/
│   ├── main.rs              # Application entry point, CLI argument routing, Tokio runtime
│   ├── lib.rs               # Library root and shared exports
│   ├── app.rs               # Main GUI state machine, message handlers, and reactive views
│   ├── model.rs             # Core data structures (Device, Category, NetworkContext, PortInfo)
│   ├── scanner.rs           # Asynchronous network scanner (ARP, Ping, reverse DNS, WoL)
│   ├── speedtest.rs         # High-speed internet benchmark engine
│   ├── telemetry.rs         # Bandwidth (/proc/net/dev) and Wi-Fi signal telemetry
│   ├── storage.rs           # Persistent state manager (~/.config/netlens/state.json)
│   ├── security_audit.rs    # Network vulnerability and health auditing engine
│   ├── history.rs           # Event timeline and state transition logger
│   ├── realtime_monitor.rs  # Kernel Netlink socket listener for instant link transitions
│   ├── notification.rs      # Desktop notification dispatcher and urgency controller
│   ├── tray.rs              # StatusNotifierItem system tray background service
│   ├── wifi_analyzer.rs     # Wi-Fi spectrum and channel congestion analyzer
│   ├── traceroute.rs        # Visual hop-by-hop route diagnostics
│   ├── uptime.rs            # Connectivity and internet stability monitor
│   ├── theme.rs             # Visual color palettes (TokyoNight, Nord, Dracula, etc.)
│   ├── i18n.rs              # Localization engine for the top 10 world languages
│   ├── export.rs            # JSON and CSV report generation
│   ├── autostart.rs         # XDG autostart service installer and manager
│   ├── oui.rs               # IEEE MAC OUI manufacturer database
│   └── router/
│       ├── mod.rs
│       ├── linksys.rs       # Linksys Mesh / JNAP API router integration
│       ├── upnp.rs          # Universal UPnP / IGD port forwarding engine
│       └── storage.rs       # Router USB storage SMB explorer and recursive uploader
├── tests/
│   └── integration_tests.rs # Automated test suite (unit and integration tests)
└── packaging/
    ├── archlinux/
    │   └── PKGBUILD         # Arch Linux packaging recipe
    └── build-pacman-repo.sh # Automated repository build script
```

---

## 🧪 Testing

Run the automated test suite:
```bash
cargo test
```

All 51 unit and integration tests validate:
1. Randomized MAC detection (LAA privacy bits).
2. OUI database lookup accuracy.
3. Heuristic device categorization across all categories.
4. JSON and CSV export consistency.
5. Wake-on-LAN magic packet formation.
6. Internet stability outage detection and recovery debouncing.
7. Heartbeat keep-alive disconnection timeouts.
8. Recursive directory collection and progress reporting for router NAS.
9. System tray icon byte extraction and desktop entry autostart safety.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
