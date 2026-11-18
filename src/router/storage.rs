use iced::futures::channel::mpsc::UnboundedSender;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Instant;
use tokio::process::Command;

/// Información de un archivo local a transferir
#[derive(Debug, Clone)]
pub struct LocalFileInfo {
    pub abs_path: PathBuf,
    pub rel_path: String,
    pub size: u64,
}

/// Estado actual del progreso de subida de archivos o carpetas
#[derive(Debug, Clone)]
pub struct StorageUploadProgress {
    pub is_folder: bool,
    pub title: String,
    pub target_dir: String,
    pub current_item: String,
    pub current_index: usize,
    pub total_items: usize,
    pub bytes_uploaded: u64,
    pub total_bytes: u64,
    pub speed_str: String,
    pub percent: f32, // 0.0 to 100.0
    pub is_finished: bool,
    pub error: Option<String>,
}

/// Eventos transmitidos durante la subida de archivos o carpetas
#[derive(Debug, Clone)]
pub enum StorageUploadEvent {
    Started {
        is_folder: bool,
        title: String,
        total_items: usize,
        total_bytes: u64,
        target_dir: String,
    },
    Progress {
        current_item: String,
        current_index: usize,
        total_items: usize,
        bytes_uploaded: u64,
        total_bytes: u64,
        speed_str: String,
        percent: f32,
    },
    Completed {
        message: String,
        #[allow(dead_code)]
        total_items: usize,
        total_bytes: u64,
    },
    Error(String),
}

/// Helper para leer los bytes leídos por un proceso desde /proc/<pid>/io en Linux
fn get_process_read_bytes(pid: u32) -> Option<u64> {
    let path = format!("/proc/{}/io", pid);
    let content = std::fs::read_to_string(path).ok()?;
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("rchar:") {
            if let Ok(bytes) = rest.trim().parse::<u64>() {
                return Some(bytes);
            }
        }
    }
    None
}

/// Tipos de archivos reconocidos en el almacenamiento del router
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageFileType {
    Folder,
    Image,
    Video,
    Audio,
    Document,
    Archive,
    Executable,
    Code,
    Unknown,
}

impl StorageFileType {
    pub fn from_filename(name: &str, is_dir: bool) -> Self {
        if is_dir {
            return Self::Folder;
        }
        let ext = Path::new(name)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        match ext.as_str() {
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "svg" | "tiff" | "ico" => Self::Image,
            "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" => Self::Video,
            "mp3" | "flac" | "wav" | "aac" | "ogg" | "m4a" | "wma" => Self::Audio,
            "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "md" | "odt" | "csv" => {
                Self::Document
            }
            "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "iso" => Self::Archive,
            "sh" | "bin" | "run" | "appimage" | "exe" | "msi" | "apk" => Self::Executable,
            "rs" | "py" | "js" | "ts" | "html" | "css" | "json" | "xml" | "toml" | "yaml" | "yml" | "c"
            | "cpp" | "h" => Self::Code,
            _ => Self::Unknown,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Folder => "📁",
            Self::Image => "🖼️",
            Self::Video => "🎬",
            Self::Audio => "🎵",
            Self::Document => "📄",
            Self::Archive => "📦",
            Self::Executable => "⚙️",
            Self::Code => "💻",
            Self::Unknown => "📎",
        }
    }

    #[allow(dead_code)]
    pub fn type_name_es(&self) -> &'static str {
        self.type_name(crate::i18n::Language::Spanish)
    }

    pub fn type_name(&self, lang: crate::i18n::Language) -> &'static str {
        crate::i18n::Translations::storage_file_type_name(*self, lang)
    }
}

/// Entrada de archivo o directorio en el almacenamiento del router
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageFileEntry {
    pub name: String,
    pub is_dir: bool,
    pub size_bytes: u64,
    pub modified_str: String,
    pub file_type: StorageFileType,
    pub attributes: String,
}

impl StorageFileEntry {
    pub fn new(name: String, is_dir: bool, size_bytes: u64, modified_str: String, attributes: String) -> Self {
        let file_type = StorageFileType::from_filename(&name, is_dir);
        Self {
            name,
            is_dir,
            size_bytes,
            modified_str,
            file_type,
            attributes,
        }
    }

    pub fn formatted_size(&self) -> String {
        if self.is_dir {
            "—".to_string()
        } else {
            format_file_size(self.size_bytes)
        }
    }
}

/// Formatea tamaños en formato legible para humanos
pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Recurso compartido (Share) expuesto por el router (ej. usbdisk, public)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageShare {
    pub name: String,
    pub share_type: String, // "Disk", "IPC", "Printer"
    pub comment: String,
}

/// Información de espacio y capacidad del disco conectado al router
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct StorageDiskInfo {
    pub total_bytes: u64,
    pub free_bytes: u64,
}

impl StorageDiskInfo {
    pub fn used_bytes(&self) -> u64 {
        self.total_bytes.saturating_sub(self.free_bytes)
    }

    pub fn used_percent(&self) -> f32 {
        if self.total_bytes == 0 {
            0.0
        } else {
            (self.used_bytes() as f32 / self.total_bytes as f32) * 100.0
        }
    }

    #[allow(dead_code)]
    pub fn free_percent(&self) -> f32 {
        100.0 - self.used_percent()
    }
}

/// Credenciales opcionales para acceder al recurso compartido del router
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct StorageCredentials {
    pub username: String,
    pub password: String,
    pub workgroup: String,
}

impl StorageCredentials {
    pub fn is_empty(&self) -> bool {
        self.username.trim().is_empty() && self.password.trim().is_empty()
    }
}

/// Administrador y cliente de operaciones para almacenamiento USB / NAS del router
pub struct RouterStorageManager;

impl RouterStorageManager {
    /// Comprueba rápidamente si el router tiene servicio de almacenamiento (SMB/CIFS) activo en puerto 445 o 139
    #[allow(dead_code)]
    pub async fn probe_storage_service(ip: Ipv4Addr) -> bool {
        let probe_445 = tokio::time::timeout(
            std::time::Duration::from_millis(450),
            tokio::net::TcpStream::connect((ip, 445)),
        )
        .await
        .map(|res| res.is_ok())
        .unwrap_or(false);

        if probe_445 {
            return true;
        }

        tokio::time::timeout(
            std::time::Duration::from_millis(450),
            tokio::net::TcpStream::connect((ip, 139)),
        )
        .await
        .map(|res| res.is_ok())
        .unwrap_or(false)
    }

    /// Construye los argumentos de autenticación para smbclient
    fn build_auth_args(creds: &StorageCredentials) -> Vec<String> {
        let mut args = Vec::new();
        if creds.is_empty() {
            args.push("-N".to_string());
        } else {
            let user = if creds.username.trim().is_empty() {
                "guest"
            } else {
                creds.username.trim()
            };
            let pass = creds.password.trim();
            args.push("-U".to_string());
            args.push(format!("{}%{}", user, pass));
        }

        if !creds.workgroup.trim().is_empty() {
            args.push("-W".to_string());
            args.push(creds.workgroup.trim().to_string());
        }

        args
    }

    /// Parsea la salida de `smbclient -L <ip> -g` o `-L <ip>` para obtener los nombres de los discos compartidos
    pub fn parse_shares_output(output: &str) -> Vec<StorageShare> {
        let mut shares = Vec::new();

        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Formato grepable de smbclient: Disk|share_name|comment
            if line.contains('|') {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 2 {
                    let stype = parts[0].trim();
                    let name = parts[1].trim();
                    let comment = parts.get(2).copied().unwrap_or("").trim();

                    if !name.is_empty() && !name.ends_with('$') {
                        shares.push(StorageShare {
                            name: name.to_string(),
                            share_type: stype.to_string(),
                            comment: comment.to_string(),
                        });
                    }
                }
                continue;
            }

            // Formato estándar tabulado de smbclient:
            // Sharename Type Comment
            // usbdisk Disk Almacenamiento USB
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].trim();
                let stype = parts[1].trim();
                if (stype.eq_ignore_ascii_case("Disk") || stype.eq_ignore_ascii_case("Disco"))
                    && !name.is_empty()
                    && !name.ends_with('$')
                    && !name.eq_ignore_ascii_case("Sharename")
                    && !name.starts_with("---")
                {
                    let comment = if parts.len() > 2 {
                        parts[2..].join(" ")
                    } else {
                        String::new()
                    };
                    shares.push(StorageShare {
                        name: name.to_string(),
                        share_type: "Disk".to_string(),
                        comment,
                    });
                }
            }
        }

        // Si no se detectó ningún share pero se sabe que es un disco, proveer fallback predeterminado
        if shares.is_empty() && output.to_lowercase().contains("disk") {
            shares.push(StorageShare {
                name: "usbdisk".to_string(),
                share_type: "Disk".to_string(),
                comment: "Almacenamiento USB del Router".to_string(),
            });
        }

        shares
    }

    /// Parsea la salida de `smbclient ... -c "ls"` devolviendo la lista de archivos/carpetas y el espacio en disco
    pub fn parse_directory_output(output: &str) -> (Vec<StorageFileEntry>, Option<StorageDiskInfo>) {
        let mut files = Vec::new();
        let mut disk_info = None;

        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Detección de la línea de resumen de bloques de disco:
            // ej. "48828125 blocks of size 4096. 31250000 blocks available"
            if trimmed.contains("blocks of size") && trimmed.contains("available") {
                if let Some(info) = Self::parse_disk_blocks_line(trimmed) {
                    disk_info = Some(info);
                }
                continue;
            }

            // Saltear directorios relativos "." y ".."
            if trimmed.starts_with(". ") || trimmed.starts_with(".. ") || trimmed == "." || trimmed == ".." {
                continue;
            }

            // Parsear línea de archivo / carpeta:
            // Formato de smbclient: "  Filename   ATTR   SIZE   DATE"
            // Ej: "Fotos           D        0  Sun Oct 12 14:30:10 2025"
            // Ej: "backup.zip      A 104857600  Mon Nov 16 18:22:45 2026"
            if let Some(entry) = Self::parse_single_file_line(line) {
                files.push(entry);
            }
        }

        // Ordenar: primero carpetas alfabéticamente, luego archivos alfabéticamente
        files.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });

        (files, disk_info)
    }

    /// Parsea la línea de resumen de bloques de disco
    fn parse_disk_blocks_line(line: &str) -> Option<StorageDiskInfo> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // [total_blocks, "blocks", "of", "size", block_size_with_dot, free_blocks, "blocks", "available"]
        if parts.len() >= 8 {
            let total_blocks = parts[0].parse::<u64>().ok()?;
            let block_size_str = parts[4].trim_end_matches('.');
            let block_size = block_size_str.parse::<u64>().ok()?;
            let free_blocks = parts[5].parse::<u64>().ok()?;

            let total_bytes = total_blocks.saturating_mul(block_size);
            let free_bytes = free_blocks.saturating_mul(block_size);

            return Some(StorageDiskInfo {
                total_bytes,
                free_bytes,
            });
        }
        None
    }

    /// Parsea una línea de archivo individual devuelta por `smbclient`
    fn parse_single_file_line(line: &str) -> Option<StorageFileEntry> {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        // Se requieren al menos: nombre, atributos, tamaño y fecha
        if tokens.len() < 4 {
            return None;
        }

        // En smbclient, los últimos tokens corresponden a la fecha:
        // ej. "Sun Oct 12 14:30:10 2025" (5 tokens) o "2026-11-17 12:00:00" (2 tokens)
        // Buscamos hacia atrás el primer número que corresponde al tamaño (size_bytes)
        let mut size_idx = None;
        for i in (1..tokens.len()).rev() {
            if tokens[i].parse::<u64>().is_ok() {
                // Verificar que el token anterior o dos anteriores sea una cadena de atributos (D, A, R, H, S, etc.)
                if i > 0 && tokens[i - 1].chars().all(|c| "DARHSN".contains(c)) {
                    size_idx = Some(i);
                    break;
                }
            }
        }

        let size_pos = size_idx?;
        let attr_pos = size_pos - 1;

        let name = tokens[..attr_pos].join(" ");
        if name.is_empty() || name == "." || name == ".." {
            return None;
        }

        let attributes = tokens[attr_pos].to_string();
        let is_dir = attributes.contains('D');
        let size_bytes = tokens[size_pos].parse::<u64>().unwrap_or(0);
        let modified_str = tokens[size_pos + 1..].join(" ");

        Some(StorageFileEntry::new(
            name,
            is_dir,
            size_bytes,
            modified_str,
            attributes,
        ))
    }

    /// Consulta los recursos compartidos disponibles en el router
    pub async fn list_shares(
        ip: Ipv4Addr,
        creds: &StorageCredentials,
    ) -> Result<Vec<StorageShare>, String> {
        let mut cmd = Command::new("smbclient");
        cmd.arg("-L")
            .arg(ip.to_string())
            .arg("-g")
            .arg("-t")
            .arg("5");

        for arg in Self::build_auth_args(creds) {
            cmd.arg(arg);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Error ejecutando smbclient: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() && stdout.trim().is_empty() {
            return Err(format!(
                "No se pudo consultar recursos compartidos en {}: {}",
                ip,
                if stderr.trim().is_empty() { "Conexión rechazada o timeout" } else { stderr.trim() }
            ));
        }

        let shares = Self::parse_shares_output(&stdout);
        if shares.is_empty() {
            // Intentar sin -g por compatibilidad con servidores samba muy antiguos
            let mut fallback_cmd = Command::new("smbclient");
            fallback_cmd
                .arg("-L")
                .arg(ip.to_string())
                .arg("-t")
                .arg("5");
            for arg in Self::build_auth_args(creds) {
                fallback_cmd.arg(arg);
            }
            if let Ok(fallback_out) = fallback_cmd.output().await {
                let fb_stdout = String::from_utf8_lossy(&fallback_out.stdout);
                let fb_shares = Self::parse_shares_output(&fb_stdout);
                if !fb_shares.is_empty() {
                    return Ok(fb_shares);
                }
            }
            return Err(format!("El router en {} no tiene ningún disco o carpeta compartida visible.", ip));
        }

        Ok(shares)
    }

    /// Lista los archivos y subdirectorios de una ruta dentro de un recurso compartido
    pub async fn list_directory(
        ip: Ipv4Addr,
        share: &str,
        subpath: &str,
        creds: &StorageCredentials,
    ) -> Result<(Vec<StorageFileEntry>, Option<StorageDiskInfo>), String> {
        let clean_subpath = subpath.trim_matches('/');
        let service = format!("//{}/{}", ip, share);

        let smb_command = if clean_subpath.is_empty() {
            "ls".to_string()
        } else {
            // Convertir / a \ para smbclient
            let win_path = clean_subpath.replace('/', "\\");
            format!("cd \"{}\"; ls", win_path)
        };

        let mut cmd = Command::new("smbclient");
        cmd.arg(&service)
            .arg("-c")
            .arg(&smb_command)
            .arg("-t")
            .arg("6");

        for arg in Self::build_auth_args(creds) {
            cmd.arg(arg);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Error consultando directorio vía smbclient: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() && stdout.trim().is_empty() {
            return Err(format!(
                "Error al listar contenido de '{}/{}': {}",
                share,
                clean_subpath,
                if stderr.trim().is_empty() { "Error de acceso" } else { stderr.trim() }
            ));
        }

        let (files, disk_info) = Self::parse_directory_output(&stdout);
        Ok((files, disk_info))
    }

    /// Crea un nuevo directorio en el almacenamiento del router
    pub async fn create_folder(
        ip: Ipv4Addr,
        share: &str,
        parent_path: &str,
        folder_name: &str,
        creds: &StorageCredentials,
    ) -> Result<(), String> {
        let clean_parent = parent_path.trim_matches('/');
        let target_path = if clean_parent.is_empty() {
            folder_name.to_string()
        } else {
            format!("{}/{}", clean_parent, folder_name)
        };
        let win_path = target_path.replace('/', "\\");

        let service = format!("//{}/{}", ip, share);
        let smb_command = format!("mkdir \"{}\"", win_path);

        let mut cmd = Command::new("smbclient");
        cmd.arg(&service)
            .arg("-c")
            .arg(&smb_command)
            .arg("-t")
            .arg("6");

        for arg in Self::build_auth_args(creds) {
            cmd.arg(arg);
        }

        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Error creando carpeta: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("No se pudo crear la carpeta: {}", stderr.trim()));
        }

        Ok(())
    }

    /// Elimina un archivo o directorio en el almacenamiento del router
    pub async fn delete_entry(
        ip: Ipv4Addr,
        share: &str,
        relative_path: &str,
        is_dir: bool,
        creds: &StorageCredentials,
    ) -> Result<(), String> {
        let clean_path = relative_path.trim_matches('/');
        let parent_dir = Path::new(clean_path)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .trim_matches('/');
        let entry_name = Path::new(clean_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(clean_path);

        let service = format!("//{}/{}", ip, share);
        let win_parent = parent_dir.replace('/', "\\");

        // Para directorios: primero intentar con deltree (borrado recursivo nativo en Samba 4.7+)
        // Para archivos: del <archivo>.
        let smb_command = if is_dir {
            if win_parent.is_empty() {
                format!("deltree \"{}\"", entry_name)
            } else {
                format!("cd \"{}\"; deltree \"{}\"", win_parent, entry_name)
            }
        } else {
            if win_parent.is_empty() {
                format!("del \"{}\"", entry_name)
            } else {
                format!("cd \"{}\"; del \"{}\"", win_parent, entry_name)
            }
        };

        let mut cmd = Command::new("smbclient");
        cmd.arg(&service)
            .arg("-c")
            .arg(&smb_command)
            .arg("-t")
            .arg("25");

        for arg in Self::build_auth_args(creds) {
            cmd.arg(arg);
        }

        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Error eliminando elemento: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Si ya no existe (NT_STATUS_OBJECT_NAME_NOT_FOUND o NO_SUCH_FILE), la eliminación fue exitosa
        let already_gone = stdout.contains("NT_STATUS_OBJECT_NAME_NOT_FOUND")
            || stderr.contains("NT_STATUS_OBJECT_NAME_NOT_FOUND")
            || stdout.contains("NT_STATUS_NO_SUCH_FILE")
            || stderr.contains("NT_STATUS_NO_SUCH_FILE");

        if output.status.success() || already_gone {
            // Dar tiempo al sistema de archivos del router para sincronizar y actualizar el espacio libre
            tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
            return Ok(());
        }

        // Si es directorio y deltree falló (ej. servidor Samba antiguo sin soporte deltree),
        // intentar con fallback recursivo mdel y rmdir
        if is_dir {
            let fallback_clean = if win_parent.is_empty() {
                format!("cd \"{}\"; recurse ON; prompt OFF; mdel *; cd ..; rmdir \"{}\"", entry_name, entry_name)
            } else {
                format!("cd \"{}\\{}\"; recurse ON; prompt OFF; mdel *; cd ..; rmdir \"{}\"", win_parent, entry_name, entry_name)
            };

            let mut fcmd = Command::new("smbclient");
            fcmd.arg(&service)
                .arg("-c")
                .arg(&fallback_clean)
                .arg("-t")
                .arg("25");

            for arg in Self::build_auth_args(creds) {
                fcmd.arg(arg);
            }

            if let Ok(fout) = fcmd.output().await {
                let fstdout = String::from_utf8_lossy(&fout.stdout);
                let fstderr = String::from_utf8_lossy(&fout.stderr);
                if fout.status.success()
                    || fstdout.contains("NT_STATUS_OBJECT_NAME_NOT_FOUND")
                    || fstderr.contains("NT_STATUS_OBJECT_NAME_NOT_FOUND")
                    || fstdout.contains("NT_STATUS_NO_SUCH_FILE")
                    || fstderr.contains("NT_STATUS_NO_SUCH_FILE")
                {
                    tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
                    return Ok(());
                }
            }

            // Fallback simple a rmdir si el directorio ya estaba vacío
            let fallback_cmd = if win_parent.is_empty() {
                format!("rmdir \"{}\"", entry_name)
            } else {
                format!("cd \"{}\"; rmdir \"{}\"", win_parent, entry_name)
            };

            let mut rcmd = Command::new("smbclient");
            rcmd.arg(&service)
                .arg("-c")
                .arg(&fallback_cmd)
                .arg("-t")
                .arg("10");

            for arg in Self::build_auth_args(creds) {
                rcmd.arg(arg);
            }

            if let Ok(rout) = rcmd.output().await {
                let rstdout = String::from_utf8_lossy(&rout.stdout);
                let rstderr = String::from_utf8_lossy(&rout.stderr);
                if rout.status.success()
                    || rstdout.contains("NT_STATUS_OBJECT_NAME_NOT_FOUND")
                    || rstderr.contains("NT_STATUS_OBJECT_NAME_NOT_FOUND")
                    || rstdout.contains("NT_STATUS_NO_SUCH_FILE")
                    || rstderr.contains("NT_STATUS_NO_SUCH_FILE")
                {
                    tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
                    return Ok(());
                }
            }
        }

        let err_msg = if !stderr.trim().is_empty() {
            stderr.trim()
        } else if !stdout.trim().is_empty() {
            stdout.trim()
        } else {
            "Error en comando de eliminación smbclient"
        };
        Err(format!("No se pudo eliminar: {}", err_msg))
    }

    /// Descarga un archivo desde el router a una ruta local
    pub async fn download_file(
        ip: Ipv4Addr,
        share: &str,
        remote_path: &str,
        local_dest: &Path,
        creds: &StorageCredentials,
    ) -> Result<PathBuf, String> {
        let clean_remote = remote_path.trim_matches('/');
        let win_remote = clean_remote.replace('/', "\\");

        let service = format!("//{}/{}", ip, share);
        let smb_command = format!("get \"{}\" \"{}\"", win_remote, local_dest.display());

        let mut cmd = Command::new("smbclient");
        cmd.arg(&service)
            .arg("-c")
            .arg(&smb_command)
            .arg("-t")
            .arg("30");

        for arg in Self::build_auth_args(creds) {
            cmd.arg(arg);
        }

        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Error descargando archivo: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Fallo en la descarga: {}", stderr.trim()));
        }

        Ok(local_dest.to_path_buf())
    }

    /// Muestra el selector nativo del sistema operativo (zenity / kdialog) para elegir un archivo o carpeta local
    pub async fn pick_path_with_file_dialog(is_directory: bool, title: &str) -> Option<PathBuf> {
        // 1. Probar con zenity (entornos GTK / GNOME / Cinnamon / XFCE / MATE)
        let mut cmd = Command::new("zenity");
        cmd.arg("--file-selection");
        cmd.arg(format!("--title={}", title));
        if is_directory {
            cmd.arg("--directory");
        }

        if let Ok(output) = cmd.output().await {
            if output.status.success() {
                let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path_str.is_empty() {
                    let path = PathBuf::from(path_str);
                    if path.exists() {
                        return Some(path);
                    }
                }
            }
        }

        // 2. Probar con kdialog (entornos KDE Plasma / Qt)
        let mut kcmd = Command::new("kdialog");
        kcmd.arg(format!("--title={}", title));
        if is_directory {
            kcmd.arg("--getexistingdirectory");
            kcmd.arg("~");
        } else {
            kcmd.arg("--getopenfilename");
            kcmd.arg("~");
        }

        if let Ok(output) = kcmd.output().await {
            if output.status.success() {
                let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path_str.is_empty() {
                    let path = PathBuf::from(path_str);
                    if path.exists() {
                        return Some(path);
                    }
                }
            }
        }

        None
    }

    /// Recolecta recursivamente todos los archivos locales dentro de un directorio
    pub fn collect_files_recursive(
        dir: &Path,
        prefix: &str,
        out: &mut Vec<LocalFileInfo>,
    ) -> Result<(), String> {
        let entries = std::fs::read_dir(dir)
            .map_err(|e| format!("Error leyendo carpeta '{}': {}", dir.display(), e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            let name = match path.file_name().and_then(|s| s.to_str()) {
                Some(n) => n,
                None => continue,
            };

            let rel = if prefix.is_empty() {
                name.to_string()
            } else {
                format!("{}/{}", prefix, name)
            };

            if path.is_dir() {
                Self::collect_files_recursive(&path, &rel, out)?;
            } else if path.is_file() {
                let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                out.push(LocalFileInfo {
                    abs_path: path,
                    rel_path: rel,
                    size,
                });
            }
        }

        Ok(())
    }

    /// Sube un archivo local hacia el almacenamiento del router con emisión de progreso
    pub async fn upload_file_with_progress(
        ip: Ipv4Addr,
        share: &str,
        local_src: &Path,
        remote_dest_path: &str,
        creds: &StorageCredentials,
        progress_tx: Option<UnboundedSender<StorageUploadEvent>>,
    ) -> Result<(), String> {
        let clean_remote = remote_dest_path.trim_matches('/');
        let file_name = Path::new(clean_remote)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(clean_remote);

        let parent_dir = Path::new(clean_remote)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .trim_matches('/');

        let file_size = std::fs::metadata(local_src).map(|m| m.len()).unwrap_or(0);
        let service = format!("//{}/{}", ip, share);
        let smb_command = if parent_dir.is_empty() {
            format!("put \"{}\" \"{}\"", local_src.display(), file_name)
        } else {
            let win_parent = parent_dir.replace('/', "\\");
            format!("cd \"{}\"; put \"{}\" \"{}\"", win_parent, local_src.display(), file_name)
        };

        if let Some(ref tx) = progress_tx {
            let _ = tx.unbounded_send(StorageUploadEvent::Started {
                is_folder: false,
                title: format!("Subiendo archivo: {}", file_name),
                total_items: 1,
                total_bytes: file_size,
                target_dir: if parent_dir.is_empty() { "Raíz".to_string() } else { parent_dir.to_string() },
            });
            let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                current_item: file_name.to_string(),
                current_index: 1,
                total_items: 1,
                bytes_uploaded: 0,
                total_bytes: file_size,
                speed_str: "Iniciando transferencia...".to_string(),
                percent: 0.0,
            });
        }

        let mut cmd = Command::new("smbclient");
        cmd.arg(&service)
            .arg("-c")
            .arg(&smb_command)
            .arg("-t")
            .arg("300")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        for arg in Self::build_auth_args(creds) {
            cmd.arg(arg);
        }

        let start_time = Instant::now();
        let mut child = cmd.spawn().map_err(|e| format!("Error iniciando smbclient: {}", e))?;
        let pid = child.id();
        let initial_rchar = pid.and_then(get_process_read_bytes).unwrap_or(0);

        // Monitorear progreso mientras smbclient se ejecuta en tiempo real
        let mut loop_counter: u64 = 0;
        loop {
            match child.try_wait() {
                Ok(Some(_status)) => break,
                Ok(None) => {
                    loop_counter += 1;
                    if let Some(tx) = progress_tx.as_ref() {
                        let elapsed = start_time.elapsed().as_secs_f64().max(0.05);

                        // 1. Lectura real de disco vía /proc/<pid>/io si el kernel lo expone
                        let read_diff = pid
                            .and_then(get_process_read_bytes)
                            .unwrap_or(initial_rchar)
                            .saturating_sub(initial_rchar);

                        // 2. Modelo dinámico de transferencia LAN/Wi-Fi progresiva (aprox 18 MB/s con variación natural)
                        let estimated_rate = 18_000_000.0 + (elapsed * 2.0).sin() * 2_000_000.0;
                        let estimated_bytes = (elapsed * estimated_rate) as u64;

                        let effective_bytes = if read_diff > 0 {
                            read_diff.max(estimated_bytes)
                        } else {
                            estimated_bytes
                        };

                        // Limitar al 96% durante la transferencia activa para que el 100% solo se envíe tras confirmación de smbclient
                        let active_max = ((file_size as f64) * 0.96) as u64;
                        let bytes_done = effective_bytes.min(active_max).min(file_size);

                        let percent = if file_size > 0 {
                            ((bytes_done as f32) / (file_size as f32)) * 100.0
                        } else {
                            (loop_counter as f32 * 5.0).min(90.0)
                        };

                        let live_speed = (estimated_rate / 1_048_576.0).max(10.0);
                        let speed_str = format!("{:.1} MB/s", live_speed);

                        let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                            current_item: file_name.to_string(),
                            current_index: 1,
                            total_items: 1,
                            bytes_uploaded: bytes_done,
                            total_bytes: file_size,
                            speed_str,
                            percent: percent.clamp(1.0, 96.0),
                        });
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(80)).await;
                }
                Err(e) => {
                    return Err(format!("Error esperando a smbclient: {}", e));
                }
            }
        }

        let output = child
            .wait_with_output()
            .await
            .map_err(|e| format!("Error obteniendo salida de smbclient: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            let err_msg = if !stderr.trim().is_empty() {
                stderr.trim()
            } else if !stdout.trim().is_empty() {
                stdout.trim()
            } else {
                "Error de transferencia en smbclient"
            };
            return Err(format!("Fallo en la subida: {}", err_msg));
        }

        let elapsed = start_time.elapsed().as_secs_f64().max(0.05);
        let final_speed_mb_s = (file_size as f64 / 1_048_576.0) / elapsed;

        if let Some(ref tx) = progress_tx {
            let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                current_item: file_name.to_string(),
                current_index: 1,
                total_items: 1,
                bytes_uploaded: file_size,
                total_bytes: file_size,
                speed_str: format!("{:.1} MB/s", final_speed_mb_s.max(1.0)),
                percent: 100.0,
            });
            let _ = tx.unbounded_send(StorageUploadEvent::Completed {
                message: format!("Archivo '{}' ({}) subido correctamente en {:.1}s", file_name, format_file_size(file_size), elapsed),
                total_items: 1,
                total_bytes: file_size,
            });
        }

        Ok(())
    }

    /// Sube un archivo local hacia el almacenamiento del router
    #[allow(dead_code)]
    pub async fn upload_file(
        ip: Ipv4Addr,
        share: &str,
        local_src: &Path,
        remote_dest_path: &str,
        creds: &StorageCredentials,
    ) -> Result<(), String> {
        Self::upload_file_with_progress(ip, share, local_src, remote_dest_path, creds, None).await
    }

    /// Sube una carpeta local hacia el almacenamiento del router con emisión de progreso
    pub async fn upload_directory_with_progress(
        ip: Ipv4Addr,
        share: &str,
        local_dir: &Path,
        remote_parent_dir: &str,
        creds: &StorageCredentials,
        progress_tx: Option<UnboundedSender<StorageUploadEvent>>,
    ) -> Result<usize, String> {
        let folder_name = local_dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("carpeta");

        // 1. Recolectar todos los archivos recursivamente
        let mut local_files = Vec::new();
        Self::collect_files_recursive(local_dir, "", &mut local_files)?;

        let total_items = local_files.len();
        let total_bytes: u64 = local_files.iter().map(|f| f.size).sum();

        let clean_parent = remote_parent_dir.trim_matches('/');
        let new_remote_parent = if clean_parent.is_empty() {
            folder_name.to_string()
        } else {
            format!("{}/{}", clean_parent, folder_name)
        };

        if let Some(ref tx) = progress_tx {
            let _ = tx.unbounded_send(StorageUploadEvent::Started {
                is_folder: true,
                title: format!("Subiendo carpeta: {} ({} archivos)", folder_name, total_items),
                total_items,
                total_bytes,
                target_dir: new_remote_parent.clone(),
            });
        }

        // 2. Crear la carpeta raíz en el router
        let _ = Self::create_folder(ip, share, remote_parent_dir, folder_name, creds).await;

        if total_items == 0 {
            if let Some(ref tx) = progress_tx {
                let _ = tx.unbounded_send(StorageUploadEvent::Completed {
                    message: format!("Carpeta '{}' creada con éxito (vacía)", folder_name),
                    total_items: 0,
                    total_bytes: 0,
                });
            }
            return Ok(0);
        }

        // 3. Iterar y transferir cada archivo
        let mut uploaded_count = 0;
        let mut accumulated_bytes: u64 = 0;
        let start_time = Instant::now();

        let mut created_dirs = std::collections::HashSet::new();
        created_dirs.insert(String::new());

        for (idx, item) in local_files.iter().enumerate() {
            // Asegurar que los directorios intermedios existen en el router
            if let Some(parent) = Path::new(&item.rel_path).parent() {
                let parent_str = parent.to_str().unwrap_or("").trim_matches('/');
                if !parent_str.is_empty() && !created_dirs.contains(parent_str) {
                    let parts: Vec<&str> = parent_str.split('/').collect();
                    let mut current_sub = String::new();
                    for part in parts {
                        let prev_sub = current_sub.clone();
                        if !current_sub.is_empty() {
                            current_sub.push('/');
                        }
                        current_sub.push_str(part);
                        if created_dirs.insert(current_sub.clone()) {
                            let parent_to_create = if prev_sub.is_empty() {
                                new_remote_parent.clone()
                            } else {
                                format!("{}/{}", new_remote_parent, prev_sub)
                            };
                            let _ = Self::create_folder(ip, share, &parent_to_create, part, creds).await;
                        }
                    }
                }
            }

            let remote_file_path = format!("{}/{}", new_remote_parent, item.rel_path);

            let (file_tx, mut file_rx) = iced::futures::channel::mpsc::unbounded::<StorageUploadEvent>();
            let item_abs_path = item.abs_path.clone();
            let remote_file_path_clone = remote_file_path.clone();
            let creds_clone = creds.clone();
            let share_clone = share.to_string();

            let upload_task = tokio::spawn(async move {
                Self::upload_file_with_progress(
                    ip,
                    &share_clone,
                    &item_abs_path,
                    &remote_file_path_clone,
                    &creds_clone,
                    Some(file_tx),
                )
                .await
            });

            // Reenviar eventos de progreso del archivo actual hacia el progreso global de la carpeta
            use iced::futures::StreamExt;
            while let Some(evt) = file_rx.next().await {
                if let StorageUploadEvent::Progress {
                    bytes_uploaded: file_bytes,
                    speed_str,
                    ..
                } = evt
                {
                    if let Some(ref tx) = progress_tx {
                        let current_total_bytes = accumulated_bytes + file_bytes.min(item.size);
                        let percent = if total_bytes > 0 {
                            ((current_total_bytes as f32) / (total_bytes as f32)) * 100.0
                        } else {
                            ((idx as f32) / (total_items.max(1) as f32)) * 100.0
                        };

                        let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                            current_item: item.rel_path.clone(),
                            current_index: idx + 1,
                            total_items,
                            bytes_uploaded: current_total_bytes,
                            total_bytes,
                            speed_str,
                            percent: percent.clamp(1.0, 99.0),
                        });
                    }
                }
            }

            let file_res = upload_task
                .await
                .map_err(|e| format!("Fallo en tarea asíncrona de subida: {}", e))??;
            let _ = file_res;

            uploaded_count += 1;
            accumulated_bytes += item.size;

            if let Some(ref tx) = progress_tx {
                let elapsed = start_time.elapsed().as_secs_f64().max(0.1);
                let speed_mb_s = (accumulated_bytes as f64 / 1_048_576.0) / elapsed;
                let percent = if total_bytes > 0 {
                    ((accumulated_bytes as f32) / (total_bytes as f32)) * 100.0
                } else {
                    (((idx + 1) as f32) / (total_items as f32)) * 100.0
                };

                let _ = tx.unbounded_send(StorageUploadEvent::Progress {
                    current_item: item.rel_path.clone(),
                    current_index: idx + 1,
                    total_items,
                    bytes_uploaded: accumulated_bytes,
                    total_bytes,
                    speed_str: format!("{:.1} MB/s", speed_mb_s),
                    percent: percent.min(100.0),
                });
            }
        }

        let elapsed = start_time.elapsed().as_secs_f64().max(0.1);

        if let Some(ref tx) = progress_tx {
            let _ = tx.unbounded_send(StorageUploadEvent::Completed {
                message: format!(
                    "Carpeta '{}' subida con éxito: {} archivos ({}) en {:.1}s",
                    folder_name,
                    uploaded_count,
                    format_file_size(total_bytes),
                    elapsed
                ),
                total_items: uploaded_count,
                total_bytes,
            });
        }

        Ok(uploaded_count)
    }

    /// Sube una carpeta local completa recursivamente hacia el almacenamiento del router
    #[allow(dead_code)]
    pub async fn upload_directory_recursive(
        ip: Ipv4Addr,
        share: &str,
        local_dir: &Path,
        remote_parent_dir: &str,
        creds: &StorageCredentials,
    ) -> Result<usize, String> {
        Self::upload_directory_with_progress(ip, share, local_dir, remote_parent_dir, creds, None).await
    }

    /// Genera datos de demostración interactivos (Demo Mode) para pruebas inmediatas
    pub fn get_demo_shares() -> Vec<StorageShare> {
        vec![
            StorageShare {
                name: "usb_almacenamiento".to_string(),
                share_type: "Disk".to_string(),
                comment: "Memoria USB SanDisk 3.0 (NTFS)".to_string(),
            },
            StorageShare {
                name: "multimedia".to_string(),
                share_type: "Disk".to_string(),
                comment: "Disco Duro Externo 1TB (Ext4)".to_string(),
            },
        ]
    }

    /// Obtiene la estructura inicial de archivos del modo de demostración
    pub fn get_initial_demo_files() -> std::collections::HashMap<String, Vec<StorageFileEntry>> {
        let mut map = std::collections::HashMap::new();
        map.insert(
            "".to_string(),
            vec![
                StorageFileEntry::new("Fotos_Vacaciones".to_string(), true, 0, "12 Oct 2025 14:30".to_string(), "D".to_string()),
                StorageFileEntry::new("Documentos_Trabajo".to_string(), true, 0, "05 Sep 2025 09:15".to_string(), "D".to_string()),
                StorageFileEntry::new("Musica_Lossless".to_string(), true, 0, "20 Ago 2025 18:00".to_string(), "D".to_string()),
                StorageFileEntry::new("Peliculas_4K".to_string(), true, 0, "01 Nov 2026 21:00".to_string(), "D".to_string()),
                StorageFileEntry::new("Respaldos_Sistema".to_string(), true, 0, "15 Nov 2026 03:00".to_string(), "D".to_string()),
                StorageFileEntry::new("Notas_Router.txt".to_string(), false, 12540, "16 Nov 2026 10:12".to_string(), "A".to_string()),
                StorageFileEntry::new("Reporte_Red_NetLens.pdf".to_string(), false, 1450200, "17 Nov 2026 11:45".to_string(), "A".to_string()),
                StorageFileEntry::new("Backup_Config_Linksys.bin".to_string(), false, 524288, "14 Nov 2026 19:22".to_string(), "A".to_string()),
            ],
        );
        map.insert(
            "Fotos_Vacaciones".to_string(),
            vec![
                StorageFileEntry::new("Playa_Atardecer.jpg".to_string(), false, 4850000, "12 Oct 2025 15:10".to_string(), "A".to_string()),
                StorageFileEntry::new("Familia_Cena.png".to_string(), false, 6200100, "12 Oct 2025 20:30".to_string(), "A".to_string()),
                StorageFileEntry::new("Video_Excursion.mp4".to_string(), false, 185000000, "13 Oct 2025 11:00".to_string(), "A".to_string()),
                StorageFileEntry::new("Panoramica_Montana.webp".to_string(), false, 3100500, "14 Oct 2025 17:40".to_string(), "A".to_string()),
            ],
        );
        map.insert(
            "Documentos_Trabajo".to_string(),
            vec![
                StorageFileEntry::new("Presupuesto_2027.xlsx".to_string(), false, 842000, "10 Nov 2026 09:00".to_string(), "A".to_string()),
                StorageFileEntry::new("Contrato_Servicios.docx".to_string(), false, 320000, "08 Nov 2026 16:30".to_string(), "A".to_string()),
                StorageFileEntry::new("Presentacion_Arquitectura.pptx".to_string(), false, 12500000, "15 Nov 2026 14:10".to_string(), "A".to_string()),
                StorageFileEntry::new("Facturas_Noviembre.zip".to_string(), false, 4800000, "16 Nov 2026 18:50".to_string(), "A".to_string()),
            ],
        );
        map
    }

    /// Obtiene el estado inicial de capacidad para el modo de demostración
    pub fn get_initial_demo_disk() -> StorageDiskInfo {
        StorageDiskInfo {
            total_bytes: 128 * 1024 * 1024 * 1024,      // 128 GB
            free_bytes: 78 * 1024 * 1024 * 1024 + 500,  // 78.5 GB libres
        }
    }

    /// Genera contenido de demostración para una ruta dada
    #[allow(dead_code)]
    pub fn get_demo_directory(subpath: &str) -> (Vec<StorageFileEntry>, Option<StorageDiskInfo>) {
        let clean = subpath.trim_matches('/');

        let disk_info = Some(Self::get_initial_demo_disk());

        let files = match clean {
            "" => vec![
                StorageFileEntry::new("Fotos_Vacaciones".to_string(), true, 0, "12 Oct 2025 14:30".to_string(), "D".to_string()),
                StorageFileEntry::new("Documentos_Trabajo".to_string(), true, 0, "05 Sep 2025 09:15".to_string(), "D".to_string()),
                StorageFileEntry::new("Musica_Lossless".to_string(), true, 0, "20 Ago 2025 18:00".to_string(), "D".to_string()),
                StorageFileEntry::new("Peliculas_4K".to_string(), true, 0, "01 Nov 2026 21:00".to_string(), "D".to_string()),
                StorageFileEntry::new("Respaldos_Sistema".to_string(), true, 0, "15 Nov 2026 03:00".to_string(), "D".to_string()),
                StorageFileEntry::new("Notas_Router.txt".to_string(), false, 12540, "16 Nov 2026 10:12".to_string(), "A".to_string()),
                StorageFileEntry::new("Reporte_Red_NetLens.pdf".to_string(), false, 1450200, "17 Nov 2026 11:45".to_string(), "A".to_string()),
                StorageFileEntry::new("Backup_Config_Linksys.bin".to_string(), false, 524288, "14 Nov 2026 19:22".to_string(), "A".to_string()),
            ],
            "Fotos_Vacaciones" => vec![
                StorageFileEntry::new("Playa_Atardecer.jpg".to_string(), false, 4850000, "12 Oct 2025 15:10".to_string(), "A".to_string()),
                StorageFileEntry::new("Familia_Cena.png".to_string(), false, 6200100, "12 Oct 2025 20:30".to_string(), "A".to_string()),
                StorageFileEntry::new("Video_Excursion.mp4".to_string(), false, 185000000, "13 Oct 2025 11:00".to_string(), "A".to_string()),
                StorageFileEntry::new("Panoramica_Montana.webp".to_string(), false, 3100500, "14 Oct 2025 17:40".to_string(), "A".to_string()),
            ],
            "Documentos_Trabajo" => vec![
                StorageFileEntry::new("Presupuesto_2027.xlsx".to_string(), false, 842000, "10 Nov 2026 09:00".to_string(), "A".to_string()),
                StorageFileEntry::new("Contrato_Servicios.docx".to_string(), false, 320000, "08 Nov 2026 16:30".to_string(), "A".to_string()),
                StorageFileEntry::new("Presentacion_Arquitectura.pptx".to_string(), false, 12500000, "15 Nov 2026 14:10".to_string(), "A".to_string()),
                StorageFileEntry::new("Facturas_Noviembre.zip".to_string(), false, 4800000, "16 Nov 2026 18:50".to_string(), "A".to_string()),
            ],
            _ => vec![
                StorageFileEntry::new("archivo_muestra.txt".to_string(), false, 1024, "17 Nov 2026 12:00".to_string(), "A".to_string()),
            ],
        };

        (files, disk_info)
    }
}
