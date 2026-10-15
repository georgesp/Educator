---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsEFQ
rev: 2026-10-15T13:35:18Z
url: https://github.com/georgesp/Educator/issues/251
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023812 — Intégration continue mobile (compilation iOS, front en viewport tablette)"
status: In Review
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

- Dans `.github/workflows/ci.yml` (ou un fichier dédié) : `cargo check` / `cargo clippy` des crates pour `aarch64-apple-ios` (runner macOS), ne ciblant que ce qui compile sans appareil.
- Option : build iOS pour le simulateur (`aarch64-apple-ios-sim`), selon le coût des minutes macOS (à noter).
- Playwright : un parcours avec un viewport de tablette (paysage) en plus du bureau.
- La CI existante (macOS, Windows, Linux) reste inchangée.

## Critères d'acceptation

- [ ] Un changement qui casse la compilation mobile fait échouer la CI.
- [ ] Durée de la CI notée ; si elle double, séparer le travail mobile (déclenché sur les modifications de `web/src-tauri`, `crates/` ou du front seulement).

## Implémentation (2026-10-15)

- `.github/workflows/ios.yml` : `cargo clippy --workspace --target aarch64-apple-ios -- -D warnings` sur `macos-latest` (plugin Swift compilé par son script de build, sans signature), lancé seulement quand `crates/`, `plugins/`, `web/src-tauri/` ou `Cargo.*` changent (minutes macOS). `ci.yml` inchangé.
- Playwright : projet `ipad-landscape` (`web/playwright.config.ts`) dans la CI existante.
- Pas de build complet pour le simulateur pour l'instant (coût) ; à reconsidérer si la compilation seule ne suffit pas.

## Corrections après relecture (2026-10-15)

- Le workflow `ios.yml` n'a pas encore tourné sur GitHub : à valider à son premier passage (une modification de `crates/`, `plugins/` ou `web/src-tauri/` le déclenche).

## Tests à faire (revue)

1. Pousser une branche qui touche `crates/`, `plugins/` ou `web/src-tauri/` → le workflow « iOS » apparaît dans GitHub Actions et passe ; noter sa durée ici.
2. Vérifier qu'une modification du front seul (ex. `web/src/`) ne lance pas « iOS » (filtre de chemins).
3. Le workflow « CI » existant passe toujours, avec le projet Playwright `ipad-landscape`.
4. Facultatif : casser volontairement la compilation iOS (ex. une erreur dans `ios.rs`) sur une branche → « iOS » échoue.
