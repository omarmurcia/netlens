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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MacFilterSettings {
    #[serde(rename = "macFilterMode", default)]
    pub mac_filter_mode: String, // "Disabled", "Allow", "Deny"
    #[serde(rename = "macAddresses", default)]
    pub mac_addresses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterWirelessInfo {
    pub bssid: Option<String>,
    #[serde(rename = "isGuest", default)]
    pub is_guest: bool,
    pub band: Option<String>,
    #[serde(rename = "signalDecibels")]
    pub signal_decibels: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterConnection {
    #[serde(rename = "macAddress")]
    pub mac_address: String,
    #[serde(rename = "negotiatedMbps")]
    pub negotiated_mbps: Option<u32>,
    pub wireless: Option<RouterWirelessInfo>,
}

#[derive(Deserialize)]
struct GetNetworkConnectionsOutput {
    #[serde(default)]
    connections: Vec<RouterConnection>,
}

#[derive(Deserialize)]
struct JnapGenericResponse<T> {
    result: String,
    output: Option<T>,
    error: Option<String>,
}

pub struct LinksysRouter;

impl LinksysRouter {
    /// Standalone Base64 encoder for HTTP Basic headers
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

    /// Executes a JNAP call to the Linksys router deserializing general response container
    async fn execute_jnap_raw<T: for<'de> Deserialize<'de>>(
        ip: &str,
        action: &str,
        payload_json: &str,
        password: Option<&str>,
    ) -> Result<JnapGenericResponse<T>, String> {
        let url = format!("http://{}/JNAP/", ip);
        let mut cmd = crate::procutil::NoWindow::no_window(Command::new("curl"));
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
            .map_err(|e| format!("Error initiating curl JNAP request: {}", e))?;

        if let Some(mut stdin) = child.stdin.take() {
            let data = payload_json.as_bytes().to_vec();
            tokio::spawn(async move {
                let _ = stdin.write_all(&data).await;
            });
        }

        let output = child
            .wait_with_output()
            .await
            .map_err(|e| format!("Error awaiting JNAP response: {}", e))?;

        if !output.status.success() {
            return Err("Router did not respond or is not reachable at the specified IP".to_string());
        }

        let body = String::from_utf8_lossy(&output.stdout);
        let resp: JnapGenericResponse<T> = serde_json::from_str(&body)
            .map_err(|e| format!("Error deserializing JNAP response: {}. Response: {}", e, body))?;

        if resp.result != "OK" {
            let err_msg = resp.error.unwrap_or(resp.result);
            return Err(format!("Router JNAP error: {}", err_msg));
        }

        Ok(resp)
    }

    /// Executes a query-type JNAP call (GET) that returns data in "output" field
    pub async fn execute_jnap<T: for<'de> Deserialize<'de>>(
        ip: &str,
        action: &str,
        payload_json: &str,
        password: Option<&str>,
    ) -> Result<T, String> {
        let resp: JnapGenericResponse<T> =
            Self::execute_jnap_raw(ip, action, payload_json, password).await?;
        resp.output
            .ok_or_else(|| "Router response did not include data section (output)".to_string())
    }

    /// Executes an action/configuration JNAP call (SET) where response is {"result": "OK"}
    pub async fn execute_jnap_action(
        ip: &str,
        action: &str,
        payload_json: &str,
        password: Option<&str>,
    ) -> Result<(), String> {
        let _: JnapGenericResponse<serde_json::Value> =
            Self::execute_jnap_raw(ip, action, payload_json, password).await?;
        Ok(())
    }

    /// Retrieves hardware and firmware information from Linksys router
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

    /// Verifies if administrative password is valid
    pub async fn verify_admin_password(ip: &str, password: &str) -> Result<bool, String> {
        let res: Result<JnapGenericResponse<serde_json::Value>, String> = Self::execute_jnap_raw(
            ip,
            "http://linksys.com/jnap/macfilter/GetMACFilterSettings",
            "{}",
            Some(password),
        )
        .await;

        match res {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.contains("Unauthorized") || e.contains("credentials") || e.contains("Invalid") {
                    Ok(false)
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Retrieves current MAC address filter configuration
    pub async fn get_mac_filter_settings(ip: &str, password: &str) -> Result<MacFilterSettings, String> {
        Self::execute_jnap(
            ip,
            "http://linksys.com/jnap/macfilter/GetMACFilterSettings",
            "{}",
            Some(password),
        )
        .await
    }

    /// Applies new MAC address filter settings on router
    pub async fn set_mac_filter_settings(
        ip: &str,
        password: &str,
        settings: &MacFilterSettings,
    ) -> Result<(), String> {
        let payload = serde_json::to_string(settings)
            .map_err(|e| format!("Error serializing MacFilterSettings: {}", e))?;

        Self::execute_jnap_action(
            ip,
            "http://linksys.com/jnap/macfilter/SetMACFilterSettings",
            &payload,
            Some(password),
        )
        .await
    }

    /// Blocks a MAC address on router using permanent deny list
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

        current.mac_filter_mode = "Deny".to_string();
        Self::set_mac_filter_settings(ip, password, &current).await
    }

    /// Unblocks / readmits a MAC address on router
    pub async fn unblock_mac(ip: &str, password: &str, mac_to_unblock: &str) -> Result<(), String> {
        let mut current = Self::get_mac_filter_settings(ip, password).await?;
        let mac_norm = mac_to_unblock.trim();

        current
            .mac_addresses
            .retain(|m| !m.trim().eq_ignore_ascii_case(mac_norm));

        if current.mac_addresses.is_empty() {
            current.mac_filter_mode = "Disabled".to_string();
        } else {
            current.mac_filter_mode = "Deny".to_string();
        }

        Self::set_mac_filter_settings(ip, password, &current).await
    }

    /// Retrieves active physical and Wi-Fi connections directly from Linksys router
    pub async fn get_network_connections(
        ip: &str,
        password: &str,
    ) -> Result<Vec<RouterConnection>, String> {
        let out: GetNetworkConnectionsOutput = Self::execute_jnap(
            ip,
            "http://linksys.com/jnap/networkconnections/GetNetworkConnections",
            "{}",
            Some(password),
        )
        .await?;

        Ok(out.connections)
    }
}
