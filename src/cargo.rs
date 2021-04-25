use anyhow::Result;
use cargo_toml::Manifest;
use serde::Deserialize;
use std::env::current_dir;

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub commits: CommitsMetadata,
}

#[derive(Debug, Deserialize)]
pub struct CommitsMetadata {
    pub defaults: bool,
    pub r#type: Option<Vec<CommitDeclarationMetadata>>,
}

#[derive(Debug, Deserialize)]
pub struct CommitDeclarationMetadata {
    pub name: String,
    pub desc: String,
}

/// Parses the `Cargo.toml` manifest file.
pub fn parse_manifest() -> Result<Manifest<Metadata>> {
    let path = current_dir()?.join("Cargo.toml");
    let manifest: Manifest<Metadata> = Manifest::from_path_with_metadata(path)?;

    Ok(manifest)
}
