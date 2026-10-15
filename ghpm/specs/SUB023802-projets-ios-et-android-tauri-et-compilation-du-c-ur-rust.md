---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsDKY
url: https://github.com/georgesp/Educator/issues/241
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB023802 — Projet iOS Tauri et compilation du cœur Rust
status: Todo
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

- [ ] L'application démarre dans le simulateur iPad (iPad Air 13 pouces M2) et affiche l'écran de choix du profil.
- [ ] Même chose sur la tablette physique.
- [ ] `cargo test --workspace` reste vert sur macOS.
- [ ] La barre d'état indique que la voix est indisponible (en attendant les fiches voix), sans erreur.
