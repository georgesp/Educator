---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAAv4s
rev: 2026-10-11T15:22:59Z
url: https://github.com/georgesp/Educator/issues/83
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB007206 — Retirer le host .NET et mettre à jour CI et ADR
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#72"
order: 180
---

## User story

En tant que développeur, je veux un dépôt sans code .NET afin de n'avoir qu'une seule chaîne de build à maintenir.

## Comportement attendu

- Suppression de `src/`, `tests/`, `Educator.sln`, `Directory.Build.props`, `Directory.Packages.props`, `global.json`, `.config/dotnet-tools.json`, `web/src-tauri/src/backend.rs`, `web/openapi/` et `web/scripts/generate-api.mjs`.
- Coque Tauri : plus de lancement de `educator-host` (ni `dotnet run` en développement, ni binaire publié).
- CI : le job `.NET` est remplacé par un job `cargo fmt --check`, `cargo clippy -D warnings` et `cargo test` sur macOS et Windows.
- Nouvelle ADR `docs/adr/0003-backend-rust.md` (contexte tablette, décision, décision sur la base existante) ; l'ADR 0001 passe au statut « remplacée par 0003 ».
- `README.md` : prérequis et commandes à jour (plus de .NET).

## Critères d'acceptation

- [x] Le dépôt se compile et se teste sur une machine sans .NET.
- [x] Pipeline CI vert sur macOS et Windows.

## Notes de réalisation

Fait :
- Coque Tauri : plus de lancement de `educator-host` (`lib.rs` ne déclare plus le module `backend`).
- CI (`.github/workflows/ci.yml`) : job Rust (`cargo fmt --check`, `clippy -D warnings`, `cargo test --workspace`,
  `types.ts` à jour) sur macOS et Windows ; front et Playwright inchangés. Pas encore lancée sur GitHub.
- ADR `docs/adr/0003-backend-rust.md` ; ADR 0001 au statut « remplacée par 0003 ».
- `README.md`, `web/README.md`, `CLAUDE.md`, `THIRD-PARTY-NOTICES.md`, `.gitignore` (`/target/`) à jour.

- Supprimés (2026-10-11, avec l'accord de Georges) : `src/`, `tests/`, `Educator.sln`, `Directory.Build.props`,
  `Directory.Packages.props`, `global.json`, `.config/dotnet-tools.json`, `web/src-tauri/src/backend.rs`,
  `web/src-tauri/Cargo.lock` (remplacé par le `Cargo.lock` racine), `web/openapi/`, `web/scripts/generate-api.mjs`,
  `web/src/api/schema.d.ts`, `web/.env.example`. Sections .NET retirées de `.gitignore` et `.editorconfig`,
  `docs/specs/regles-de-generation.md` mis à jour.

CI verte sur le commit `2c6398d` (« Migration Rust », branche `feature/refonte_rust`) : Rust et front sur macOS et Windows, Playwright sur Linux.
