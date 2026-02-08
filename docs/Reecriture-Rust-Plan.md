# Réécriture Rust — Architecture, cahier des charges, MVP & planning

## 1. Objectif
Réécrire Space Engineers **from scratch** en Rust, en **supprimant Steam** tout en restant **compatible avec les assets existants** (formats et arborescence), et livrer une **pré‑alpha fonctionnelle** avec un périmètre réduit.

## 2. Contraintes clés
- **Compatibilité assets** : lecture des assets existants (Textures, modèles, définitions). Conversion seulement si indispensable, avec pipeline reproductible.
- **Sans Steam** : remplacement par services locaux (auth locale, liste de serveurs, stockage local).
- **Pré‑alpha** : boucle de jeu minimaliste, pas de fonctionnalités haut niveau (scénarios, workshop, etc.).

## 3. Architecture technique Rust (modules, crates, responsabilités)
### 3.1 Workspace & crates
- **se_app** (binaire)
  - Démarrage de l’app, chargement config, initialisation des services, boucle de jeu minimale.
- **se_engine** (lib)
  - Noyau runtime : ECS léger, tick/update, gestion scène, orchestrateur des sous‑systèmes.
- **se_assets** (lib)
  - Catalogage et chargement des assets compatibles (scan, index, mapping types).

### 3.2 Modules internes (se_engine)
- **core** : timing, loop, scheduler.
- **scene** : entités, composants, transforms.
- **physics** (placeholder) : intégration future (Rapier3D) pour rigid bodies.
- **render** (placeholder) : intégration future (wgpu) pour pipeline graphique.
- **gameplay** : règles minimales (spawn, input basique).
- **platform** : services OS (fenêtre, fichiers, audio minimal).
- **net** (optionnel en pré‑alpha) : stub réseau local.

### 3.3 Services hors Steam
- **Auth locale** : profil local (fichier JSON + UUID).
- **Content path** : chemins configurables pour assets (Steam non requis).
- **Server list** : JSON local (pré‑alpha) ou discovery UDP simple.

## 4. Cahier des charges de la réécriture (from scratch)
### 4.1 Fonctionnel (pré‑alpha)
- Démarrer l’application et afficher une scène 3D vide.
- Charger un catalogue d’assets (scan + index).
- Instancier un objet de test (mesh placeholder) pour valider pipeline assets.
- Contrôle caméra simple (WASD + souris).
- Sauvegarde/chargement d’une scène minimale.

### 4.2 Non‑fonctionnel
- Multiplateforme cible (Windows en priorité).
- Observabilité : logs structurés (tracing).
- Performance : 60 fps sur scène vide.

### 4.3 Compatibilité assets
- Conserver les chemins attendus (ex: `Content/Textures`, `Content/Models`).
- Mettre en place un **index d’assets** qui mappe les fichiers existants vers des types Rust.
- Prévoir des convertisseurs (optionnels) vers formats runtime (ex: glTF).

## 5. MVP réaliste
**MVP = pré‑alpha jouable (tech‑demo)**
- Fenêtre + boucle de rendu
- Chargement assets (scan + preview)
- Scène minimale avec un cube/mesh placeholder
- Caméra libre + collision très simple
- Config + logs

## 6. Planning en sprints (6 sprints de 2 semaines)
1. **S1 — Boot & config** : workspace Rust, config, logging, binaire qui démarre.
2. **S2 — Assets** : catalogage, index, mapping formats + tests.
3. **S3 — Rendering minimal** : fenêtre + triangle/cube via wgpu.
4. **S4 — Scene & input** : caméra libre, transforms, input.
5. **S5 — Physique minimale** : collision simple, rigid bodies basiques.
6. **S6 — Stabilisation** : sauvegarde/chargement scène, packaging.

## 7. Pré‑alpha fonctionnelle (définition)
Une pré‑alpha est considérée **fonctionnelle** si :
- Le binaire se lance.
- Les assets sont scannés et listés.
- Une scène minimale est rendue.
- La caméra peut se déplacer.
- Les logs confirment un cycle d’update stable.

## 8. Squelette Rust (repo, crates, structure)
Le squelette est créé dans `rust-rewrite/` avec un workspace Cargo et 3 crates (app, engine, assets).
