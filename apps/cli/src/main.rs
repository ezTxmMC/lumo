use anyhow::Ok;
use lumo_api::PackageBackend;
use lumo_pacman::PacmanBackend;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let backend = PacmanBackend;
    let results = backend.search("zsh-antidote").await?;

    println!("{:?}", results);

    Ok(())
}
