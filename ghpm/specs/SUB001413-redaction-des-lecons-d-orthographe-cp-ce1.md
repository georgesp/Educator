---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Wd4
rev: 2026-10-16T19:32:37Z
url: https://github.com/georgesp/Educator/issues/29
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001413 — Rédaction des leçons d'orthographe CP–CE1
status: Done
size: L
estimate:
startDate:
endDate:
parent: "#14"
order: 460
---

## User story

En tant qu'enfant, je veux une leçon sur chaque son et ses différentes écritures afin de savoir comment l'écrire.

## Contenu

Une leçon par compétence d'orthographe CP–CE1 (sons et graphies, m devant m/b/p, g/gu/ge, lettres muettes, pluriel en -s), organisée **par sons et graphies**.

## Méthode

Même méthode que pour les leçons de mathématiques : collecte des sources par Claude, rédaction originale, relecture par Georges.

## Critères d'acceptation

- [ ] Une leçon validée par compétence, sources listées.
- [x] Chaque leçon utilise au moins une animation `letterTiles` ou `soundSort`.

## Implémentation (2026-10-14)

- 22 leçons `content/lessons/spelling/*.json`, une par compétence d'orthographe CP–CE1, organisées par sons et
  graphies, rédaction originale à partir d'eduscol, Canopé et Lumni (sources listées dans chaque fichier).
- Chaque leçon utilise `letterTiles` ou `soundSort` (test `every_spelling_skill_has_its_lesson_with_a_spelling_animation`).
- Reste : relecture et validation par Georges.
