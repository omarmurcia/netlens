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

pub struct UpdateChecker;

impl UpdateChecker {
    pub const REMOTE_VERSION_URL: &'static str =
        "https://omarmurcia.github.io/netlens-repo/version.json";
    pub const CURRENT_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const CURRENT_PKGREL: u32 = 2;

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
}
