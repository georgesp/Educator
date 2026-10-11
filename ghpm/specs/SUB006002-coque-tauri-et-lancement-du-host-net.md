---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fLc
rev: 2026-10-11T07:56:42Z
url: https://github.com/georgesp/Educator/issues/62
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006002 — Application Tauri et démarrage du cœur Rust
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#60"
order: 3
---

## User story

En tant qu'utilisateur, je veux lancer une seule application afin de ne jamais voir de terminal ni de serveur.

## Comportement attendu

- Projet `web/src-tauri` (`educator-shell`, identifiant `com.georgesportier.educator`).
- Au lancement, la fenêtre s'ouvre tout de suite ; le contenu et la base sont chargés dans un thread, et le front attend `get_startup` (« Démarrage… »).
- Seul processus annexe : Piper (voix), arrêté avec l'application.
- Plugins : `single-instance`, `window-state`.
- Fenêtre par défaut 1280 × 800, minimum 1024 × 700, titre « Educator ».

## Critères d'acceptation

- [ ] Aucun processus Piper orphelin après fermeture (vérifié sur macOS et Windows).
- [x] Message d'erreur lisible si le démarrage échoue (contenu introuvable, base inaccessible).
- [x] Aucune règle métier dans `web/src-tauri` (adaptateurs fins vers les crates).

## Implémentation

`state.rs` (démarrage en arrière-plan, état `starting` / `ready` / `failed`), `BackendGate` côté front. Plugins
`single-instance` et `window-state` sur le bureau. Vérifié sous Linux (Xvfb) : la fenêtre affiche l'application ; avec
une base inaccessible, l'écran « Oups ! L'application n'a pas pu démarrer. » affiche la cause et un bouton Réessayer.
Piper lit ses consignes sur l'entrée standard : il s'arrête de lui-même quand l'application disparaît. Reste à
vérifier l'absence de processus orphelin sur macOS et Windows.

*Première version en .NET (2026-10-10), portée en Rust au lot 1bis (2026-10-11, ADR 0003).*
