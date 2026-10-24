use async_trait::async_trait;
use lumo_core::Package;

#[async_trait]
pub trait PackageBackend: Send + Sync {
    fn name(&self) -> &str;

    async fn search(&self, query: &str) -> anyhow::Result<Vec<Package>>;

    async fn install(&self, package: &str) -> anyhow::Result<()>;

    async fn remove(&self, package: &str) -> anyhow::Result<()>;

    async fn update(&self, package: &str) -> anyhow::Result<()>;
}
