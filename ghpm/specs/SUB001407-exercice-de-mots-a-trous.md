---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WT4
url: https://github.com/georgesp/Educator/issues/23
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001407 — Exercice de mots à trous
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#14"
order: 270
---

## User story

En tant qu'enfant, je veux compléter les lettres manquantes d'un mot afin de retenir les graphies difficiles.

## Comportement attendu

- Le mot est affiché avec la ou les graphies travaillées remplacées par des « _ » (ex. `bat___` pour « bateau », compétence [o] → eau).
- Deux modes : saisie des lettres manquantes, ou choix parmi 3 graphies (`o` / `au` / `eau`).
- Le mot peut être écouté.
- Le choix des lettres masquées s'appuie sur `graphemes` (pas de masquage aléatoire de lettres sans intérêt).

## Critères d'acceptation

- [ ] Les propositions du mode choix sont des graphies concurrentes du même son.
- [ ] Tests du générateur (masquage, propositions).
