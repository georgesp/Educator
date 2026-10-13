---
type: Issue
title: Écouter le mot plus lentement
status: Todo
size: S
parent: specs/lot-2bis-voix-en-direct.md
order: 505
---

## User story

En tant qu'enfant, je veux pouvoir réécouter un mot très lentement pour bien entendre chaque son.

## Comportement attendu

- Dans les exercices d'orthographe et le quiz des leçons d'orthographe, un bouton « Plus lentement » (icône tortue) à côté de « Réécouter le mot ».
- Le mot est lu à la vitesse 0,5 (Piper `--length_scale 2`), sans déformation du son.
- La première écoute lente d'un mot peut prendre un peu plus de temps (synthèse) ; les suivantes viennent du cache.
- Texte du bouton dans `fr.ts`.

## Critères d'acceptation

- [ ] Bouton utilisable à la souris et au clavier ; test Vitest.
- [ ] Écoute de contrôle par Georges : les mots lus à 0,5 restent naturels (sinon ajuster la vitesse et la noter ici).
