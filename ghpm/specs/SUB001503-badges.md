---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Wls
rev: 2026-10-10T19:39:26Z
url: https://github.com/georgesp/Educator/issues/36
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001503 — Badges
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#15"
order: 540
---

## User story

En tant qu'enfant, je veux gagner des badges sur les notions que je maîtrise afin d'en être fier.

## Comportement attendu

- Badges décrits dans `content/rewards/badges.json` : `id`, `title` (« Champion des compléments à 10 »), `icon` (SVG), `condition` (ex. maîtrise ≥ 90 % sur une compétence, 7 jours d'utilisation, 10 séries sans faute).
- Moteur d'évaluation des conditions en fin de séance.
- Écran **Mes badges** : badges obtenus en couleur, badges à gagner en gris avec leur condition expliquée simplement.

## Critères d'acceptation

- [ ] Une vingtaine de badges initiaux (maths et orthographe, régularité).
- [ ] Tests du moteur de conditions.
