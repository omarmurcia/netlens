use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Gestor de inicio automático en entornos de escritorio Linux (estándar XDG Autostart)
pub struct AutostartManager;

impl AutostartManager {
    /// Obtiene la ruta al archivo `.desktop` en el directorio de autostart del usuario (~/.config/autostart/netlens.desktop)
    pub fn autostart_file_path() -> Option<PathBuf> {
        let home = env::var("HOME").ok()?;
        Some(PathBuf::from(home).join(".config").join("autostart").join("netlens.desktop"))
    }

    /// Asegura que el icono oficial de 512x512 esté instalado en ~/.local/share/icons/...
    pub fn ensure_icon_installed() -> Option<PathBuf> {
        let home = env::var("HOME").ok()?;
        let icon_dir = PathBuf::from(&home).join(".local/share/icons/hicolor/512x512/apps");
        let icon_path = icon_dir.join("netlens.png");
        if !icon_path.exists() {
            let _ = fs::create_dir_all(&icon_dir);
            let _ = fs::write(&icon_path, include_bytes!("../assets/netlens.png"));
        }
        Some(icon_path)
    }

    /// Asegura la integración en el menú de aplicaciones del sistema (~/.local/share/applications/netlens.desktop)
    pub fn ensure_applications_desktop(exec_path: &Path) {
        if let Ok(home) = env::var("HOME") {
            let app_dir = PathBuf::from(&home).join(".local/share/applications");
            let app_desktop = app_dir.join("netlens.desktop");
            let _ = fs::create_dir_all(&app_dir);

            let icon_path_str = Self::ensure_icon_installed()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "netlens".to_string());

            let content = format!(
                "[Desktop Entry]\n\
                Type=Application\n\
                Name=NetLens\n\
                GenericName=Network Monitor\n\
                Comment=Local Network Device Explorer & Monitor\n\
                Exec=\"{}\"\n\
                Icon={}\n\
                Terminal=false\n\
                Categories=Network;Monitor;System;\n\
                StartupWMClass=netlens\n",
                exec_path.to_string_lossy(),
                icon_path_str
            );
            let _ = fs::write(app_desktop, content);
        }
    }

    /// Determina la ruta permanente y válida del ejecutable en el sistema.
    /// Resuelve el problema crítico de AppImage: evita usar rutas temporales efímeras de /tmp/.mount_XXXXXX
    /// que se destruyen al reiniciar la máquina.
    pub fn get_persistent_executable_path() -> Result<PathBuf, String> {
        // 1. Verificar variables de entorno del runtime de AppImage
        for var in &["APPIMAGE", "TARGET_APPIMAGE", "ARGV0"] {
            if let Ok(val) = env::var(var) {
                let trimmed = val.trim();
                if !trimmed.is_empty() {
                    let p = PathBuf::from(trimmed);
                    if p.exists() {
                        return Ok(fs::canonicalize(&p).unwrap_or(p));
                    }
                }
            }
        }

        // 2. Comprobar ruta del ejecutable actual
        let current_exe = env::current_exe()
            .map_err(|e| format!("No se pudo determinar la ruta del ejecutable: {}", e))?;

        let exe_str = current_exe.to_string_lossy();

        // 3. Si la ruta apunta a un punto de montaje temporal en /tmp (AppImage) pero no había variable
        if exe_str.starts_with("/tmp/.mount_") {
            if let Ok(home) = env::var("HOME") {
                let candidates = [
                    PathBuf::from(&home).join("Documents/homenet/netlens/dist/NetLens-x86_64.AppImage"),
                    PathBuf::from(&home).join(".local/bin/NetLens-x86_64.AppImage"),
                    PathBuf::from(&home).join("Applications/NetLens-x86_64.AppImage"),
                    PathBuf::from(&home).join("Desktop/NetLens-x86_64.AppImage"),
                ];
                for candidate in &candidates {
                    if candidate.exists() {
                        return Ok(fs::canonicalize(candidate).unwrap_or_else(|_| candidate.clone()));
                    }
                }
            }
            return Err("NetLens se está ejecutando desde un punto de montaje temporal y no se encontró la ruta del AppImage permanente.".to_string());
        }

        Ok(current_exe)
    }

    /// Comprueba si el inicio automático al encender el sistema está activo y es VÁLIDO
    pub fn is_enabled() -> bool {
        let Some(path) = Self::autostart_file_path() else {
            return false;
        };

        if !path.exists() {
            return false;
        }

        if let Ok(content) = fs::read_to_string(&path) {
            // Si apunta a una ruta efímera muerta de /tmp, NO es un autostart válido
            if content.contains("/tmp/.mount_") {
                return false;
            }
            // Si está explícitamente desactivado por el gestor de inicio
            if content.contains("X-GNOME-Autostart-enabled=false") {
                return false;
            }
            // Comprobar que contenga la directiva Exec
            content.contains("Exec=")
        } else {
            false
        }
    }

    /// Genera el contenido del archivo desktop para autostart apuntando al binario con el argumento `--minimized`
    pub fn generate_desktop_entry(exec_path: &str) -> String {
        let formatted_exec = if exec_path.contains(' ') && !exec_path.starts_with('"') {
            format!("\"{}\"", exec_path)
        } else {
            exec_path.to_string()
        };

        let icon_str = Self::ensure_icon_installed()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "netlens".to_string());

        format!(
            "[Desktop Entry]\n\
            Type=Application\n\
            Name=NetLens\n\
            GenericName=Network Monitor\n\
            Comment=Local Network Device Explorer & Monitor\n\
            Exec={} --minimized\n\
            Icon={}\n\
            Terminal=false\n\
            Categories=Network;Monitor;System;\n\
            X-GNOME-Autostart-enabled=true\n\
            X-KDE-autostart-after=panel\n\
            X-MATE-Autostart-enabled=true\n\
            StartupNotify=false\n",
            formatted_exec,
            icon_str
        )
    }

    /// Activa o desactiva el inicio automático con el sistema
    pub fn set_enabled(enable: bool) -> Result<(), String> {
        let path = Self::autostart_file_path()
            .ok_or_else(|| "No se pudo determinar el directorio HOME del usuario".to_string())?;

        if enable {
            let persistent_exe = Self::get_persistent_executable_path()?;
            let exe_str = persistent_exe.to_string_lossy();

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Error creando directorio de autostart {:?}: {}", parent, e))?;
            }

            // Asegurar que el icono e integración de escritorio general estén creados
            let _ = Self::ensure_icon_installed();
            Self::ensure_applications_desktop(&persistent_exe);

            let content = Self::generate_desktop_entry(&exe_str);
            fs::write(&path, content)
                .map_err(|e| format!("Error escribiendo archivo de autostart {:?}: {}", path, e))?;
        } else if path.exists() {
            fs::remove_file(&path)
                .map_err(|e| format!("Error eliminando archivo de autostart {:?}: {}", path, e))?;
        }

        Ok(())
    }

    /// Comprueba si el autostart está configurado como activo pero tiene una ruta rota o desactualizada,
    /// y lo repara automáticamente.
    pub fn sync_if_needed(autostart_enabled: bool) {
        if !autostart_enabled {
            return;
        }

        let needs_repair = match Self::autostart_file_path() {
            Some(path) => {
                if !path.exists() {
                    true
                } else if let Ok(content) = fs::read_to_string(&path) {
                    content.contains("/tmp/.mount_") || !Self::is_enabled()
                } else {
                    true
                }
            }
            None => false,
        };

        if needs_repair {
            if let Err(e) = Self::set_enabled(true) {
                eprintln!("Aviso: No se pudo auto-reparar la entrada de autostart: {}", e);
            }
        }
    }
}
