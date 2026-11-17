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

2. **Descubrimiento Inteligente Multivectorial:**
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

2. **Panel Inspector Detallado:**
   - **Ficha Técnica Completa:** IP, MAC, Fabricante, Hostname DNS, Tipo de MAC, Interfaz de red y hora de última respuesta.
   - **Editor de Alias y Notas:** Asigna un apodo personalizado a cada dispositivo (ej. *"Móvil de Juan"*, *"Enchufe Lámpara Salón"*).
   - **Herramienta de Ping en Vivo:** Envía 4 paquetes ICMP y calcula paquetes transmitidos, recibidos, porcentaje de pérdida y latencia mínima, media y máxima.
   - **Escáner Profundo de Puertos:** Escanea más de 30 puertos estándar (SSH, HTTP, HTTPS, SMB, DNS, IPP, RTSP, MySQL, RDP, Cast, UPnP, Portainer, etc.) con descripción del servicio.
   - **Acceso Web Rápido:** Abre directamente el panel de administración o interfaz web en el navegador por defecto.
   - **Wake-on-LAN (WoL):** Envía paquetes mágicos de broadcast UDP para encender equipos remotamente.
   - **Botones de Copiado:** Copia al portapapeles la IP o la MAC con un solo clic.

3. **Interfaz de Usuario Moderna (Iced 0.14):**
   - **Soporte Nativo Wayland y X11** acelerado por hardware (`wgpu`).
   - **Selector de Temas:** Cambia al instante entre temas oscuros de alta estética (`TokyoNight`, `CatppuccinMocha`, `Nord`, `Dracula`, `Dark`).
   - **Buscador en Tiempo Real:** Filtra simultáneamente por IP, nombre, fabricante, MAC o alias.
   - **Filtros por Categoría:** Botones rápidos para mostrar solo Routers, Móviles, PCs, TVs o Domótica.
   - **Criterios de Ordenación:** Ordena por IP, Latencia (Ping), Nombre, Fabricante o Categoría.
   - **Métricas Globales:** Tarjetas superiores con total de equipos, datos de la puerta de enlace, host actual y recuento por tipos.

4. **Exportación de Informes:**
   - **JSON estructurado:** Exporta el mapa completo de la red con marcas de tiempo y metadatos.
   - **CSV estándar:** Exporta una tabla delimitada por comas para abrir en LibreOffice Calc, Excel u otras herramientas.

5. **Doble Modo (GUI y CLI):**
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
│   ├── main.rs              # Punto de entrada (enrutamiento GUI / CLI)
│   ├── lib.rs               # Exportación de módulos para biblioteca y pruebas
│   ├── model.rs             # Modelos de datos (Device, DeviceCategory, NetworkContext, PortInfo)
│   ├── scanner.rs           # Motor de escaneo asíncrono, ARP, Ping, DNS inverso y WoL
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
