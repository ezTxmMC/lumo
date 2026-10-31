use anyhow::Ok;
use async_trait::async_trait;
use lumo_api::PackageBackend;
use lumo_core::Package;
use std::collections::HashSet;
use tokio::process::Command;

pub struct PacmanBackend;

async fn run_pacman(args: &[&str]) -> anyhow::Result<String> {
    let output = Command::new("pacman").args(args).output().await?;

    anyhow::ensure!(
        output.status.success(),
        "pacman {:?} failed with {}",
        args,
        output.status
    );

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

async fn installed_names() -> anyhow::Result<HashSet<String>> {
    let output = Command::new("pacman").arg("-Q").output().await?;

    let names: HashSet<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .map(str::to_string)
        .collect();

    Ok(names)
}

fn parse_packages(stdout: &str, is_installed: impl Fn(&str) -> bool) -> Vec<Package> {
    let mut packages: Vec<Package> = Vec::new();
    let mut lines = stdout.lines();

    while let Some(package_line) = lines.next() {
        let description = lines.next().unwrap_or("").trim();
        let mut parts = package_line.split_whitespace();

        let repository_and_name = parts.next().unwrap_or("");
        let version = parts.next().unwrap_or("");

        let (_, name) = repository_and_name
            .split_once('/')
            .unwrap_or(("", repository_and_name));

        packages.push(Package {
            description: Some(description.to_string()),
            name: name.to_string(),
            version: version.to_string(),
            id: repository_and_name.to_string(),
            source: lumo_core::PackageSource::Pacman,
            installed: is_installed(name),
        });
    }

    packages
}

async fn authenticate_sudo() -> anyhow::Result<()> {
    let status = Command::new("sudo").arg("-v").status().await?;

    anyhow::ensure!(status.success(), "sudo authentication failed");

    Ok(())
}

#[async_trait]
impl PackageBackend for PacmanBackend {
    fn name(&self) -> &str {
        "pacman"
    }

    async fn search(&self, query: &str) -> anyhow::Result<Vec<Package>> {
        let stdout = run_pacman(&["-Ss", query]).await?;
        let installed = installed_names().await?;

        Ok(parse_packages(&stdout, |name| installed.contains(name)))
    }

    async fn is_installed(&self, package: &str) -> anyhow::Result<bool> {
        let output = Command::new("pacman")
            .args(["-Qs", package])
            .output()
            .await
            .unwrap();

        let status_code = output.status.code().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut is_installed = false;
        let mut is_existing = true;

        if stdout == "" {
            is_existing = false;
        }

        if status_code == 0 && is_existing {
            let mut lines = stdout.lines();

            while let Some(_) = lines.next() {
                is_installed = true;
            }
        }

        Ok(is_installed)
    }

    async fn install(&self, package: &str) -> anyhow::Result<()> {
        authenticate_sudo().await?;

        let status = Command::new("sudo")
            .args(["pacman", "-S", "--", package])
            .status()
            .await?;

        anyhow::ensure!(status.success(), "Pacman installation failed");

        Ok(())
    }

    async fn remove(&self, package: &str) -> anyhow::Result<()> {
        authenticate_sudo().await?;

        let status = Command::new("sudo")
            .args(["pacman", "-R", "--", package])
            .status()
            .await?;

        anyhow::ensure!(status.success(), "Pacman removal failed");

        Ok(())
    }

    async fn upgrade(&self, _package: &str) -> anyhow::Result<()> {
        anyhow::bail!("Per-package upgrade is not implemented")
    }

    async fn update_repos(&self) -> anyhow::Result<()> {
        anyhow::bail!("Repos update is not implemented")
    }

    async fn upgrade_all(&self) -> anyhow::Result<()> {
        anyhow::bail!("All-packages upgrade is not implemented")
    }

    async fn list(&self) -> anyhow::Result<Vec<Package>> {
        let stdout = run_pacman(&["-Qs"]).await?;

        Ok(parse_packages(&stdout, |_| true))
    }
}
