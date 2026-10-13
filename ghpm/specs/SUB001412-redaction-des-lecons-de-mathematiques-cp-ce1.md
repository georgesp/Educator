---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Wc8
rev: 2026-10-10T19:39:17Z
url: https://github.com/georgesp/Educator/issues/28
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001412 — Rédaction des leçons de mathématiques CP–CE1
status: In progress
size: L
estimate:
startDate:
endDate:
parent: "#14"
order: 450
---

## User story

En tant qu'enfant, je veux une leçon qui m'explique comment calculer facilement avant de m'entraîner.

## Contenu

Une leçon par compétence du lot 1 (10 leçons), avec astuces :

- compléments à 10 et à 100 ;
- décomposer (27 + 8 = 27 + 3 + 5), passer par la dizaine ;
- doubles et presque-doubles ;
- vérifier une soustraction par l'addition ;
- poser une addition, puis une soustraction, avec et sans retenue.

## Méthode

1. Claude collecte les sources (repères annuels et programmes sur eduscol, Les Fondamentaux de Réseau Canopé, Lumni, documents académiques).
2. Claude rédige une leçon **originale**, reformulée (pas de copie de texte ni de visuel), au format JSON.
3. Georges relit et valide.

## Critères d'acceptation

- [ ] 10 leçons validées, sources listées dans chaque fichier.
- [x] Vocabulaire conforme à celui des programmes (ex. « dizaine », « retenue »).

## Implémentation (2026-10-14)

- Les leçons de mathématiques CP–CE1 ont été rédigées au lot 1 (une par compétence, `content/lessons/math/`,
  sources listées dans chaque fichier). Astuces couvertes :
  - compléments à 10 et à la dizaine (`cp-add-sum-10`, `cp-complement-next-ten`) ; compléments à 100 en passant
    par la dizaine, 65 + 5 + 30 (`ce1-add-sub-tens-hundreds`, slide ajoutée le 2026-10-14) ;
  - décomposer et passer par la dizaine (`cp-add-small-crossing`, `cp-complement-next-ten`) ;
  - doubles et presque-doubles (`cp-add-tables`, `ce1-add-tables`) ;
  - vérifier une soustraction par l'addition (`ce1-sub-2-digits`, `ce1-sub-small`, `ce1-add-sub-3-digits`) ;
  - poser une addition et une soustraction sans retenue (34 + 25 et 68 − 25, slides ajoutées le 2026-10-14), puis
    avec retenue (`ce1-add-2-digits`, `ce1-sub-2-digits`, `ce1-add-sub-3-digits`).
- Reste : validation des leçons par Georges.
