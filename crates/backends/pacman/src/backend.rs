use async_trait::async_trait;
use lumo_api::PackageBackend;
use lumo_core::Package;
use tokio::process::Command;

pub struct PacmanBackend;

#[async_trait]
impl PackageBackend for PacmanBackend {
    fn name(&self) -> &str {
        "pacman"
    }

    async fn search(&self, _query: &str) -> anyhow::Result<Vec<Package>> {
        let packages: Vec<Package> = Vec::new();

        let status = Command::new("pkexec")
            .args(["pacman", "-Ss"])
            .status()
            .await?;

        // TODO: Process command status to packages

        Ok(packages)
    }

    async fn install(&self, package: &str) -> anyhow::Result<()> {
        let status = Command::new("pkexec")
            .args(["pacman", "-S", "--", package])
            .status()
            .await?;

        anyhow::ensure!(status.success(), "Pacman installation failed");

        Ok(())
    }

    async fn remove(&self, package: &str) -> anyhow::Result<()> {
        let status = Command::new("pkexec")
            .args(["pacman", "-R", "--", package])
            .status()
            .await?;

        anyhow::ensure!(status.success(), "Pacman removal failed");

        Ok(())
    }

    async fn update(&self, _package: &str) -> anyhow::Result<()> {
        anyhow::bail!("Per-package update is not implemented")
    }
}
