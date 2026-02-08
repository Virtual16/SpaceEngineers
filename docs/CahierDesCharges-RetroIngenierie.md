# Cahier des charges (rétro‑ingénierie) — Space Engineers (version source archivées)

> Document produit à partir du code source et des fichiers de configuration du dépôt.

## 1. Contexte et objectif du produit
- **Jeu vidéo de construction et simulation spatiale** nommé *Space Engineers*, avec un client de jeu et un serveur dédié. Le dépôt fournit le **code source**; les **assets** (audio, modèles, textures) ne sont pas inclus et le jeu complet nécessite l’installation Steam. 【F:README.md†L1-L19】
- Le dépôt représente une **version archivée** (non à jour) du jeu. 【F:README.md†L1-L3】

## 2. Périmètre fonctionnel (haut niveau)
### 2.1 Applications livrées
1. **Client de jeu Space Engineers** (exécutable principal).
   - Point d’entrée : `SpaceEngineers` (MyProgram.Main). 【F:Sources/SpaceEngineers/MyProgram.cs†L1-L126】
2. **Serveur dédié Space Engineers** (exécutable serveur + service Windows).
   - Point d’entrée : `SpaceEngineersDedicated` (MyProgram.Main). 【F:Sources/SpaceEngineers.Dedicated/MyProgram.cs†L1-L58】
   - Possibilité d’installation comme **service Windows**. 【F:Sources/SpaceEngineers.Dedicated/MyProgram.cs†L60-L133】

### 2.2 Fonctionnalités activées par la configuration du jeu
Ces fonctionnalités sont déclarées/activées dans la configuration interne du jeu :
- **Scénarios**, **recherche**, **IA**, **pathfinding**, **jump drive**, **voice chat**, **système de son des vaisseaux**, **gravitations globales** désactivées, etc. 【F:Sources/SpaceEngineers.Game/SpaceEngineersGame.cs†L44-L174】
- Rendu **DirectX 11** requis côté client. 【F:Sources/SpaceEngineers/MyProgram.cs†L96-L125】

## 3. Parties prenantes et utilisateurs cibles
- **Joueurs** (client) : lancement du jeu, création/chargement de mondes, sessions solo/multijoueur (impliqué par l’architecture client/serveur et l’intégration Steam). 【F:Sources/SpaceEngineers/MyProgram.cs†L67-L108】
- **Administrateurs de serveur** : configuration et exploitation d’un serveur dédié, y compris mode service Windows. 【F:Sources/SpaceEngineers.Dedicated/MyProgram.cs†L25-L133】

## 4. Architecture logique (référentiel de projets)
Le dépôt est structuré en plusieurs **projets Visual Studio**. Les principaux modules sont :
- **SpaceEngineers** (client), **SpaceEngineers.Game** (spécifique jeu), **SpaceEngineers.Dedicated**, **SpaceEngineers.ObjectBuilders**. 【F:SpaceEngineers.sln†L5-L44】
- **Sandbox.\*** (mécanismes communs jeu/simulation/graphique). 【F:SpaceEngineers.sln†L12-L44】
- **VRage.\*** (moteur/infra : rendu, audio, input, réseau, math, etc.). 【F:SpaceEngineers.sln†L17-L44】

## 5. Flux d’exécution (client)
1. Initialisation des informations de base du jeu (`SetupBasicGameInfo`). 【F:Sources/SpaceEngineers/MyProgram.cs†L55-L74】
2. Démarrage avec gestion **Steam** (AppId 244850), initialisation du rendu, vérifications d’environnement (mono‑instance, 64‑bit, rendu compatible). 【F:Sources/SpaceEngineers/MyProgram.cs†L41-L126】
3. Lancement de la boucle principale via `SpaceEngineersGame`. 【F:Sources/SpaceEngineers/MyProgram.cs†L99-L115】

## 6. Flux d’exécution (serveur dédié)
1. Initialisation des informations de base et des paramètres de jeu. 【F:Sources/SpaceEngineers.Dedicated/MyProgram.cs†L25-L47】
2. Paramétrage du **serveur dédié** (nom, description, AppId 244850). 【F:Sources/SpaceEngineers.Dedicated/MyProgram.cs†L33-L52】
3. Lancement du serveur via `DedicatedServer.Run`. 【F:Sources/SpaceEngineers.Dedicated/MyProgram.cs†L49-L58】

## 7. Configuration et dépendances
### 7.1 Dépendances externes
- **Steam** requis pour exécuter le jeu et fournir le contenu (assets). 【F:README.md†L16-L30】
- **Visual Studio 2013** (ou compatible) pour compiler le code. 【F:README.md†L23-L27】

### 7.2 Paramètres de build
- Solution Visual Studio : `SpaceEngineers.sln`, configurations x86/x64 Debug/Release. 【F:SpaceEngineers.sln†L1-L56】

### 7.3 Dépendance Steam (analyse + découplage proposé)
Le code client initialise explicitement **Steam** via `MySteamService` (AppId 244850) lors du lancement. 【F:Sources/SpaceEngineers/MyProgram.cs†L41-L115】  
Cette dépendance est structurante pour : authentification, services réseau et accès au contenu du jeu (assets). 【F:README.md†L9-L30】

**Objectif de découplage (proposition de cahier de charges technique) :**
1. **Isoler l’intégration Steam** derrière une interface dédiée (ex. `IGamePlatformServices`), afin de permettre un remplacement par un mock/local service. 【F:Sources/SpaceEngineers/MyProgram.cs†L41-L115】
2. **Ajouter un mode “offline/dev”** piloté par configuration/flag CLI qui bypass Steam et utilise des services locaux. 【F:Sources/SpaceEngineers/MyProgram.cs†L67-L115】
3. **Permettre un dépôt d’assets alternatif** (ex. chemin local configurable) pour exécuter le jeu sans installation Steam. 【F:README.md†L9-L30】

> Note : cette découpe suppose un chantier d’architecture (abstraction des services Steam) et ne peut pas être obtenue uniquement par configuration.

## 8. Exigences non fonctionnelles
### 8.1 Plateformes
- **Windows** (inféré par VS2013 + service Windows + DirectX 11). 【F:README.md†L23-L27】【F:Sources/SpaceEngineers.Dedicated/MyProgram.cs†L60-L133】【F:Sources/SpaceEngineers/MyProgram.cs†L96-L125】

### 8.2 Performance & compatibilité
- **Rendu DirectX 11** obligatoire côté client. 【F:Sources/SpaceEngineers/MyProgram.cs†L96-L125】
- Support d’exécution **64‑bit** côté client (vérification au lancement). 【F:Sources/SpaceEngineers/MyProgram.cs†L75-L83】

## 9. Contenu et assets
- Le code source ne contient **pas** les assets; ceux‑ci proviennent de l’installation Steam. 【F:README.md†L9-L19】

## 10. Sécurité / conformité
- Licence : **EULA** du jeu, à lire avant usage. 【F:README.md†L9-L13】

## 11. Critères d’acceptation (déduits)
1. Le client démarre, initialise Steam, vérifie DirectX 11 et lance la boucle de jeu sans erreur. 【F:Sources/SpaceEngineers/MyProgram.cs†L67-L125】
2. Le serveur dédié démarre et charge la configuration par défaut. 【F:Sources/SpaceEngineers.Dedicated/MyProgram.cs†L25-L58】
3. Les configurations Visual Studio permettent la compilation en x86/x64. 【F:SpaceEngineers.sln†L1-L56】

## 12. Limites connues
- Version **archivée**, non à jour. 【F:README.md†L1-L3】
- Absence d’assets dans le dépôt (dépendance à Steam). 【F:README.md†L9-L19】
