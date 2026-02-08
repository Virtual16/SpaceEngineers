use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::thread::sleep;
use std::time::{Duration, Instant};
use tracing::info;

use se_assets::AssetCatalog;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    pub assets_path: String,
    pub manifest_path: String,
    pub compatibility_mode: CompatibilityMode,
    pub loop_config: GameLoopConfig,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            assets_path: "../Sources/SpaceEngineers/Content".to_string(),
            manifest_path: "asset-manifest.json".to_string(),
            compatibility_mode: CompatibilityMode::SteamAssets,
            loop_config: GameLoopConfig::default(),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameLoopConfig {
    pub tick_rate_hz: u32,
    pub max_ticks: u32,
}

impl Default for GameLoopConfig {
    fn default() -> Self {
        Self {
            tick_rate_hz: 60,
            max_ticks: 3,
        }
    }
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

    pub fn run(&self, loop_config: GameLoopConfig) -> Result<()> {
        let tick_duration = Duration::from_millis(1000 / loop_config.tick_rate_hz as u64);
        for tick in 1..=loop_config.max_ticks {
            let start = Instant::now();
            info!(tick, "engine tick");
            let elapsed = start.elapsed();
            if elapsed < tick_duration {
                sleep(tick_duration - elapsed);
            }
        }
        Ok(())
    }
}
