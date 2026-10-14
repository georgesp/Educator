---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WkA
rev: 2026-10-10T19:39:24Z
url: https://github.com/georgesp/Educator/issues/34
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001501 — Niveaux du profil
status: Todo
size: S
estimate:
startDate:
endDate:
parent: "#15"
order: 611
---

## User story

En tant qu'enfant, je veux monter de niveau en gagnant des points afin de voir que je progresse.

## Comportement attendu

- Niveau calculé à partir des points cumulés, barème progressif défini dans `content/rewards/levels.json` (ex. niveau 2 à 100 points, niveau 3 à 250…).
- Jauge de progression vers le niveau suivant dans le menu.
- Animation de montée de niveau, avec annonce des nouveautés débloquées (accessoires).

## Critères d'acceptation

- [ ] Barème en fichier de contenu.
- [ ] Niveau recalculé correctement pour les profils existants à la migration.
