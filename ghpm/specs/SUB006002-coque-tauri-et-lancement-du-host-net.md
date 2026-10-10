---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fLc
url: https://github.com/georgesp/Educator/issues/62
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006002 — Coque Tauri et lancement du host .NET
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#60"
order: 3
---

## User story

En tant qu'utilisateur, je veux lancer une seule application afin de ne jamais voir le backend ni de terminal.

## Comportement attendu

- Projet `web/src-tauri` repris de GitHubProjectManager (`backend.rs`) et renommé (`educator-shell`, identifiant `com.georgesportier.educator`).
- Tauri lance `Educator.Host` comme processus annexe, lit `{ baseUrl, token }` sur sa sortie standard et le transmet au front.
- Arrêt propre du host à la fermeture de la fenêtre, y compris en cas de crash du front.
- Plugins : `single-instance`, `window-state`.
- Fenêtre par défaut 1280 × 800, minimum 1024 × 700, titre « Educator ».

## Critères d'acceptation

- [ ] Aucun processus host orphelin après fermeture (vérifié sur macOS et Windows).
- [ ] Message d'erreur lisible si le host ne démarre pas dans les 10 s.
- [ ] Aucune logique métier dans le code Rust.
