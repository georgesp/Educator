---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fSg
rev: 2026-10-11T19:12:54Z
url: https://github.com/georgesp/Educator/issues/67
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006007 — Premier exercice d'addition de bout en bout
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#60"
order: 80
---

## User story

En tant que développeur, je veux un exercice minimal qui traverse toutes les couches afin de valider l'architecture avant le lot 1.

## Comportement attendu

- Le cœur génère une addition aléatoire simple (deux nombres de 0 à 9) via `educator-core`.
- Le front l'affiche, l'utilisateur tape la réponse, le cœur la valide.
- Un bouton lit le calcul avec Piper.
- Code jetable ou réutilisé par le moteur du lot 1, sans contrainte de design.

## Critères d'acceptation

- [ ] Parcours fonctionnel sur macOS et Windows.
- [x] Temps de réponse ressenti instantané (< 200 ms hors synthèse vocale).

## Implémentation (2026-10-10)

Première version (.NET) : exercice gardé en mémoire, réponse attendue jamais transmise avant la réponse de l'enfant, vérifié de bout en bout sous Linux avec le vrai Piper. Reste le parcours sur macOS et Windows.

**Lot 1 (2026-10-11)** : cette preuve de concept est remplacée par les séries d'exercices (SUB000107, SUB000109). Ont été retirés : la génération d'addition isolée, l'écran `AdditionScreen` et leurs tests. Les séries ont ensuite été portées en Rust (lot 1bis).
