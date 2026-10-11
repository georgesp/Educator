---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5k7s
rev: 2026-10-11T07:56:50Z
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
- Rust : `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
- Front : `npm ci`, `npm run build`, Vitest ; types TypeScript générés vérifiés à jour.
- Caches npm et Cargo pour garder un temps d'exécution raisonnable.
- Playwright : au moins le parcours d'exemple sur une plateforme.

## Critères d'acceptation

- [ ] Pipeline vert sur macOS et Windows.
- [ ] Un test volontairement cassé fait échouer le pipeline.
- [x] Badge de statut dans le `README.md`.

## Implémentation (2026-10-10)

Workflow `.github/workflows/ci.yml` : job Rust (macOS, Windows), job front (macOS, Windows), Playwright (Linux). Badge
dans le `README.md`. Caches : npm (`setup-node`) et Cargo (`rust-cache`). Chaîne vérifiée dans l'environnement de
Claude ; reste à la voir verte sur GitHub.

*Première version en .NET (2026-10-10), portée en Rust au lot 1bis (2026-10-11, ADR 0003).*
