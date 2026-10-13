---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WT4
rev: 2026-10-10T19:39:12Z
url: https://github.com/georgesp/Educator/issues/23
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001407 — Exercice de mots à trous
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#14"
order: 400
---

## User story

En tant qu'enfant, je veux compléter les lettres manquantes d'un mot afin de retenir les graphies difficiles.

## Comportement attendu

- Le mot est affiché avec la ou les graphies travaillées remplacées par des « _ » (ex. `bat___` pour « bateau », compétence [o] → eau).
- Deux modes : saisie des lettres manquantes, ou choix parmi 3 graphies (`o` / `au` / `eau`).
- Le mot peut être écouté.
- Le choix des lettres masquées s'appuie sur `graphemes` (pas de masquage aléatoire de lettres sans intérêt).

## Critères d'acceptation

- [x] Les propositions du mode choix sont des graphies concurrentes du même son (confusions fréquentes pour un son écrit d'une seule façon, décision du 2026-10-14).
- [x] Tests du générateur (masquage, propositions).

## Implémentation (2026-10-14)

- Format `missingLetters` : une graphie travaillée par mot (champ `graphemes`) est masquée ; le mot est affiché
  avec un tiret par lettre manquante (« bat___ ») : dans la case à compléter (saisie) ou à la place des lettres
  (choix). Deux modes : saisie des lettres ou choix parmi 3 graphies. Un seul trou par mot (décision du
  2026-10-14).
- Propositions : graphies concurrentes du même son (`content/spelling/sounds.json`). Pour un son qui n'a qu'une
  graphie au CP (ou, on, oi, ch), il n'en existe pas : les propositions viennent alors des confusions fréquentes
  (o / u pour ou, an / no pour on…), qui sont les erreurs réelles des enfants de CP. Décision de Georges du
  2026-10-14 : on garde ces confusions.
- Les mots outils ne sont pas proposés en mots à trous (pas de graphie travaillée).
- Tests du générateur (`spelling/exercises.rs`, `content_validation.rs`).
