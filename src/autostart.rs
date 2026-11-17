use std::env;
use std::fs;
use std::path::PathBuf;

/// Gestor de inicio automático en entornos de escritorio Linux (estándar XDG Autostart)
pub struct AutostartManager;

impl AutostartManager {
    /// Obtiene la ruta al archivo `.desktop` en el directorio de autostart del usuario
    pub fn autostart_file_path() -> Option<PathBuf> {
        let home = env::var("HOME").ok()?;
        Some(PathBuf::from(home).join(".config").join("autostart").join("netlens.desktop"))
    }

    /// Comprueba si el inicio automático al encender el sistema está activado actualmente
    pub fn is_enabled() -> bool {
        if let Some(path) = Self::autostart_file_path() {
            path.exists()
        } else {
            false
        }
    }

    /// Genera el contenido del archivo desktop para autostart apuntando al binario con el argumento `--minimized`
    pub fn generate_desktop_entry(exec_path: &str) -> String {
        format!(
            "[Desktop Entry]\n\
            Type=Application\n\
            Name=NetLens\n\
            GenericName=Network Monitor\n\
            Comment=Local Network Device Explorer & Monitor\n\
            Exec={} --minimized\n\
            Icon=network-wireless\n\
            Terminal=false\n\
            Categories=Network;Monitor;\n\
            X-GNOME-Autostart-enabled=true\n\
            StartupNotify=false\n",
            exec_path
        )
    }

    /// Activa o desactiva el inicio automático con el sistema
    pub fn set_enabled(enable: bool) -> Result<(), String> {
        let path = Self::autostart_file_path()
            .ok_or_else(|| "No se pudo determinar el directorio HOME del usuario".to_string())?;

        if enable {
            let current_exe = env::current_exe()
                .map_err(|e| format!("No se pudo determinar la ruta del ejecutable: {}", e))?;
            let exe_str = current_exe.to_string_lossy();

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Error creando directorio de autostart {:?}: {}", parent, e))?;
            }

            let content = Self::generate_desktop_entry(&exe_str);
            fs::write(&path, content)
                .map_err(|e| format!("Error escribiendo archivo de autostart {:?}: {}", path, e))?;
        } else if path.exists() {
            fs::remove_file(&path)
                .map_err(|e| format!("Error eliminando archivo de autostart {:?}: {}", path, e))?;
        }

        Ok(())
    }
}
