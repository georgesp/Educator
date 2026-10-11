---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fJ0
rev: 2026-10-11T07:56:50Z
url: https://github.com/georgesp/Educator/issues/60
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: US0060 — Lot 0 — Socle technique
status: In progress
size: XL
estimate:
startDate:
endDate:
parent:
order: 1
---

Source : `2026-10-10_notes-application-educative.docx` (cadrage, version 4), sections 2 et 13.

- **Lot** : 0 — Socle technique
- **Référence** : architecture de GitHubProjectManager (ADR 0001 stockage SQLite, ADR 0002 front Tauri), allégée pour Educator.

## Objectif

Disposer d'une application vide mais complète de bout en bout : application Tauri 2 au cœur Rust, commandes Tauri appelées par le front, base SQLite, front React, synthèse vocale Piper, intégration continue. Un premier exercice d'addition généré par le cœur Rust prouve que toute la chaîne fonctionne.

Les installeurs, la signature Apple et la validation sur Windows sont traités dans le lot packaging (fiche `lot-packaging-00-packaging-et-signature.md`) ; la CI compile et teste déjà sur Windows à chaque push.

## Architecture cible

| Couche | Rôle |
|---|---|
| Application Tauri (`web/src-tauri`) | Fenêtre, commandes Tauri, démarrage, logs, aucune règle métier |
| Front React + TypeScript | Écrans enfant, animations, lecture audio |
| `educator-core` | Domaine et moteur d'exercices, code pur |
| `educator-data` | SQLite (`rusqlite`), migrations |
| `educator-content` | Contenu JSON/SVG, schémas, Piper |

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [ ] `npm run tauri dev` lance l'application sur macOS.
- [ ] Pipeline CI vert sur macOS et Windows (build + tests Rust et front).

## Historique

Le lot 0 a d'abord été réalisé avec un host .NET 10 lancé par Tauri et une API HTTP locale (ADR 0001). Le lot 1bis
(`specs/lot-migration-rust.md`, ADR 0003) l'a remplacé par le cœur Rust décrit ci-dessus, en vue d'une version tablette.
