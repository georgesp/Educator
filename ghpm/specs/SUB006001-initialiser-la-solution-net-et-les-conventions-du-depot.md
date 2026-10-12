---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fKg
rev: 2026-10-11T15:22:40Z
url: https://github.com/georgesp/Educator/issues/61
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006001 — Initialiser le workspace Cargo et les conventions du dépôt
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#60"
order: 20
---

## User story

En tant que développeur, je veux un dépôt structuré et outillé dès le départ afin que chaque lot s'appuie sur des fondations propres.

## Contenu

- Workspace Cargo à la racine : `crates/educator-core`, `crates/educator-content`, `crates/educator-data` et l'application `web/src-tauri`.
- Tests dans `tests/` de chaque crate (et tests unitaires des commandes dans `web/src-tauri`).
- Lints partagés dans `[workspace.lints]` (`unsafe` interdit, clippy), `cargo fmt`, warnings en erreurs en CI ; versions des dépendances dans `[workspace.dependencies]`.
- `.editorconfig`, `.gitignore`, `README.md`, `CLAUDE.md` / `AGENTS.md` (code en anglais, documentation en français).
- ADR d'architecture : `docs/adr/0003-backend-rust.md` (remplace `0001-architecture.md`).

## Critères d'acceptation

- [x] `cargo clippy --workspace --all-targets -- -D warnings` et `cargo test --workspace` passent sans avertissement.
- [x] Règle de dépendance vérifiée par un test d'architecture : `educator-core` ne dépend d'aucune crate d'infrastructure.
- [x] ADR d'architecture rédigée (0001, puis 0003).

## Implémentation

Workspace Cargo (`Cargo.toml`, `Cargo.lock` à la racine), édition 2021, Rust 1.85 minimum. Test d'architecture
`crates/educator-core/tests/architecture.rs` sur `cargo metadata` : liste blanche des dépendances du `core`, et
`data` / `content` dépendent du `core` sans dépendre l'un de l'autre. Vérifié dans l'environnement Linux de Claude.

*Première version en .NET (2026-10-10), portée en Rust au lot 1bis (2026-10-11, ADR 0003).*
