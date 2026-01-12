use crate::PackageManager;
use anyhow::Result;
use async_trait::async_trait;
use tokio::process::Command;

pub struct Winget;

#[async_trait]
impl PackageManager for Winget {
    fn id(&self) -> &str {
        "winget"
    }

    fn name(&self) -> &str {
        "Winget (Windows Pacmage Manager)"
    }

    async fn is_available(&self) -> bool {
        match Command::new("winget").arg("--version").output().await {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    async fn install(&self, package_id: &str) -> Result<()> {
        println!("Installing {} via Winget...", package_id);

        // Winget needs explicit flags to be fully automated:
        // -e: Exact match (prevents installing 'Firefox Beta' when you want 'Firefox')
        // --id: Search by ID, not name
        // --silent: No popups
        // --accept-*-agreements: Bypass legal prompts
        let status = Command::new("winget")
            .arg("install")
            .arg("-e")
            .arg("--id")
            .arg(package_id)
            .arg("--silent")
            .arg("--accept-package-agreements")
            .arg("--accept-source-agreements")
            .status()
            .await?;

        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Winget failed to install {}", package_id))
        }
    }
}
