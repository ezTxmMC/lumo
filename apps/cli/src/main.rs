use anyhow::Ok;
use lumo_api::PackageBackend;
use lumo_core::Package;
use lumo_pacman::PacmanBackend;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let backends = [PacmanBackend, SnapBackend];
    let mut results: Vec<Package> = Vec::new();

    for backend in backends {
        let backend_results = backend.list().await?;

        for result in backend_results {
            results.push(result);
        }
    }

    println!("{:?}", results);

    Ok(())
}
