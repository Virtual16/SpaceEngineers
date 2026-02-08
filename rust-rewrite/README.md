# Space Engineers — Rust Rewrite (Pre‑Alpha)

## Objectif
Squelette Rust pour une réécriture from scratch sans Steam, compatible avec les assets existants.

## Structure
- `crates/se_app` : binaire de démarrage.
- `crates/se_engine` : noyau runtime.
- `crates/se_assets` : catalogage des assets.

## Lancer (local)
```bash
cd rust-rewrite
cargo run -p se_app
```
Pour utiliser un chemin de configuration alternatif :
```bash
cargo run -p se_app -- ./config.example.json
```

## Notes
- Par défaut, le chemin des assets pointe vers `../Sources/SpaceEngineers/Content`.
- Un `config.json` (optionnel) permet de surcharger les chemins et la boucle de jeu (voir `config.example.json`).
- Le binaire démarre, scanne les assets et affiche des logs de bootstrap.
- La boucle de jeu minimale exécute quelques ticks pour valider le runtime.
- Un manifeste JSON est écrit dans `rust-rewrite/asset-manifest.json` (ou `manifest_path` dans la config).
