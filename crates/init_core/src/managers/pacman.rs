use crate::PackageManager;
use anyhow::Result;
use async_trait::async_trait;
use tokio::process::Command;

// A unit struct - it holds no data, just behavior
pub struct Pacman;

#[async_trait]
impl PackageManager for Pacman {
    fn id(&self) -> &str {
        "pacman"
    }

    fn name(&self) -> &str {
        "Pacman"
    }

    async fn is_available(&self) -> bool {
        match Command::new("pacman").arg("--version").output().await {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    async fn install(&self, package_id: &str) -> Result<()> {
        println!("Installing {} via Pacman...", package_id);

        let status = Command::new("pacman")
            .arg("-S")
            .arg("--noconfirm")
            .arg("--needed")
            .arg(package_id)
            .status()
            .await?;

        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Pacman failed to install {}", package_id))
        }
    }
}
