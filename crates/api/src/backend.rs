use async_trait::async_trait;
use lumo_core::Package;

#[async_trait]
pub trait PackageBackend: Send + Sync {
    fn name(&self) -> &str;

    async fn search(&self, query: &str) -> anyhow::Result<Vec<Package>>;

    async fn is_installed(&self, package: &str) -> anyhow::Result<bool>;

    async fn install(&self, package: &str) -> anyhow::Result<()>;

    async fn remove(&self, package: &str) -> anyhow::Result<()>;

    async fn upgrade(&self, package: &str) -> anyhow::Result<()>;

    async fn update_repos(&self) -> anyhow::Result<()>;

    async fn upgrade_all(&self) -> anyhow::Result<()>;

    async fn list(&self) -> anyhow::Result<Vec<Package>>;
}
