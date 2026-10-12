---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WY4
rev: 2026-10-10T19:39:15Z
url: https://github.com/georgesp/Educator/issues/26
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001410 — Lecteur de leçons et animations interactives
status: Todo
size: L
estimate:
startDate:
endDate:
parent: "#14"
order: 430
---

## User story

En tant qu'enfant, je veux une leçon courte où je manipule les objets afin de comprendre plutôt que de regarder.

## Comportement attendu

- Écran Leçons : choix de la matière, puis liste des leçons de la classe (période en cours mise en avant).
- Lecteur : une slide à la fois, texte lu automatiquement, boutons Précédent / Suivant / Réécouter.
- Bibliothèque d'animations interactives (React + Framer Motion + dnd-kit), paramétrées par la leçon :
  - `numberLine` : droite numérique avec bonds ;
  - `groupObjects` : objets à regrouper par 10 ;
  - `tensUnits` : barres de dizaines et cubes d'unités ;
  - `letterTiles` : lettres à glisser pour former un mot ;
  - `soundSort` : mots à trier par son.
- Fin de leçon → quiz (fiche suivante) → lien vers les exercices de la compétence.

## Critères d'acceptation

- [ ] Les 5 animations fonctionnent à la souris et sont testées (Vitest).
- [ ] Une leçon dure 3 à 5 minutes.
- [ ] Reprise possible d'une leçon interrompue à la slide où l'enfant s'était arrêté.
