---
type: Issue
title: Exercice de mots à trous au stylet
status: Todo
size: M
parent: specs/lot-tablette-2-stylet-et-capture-de-l-ecriture.md
order: 4090
---

## User story

En tant qu'enfant, je veux écrire à la main les lettres qui manquent dans le mot.

## Dépend de

Fiches 02, 07 et 08 ; lot 2 (SUB001407, mots à trous).

## Comportement attendu

- Le trou (une ou plusieurs lettres) est une petite zone d'écriture placée dans le mot ; reconnaissance d'une lettre ou d'un court groupe de lettres (contexte : le mot à compléter, sans le révéler au front).
- Même confirmation de la lecture que pour la dictée, adaptée aux lettres isolées (plus ambiguës : « a » / « o », « é » / « e »).
- Repli au clavier / aux boutons de lettres à tout moment.
- Les exercices « Choisis la bonne écriture » (QCM) ne changent pas.

## Critères d'acceptation

- [ ] Série de mots à trous complète au stylet sur la tablette de test.
- [ ] Taux de reconnaissance sur les lettres isolées mesuré (fiche 12) ; si insuffisant, l'exercice reste au clavier et la fiche est close avec explication.
