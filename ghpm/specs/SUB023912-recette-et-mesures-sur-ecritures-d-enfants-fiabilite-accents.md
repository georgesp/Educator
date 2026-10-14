---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsFF8
url: https://github.com/georgesp/Educator/issues/265
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023912 — Recette et mesures sur écritures d'enfants (fiabilité, accents, ressenti)"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#239"
order: 4120
---

## User story

En tant que Georges, je veux savoir si le stylet est vraiment utilisable par un enfant avant de le proposer, et où il faut encore progresser.

## Dépend de

Toutes les autres fiches du lot.

## Comportement attendu

- Rejouer le protocole de la fiche 01 avec l'application finale : taux de mots exacts, d'erreurs d'accents, de lectures corrigées par l'enfant (mesure de la fiche 07), temps de reconnaissance.
- Séances d'essai avec un ou plusieurs enfants (avec l'accord des parents) : observations de l'effort, des abandons au profit du clavier, du confort de la zone d'écriture, de la taille du trait.
- Comparaison avec le clavier : durée d'une dictée de 10 mots, nombre d'erreurs « injustes » (reconnaissance) par séance.
- Liste des améliorations et nouvelles fiches (écritures difficiles, mathématiques au stylet, autres langues).

## Critères d'acceptation

- [ ] Rapport de recette rédigé dans `docs/` avec chiffres et décision : activer le stylet par défaut, le laisser en option, ou le désactiver.
- [ ] Seuil de fiabilité de l'ADR 0006 atteint, sinon le stylet reste désactivé par défaut et les raisons sont écrites.
