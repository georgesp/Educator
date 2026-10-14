---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WRE
rev: 2026-10-14T06:19:52Z
url: https://github.com/georgesp/Educator/issues/21
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001405 — Validation orthographique des réponses
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#14"
order: 380
---

## User story

En tant qu'enfant, je veux voir exactement où je me suis trompé afin de corriger mon orthographe.

## Règles

- Comparaison en Unicode normalisé (NFC), espaces de début et fin ignorés.
- **Accents : aucune tolérance** (« éte » ≠ « été »).
- **Majuscules : tolérées** (« Paris » = « paris ») ; règle paramétrable par compétence pour plus tard.
- Correction lettre à lettre : chaque lettre est marquée juste, fausse, manquante ou en trop (alignement de type distance de Levenshtein).

## Critères d'acceptation

- [x] Code dans la crate `educator-core`, sans dépendance d'infrastructure.
- [x] Tests unitaires : accents manquants, accent erroné (è / é), lettre doublée, lettre manquante, inversion, majuscule.
- [x] La correction renvoyée au front permet de surligner chaque lettre.

## Implémentation (2026-10-14)

- `educator-core::spelling` : comparaison après normalisation NFC (`spelling/nfc.rs`, sous-ensemble suffisant pour
  les lettres du français, sans dépendance), espaces de début et de fin ignorés, accents stricts, majuscules
  tolérées pour toutes les compétences (un réglage par compétence pourra être ajouté plus tard).
- Correction lettre à lettre par alignement de Levenshtein : chaque lettre est juste, fausse, manquante ou en trop ;
  le front la surligne (`WordCorrection`).
- Tests : accent manquant, è / é, lettre doublée, manquante, inversion, majuscule.
- La ligature « œ » est une lettre à part entière : « soeur » est compté faux pour « sœur » (décision de Georges du
  2026-10-14, puisque le clavier à l'écran propose « œ »).
