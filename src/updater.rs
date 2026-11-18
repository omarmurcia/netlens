use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteVersionInfo {
    pub version: String,
    pub pkgrel: u32,
    pub changelog: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateStatus {
    Idle,
    Checking,
    UpToDate,
    UpdateAvailable {
        remote_version: String,
        remote_pkgrel: u32,
        url: String,
        changelog: Option<String>,
    },
    CheckFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InAppUpdateProgress {
    Idle,
    AuthorizingAndInstalling,
    Success,
    Failed(String),
}

pub struct UpdateChecker;

impl UpdateChecker {
    pub const REMOTE_VERSION_URL: &'static str =
        "https://omarmurcia.github.io/netlens-repo/version.json";
    pub const CURRENT_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const CURRENT_PKGREL: u32 = 1;

    /// Asynchronously queries GitHub Pages for the latest published version
    pub async fn check_for_updates() -> UpdateStatus {
        let out = match Command::new("curl")
            .arg("-sSL")
            .arg("--max-time")
            .arg("4")
            .arg(Self::REMOTE_VERSION_URL)
            .output()
            .await
        {
            Ok(o) if o.status.success() => o.stdout,
            _ => return UpdateStatus::CheckFailed,
        };

        let info: RemoteVersionInfo = match serde_json::from_slice(&out) {
            Ok(v) => v,
            Err(_) => return UpdateStatus::CheckFailed,
        };

        if Self::is_newer(&info.version, info.pkgrel, Self::CURRENT_VERSION, Self::CURRENT_PKGREL) {
            UpdateStatus::UpdateAvailable {
                remote_version: info.version,
                remote_pkgrel: info.pkgrel,
                url: info.url.unwrap_or_else(|| "https://omarmurcia.github.io/netlens-repo".to_string()),
                changelog: info.changelog,
            }
        } else {
            UpdateStatus::UpToDate
        }
    }

    /// Compares (version, pkgrel) semver style
    pub fn is_newer(remote_ver: &str, remote_rel: u32, local_ver: &str, local_rel: u32) -> bool {
        let parse_semver = |v: &str| -> Vec<u32> {
            v.split('.')
                .filter_map(|s| s.trim_start_matches('v').parse::<u32>().ok())
                .collect()
        };

        let remote_parts = parse_semver(remote_ver);
        let local_parts = parse_semver(local_ver);

        for (r, l) in remote_parts.iter().zip(local_parts.iter()) {
            if r > l {
                return true;
            }
            if r < l {
                return false;
            }
        }

        if remote_parts.len() > local_parts.len() {
            return true;
        }
        if remote_parts.len() < local_parts.len() {
            return false;
        }

        remote_rel > local_rel
    }

    /// Automatically applies the update in-app (via pkexec for pacman or in-place download for AppImage)
    pub async fn apply_update_in_app() -> Result<String, String> {
        // 1. If running as AppImage, download the latest AppImage and atomically replace it
        if let Ok(appimage_path) = std::env::var("APPIMAGE") {
            let p = std::path::PathBuf::from(appimage_path.trim());
            if p.exists() {
                return Self::update_appimage(&p).await;
            }
        }

        // 2. Otherwise, update system package via pkexec / pacman
        Self::update_pacman_package().await
    }

    async fn update_pacman_package() -> Result<String, String> {
        if !std::path::Path::new("/usr/bin/pkexec").exists() {
            return Err("pkexec no está instalado en el sistema para elevar permisos.".to_string());
        }

        let cmd = "rm -f /var/cache/pacman/pkg/netlens* && pacman -Syy --noconfirm netlens";
        let output = Command::new("pkexec")
            .arg("bash")
            .arg("-c")
            .arg(cmd)
            .output()
            .await
            .map_err(|e| format!("No se pudo iniciar el proceso de autorización: {}", e))?;

        if output.status.success() {
            Ok("NetLens se actualizó exitosamente a través de pacman.".to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            let exit_code = output.status.code().unwrap_or(-1);
            if exit_code == 126 || exit_code == 127 {
                Err("La autorización gráfica fue cancelada o rechazada por el usuario.".to_string())
            } else if !stderr.is_empty() {
                Err(stderr)
            } else {
                Err(format!("Error en el gestor de paquetes pacman (código {})", exit_code))
            }
        }
    }

    async fn update_appimage(target_path: &std::path::Path) -> Result<String, String> {
        let download_url = "https://omarmurcia.github.io/netlens-repo/NetLens-x86_64.AppImage";
        let temp_download = target_path.with_extension("tmp_download");

        let status = Command::new("curl")
            .arg("-sSL")
            .arg("--max-time")
            .arg("120")
            .arg(download_url)
            .arg("-o")
            .arg(&temp_download)
            .status()
            .await
            .map_err(|e| format!("Error descargando AppImage: {}", e))?;

        if !status.success() {
            let _ = std::fs::remove_file(&temp_download);
            return Err("Falló la descarga del archivo AppImage.".to_string());
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(meta) = std::fs::metadata(&temp_download) {
                let mut perms = meta.permissions();
                perms.set_mode(0o755);
                let _ = std::fs::set_permissions(&temp_download, perms);
            }
        }

        std::fs::rename(&temp_download, target_path)
            .map_err(|e| format!("Error reemplazando archivo AppImage: {}", e))?;

        Ok("AppImage actualizada correctamente.".to_string())
    }

    /// Launches a visible terminal emulator running the interactive install script
    pub fn launch_terminal_updater() -> Result<(), String> {
        let script = "curl -sSL https://omarmurcia.github.io/netlens-repo/install-repo.sh | bash; echo ''; read -p 'Presione Enter para salir...'";
        let terminals: &[(&str, &[&str])] = &[
            ("konsole", &["-e", "bash", "-c", script]),
            ("alacritty", &["-e", "bash", "-c", script]),
            ("gnome-terminal", &["--", "bash", "-c", script]),
            ("xfce4-terminal", &["-e", "bash", "-c", script]),
            ("kitty", &["bash", "-c", script]),
            ("xterm", &["-e", "bash", "-c", script]),
        ];
        for (term, args) in terminals {
            if std::path::Path::new(&format!("/usr/bin/{}", term)).exists() {
                if std::process::Command::new(term).args(*args).spawn().is_ok() {
                    return Ok(());
                }
            }
        }
        Err("No se encontró ningún emulador de terminal compatible instalado.".to_string())
    }

    /// Spawns the newly installed binary detached and terminates the current instance cleanly
    pub fn restart_application() {
        // Determine the canonical binary path avoiding "(deleted)" suffix after pacman update
        let exe_path = if let Ok(appimage) = std::env::var("APPIMAGE") {
            let p = std::path::PathBuf::from(appimage.trim());
            if p.exists() {
                p
            } else {
                std::path::PathBuf::from("/usr/bin/netlens")
            }
        } else if std::path::Path::new("/usr/bin/netlens").exists() {
            std::path::PathBuf::from("/usr/bin/netlens")
        } else {
            let raw = std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("netlens"));
            let clean = raw.to_string_lossy().replace(" (deleted)", "");
            std::path::PathBuf::from(clean)
        };

        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            let mut cmd = std::process::Command::new(&exe_path);
            cmd.stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null());

            unsafe {
                cmd.pre_exec(|| {
                    libc::setsid();
                    Ok(())
                });
            }

            if let Err(e) = cmd.spawn() {
                eprintln!("Error al reiniciar NetLens ({:?}): {}", exe_path, e);
                // Fallback attempt with gtk-launch
                let _ = std::process::Command::new("gtk-launch")
                    .arg("netlens")
                    .spawn();
            }
        }

        #[cfg(not(unix))]
        {
            let _ = std::process::Command::new(&exe_path).spawn();
        }

        std::thread::sleep(std::time::Duration::from_millis(150));
        std::process::exit(0);
    }
}
