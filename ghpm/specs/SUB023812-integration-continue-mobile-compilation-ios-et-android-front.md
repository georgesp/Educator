---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsEFQ
url: https://github.com/georgesp/Educator/issues/251
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023812 — Intégration continue mobile (compilation iOS et Android, front en viewport tablette)"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#238"
order: 3120
---

## User story

En tant que Georges, je veux être prévenu dès qu'un changement casse la version tablette, sans avoir à la lancer sur la tablette à chaque fois.

## Dépend de

Fiche 02.

## Comportement attendu

- Dans `.github/workflows/ci.yml` (ou un fichier dédié) : `cargo check` / `cargo clippy` des crates pour `aarch64-apple-ios` (runner macOS) et `aarch64-linux-android` (NDK), ne ciblant que ce qui compile sans appareil.
- Option : build complet de l'application Android (APK de debug) en CI ; build iOS sur simulateur, selon le coût des minutes macOS (à noter).
- Playwright : un parcours avec un viewport de tablette (paysage) en plus du bureau.
- La CI existante (macOS, Windows, Linux) reste inchangée.

## Critères d'acceptation

- [ ] Un changement qui casse la compilation mobile fait échouer la CI.
- [ ] Durée de la CI notée ; si elle double, séparer le travail mobile (déclenché sur les modifications de `web/src-tauri`, `crates/` ou du front seulement).
