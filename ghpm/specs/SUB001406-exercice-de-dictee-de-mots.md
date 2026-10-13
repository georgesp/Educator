---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WSc
rev: 2026-10-10T19:39:10Z
url: https://github.com/georgesp/Educator/issues/22
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001406 — Exercice de dictée de mots
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#14"
order: 390
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

- [x] Clavier à l'écran utilisable à la souris seule.
- [x] Test de parcours (Playwright) avec graine fixe.

## Implémentation (2026-10-14)

- Format `dictation` des compétences d'orthographe : 10 mots sans doublon, graine enregistrée dans la séance.
- Le mot est lu à l'affichage (seul puis dans sa phrase), jamais envoyé au front avant la réponse ; boutons
  « Réécouter le mot » et « Le mot dans une phrase ».
- `LetterKeyboard` : clavier à l'écran avec une rangée de lettres accentuées (é è ê à â ù û ç ï î ô œ…).
- Mot faux : le mot de l'enfant avec les lettres fausses surlignées, puis le mot juste, lu à voix haute.
- Points selon le barème du lot 1. Parcours Playwright `web/e2e/lot2.spec.ts` (graine fixe, souris seule).
