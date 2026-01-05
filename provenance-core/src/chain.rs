use crate::model::Provenance;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvenanceChain {
    pub entries: Vec<Provenance>,
}

impl ProvenanceChain {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn append(&mut self, entry: Provenance) {
        self.entries.push(entry);
    }
}

/// Given a repo root and a file path (relative to the repo root)
/// return the path to the chain file: .provenance/chains/<file>.yaml
pub fn chain_path_for(repo_root: &Path, file_path: &Path) -> PathBuf {
    let mut path = repo_root.to_path_buf();
    path.push(".provenance");
    path.push("chains");
    path.push(file_path);

    let ext = file_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("txt");

    path.set_extension(format!("{ext}.yaml"));
    path
}

pub fn load_chain(path: &Path) -> anyhow::Result<ProvenanceChain> {
    if !path.exists() {
        return Ok(ProvenanceChain::new());
    }

    let text = fs::read_to_string(path)?;
    let entries: Vec<Provenance> = serde_yaml::from_str(&text)?;
    Ok(ProvenanceChain { entries })
}

pub fn save_chain(path: &Path, chain: &ProvenanceChain) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let yaml = serde_yaml::to_string(&chain.entries)?;
    fs::write(path, yaml)?;
    Ok(())
}