#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use iced::futures::StreamExt;
use netlens::app::App;
use netlens::i18n;
use netlens::oui::Oui;
use netlens::scanner;
use netlens::single_instance;
use std::env;
use std::sync::Arc;

fn truncate_chars(s: &str, max_chars: usize) -> String {
    if s.chars().count() > max_chars {
        let head: String = s.chars().take(max_chars.saturating_sub(3)).collect();
        format!("{}...", head)
    } else {
        s.to_string()
    }
}

fn main() -> iced::Result {
    let args: Vec<String> = env::args().collect();

    // Winit 0.30 / Iced limitation on Wayland:
    // Winit does not implement WindowEvent::DroppedFile or HoveredFile on Wayland natively.
    // When XWayland is available (DISPLAY is set), running via the X11 backend enables full XDnD
    // drag-and-drop support from file managers like Dolphin, Nautilus, Thunar, etc.
    #[cfg(target_os = "linux")]
    {
        if env::var("NETLENS_NATIVE_WAYLAND").is_err() {
            if env::var("DISPLAY").map(|d| !d.is_empty()).unwrap_or(false)
                && env::var("WAYLAND_DISPLAY").map(|w| !w.is_empty()).unwrap_or(false)
            {
                unsafe {
                    env::remove_var("WAYLAND_DISPLAY");
                }
            }
        }
    }

    // Initialize optimized multi-threaded Tokio runtime (4 worker threads)
    // Drastically reduces memory usage by preventing glibc from creating 20+ arenas on high-core CPUs
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .thread_stack_size(1024 * 1024)
        .enable_all()
        .build()
        .expect("Failed to initialize Tokio runtime");
    let _guard = rt.enter();

    // Optional CLI mode: allows scanning and printing directly to terminal via --cli or --scan
    #[cfg(windows)]
    {
        if args.iter().any(|a| a == "--cli" || a == "--scan" || a == "-c") {
            // GUI-subsystem binaries have no console; attach to the parent terminal for CLI output
            unsafe {
                windows_sys::Win32::System::Console::AttachConsole(windows_sys::Win32::System::Console::ATTACH_PARENT_PROCESS);
            }
        }
    }

    if args.iter().any(|a| a == "--cli" || a == "--scan" || a == "-c") {
        rt.block_on(run_cli_scan());
        return Ok(());
    }

    // Single-instance enforcement:
    // If NetLens is already running, notify it to restore/focus its window and exit immediately
    let is_minimized = args.iter().any(|a| a == "--minimized" || a == "--background");
    let is_restart = args.iter().any(|a| a == "--from-restart");
    let _instance_guard = match rt.block_on(single_instance::acquire_single_instance(is_minimized, is_restart)) {
        single_instance::InstanceStatus::Primary(guard) => guard,
        single_instance::InstanceStatus::SecondaryNotified => {
            // Already running; notified primary instance to focus window. Exit cleanly.
            return Ok(());
        }
    };

    // GUI Daemon mode with Iced (runs in background with System Tray without remaining pinned to taskbar)
    iced::daemon(App::boot, App::update, App::view)
        .title(|app: &App, _win: iced::window::Id| i18n::Translations::window_title(app.language).to_string())
        .theme(|app: &App, _win: iced::window::Id| Some(app.current_theme.clone()))
        .subscription(App::subscription)
        .run()
}

async fn run_cli_scan() {
    println!("\n=======================================================");
    println!("⚡ NetLens CLI - Local Network Device Explorer");
    println!("=======================================================\n");

    let ctx = scanner::Scanner::detect_network_context();
    let gw_str = ctx
        .gateway_ip
        .map(|ip| ip.to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    let local_str = ctx
        .local_ip
        .map(|ip| ip.to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    println!("📡 Network Interface : {}", ctx.interface);
    println!("📍 Local IP (This host): {}", local_str);
    println!("🌐 Gateway IP        : {}", gw_str);
    println!("🔍 Subnet to Scan     : {}\n", ctx.subnet_cidr);

    let oui_db = Oui::default().ok().map(Arc::new);
    let (tx, mut rx) = iced::futures::channel::mpsc::unbounded();

    tokio::spawn(async move {
        scanner::Scanner::execute_scan(ctx, oui_db, tx).await;
    });

    println!("{:<16} {:<18} {:<24} {:<22} {:<10}", "IP ADDRESS", "MAC ADDRESS", "NAME / HOSTNAME", "MANUFACTURER", "LATENCY");
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
                let name_truncated = truncate_chars(&name, 23);
                let vendor_truncated = truncate_chars(&dev.vendor, 21);

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
                    "\n✅ Scan completed successfully: {} devices detected in {:.2} s.\n",
                    total_found,
                    duration_ms as f32 / 1000.0
                );
            }
            scanner::ScanEvent::Error(err) => {
                eprintln!("\n❌ Error during scan: {}\n", err);
            }
        }
    }
}
