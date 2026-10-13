---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Wa8
rev: 2026-10-10T19:39:16Z
url: https://github.com/georgesp/Educator/issues/27
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001411 — Quiz de fin de leçon
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#14"
order: 440
---

## User story

En tant qu'enfant, je veux un petit quiz à la fin de la leçon afin de vérifier que j'ai compris.

## Comportement attendu

- 3 à 5 questions issues de la leçon, même composants que les exercices (QCM ou saisie).
- Explication affichée et lue après chaque réponse.
- Résultat enregistré (session `lesson-quiz`) et points attribués.

## Critères d'acceptation

- [x] Quiz enregistré dans les statistiques.
- [x] Bouton « Revoir la leçon » en cas de résultat < 50 %.

## Implémentation (2026-10-14)

- Le quiz est une séance `lesson-quiz` (commande `start_lesson_quiz`), avec les composants des exercices ;
  explication affichée et lue après chaque réponse ; points selon le barème.
- Sous 50 % de bonnes réponses, le récapitulatif met « Revoir la leçon » en premier (décidé côté Rust :
  `reviewLesson` dans le récapitulatif).
- Orthographe : le mot à écrire est lu, jamais affiché.
