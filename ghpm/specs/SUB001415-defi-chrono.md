---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Wgc
rev: 2026-10-10T19:39:20Z
url: https://github.com/georgesp/Educator/issues/31
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001415 — Défi chrono
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#14"
order: 480
---

## User story

En tant qu'enfant, je veux un défi contre la montre afin de m'amuser à aller plus vite.

## Comportement attendu

- Depuis Examen : **Défi chrono**, maximum de bonnes réponses en 2 minutes (durée configurable 1 / 2 / 3 min).
- Compétences déjà travaillées uniquement (maîtrise ≥ 60 %), pour ne pas mettre l'enfant en échec sous pression.
- Chrono visible mais discret (barre qui se vide, pas de compte à rebours sonore).
- Meilleur score personnel affiché.
- Mode **optionnel** : il pourra être désactivé dans les Options (lot 3).

## Critères d'acceptation

- [x] Session `timed` enregistrée.
- [x] Pas de pénalité pour une mauvaise réponse (passage à la suivante).

## Implémentation (2026-10-14)

- Depuis Examen : Défi chrono, durée 1, 2 ou 3 minutes (2 par défaut), compétences de maîtrise ≥ 60 % seulement.
- Barre de temps discrète (`TimerBar`), sans son ; une mauvaise réponse passe à la suivante sans pénalité.
- Fin du temps : `finish_series` (refusé pour une autre séance) ; seules les questions répondues comptent.
- Meilleur score par durée affiché ; le « nouveau record » est décidé côté Rust (`previousBest`, `newRecord`).
- La désactivation dans les Options viendra au lot 3 (SUB001512).
