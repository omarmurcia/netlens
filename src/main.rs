mod app;
mod export;
mod i18n;
mod model;
mod router;
mod scanner;
mod speedtest;
mod storage;
mod telemetry;
mod theme;

use app::App;
use iced::futures::StreamExt;
use iced::Size;
use mac_oui::Oui;
use std::env;
use std::sync::Arc;

fn main() -> iced::Result {
    let args: Vec<String> = env::args().collect();

    // Inicializar el runtime de Tokio multihilo con soporte completo de reactor de red y temporizadores
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Fallo al inicializar runtime de Tokio");
    let _guard = rt.enter();

    // Modo CLI opcional: permite escanear e imprimir en terminal si se pasa --cli o --scan
    if args.iter().any(|a| a == "--cli" || a == "--scan" || a == "-c") {
        rt.block_on(run_cli_scan());
        return Ok(());
    }

    // Modo GUI con Iced
    iced::application(App::new, App::update, App::view)
        .title(|app: &App| i18n::Translations::window_title(app.language).to_string())
        .theme(|app: &App| app.current_theme.clone())
        .subscription(App::subscription)
        .window_size(Size::new(1220.0, 820.0))
        .centered()
        .run()
}

async fn run_cli_scan() {
    println!("\n=======================================================");
    println!("⚡ NetLens CLI - Explorador de Dispositivos de Red Local");
    println!("=======================================================\n");

    let ctx = scanner::Scanner::detect_network_context();
    let gw_str = ctx
        .gateway_ip
        .map(|ip| ip.to_string())
        .unwrap_or_else(|| "Desconocido".to_string());
    let local_str = ctx
        .local_ip
        .map(|ip| ip.to_string())
        .unwrap_or_else(|| "Desconocido".to_string());

    println!("📡 Interfaz de Red    : {}", ctx.interface);
    println!("📍 IP Local (Este host): {}", local_str);
    println!("🌐 Puerta de Enlace   : {}", gw_str);
    println!("🔍 Subred a Escanear  : {}\n", ctx.subnet_cidr);

    let oui_db = Oui::default().ok().map(Arc::new);
    let (tx, mut rx) = iced::futures::channel::mpsc::unbounded();

    tokio::spawn(async move {
        scanner::Scanner::execute_scan(ctx, oui_db, tx).await;
    });

    println!("{:<16} {:<18} {:<24} {:<22} {:<10}", "DIRECCIÓN IP", "DIRECCIÓN MAC", "NOMBRE / HOSTNAME", "FABRICANTE", "LATENCIA");
    println!("{:-<16} {:-<18} {:-<24} {:-<22} {:-<10}", "", "", "", "", "");

    while let Some(event) = rx.next().await {
        match event {
            scanner::ScanEvent::Started { .. } => {}
            scanner::ScanEvent::Progress { .. } => {}
            scanner::ScanEvent::HostFound(dev) => {
                let lat_str = dev
                    .latency_ms
                    .map(|l| format!("{:.1} ms", l))
                    .unwrap_or_else(|| "-".to_string());
                let name = dev.display_name();
                let name_truncated = if name.len() > 23 {
                    format!("{}...", &name[..20])
                } else {
                    name
                };
                let vendor_truncated = if dev.vendor.len() > 21 {
                    format!("{}...", &dev.vendor[..18])
                } else {
                    dev.vendor.clone()
                };

                println!(
                    "{:<16} {:<18} {:<24} {:<22} {:<10}",
                    dev.ip, dev.mac, name_truncated, vendor_truncated, lat_str
                );
            }
            scanner::ScanEvent::Completed {
                total_found,
                duration_ms,
            } => {
                println!(
                    "\n✅ Escaneo completado con éxito: {} dispositivos detectados en {:.2} s.\n",
                    total_found,
                    duration_ms as f32 / 1000.0
                );
            }
            scanner::ScanEvent::Error(err) => {
                eprintln!("\n❌ Error durante el escaneo: {}\n", err);
            }
        }
    }
}
