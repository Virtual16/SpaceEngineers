use std::fs;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct AssetCatalog {
    root: PathBuf,
    manifest: Vec<AssetEntry>,
}

#[derive(Debug, Clone)]
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

    pub fn scan(&mut self) -> std::io::Result<()> {
        self.manifest.clear();
        if !self.root.exists() {
            println!("assets path does not exist: {}", self.root.display());
            return Ok(());
        }
        let root = self.root.clone();
        self.walk_dir(&root)?;
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.manifest.len()
    }

    pub fn write_manifest(&self, output: impl AsRef<Path>) -> std::io::Result<()> {
        let file = fs::File::create(output.as_ref())?;
        let mut writer = BufWriter::new(file);
        use std::io::Write;
        writeln!(writer, "path,size_bytes")?;
        for entry in &self.manifest {
            writeln!(writer, "{},{}", entry.path.display(), entry.size_bytes)?;
        }
        Ok(())
    }

    fn walk_dir(&mut self, root: &Path) -> std::io::Result<()> {
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
