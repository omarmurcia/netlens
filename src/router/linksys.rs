use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterInfo {
    pub manufacturer: String,
    pub model_number: String,
    pub description: String,
    pub firmware_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacFilterSettings {
    #[serde(rename = "macFilterMode")]
    pub mac_filter_mode: String, // "disabled", "allow", "deny"
    #[serde(rename = "macAddresses")]
    pub mac_addresses: Vec<String>,
}

#[derive(Deserialize)]
struct JnapGenericResponse<T> {
    result: String,
    output: Option<T>,
    error: Option<String>,
}

pub struct LinksysRouter;

impl LinksysRouter {
    /// Codificador Base64 autónomo para cabeceras HTTP Basic
    pub fn base64_encode(input: &str) -> String {
        const CHARSET: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let bytes = input.as_bytes();
        let mut result = String::with_capacity((bytes.len() + 2) / 3 * 4);

        for chunk in bytes.chunks(3) {
            let b0 = chunk[0] as usize;
            let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
            let b2 = chunk.get(2).copied().unwrap_or(0) as usize;

            let triple = (b0 << 16) | (b1 << 8) | b2;

            result.push(CHARSET[(triple >> 18) & 0x3F] as char);
            result.push(CHARSET[(triple >> 12) & 0x3F] as char);

            if chunk.len() > 1 {
                result.push(CHARSET[(triple >> 6) & 0x3F] as char);
            } else {
                result.push('=');
            }

            if chunk.len() > 2 {
                result.push(CHARSET[triple & 0x3F] as char);
            } else {
                result.push('=');
            }
        }

        result
    }

    /// Ejecuta una llamada JNAP al router Linksys
    pub async fn execute_jnap<T: for<'de> Deserialize<'de>>(
        ip: &str,
        action: &str,
        payload_json: &str,
        password: Option<&str>,
    ) -> Result<T, String> {
        let url = format!("http://{}/JNAP/", ip);
        let mut cmd = Command::new("curl");
        cmd.arg("-s")
            .arg("--max-time")
            .arg("4")
            .arg("-H")
            .arg("Content-Type: application/json; charset=UTF-8")
            .arg("-H")
            .arg(format!("X-JNAP-Action: {}", action))
            .arg("-X")
            .arg("POST")
            .arg("--data-binary")
            .arg("@-")
            .arg(&url)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped());

        if let Some(pass) = password {
            let auth = format!("admin:{}", pass);
            let b64 = Self::base64_encode(&auth);
            cmd.arg("-H").arg(format!("X-JNAP-Authorization: Basic {}", b64));
        }

        let mut child = cmd
            .spawn()
            .map_err(|e| format!("Error iniciando petición curl JNAP: {}", e))?;

        if let Some(mut stdin) = child.stdin.take() {
            let data = payload_json.as_bytes().to_vec();
            tokio::spawn(async move {
                let _ = stdin.write_all(&data).await;
            });
        }

        let output = child
            .wait_with_output()
            .await
            .map_err(|e| format!("Error esperando respuesta JNAP: {}", e))?;

        if !output.status.success() {
            return Err("El router no respondió o no se encuentra accesible en la IP especificada".to_string());
        }

        let body = String::from_utf8_lossy(&output.stdout);
        let resp: JnapGenericResponse<T> = serde_json::from_str(&body)
            .map_err(|e| format!("Error deserializando respuesta JNAP: {}. Respuesta: {}", e, body))?;

        if resp.result != "OK" {
            let err_msg = resp.error.unwrap_or(resp.result);
            return Err(format!("Error en router JNAP: {}", err_msg));
        }

        resp.output
            .ok_or_else(|| "Respuesta de router no incluyó sección de datos (output)".to_string())
    }

    /// Obtiene información de hardware y firmware del router Linksys
    #[allow(dead_code)]
    pub async fn get_device_info(ip: &str) -> Result<RouterInfo, String> {
        #[derive(Deserialize)]
        struct Output {
            manufacturer: String,
            #[serde(rename = "modelNumber")]
            model_number: String,
            description: String,
            #[serde(rename = "firmwareVersion")]
            firmware_version: String,
        }

        let out: Output = Self::execute_jnap(ip, "http://linksys.com/jnap/core/GetDeviceInfo", "{}", None).await?;
        Ok(RouterInfo {
            manufacturer: out.manufacturer,
            model_number: out.model_number,
            description: out.description,
            firmware_version: out.firmware_version,
        })
    }

    /// Verifica si la contraseña de administración ingresada es válida
    pub async fn verify_admin_password(ip: &str, password: &str) -> Result<bool, String> {
        let res: Result<serde_json::Value, String> = Self::execute_jnap(
            ip,
            "http://linksys.com/jnap/macfilter/GetMACFilterSettings",
            "{}",
            Some(password),
        )
        .await;

        match res {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.contains("Unauthorized") || e.contains("credentials") {
                    Ok(false)
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Obtiene la configuración actual del filtro de direcciones MAC
    pub async fn get_mac_filter_settings(ip: &str, password: &str) -> Result<MacFilterSettings, String> {
        Self::execute_jnap(
            ip,
            "http://linksys.com/jnap/macfilter/GetMACFilterSettings",
            "{}",
            Some(password),
        )
        .await
    }

    /// Aplica una nueva lista de filtro MAC en el router
    pub async fn set_mac_filter_settings(
        ip: &str,
        password: &str,
        settings: &MacFilterSettings,
    ) -> Result<(), String> {
        let payload = serde_json::to_string(settings)
            .map_err(|e| format!("Error serializando MacFilterSettings: {}", e))?;

        let _: serde_json::Value = Self::execute_jnap(
            ip,
            "http://linksys.com/jnap/macfilter/SetMACFilterSettings",
            &payload,
            Some(password),
        )
        .await?;

        Ok(())
    }

    /// Bloquea una dirección MAC en el router mediante la lista negra permanente
    pub async fn block_mac(ip: &str, password: &str, mac_to_block: &str) -> Result<(), String> {
        let mut current = Self::get_mac_filter_settings(ip, password).await?;
        let mac_norm = mac_to_block.trim().to_uppercase();

        if !current
            .mac_addresses
            .iter()
            .any(|m| m.trim().eq_ignore_ascii_case(&mac_norm))
        {
            current.mac_addresses.push(mac_norm);
        }

        current.mac_filter_mode = "deny".to_string();
        Self::set_mac_filter_settings(ip, password, &current).await
    }

    /// Desbloquea / readmite una dirección MAC en el router
    pub async fn unblock_mac(ip: &str, password: &str, mac_to_unblock: &str) -> Result<(), String> {
        let mut current = Self::get_mac_filter_settings(ip, password).await?;
        let mac_norm = mac_to_unblock.trim();

        current
            .mac_addresses
            .retain(|m| !m.trim().eq_ignore_ascii_case(mac_norm));

        if current.mac_addresses.is_empty() {
            current.mac_filter_mode = "disabled".to_string();
        }

        Self::set_mac_filter_settings(ip, password, &current).await
    }
}
