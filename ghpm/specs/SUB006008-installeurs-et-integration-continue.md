---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fTg
rev: 2026-10-10T20:08:45Z
url: https://github.com/georgesp/Educator/issues/68
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006008 — Installeurs
status: Todo
size: L
estimate:
startDate:
endDate:
parent: "#69"
order: 810
---

## User story

En tant que parent, je veux installer l'application comme n'importe quel logiciel afin de ne rien avoir à configurer.

## Comportement attendu

- Application Tauri compilée pour `aarch64-apple-darwin`, `x86_64-apple-darwin` et `x86_64-pc-windows-msvc`, avec le contenu (déjà en ressource Tauri), Piper et la voix embarqués.
- Installeurs : `.dmg` (macOS), `.msi` ou `.exe` NSIS (Windows).
- Génération des installeurs par GitHub Actions sur tag (le workflow de build et tests à chaque push est livré au lot 0).
- Signature et notarisation : traitées dans le lot packaging, hors de cette fiche.

## Critères d'acceptation

- [ ] Installation et premier lancement réussis sur une machine sans Node ni Rust.
- [ ] Taille de l'installeur documentée (objectif < 150 Mo, voix comprise).
- [ ] Installeurs produits automatiquement sur tag pour les deux plateformes.
