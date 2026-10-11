---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5k7s
url: https://github.com/georgesp/Educator/issues/70
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006009 — Intégration continue (build et tests)
status: In progress
size: S
estimate:
startDate:
endDate:
parent: "#60"
order: 9
---

## User story

En tant que développeur, je veux que chaque push soit compilé et testé sur macOS et Windows afin de détecter
au plus tôt une régression ou un problème propre à une plateforme, sans machine Windows à disposition.

## Comportement attendu

- Workflow GitHub Actions `ci.yml` déclenché à chaque push et pull request.
- Matrice `macos-latest` et `windows-latest`.
- .NET : `dotnet restore`, `dotnet build` (warnings en erreurs), `dotnet test` (catégories `Unit` et `Integration`).
- Front : `npm ci`, vérification que le client API généré est à jour, `npm run build`, Vitest.
- Coque Tauri : `cargo check` sur `web/src-tauri` (sans packaging).
- Caches NuGet, npm et Cargo pour garder un temps d'exécution raisonnable.
- Playwright : au moins le parcours d'exemple sur une plateforme.

## Critères d'acceptation

- [ ] Pipeline vert sur macOS et Windows.
- [ ] Un test volontairement cassé fait échouer le pipeline.
- [ ] Badge de statut dans le `README.md`.

## Implémentation (2026-10-10)

Workflow `.github/workflows/ci.yml` écrit (jobs .NET, front + coque, Playwright) mais à copier à la main dans le dépôt : les outils de Claude ne peuvent pas écrire dans `.github/workflows`. Badge ajouté au `README.md`.
Caches : NuGet (`actions/cache`, clé sur `Directory.Packages.props` et les `.csproj`), npm (`setup-node`) et Cargo (`rust-cache`). Chaîne du job vérifiée sur un clone neuf dans l'environnement de Claude.
