# Educator

[![CI](https://github.com/georgesp/Educator/actions/workflows/ci.yml/badge.svg)](https://github.com/georgesp/Educator/actions/workflows/ci.yml)

Application de bureau gratuite pour apprendre les **mathématiques** et l'**orthographe** du **CP au CM2**,
sur macOS et Windows, entièrement hors ligne.

> Projet en construction : lots 0 et 1 terminés côté code ; lot 1bis (backend en Rust) en validation.

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
| `ghpm/` | Backlog en fiches, synchronisé avec le projet GitHub par GitHubProjectManager |

- Architecture : [`docs/adr/0003-backend-rust.md`](docs/adr/0003-backend-rust.md) (remplace l'ADR 0001)
- Voix : [`docs/adr/0002-voix-piper.md`](docs/adr/0002-voix-piper.md)
- Conventions et règles de travail : [`CLAUDE.md`](CLAUDE.md)
- Licences des composants tiers : [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md)
