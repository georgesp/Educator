---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBVG3M
rev: 2026-10-16T19:31:45Z
url: https://github.com/georgesp/Educator/issues/290
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001515 — Coefficient de points selon la période
status: In Review
size: S
estimate:
startDate:
endDate:
parent: "#15"
order: 645
---

## User story

En tant qu'enfant, je veux gagner plus de points quand je prends de l'avance sur l'année afin d'avoir envie
d'avancer ; en tant que parent, je veux que refaire ce qui est déjà maîtrisé rapporte moins, sans que l'enfant
qui a besoin de revoir une notion soit pénalisé.

## Règles

Les points d'une **série d'exercices** ou d'un **quiz de leçon** sont multipliés par un coefficient, fixé au
démarrage de la série selon la période de l'année (`content/calendar/periods.json`) :

| Situation | Coefficient |
|---|---|
| Période en cours | 100 % |
| Période à venir | **150 %** |
| Période passée, compétence(s) déjà maîtrisée(s) | **50 %** |
| Période passée, pas encore maîtrisée | 100 % |
| Révision de la classe précédente (SUB000113), compétence ou leçon d'une autre classe | 100 % |
| Classe sans contenu (CE2 → CM2 aujourd'hui) qui travaille les exercices du CE1 | 100 % (autre classe) |

- « Maîtrisée » : la même définition que « Je te conseille… » (SUB001508) : au moins `minAnswers` réponses et
  `raiseAbove` % de réussite (`content/progress/adaptive.json`). Une leçon est maîtrisée quand toutes ses
  compétences le sont.
- « Mes révisions » (SUB001509), les examens, le contre-la-montre et l'évaluation de niveau : toujours 100 %.
- Le coefficient s'applique au total de la séance (bonnes réponses et bonus), série abandonnée comprise, arrondi
  au point le plus proche. Le bonus de l'objectif du jour n'est pas concerné.
- L'enfant voit le bonus (« Bonus ×1,5 » sur la carte, « Bonus ×1,5 : tu prends de l'avance ! » au
  récapitulatif) ; la réduction d'une période passée n'est jamais affichée.

## Critères d'acceptation

- [x] Coefficients dans `content/rewards/points.json` (`pastPeriodPercent`, `nextPeriodPercent`), validés par le schéma.
- [x] Règle dans `educator-core` (`points::pace`, `PaceContext::skill` / `lesson`), appliquée par `SeriesService` ; les listes
  (`get_skills`, `list_lessons`) utilisent la même règle avec la classe du profil.
- [x] Les points enregistrés dans la séance (`Session.PointsEarned`) sont ceux du coefficient : synchronisation et
  recalcul des points inchangés.
- [x] `pointsPercent` dans `SkillResponse`, `LessonSummaryResponse` et `SeriesSummaryResponse`.
- [x] Bonus affiché sur les cartes des exercices et des leçons à venir, et au récapitulatif.
- [x] Tests : `educator-core/tests/points.rs`, `series.rs` (série, abandon, quiz de leçon, classe de repli, autre
  classe, contre-la-montre), `reviews.rs` (« Mes révisions » à 100 %), test de `services.rs`, `SkillsScreen.test.tsx`.
- [ ] Vérifié par Georges (`cargo test --workspace`, `npm test`, essai dans l'application).

## Notes techniques

- `start_series` et `start_lesson_quiz` calculent la période du jour (comme `start_exam`) et la transmettent à
  `SeriesService::start` / `start_lesson_quiz`.
- Le rythme des niveaux (`levels.json`) et des badges liés aux points change un peu : à surveiller.
- Statistiques de l'espace parent : les points par séance mélangent désormais 50, 100 et 150 % ; une révision maîtrisée
  rapporte moins sans que ce soit une baisse de niveau.
- Pas de bonus affiché sur la carte « Je te conseille… » ni sur la carte de progression (possible plus tard).
