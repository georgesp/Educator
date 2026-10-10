---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WSc
url: https://github.com/georgesp/Educator/issues/22
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001406 — Exercice de dictée de mots
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#14"
order: 260
---

## User story

En tant qu'enfant, je veux écouter un mot et l'écrire afin de m'entraîner à l'orthographe.

## Comportement attendu

- Série de 10 mots tirés de la compétence (sans doublon, graine enregistrée).
- Le mot est lu automatiquement ; bouton pour le réécouter (mot seul ou dans sa phrase).
- Saisie au clavier physique ou au **clavier à l'écran avec lettres accentuées** bien visibles (é, è, ê, à, ù, ç, ï…).
- Après validation : mot correct → animation ; mot faux → le mot de l'enfant est affiché avec les lettres fautives surlignées, puis le mot correct, lu à voix haute.
- Points selon le barème du lot 1.

## Critères d'acceptation

- [ ] Clavier à l'écran utilisable à la souris seule.
- [ ] Test de parcours (Playwright) avec graine fixe.
