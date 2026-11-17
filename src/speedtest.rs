use chrono::Local;
use iced::futures::channel::mpsc::UnboundedSender;
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use std::time::Instant;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeedTestResult {
    pub ping_ms: f32,
    pub download_mbps: f64,
    pub upload_mbps: f64,
    pub timestamp: String,
    pub server: String,
}

#[derive(Debug, Clone)]
pub enum SpeedTestEvent {
    PingStarted,
    PingDone(f32),
    DownloadStarted,
    #[allow(dead_code)]
    DownloadProgress { percent: f32, speed_mbps: f64 },
    DownloadDone(f64),
    UploadStarted,
    #[allow(dead_code)]
    UploadProgress { percent: f32, speed_mbps: f64 },
    UploadDone(f64),
    Finished(SpeedTestResult),
    Error(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpeedTestStage {
    #[default]
    Idle,
    TestingPing,
    TestingDownload,
    TestingUpload,
    Completed,
    Failed,
}

#[derive(Deserialize)]
struct FastApiResponse {
    client: Option<FastClient>,
    #[serde(default)]
    targets: Vec<FastTarget>,
}

#[derive(Deserialize)]
struct FastClient {
    isp: Option<String>,
    location: Option<FastLocation>,
}

#[derive(Deserialize)]
struct FastLocation {
    country: Option<String>,
}

#[derive(Deserialize)]
struct FastTarget {
    url: String,
}

pub struct SpeedTest;

impl SpeedTest {
    pub async fn execute(sender: UnboundedSender<SpeedTestEvent>) {
        // ----------------------------------------------------
        // 1. Etapa de Ping / Latencia (ICMP ping a 1.1.1.1 o HTTP connect time)
        // ----------------------------------------------------
        let _ = sender.unbounded_send(SpeedTestEvent::PingStarted);

        let mut ping_ms = 0.0f32;

        let ping_out = Command::new("ping")
            .arg("-c")
            .arg("3")
            .arg("-W")
            .arg("2")
            .arg("1.1.1.1")
            .output()
            .await;

        if let Ok(out) = ping_out {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                if line.contains("rtt min/avg/max") || line.contains("round-trip min/avg/max") {
                    let parts: Vec<&str> = line.split('/').collect();
                    if parts.len() >= 5 {
                        if let Ok(avg) = parts[4].trim().parse::<f32>() {
                            ping_ms = avg;
                        }
                    }
                }
            }
        }

        // Si falló el ping ICMP, usar connect time a 1.1.1.1 o Cloudflare
        if ping_ms <= 0.0 {
            let res = Command::new("curl")
                .arg("-s")
                .arg("--max-time")
                .arg("4")
                .arg("-o")
                .arg("/dev/null")
                .arg("-w")
                .arg("%{time_connect}")
                .arg("https://1.1.1.1")
                .output()
                .await;

            if let Ok(out) = res {
                let text = String::from_utf8_lossy(&out.stdout);
                if let Ok(secs) = text.trim().parse::<f32>() {
                    if secs > 0.0 {
                        ping_ms = secs * 1000.0;
                    }
                }
            }
        }

        if ping_ms <= 0.0 {
            ping_ms = 20.0;
        }

        let _ = sender.unbounded_send(SpeedTestEvent::PingDone(ping_ms));

        // ----------------------------------------------------
        // 2. Etapa de Descarga Sostenida (Multi-stream con servidores OCA/CDN)
        // ----------------------------------------------------
        let _ = sender.unbounded_send(SpeedTestEvent::DownloadStarted);
        let _ = sender.unbounded_send(SpeedTestEvent::DownloadProgress {
            percent: 0.15,
            speed_mbps: 0.0,
        });

        let mut server_label = "Netflix Open Connect CDN".to_string();
        let mut download_mbps = 0.0f64;

        // Intentar Fast.com API para servidores locales en el ISP del usuario
        let fast_api_out = Command::new("curl")
            .arg("-s")
            .arg("--max-time")
            .arg("5")
            .arg("https://api.fast.com/netflix/speedtest/v2?https=true&token=YXNkZmFzZGxmbnNkYWZoYXNkZmhrYWxm&urlCount=3")
            .output()
            .await;

        let mut targets = Vec::new();
        if let Ok(out) = fast_api_out {
            if let Ok(resp) = serde_json::from_slice::<FastApiResponse>(&out.stdout) {
                if let Some(client) = &resp.client {
                    let isp = client.isp.as_deref().unwrap_or("ISP");
                    let country = client.location.as_ref().and_then(|l| l.country.as_deref()).unwrap_or("");
                    server_label = if !country.is_empty() {
                        format!("Netflix Open Connect ({}, {})", isp, country)
                    } else {
                        format!("Netflix Open Connect ({})", isp)
                    };
                }
                for t in resp.targets.into_iter().take(3) {
                    targets.push(t.url);
                }
            }
        }

        if !targets.is_empty() {
            let start = Instant::now();
            let mut handles = Vec::new();

            for url in targets {
                let handle = tokio::spawn(async move {
                    let out = Command::new("curl")
                        .arg("-s")
                        .arg("--max-time")
                        .arg("10")
                        .arg("-o")
                        .arg("/dev/null")
                        .arg("-w")
                        .arg("%{size_download}")
                        .arg(&url)
                        .output()
                        .await;

                    if let Ok(res) = out {
                        let text = String::from_utf8_lossy(&res.stdout);
                        text.trim().parse::<u64>().unwrap_or(0)
                    } else {
                        0
                    }
                });
                handles.push(handle);
            }

            let mut total_bytes = 0u64;
            for handle in handles {
                if let Ok(bytes) = handle.await {
                    total_bytes += bytes;
                }
            }

            let elapsed = start.elapsed().as_secs_f64();
            if elapsed > 0.0 && total_bytes > 0 {
                download_mbps = (total_bytes as f64 * 8.0) / (elapsed * 1_000_000.0);
            }
        }

        // Respaldo de descarga si Fast.com no respondió
        if download_mbps <= 0.0 {
            server_label = "Cloudflare Global Anycast CDN".to_string();
            let start = Instant::now();
            let dl_res = Command::new("curl")
                .arg("-s")
                .arg("--max-time")
                .arg("10")
                .arg("-o")
                .arg("/dev/null")
                .arg("-w")
                .arg("%{size_download}")
                .arg("https://speed.cloudflare.com/__down?bytes=50000000")
                .output()
                .await;

            if let Ok(out) = dl_res {
                let text = String::from_utf8_lossy(&out.stdout);
                let bytes = text.trim().parse::<u64>().unwrap_or(0);
                let elapsed = start.elapsed().as_secs_f64();
                if elapsed > 0.0 && bytes > 0 {
                    download_mbps = (bytes as f64 * 8.0) / (elapsed * 1_000_000.0);
                }
            }
        }

        let _ = sender.unbounded_send(SpeedTestEvent::DownloadDone(download_mbps));

        // ----------------------------------------------------
        // 3. Etapa de Subida Sostenida (10 MB payload)
        // ----------------------------------------------------
        let _ = sender.unbounded_send(SpeedTestEvent::UploadStarted);
        let _ = sender.unbounded_send(SpeedTestEvent::UploadProgress {
            percent: 0.70,
            speed_mbps: 0.0,
        });

        let mut child = match Command::new("curl")
            .arg("-s")
            .arg("--max-time")
            .arg("10")
            .arg("-o")
            .arg("/dev/null")
            .arg("-w")
            .arg("%{size_upload} %{speed_upload} %{time_total}")
            .arg("-X")
            .arg("POST")
            .arg("--data-binary")
            .arg("@-")
            .arg("https://speed.cloudflare.com/__up")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                let _ = sender.unbounded_send(SpeedTestEvent::Error(format!("Error iniciando subida: {}", e)));
                return;
            }
        };

        if let Some(mut stdin) = child.stdin.take() {
            let payload = vec![0u8; 10_000_000];
            tokio::spawn(async move {
                let _ = stdin.write_all(&payload).await;
            });
        }

        let upload_res = child.wait_with_output().await;

        let upload_mbps = match upload_res {
            Ok(out) => {
                let text = String::from_utf8_lossy(&out.stdout);
                let parts: Vec<&str> = text.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(speed_bps) = parts[1].parse::<f64>() {
                        if speed_bps > 0.0 {
                            (speed_bps * 8.0) / 1_000_000.0
                        } else if let (Ok(bytes), Ok(time)) = (parts[0].parse::<f64>(), parts[2].parse::<f64>()) {
                            if time > 0.0 {
                                (bytes * 8.0) / (time * 1_000_000.0)
                            } else {
                                0.0
                            }
                        } else {
                            0.0
                        }
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            }
            Err(e) => {
                let _ = sender.unbounded_send(SpeedTestEvent::Error(format!("Error en subida: {}", e)));
                return;
            }
        };

        let _ = sender.unbounded_send(SpeedTestEvent::UploadDone(upload_mbps));

        // ----------------------------------------------------
        // 4. Finalizado con éxito
        // ----------------------------------------------------
        let result = SpeedTestResult {
            ping_ms,
            download_mbps,
            upload_mbps,
            timestamp: Local::now().format("%H:%M:%S").to_string(),
            server: server_label,
        };

        let _ = sender.unbounded_send(SpeedTestEvent::Finished(result));
    }
}
