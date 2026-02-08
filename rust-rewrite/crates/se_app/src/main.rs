use se_assets::AssetCatalog;
use se_engine::{EngineConfig, EngineCore};

fn main() -> std::io::Result<()> {
    let config = load_config();
    let mut assets = AssetCatalog::new(config.assets_path.clone());
    assets.scan()?;
    assets.write_manifest(&config.manifest_path)?;

    let mut engine = EngineCore::new(config.clone());
    engine.bootstrap(assets)?;
    engine.run(config.loop_config.clone())?;

    println!("pre-alpha bootstrap complete");
    Ok(())
}

fn load_config() -> EngineConfig {
    EngineConfig::from_env()
}
