---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WLo
rev: 2026-10-11T15:22:33Z
url: https://github.com/georgesp/Educator/issues/17
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001401 — Enregistrement des séances et des réponses
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#14"
order: 340
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
- Les séries du lot 1 passent de la mémoire de l'application (`SeriesService` de `educator-core`) à la base.

## Critères d'acceptation

- [x] Migration `educator-data` (nouvelle entrée de `MIGRATIONS`) et tests d'intégration (SQLite réel en fichier temporaire).
- [x] Séance abandonnée enregistrée avec `Abandoned = true` et les réponses déjà données.
- [x] Suppression d'un profil → suppression en cascade de ses séances, réponses et maîtrises.
- [x] Aucune donnée ne quitte l'ordinateur.

## Implémentation (2026-10-14)

- Migration `0002_sessions` (`crates/educator-data/src/migrations.rs`) : tables `Session`, `Answer`, `SkillMastery`
  et `LessonProgress` (slide où une leçon a été laissée, SUB001410), clés étrangères `ON DELETE CASCADE` vers
  `Profile` (activées sur chaque connexion).
- `Session` porte aussi `Subject`, `Grade`, `LessonId`, `Format`, `QuestionCount`, `TimeLimitSeconds`,
  `CorrectCount`, `PointsEarned` ; `Answer` porte le `SkillId` de chaque question (un examen mélange les compétences).
  `Type` : `Exercise`, `Exam`, `Timed`, `LessonQuiz`. `AnswerMode` vide pour un examen ou un défi (modes mélangés).
- Trait `SessionStore` (`educator-core::sessions`), implémenté par `educator-data::sessions` (SQLite) ; le
  `SeriesService` enregistre la séance à son ouverture et chaque réponse dès qu'elle est donnée (durée mesurée
  par le front).
- `SkillMastery` : pourcentage de bonnes réponses parmi les 20 dernières de la compétence, recalculé en fin de séance.
- Les séries restent en mémoire pendant qu'elles se déroulent. Quitter une série, un examen, un défi ou un quiz de
  leçon avant la fin l'enregistre comme abandonné (bonnes réponses comptées, maîtrises mises à jour) ; une séance
  laissée ouverte (application fermée, série sortie de la mémoire) est fermée comme abandonnée au démarrage
  suivant, et ses réponses comptent dans les maîtrises.
- Supprimer un profil oublie aussi ses séries en cours.
- Tests : `crates/educator-data/tests/sessions.rs` (SQLite réel, cascade, abandon), `crates/educator-core/tests/series.rs`.
