---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WRE
url: https://github.com/georgesp/Educator/issues/21
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001405 — Validation orthographique des réponses
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#14"
order: 250
---

## User story

En tant qu'enfant, je veux voir exactement où je me suis trompé afin de corriger mon orthographe.

## Règles

- Comparaison en Unicode normalisé (NFC), espaces de début et fin ignorés.
- **Accents : aucune tolérance** (« éte » ≠ « été »).
- **Majuscules : tolérées** (« Paris » = « paris ») ; règle paramétrable par compétence pour plus tard.
- Correction lettre à lettre : chaque lettre est marquée juste, fausse, manquante ou en trop (alignement de type distance de Levenshtein).

## Critères d'acceptation

- [ ] Code dans `Educator.Core`, sans dépendance.
- [ ] Tests unitaires : accents manquants, accent erroné (è / é), lettre doublée, lettre manquante, inversion, majuscule.
- [ ] La correction renvoyée au front permet de surligner chaque lettre.
