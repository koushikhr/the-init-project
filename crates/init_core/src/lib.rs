use anyhow::Result;
use async_trait::async_trait;

pub mod detectors;
pub mod managers;
pub mod manifest;

/// The contract that all package managers (Winget, Apt, Dnf) must follow.
#[async_trait]
pub trait PackageManager: Send + Sync + std::fmt::Debug {
    // It returns the key used in apps.toml (e.g., "winget", "paru")
    fn id(&self) -> &str;

    /// Returns the display name of the manager (e.g., "Winget").
    /// This is synchronous because it's just returning a static string.
    fn name(&self) -> &str;

    /// Checks if this package manager is actually installed on the system.
    /// Returns true if the binary exists and is executable.
    /// This is async because it might need to run a shell command to check.
    async fn is_available(&self) -> bool;

    /// Installs a package by its specific ID.
    /// Returns Ok(()) on success, or an Error if it fails.
    async fn install(&self, package_id: &str) -> Result<()>;

    /// Installs multiple packages.
    /// Can be overridden for optimization (e.g. "pacman -S pkg1 pkg2")
    async fn install_many(&self, package_ids: &[&str]) -> Result<()> {
        for id in package_ids {
            self.install(id).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::managers::pacman::Pacman;

    #[tokio::test]
    async fn test_pacman_detection() {
        let managers = Pacman;
        let available = managers.is_available().await;
        println!("Is Pacman available ? {}", available);
        assert!(available, "Pacman should be detected on Arch Linux");
    }
}

#[tokio::test]
async fn test_manifest_loading() {
    // Point to the apps.toml in the root directory
    // Note: When running tests, the path is relative to the crate folder,
    // so we need to go up two levels: ../../apps.toml
    let path = "../../apps.toml";

    let manifest = crate::manifest::load_manifest(path).await;

    match manifest {
        Ok(m) => {
            println!("Loaded {} apps", m.apps.len());
            assert!(m.apps.len() > 0);
            assert_eq!(m.apps[0].id, "firefox");
            // Check if pacman key exists
            assert_eq!(
                m.apps[0].packages.get("pacman"),
                Some(&"firefox".to_string())
            );
        }
        Err(e) => panic!("Failed to load mainfest: {}", e),
    }
}
