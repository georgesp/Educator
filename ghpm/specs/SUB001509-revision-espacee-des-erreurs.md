---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Wts
url: https://github.com/georgesp/Educator/issues/42
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001509 — Révision espacée des erreurs
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#15"
order: 490
---

## User story

En tant qu'enfant, je veux revoir plus tard les calculs et les mots que j'ai ratés afin de ne plus me tromper.

## Comportement attendu

- Chaque erreur alimente une file de révision par profil (exercice ou mot), avec une prochaine date de révision (J+1, J+3, J+7, J+14).
- Série **« Mes révisions »** proposée quand des éléments sont à revoir ; les éléments réussis s'espacent, les éléments ratés reviennent plus tôt.
- Pour les maths, on revoit la même opération ou une opération proche (même paramètres).

## Critères d'acceptation

- [ ] Table de révision + migration.
- [ ] Algorithme testé avec un `TimeProvider` simulé.
