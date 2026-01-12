use crate::PackageManager;
use anyhow::Result;
use async_trait::async_trait;
use tokio::process::Command;

#[derive(Debug)]
pub struct Paru;

#[async_trait]
impl PackageManager for Paru {
    fn id(&self) -> &str {
        "paru"
    }

    fn name(&self) -> &str {
        "Paru (AUR Helper)"
    }

    async fn is_available(&self) -> bool {
        match Command::new("paru").arg("--version").output().await {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    async fn install(&self, package_id: &str) -> Result<()> {
        println!("Installing {} via Paru...", package_id);

        // Paru arguments:
        // -S: Sync/Install
        // --noconfirm: Auto-yes
        // --needed: Skip if up to date
        let status = Command::new("paru")
            .arg("-S")
            .arg("--noconfirm")
            .arg("--needed")
            .arg(package_id)
            .status()
            .await?;

        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Paru failed to install {}", package_id))
        }
    }

    async fn install_many(&self, package_ids: &[&str]) -> Result<()> {
        if package_ids.is_empty() {
            return Ok(());
        }

        println!("Installing batch via Paru: {:?}", package_ids);

        let status = Command::new("paru")
            .arg("-S")
            .arg("--noconfirm")
            .arg("--needed")
            .args(package_ids)
            .status()
            .await?;

        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Paru failed to install batch"))
        }
    }
}
