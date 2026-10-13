---
type: Issue
title: Exercice de dictée de mots au stylet
status: Todo
size: L
parent: specs/lot-tablette-2-stylet-et-capture-de-l-ecriture.md
order: 4080
---

## User story

En tant qu'enfant, je veux écrire à la main le mot que j'ai entendu, comme en classe.

## Dépend de

Fiches 02, 07 et 03 ou 04 à 06 ; lot 2 (SUB001406, dictée de mots).

## Comportement attendu

- Dans la dictée (SUB001406), l'enfant peut choisir « Écrire au stylet » ou « Écrire au clavier » à tout moment, dans la même série.
- Le mot est lu (voix de la tablette), l'enfant écrit sur la zone d'écriture (fiche 02) ou dans la case (fiche 03), confirme la lecture (fiche 07), puis la validation orthographique habituelle s'applique (accents stricts).
- Le mot attendu n'est jamais envoyé au front avant la réponse (inchangé).
- Le bouton « Réécouter le mot » (et « Plus lentement » du lot 2bis) reste accessible pendant l'écriture ; la zone se vide au mot suivant.
- Examen et défi chrono : le temps passé à confirmer la lecture est compté à part ou neutralisé (à trancher dans la fiche, ne pas pénaliser l'enfant).

## Critères d'acceptation

- [ ] Série de dictée complète au stylet sur la tablette de test, résultats et points identiques à ceux du clavier.
- [ ] Bascule stylet / clavier en cours de série sans perdre la progression.
- [ ] Tests Vitest et Playwright (tracés simulés) ; le comportement au clavier est inchangé.
