use std::thread::sleep;
use std::time::{Duration, Instant};

use se_assets::AssetCatalog;

#[derive(Debug, Clone)]
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
            manifest_path: "asset-manifest.csv".to_string(),
            compatibility_mode: CompatibilityMode::SteamAssets,
            loop_config: GameLoopConfig::default(),
        }
    }
}

impl EngineConfig {
    pub fn from_env() -> Self {
        let assets_path = std::env::var("SE_ASSETS_PATH")
            .unwrap_or_else(|_| "../Sources/SpaceEngineers/Content".to_string());
        let manifest_path =
            std::env::var("SE_MANIFEST_PATH").unwrap_or_else(|_| "asset-manifest.csv".to_string());
        let tick_rate_hz = std::env::var("SE_TICK_RATE_HZ")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(60)
            .max(1);
        let max_ticks = std::env::var("SE_MAX_TICKS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(3);

        Self {
            assets_path,
            manifest_path,
            compatibility_mode: CompatibilityMode::SteamAssets,
            loop_config: GameLoopConfig {
                tick_rate_hz,
                max_ticks,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum CompatibilityMode {
    SteamAssets,
    LocalPack,
}

pub struct EngineCore {
    config: EngineConfig,
}

#[derive(Debug, Clone)]
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

    pub fn bootstrap(&mut self, catalog: AssetCatalog) -> std::io::Result<()> {
        println!(
            "engine bootstrap: assets_path={}, compatibility={:?}, asset_count={}",
            self.config.assets_path,
            self.config.compatibility_mode,
            catalog.count()
        );
        Ok(())
    }

    pub fn run(&self, loop_config: GameLoopConfig) -> std::io::Result<()> {
        let tick_duration = Duration::from_millis(1000 / loop_config.tick_rate_hz as u64);
        for tick in 1..=loop_config.max_ticks {
            let start = Instant::now();
            println!("engine tick {}", tick);
            let elapsed = start.elapsed();
            if elapsed < tick_duration {
                sleep(tick_duration - elapsed);
            }
        }
        Ok(())
    }
}
