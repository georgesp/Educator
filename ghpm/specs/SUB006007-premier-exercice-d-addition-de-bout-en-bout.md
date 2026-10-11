---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fSg
rev: 2026-10-10T19:56:57Z
url: https://github.com/georgesp/Educator/issues/67
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006007 — Premier exercice d'addition de bout en bout
status: In progress
size: S
estimate:
startDate:
endDate:
parent: "#60"
order: 8
---

## User story

En tant que développeur, je veux un exercice minimal qui traverse toutes les couches afin de valider l'architecture avant le lot 1.

## Comportement attendu

- Le host génère une addition aléatoire simple (deux nombres de 0 à 9) via `Educator.Core`.
- Le front l'affiche, l'utilisateur tape la réponse, le host la valide.
- Un bouton lit le calcul avec Piper.
- Code jetable ou réutilisé par le moteur du lot 1, sans contrainte de design.

## Critères d'acceptation

- [ ] Parcours fonctionnel sur macOS et Windows.
- [x] Temps de réponse ressenti instantané (< 200 ms hors synthèse vocale).

## Implémentation (2026-10-10)

Endpoints `POST /exercises/additions`, `POST …/{id}/answer`, `GET …/{id}/speech` ; le host garde les exercices en mémoire et ne transmet jamais la réponse attendue avant la réponse de l'enfant. Vérifié de bout en bout sous Linux, avec le vrai Piper. Reste le parcours sur macOS et Windows.

**Lot 1 (2026-10-11)** : cette preuve de concept est remplacée par les séries d'exercices (SUB000107, SUB000109). Ont été retirés : les routes `/exercises/additions`, `AdditionGenerator`, `PendingExercises`, l'écran `AdditionScreen` et leurs tests.
