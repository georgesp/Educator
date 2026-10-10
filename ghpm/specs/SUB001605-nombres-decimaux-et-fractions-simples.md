---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5W4U
url: https://github.com/georgesp/Educator/issues/50
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001605 — Nombres décimaux et fractions simples
status: Todo
size: L
estimate:
startDate:
endDate:
parent: "#16"
order: 650
---

## User story

En tant qu'enfant de CM1 ou CM2, je veux m'entraîner sur les décimaux et les fractions afin de suivre le programme du cycle 3.

## Comportement attendu

- Le moteur gère des nombres décimaux (précision fixe, pas d'erreurs d'arrondi : calcul en `decimal`).
- Exercices : comparer et ranger des décimaux, additionner / soustraire des décimaux, multiplier par 10 / 100 / 1 000, fractions simples (lire, placer sur une droite graduée, fraction d'une quantité).
- Saisie avec la virgule française (« 3,5 ») ; le point est accepté et converti.

## Critères d'acceptation

- [ ] Validation tolérante aux zéros inutiles (« 3,50 » = « 3,5 »).
- [ ] Animation `numberLine` étendue aux décimaux et fractions.
