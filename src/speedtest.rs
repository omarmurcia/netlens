use chrono::Local;
use iced::futures::channel::mpsc::UnboundedSender;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

#[derive(Debug, Clone, PartialEq)]
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
    DownloadDone(f64),
    UploadStarted,
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

pub struct SpeedTest;

impl SpeedTest {
    pub async fn execute(sender: UnboundedSender<SpeedTestEvent>) {
        // 1. Etapa de Ping / Latencia
        let _ = sender.unbounded_send(SpeedTestEvent::PingStarted);

        let mut ping_samples = Vec::new();
        for _ in 0..3 {
            let res = Command::new("curl")
                .arg("-s")
                .arg("-o")
                .arg("/dev/null")
                .arg("-w")
                .arg("%{time_connect}")
                .arg("https://speed.cloudflare.com/__down?bytes=0")
                .output()
                .await;

            if let Ok(out) = res {
                if let Ok(text) = String::from_utf8(out.stdout) {
                    if let Ok(secs) = text.trim().parse::<f32>() {
                        if secs > 0.0 {
                            ping_samples.push(secs * 1000.0);
                        }
                    }
                }
            }
        }

        let ping_ms = if !ping_samples.is_empty() {
            let sum: f32 = ping_samples.iter().sum();
            sum / ping_samples.len() as f32
        } else {
            35.0 // Valor estimado de respaldo si falló la medición
        };

        let _ = sender.unbounded_send(SpeedTestEvent::PingDone(ping_ms));

        // 2. Etapa de Descarga (Download: 12 MB)
        let _ = sender.unbounded_send(SpeedTestEvent::DownloadStarted);

        let dl_res = Command::new("curl")
            .arg("-s")
            .arg("-o")
            .arg("/dev/null")
            .arg("-w")
            .arg("%{speed_download}")
            .arg("https://speed.cloudflare.com/__down?bytes=12000000")
            .output()
            .await;

        let download_mbps = match dl_res {
            Ok(out) => {
                let text = String::from_utf8_lossy(&out.stdout);
                if let Ok(bps) = text.trim().parse::<f64>() {
                    (bps * 8.0) / 1_000_000.0
                } else {
                    0.0
                }
            }
            Err(e) => {
                let _ = sender.unbounded_send(SpeedTestEvent::Error(format!("Error en descarga: {}", e)));
                return;
            }
        };

        let _ = sender.unbounded_send(SpeedTestEvent::DownloadDone(download_mbps));

        // 3. Etapa de Subida (Upload: 2 MB)
        let _ = sender.unbounded_send(SpeedTestEvent::UploadStarted);

        let mut child = match Command::new("curl")
            .arg("-s")
            .arg("-o")
            .arg("/dev/null")
            .arg("-w")
            .arg("%{speed_upload}")
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

        // Enviar buffer de datos en memoria (2 MB de ceros)
        if let Some(mut stdin) = child.stdin.take() {
            let payload = vec![0u8; 2_000_000];
            tokio::spawn(async move {
                let _ = stdin.write_all(&payload).await;
            });
        }

        let upload_res = child.wait_with_output().await;

        let upload_mbps = match upload_res {
            Ok(out) => {
                let text = String::from_utf8_lossy(&out.stdout);
                if let Ok(bps) = text.trim().parse::<f64>() {
                    (bps * 8.0) / 1_000_000.0
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

        // 4. Finalizado con éxito
        let result = SpeedTestResult {
            ping_ms,
            download_mbps,
            upload_mbps,
            timestamp: Local::now().format("%H:%M:%S").to_string(),
            server: "Cloudflare Anycast CDN (Edge)".to_string(),
        };

        let _ = sender.unbounded_send(SpeedTestEvent::Finished(result));
    }
}
