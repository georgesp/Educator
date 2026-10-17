---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBdqNs
url: https://github.com/georgesp/Educator/issues/338
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB033503 — Placement des anecdotes dans les leçons et en fin de série
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#335"
order: 653
---

## User story

En tant qu'enfant, je veux tomber de temps en temps sur une anecdote au bon moment, sans être coupé dans mon effort.

## Comportement attendu

- **Leçons** : le lecteur (SUB001410) insère au plus une carte par leçon, après une slide d'explication située au milieu de la leçon (jamais avant la première slide d'explication, jamais entre une slide interactive et sa correction, jamais après « Je retiens »). La carte n'est pas une slide : elle ne compte pas dans les 8 à 12 slides du format et ne change pas les fichiers de leçons.
- **Fin de série** : sur l'écran de fin de série d'exercices (SUB000109), une tuile « Le savais-tu ? » à côté du récapitulatif ; elle s'ouvre au clic. Elle n'apparaît que si la série est terminée.
- **Exclusions** : aucune anecdote pendant une série, dans le défi chrono (SUB001415), le mode examen (SUB001414), l'évaluation de niveau (SUB001418) ni la révision espacée en cours (SUB001509).
- **Fréquence** : au plus une anecdote dans une leçon, et en fin de série une fois sur deux au plus, pour garder l'effet de surprise. L'anecdote choisie est liée aux compétences de la leçon ou de la série et, si possible, pas encore vue par le profil.
- Respecte le réglage parent (fiche « Réglage parent »).

## Critères d'acceptation

- [ ] Tests du choix de l'anecdote (compétences, déjà vues, fréquence) dans `educator-core`.
- [ ] Tests Playwright : présence dans une leçon et en fin de série ; absence dans le défi chrono, l'examen et l'évaluation de niveau.
