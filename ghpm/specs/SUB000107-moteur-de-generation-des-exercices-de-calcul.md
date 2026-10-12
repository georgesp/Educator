---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Rzc
rev: 2026-10-11T07:56:35Z
url: https://github.com/georgesp/Educator/issues/8
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000107 — Moteur de génération des exercices de calcul
status: Done
size: L
estimate:
startDate:
endDate:
parent: "#1"
order: 80
---

## User story

En tant qu'enfant, je veux des exercices toujours nouveaux et adaptés à ma classe afin de m'entraîner sans retomber sur les mêmes calculs.

## Comportement attendu

- À partir d'une compétence (voir « Format des règles »), le moteur génère une **série** de N exercices (10 par défaut).
- Tirage aléatoire des opérandes dans les bornes, en respectant les contraintes (résultat max, retenue, résultat positif pour la soustraction, exclusion des cas triviaux).
- Format tiré parmi `formats` (ex. 60 % `result`, 40 % `missingOperand`).
- Pas de doublon dans une même série (7 + 5 et 5 + 7 comptent comme doublons).
- Mode QCM : 3 propositions + la bonne réponse, avec des **distracteurs crédibles** :
  - résultat ± 1 ;
  - erreur de retenue (± 10) ;
  - opération inversée (addition au lieu de soustraction) ;
  - jamais négatifs, jamais en double, jamais égaux à la bonne réponse.
- Une **graine** aléatoire est attribuée à chaque série : la même graine + la même compétence redonnent exactement la même série (débogage, tests).

## Commandes Tauri

- `start_series` `{ profileId, skillId, answerMode }` → `{ seriesId, seed, exercises: [{ index, prompt, format, choices? }] }` (sans les réponses).
- `answer_series` `{ seriesId, index, answer }` → `{ correct, expected, explanation }`.

## Critères d'acceptation

- [x] Génération dans `educator-core`, sans dépendance à la base ni à Tauri.
- [x] Tests de propriété : pour chaque compétence du contenu, 10 000 tirages respectent toutes les contraintes.
- [x] Tests des distracteurs (pas de doublon, pas de négatif, pas la bonne réponse).
- [x] Test de reproductibilité par graine.
- [x] La réponse attendue n'est jamais envoyée au front avant que l'enfant ait répondu.

## Notes techniques

- La série en cours est gardée en mémoire dans le cœur (série courte) ; l'enregistrement détaillé des séances et réponses arrive au lot 2.
- `explanation` : phrase simple affichée et lue en cas d'erreur, ex. « 7 + 5 = 12 ».

## Implémentation (2026-10-11)

- `CalculationSpace` énumère une fois tous les calculs permis par une compétence. Le moteur tire dans cette liste : chaque exercice est valide par construction, et le compte des exercices différents est exact.
- `generate_series` : 10 exercices sans doublon (7 + 5 = 5 + 7), 60 % `result` et 40 % `missingOperand`, graine reproductible (ChaCha8).
- Distracteurs du QCM :
  - ±1 ;
  - ±10 (erreur de retenue) ;
  - opération inversée quand on demande le résultat ;
  - quand un nombre manque, le résultat recopié, ou pour `? − b = r` la soustraction au lieu de l'addition.
  - Ce dernier cas a été ajusté après revue : l'opération inversée y donnait des valeurs peu crédibles (68 + ? = 70 → 138).
- `SeriesService` : séries en mémoire (50 au plus), une réponse par exercice, points versés une seule fois en fin de série ou à l'abandon.
- Commandes : `start_series` (paramètre `seed` facultatif pour rejouer une série), `answer_series` et `abandon_series`. Le nombre demandé est mis à `null` dans la réponse.
- Tests : `educator-core/tests/generator.rs` et `series.rs`, `educator-content/tests/content_validation.rs` (10 000 tirages par compétence), tests de `services.rs`.
