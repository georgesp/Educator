# Educator

[![CI](https://github.com/georgesp/Educator/actions/workflows/ci.yml/badge.svg)](https://github.com/georgesp/Educator/actions/workflows/ci.yml)

Application de bureau gratuite pour apprendre les **mathématiques** et l'**orthographe** du **CP au CM2**,
sur macOS et Windows, entièrement hors ligne.

## Fonctionnalités

- **Mathématiques** du CP au CM2 : exercices **générés en direct** à partir de règles par niveau, leçons illustrées, tables de multiplication (« Mes tables »), mode contre-la-montre.
- **Orthographe** : dictées et règles, sans aucune tolérance sur les accents (prévu au lot 2).
- **Plusieurs profils locaux** par ordinateur (nom + personnage), sans mot de passe, choisis à l'ouverture ; un espace parent affichera les statistiques.
- **Voix** pour les consignes et les dictées : synthèse vocale locale avec [Piper](https://github.com/rhasspy/piper), hors ligne.
- **Aucun compte, aucune donnée envoyée** : tout reste sur l'ordinateur.
- Contenu conforme aux programmes officiels de l'Éducation nationale, retours toujours bienveillants.

## État d'avancement

> Projet en construction, développé lot par lot (backlog dans [`ghpm/`](ghpm/)).

- **Disponible** : profils et personnages, menu, séries d'exercices (clavier et QCM), points, voix, tables de multiplication, leçons et exercices de mathématiques CP et CE1, backend 100 % Rust (lot 1bis).
- **En cours** : exercices et leçons de mathématiques du CE2 au CM2.
- **À venir** : orthographe, examens, motivation et espace parent, puis packaging et signature des applications.

## Stack technique

| Couche | Technologie |
|---|---|
| Application | Tauri 2 (Rust) |
| Cœur | Workspace Cargo : `educator-core`, `educator-content`, `educator-data` |
| Front | React 19 + TypeScript (Vite), TanStack Query, Framer Motion |
| Stockage | SQLite embarqué (`rusqlite`), migrations versionnées |
| Voix | Piper (binaire + voix française `fr_FR-siwis-medium`) |
| Tests | `cargo test`, Vitest, Playwright |

## Prérequis

- Node.js 22 ou plus récent et npm
- Rust stable récent (1.85 ou plus, `rustup update`) et les [prérequis Tauri 2](https://tauri.app/start/prerequisites/) de la plateforme
- macOS sur Apple Silicon : Rosetta 2 pour la voix (`softwareupdate --install-rosetta --agree-to-license`)

## Démarrer

```bash
scripts/fetch-piper.sh        # Windows : .\scripts\fetch-piper.ps1 — moteur de voix + voix française
cd web
npm install
npm run tauri dev             # compile le cœur Rust et lance l'application
```

## Commandes

```bash
cargo test --workspace                   # tous les tests Rust (domaine, contenu, SQLite, commandes)
cargo test -p educator-core              # domaine et moteur d'exercices seulement
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all

cd web
npm run build                            # vérification TypeScript + build Vite
npm test                                 # Vitest
npm run test:e2e                         # Playwright (première fois : npx playwright install chromium)
npm run api:generate                     # régénère src/api/types.ts après une modification des contrats
```

## Organisation

| Dossier | Contenu |
|---|---|
| `crates/` | Cœur Rust : `educator-core` (domaine), `educator-content` (contenu, voix), `educator-data` (SQLite) ; tests dans `tests/` de chaque crate |
| `content/` | Contenu pédagogique (règles d'exercices, personnages, calendrier, points) et schémas JSON |
| `web/` | Front React + TypeScript et application Tauri (`web/src-tauri/`), voir [`web/README.md`](web/README.md) |
| `scripts/` | Installation de Piper, comparaison des voix |
| `docs/adr/` | Décisions d'architecture |
| `docs/specs/` | Règles de génération des exercices et échantillons de contenu |
| `ghpm/` | Backlog en fiches, synchronisé avec le projet GitHub par GitHubProjectManager |

- Architecture : [`docs/adr/0003-backend-rust.md`](docs/adr/0003-backend-rust.md) (remplace l'ADR 0001)
- Voix : [`docs/adr/0002-voix-piper.md`](docs/adr/0002-voix-piper.md)
- Conventions et règles de travail : [`CLAUDE.md`](CLAUDE.md)
- Licences des composants tiers : [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md)

## Données locales

La base `educator.db` et les logs sont stockés dans le dossier de données de l'application :
macOS `~/Library/Application Support/Educator/`, Windows `%APPDATA%\Educator\`.

| Variable | Rôle |
|---|---|
| `EDUCATOR_DATA_DIR` | Dossier de données au lieu du dossier standard (tests, débogage) |
| `EDUCATOR_PIPER_DIR`, `EDUCATOR_PIPER_VOICE` | Emplacement de Piper et voix utilisée |
| `EDUCATOR_CONTENT_DIR` | Contenu pédagogique au lieu de la copie embarquée |

Sans Piper, l'application fonctionne sans le son.

## Contribuer

- Le backlog est suivi dans le [projet GitHub](https://github.com/users/georgesp/projects/7) et en fiches Markdown dans [`ghpm/specs/`](ghpm/specs/).
- La CI ([`ci.yml`](.github/workflows/ci.yml)) lance `cargo fmt --check`, `clippy` et `cargo test` sur macOS et Windows, compile et teste le front, et exécute Playwright sur Linux.
- Avant une contribution, lire [`CLAUDE.md`](CLAUDE.md) (aussi référencé par [`AGENTS.md`](AGENTS.md)) : conventions de code, règles pédagogiques, règles de travail.

## Licence

Aucun fichier `LICENSE` n'est encore publié à la racine. Les composants tiers (Piper, voix, polices, images) sont listés dans [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md).
