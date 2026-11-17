# ⚡ NetLens

**NetLens** es una aplicación moderna, rápida y estética para Linux escrita en **Rust** con interfaz gráfica nativa en **Iced 0.14**. Su objetivo es descubrir y monitorizar todos los dispositivos conectados a tu router/red local, proporcionando información detallada, enriquecida y en tiempo real de cada uno.

![Rust](https://img.shields.io/badge/Language-Rust_1.100-orange.svg)
![GUI](https://img.shields.io/badge/GUI-Iced_0.14-blue.svg)
![Platform](https://img.shields.io/badge/Platform-Linux_(Wayland_/_X11)-green.svg)
![License](https://img.shields.io/badge/License-MIT-purple.svg)

---

## ✨ Características Principales

1. **Telemetría de Red y Velocidades en Tiempo Real (En la cabecera):**
   - **Monitor de Velocidad de Descarga (⬇️):** Muestra los KB/s o MB/s consumidos en tiempo real por el equipo y el total descargado en la sesión.
   - **Monitor de Velocidad de Subida (⬆️):** Muestra la velocidad de transmisión en tiempo real y el total subido en la sesión.
   - **Intensidad de Señal Wi-Fi (📶):** Muestra la fuerza de la señal en dBm y porcentaje (ej. `100% (-39 dBm)`), SSID (`omar_internet`), banda (`5 GHz`) y bitrate negociado (`1200 Mbps`), o el estado del cable Ethernet Gigabit.
   - **Estimación de Calidad de Conexión:** Evalúa la señal de cada dispositivo conectado en la red según su latencia (Excelente, Buena, Moderada, Débil).

2. **Test de Velocidad de Internet de Alto Rendimiento e Historial Persistente (🚀 Speed Test):**
   - **Medición de Latencia y RTT (🏓 Ping):** Mide con precisión la latencia de red ICMP hacia Anycast Edge (`1.1.1.1` / `8.8.8.8`).
   - **Medición de Descarga de Ancho de Banda Completo (⬇️ Download):** Arquitectura multi-stream paralela conectada con nodos locales de ISP (Netflix Open Connect CDN y Cloudflare Anycast) que elimina el estrangulamiento de TCP Slow Start y mide la velocidad real de líneas de banda ancha de 50 a 500+ Mbps.
   - **Medición de Subida Sostenida (⬆️ Upload):** Envía un flujo de datos sostenido (10 MB) para saturar el canal de upstream y medir con precisión la tasa real en Mbps y MB/s.
   - **Historial Persistente de Pruebas:** Almacena automáticamente los últimos tests de velocidad con fecha/hora, ping, bajada, subida y nodo CDN, presentados en una tabla interactiva con opción de limpieza.

3. **Descubrimiento Inteligente Multivectorial:**
   - **Sondeo ARP y Barrido ICMP/TCP:** Detecta equipos en la subred local (`/24`) en cuestión de segundos utilizando concurrencia asíncrona (`tokio`).
   - **Identificación de Fabricante (OUI IEEE/Wireshark):** Base de datos integrada con miles de fabricantes (Apple, Samsung, Intel, Belkin, Tuya Smart, Raspberry Pi, TP-Link, Xiaomi, etc.).
   - **Detección de Privacidad de MAC:** Identifica automáticamente si un dispositivo usa **dirección MAC aleatoria/privada** (LAA - *Locally Administered Address*, típico de Android, iOS y Windows 11).
   - **Resolución de Nombres DNS / mDNS:** Resuelve los hostnames amigables reportados al router (ej. `Linksys02323`, `Marvin-s-A25`, `Samsung`, `cachyos-x8664`).
   - **Fingerprinting y Clasificación Automática:** Clasifica cada dispositivo con su icono y categoría correspondiente:
     - 🌐 **Router / Puerta de Enlace**
     - 📱 **Smartphones y Tablets**
     - 💻 **Ordenadores y Laptops**
     - 📺 **Smart TVs y Reproductores Multimedia**
     - 🔌 **Dispositivos IoT y Domótica**
     - 🖨️ **Impresoras de Red**
     - 🖧 **Servidores y NAS**

4. **Panel de Configuración y Persistencia Total (Settings & Storage):**
   - **Persistencia Trans-sesión en Disco:** Guarda automáticamente el idioma, tema gráfico, dispositivos conocidos, apodos/alias personalizados y overrides de categorías en `~/.config/netlens/state.json`.
   - **Selector de Idiomas (Top 10 Mundial):** Interfaz completamente localizada en los 10 idiomas más hablados del planeta:
     - 🇪🇸 **Español** (Español)
     - 🇬🇧 **English** (Inglés)
     - 🇨🇳 **中文** (Chino Mandarín)
     - 🇮🇳 **हिन्दी** (Hindi)
     - 🇸🇦 **العربية** (Árabe Estándar)
     - 🇫🇷 **Français** (Francés)
     - 🇧🇩 **বাংলা** (Bengalí)
     - 🇧🇷 **Português** (Portugués)
     - 🇷🇺 **Русский** (Ruso)
     - 🇵🇰 **اردو** (Urdu)
   - **Selector de Temas:** Cambia al instante entre temas de alta estética (`TokyoNight`, `CatppuccinMocha`, `Nord`, `Dracula`, `Dark`, `Light`, `SolarizedDark`, `GruvboxDark`).

5. **Control de Router, Bloqueo Permanente (24/7) y Listas Blancas/Negras:**
   - **Integración Nativa con Router (Linksys MR73 / JNAP API):** Se comunica formalmente con la API de red del router para aplicar directivas de firewall y filtro MAC sin intermediarios.
   - **Bloqueo Permanente (24/7):** Corta el acceso a la red de dispositivos específicos a nivel de hardware. La regla queda grabada en el router y funciona las 24 horas del día aunque apagues el ordenador.
   - **Readmisión Inmediata con 1 Clic:** Desbloquea y restablece la conectividad al instante mediante el botón reactivo en el panel inspector.
   - **Gestión de Lista Blanca (Dispositivos de Confianza):** Etiqueta equipos autorizados para identificarlos rápidamente en el mapa de red.
   - **Protección de Seguridad:** Bloqueo preventivo que impide auto-bloquear por error la propia PC local o la puerta de enlace / router.
   - **Filtros Rápidos en Interfaz:** Pestañas para aislar dispositivos en `[Lista Blanca]` o `[Bloqueados]` con contadores en tiempo real.

6. **Clasificación y Override Persistente de Dispositivos:**
   - En el panel de detalles/inspector, los usuarios pueden seleccionar o corregir libremente la categoría del dispositivo (`Router`, `Smartphone`, `Computer`, `Smart TV`, `IoT`, `Printer`, `Server`, `Generic`).
   - Todos los cambios se asocian permanentemente a la dirección MAC del dispositivo y se restauran de forma instantánea en cada inicio de la aplicación.

6. **Panel Inspector Detallado:**
   - **Ficha Técnica Completa:** IP, MAC, Fabricante, Hostname DNS, Tipo de MAC, Interfaz de red y hora de última respuesta.
   - **Editor de Alias y Notas:** Asigna un apodo personalizado a cada dispositivo (ej. *"Móvil de Juan"*, *"Enchufe Lámpara Salón"*).
   - **Herramienta de Ping en Vivo:** Envía 4 paquetes ICMP y calcula paquetes transmitidos, recibidos, porcentaje de pérdida y latencia mínima, media y máxima.
   - **Escáner Profundo de Puertos:** Escanea más de 30 puertos estándar (SSH, HTTP, HTTPS, SMB, DNS, IPP, RTSP, MySQL, RDP, Cast, UPnP, Portainer, etc.) con descripción del servicio.
   - **Acceso Web Rápido:** Abre directamente el panel de administración o interfaz web en el navegador por defecto.
   - **Wake-on-LAN (WoL):** Envía paquetes mágicos de broadcast UDP para encender equipos remotamente.
   - **Botones de Copiado:** Copia al portapapeles la IP o la MAC con un solo clic.

7. **Exportación de Informes:**
   - **JSON estructurado:** Exporta el mapa completo de la red con marcas de tiempo y metadatos.
   - **CSV estándar:** Exporta una tabla delimitada por comas para abrir en LibreOffice Calc, Excel u otras herramientas.

8. **Doble Modo (GUI y CLI):**
   - Ejecución gráfica estándar o escaneo directo por terminal con la bandera `--scan` o `--cli`.

---

## 🚀 Inicio Rápido

### Requisitos en Linux
- Sistema operativo Linux (Wayland o X11).
- Rust y Cargo (1.88+ recomendado).
- Paquetes del sistema comunes: `iproute2` e `iputils` (disponibles por defecto en casi todas las distribuciones).

### Compilación y Ejecución

#### Modo Gráfico (GUI)
```bash
cd netlens
cargo run --release
```

O ejecuta directamente el binario optimizado:
```bash
./target/release/netlens
```

#### Modo Terminal (CLI)
Para realizar un escaneo rápido por consola sin levantar la interfaz gráfica:
```bash
./target/release/netlens --scan
```

### Acceso Directo de Escritorio
Se incluye el archivo `netlens.desktop` preconfigurado. Para que aparezca en tu lanzador de aplicaciones (GNOME, KDE Plasma, etc.):
```bash
cp netlens.desktop ~/.local/share/applications/
update-desktop-database ~/.local/share/applications
```

---

## 🛠️ Estructura del Proyecto

```
netlens/
├── Cargo.toml               # Dependencias (iced 0.14, tokio, mac_oui, chrono, serde)
├── netlens.desktop          # Lanzador para el menú de aplicaciones de Linux
├── README.md                # Documentación del proyecto
├── src/
│   ├── main.rs              # Punto de entrada (enrutamiento GUI / CLI y runtime Tokio)
│   ├── lib.rs               # Exportación de módulos para biblioteca y pruebas
│   ├── model.rs             # Modelos de datos (Device, DeviceCategory, NetworkContext, PortInfo)
│   ├── router/              # Controlador de router (Linksys MR73 Mesh / JNAP API)
│   │   ├── mod.rs
│   │   └── linksys.rs
│   ├── scanner.rs           # Motor de escaneo asíncrono, ARP, Ping, DNS inverso y WoL
│   ├── speedtest.rs         # Motor de test de velocidad de Internet (Ping, Descarga, Subida)
│   ├── storage.rs           # Capa de persistencia (XDG state.json, metadatos y configuración)
│   ├── telemetry.rs         # Telemetría de ancho de banda (/proc/net/dev) y señal Wi-Fi (iw)
│   ├── i18n.rs              # Sistema de internacionalización (Top 10 idiomas mundiales)
│   ├── export.rs            # Generador de informes en JSON y CSV
│   ├── theme.rs             # Paleta de colores, estilos visuales y tarjetas
│   └── app.rs               # Lógica de la GUI en Iced (State, Message, Update, View)
└── tests/
    └── integration_tests.rs # Pruebas unitarias y de integración automáticas
```

---

## 🧪 Pruebas Automatizadas

El proyecto cuenta con una suite completa de pruebas:
```bash
cargo test
```

Verifica:
1. Detección de direcciones MAC aleatorias/privadas (LAA).
2. Identificación de fabricantes OUI.
3. Clasificación heurística de dispositivos.
4. Generación y formato de archivos JSON y CSV.
5. Validación y construcción de paquetes mágicos Wake-on-LAN.

---

## 📄 Licencia

Este proyecto está bajo la licencia [MIT](LICENSE).
