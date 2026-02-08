use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::to_writer_pretty;
use std::fs;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetCatalog {
    root: PathBuf,
    manifest: Vec<AssetEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetEntry {
    pub path: PathBuf,
    pub size_bytes: u64,
}

impl AssetCatalog {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            manifest: Vec::new(),
        }
    }

    pub fn scan(&mut self) -> Result<()> {
        self.manifest.clear();
        if !self.root.exists() {
            info!(root = %self.root.display(), "assets path does not exist");
            return Ok(());
        }
        self.walk_dir(&self.root)?;
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.manifest.len()
    }

    pub fn write_manifest(&self, output: impl AsRef<Path>) -> Result<()> {
        let file = fs::File::create(output.as_ref())?;
        let writer = BufWriter::new(file);
        to_writer_pretty(writer, &self.manifest)?;
        Ok(())
    }

    fn walk_dir(&mut self, root: &Path) -> Result<()> {
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.walk_dir(&path)?;
            } else if let Ok(metadata) = entry.metadata() {
                self.manifest.push(AssetEntry {
                    path,
                    size_bytes: metadata.len(),
                });
            }
        }
        Ok(())
    }
}
