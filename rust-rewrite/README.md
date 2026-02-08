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

## Notes
- Par défaut, le chemin des assets pointe vers `../Sources/SpaceEngineers/Content`.
- Le binaire démarre, scanne les assets et affiche des logs de bootstrap.
