---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsDKY
url: https://github.com/georgesp/Educator/issues/241
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB023802 — Projets iOS et Android Tauri et compilation du cœur Rust
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#238"
order: 3020
---

## User story

En tant que Georges, je veux lancer l'application dans un simulateur ou un émulateur, puis sur ma tablette, avec `npm run tauri ios dev` / `npm run tauri android dev`.

## Dépend de

Fiche 01 (plateformes retenues).

## Comportement attendu

- Initialisation des projets mobiles Tauri (`tauri ios init`, `tauri android init`) ; décider quels fichiers générés (`web/src-tauri/gen/`) sont commités et lesquels sont ignorés.
- `tauri.conf.json` : identifiant, version, orientations autorisées, configuration mobile distincte de la fenêtre bureau (les tailles `minWidth` / `minHeight` ne s'appliquent pas).
- Les trois crates (`educator-core`, `educator-content`, `educator-data`) compilent pour les cibles mobiles ; la compilation conditionnelle de Piper (`educator-content::speech`, absent sur iOS / Android) est vérifiée.
- Capacités (`capabilities/default.json`) : vérifier les permissions nécessaires sur mobile.
- Scripts documentés dans `CLAUDE.md` et `docs/tablette.md` (commandes de lancement et ce qu'elles doivent produire).

## Critères d'acceptation

- [ ] L'application démarre dans le simulateur iPad et/ou l'émulateur Android et affiche l'écran de choix du profil.
- [ ] Même chose sur la tablette physique.
- [ ] `cargo test --workspace` reste vert sur macOS.
- [ ] La barre d'état indique que la voix est indisponible (en attendant les fiches voix), sans erreur.
