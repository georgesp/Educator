---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAzCFc
rev: 2026-10-15T09:32:38Z
url: https://github.com/georgesp/Educator/issues/269
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB026702 — Écouter le mot plus lentement
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#267"
order: 560
---

## User story

En tant qu'enfant, je veux pouvoir réécouter un mot très lentement pour bien entendre chaque son.

## Comportement attendu

- Dans les exercices d'orthographe et le quiz des leçons d'orthographe, un bouton « Plus lentement » (icône tortue) à côté de « Réécouter le mot ».
- Le mot est lu à la vitesse 0,5 (Piper `--length_scale 2`), sans déformation du son.
- La première écoute lente d'un mot peut prendre un peu plus de temps (synthèse) ; les suivantes viennent du cache.
- Texte du bouton dans `fr.ts`.

## Critères d'acceptation

- [x] Bouton utilisable à la souris et au clavier ; test Vitest.
- [x] Écoute de contrôle par Georges : les mots lus à 0,5 restent naturels (sinon ajuster la vitesse et la noter ici).

## Implémentation (2026-10-15)

- Bouton « Plus lentement » (icône tortue, `TortoiseIcon`) à côté de « Réécouter le mot » dans les séries
  d'orthographe et le quiz des leçons ; texte `fr.spelling.listenSlowly`.
- Paramètre `slow` des commandes `speak_series_word` et `speak_word` : le mot est lu à 0,5 (`speech::speed::SLOW`,
  Piper `--length_scale 2`) par un processus Piper dédié (SUB026708), puis servi par le cache.
- Test Vitest : « « Plus lentement » reads the word very slowly, with the mouse or the keyboard ».
- Écoute de contrôle à 0,5 validée par Georges le 2026-10-15 : la lecture lente reste naturelle, vitesse 0,5
  gardée (`scripts/listen-words.sh --sample 20 --slow` écrit aussi les fichiers `…-lent.wav`).
