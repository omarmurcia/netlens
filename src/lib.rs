pub mod app;
pub mod autostart;
pub mod discovery;
pub mod export;
pub mod history;
pub mod i18n;
pub mod model;
pub mod notification;
pub mod oui;
pub mod realtime_monitor;
pub mod router;
pub mod scanner;
pub mod security_audit;
pub mod speedtest;
pub mod storage;
pub mod telemetry;
pub mod theme;
pub mod traceroute;
pub mod tray;
pub mod uptime;
pub mod widget;
pub mod wifi_analyzer;
pub mod updater;
pub mod single_instance;

/// Requests the system memory allocator (glibc) to release unused heap pages back to the kernel
#[cfg(target_os = "linux")]
pub fn trim_memory() {
    unsafe extern "C" {
        fn malloc_trim(pad: usize) -> i32;
    }
    unsafe {
        malloc_trim(0);
    }
}

#[cfg(not(target_os = "linux"))]
pub fn trim_memory() {}
