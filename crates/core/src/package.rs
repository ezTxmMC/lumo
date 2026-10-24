use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageSource {
    Pacman,
    Aur,
    Apt,
    Dnf,
    Flatpak,
    Snap,
    Github,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub source: PackageSource,
    pub installed: bool,
}
