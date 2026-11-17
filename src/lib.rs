pub mod app;
pub mod autostart;
pub mod export;
pub mod i18n;
pub mod model;
pub mod oui;
pub mod router;
pub mod scanner;
pub mod speedtest;
pub mod storage;
pub mod telemetry;
pub mod theme;
pub mod traceroute;
pub mod tray;
pub mod widget;

/// Solicita al asignador de memoria del sistema (glibc) liberar páginas no utilizadas de vuelta al kernel
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
