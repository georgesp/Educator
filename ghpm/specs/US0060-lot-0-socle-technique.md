---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fJ0
rev: 2026-10-10T20:08:43Z
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

Disposer d'une application vide mais complète de bout en bout : coque Tauri 2, host .NET 10 lancé par Tauri, API locale sécurisée, base SQLite, front React, synthèse vocale Piper, intégration continue. Un premier exercice d'addition généré par le host prouve que toute la chaîne fonctionne.

Les installeurs, la signature Apple et la validation sur Windows sont traités dans le lot packaging (fiche `lot-packaging-00-packaging-et-signature.md`) ; la CI compile et teste déjà sur Windows à chaque push.

## Architecture cible

| Couche | Rôle |
|---|---|
| Coque Tauri (Rust) | Fenêtre, cycle de vie du host, aucune logique métier |
| Front React + TypeScript | Écrans enfant, animations, lecture audio |
| `Educator.Host` | API locale `/api/ui`, injection de dépendances, logs |
| `Educator.Core` | Domaine et moteur d'exercices, code pur |
| `Educator.Data` | EF Core, SQLite, migrations |
| `Educator.Content` | Contenu JSON/SVG, schémas, Piper |

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [ ] `npm run tauri dev` lance l'application sur macOS.
- [ ] Pipeline CI vert sur macOS et Windows (build + tests .NET et front).
