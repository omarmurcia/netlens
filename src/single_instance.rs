use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub enum InstanceStatus {
    Primary(SingleInstanceGuard),
    SecondaryNotified,
}

#[derive(Debug)]
pub struct SingleInstanceGuard {
    pub socket_path: PathBuf,
    pub abort_handle: Option<tokio::task::AbortHandle>,
}

impl Drop for SingleInstanceGuard {
    fn drop(&mut self) {
        if let Some(handle) = &self.abort_handle {
            handle.abort();
        }
        let _ = std::fs::remove_file(&self.socket_path);
    }
}

/// Computes the user-isolated socket path using $XDG_RUNTIME_DIR or /tmp fallback
#[cfg(unix)]
pub fn get_socket_path() -> PathBuf {
    if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
        let trimmed = runtime_dir.trim();
        if !trimmed.is_empty() {
            let p = PathBuf::from(trimmed);
            if p.is_dir() {
                return p.join("netlens.sock");
            }
        }
    }
    let uid = unsafe { libc::getuid() };
    std::env::temp_dir().join(format!("netlens-{}.sock", uid))
}

/// Windows: loopback TCP port used as the single-instance lock + IPC channel.
#[cfg(windows)]
const SINGLE_INSTANCE_ADDR: &str = "127.0.0.1:47613";

#[cfg(windows)]
pub fn get_socket_path() -> PathBuf {
    // Not a real socket on Windows (TCP loopback is used); kept for API compatibility.
    std::env::temp_dir().join("netlens.lock")
}

/// Acquires the single instance lock for NetLens.
/// If an instance is already running, it notifies it via IPC to bring its window to focus,
/// and returns `InstanceStatus::SecondaryNotified`.
/// If no instance is running, it binds the Unix Domain Socket and returns `InstanceStatus::Primary(guard)`.
#[cfg(unix)]
pub async fn acquire_single_instance(is_minimized: bool, is_restart: bool) -> InstanceStatus {
    let sock_path = get_socket_path();

    if is_restart {
        // When restarting to apply an update, wait a brief moment for the parent process to exit
        for _ in 0..10 {
            if !sock_path.exists() {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
        let _ = std::fs::remove_file(&sock_path);
    }

    // Try to connect to an existing running instance
    if let Ok(mut stream) = tokio::net::UnixStream::connect(&sock_path).await {
        if !is_minimized {
            let _ = stream.write_all(b"RESTORE\n").await;
        }
        return InstanceStatus::SecondaryNotified;
    }

    // If connection failed, remove any stale socket file left from an abnormal exit or crash
    let _ = std::fs::remove_file(&sock_path);

    // Bind to the socket as the primary instance
    match tokio::net::UnixListener::bind(&sock_path) {
        Ok(listener) => {
            let join_handle = tokio::spawn(async move {
                while let Ok((mut stream, _)) = listener.accept().await {
                    let mut buf = [0u8; 64];
                    if let Ok(n) = stream.read(&mut buf).await {
                        let msg = std::str::from_utf8(&buf[..n]).unwrap_or("");
                        if msg.contains("RESTORE") {
                            crate::tray::send_tray_action(crate::tray::TrayAction::RestoreWindow);
                        }
                    }
                }
            });

            InstanceStatus::Primary(SingleInstanceGuard {
                socket_path: sock_path,
                abort_handle: Some(join_handle.abort_handle()),
            })
        }
        Err(e) => {
            eprintln!("Warning: Failed to bind single instance socket ({:?}): {}", sock_path, e);
            // If binding failed because another instance bound just now, try connecting one more time
            if let Ok(mut stream) = tokio::net::UnixStream::connect(&sock_path).await {
                if !is_minimized {
                    let _ = stream.write_all(b"RESTORE\n").await;
                }
                return InstanceStatus::SecondaryNotified;
            }
            // Otherwise, allow running to avoid preventing the user from using the app
            InstanceStatus::Primary(SingleInstanceGuard {
                socket_path: sock_path,
                abort_handle: None,
            })
        }
    }
}


/// Windows implementation: the first instance binds 127.0.0.1:47613; later instances connect and send "RESTORE".
#[cfg(windows)]
pub async fn acquire_single_instance(is_minimized: bool, is_restart: bool) -> InstanceStatus {
    use tokio::net::{TcpListener, TcpStream};

    let sock_path = get_socket_path();

    let notify_existing = |minimized: bool| async move {
        if let Ok(mut stream) = TcpStream::connect(SINGLE_INSTANCE_ADDR).await {
            if !minimized {
                let _ = stream.write_all(b"RESTORE\n").await;
            }
            true
        } else {
            false
        }
    };

    if is_restart {
        // Give the parent process a brief moment to exit and release the port
        for _ in 0..20 {
            if TcpStream::connect(SINGLE_INSTANCE_ADDR).await.is_err() {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    if notify_existing(is_minimized).await {
        return InstanceStatus::SecondaryNotified;
    }

    match TcpListener::bind(SINGLE_INSTANCE_ADDR).await {
        Ok(listener) => {
            let join_handle = tokio::spawn(async move {
                while let Ok((mut stream, _)) = listener.accept().await {
                    let mut buf = [0u8; 64];
                    if let Ok(n) = stream.read(&mut buf).await {
                        let msg = std::str::from_utf8(&buf[..n]).unwrap_or("");
                        if msg.contains("RESTORE") {
                            crate::tray::send_tray_action(crate::tray::TrayAction::RestoreWindow);
                        }
                    }
                }
            });
            InstanceStatus::Primary(SingleInstanceGuard {
                socket_path: sock_path,
                abort_handle: Some(join_handle.abort_handle()),
            })
        }
        Err(e) => {
            eprintln!("Warning: Failed to bind single instance port ({}): {}", SINGLE_INSTANCE_ADDR, e);
            if notify_existing(is_minimized).await {
                return InstanceStatus::SecondaryNotified;
            }
            InstanceStatus::Primary(SingleInstanceGuard {
                socket_path: sock_path,
                abort_handle: None,
            })
        }
    }
}