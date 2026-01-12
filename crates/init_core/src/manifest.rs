use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Manifest {
    pub apps: Vec<App>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct App {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    // Maps "pacman", "winget", "apt" -> "package_id"
    // We use "packages" to match the [apps.packages] in TOML
    pub packages: HashMap<String, String>,
}

pub async fn load_manifest(path: &str) -> Result<Manifest> {
    let connect = tokio::fs::read_to_string(path).await?;
    let manifest: Manifest = toml::from_str(&connect)?;

    Ok(manifest)
}
