use anyhow::Result;
use se_assets::AssetCatalog;
use se_engine::{EngineConfig, EngineCore, GameLoopConfig};
use tracing::{info, warn};
use std::fs;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = load_config();
    let mut assets = AssetCatalog::new(config.assets_path.clone());
    assets.scan()?;
    assets.write_manifest(&config.manifest_path)?;

    let mut engine = EngineCore::new(config);
    engine.bootstrap(assets)?;
    engine.run(config.loop_config.clone())?;

    info!("pre-alpha bootstrap complete");
    Ok(())
}

fn load_config() -> EngineConfig {
    let mut args = std::env::args().skip(1);
    let config_path = args.next().unwrap_or_else(|| "config.json".to_string());
    let contents = match fs::read_to_string(&config_path) {
        Ok(contents) => contents,
        Err(error) => {
            warn!(path = %config_path, %error, "config not found, using defaults");
            return EngineConfig::default();
        }
    };

    match serde_json::from_str(&contents) {
        Ok(config) => config,
        Err(error) => {
            warn!(path = %config_path, %error, "invalid config, using defaults");
            EngineConfig::default()
        }
    }
}
