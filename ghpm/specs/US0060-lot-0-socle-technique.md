---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fJ0
url: https://github.com/georgesp/Educator/issues/60
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: US0060 — Lot 0 — Socle technique
status: Todo
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

Disposer d'une application vide mais complète de bout en bout, sur macOS et Windows : coque Tauri 2, host .NET 10 lancé par Tauri, API locale sécurisée, base SQLite, front React, synthèse vocale Piper, installeurs. Un premier exercice d'addition généré par le host prouve que toute la chaîne fonctionne.

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
- [ ] `npm run tauri dev` lance l'application sur macOS et Windows.
- [ ] Un installeur par plateforme installe et lance l'application sur une machine sans SDK.
