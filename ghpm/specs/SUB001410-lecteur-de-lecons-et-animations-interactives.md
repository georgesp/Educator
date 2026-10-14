---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WY4
rev: 2026-10-14T06:19:59Z
url: https://github.com/georgesp/Educator/issues/26
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001410 — Lecteur de leçons et animations interactives
status: Done
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

- [x] Les 5 animations fonctionnent à la souris et sont testées (Vitest).
- [x] Une leçon dure 3 à 5 minutes.
- [x] Reprise possible d'une leçon interrompue à la slide où l'enfant s'était arrêté.

## Implémentation (2026-10-14)

- Leçons → choix de la matière (`SubjectScreen`), puis leçons de la classe, période en cours mise en avant.
- Lecteur : une slide à la fois, texte lu, Précédent / Suivant / Réécouter ; fin → quiz → exercices de la compétence.
- Animations : `numberLine`, `groupObjects`, `tensUnits` (lot 1). `groupObjects` en mode `groupBy` est devenu
  manipulable : l'enfant fait les paquets de 10 en cliquant sur les objets (11 à 39), puis choisit le nombre parmi
  3 propositions (dont l'inversion dizaines / unités, 23 / 32) ; utilisé dans la leçon CP « Ajouter ou enlever des
  dizaines ». `letterTiles` (lettres à glisser sur le mot ou à cliquer pour
  former le mot) et `soundSort` (mots à trier par graphie) ajoutées, testées avec Vitest.
- Reprise : la slide atteinte est enregistrée (`LessonProgress`, commande `save_lesson_progress`) ; à la
  réouverture, « Reprendre » ou « Recommencer ».
