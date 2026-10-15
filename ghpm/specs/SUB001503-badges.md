---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Wls
rev: 2026-10-10T19:39:26Z
url: https://github.com/georgesp/Educator/issues/36
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001503 — Badges
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#15"
order: 613
---

## User story

En tant qu'enfant, je veux gagner des badges sur les notions que je maîtrise afin d'en être fier.

## Comportement attendu

- Badges décrits dans `content/rewards/badges.json` : `id`, `title` (« Champion des compléments à 10 »), `icon` (SVG), `condition` (ex. maîtrise ≥ 90 % sur une compétence, 7 jours d'utilisation, 10 séries sans faute).
- Moteur d'évaluation des conditions en fin de séance.
- Écran **Mes badges** : badges obtenus en couleur, badges à gagner en gris avec leur condition expliquée simplement.

## Critères d'acceptation

- [x] Une vingtaine de badges initiaux (maths et orthographe, régularité).
- [x] Tests du moteur de conditions.

## Réalisation

- 23 badges dans `content/rewards/badges.json` (schéma `badges.schema.json`) : 9 de maths, 5 d'orthographe, 9 de régularité et d'effort. Icônes SVG dans `content/rewards/badges/` (médaille du style des personnages ; bleue pour les maths, violette pour l'orthographe, jaune pour la régularité), planche de contrôle `content/rewards/planche-badges.html`.
- Conditions : `skillMastery` (une des compétences maîtrisée à X %, au moins 10 réponses), `categoryMastery` (N compétences d'une famille), `seriesCompleted`, `perfectSeries`, `activeDays`, `goalStreak`, `lessonsFinished`, `stars`, `level`. Une compétence inconnue rend le badge invalide (signalé, ignoré).
- Moteur `educator-core::rewards::badges` : les faits du profil (`BadgeFacts`) sont relus dans les séances, les maîtrises et les jours d'objectif à la fin de chaque séance ; un badge gagné est gardé pour toujours (table `EarnedBadge`), même si la maîtrise baisse ensuite.
- Écran **Mes badges** (`/badges`, depuis le menu) : badges gagnés en couleur avec leur date, badges à gagner en gris avec leur condition dite simplement. Les nouveaux badges s'affichent à la fin de la séance.
- Tests : `crates/educator-core/tests/rewards.rs`, `there_are_about_twenty_badges_of_every_kind` (contenu).
