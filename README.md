# Educator

[![CI](https://github.com/georgesp/Educator/actions/workflows/ci.yml/badge.svg)](https://github.com/georgesp/Educator/actions/workflows/ci.yml)

Application de bureau gratuite pour apprendre les **mathématiques** et l'**orthographe** du **CP au CM2**,
sur macOS et Windows, entièrement hors ligne.

> Projet en construction : lot 0 (socle technique) terminé côté code, en validation.

## Prérequis

- .NET SDK 10
- Node.js 22 ou plus récent et npm
- Rust (stable) et les [prérequis Tauri 2](https://tauri.app/start/prerequisites/) de la plateforme
- macOS sur Apple Silicon : Rosetta 2 pour la voix (`softwareupdate --install-rosetta --agree-to-license`)

## Démarrer

```bash
scripts/fetch-piper.sh        # Windows : .\scripts\fetch-piper.ps1 — moteur de voix + voix française
cd web
npm install
npm run tauri dev             # lance la coque, le host .NET et le front
```

## Commandes

```bash
dotnet build
dotnet test                              # tous les tests .NET
dotnet test --filter "Category=Unit"     # tests unitaires seulement

cd web
npm run build                            # vérification TypeScript + build Vite
npm test                                 # Vitest
npm run test:e2e                         # Playwright (première fois : npx playwright install chromium)
npm run api:generate                     # régénère le client TS après une modification de l'API
```

## Organisation

| Dossier | Contenu |
|---|---|
| `src/` | Solution .NET : `Educator.Core`, `Educator.Data`, `Educator.Content`, `Educator.Host` |
| `tests/` | Un projet de tests par projet de `src/` |
| `web/` | Front React + TypeScript et coque Tauri (`web/src-tauri/`), voir [`web/README.md`](web/README.md) |
| `scripts/` | Installation de Piper, comparaison des voix |
| `docs/adr/` | Décisions d'architecture |
| `ghpm/` | Backlog en fiches, synchronisé avec le projet GitHub par GitHubProjectManager |

- Architecture : [`docs/adr/0001-architecture.md`](docs/adr/0001-architecture.md)
- Voix : [`docs/adr/0002-voix-piper.md`](docs/adr/0002-voix-piper.md)
- Conventions et règles de travail : [`CLAUDE.md`](CLAUDE.md)
- Licences des composants tiers : [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md)
