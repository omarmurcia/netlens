//! Platform dispatch for system autostart management.
#[cfg(not(windows))]
#[path = "autostart_unix.rs"]
mod imp;

#[cfg(windows)]
#[path = "autostart_windows.rs"]
mod imp;

pub use imp::AutostartManager;
