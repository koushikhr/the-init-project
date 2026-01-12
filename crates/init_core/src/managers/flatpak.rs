use crate::PackageManager;
use anyhow::Result;
use async_trait::async_trait;
use tokio::process::Command;

#[derive(Debug)]
pub struct Flatpak;

#[async_trait]
impl PackageManager for Flatpak {
    fn id(&self) -> &str {
        "flatpak"
    }

    fn name(&self) -> &str {
        "Flatpak"
    }

    async fn is_available(&self) -> bool {
        match Command::new("flatpak").arg("--version").output().await {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    async fn install(&self, package_id: &str) -> Result<()> {
        println!("Installing {} via Flatpak...", package_id);

        // -y: Assume yes
        // --noninteractive: No prompts
        let status = Command::new("flatpak")
            .arg("install")
            .arg("-y")
            .arg("--noninteractive")
            .arg(package_id)
            .status()
            .await?;

        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Flatpak failed to install {}", package_id))
        }
    }
}
