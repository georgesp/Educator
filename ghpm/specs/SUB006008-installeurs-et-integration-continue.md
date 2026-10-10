---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fTg
url: https://github.com/georgesp/Educator/issues/68
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006008 — Installeurs et intégration continue
status: Todo
size: L
estimate:
startDate:
endDate:
parent: "#60"
order: 9
---

## User story

En tant que parent, je veux installer l'application comme n'importe quel logiciel afin de ne rien avoir à configurer.

## Comportement attendu

- Host publié en self-contained (`osx-arm64`, `osx-x64`, `win-x64`) et embarqué comme ressource Tauri, avec Piper et la voix.
- Installeurs : `.dmg` (macOS), `.msi` ou `.exe` NSIS (Windows).
- Signature macOS (et notarisation si un compte développeur Apple est disponible) pour éviter les alertes Gatekeeper.
- GitHub Actions : build + tests .NET et front sur macOS et Windows à chaque push ; génération des installeurs sur tag.

## Critères d'acceptation

- [ ] Installation et premier lancement réussis sur une machine sans .NET, Node ni Rust.
- [ ] Taille de l'installeur documentée (objectif < 150 Mo, voix comprise).
- [ ] Pipeline CI vert sur les deux plateformes.
