---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Rzc
url: https://github.com/georgesp/Educator/issues/8
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000107 — Moteur de génération des exercices de calcul
status: Todo
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

## API locale

- `POST /api/ui/series` `{ profileId, skillId, answerMode }` → `{ seriesId, seed, exercises: [{ index, prompt, format, choices? }] }` (sans les réponses).
- `POST /api/ui/series/{seriesId}/answers` `{ index, answer }` → `{ correct, expected, explanation }`.

## Critères d'acceptation

- [ ] Génération dans `Educator.Core`, sans dépendance à la base ni à HTTP.
- [ ] Tests de propriété : pour chaque compétence du contenu, 10 000 tirages respectent toutes les contraintes.
- [ ] Tests des distracteurs (pas de doublon, pas de négatif, pas la bonne réponse).
- [ ] Test de reproductibilité par graine.
- [ ] La réponse attendue n'est jamais envoyée au front avant que l'enfant ait répondu.

## Notes techniques

- La série en cours est gardée en mémoire dans le host (série courte) ; l'enregistrement détaillé des séances et réponses arrive au lot 2.
- `explanation` : phrase simple affichée et lue en cas d'erreur, ex. « 7 + 5 = 12 ».
