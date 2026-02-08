use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::info;

use se_assets::AssetCatalog;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    pub assets_path: String,
    pub compatibility_mode: CompatibilityMode,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            assets_path: "../Sources/SpaceEngineers/Content".to_string(),
            compatibility_mode: CompatibilityMode::SteamAssets,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompatibilityMode {
    SteamAssets,
    LocalPack,
}

pub struct EngineCore {
    config: EngineConfig,
}

impl EngineCore {
    pub fn new(config: EngineConfig) -> Self {
        Self { config }
    }

    pub fn bootstrap(&mut self, catalog: AssetCatalog) -> Result<()> {
        info!(
            assets_path = %self.config.assets_path,
            compatibility = ?self.config.compatibility_mode,
            asset_count = catalog.count(),
            "engine bootstrap"
        );
        Ok(())
    }
}
