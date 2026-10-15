---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Wk8
rev: 2026-10-10T19:39:25Z
url: https://github.com/georgesp/Educator/issues/35
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001502 — Étoiles par série et par leçon
status: In Review
size: S
estimate:
startDate:
endDate:
parent: "#15"
order: 612
---

## User story

En tant qu'enfant, je veux gagner jusqu'à 3 étoiles par série afin d'avoir envie de refaire pour toutes les obtenir.

## Règles

- 1 étoile : série terminée ; 2 étoiles : ≥ 70 % de réussite ; 3 étoiles : ≥ 90 %.
- Meilleur résultat conservé par profil et par compétence, et par leçon (quiz).

## Critères d'acceptation

- [x] Étoiles affichées dans la liste des compétences et des leçons.
- [x] Seuils en fichier de contenu.

## Réalisation

- Seuils dans `content/rewards/stars.json` (70 % et 90 %), lus par `educator-core::rewards::stars::StarRules`.
- Le meilleur résultat n'est pas stocké à part : il est relu dans les séances enregistrées (`BestStars`), par compétence (séries d'exercices terminées) et par leçon (quiz terminé). Un examen, un défi chrono, une évaluation ou « Mes révisions » ne donnent pas d'étoiles.
- `SkillResponse.stars` et `LessonSummaryResponse.stars` (0 à 3) : étoiles sous chaque carte des écrans Exercices et Leçons, et sur la carte de progression.
- Fin de série et de quiz : les étoiles gagnées, et « Plus d'étoiles que la dernière fois ! » quand le record de la compétence est battu.
