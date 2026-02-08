use anyhow::Result;
use se_assets::AssetCatalog;
use se_engine::{EngineConfig, EngineCore};
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = EngineConfig::default();
    let mut assets = AssetCatalog::new(config.assets_path.clone());
    assets.scan()?;

    let mut engine = EngineCore::new(config);
    engine.bootstrap(assets)?;

    info!("pre-alpha bootstrap complete");
    Ok(())
}
