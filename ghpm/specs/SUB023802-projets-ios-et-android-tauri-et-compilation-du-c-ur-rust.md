---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsDKY
rev: 2026-10-15T15:35:14Z
url: https://github.com/georgesp/Educator/issues/241
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB023802 — Projet iOS Tauri et compilation du cœur Rust
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#238"
order: 3020
---

## User story

En tant que Georges, je veux lancer l'application dans le simulateur de l'iPad, puis sur mon iPad, avec `npm run tauri ios dev`.

## Dépend de

Fiche 01 (plateformes retenues).

## Comportement attendu

- Initialisation du projet iOS Tauri (`tauri ios init`) ; décider quels fichiers générés (`web/src-tauri/gen/`, dont `gen/apple`) sont commités et lesquels sont ignorés (`.gitignore` ignore aujourd'hui tout `gen/`).
- `tauri.conf.json` : identifiant, version, orientations autorisées, configuration mobile distincte de la fenêtre bureau (les tailles `minWidth` / `minHeight` ne s'appliquent pas).
- Les trois crates (`educator-core`, `educator-content`, `educator-data`) compilent pour les cibles mobiles ; la compilation conditionnelle de Piper (`educator-content::speech`, absent sur iOS / Android) est vérifiée.
- Capacités (`capabilities/default.json`) : vérifier les permissions nécessaires sur mobile.
- Scripts documentés dans `CLAUDE.md` et `docs/tablette.md` (commandes de lancement et ce qu'elles doivent produire).

## Critères d'acceptation

- [x] L'application démarre dans le simulateur iPad (iPad Air 13 pouces M2) et affiche l'écran de choix du profil.
- [x] Même chose sur la tablette physique.
- [x] `cargo test --workspace` reste vert sur macOS (et `clippy` sans avertissement).
- [x] La barre d'état indique que la voix est indisponible (en attendant les fiches voix), sans erreur.

## Implémentation (2026-10-15)

- `tauri ios init` : projet Xcode `web/src-tauri/gen/apple` commité (`project.yml` fait foi) ; `.gitignore` n'ignore plus que `gen/schemas/` et `gen/apple/assets/` (copie de `content/` faite au build).
- iPad seulement (`TARGETED_DEVICE_FAMILY = 2`) et paysage seul (`project.yml`, `Info.plist`, `project.pbxproj`) ; `bundle.iOS.minimumSystemVersion` = 17.0 dans `tauri.conf.json`.
- Permissions séparées : `capabilities/default.json` (`core:default`, partout) et `capabilities/desktop.json` (`window-state`, `dialog`, bureau seulement).
- Signature : équipe « Personal Team » choisie dans Xcode, identifiant passé par la variable `APPLE_DEVELOPMENT_TEAM` (hors dépôt).
- `libc` bloquée à 0.2.189 dans `Cargo.lock` : la 0.2.190 réserve `mach_task_self` à macOS et casse `num_threads` (via `time`, `tauri-plugin-log`) pour iOS (ADR 0006, risques).
- `CLAUDE.md` : commandes iOS et dossier `gen/apple`.
