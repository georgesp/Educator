---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WLo
url: https://github.com/georgesp/Educator/issues/17
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001401 — Enregistrement des séances et des réponses
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#14"
order: 210
---

## User story

En tant que parent, je veux que chaque séance de mon enfant soit enregistrée afin de pouvoir suivre ses progrès plus tard (espace parent au lot 3).

## Modèle

| Entité | Champs principaux |
|---|---|
| `Session` | `Id`, `ProfileId`, `Type` (`exercise` / `exam` / `timed`), `SkillId`, `AnswerMode`, `Seed`, `StartedAt`, `EndedAt`, `Abandoned` |
| `Answer` | `Id`, `SessionId`, `Index`, `Prompt`, `Expected`, `Given`, `IsCorrect`, `DurationMs`, `AnsweredAt` |
| `SkillMastery` | `ProfileId`, `SkillId`, `Score` (0–100), `AttemptCount`, `LastPracticedAt` |

## Comportement attendu

- Les séries d'exercices (maths et orthographe), quiz de leçon et examens créent une `Session` et une `Answer` par question.
- Le temps de réponse est mesuré côté front et transmis avec la réponse.
- `SkillMastery` est mise à jour en fin de séance (moyenne glissante des 20 dernières réponses). Elle servira à la difficulté adaptative et à l'espace parent (lot 3).
- Les séries du lot 1 passent de la mémoire du host à la base.

## Critères d'acceptation

- [ ] Migration EF Core et tests d'intégration.
- [ ] Séance abandonnée enregistrée avec `Abandoned = true` et les réponses déjà données.
- [ ] Suppression d'un profil → suppression en cascade de ses séances, réponses et maîtrises.
- [ ] Aucune donnée ne quitte l'ordinateur.
